//! Checked ledger mutations, applied only to a tentative financial transaction.
//! A failure can leave tentative changes; the execution owner discards the whole transaction.

use super::{
    AccountBalances, Asset, Ledger, LedgerAccount, LedgerError, LedgerTransfer,
    MAX_ACCOUNT_BALANCE, MAX_ASSET_SUPPLY, PostingId,
};

use std::collections::BTreeMap;

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
    pub(crate) fn ensure_account(&mut self, account: u64) -> Result<(), LedgerError> {
        if !self.accounts.contains_key(&account) {
            if self.accounts.len() >= self.max_accounts {
                return Err(LedgerError::AccountCapacity);
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
    ) -> Result<(), LedgerError> {
        if amount == 0 {
            return Ok(());
        }
        if from == to || to == LedgerAccount::Issuance {
            return Err(LedgerError::InvariantViolation);
        }
        if self.transfers.len() >= self.max_transfers {
            return Err(LedgerError::LedgerCapacity);
        }
        let ordinal = match self.transfers.last() {
            Some(last) if last.id.sequence == sequence => last
                .id
                .ordinal
                .checked_add(1)
                .ok_or(LedgerError::ArithmeticOverflow)?,
            _ => 0,
        };
        if from == LedgerAccount::Issuance {
            let issued = self
                .issued(asset)
                .checked_add(amount)
                .ok_or(LedgerError::ArithmeticOverflow)?;
            if issued > MAX_ASSET_SUPPLY {
                return Err(LedgerError::SupplyLimit);
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
    ) -> Result<(), LedgerError> {
        let (account, reserved) = match bucket {
            LedgerAccount::Available(account) => (account, false),
            LedgerAccount::Reserved(account) => (account, true),
            LedgerAccount::Issuance => return Err(LedgerError::InvariantViolation),
        };
        let balance = self
            .accounts
            .get_mut(&account)
            .ok_or(LedgerError::UnknownAccount)?
            .get_mut(asset);
        let field = if reserved {
            &mut balance.reserved
        } else {
            &mut balance.available
        };
        *field = field.checked_sub(amount).ok_or(if reserved {
            LedgerError::InvariantViolation
        } else {
            LedgerError::InsufficientFunds
        })?;
        Ok(())
    }
    fn credit(
        &mut self,
        bucket: LedgerAccount,
        asset: Asset,
        amount: u64,
    ) -> Result<(), LedgerError> {
        let (account, reserved) = match bucket {
            LedgerAccount::Available(account) => (account, false),
            LedgerAccount::Reserved(account) => (account, true),
            LedgerAccount::Issuance => return Err(LedgerError::InvariantViolation),
        };
        let balance = self
            .accounts
            .get_mut(&account)
            .ok_or(LedgerError::UnknownAccount)?
            .get_mut(asset);
        let total = balance
            .available
            .checked_add(balance.reserved)
            .and_then(|n| n.checked_add(amount))
            .ok_or(LedgerError::ArithmeticOverflow)?;
        if total > MAX_ACCOUNT_BALANCE {
            return Err(LedgerError::BalanceLimit);
        }
        let field = if reserved {
            &mut balance.reserved
        } else {
            &mut balance.available
        };
        *field = field
            .checked_add(amount)
            .ok_or(LedgerError::ArithmeticOverflow)?;
        Ok(())
    }
}
