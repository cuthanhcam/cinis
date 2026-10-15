use cinis_engine::{
    AdmissionError, Asset, Balance, Event, EventKind, ExecutionCommand as Cmd, ExecutionCore,
    ExecutionReject as Reject, ExecutionRequest, ExecutionResponse, LedgerAccount, Limits,
    MAX_ACCOUNT_BALANCE, MAX_ASSET_SUPPLY, Permissions, Reject as MatchingReject, Side,
};

fn request(account: u64, id: u64, command: Cmd) -> ExecutionRequest {
    ExecutionRequest {
        account_id: account,
        command_id: id,
        command,
    }
}
fn submit(core: &mut ExecutionCore, account: u64, id: u64, command: Cmd) -> ExecutionResponse {
    let response = core
        .execute(request(account, id, command), Permissions::ADMIN)
        .unwrap();
    core.check_invariants().unwrap();
    response
}
fn fund(core: &mut ExecutionCore, account: u64, id: u64, asset: Asset, amount: u64) {
    assert!(
        submit(core, account, id, Cmd::Fund { asset, amount })
            .result
            .is_ok()
    );
}
fn place(
    core: &mut ExecutionCore,
    account: u64,
    id: u64,
    order_id: u64,
    side: Side,
    price: u64,
    quantity: u64,
) -> ExecutionResponse {
    submit(
        core,
        account,
        id,
        Cmd::Place {
            order_id,
            side,
            price,
            quantity,
        },
    )
}
fn balance(core: &ExecutionCore, account: u64, asset: Asset) -> Balance {
    core.state().account(account).unwrap().get(asset)
}
fn rejected(
    core: &mut ExecutionCore,
    account: u64,
    id: u64,
    command: Cmd,
    expected: Reject,
) -> ExecutionResponse {
    let before = core.state().clone();
    let sequence = core.next_sequence();
    let response = submit(core, account, id, command);
    assert_eq!(response.result, Err(expected));
    assert_eq!(response.sequence, sequence);
    assert_eq!(core.next_sequence(), sequence + 1);
    assert_eq!(
        core.state(),
        &before,
        "business rejection must roll back all financial state"
    );
    response
}

#[test]
fn funding_has_balanced_issuance_and_stable_posting_identity() {
    let mut core = ExecutionCore::default();
    let response = submit(
        &mut core,
        1,
        1,
        Cmd::Fund {
            asset: Asset::Quote,
            amount: 1_000,
        },
    );
    let effects = response.result.unwrap();
    assert_eq!(
        balance(&core, 1, Asset::Quote),
        Balance {
            available: 1_000,
            reserved: 0
        }
    );
    assert_eq!(core.state().issued(Asset::Quote), 1_000);
    assert_eq!(effects.postings.len(), 1);
    let posting = &effects.postings[0];
    assert_eq!((posting.id.sequence, posting.id.ordinal), (1, 0));
    assert_eq!(
        (posting.from, posting.to, posting.amount),
        (LedgerAccount::Issuance, LedgerAccount::Available(1), 1_000)
    );
    assert_eq!(
        effects.events[0].kind,
        EventKind::Funded {
            account_id: 1,
            asset: Asset::Quote,
            amount: 1_000
        }
    );
}

#[test]
fn funded_partial_fill_retry_cancel_reconciles_exact_balances() {
    let mut core = ExecutionCore::default();
    fund(&mut core, 1, 1, Asset::Quote, 1_000);
    fund(&mut core, 2, 1, Asset::Base, 10);
    assert!(
        place(&mut core, 2, 2, 10, Side::Sell, 100, 10)
            .result
            .is_ok()
    );
    let buy = request(
        1,
        2,
        Cmd::Place {
            order_id: 20,
            side: Side::Buy,
            price: 100,
            quantity: 4,
        },
    );
    let response = core.execute(buy.clone(), Permissions::TRADER).unwrap();
    let effects = response.result.as_ref().unwrap();
    assert_eq!(effects.events.len(), 2);
    assert_eq!(
        effects.events[1].kind,
        EventKind::Order(Event::Trade {
            maker: 10,
            taker: 20,
            price: 100,
            quantity: 4
        })
    );
    assert_eq!(
        (effects.events[1].id.sequence, effects.events[1].id.ordinal),
        (4, 1)
    );
    assert_eq!(effects.postings.len(), 3);
    let committed = core.clone();
    assert_eq!(core.execute(buy, Permissions::TRADER), Ok(response));
    assert_eq!(core, committed);
    assert_eq!(
        balance(&core, 2, Asset::Base),
        Balance {
            available: 0,
            reserved: 6
        }
    );
    assert!(
        submit(&mut core, 2, 3, Cmd::Cancel { order_id: 10 })
            .result
            .is_ok()
    );
    assert_eq!(
        balance(&core, 1, Asset::Base),
        Balance {
            available: 4,
            reserved: 0
        }
    );
    assert_eq!(
        balance(&core, 1, Asset::Quote),
        Balance {
            available: 600,
            reserved: 0
        }
    );
    assert_eq!(
        balance(&core, 2, Asset::Base),
        Balance {
            available: 6,
            reserved: 0
        }
    );
    assert_eq!(
        balance(&core, 2, Asset::Quote),
        Balance {
            available: 400,
            reserved: 0
        }
    );
    assert_eq!(core.state().resting_count(), 0);
    assert_eq!(core.state().ledger().len(), 7);
    core.check_invariants().unwrap();
}

#[test]
fn price_improvement_releases_quote_and_keeps_only_remaining_obligation() {
    let mut core = ExecutionCore::default();
    fund(&mut core, 1, 1, Asset::Quote, 1_000);
    fund(&mut core, 2, 1, Asset::Base, 2);
    place(&mut core, 2, 2, 1, Side::Sell, 90, 2).result.unwrap();
    place(&mut core, 1, 2, 2, Side::Buy, 100, 4).result.unwrap();
    assert_eq!(
        balance(&core, 1, Asset::Quote),
        Balance {
            available: 620,
            reserved: 200
        }
    );
    assert_eq!(balance(&core, 1, Asset::Base).available, 2);
    assert_eq!(balance(&core, 2, Asset::Quote).available, 180);
    submit(&mut core, 1, 3, Cmd::Cancel { order_id: 2 })
        .result
        .unwrap();
    assert_eq!(
        balance(&core, 1, Asset::Quote),
        Balance {
            available: 820,
            reserved: 0
        }
    );
}

#[test]
fn exact_price_improved_fill_matches_sc16() {
    let mut core = ExecutionCore::default();
    fund(&mut core, 1, 1, Asset::Quote, 1_000);
    fund(&mut core, 2, 1, Asset::Base, 4);
    place(&mut core, 2, 2, 1, Side::Sell, 90, 4).result.unwrap();
    place(&mut core, 1, 2, 2, Side::Buy, 100, 4).result.unwrap();
    assert_eq!(
        balance(&core, 1, Asset::Quote),
        Balance {
            available: 640,
            reserved: 0
        }
    );
    assert_eq!(balance(&core, 2, Asset::Quote).available, 360);
}

#[test]
fn incoming_sell_settles_at_resting_bid_and_cancel_releases_buy_remainder() {
    let mut core = ExecutionCore::default();
    fund(&mut core, 1, 1, Asset::Quote, 1_000);
    fund(&mut core, 2, 1, Asset::Base, 4);
    place(&mut core, 1, 2, 1, Side::Buy, 100, 10)
        .result
        .unwrap();
    let response = place(&mut core, 2, 2, 2, Side::Sell, 90, 4);
    assert_eq!(
        response.result.unwrap().events[1].kind,
        EventKind::Order(Event::Trade {
            maker: 1,
            taker: 2,
            price: 100,
            quantity: 4
        })
    );
    assert_eq!(
        balance(&core, 1, Asset::Quote),
        Balance {
            available: 0,
            reserved: 600
        }
    );
    assert_eq!(balance(&core, 2, Asset::Quote).available, 400);
    submit(&mut core, 1, 3, Cmd::Cancel { order_id: 1 })
        .result
        .unwrap();
    assert_eq!(
        balance(&core, 1, Asset::Quote),
        Balance {
            available: 600,
            reserved: 0
        }
    );
}

#[test]
fn multifill_posts_each_asset_and_improvement_at_each_resting_price() {
    let mut core = ExecutionCore::default();
    fund(&mut core, 1, 1, Asset::Quote, 1_000);
    for (account, price) in [(2, 90), (3, 100)] {
        fund(&mut core, account, 1, Asset::Base, 2);
        place(&mut core, account, 2, account, Side::Sell, price, 2)
            .result
            .unwrap();
    }
    let effects = place(&mut core, 1, 2, 1, Side::Buy, 110, 3).result.unwrap();
    assert_eq!(effects.events.len(), 3);
    assert_eq!(effects.postings.len(), 7);
    assert_eq!(
        balance(&core, 1, Asset::Quote),
        Balance {
            available: 720,
            reserved: 0
        }
    );
    assert_eq!(balance(&core, 1, Asset::Base).available, 3);
    assert_eq!(balance(&core, 2, Asset::Quote).available, 180);
    assert_eq!(balance(&core, 3, Asset::Quote).available, 100);
    assert_eq!(balance(&core, 3, Asset::Base).reserved, 1);
}

#[test]
fn no_overspend_and_cached_reject_does_not_change_after_later_funding() {
    let mut core = ExecutionCore::default();
    fund(&mut core, 1, 1, Asset::Quote, 100);
    place(&mut core, 1, 2, 1, Side::Buy, 80, 1).result.unwrap();
    let command = Cmd::Place {
        order_id: 2,
        side: Side::Buy,
        price: 80,
        quantity: 1,
    };
    let original = rejected(&mut core, 1, 3, command.clone(), Reject::InsufficientFunds);
    fund(&mut core, 1, 4, Asset::Quote, 100);
    let before = core.clone();
    assert_eq!(
        core.execute(request(1, 3, command.clone()), Permissions::TRADER),
        Ok(original)
    );
    assert_eq!(core, before);
    submit(&mut core, 1, 5, command).result.unwrap();
    assert_eq!(
        balance(&core, 1, Asset::Quote),
        Balance {
            available: 40,
            reserved: 160
        }
    );
}

#[test]
fn insufficient_base_and_wrong_owner_leave_holds_unchanged() {
    let mut core = ExecutionCore::default();
    fund(&mut core, 1, 1, Asset::Base, 5);
    place(&mut core, 1, 2, 1, Side::Sell, 100, 4)
        .result
        .unwrap();
    rejected(
        &mut core,
        1,
        3,
        Cmd::Place {
            order_id: 2,
            side: Side::Sell,
            price: 100,
            quantity: 2,
        },
        Reject::InsufficientFunds,
    );
    rejected(
        &mut core,
        2,
        1,
        Cmd::Cancel { order_id: 1 },
        Reject::Matching(MatchingReject::NotOwner),
    );
    assert_eq!(
        balance(&core, 1, Asset::Base),
        Balance {
            available: 1,
            reserved: 4
        }
    );
}

#[test]
fn terminal_cancel_and_duplicate_commands_never_post_twice() {
    let mut core = ExecutionCore::default();
    fund(&mut core, 1, 1, Asset::Base, 5);
    place(&mut core, 1, 2, 1, Side::Sell, 100, 5)
        .result
        .unwrap();
    submit(&mut core, 1, 3, Cmd::Cancel { order_id: 1 })
        .result
        .unwrap();
    let count = core.state().ledger().len();
    let response = submit(&mut core, 1, 4, Cmd::Cancel { order_id: 1 });
    assert!(response.result.unwrap().postings.is_empty());
    assert_eq!(core.state().ledger().len(), count);
    let before = core.clone();
    assert_eq!(
        core.execute(
            request(
                1,
                1,
                Cmd::Fund {
                    asset: Asset::Base,
                    amount: 6
                }
            ),
            Permissions::ADMIN
        ),
        Err(AdmissionError::DuplicateConflict)
    );
    assert_eq!(core, before);
}

#[test]
fn selftrade_rejection_rolls_back_preceding_possible_fill_and_reservations() {
    let mut core = ExecutionCore::default();
    for (account, price) in [(1, 90), (2, 100)] {
        fund(&mut core, account, 1, Asset::Base, 1);
        fund(&mut core, account, 2, Asset::Quote, 1_000);
        place(&mut core, account, 3, account, Side::Sell, price, 1)
            .result
            .unwrap();
    }
    rejected(
        &mut core,
        2,
        4,
        Cmd::Place {
            order_id: 3,
            side: Side::Buy,
            price: 100,
            quantity: 2,
        },
        Reject::Matching(MatchingReject::SelfTrade),
    );
}

#[test]
fn funding_and_market_controls_require_admin_even_on_cached_retry() {
    let mut core = ExecutionCore::default();
    let command = request(
        1,
        1,
        Cmd::Fund {
            asset: Asset::Quote,
            amount: 10,
        },
    );
    let before = core.clone();
    assert_eq!(
        core.execute(command.clone(), Permissions::TRADER),
        Err(AdmissionError::Unauthorized)
    );
    assert_eq!(core, before);
    core.execute(command.clone(), Permissions::ADMIN).unwrap();
    let funded = core.clone();
    assert_eq!(
        core.execute(command, Permissions::TRADER),
        Err(AdmissionError::Unauthorized)
    );
    assert_eq!(
        core.execute(
            request(1, 2, Cmd::SetMarketState { open: false }),
            Permissions::TRADER
        ),
        Err(AdmissionError::Unauthorized)
    );
    assert_eq!(core, funded);
}

#[test]
fn invalid_envelopes_and_missing_trade_permission_do_not_consume_sequence() {
    let mut core = ExecutionCore::default();
    for (account, id, permission, expected) in [
        (0, 1, Permissions::TRADER, AdmissionError::InvalidAccountId),
        (1, 0, Permissions::TRADER, AdmissionError::InvalidCommandId),
        (1, 1, Permissions::default(), AdmissionError::Unauthorized),
    ] {
        let before = core.clone();
        assert_eq!(
            core.execute(
                request(account, id, Cmd::Cancel { order_id: 1 }),
                permission
            ),
            Err(expected)
        );
        assert_eq!(core, before);
    }
}

#[test]
fn identical_command_ids_on_different_accounts_are_independent() {
    let mut core = ExecutionCore::default();
    fund(&mut core, 1, 1, Asset::Base, 10);
    fund(&mut core, 2, 1, Asset::Base, 20);
    assert_eq!(core.retained_commands(), 2);
    assert_eq!(balance(&core, 1, Asset::Base).available, 10);
    assert_eq!(balance(&core, 2, Asset::Base).available, 20);
}

#[test]
fn account_capacity_and_invalid_funding_never_create_partial_accounts() {
    let mut core = ExecutionCore::with_limits(Limits {
        max_accounts: 1,
        ..Limits::default()
    })
    .unwrap();
    fund(&mut core, 1, 1, Asset::Quote, 10);
    rejected(
        &mut core,
        2,
        1,
        Cmd::Fund {
            asset: Asset::Base,
            amount: 1,
        },
        Reject::AccountCapacity,
    );
    rejected(
        &mut core,
        3,
        1,
        Cmd::Fund {
            asset: Asset::Base,
            amount: 0,
        },
        Reject::InvalidAmount,
    );
    rejected(
        &mut core,
        3,
        2,
        Cmd::Fund {
            asset: Asset::Base,
            amount: MAX_ACCOUNT_BALANCE + 1,
        },
        Reject::InvalidAmount,
    );
    assert_eq!(core.state().accounts().len(), 1);
}

#[test]
fn account_balance_limit_and_asset_supply_limit_are_checked_atomically() {
    let mut core = ExecutionCore::default();
    for account in 1..=9 {
        fund(&mut core, account, 1, Asset::Quote, MAX_ACCOUNT_BALANCE);
    }
    assert_eq!(core.state().issued(Asset::Quote), MAX_ASSET_SUPPLY);
    rejected(
        &mut core,
        10,
        1,
        Cmd::Fund {
            asset: Asset::Quote,
            amount: 1,
        },
        Reject::SupplyLimit,
    );
    let mut other = ExecutionCore::default();
    fund(&mut other, 1, 1, Asset::Base, MAX_ACCOUNT_BALANCE);
    rejected(
        &mut other,
        1,
        2,
        Cmd::Fund {
            asset: Asset::Base,
            amount: 1,
        },
        Reject::BalanceLimit,
    );
}

#[test]
fn ledger_exhaustion_during_settlement_rolls_back_every_tentative_effect() {
    let mut core = ExecutionCore::with_limits(Limits {
        max_ledger_transfers: 5,
        ..Limits::default()
    })
    .unwrap();
    fund(&mut core, 1, 1, Asset::Quote, 1_000);
    fund(&mut core, 2, 1, Asset::Base, 10);
    place(&mut core, 2, 2, 1, Side::Sell, 90, 10)
        .result
        .unwrap();
    // Reserve and quote settlement fit, but price improvement would exceed the global history limit.
    rejected(
        &mut core,
        1,
        2,
        Cmd::Place {
            order_id: 2,
            side: Side::Buy,
            price: 100,
            quantity: 4,
        },
        Reject::LedgerCapacity,
    );
    assert_eq!(balance(&core, 1, Asset::Quote).available, 1_000);
    assert_eq!(balance(&core, 2, Asset::Base).reserved, 10);
    assert_eq!(core.state().ledger().len(), 3);
}

#[test]
fn recipient_balance_limits_roll_back_quote_and_base_settlement() {
    for saturated_asset in [Asset::Base, Asset::Quote] {
        let mut core = ExecutionCore::default();
        let saturated_account = if saturated_asset == Asset::Base { 1 } else { 2 };
        fund(
            &mut core,
            saturated_account,
            1,
            saturated_asset,
            MAX_ACCOUNT_BALANCE,
        );
        fund(&mut core, 1, 2, Asset::Quote, 100);
        fund(&mut core, 2, 2, Asset::Base, 1);
        place(&mut core, 2, 3, 1, Side::Sell, 100, 1)
            .result
            .unwrap();
        rejected(
            &mut core,
            1,
            3,
            Cmd::Place {
                order_id: 2,
                side: Side::Buy,
                price: 100,
                quantity: 1,
            },
            Reject::BalanceLimit,
        );
    }
}

#[test]
fn output_and_order_capacities_reject_without_financial_effects() {
    let mut core = ExecutionCore::with_limits(Limits {
        max_events: 1,
        ..Limits::default()
    })
    .unwrap();
    fund(&mut core, 1, 1, Asset::Quote, 100);
    fund(&mut core, 2, 1, Asset::Base, 1);
    place(&mut core, 2, 2, 1, Side::Sell, 100, 1)
        .result
        .unwrap();
    rejected(
        &mut core,
        1,
        2,
        Cmd::Place {
            order_id: 2,
            side: Side::Buy,
            price: 100,
            quantity: 1,
        },
        Reject::Matching(MatchingReject::EventCapacity),
    );
    let mut no_orders = ExecutionCore::with_limits(Limits {
        max_orders: 0,
        ..Limits::default()
    })
    .unwrap();
    fund(&mut no_orders, 1, 1, Asset::Quote, 100);
    rejected(
        &mut no_orders,
        1,
        2,
        Cmd::Place {
            order_id: 1,
            side: Side::Buy,
            price: 100,
            quantity: 1,
        },
        Reject::Matching(MatchingReject::OrderCapacity),
    );
}

#[test]
fn receipt_capacity_preserves_retries_and_zero_event_budget_rejects_funding() {
    let mut core = ExecutionCore::with_limits(Limits {
        max_commands: 1,
        ..Limits::default()
    })
    .unwrap();
    let command = request(
        1,
        1,
        Cmd::Fund {
            asset: Asset::Base,
            amount: 1,
        },
    );
    let original = core.execute(command.clone(), Permissions::ADMIN).unwrap();
    let before = core.clone();
    assert_eq!(core.execute(command, Permissions::ADMIN), Ok(original));
    assert_eq!(
        core.execute(
            request(1, 2, Cmd::Cancel { order_id: 1 }),
            Permissions::TRADER
        ),
        Err(AdmissionError::CommandCapacity)
    );
    assert_eq!(core, before);
    let mut no_events = ExecutionCore::with_limits(Limits {
        max_events: 0,
        ..Limits::default()
    })
    .unwrap();
    rejected(
        &mut no_events,
        1,
        1,
        Cmd::Fund {
            asset: Asset::Base,
            amount: 1,
        },
        Reject::EventCapacity,
    );
}

#[test]
fn pause_allows_cancel_and_funding_but_never_reexecutes_a_cached_order() {
    let mut core = ExecutionCore::default();
    fund(&mut core, 1, 1, Asset::Quote, 100);
    let command = Cmd::Place {
        order_id: 1,
        side: Side::Buy,
        price: 100,
        quantity: 1,
    };
    let original = submit(&mut core, 1, 2, command.clone());
    submit(&mut core, 99, 1, Cmd::SetMarketState { open: false })
        .result
        .unwrap();
    rejected(
        &mut core,
        1,
        3,
        Cmd::Place {
            order_id: 2,
            side: Side::Buy,
            price: 100,
            quantity: 1,
        },
        Reject::MarketPaused,
    );
    submit(&mut core, 1, 4, Cmd::Cancel { order_id: 1 })
        .result
        .unwrap();
    fund(&mut core, 2, 1, Asset::Base, 1);
    let before = core.clone();
    assert_eq!(
        core.execute(request(1, 2, command), Permissions::TRADER),
        Ok(original)
    );
    assert_eq!(core, before);
    assert_eq!(core.state().resting_count(), 0);
    submit(&mut core, 99, 2, Cmd::SetMarketState { open: true })
        .result
        .unwrap();
    place(&mut core, 1, 5, 2, Side::Buy, 100, 1).result.unwrap();
}

#[test]
fn invalid_orders_unknown_accounts_and_duplicate_order_ids_are_atomic() {
    let mut core = ExecutionCore::default();
    rejected(
        &mut core,
        1,
        1,
        Cmd::Place {
            order_id: 1,
            side: Side::Buy,
            price: 0,
            quantity: 1,
        },
        Reject::Matching(MatchingReject::InvalidPrice),
    );
    rejected(
        &mut core,
        1,
        2,
        Cmd::Place {
            order_id: 1,
            side: Side::Buy,
            price: 100,
            quantity: 1,
        },
        Reject::UnknownAccount,
    );
    rejected(
        &mut core,
        1,
        3,
        Cmd::Cancel { order_id: 1 },
        Reject::Matching(MatchingReject::UnknownOrder),
    );
    fund(&mut core, 1, 4, Asset::Quote, 100);
    place(&mut core, 1, 5, 1, Side::Buy, 100, 1).result.unwrap();
    rejected(
        &mut core,
        1,
        6,
        Cmd::Place {
            order_id: 1,
            side: Side::Buy,
            price: 100,
            quantity: 1,
        },
        Reject::Matching(MatchingReject::DuplicateOrderId),
    );
}

#[test]
fn maximum_admitted_notional_settles_without_overflow_or_rounding() {
    let mut core = ExecutionCore::default();
    fund(&mut core, 1, 1, Asset::Quote, MAX_ACCOUNT_BALANCE);
    fund(&mut core, 2, 1, Asset::Base, 1_000_000_000);
    place(&mut core, 2, 2, 1, Side::Sell, 1_000_000_000, 1_000_000_000)
        .result
        .unwrap();
    place(&mut core, 1, 2, 2, Side::Buy, 1_000_000_000, 1_000_000_000)
        .result
        .unwrap();
    assert_eq!(
        balance(&core, 1, Asset::Quote),
        Balance {
            available: 0,
            reserved: 0
        }
    );
    assert_eq!(balance(&core, 1, Asset::Base).available, 1_000_000_000);
    assert_eq!(
        balance(&core, 2, Asset::Quote).available,
        MAX_ACCOUNT_BALANCE
    );
}

#[test]
fn changed_order_quantity_conflicts_before_matching_or_reservation() {
    let mut core = ExecutionCore::default();
    fund(&mut core, 1, 1, Asset::Quote, 1_000);
    place(&mut core, 1, 2, 1, Side::Buy, 100, 1).result.unwrap();
    let before = core.clone();
    let changed = request(
        1,
        2,
        Cmd::Place {
            order_id: 1,
            side: Side::Buy,
            price: 100,
            quantity: 2,
        },
    );
    assert_eq!(
        core.execute(changed, Permissions::TRADER),
        Err(AdmissionError::DuplicateConflict)
    );
    assert_eq!(core, before);
    core.check_invariants().unwrap();
}

#[test]
fn funding_limit_includes_reserved_balance_and_preserves_issuance_on_reject() {
    let mut core = ExecutionCore::default();
    fund(&mut core, 1, 1, Asset::Quote, MAX_ACCOUNT_BALANCE);
    place(&mut core, 1, 2, 1, Side::Buy, 1_000_000_000, 1_000_000_000)
        .result
        .unwrap();
    rejected(
        &mut core,
        1,
        3,
        Cmd::Fund {
            asset: Asset::Quote,
            amount: 1,
        },
        Reject::BalanceLimit,
    );
    assert_eq!(
        balance(&core, 1, Asset::Quote),
        Balance {
            available: 0,
            reserved: MAX_ACCOUNT_BALANCE
        }
    );
    assert_eq!(core.state().issued(Asset::Quote), MAX_ACCOUNT_BALANCE);
}
