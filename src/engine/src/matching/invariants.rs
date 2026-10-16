//! Full matching-state audit, independent of command mutation.

use super::{Engine, MAX_PRICE, MAX_QUANTITY, Side};

impl Engine {
    /// Check logical matching invariants; financial invariants require the M2 layer.
    pub fn check_invariants(&self) -> Result<(), &'static str> {
        for (i, o) in self.orders.iter().enumerate() {
            if o.original
                != o.filled
                    .checked_add(o.remaining)
                    .and_then(|q| q.checked_add(o.cancelled))
                    .ok_or("quantity overflow")?
            {
                return Err("quantity conservation");
            }
            if o.original == 0
                || o.price == 0
                || o.price > MAX_PRICE
                || o.original > MAX_QUANTITY
                || o.id == 0
                || o.account == 0
            {
                return Err("invalid order");
            }
            if o.cancelled > 0 && o.remaining > 0 {
                return Err("cancelled order rests");
            }
            if o.sequence >= self.next_sequence {
                return Err("invalid acceptance sequence");
            }
            for other in &self.orders[i + 1..] {
                if o.id == other.id || o.sequence == other.sequence {
                    return Err("duplicate identity/priority");
                }
                if o.remaining > 0 && other.remaining > 0 && o.side != other.side {
                    let (bid, ask) = if o.side == Side::Buy {
                        (o.price, other.price)
                    } else {
                        (other.price, o.price)
                    };
                    if bid >= ask {
                        return Err("crossed resting book");
                    }
                }
            }
        }
        if self.orders.len() > self.max_orders {
            return Err("order capacity");
        }
        Ok(())
    }
}
