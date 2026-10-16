//! Authorization, sequencing, tentative transaction and retained outcomes.

use super::{
    AdmissionError, DecisionSource, ExecutionCommand, ExecutionCore, ExecutionOutcome,
    ExecutionReject, ExecutionRequest, ExecutionResponse, ExecutionState, Limits, Permissions,
    Receipt,
};
use std::collections::BTreeMap;

impl Default for ExecutionCore {
    fn default() -> Self {
        Self::with_limits(Limits::default()).expect("valid default limits")
    }
}
impl ExecutionCore {
    /// Construct an isolated volatile owner with instance-lifetime capacities.
    ///
    /// # Errors
    /// Returns [`AdmissionError::InvalidLimits`] if event or posting capacity
    /// cannot be represented by the stable u32 ordinal identity.
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
    /// Inspect current state without exposing mutable financial storage.
    pub fn state(&self) -> &ExecutionState {
        &self.state
    }
    /// Next fresh admitted decision sequence, including business rejections.
    pub fn next_sequence(&self) -> u64 {
        self.next_sequence
    }
    /// Number of original success/rejection receipts retained in this instance.
    pub fn retained_commands(&self) -> usize {
        self.receipts.len()
    }
    /// Execute a trusted request, returning the stable original response.
    ///
    /// Use [`Self::execute_with_outcome`] when an adapter needs to distinguish a
    /// new decision from a historical retry. Permissions are not authentication.
    ///
    /// # Errors
    /// Returns [`AdmissionError`] without mutation for invalid identity, missing
    /// permission, duplicate conflict, receipt capacity or sequence exhaustion.
    /// Business rejections instead appear inside [`ExecutionResponse::result`].
    pub fn execute(
        &mut self,
        request: ExecutionRequest,
        permissions: Permissions,
    ) -> Result<ExecutionResponse, AdmissionError> {
        self.execute_with_outcome(request, permissions)
            .map(|outcome| outcome.response)
    }

    /// Execute a trusted request and classify this invocation as new or retained.
    ///
    /// Authorization always precedes receipt lookup. A new business rejection is
    /// still a new decision with a sequence, but has no financial effects. A
    /// retained success contains historical events that must not be republished.
    /// All state and receipts remain volatile; this does not implement an outbox.
    ///
    /// # Errors
    /// Same admission failures and no-mutation contract as [`Self::execute`].
    pub fn execute_with_outcome(
        &mut self,
        request: ExecutionRequest,
        permissions: Permissions,
    ) -> Result<ExecutionOutcome, AdmissionError> {
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
                Ok(ExecutionOutcome {
                    source: DecisionSource::Retained,
                    response: receipt.response.clone(),
                })
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
        Ok(ExecutionOutcome {
            source: DecisionSource::New,
            response,
        })
    }
}
