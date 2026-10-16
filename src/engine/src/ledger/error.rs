//! Private financial mutation failures, translated by execution.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LedgerError {
    UnknownAccount,
    InsufficientFunds,
    AccountCapacity,
    BalanceLimit,
    SupplyLimit,
    LedgerCapacity,
    ArithmeticOverflow,
    InvariantViolation,
}
