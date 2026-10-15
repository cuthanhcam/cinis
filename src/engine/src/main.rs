use cinis_engine::{
    Asset, Balance, ExecutionCommand as Command, ExecutionCore, ExecutionRequest, Permissions, Side,
};

fn main() {
    println!(
        "Cinis financial execution demo: synthetic assets, volatile state, no recovery guarantee"
    );
    let mut core = ExecutionCore::default();
    let requests = [
        ExecutionRequest {
            account_id: 1,
            command_id: 1,
            command: Command::Fund {
                asset: Asset::Quote,
                amount: 1_000,
            },
        },
        ExecutionRequest {
            account_id: 2,
            command_id: 1,
            command: Command::Fund {
                asset: Asset::Base,
                amount: 10,
            },
        },
        ExecutionRequest {
            account_id: 2,
            command_id: 2,
            command: Command::Place {
                order_id: 10,
                side: Side::Sell,
                price: 100,
                quantity: 10,
            },
        },
        ExecutionRequest {
            account_id: 1,
            command_id: 2,
            command: Command::Place {
                order_id: 20,
                side: Side::Buy,
                price: 100,
                quantity: 4,
            },
        },
    ];
    for request in &requests {
        println!("Request: {request:?}");
        let response = core
            .execute(request.clone(), Permissions::ADMIN)
            .expect("admitted demo request");
        println!(
            "Sequence {}: {:?}",
            response.sequence,
            response.result.as_ref().expect("successful demo command")
        );
        core.check_invariants().expect("execution invariants");
    }
    let before_retry = core.clone();
    let retry = core
        .execute(requests[3].clone(), Permissions::TRADER)
        .expect("authorized retry");
    assert_eq!(core, before_retry);
    println!(
        "Retry returned original sequence {}; no additional financial effect.",
        retry.sequence
    );
    let cancel = ExecutionRequest {
        account_id: 2,
        command_id: 3,
        command: Command::Cancel { order_id: 10 },
    };
    core.execute(cancel, Permissions::TRADER)
        .expect("admitted cancellation")
        .result
        .expect("successful cancellation");
    core.check_invariants()
        .expect("reconciled ledger and holds");
    let buyer = core.state().account(1).expect("buyer");
    let seller = core.state().account(2).expect("seller");
    assert_eq!(
        buyer.base,
        Balance {
            available: 4,
            reserved: 0
        }
    );
    assert_eq!(
        buyer.quote,
        Balance {
            available: 600,
            reserved: 0
        }
    );
    assert_eq!(
        seller.base,
        Balance {
            available: 6,
            reserved: 0
        }
    );
    assert_eq!(
        seller.quote,
        Balance {
            available: 400,
            reserved: 0
        }
    );
    assert_eq!(core.state().resting_count(), 0);
    println!("Buyer: {buyer:?}");
    println!("Seller: {seller:?}");
    println!(
        "Ledger transfers: {}; base supply: {}; quote supply: {}",
        core.state().ledger().len(),
        core.state().issued(Asset::Base),
        core.state().issued(Asset::Quote)
    );
    println!(
        "Book empty; balances and reservations reconcile. Restart recovery is not implemented."
    );
}
