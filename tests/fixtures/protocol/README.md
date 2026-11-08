# Funding protocol probe fixtures

Status: Research-only golden bytes. These are not accepted public financial contracts.

funding_request.hex and funding_reply.hex contain whitespace-separated hexadecimal bytes including the four-byte big-endian length header. The request uses correlation 0x0102030405060708, account 9007199254740993, command 18446744073709551615, quote asset and amount 17. Both Rust unit tests and the .NET-owned process harness consume the same fixtures and compare exact bytes.

The reply describes a new successful decision at sequence 1, one event/posting, quote available 17 and zero other balances. See the [research report/schema](../../../research/protocol-spike/README.md). When changing the provisional schema, review both encoders and the independent fixture bytes together; do not regenerate expected values from the implementation under test.
