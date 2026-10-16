//! Per-asset balances and stable balanced-transfer identities.

pub const MAX_ACCOUNT_BALANCE: u64 = 1_000_000_000_000_000_000;
pub const MAX_ASSET_SUPPLY: u64 = 9_000_000_000_000_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// The two synthetic assets in the single-market baseline.
pub enum Asset {
    Base,
    Quote,
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
/// Available and reserved amounts of a single asset, in atoms.
pub struct Balance {
    pub available: u64,
    pub reserved: u64,
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
/// Per-account atom balances, maintained only by financial execution.
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
    pub(super) fn get_mut(&mut self, asset: Asset) -> &mut Balance {
        match asset {
            Asset::Base => &mut self.base,
            Asset::Quote => &mut self.quote,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Accounting bucket; synthetic issuance is an explicit non-customer counterparty.
pub enum LedgerAccount {
    Issuance,
    Available(u64),
    Reserved(u64),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Stable transfer identity in its own namespace, separate from execution EventId.
pub struct PostingId {
    pub sequence: u64,
    pub ordinal: u32,
}
#[derive(Debug, Clone, PartialEq, Eq)]
/// Positive per-asset movement with balanced source/destination legs.
pub struct LedgerTransfer {
    pub id: PostingId,
    pub asset: Asset,
    pub from: LedgerAccount,
    pub to: LedgerAccount,
    pub amount: u64,
}
