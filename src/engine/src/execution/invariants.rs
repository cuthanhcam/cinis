//! Full owner audit including command receipts and stable identities.

use super::ExecutionCore;
use crate::ledger::LedgerTransfer;
use std::collections::{BTreeMap, BTreeSet};

impl ExecutionCore {
    /// Audit financial state, retained decisions and event/posting identities.
    ///
    /// # Errors
    /// Returns a diagnostic invariant description if the owned state is inconsistent.
    pub fn check_invariants(&self) -> Result<(), &'static str> {
        self.state.check_invariants()?;
        if self.receipts.len() > self.limits.max_commands {
            return Err("receipt capacity");
        }
        if u64::try_from(self.receipts.len())
            .ok()
            .and_then(|n| n.checked_add(1))
            != Some(self.next_sequence)
        {
            return Err("command sequence gap");
        }
        let mut sequences = BTreeSet::new();
        let mut postings: BTreeMap<(u64, u32), &LedgerTransfer> = self
            .state
            .ledger()
            .iter()
            .map(|posting| ((posting.id.sequence, posting.id.ordinal), posting))
            .collect();
        for (&(account, command), receipt) in &self.receipts {
            let sequence = receipt.response.sequence;
            if account == 0
                || command == 0
                || sequence == 0
                || sequence >= self.next_sequence
                || !sequences.insert(sequence)
            {
                return Err("invalid receipt identity");
            }
            if let Ok(effects) = &receipt.response.result {
                if effects.events.is_empty() || effects.events.len() > self.limits.max_events {
                    return Err("event capacity");
                }
                for (ordinal, event) in effects.events.iter().enumerate() {
                    if event.id.sequence != sequence
                        || usize::try_from(event.id.ordinal).ok() != Some(ordinal)
                    {
                        return Err("event identity");
                    }
                }
                for (ordinal, posting) in effects.postings.iter().enumerate() {
                    if posting.id.sequence != sequence
                        || usize::try_from(posting.id.ordinal).ok() != Some(ordinal)
                        || postings.remove(&(sequence, posting.id.ordinal)) != Some(posting)
                    {
                        return Err("receipt/ledger mismatch");
                    }
                }
            }
        }
        if !postings.is_empty() {
            return Err("ledger transfer without successful receipt");
        }
        Ok(())
    }
}
