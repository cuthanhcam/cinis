//! Reconstruct balances independently from retained per-asset transfers.

use super::{Asset, Ledger, LedgerAccount, MAX_ACCOUNT_BALANCE, MAX_ASSET_SUPPLY, PostingId};
use std::collections::BTreeMap;

impl Ledger {
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
