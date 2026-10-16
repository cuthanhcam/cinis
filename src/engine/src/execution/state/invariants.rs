//! Matching/ledger reconciliation and active-order reservation equality.

use super::ExecutionState;
use crate::ledger::Asset;
use crate::matching::Side;
use std::collections::BTreeMap;

impl ExecutionState {
    pub fn check_invariants(&self) -> Result<(), &'static str> {
        self.book.check_invariants()?;
        self.ledger.reconcile()?;
        let mut obligations: BTreeMap<(u64, Asset), u64> = BTreeMap::new();
        for order in self.orders() {
            if !self.ledger.accounts().contains_key(&order.account) {
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
        for (&account, balances) in self.ledger.accounts() {
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
}
