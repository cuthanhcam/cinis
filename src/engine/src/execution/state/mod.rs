//! Owned financial state and immutable query views.

mod invariants;
mod settlement;
mod transition;

use crate::execution::Limits;
use crate::ledger::{AccountBalances, Asset, Ledger, LedgerTransfer};
use crate::matching::{Engine, Order};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Immutable public view of the financial state privately owned by ExecutionCore.
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
    /// Current and terminal order records; history remains bounded by order capacity.
    pub fn orders(&self) -> &[Order] {
        self.book.orders()
    }
    /// Current account balances in integral base/quote atoms.
    pub fn accounts(&self) -> &BTreeMap<u64, AccountBalances> {
        self.ledger.accounts()
    }
    /// Copy the balance view for an existing account, or None if absent.
    pub fn account(&self, account_id: u64) -> Option<AccountBalances> {
        self.ledger.accounts().get(&account_id).copied()
    }
    /// Historical balanced transfers, not instructions to apply them again.
    pub fn ledger(&self) -> &[LedgerTransfer] {
        self.ledger.transfers()
    }
    /// Total synthetic issuance for one asset in atoms.
    pub fn issued(&self, asset: Asset) -> u64 {
        self.ledger.issued(asset)
    }
    /// Whether fresh order placement is permitted by the sequenced pause state.
    pub fn is_market_open(&self) -> bool {
        self.market_open
    }
    /// Number of orders with a nonzero remaining quantity.
    pub fn resting_count(&self) -> usize {
        self.book.resting_count()
    }
}
