//! Matching commands, results, order lifecycle and instrument bounds.

/// Maximum accepted price in integral quote atoms per base atom.
pub const MAX_PRICE: u64 = 1_000_000_000;
/// Maximum accepted order quantity in integral base atoms.
pub const MAX_QUANTITY: u64 = 1_000_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Open,
    PartiallyFilled,
    Filled,
    Cancelled,
}
#[derive(Debug, Clone, PartialEq, Eq)]
/// Order history with immutable priority and conserved original quantity.
pub struct Order {
    pub id: u64,
    pub account: u64,
    pub side: Side,
    pub price: u64,
    pub original: u64,
    pub remaining: u64,
    pub filled: u64,
    pub cancelled: u64,
    /// Accepted-place priority ordinal; not the global financial command sequence.
    pub sequence: u64,
}
impl Order {
    pub fn status(&self) -> Status {
        if self.cancelled > 0 {
            Status::Cancelled
        } else if self.remaining == 0 {
            Status::Filled
        } else if self.filled > 0 {
            Status::PartiallyFilled
        } else {
            Status::Open
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
/// Matching-only command; deliberately omits funding and financial admission.
pub enum Command {
    Place {
        id: u64,
        account: u64,
        side: Side,
        price: u64,
        quantity: u64,
    },
    Cancel {
        id: u64,
        account: u64,
    },
}
#[derive(Debug, Clone, PartialEq, Eq)]
/// Matching facts embedded in financial execution events by the owner.
pub enum Event {
    Accepted {
        id: u64,
    },
    Trade {
        maker: u64,
        taker: u64,
        price: u64,
        quantity: u64,
    },
    Cancelled {
        id: u64,
        quantity: u64,
    },
    Terminal {
        id: u64,
        status: Status,
    },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Atomic matching rejection, mapped into financial execution when appropriate.
pub enum Reject {
    InvalidId,
    InvalidPrice,
    InvalidQuantity,
    DuplicateOrderId,
    UnknownOrder,
    NotOwner,
    SelfTrade,
    OrderCapacity,
    EventCapacity,
    ArithmeticOverflow,
}
