//! Ordered matching preflight and mutation for the Vec baseline.

use super::{Command, Engine, Event, MAX_PRICE, MAX_QUANTITY, Order, Reject, Side};
use std::cmp::Reverse;

impl Default for Engine {
    fn default() -> Self {
        Self::new(10_000, 1_024)
    }
}
impl Engine {
    pub fn new(max_orders: usize, max_events: usize) -> Self {
        Self {
            orders: Vec::new(),
            next_sequence: 1,
            max_orders,
            max_events,
        }
    }
    pub fn orders(&self) -> &[Order] {
        &self.orders
    }
    pub fn resting_count(&self) -> usize {
        self.orders.iter().filter(|o| o.remaining > 0).count()
    }
    pub fn execute(&mut self, command: Command) -> Result<Vec<Event>, Reject> {
        match command {
            Command::Place {
                id,
                account,
                side,
                price,
                quantity,
            } => self.place(id, account, side, price, quantity),
            Command::Cancel { id, account } => self.cancel(id, account),
        }
    }
    fn place(
        &mut self,
        id: u64,
        account: u64,
        side: Side,
        price: u64,
        quantity: u64,
    ) -> Result<Vec<Event>, Reject> {
        if id == 0 || account == 0 {
            return Err(Reject::InvalidId);
        }
        if price == 0 || price > MAX_PRICE {
            return Err(Reject::InvalidPrice);
        }
        if quantity == 0 || quantity > MAX_QUANTITY {
            return Err(Reject::InvalidQuantity);
        }
        price
            .checked_mul(quantity)
            .ok_or(Reject::ArithmeticOverflow)?;
        if self.orders.iter().any(|o| o.id == id) {
            return Err(Reject::DuplicateOrderId);
        }
        if self.orders.len() >= self.max_orders {
            return Err(Reject::OrderCapacity);
        }
        let following_sequence = self
            .next_sequence
            .checked_add(1)
            .ok_or(Reject::ArithmeticOverflow)?;

        // Plan the actual fill path before changing any order. Rejects are atomic.
        let mut candidates: Vec<usize> = self
            .orders
            .iter()
            .enumerate()
            .filter(|(_, o)| {
                o.remaining > 0
                    && o.side != side
                    && match side {
                        Side::Buy => o.price <= price,
                        Side::Sell => o.price >= price,
                    }
            })
            .map(|(i, _)| i)
            .collect();
        match side {
            Side::Buy => {
                candidates.sort_by_key(|&i| (self.orders[i].price, self.orders[i].sequence))
            }
            Side::Sell => candidates
                .sort_by_key(|&i| (Reverse(self.orders[i].price), self.orders[i].sequence)),
        }
        let mut remaining = quantity;
        let mut plan = Vec::new();
        for i in candidates {
            if remaining == 0 {
                break;
            }
            let maker = &self.orders[i];
            if maker.account == account {
                return Err(Reject::SelfTrade);
            }
            let q = remaining.min(maker.remaining);
            plan.push((i, q));
            remaining -= q;
        }
        if plan
            .len()
            .checked_add(1)
            .ok_or(Reject::ArithmeticOverflow)?
            > self.max_events
        {
            return Err(Reject::EventCapacity);
        }
        let mut events = Vec::with_capacity(plan.len() + 1);
        events.push(Event::Accepted { id });
        for (i, q) in plan {
            let maker = &mut self.orders[i];
            // q <= remaining, and original <= MAX_QUANTITY: additions cannot overflow.
            maker.remaining -= q;
            maker.filled += q;
            events.push(Event::Trade {
                maker: maker.id,
                taker: id,
                price: maker.price,
                quantity: q,
            });
        }
        self.orders.push(Order {
            id,
            account,
            side,
            price,
            original: quantity,
            remaining,
            filled: quantity - remaining,
            cancelled: 0,
            sequence: self.next_sequence,
        });
        self.next_sequence = following_sequence;
        Ok(events)
    }
    fn cancel(&mut self, id: u64, account: u64) -> Result<Vec<Event>, Reject> {
        if id == 0 || account == 0 {
            return Err(Reject::InvalidId);
        }
        let order = self
            .orders
            .iter_mut()
            .find(|o| o.id == id)
            .ok_or(Reject::UnknownOrder)?;
        if order.account != account {
            return Err(Reject::NotOwner);
        }
        if self.max_events == 0 {
            return Err(Reject::EventCapacity);
        }
        if order.remaining == 0 {
            return Ok(vec![Event::Terminal {
                id,
                status: order.status(),
            }]);
        }
        let quantity = order.remaining;
        order.cancelled = quantity;
        order.remaining = 0;
        Ok(vec![Event::Cancelled { id, quantity }])
    }
}
