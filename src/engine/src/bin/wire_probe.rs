//! Isolated synthetic-funding experiment over parent-owned process pipes, never a network service.
#[path = "wire_probe/protocol.rs"]
mod protocol;

use cinis_engine::{ExecutionCore, Limits};
use std::io::{self, Write};

fn run(drop_first: bool) -> io::Result<()> {
    let mut core = ExecutionCore::with_limits(Limits {
        max_accounts: 8,
        max_commands: 32,
        max_ledger_transfers: 32,
        max_orders: 0,
        max_events: 4,
    })
    .map_err(|_| io::Error::other("invalid experiment limits"))?;
    let mut input = io::stdin().lock();
    let mut output = io::stdout().lock();
    for index in 0..64 {
        let Some(payload) = protocol::read_frame(&mut input)? else {
            return Ok(());
        };
        let reply = protocol::execute(&mut core, &payload)?;
        // Explicit research fault: a committed first decision can lose its reply. No state rollback or auto retry.
        if drop_first && index == 0 {
            continue;
        }
        output.write_all(&(reply.len() as u32).to_be_bytes())?;
        output.write_all(&reply)?;
        output.flush()?;
    }
    Err(io::Error::other("experiment request budget exhausted"))
}

fn main() {
    let arguments: Vec<_> = std::env::args().skip(1).collect();
    let drop_first = arguments == ["--stdio-research", "--drop-first-reply"];
    if !(drop_first || arguments == ["--stdio-research"]) {
        eprintln!("Usage: cinis-wire-probe --stdio-research [--drop-first-reply]");
        std::process::exit(2);
    }
    if run(drop_first).is_err() {
        // stdout is exclusively framed bytes; do not print input, credentials or arbitrary error strings.
        eprintln!(
            "Research peer stopped: invalid/incomplete frame, I/O failure or exhausted budget."
        );
        std::process::exit(2);
    }
}
