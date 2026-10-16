//! Checked quote/base settlement and buy price-improvement release.

use super::ExecutionState;
use crate::execution::ExecutionReject;
use crate::ledger::{Asset, LedgerAccount};

impl ExecutionState {
    pub(super) fn settle(
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
