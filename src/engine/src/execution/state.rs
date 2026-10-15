//! Tentative matching, checked postings, and financial invariant validation.
use super::{
    Effects, EventId, EventKind, ExecutionCommand, ExecutionEvent, ExecutionReject, Limits,
};
use crate::ledger::Ledger;
use crate::{
    AccountBalances, Asset, Command, Engine, Event, LedgerAccount, LedgerTransfer,
    MAX_ACCOUNT_BALANCE, Order, Side,
};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionState {
    book: Engine,
    ledger: Ledger,
    market_open: bool,
}
impl ExecutionState {
    pub(super) fn new(limits: Limits) -> Self {
        Self {
            book: Engine::new(limits.max_orders, limits.max_events),
            ledger: Ledger::new(limits.max_accounts, limits.max_ledger_transfers),
            market_open: true,
        }
    }
    pub fn orders(&self) -> &[Order] {
        self.book.orders()
    }
    pub fn accounts(&self) -> &BTreeMap<u64, AccountBalances> {
        &self.ledger.accounts
    }
    pub fn account(&self, account_id: u64) -> Option<AccountBalances> {
        self.ledger.accounts.get(&account_id).copied()
    }
    pub fn ledger(&self) -> &[LedgerTransfer] {
        &self.ledger.transfers
    }
    pub fn issued(&self, asset: Asset) -> u64 {
        self.ledger.issued(asset)
    }
    pub fn is_market_open(&self) -> bool {
        self.market_open
    }
    pub fn resting_count(&self) -> usize {
        self.book.resting_count()
    }
    pub fn check_invariants(&self) -> Result<(), &'static str> {
        self.book.check_invariants()?;
        self.ledger.reconcile()?;
        let mut obligations: BTreeMap<(u64, Asset), u64> = BTreeMap::new();
        for order in self.orders() {
            if !self.ledger.accounts.contains_key(&order.account) {
                return Err("order has no financial account");
            }
            if order.remaining == 0 {
                continue;
            }
            let (asset, amount) = match order.side {
                Side::Buy => (
                    Asset::Quote,
                    order
                        .price
                        .checked_mul(order.remaining)
                        .ok_or("obligation overflow")?,
                ),
                Side::Sell => (Asset::Base, order.remaining),
            };
            let value = obligations.entry((order.account, asset)).or_default();
            *value = value
                .checked_add(amount)
                .ok_or("aggregate obligation overflow")?;
        }
        for (&account, balances) in &self.ledger.accounts {
            for asset in [Asset::Base, Asset::Quote] {
                if balances.get(asset).reserved
                    != obligations.remove(&(account, asset)).unwrap_or_default()
                {
                    return Err("reservation/order mismatch");
                }
            }
        }
        if !obligations.is_empty() {
            return Err("orphan reservation obligation");
        }
        Ok(())
    }
    pub(super) fn apply(
        &mut self,
        account: u64,
        command: &ExecutionCommand,
        sequence: u64,
        max_events: usize,
    ) -> Result<Effects, ExecutionReject> {
        if max_events == 0 {
            return Err(ExecutionReject::EventCapacity);
        }
        let start = self.ledger.transfers.len();
        let kinds = match *command {
            ExecutionCommand::Fund { asset, amount } => {
                if amount == 0 || amount > MAX_ACCOUNT_BALANCE {
                    return Err(ExecutionReject::InvalidAmount);
                }
                self.ledger.ensure_account(account)?;
                self.ledger.transfer(
                    sequence,
                    asset,
                    LedgerAccount::Issuance,
                    LedgerAccount::Available(account),
                    amount,
                )?;
                vec![EventKind::Funded {
                    account_id: account,
                    asset,
                    amount,
                }]
            }
            ExecutionCommand::SetMarketState { open } => {
                self.market_open = open;
                vec![EventKind::MarketStateChanged { open }]
            }
            ExecutionCommand::Place {
                order_id,
                side,
                price,
                quantity,
            } => {
                if !self.market_open {
                    return Err(ExecutionReject::MarketPaused);
                }
                let mut planned_book = self.book.clone();
                let events = planned_book
                    .execute(Command::Place {
                        id: order_id,
                        account,
                        side,
                        price,
                        quantity,
                    })
                    .map_err(ExecutionReject::Matching)?;
                let (asset, amount) = match side {
                    Side::Buy => (
                        Asset::Quote,
                        price
                            .checked_mul(quantity)
                            .ok_or(ExecutionReject::ArithmeticOverflow)?,
                    ),
                    Side::Sell => (Asset::Base, quantity),
                };
                self.ledger.transfer(
                    sequence,
                    asset,
                    LedgerAccount::Available(account),
                    LedgerAccount::Reserved(account),
                    amount,
                )?;
                for event in &events {
                    if let Event::Trade {
                        maker,
                        price: execution_price,
                        quantity: filled,
                        ..
                    } = *event
                    {
                        let resting = self
                            .book
                            .orders()
                            .iter()
                            .find(|order| order.id == maker)
                            .ok_or(ExecutionReject::InvariantViolation)?;
                        let (buyer, seller, buy_limit) = match side {
                            Side::Buy => (account, resting.account, price),
                            Side::Sell => (resting.account, account, resting.price),
                        };
                        self.settle(sequence, buyer, seller, buy_limit, execution_price, filled)?;
                    }
                }
                self.book = planned_book;
                events.into_iter().map(EventKind::Order).collect()
            }
            ExecutionCommand::Cancel { order_id } => {
                let mut planned_book = self.book.clone();
                let events = planned_book
                    .execute(Command::Cancel {
                        id: order_id,
                        account,
                    })
                    .map_err(ExecutionReject::Matching)?;
                if let Some(Event::Cancelled { quantity, .. }) = events.first() {
                    let order = self
                        .book
                        .orders()
                        .iter()
                        .find(|order| order.id == order_id)
                        .ok_or(ExecutionReject::InvariantViolation)?;
                    let (asset, amount) = match order.side {
                        Side::Buy => (
                            Asset::Quote,
                            order
                                .price
                                .checked_mul(*quantity)
                                .ok_or(ExecutionReject::ArithmeticOverflow)?,
                        ),
                        Side::Sell => (Asset::Base, *quantity),
                    };
                    self.ledger.transfer(
                        sequence,
                        asset,
                        LedgerAccount::Reserved(account),
                        LedgerAccount::Available(account),
                        amount,
                    )?;
                }
                self.book = planned_book;
                events.into_iter().map(EventKind::Order).collect()
            }
        };
        let events = kinds
            .into_iter()
            .enumerate()
            .map(|(ordinal, kind)| {
                Ok(ExecutionEvent {
                    id: EventId {
                        sequence,
                        ordinal: u32::try_from(ordinal)
                            .map_err(|_| ExecutionReject::ArithmeticOverflow)?,
                    },
                    kind,
                })
            })
            .collect::<Result<Vec<_>, ExecutionReject>>()?;
        Ok(Effects {
            events,
            postings: self.ledger.transfers[start..].to_vec(),
        })
    }
    fn settle(
        &mut self,
        sequence: u64,
        buyer: u64,
        seller: u64,
        buy_limit: u64,
        price: u64,
        quantity: u64,
    ) -> Result<(), ExecutionReject> {
        let cost = price
            .checked_mul(quantity)
            .ok_or(ExecutionReject::ArithmeticOverflow)?;
        let improvement = buy_limit
            .checked_sub(price)
            .and_then(|n| n.checked_mul(quantity))
            .ok_or(ExecutionReject::ArithmeticOverflow)?;
        self.ledger.transfer(
            sequence,
            Asset::Quote,
            LedgerAccount::Reserved(buyer),
            LedgerAccount::Available(seller),
            cost,
        )?;
        self.ledger.transfer(
            sequence,
            Asset::Quote,
            LedgerAccount::Reserved(buyer),
            LedgerAccount::Available(buyer),
            improvement,
        )?;
        self.ledger.transfer(
            sequence,
            Asset::Base,
            LedgerAccount::Reserved(seller),
            LedgerAccount::Available(buyer),
            quantity,
        )?;
        Ok(())
    }
}
