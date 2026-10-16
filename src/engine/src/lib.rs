//! Deterministic single-market execution with volatile financial state.
//! [`Engine`] is the matching-only research primitive; [`ExecutionCore`] is the
//! single owner of balances, orders, holds, ledger and retained command decisions.
//! Neither entry point provides persistence or network authentication.
//!
//! # Trusted submission and retry
//!
//! ```
//! use cinis_engine::{Asset, DecisionSource, ExecutionCommand, ExecutionCore,
//!     ExecutionRequest, Permissions};
//! # fn main() -> Result<(), cinis_engine::AdmissionError> {
//! let mut core = ExecutionCore::default();
//! let request = ExecutionRequest {
//!     account_id: 1,
//!     command_id: 1,
//!     command: ExecutionCommand::Fund { asset: Asset::Quote, amount: 1_000 },
//! };
//! let fresh: cinis_engine::ExecutionOutcome = core.execute_with_outcome(request.clone(), Permissions::ADMIN)?;
//! assert_eq!(fresh.source, DecisionSource::New);
//! assert!(fresh.response.result.is_ok());
//! let retry = core.execute_with_outcome(request, Permissions::ADMIN)?;
//! assert_eq!(retry.source, DecisionSource::Retained);
//! assert_eq!(retry.response, fresh.response);
//! assert_eq!(core.state().account(1).expect("funded account").quote.available, 1_000);
//! # Ok(())
//! # }
//! ```
//!
//! A retained response is historical data, not a new event batch. New business
//! rejections still consume a sequence/receipt. A future network adapter must
//! authenticate first and must never let clients select [`Permissions`].
//!
//! # Architecture
//!
//! Private matching and ledger modules are composed by [`execution`]. Public
//! compatibility exports remain at this crate root; implementation file paths
//! are not contracts. I/O, runtime supervision and persistence belong outside
//! this synchronous deterministic library.
pub mod execution;
pub mod ledger;
mod matching;

pub use execution::{
    AdmissionError, DecisionSource, Effects, EventId, EventKind, ExecutionCommand, ExecutionCore,
    ExecutionEvent, ExecutionOutcome, ExecutionReject, ExecutionRequest, ExecutionResponse,
    ExecutionState, Limits, Permissions,
};
pub use ledger::{
    AccountBalances, Asset, Balance, LedgerAccount, LedgerTransfer, MAX_ACCOUNT_BALANCE,
    MAX_ASSET_SUPPLY, PostingId,
};
pub use matching::{Command, Engine, Event, MAX_PRICE, MAX_QUANTITY, Order, Reject, Side, Status};
