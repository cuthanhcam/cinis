//! Adapter-facing invocation classification without changing financial semantics.
use cinis_engine::{
    AdmissionError, Asset, DecisionSource, ExecutionCommand, ExecutionCore, ExecutionReject,
    ExecutionRequest, Limits, Permissions, Side,
};

fn funding(command_id: u64, amount: u64) -> ExecutionRequest {
    ExecutionRequest {
        account_id: 1,
        command_id,
        command: ExecutionCommand::Fund {
            asset: Asset::Quote,
            amount,
        },
    }
}

#[test]
fn retained_success_returns_original_effects_without_new_financial_work() {
    let mut core = ExecutionCore::default();
    let request = funding(1, 100);
    let fresh = core
        .execute_with_outcome(request.clone(), Permissions::ADMIN)
        .unwrap();
    assert_eq!(fresh.source, DecisionSource::New);
    assert_eq!(fresh.response.result.as_ref().unwrap().postings.len(), 1);
    core.execute(funding(2, 50), Permissions::ADMIN)
        .unwrap()
        .result
        .unwrap();
    let before = core.clone();
    let retry = core
        .execute_with_outcome(request, Permissions::ADMIN)
        .unwrap();
    assert_eq!(retry.source, DecisionSource::Retained);
    assert_eq!(retry.response, fresh.response);
    assert_eq!(core, before);
    assert_eq!(core.state().account(1).unwrap().quote.available, 150);
    core.check_invariants().unwrap();
}

#[test]
fn rejected_new_decision_has_progress_but_retry_cannot_reevaluate_it() {
    let mut core = ExecutionCore::default();
    let request = funding(1, 0);
    let fresh = core
        .execute_with_outcome(request.clone(), Permissions::ADMIN)
        .unwrap();
    assert_eq!(fresh.source, DecisionSource::New);
    assert_eq!(fresh.response.sequence, 1);
    assert_eq!(fresh.response.result, Err(ExecutionReject::InvalidAmount));
    assert_eq!(core.next_sequence(), 2);
    assert_eq!(core.retained_commands(), 1);
    assert!(core.state().accounts().is_empty());
    core.execute(funding(2, 100), Permissions::ADMIN)
        .unwrap()
        .result
        .unwrap();
    let before = core.clone();
    let retry = core
        .execute_with_outcome(request, Permissions::ADMIN)
        .unwrap();
    assert_eq!(retry.source, DecisionSource::Retained);
    assert_eq!(retry.response, fresh.response);
    assert_eq!(core, before);
    core.check_invariants().unwrap();
}

#[test]
fn authorization_and_conflict_fail_without_exposing_an_outcome() {
    let mut core = ExecutionCore::default();
    core.execute(funding(1, 100), Permissions::ADMIN).unwrap();
    let before = core.clone();
    assert_eq!(
        core.execute_with_outcome(funding(1, 100), Permissions::TRADER),
        Err(AdmissionError::Unauthorized)
    );
    assert_eq!(
        core.execute_with_outcome(funding(1, 200), Permissions::ADMIN),
        Err(AdmissionError::DuplicateConflict)
    );
    assert_eq!(core, before);
}

#[test]
fn retained_outcome_remains_available_at_receipt_capacity() {
    let mut core = ExecutionCore::with_limits(Limits {
        max_commands: 1,
        ..Limits::default()
    })
    .unwrap();
    let first = core
        .execute_with_outcome(funding(1, 100), Permissions::ADMIN)
        .unwrap();
    let before = core.clone();
    assert_eq!(
        core.execute_with_outcome(funding(2, 1), Permissions::ADMIN),
        Err(AdmissionError::CommandCapacity)
    );
    let retry = core
        .execute_with_outcome(funding(1, 100), Permissions::ADMIN)
        .unwrap();
    assert_eq!(retry.source, DecisionSource::Retained);
    assert_eq!(retry.response, first.response);
    assert_eq!(core, before);
}

#[test]
fn compatibility_and_classified_apis_produce_identical_state_and_responses() {
    let mut original = ExecutionCore::default();
    let mut classified = ExecutionCore::default();
    let order = ExecutionRequest {
        account_id: 1,
        command_id: 2,
        command: ExecutionCommand::Place {
            order_id: 1,
            side: Side::Buy,
            price: 10,
            quantity: 4,
        },
    };
    let requests = [
        funding(1, 100),
        order.clone(),
        order,
        ExecutionRequest {
            account_id: 1,
            command_id: 3,
            command: ExecutionCommand::Cancel { order_id: 1 },
        },
        funding(4, 0),
        funding(4, 0),
        funding(1, 200),
    ];
    let expected = [
        Some(DecisionSource::New),
        Some(DecisionSource::New),
        Some(DecisionSource::Retained),
        Some(DecisionSource::New),
        Some(DecisionSource::New),
        Some(DecisionSource::Retained),
        None,
    ];
    for (request, source) in requests.into_iter().zip(expected) {
        let old = original.execute(request.clone(), Permissions::ADMIN);
        let new = classified.execute_with_outcome(request, Permissions::ADMIN);
        assert_eq!(new.as_ref().ok().map(|v| v.source), source);
        assert_eq!(old, new.map(|v| v.response));
        assert_eq!(original, classified);
        original.check_invariants().unwrap();
    }
}
