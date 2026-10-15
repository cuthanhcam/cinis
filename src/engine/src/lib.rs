//! Deterministic single-market execution with volatile financial state.
//! Engine is the matching-only primitive; ExecutionCore owns financial transitions.
pub mod execution;
pub mod ledger;
mod matching;

pub use execution::{
    AdmissionError, Effects, EventId, EventKind, ExecutionCommand, ExecutionCore, ExecutionEvent,
    ExecutionReject, ExecutionRequest, ExecutionResponse, ExecutionState, Limits, Permissions,
};
pub use ledger::{
    AccountBalances, Asset, Balance, LedgerAccount, LedgerTransfer, MAX_ACCOUNT_BALANCE,
    MAX_ASSET_SUPPLY, PostingId,
};
pub use matching::{Command, Engine, Event, MAX_PRICE, MAX_QUANTITY, Order, Reject, Side, Status};
