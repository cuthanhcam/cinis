//! Independent map-based, speculative-transaction oracle for the Vec preflight matcher.
use cinis_engine::{Command, Engine, Event, Order, Reject, Side, Status};
use std::collections::BTreeMap;

#[derive(Clone)]
struct Model {
    orders: BTreeMap<u64, Order>,
    // Side 0: bids with reversed price; side 1: asks with ordinary price.
    book: BTreeMap<(u8, u64, u64), u64>,
    sequence: u64,
    max_orders: usize,
    max_events: usize,
}
impl Model {
    fn new(max_orders: usize, max_events: usize) -> Self {
        Self {
            orders: BTreeMap::new(),
            book: BTreeMap::new(),
            sequence: 1,
            max_orders,
            max_events,
        }
    }
    fn key(order: &Order) -> (u8, u64, u64) {
        match order.side {
            Side::Buy => (0, u64::MAX - order.price, order.sequence),
            Side::Sell => (1, order.price, order.sequence),
        }
    }
    fn snapshot(&self) -> Vec<Order> {
        let mut records: Vec<Order> = self.orders.values().cloned().collect();
        records.sort_by_key(|o| o.sequence);
        records
    }
    fn execute(&mut self, command: Command) -> Result<Vec<Event>, Reject> {
        // Unlike the core's preflight plan, mutate a speculative transaction and roll it back on error.
        let mut transaction = self.clone();
        let result = transaction.apply(command);
        if result.is_ok() {
            *self = transaction;
        }
        result
    }
    fn apply(&mut self, command: Command) -> Result<Vec<Event>, Reject> {
        match command {
            Command::Cancel { id, account } => {
                if id == 0 || account == 0 {
                    return Err(Reject::InvalidId);
                }
                let order = self.orders.get_mut(&id).ok_or(Reject::UnknownOrder)?;
                if order.account != account {
                    return Err(Reject::NotOwner);
                }
                if self.max_events == 0 {
                    return Err(Reject::EventCapacity);
                }
                if order.remaining == 0 {
                    let status = if order.cancelled == 0 {
                        Status::Filled
                    } else {
                        Status::Cancelled
                    };
                    return Ok(vec![Event::Terminal { id, status }]);
                }
                let quantity = order.remaining;
                self.book.remove(&Self::key(order));
                order.remaining = 0;
                order.cancelled = quantity;
                Ok(vec![Event::Cancelled { id, quantity }])
            }
            Command::Place {
                id,
                account,
                side,
                price,
                quantity,
            } => {
                if id == 0 || account == 0 {
                    return Err(Reject::InvalidId);
                }
                if !(1..=1_000_000_000).contains(&price) {
                    return Err(Reject::InvalidPrice);
                }
                if !(1..=1_000_000_000).contains(&quantity) {
                    return Err(Reject::InvalidQuantity);
                }
                if self.orders.contains_key(&id) {
                    return Err(Reject::DuplicateOrderId);
                }
                if self.orders.len() == self.max_orders {
                    return Err(Reject::OrderCapacity);
                }
                let next = self
                    .sequence
                    .checked_add(1)
                    .ok_or(Reject::ArithmeticOverflow)?;
                let mut incoming = Order {
                    id,
                    account,
                    side,
                    price,
                    original: quantity,
                    remaining: quantity,
                    filled: 0,
                    cancelled: 0,
                    sequence: self.sequence,
                };
                let opposite = if side == Side::Buy { 1 } else { 0 };
                let mut events = vec![Event::Accepted { id }];
                while incoming.remaining != 0 {
                    let best = self
                        .book
                        .range((opposite, 0, 0)..=(opposite, u64::MAX, u64::MAX))
                        .next()
                        .map(|(key, maker)| (*key, *maker));
                    let Some((key, maker_id)) = best else {
                        break;
                    };
                    let maker = self.orders.get_mut(&maker_id).unwrap();
                    let crosses = match side {
                        Side::Buy => price >= maker.price,
                        Side::Sell => price <= maker.price,
                    };
                    if !crosses {
                        break;
                    }
                    if maker.account == account {
                        return Err(Reject::SelfTrade);
                    }
                    let traded = incoming.remaining.min(maker.remaining);
                    incoming.remaining -= traded;
                    incoming.filled += traded;
                    maker.remaining -= traded;
                    maker.filled += traded;
                    events.push(Event::Trade {
                        maker: maker_id,
                        taker: id,
                        price: maker.price,
                        quantity: traded,
                    });
                    if maker.remaining == 0 {
                        self.book.remove(&key);
                    }
                }
                if events.len() > self.max_events {
                    return Err(Reject::EventCapacity);
                }
                if incoming.remaining > 0 {
                    self.book.insert(Self::key(&incoming), id);
                }
                self.orders.insert(id, incoming);
                self.sequence = next;
                Ok(events)
            }
        }
    }
}

// SplitMix64: reproducible fixed algorithm, no dependency on platform RNG or test execution order.
struct Generator(u64);
impl Generator {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut value = self.0;
        value = (value ^ (value >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94d049bb133111eb);
        value ^ (value >> 31)
    }
    fn command(&mut self, step: u64) -> Command {
        let choice = self.next();
        let id = match choice % 16 {
            0 => 0,
            1..=4 => self.next() % 128 + 1,
            _ => step + 1,
        };
        let account = if choice.is_multiple_of(23) {
            0
        } else {
            self.next() % 6 + 1
        };
        if choice.is_multiple_of(4) {
            return Command::Cancel { id, account };
        }
        let price = match self.next() % 20 {
            0 => 0,
            1 => 1_000_000_001,
            2 => 1_000_000_000,
            _ => 90 + self.next() % 21,
        };
        let quantity = match self.next() % 20 {
            0 => 0,
            1 => 1_000_000_001,
            2 => 1_000_000_000,
            _ => self.next() % 12 + 1,
        };
        Command::Place {
            id,
            account,
            side: if self.next().is_multiple_of(2) {
                Side::Buy
            } else {
                Side::Sell
            },
            price,
            quantity,
        }
    }
}

fn compare(seed: u64, limits: (usize, usize), count: u64) {
    let mut generator = Generator(seed);
    let mut engine = Engine::new(limits.0, limits.1);
    let mut repeated = Engine::new(limits.0, limits.1);
    let mut model = Model::new(limits.0, limits.1);
    for step in 0..count {
        let command = generator.command(step);
        let before = engine.clone();
        let result = engine.execute(command.clone());
        let expected = model.execute(command.clone());
        assert_eq!(
            result, expected,
            "seed={seed} step={step} limits={limits:?} command={command:?}"
        );
        assert_eq!(
            engine.orders(),
            model.snapshot(),
            "seed={seed} step={step} state"
        );
        assert_eq!(
            result,
            repeated.execute(command.clone()),
            "repeat seed={seed} step={step}"
        );
        assert_eq!(engine, repeated, "repeat state seed={seed} step={step}");
        if result.is_err() {
            assert_eq!(engine, before, "reject seed={seed} step={step}");
        }
        engine
            .check_invariants()
            .unwrap_or_else(|error| panic!("seed={seed} step={step}: {error}"));
    }
}
macro_rules! seeded {
    ($name:ident, $seed:expr) => {
        #[test]
        fn $name() {
            for limits in [(0, 32), (96, 0), (96, 2), (256, 32)] {
                compare($seed, limits, 256);
            }
        }
    };
}
seeded!(seed_1, 1);
seeded!(seed_7, 7);
seeded!(seed_42, 42);
seeded!(seed_2026, 2026);
seeded!(seed_65535, 65535);
seeded!(seed_123456789, 123456789);
seeded!(seed_high_bit, 1_u64 << 63);
seeded!(seed_max, u64::MAX);

#[test]
fn reference_cancel_and_terminal_transitions() {
    let mut engine = Engine::default();
    let mut model = Model::new(10_000, 1_024);
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
        Command::Cancel { id: 1, account: 2 },
        Command::Cancel { id: 1, account: 1 },
        Command::Cancel { id: 1, account: 1 },
        Command::Cancel { id: 2, account: 2 },
        Command::Cancel { id: 99, account: 1 },
        Command::Place {
            id: 1,
            account: 1,
            side: Side::Buy,
            price: 100,
            quantity: 1,
        },
    ];
    for command in commands {
        assert_eq!(engine.execute(command.clone()), model.execute(command));
        assert_eq!(engine.orders(), model.snapshot());
        engine.check_invariants().unwrap();
    }
}
