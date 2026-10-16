//! Apply one command to isolated state; caller commits only after validation.

use super::ExecutionState;
use crate::execution::{
    Effects, EventId, EventKind, ExecutionCommand, ExecutionEvent, ExecutionReject,
};
use crate::ledger::{Asset, LedgerAccount, MAX_ACCOUNT_BALANCE};
use crate::matching::{Command, Event, Side};

impl ExecutionState {
    pub(in crate::execution) fn apply(
        &mut self,
        account: u64,
        command: &ExecutionCommand,
        sequence: u64,
        max_events: usize,
    ) -> Result<Effects, ExecutionReject> {
        if max_events == 0 {
            return Err(ExecutionReject::EventCapacity);
        }
        let start = self.ledger.transfers().len();
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
            postings: self.ledger.transfers()[start..].to_vec(),
        })
    }
}
