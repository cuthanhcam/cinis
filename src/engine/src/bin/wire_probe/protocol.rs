//! Provisional fixed-width probe schema, intentionally private to the research binary.
use cinis_engine::{
    AdmissionError, Asset, DecisionSource, ExecutionCommand, ExecutionCore, ExecutionReject,
    ExecutionRequest, Permissions,
};
use std::io::{self, Read};

const MAX_BODY: usize = 128;

pub(super) fn read_frame(input: &mut impl Read) -> io::Result<Option<Vec<u8>>> {
    let mut header = [0; 4];
    match input.read_exact(&mut header[..1]) {
        Err(error) if error.kind() == io::ErrorKind::UnexpectedEof => return Ok(None),
        result => result?,
    }
    input.read_exact(&mut header[1..])?;
    let length = u32::from_be_bytes(header) as usize;
    if length == 0 || length > MAX_BODY {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "frame length"));
    }
    let mut body = vec![0; length];
    input.read_exact(&mut body)?;
    Ok(Some(body))
}

fn number(body: &[u8], offset: usize) -> u64 {
    // All callers first validate the enclosing fixed-width layout.
    u64::from_be_bytes(
        body[offset..offset + 8]
            .try_into()
            .expect("validated offset"),
    )
}

pub(super) fn execute(core: &mut ExecutionCore, body: &[u8]) -> io::Result<Vec<u8>> {
    let (correlation, account, command) = if body.len() >= 26 {
        (number(body, 2), number(body, 10), number(body, 18))
    } else {
        (0, 0, 0)
    };
    let mut kind = 4; // Protocol failure: no execution admission.
    let mut code = if body.len() != 35 {
        5
    } else if body[0] != 1 {
        1
    } else if body[1] != 1 {
        2
    } else if body[26] > 1 {
        3
    } else if correlation == 0 {
        4
    } else {
        0
    };
    let (mut source, mut sequence, mut events, mut postings) = (0, 0u64, 0u32, 0u32);
    if code == 0 {
        let request = ExecutionRequest {
            account_id: account,
            command_id: command,
            command: ExecutionCommand::Fund {
                asset: if body[26] == 0 {
                    Asset::Base
                } else {
                    Asset::Quote
                },
                amount: number(body, 27),
            },
        };
        // The parent owns this synthetic experiment. There is no wire-selected Permissions field or listener.
        match core.execute_with_outcome(request, Permissions::ADMIN) {
            Err(error) => {
                kind = 1;
                code = admission_code(error);
            }
            Ok(outcome) => {
                source = match outcome.source {
                    DecisionSource::New => 1,
                    DecisionSource::Retained => 2,
                };
                sequence = outcome.response.sequence;
                match outcome.response.result {
                    Ok(effects) => {
                        kind = 2;
                        events = u32::try_from(effects.events.len()).map_err(io::Error::other)?;
                        postings =
                            u32::try_from(effects.postings.len()).map_err(io::Error::other)?;
                    }
                    Err(error) => {
                        kind = 3;
                        code = rejection_code(error);
                    }
                }
            }
        }
    }
    core.check_invariants().map_err(io::Error::other)?;
    let balances = core.state().account(account).unwrap_or_default();
    let mut reply = vec![1, 2];
    for value in [correlation, account, command] {
        reply.extend(value.to_be_bytes());
    }
    reply.extend([kind, code, source]);
    reply.extend(sequence.to_be_bytes());
    reply.extend(events.to_be_bytes());
    reply.extend(postings.to_be_bytes());
    // These are current diagnostic balances, NOT part of the immutable historical receipt or a public query API.
    for value in [
        balances.base.available,
        balances.base.reserved,
        balances.quote.available,
        balances.quote.reserved,
    ] {
        reply.extend(value.to_be_bytes());
    }
    Ok(reply)
}

fn admission_code(error: AdmissionError) -> u8 {
    match error {
        AdmissionError::InvalidAccountId => 16,
        AdmissionError::InvalidCommandId => 17,
        AdmissionError::Unauthorized => 18,
        AdmissionError::DuplicateConflict => 19,
        AdmissionError::CommandCapacity => 20,
        AdmissionError::SequenceExhausted => 21,
        AdmissionError::InvalidLimits => 22,
    }
}

fn rejection_code(error: ExecutionReject) -> u8 {
    match error {
        ExecutionReject::InvalidAmount => 32,
        ExecutionReject::BalanceLimit => 33,
        ExecutionReject::SupplyLimit => 34,
        ExecutionReject::AccountCapacity => 35,
        ExecutionReject::LedgerCapacity => 36,
        ExecutionReject::ArithmeticOverflow => 37,
        ExecutionReject::InvariantViolation => 38,
        // Non-funding variants are deliberately not a complete financial error taxonomy in this experiment.
        _ => 63,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn fixture(name: &str) -> Vec<u8> {
        let text = match name {
            "request" => include_str!("../../../../../tests/fixtures/protocol/funding_request.hex"),
            _ => include_str!("../../../../../tests/fixtures/protocol/funding_reply.hex"),
        };
        text.split_whitespace()
            .map(|byte| u8::from_str_radix(byte, 16).unwrap())
            .collect()
    }

    #[test]
    fn golden_u64_funding_and_retained_decision() {
        let mut core = ExecutionCore::default();
        let frame = fixture("request");
        let body = read_frame(&mut Cursor::new(frame)).unwrap().unwrap();
        let reply = execute(&mut core, &body).unwrap();
        assert_eq!(reply, fixture("reply")[4..]);
        let retained = execute(&mut core, &body).unwrap();
        assert_eq!(retained[28], 2);
        assert_eq!(number(&retained, 29), 1);
        assert_eq!(number(&retained, 61), 17);
        assert_eq!(core.next_sequence(), 2);
    }

    #[test]
    fn framing_rejects_bad_lengths_and_truncation_but_accepts_clean_eof() {
        assert!(read_frame(&mut Cursor::new([])).unwrap().is_none());
        for bytes in [
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 129],
            vec![0, 0],
            vec![0, 0, 0, 2, 1],
        ] {
            assert!(read_frame(&mut Cursor::new(bytes)).is_err());
        }
    }

    #[test]
    fn malformed_protocol_cannot_admit_or_fund() {
        let mut core = ExecutionCore::default();
        for length in 1..=128 {
            let reply = execute(&mut core, &vec![0; length]).unwrap();
            assert_eq!(reply[26], 4);
        }
        assert_eq!(core.next_sequence(), 1);
        assert!(core.state().accounts().is_empty());
    }
}
