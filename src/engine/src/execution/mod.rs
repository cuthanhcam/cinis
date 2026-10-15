//! Single-owner volatile execution: authorize, deduplicate, transact, reconcile, retain result.
mod state;
mod types;
use crate::LedgerTransfer;
pub use state::ExecutionState;
use std::collections::{BTreeMap, BTreeSet};
pub use types::{
    AdmissionError, Effects, EventId, EventKind, ExecutionCommand, ExecutionEvent, ExecutionReject,
    ExecutionRequest, ExecutionResponse, Limits, Permissions,
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
impl Default for ExecutionCore {
    fn default() -> Self {
        Self::with_limits(Limits::default()).expect("valid default limits")
    }
}
impl ExecutionCore {
    pub fn with_limits(limits: Limits) -> Result<Self, AdmissionError> {
        if u32::try_from(limits.max_events).is_err()
            || u32::try_from(limits.max_ledger_transfers).is_err()
        {
            return Err(AdmissionError::InvalidLimits);
        }
        Ok(Self {
            state: ExecutionState::new(limits),
            limits,
            next_sequence: 1,
            receipts: BTreeMap::new(),
        })
    }
    pub fn state(&self) -> &ExecutionState {
        &self.state
    }
    pub fn next_sequence(&self) -> u64 {
        self.next_sequence
    }
    pub fn retained_commands(&self) -> usize {
        self.receipts.len()
    }
    /// Trusted in-process caller supplies permissions; this is not network authentication.
    pub fn execute(
        &mut self,
        request: ExecutionRequest,
        permissions: Permissions,
    ) -> Result<ExecutionResponse, AdmissionError> {
        if request.account_id == 0 {
            return Err(AdmissionError::InvalidAccountId);
        }
        if request.command_id == 0 {
            return Err(AdmissionError::InvalidCommandId);
        }
        let authorized = match request.command {
            ExecutionCommand::Fund { .. } | ExecutionCommand::SetMarketState { .. } => {
                permissions.can_administer
            }
            _ => permissions.can_trade,
        };
        if !authorized {
            return Err(AdmissionError::Unauthorized);
        }
        let key = (request.account_id, request.command_id);
        if let Some(receipt) = self.receipts.get(&key) {
            return if receipt.command == request.command {
                Ok(receipt.response.clone())
            } else {
                Err(AdmissionError::DuplicateConflict)
            };
        }
        if self.receipts.len() >= self.limits.max_commands {
            return Err(AdmissionError::CommandCapacity);
        }
        let following = self
            .next_sequence
            .checked_add(1)
            .ok_or(AdmissionError::SequenceExhausted)?;
        let mut transaction = self.state.clone();
        let result = transaction
            .apply(
                request.account_id,
                &request.command,
                self.next_sequence,
                self.limits.max_events,
            )
            .and_then(|effects| {
                transaction
                    .check_invariants()
                    .map(|()| effects)
                    .map_err(|_| ExecutionReject::InvariantViolation)
            });
        if result.is_ok() {
            self.state = transaction;
        }
        let response = ExecutionResponse {
            sequence: self.next_sequence,
            result,
        };
        self.receipts.insert(
            key,
            Receipt {
                command: request.command,
                response: response.clone(),
            },
        );
        self.next_sequence = following;
        Ok(response)
    }
    pub fn check_invariants(&self) -> Result<(), &'static str> {
        self.state.check_invariants()?;
        if self.receipts.len() > self.limits.max_commands {
            return Err("receipt capacity");
        }
        if u64::try_from(self.receipts.len())
            .ok()
            .and_then(|n| n.checked_add(1))
            != Some(self.next_sequence)
        {
            return Err("command sequence gap");
        }
        let mut sequences = BTreeSet::new();
        let mut postings: BTreeMap<(u64, u32), &LedgerTransfer> = self
            .state
            .ledger()
            .iter()
            .map(|posting| ((posting.id.sequence, posting.id.ordinal), posting))
            .collect();
        for (&(account, command), receipt) in &self.receipts {
            let sequence = receipt.response.sequence;
            if account == 0
                || command == 0
                || sequence == 0
                || sequence >= self.next_sequence
                || !sequences.insert(sequence)
            {
                return Err("invalid receipt identity");
            }
            if let Ok(effects) = &receipt.response.result {
                if effects.events.is_empty() || effects.events.len() > self.limits.max_events {
                    return Err("event capacity");
                }
                for (ordinal, event) in effects.events.iter().enumerate() {
                    if event.id.sequence != sequence
                        || usize::try_from(event.id.ordinal).ok() != Some(ordinal)
                    {
                        return Err("event identity");
                    }
                }
                for (ordinal, posting) in effects.postings.iter().enumerate() {
                    if posting.id.sequence != sequence
                        || usize::try_from(posting.id.ordinal).ok() != Some(ordinal)
                        || postings.remove(&(sequence, posting.id.ordinal)) != Some(posting)
                    {
                        return Err("receipt/ledger mismatch");
                    }
                }
            }
        }
        if !postings.is_empty() {
            return Err("ledger transfer without successful receipt");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Asset;

    #[test]
    fn sequence_exhaustion_fails_admission_without_mutation() {
        let mut core = ExecutionCore {
            next_sequence: u64::MAX,
            ..ExecutionCore::default()
        };
        let before = core.clone();
        let request = ExecutionRequest {
            account_id: 1,
            command_id: 1,
            command: ExecutionCommand::Fund {
                asset: Asset::Base,
                amount: 1,
            },
        };
        assert_eq!(
            core.execute(request, Permissions::ADMIN),
            Err(AdmissionError::SequenceExhausted)
        );
        assert_eq!(core, before);
    }

    #[test]
    fn ordinal_limits_must_fit_event_and_posting_identifiers() {
        if let Ok(too_large) = usize::try_from(u64::from(u32::MAX) + 1) {
            for limits in [
                Limits {
                    max_events: too_large,
                    ..Limits::default()
                },
                Limits {
                    max_ledger_transfers: too_large,
                    ..Limits::default()
                },
            ] {
                assert_eq!(
                    ExecutionCore::with_limits(limits),
                    Err(AdmissionError::InvalidLimits)
                );
            }
        }
    }
}
