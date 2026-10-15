//! Balanced per-asset transfers and independently reconcilable execution balances.
use crate::execution::ExecutionReject;
use std::collections::BTreeMap;

pub const MAX_ACCOUNT_BALANCE: u64 = 1_000_000_000_000_000_000;
pub const MAX_ASSET_SUPPLY: u64 = 9_000_000_000_000_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Asset {
    Base,
    Quote,
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Balance {
    pub available: u64,
    pub reserved: u64,
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct AccountBalances {
    pub base: Balance,
    pub quote: Balance,
}
impl AccountBalances {
    pub fn get(&self, asset: Asset) -> Balance {
        match asset {
            Asset::Base => self.base,
            Asset::Quote => self.quote,
        }
    }
    fn get_mut(&mut self, asset: Asset) -> &mut Balance {
        match asset {
            Asset::Base => &mut self.base,
            Asset::Quote => &mut self.quote,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LedgerAccount {
    Issuance,
    Available(u64),
    Reserved(u64),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PostingId {
    pub sequence: u64,
    pub ordinal: u32,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerTransfer {
    pub id: PostingId,
    pub asset: Asset,
    pub from: LedgerAccount,
    pub to: LedgerAccount,
    pub amount: u64,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Ledger {
    pub(crate) accounts: BTreeMap<u64, AccountBalances>,
    pub(crate) transfers: Vec<LedgerTransfer>,
    issued_base: u64,
    issued_quote: u64,
    max_accounts: usize,
    max_transfers: usize,
}
impl Ledger {
    pub(crate) fn new(max_accounts: usize, max_transfers: usize) -> Self {
        Self {
            accounts: BTreeMap::new(),
            transfers: Vec::new(),
            issued_base: 0,
            issued_quote: 0,
            max_accounts,
            max_transfers,
        }
    }
    pub(crate) fn issued(&self, asset: Asset) -> u64 {
        match asset {
            Asset::Base => self.issued_base,
            Asset::Quote => self.issued_quote,
        }
    }
    pub(crate) fn ensure_account(&mut self, account: u64) -> Result<(), ExecutionReject> {
        if !self.accounts.contains_key(&account) {
            if self.accounts.len() >= self.max_accounts {
                return Err(ExecutionReject::AccountCapacity);
            }
            self.accounts.insert(account, AccountBalances::default());
        }
        Ok(())
    }
    pub(crate) fn transfer(
        &mut self,
        sequence: u64,
        asset: Asset,
        from: LedgerAccount,
        to: LedgerAccount,
        amount: u64,
    ) -> Result<(), ExecutionReject> {
        if amount == 0 {
            return Ok(());
        }
        if from == to || to == LedgerAccount::Issuance {
            return Err(ExecutionReject::InvariantViolation);
        }
        if self.transfers.len() >= self.max_transfers {
            return Err(ExecutionReject::LedgerCapacity);
        }
        let ordinal = match self.transfers.last() {
            Some(last) if last.id.sequence == sequence => last
                .id
                .ordinal
                .checked_add(1)
                .ok_or(ExecutionReject::ArithmeticOverflow)?,
            _ => 0,
        };
        if from == LedgerAccount::Issuance {
            let issued = self
                .issued(asset)
                .checked_add(amount)
                .ok_or(ExecutionReject::ArithmeticOverflow)?;
            if issued > MAX_ASSET_SUPPLY {
                return Err(ExecutionReject::SupplyLimit);
            }
            match asset {
                Asset::Base => self.issued_base = issued,
                Asset::Quote => self.issued_quote = issued,
            }
        } else {
            self.debit(from, asset, amount)?;
        }
        self.credit(to, asset, amount)?;
        self.transfers.push(LedgerTransfer {
            id: PostingId { sequence, ordinal },
            asset,
            from,
            to,
            amount,
        });
        Ok(())
    }
    fn debit(
        &mut self,
        bucket: LedgerAccount,
        asset: Asset,
        amount: u64,
    ) -> Result<(), ExecutionReject> {
        let (account, reserved) = match bucket {
            LedgerAccount::Available(account) => (account, false),
            LedgerAccount::Reserved(account) => (account, true),
            LedgerAccount::Issuance => return Err(ExecutionReject::InvariantViolation),
        };
        let balance = self
            .accounts
            .get_mut(&account)
            .ok_or(ExecutionReject::UnknownAccount)?
            .get_mut(asset);
        let field = if reserved {
            &mut balance.reserved
        } else {
            &mut balance.available
        };
        *field = field.checked_sub(amount).ok_or(if reserved {
            ExecutionReject::InvariantViolation
        } else {
            ExecutionReject::InsufficientFunds
        })?;
        Ok(())
    }
    fn credit(
        &mut self,
        bucket: LedgerAccount,
        asset: Asset,
        amount: u64,
    ) -> Result<(), ExecutionReject> {
        let (account, reserved) = match bucket {
            LedgerAccount::Available(account) => (account, false),
            LedgerAccount::Reserved(account) => (account, true),
            LedgerAccount::Issuance => return Err(ExecutionReject::InvariantViolation),
        };
        let balance = self
            .accounts
            .get_mut(&account)
            .ok_or(ExecutionReject::UnknownAccount)?
            .get_mut(asset);
        let total = balance
            .available
            .checked_add(balance.reserved)
            .and_then(|n| n.checked_add(amount))
            .ok_or(ExecutionReject::ArithmeticOverflow)?;
        if total > MAX_ACCOUNT_BALANCE {
            return Err(ExecutionReject::BalanceLimit);
        }
        let field = if reserved {
            &mut balance.reserved
        } else {
            &mut balance.available
        };
        *field = field
            .checked_add(amount)
            .ok_or(ExecutionReject::ArithmeticOverflow)?;
        Ok(())
    }
    pub(crate) fn reconcile(&self) -> Result<(), &'static str> {
        let mut buckets: BTreeMap<(Asset, LedgerAccount), i128> = BTreeMap::new();
        let mut previous: Option<PostingId> = None;
        for transfer in &self.transfers {
            if transfer.amount == 0
                || transfer.from == transfer.to
                || transfer.to == LedgerAccount::Issuance
                || transfer.id.sequence == 0
            {
                return Err("invalid ledger transfer");
            }
            let expected_ordinal = match previous {
                Some(id) if id.sequence == transfer.id.sequence => id
                    .ordinal
                    .checked_add(1)
                    .ok_or("posting ordinal overflow")?,
                Some(id) if id.sequence > transfer.id.sequence => {
                    return Err("ledger sequence regressed");
                }
                _ => 0,
            };
            if transfer.id.ordinal != expected_ordinal {
                return Err("posting ordinal gap");
            }
            previous = Some(transfer.id);
            for (bucket, delta) in [
                (transfer.from, -i128::from(transfer.amount)),
                (transfer.to, i128::from(transfer.amount)),
            ] {
                let value = buckets.entry((transfer.asset, bucket)).or_default();
                *value = value
                    .checked_add(delta)
                    .ok_or("ledger aggregation overflow")?;
            }
        }
        for asset in [Asset::Base, Asset::Quote] {
            let mut total = 0_u128;
            for (&account, balances) in &self.accounts {
                if account == 0 {
                    return Err("zero account identity");
                }
                let balance = balances.get(asset);
                let account_total = balance
                    .available
                    .checked_add(balance.reserved)
                    .ok_or("balance overflow")?;
                if account_total > MAX_ACCOUNT_BALANCE {
                    return Err("account balance limit");
                }
                total = total
                    .checked_add(u128::from(account_total))
                    .ok_or("asset aggregation overflow")?;
                for (bucket, value) in [
                    (LedgerAccount::Available(account), balance.available),
                    (LedgerAccount::Reserved(account), balance.reserved),
                ] {
                    if buckets.remove(&(asset, bucket)).unwrap_or_default() != i128::from(value) {
                        return Err("ledger/balance mismatch");
                    }
                }
            }
            let issued = self.issued(asset);
            if total != u128::from(issued) || issued > MAX_ASSET_SUPPLY {
                return Err("asset conservation");
            }
            if buckets
                .remove(&(asset, LedgerAccount::Issuance))
                .unwrap_or_default()
                != -i128::from(issued)
            {
                return Err("issuance mismatch");
            }
        }
        if !buckets.is_empty() {
            return Err("orphan ledger account");
        }
        if self.accounts.len() > self.max_accounts || self.transfers.len() > self.max_transfers {
            return Err("ledger capacity");
        }
        Ok(())
    }
}
