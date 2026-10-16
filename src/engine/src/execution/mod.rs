//! Single-owner volatile financial execution and retained command decisions.

mod core;
mod invariants;
mod state;
#[cfg(test)]
mod tests;
mod types;

pub use state::ExecutionState;
use std::collections::BTreeMap;
pub use types::{
    AdmissionError, DecisionSource, Effects, EventId, EventKind, ExecutionCommand, ExecutionEvent,
    ExecutionOutcome, ExecutionReject, ExecutionRequest, ExecutionResponse, Limits, Permissions,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Receipt {
    command: ExecutionCommand,
    response: ExecutionResponse,
}
#[derive(Debug, Clone, PartialEq, Eq)]
/// The sole mutable owner of matching, balances, ledger, market state, and retained decisions.
/// State is volatile; successful responses do not promise restart durability.
pub struct ExecutionCore {
    state: ExecutionState,
    limits: Limits,
    next_sequence: u64,
    receipts: BTreeMap<(u64, u64), Receipt>,
}
