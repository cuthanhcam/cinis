use cinis_engine::{Command, Engine, Side};

fn main() {
    println!("Cinis matching-only demo: volatile, no financial state or durability");
    let mut engine = Engine::default();
    let commands = [
        Command::Place {
            id: 1,
            account: 1,
            side: Side::Sell,
            price: 100,
            quantity: 10,
        },
        Command::Place {
            id: 2,
            account: 2,
            side: Side::Buy,
            price: 100,
            quantity: 4,
        },
        Command::Cancel { id: 1, account: 1 },
    ];
    for command in commands {
        println!("Command: {command:?}");
        for event in engine.execute(command).expect("valid demo command") {
            println!("  {event:?}");
        }
        engine.check_invariants().expect("matching invariants");
    }
    assert_eq!(engine.resting_count(), 0);
    println!("Final orders: {:?}", engine.orders());
    println!("Book empty; sell filled 4 and cancelled 6.");
}
