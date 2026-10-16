//! Matching-only research primitive; financial callers must use ExecutionCore.

mod book;
mod invariants;
mod types;

pub use types::{Command, Event, MAX_PRICE, MAX_QUANTITY, Order, Reject, Side, Status};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Engine {
    orders: Vec<Order>,
    next_sequence: u64,
    max_orders: usize,
    max_events: usize,
}
