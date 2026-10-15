//! Bounded generated command streams: externally tracked supply, atomic rejects, retries, determinism.
use cinis_engine::{
    AdmissionError, Asset, ExecutionCommand as Cmd, ExecutionCore, ExecutionRequest, Limits,
    Permissions, Side,
};

struct Generator(u64);
impl Generator {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut value = self.0;
        value = (value ^ (value >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94d049bb133111eb);
        value ^ (value >> 31)
    }
}
fn verify_supply(core: &ExecutionCore, expected: [u128; 2]) {
    for (asset, issued) in [(Asset::Base, expected[0]), (Asset::Quote, expected[1])] {
        let total: u128 = core
            .state()
            .accounts()
            .values()
            .map(|account| {
                let balance = account.get(asset);
                u128::from(balance.available) + u128::from(balance.reserved)
            })
            .sum();
        assert_eq!(
            total, issued,
            "external synthetic funding must equal account totals"
        );
        assert_eq!(u128::from(core.state().issued(asset)), issued);
    }
    core.check_invariants()
        .expect("ledger, reservations, quantity, sequence and receipt invariants");
}
fn run(seed: u64) {
    let limits = Limits {
        max_orders: 96,
        max_ledger_transfers: 1_000,
        max_commands: 512,
        ..Limits::default()
    };
    let mut core = ExecutionCore::with_limits(limits).unwrap();
    let mut repeated = ExecutionCore::with_limits(limits).unwrap();
    let mut generator = Generator(seed);
    let mut supply = [0_u128; 2];
    for account in 1..=4 {
        for (id, asset, amount) in [(1, Asset::Base, 100), (2, Asset::Quote, 100_000)] {
            let request = ExecutionRequest {
                account_id: account,
                command_id: id,
                command: Cmd::Fund { asset, amount },
            };
            assert_eq!(
                core.execute(request.clone(), Permissions::ADMIN),
                repeated.execute(request, Permissions::ADMIN)
            );
            supply[if asset == Asset::Base { 0 } else { 1 }] += u128::from(amount);
            verify_supply(&core, supply);
        }
    }
    let mut successful_places = 0;
    let mut trades = 0;
    let mut cancellations = 0;
    for step in 0..256_u64 {
        let account = generator.next() % 4 + 1;
        let choice = generator.next();
        let command = match choice % 13 {
            0 => Cmd::Fund {
                asset: if generator.next().is_multiple_of(2) {
                    Asset::Base
                } else {
                    Asset::Quote
                },
                amount: generator.next() % 8,
            },
            1 => Cmd::SetMarketState {
                open: !generator.next().is_multiple_of(3),
            },
            2..=4 => {
                let owned = core
                    .state()
                    .orders()
                    .iter()
                    .find(|order| order.account == account && order.remaining > 0);
                Cmd::Cancel {
                    order_id: owned.map_or(generator.next() % 96 + 1, |order| order.id),
                }
            }
            _ => Cmd::Place {
                order_id: if choice.is_multiple_of(7) {
                    generator.next() % 96 + 1
                } else {
                    step + 1
                },
                side: if generator.next().is_multiple_of(2) {
                    Side::Buy
                } else {
                    Side::Sell
                },
                price: if choice.is_multiple_of(19) {
                    0
                } else {
                    90 + generator.next() % 21
                },
                quantity: generator.next() % 9,
            },
        };
        let permissions = if choice.is_multiple_of(17) {
            Permissions::default()
        } else {
            Permissions::ADMIN
        };
        let request = ExecutionRequest {
            account_id: account,
            command_id: 1_000 + step,
            command: command.clone(),
        };
        let before = core.clone();
        let response = core.execute(request.clone(), permissions);
        assert_eq!(
            response,
            repeated.execute(request.clone(), permissions),
            "seed={seed} step={step}"
        );
        assert_eq!(
            core, repeated,
            "deterministic state seed={seed} step={step}"
        );
        match &response {
            Err(_) => assert_eq!(
                core, before,
                "admission must not change state seed={seed} step={step}"
            ),
            Ok(result) if result.result.is_err() => assert_eq!(
                core.state(),
                before.state(),
                "business rollback seed={seed} step={step}"
            ),
            Ok(result) => {
                if let Cmd::Fund { asset, amount } = command {
                    supply[if asset == Asset::Base { 0 } else { 1 }] += u128::from(amount);
                }
                for event in &result.result.as_ref().unwrap().events {
                    match event.kind {
                        cinis_engine::EventKind::Order(cinis_engine::Event::Accepted {
                            ..
                        }) => successful_places += 1,
                        cinis_engine::EventKind::Order(cinis_engine::Event::Trade { .. }) => {
                            trades += 1
                        }
                        cinis_engine::EventKind::Order(cinis_engine::Event::Cancelled {
                            ..
                        }) => cancellations += 1,
                        _ => {}
                    }
                }
            }
        }
        verify_supply(&core, supply);
        if step.is_multiple_of(7) {
            let committed = core.clone();
            assert_eq!(
                core.execute(request.clone(), permissions),
                response,
                "retry seed={seed} step={step}"
            );
            assert_eq!(core, committed);
        }
        if step.is_multiple_of(11) && response.is_ok() {
            let committed = core.clone();
            // Different command variant guarantees a different canonical payload; admin allows either.
            let changed = ExecutionRequest {
                command: match command {
                    Cmd::SetMarketState { .. } => Cmd::Fund {
                        asset: Asset::Base,
                        amount: 1,
                    },
                    _ => Cmd::SetMarketState { open: true },
                },
                ..request
            };
            assert_eq!(
                core.execute(changed, Permissions::ADMIN),
                Err(AdmissionError::DuplicateConflict)
            );
            assert_eq!(core, committed);
        }
    }
    assert!(
        successful_places > 0 && trades > 0 && cancellations > 0,
        "corpus must execute transitions seed={seed}"
    );
}
#[test]
fn financial_seed_11() {
    run(11);
}
#[test]
fn financial_seed_29() {
    run(29);
}
#[test]
fn financial_seed_2026() {
    run(2026);
}
#[test]
fn financial_seed_3735928559() {
    run(3_735_928_559);
}
