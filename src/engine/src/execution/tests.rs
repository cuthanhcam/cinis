//! Boundary tests that require access to private owner counters.

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
