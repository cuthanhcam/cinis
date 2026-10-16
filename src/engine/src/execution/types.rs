//! Requests, permissions, results, event identities, and bounded execution configuration.
use crate::ledger::{Asset, LedgerError, LedgerTransfer};
use crate::matching::{Event, Reject, Side};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Permissions supplied by a trusted in-process caller; not a network authentication mechanism.
pub struct Permissions {
    pub can_trade: bool,
    pub can_administer: bool,
}
impl Permissions {
    pub const TRADER: Self = Self {
        can_trade: true,
        can_administer: false,
    };
    pub const ADMIN: Self = Self {
        can_trade: true,
        can_administer: true,
    };
}
#[derive(Debug, Clone, PartialEq, Eq)]
/// Typed canonical payload for one synthetic-market command.
pub enum ExecutionCommand {
    Fund {
        asset: Asset,
        amount: u64,
    },
    Place {
        order_id: u64,
        side: Side,
        price: u64,
        quantity: u64,
    },
    Cancel {
        order_id: u64,
    },
    SetMarketState {
        open: bool,
    },
}
#[derive(Debug, Clone, PartialEq, Eq)]
/// Account-scoped idempotency identity; command IDs must be nonzero.
pub struct ExecutionRequest {
    pub account_id: u64,
    pub command_id: u64,
    pub command: ExecutionCommand,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// No decision was admitted: sequence, cache, and financial state are unchanged.
pub enum AdmissionError {
    InvalidAccountId,
    InvalidCommandId,
    Unauthorized,
    DuplicateConflict,
    CommandCapacity,
    SequenceExhausted,
    InvalidLimits,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Retained business decision: a sequence/cache entry exists, but financial state is unchanged.
pub enum ExecutionReject {
    Matching(Reject),
    UnknownAccount,
    InsufficientFunds,
    InvalidAmount,
    AccountCapacity,
    BalanceLimit,
    SupplyLimit,
    LedgerCapacity,
    MarketPaused,
    EventCapacity,
    ArithmeticOverflow,
    InvariantViolation,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventId {
    pub sequence: u64,
    pub ordinal: u32,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventKind {
    Funded {
        account_id: u64,
        asset: Asset,
        amount: u64,
    },
    Order(Event),
    MarketStateChanged {
        open: bool,
    },
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionEvent {
    pub id: EventId,
    pub kind: EventKind,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Effects {
    pub events: Vec<ExecutionEvent>,
    pub postings: Vec<LedgerTransfer>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
/// Original volatile decision, returned identically on an authorized retry.
/// Repeated event/posting IDs are historical results, not instructions to apply effects again.
pub struct ExecutionResponse {
    pub sequence: u64,
    pub result: Result<Effects, ExecutionReject>,
}
/// Whether this call admitted a new decision or retrieved an existing receipt.
/// Neither variant promises durable storage or successful business execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionSource {
    /// A fresh admitted command, including a retained business rejection.
    New,
    /// An authorized same-payload retry; no new transition or sequence.
    Retained,
}

/// Invocation metadata kept separate from the stable original response.
///
/// A future adapter can distinguish historical retries from new decisions without
/// guessing from sequence numbers. This is not a durable event delivery mechanism.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use = "inspect decision source before treating historical effects as new"]
pub struct ExecutionOutcome {
    /// Classification of this invocation, not of the original business result.
    pub source: DecisionSource,
    /// Original response; identical for a fresh decision and its authorized retry.
    pub response: ExecutionResponse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Instance-lifetime capacities; terminal orders and rejected decisions count as history.
pub struct Limits {
    pub max_orders: usize,
    pub max_events: usize,
    pub max_accounts: usize,
    pub max_ledger_transfers: usize,
    pub max_commands: usize,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            max_orders: 10_000,
            max_events: 1_024,
            max_accounts: 10_000,
            max_ledger_transfers: 100_000,
            max_commands: 10_000,
        }
    }
}

impl From<LedgerError> for ExecutionReject {
    fn from(error: LedgerError) -> Self {
        match error {
            LedgerError::UnknownAccount => Self::UnknownAccount,
            LedgerError::InsufficientFunds => Self::InsufficientFunds,
            LedgerError::AccountCapacity => Self::AccountCapacity,
            LedgerError::BalanceLimit => Self::BalanceLimit,
            LedgerError::SupplyLimit => Self::SupplyLimit,
            LedgerError::LedgerCapacity => Self::LedgerCapacity,
            LedgerError::ArithmeticOverflow => Self::ArithmeticOverflow,
            LedgerError::InvariantViolation => Self::InvariantViolation,
        }
    }
}
