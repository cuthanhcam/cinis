use cinis_engine::{Command, Engine, Event, Side};

fn render(event: &Event) -> String {
    match event {
        Event::Accepted { id } => format!("A {id}"),
        Event::Trade {
            maker,
            taker,
            price,
            quantity,
        } => format!("T {maker} {taker} {price} {quantity}"),
        Event::Cancelled { id, quantity } => format!("C {id} {quantity}"),
        Event::Terminal { id, status } => format!("X {id} {status:?}"),
    }
}
fn run(fixture: &str) -> Engine {
    let mut engine = Engine::default();
    let mut expected_state = Vec::new();
    let mut started = false;
    for (line_number, raw) in fixture.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let words: Vec<&str> = line.split_whitespace().collect();
        if words[0] == "LIMITS" {
            assert!(!started, "limits must precede commands");
            assert_eq!(words.len(), 3);
            engine = Engine::new(words[1].parse().unwrap(), words[2].parse().unwrap());
            continue;
        }
        if words[0] == "S" {
            expected_state.push(line.to_owned());
            continue;
        }
        started = true;
        assert!(
            expected_state.is_empty(),
            "final-state rows must follow commands"
        );
        let (input, expected) = line.split_once(" => ").expect("command expectation");
        let w: Vec<&str> = input.split_whitespace().collect();
        let n = |i: usize| w[i].parse::<u64>().expect("integer fixture field");
        let command = match w[0] {
            "P" => {
                assert_eq!(w.len(), 6);
                Command::Place {
                    id: n(1),
                    account: n(2),
                    side: match w[3] {
                        "B" => Side::Buy,
                        "S" => Side::Sell,
                        _ => panic!("side"),
                    },
                    price: n(4),
                    quantity: n(5),
                }
            }
            "C" => {
                assert_eq!(w.len(), 3);
                Command::Cancel {
                    id: n(1),
                    account: n(2),
                }
            }
            _ => panic!("unknown fixture command"),
        };
        let before = engine.clone();
        let actual = match engine.execute(command) {
            Ok(events) => events.iter().map(render).collect::<Vec<_>>().join(" | "),
            Err(error) => {
                assert_eq!(
                    engine,
                    before,
                    "rejection must be atomic at line {}",
                    line_number + 1
                );
                format!("! {error:?}")
            }
        };
        assert_eq!(actual, expected, "fixture line {}", line_number + 1);
        engine
            .check_invariants()
            .expect("invariants after every command");
    }
    let actual_state: Vec<String> = engine
        .orders()
        .iter()
        .map(|o| {
            format!(
                "S {} {} {:?} {} {} {} {} {} {:?}",
                o.id,
                o.account,
                o.side,
                o.price,
                o.original,
                o.remaining,
                o.filled,
                o.cancelled,
                o.status()
            )
        })
        .collect();
    assert_eq!(actual_state, expected_state, "complete final state");
    engine
}
macro_rules! fixture {
    ($name:ident) => {
        #[test]
        fn $name() {
            let data = include_str!(concat!(
                "../../../tests/fixtures/matching/",
                stringify!($name),
                ".txt"
            ));
            assert_eq!(run(data), run(data), "repeat must reproduce logical state");
        }
    };
}
fixture!(mc01_partial_cancel);
fixture!(mc02_exact_fill);
fixture!(mc03_noncrossing);
fixture!(mc04_price_multifill);
fixture!(mc05_ask_fifo);
fixture!(mc06_bid_priority);
fixture!(mc07_taker_remainder);
fixture!(mc08_cancel_terminal);
fixture!(mc09_invalid_units);
fixture!(mc10_duplicate_id);
fixture!(mc11_cancel_owner);
fixture!(mc12_selftrade_atomic);
fixture!(mc13_selftrade_unreached);
fixture!(mc14_order_capacity);
fixture!(mc15_event_capacity);
fixture!(mc16_bid_fifo);
fixture!(mc17_zero_capacity);
