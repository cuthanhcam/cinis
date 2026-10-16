//! Financial ledger primitives; independent of execution orchestration and I/O.

mod error;
mod invariants;
mod mutation;
mod types;

pub(crate) use error::LedgerError;
use std::collections::BTreeMap;
pub use types::{
    AccountBalances, Asset, Balance, LedgerAccount, LedgerTransfer, MAX_ACCOUNT_BALANCE,
    MAX_ASSET_SUPPLY, PostingId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Ledger {
    accounts: BTreeMap<u64, AccountBalances>,
    transfers: Vec<LedgerTransfer>,
    issued_base: u64,
    issued_quote: u64,
    max_accounts: usize,
    max_transfers: usize,
}

impl Ledger {
    pub(crate) fn accounts(&self) -> &BTreeMap<u64, AccountBalances> {
        &self.accounts
    }
    pub(crate) fn transfers(&self) -> &[LedgerTransfer] {
        &self.transfers
    }
}
