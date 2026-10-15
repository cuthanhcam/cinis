# Architecture diagram catalog

Purpose: communicate structure, authority, time ordering, failure boundaries, and expansion gates.
Status: mixed implemented/target views, explicitly labeled below. Owner: project maintainer. Reviewed: 2026-10-15; baseline 6cf0828.

These are C4-inspired context/container/component views plus dynamic and deployment views; they are not a formal C4 tooling export. Arrows mean the labeled interaction. PostgreSQL is never an execution balance authority. [Architecture Atlas](architecture-atlas.html) provides an offline interactive companion. Detailed rules remain in [solution architecture](solution-architecture.md) and the linked contracts.

## D1: System context - target v0.1

```mermaid
flowchart LR
    Researcher[Developer / researcher] -->|fixtures, seeds, reports| Research[Research toolkit]
    Trader[Strategy client / CLI] -->|authenticated commands and queries| Cinis[Cinis synthetic spot sandbox]
    Admin[Sandbox administrator] -->|provision access, fund, pause| Cinis
    Operator[Operator] -->|lifecycle, recovery, diagnostics| Cinis
    Research -->|deterministic workloads| Cinis
    Cinis -->|results, query cursors, market data| Trader
    Cinis -->|evidence and execution history| Research
    Aegis[Optional Aegis identity provider] -.->|future adapter; no hard dependency| Cinis
```

Only the local CLI/library and part of the research verification exist today. Aegis is Deferred. Real venues, custody, and blockchains are outside this system boundary.

## D2: Containers and trust - proposed integrated sandbox

```mermaid
flowchart TB
    Client[Untrusted strategy client]
    subgraph Public[Client-facing boundary]
        Gateway[.NET gateway: authentication, account scopes, bounded admission]
        Query[.NET query / market-data endpoints]
    end
    subgraph Trusted[Restricted host / trusted service boundary]
        Management[.NET management modules]
        Worker[.NET projection worker]
        Engine[Rust ExecutionCore host: sole financial writer]
        Journal[(Engine-owned journal and snapshots)]
        DB[(PostgreSQL: management and projection schemas)]
        Telemetry[Optional telemetry collector / sinks]
    end
    Client -->|HTTPS commands| Gateway
    Client -->|HTTPS queries / later WebSocket| Query
    Gateway -->|account and scope checks| Management
    Management -->|identity/access metadata only| DB
    Gateway -->|bounded framed loopback TCP, correlation| Engine
    Engine -->|result with execution sequence| Gateway
    Engine -->|append/sync before durable reply in M4| Journal
    Journal -->|recoverable ordered history| Worker
    Worker -->|transactional rows plus cursor| DB
    Query -->|derived models plus applied cursor| DB
    Gateway -.->|redacted logs / metrics| Telemetry
    Engine -.->|nonblocking diagnostics| Telemetry
```

Journal-to-worker is a logical recoverable feed, not permission for .NET to parse arbitrary private files. The export API/file feed and versioning are open decisions. Gateway, management, query and worker can start inside one modular .NET process. The collector is optional and absent today.

## D3: Execution internals - implemented M2

```mermaid
flowchart LR
    Caller[Trusted local caller] --> Admission[Identity and permissions]
    Admission --> Receipts[Account / command receipt lookup]
    Receipts -->|same payload| Original[Original historical response]
    Receipts -->|new key and capacity| Sequence[Global command sequence]
    Sequence --> Tentative[Clone tentative execution state]
    subgraph Atomic[Single-owner atomic financial transition]
        Tentative --> Preflight[Validate, reserve, match preflight]
        Preflight --> Matcher[Vec-backed price-time matcher]
        Matcher --> Ledger[Settlement / release ledger transfers]
        Ledger --> Verify[Reconcile balances, holds, ledger]
    end
    Verify -->|success| Commit[Swap financial state]
    Verify -->|business rejection| Keep[Keep original financial state]
    Commit --> Retain[Retain result and advance sequence]
    Keep --> Retain
    Retain --> Reply[Volatile response with stable IDs]
```

This is a responsibility diagram, not exact function-call order for every command. Validation/matching/settlement failures also take the rejection path. Unauthorized/conflicting/capacity admission errors return before allocation of a sequence. Receipt returns require current permission. No external event publisher or persistence is present.

## D4: Submit, lost reply, retry, projection - target M3/M4

```mermaid
sequenceDiagram
    actor Client
    participant G as .NET gateway
    participant E as Rust owner
    participant J as Journal (M4)
    participant P as Projection worker
    participant D as PostgreSQL
    Client->>G: place(account, command ID, payload)
    G->>G: authenticate and authorize account/action
    G->>E: trusted bounded envelope
    E->>E: authorize, deduplicate, admit and sequence
    E->>J: append versioned reproducible record
    J-->>E: tested synchronization boundary
    E->>E: atomic apply, retain result and stable IDs
    E--xG: result lost during disconnect
    Client->>G: retry SAME ID and payload
    G->>E: reauthorized retry
    E-->>G: original result, original sequence
    G-->>Client: original outcome
    J-->>P: recoverable command/event batch
    P->>D: transaction: idempotent effects + cursor
    D-->>P: commit
    Client->>G: query balance
    G->>D: read derived balance and cursor
    D-->>G: balance view with applied sequence
    G-->>Client: derived balance and projection cursor
```

A business rejection can also be a retained outcome. Journal admission facts/configuration and event reconstitution must be specified before M4. A retry returning historical events must not emit another live financial batch. M3 alone remains volatile.

## D5: Ledger movements - accepted zero-fee example

Synthetic initial state: buyer 1,000 quote; seller 10 base. Seller rests 10 at 100, buyer buys 4 at 100, seller cancels the remaining 6. One lot is one base atom; one tick is one quote atom per base atom in this sandbox.

```mermaid
flowchart LR
    Issuance[Synthetic issuance] -->|fund 1000 quote| BuyerQ[Buyer available quote]
    Issuance -->|fund 10 base| SellerB[Seller available base]
    SellerB -->|reserve sell 10 base| SellerR[Seller reserved base]
    BuyerQ -->|reserve buy 400 quote| BuyerR[Buyer reserved quote]
    BuyerR -->|fill: transfer 400 quote| SellerQ[Seller available quote]
    SellerR -->|fill: transfer 4 base| BuyerB[Buyer available base]
    SellerR -->|cancel: release 6 base| SellerB
```

Final buyer base 4/quote 600, seller base 6/quote 400; both reserved balances zero, book empty. Seven transfers, each with balanced per-asset source/destination legs. Funding uses issuance as the explicit counterparty. Price-improved buys additionally release excess quote; this example has no price improvement. See [ledger](../ledger.md) for the canonical posting model.

## D6: Order state machine - accepted business model

```mermaid
stateDiagram-v2
    [*] --> Submitted
    Submitted --> Rejected: preflight/business failure; no financial mutation
    Submitted --> Open: accepted remainder, no fills
    Submitted --> PartiallyFilled: fills with remainder
    Submitted --> Filled: all quantity executed
    Open --> PartiallyFilled: later incoming fill
    Open --> Filled: later complete fill
    PartiallyFilled --> PartiallyFilled: later fill leaves remainder
    PartiallyFilled --> Filled: remaining quantity executed
    Open --> Cancelled: owner cancel; release hold
    PartiallyFilled --> Cancelled: owner cancel remaining; release hold
    Rejected --> [*]
    Filled --> [*]
    Cancelled --> [*]
```

Submitted is a conceptual request stage, not a persisted order state in the current API. Admission errors are outside this state machine. Unknown cancel rejects; an owned terminal order produces its terminal response without new effects. No amendment/reopen exists. Pausing gates new placement, not already-final fills; cancels remain policy-permitted subject to accepted capacity limits.

## D7: Recovery and publication - intended M4

```mermaid
flowchart TD
    Start[Start: ingress closed] --> Snapshot[Select compatible verified snapshot]
    Snapshot --> History[Validate journal framing, versions, integrity and sequence]
    History --> Tail{Incomplete proven uncommitted tail only?}
    Tail -->|yes| Repair[Apply documented tail recovery rule; retain evidence]
    Tail -->|no incomplete tail| Replay[Replay committed suffix deterministically]
    Repair --> Replay
    History -->|committed corruption / incompatible semantics| Halt[Fail closed; preserve files]
    Replay --> Restore[Restore financial state, receipts, IDs, counters]
    Restore --> Check[Reconcile invariants and recovered cursor]
    Check -->|pass| Ready[Open ingress; consumers catch up from durable history]
    Check -->|fail| Halt
```

No snapshot can mean replay from a valid genesis/configuration record; an invalid snapshot must not silently hide committed corruption. Storage checksums detect corruption, not malicious tampering. Recovery never silently resets balances or drops committed receipts. All of this remains unimplemented.

## D8: Deployment - current versus target

```mermaid
flowchart TB
    subgraph Current[Current developer machine]
        Ops[ops scripts / logged tasks] --> CLI[Rust CLI + ExecutionCore, one process]
        Fixtures[Versioned fixtures and seeds] --> CLI
        CLI --> Logs[Ignored diagnostic artifacts]
    end
    subgraph Target[Proposed one-host integrated sandbox]
        Runner[Native ops or future Compose] --> Dotnet[.NET modular host]
        Runner --> Rust[Rust execution service]
        Runner --> Postgres[PostgreSQL instance]
        Dotnet -->|restricted loopback| Rust
        Dotnet -->|separate schema roles| Postgres
        Rust --> Volume[Engine-owned persistent history volume, M4]
        Dotnet -.-> Sink[Optional observability sink]
        Rust -.-> Sink
    end
```

HTTPS termination, ports, secrets provider, storage paths, OS support and backup policy must be decided with actual services. The current log runner does not provide persistent-service readiness, restart supervision or storage recovery. A single host has a single-host failure domain; it is not HA.

## D9: Evolution gates - capability chart, not calendar schedule

```mermaid
flowchart LR
    M0[M0: semantics and protocol discovery] --> M3[M3: trusted end-to-end gateway]
    M1[M1: matching correctness, implemented] --> M2[M2: volatile financial core, implemented]
    M2 --> M3
    M2 --> M4[M4: durable replay and crash evidence]
    M3 --> M5[M5: recoverable market data and service operations]
    M4 --> M5
    M5 --> M6[M6: reproducible full demo and measured baseline]
    M6 -.-> Optimize[Measured collection / allocation experiments]
    M6 -.-> Expand[Multi-market / replication / external settlement research]
```

M0 remains open for cross-language discovery despite M1/M2 implementation. M3 network results cannot be advertised as durable before M4. Deferred extensions need their own business and authority review. [Backlog](../../delivery/backlog.md) is canonical; this chart makes no dates or effort promises.

## D10: Logical domain relationships - authority model, not a SQL schema

```mermaid
erDiagram
    PRINCIPAL ||--o{ ACCOUNT_GRANT : receives
    ACCOUNT ||--o{ ACCOUNT_GRANT : authorizes
    PRINCIPAL ||--o{ API_KEY : authenticates_with
    ACCOUNT ||--o{ COMMAND_RECEIPT : scopes_identity
    ACCOUNT ||--o{ ORDER : owns
    MARKET ||--o{ ORDER : accepts
    ORDER ||--o| RESERVATION : requires_when_active
    ACCOUNT ||--o{ ASSET_BALANCE : holds
    ORDER ||--o{ TRADE_PARTICIPATION : fills
    TRADE ||--|{ TRADE_PARTICIPATION : has_two_sides
    COMMAND_RECEIPT ||--o{ EXECUTION_EVENT : identifies
    COMMAND_RECEIPT ||--o{ LEDGER_TRANSFER : produces
    LEDGER_TRANSFER ||--|{ TRANSFER_LEG : has_balanced_pair
    ASSET ||--o{ TRANSFER_LEG : denominates
    ASSET ||--o{ ASSET_BALANCE : denominates
    ASSET ||--o{ RESERVATION : denominates
    COMMAND_RECEIPT {
        uint64 account_id PK
        uint64 command_id PK
        uint64 execution_sequence
        typed_payload original_command
        result original_result
    }
    LEDGER_TRANSFER {
        uint64 execution_sequence PK
        uint32 posting_ordinal PK
        asset asset
        uint64 amount_atoms
        ledger_account from_account
        ledger_account to_account
    }
```

PRINCIPAL/API_KEY/ACCOUNT_GRANT are proposed management concepts, not existing tables. Account grants/revocation still need BD-01/02 review. Rust owns orders, balances, holds, execution receipts/events and ledger transfers; their PostgreSQL equivalents are derived projections. The model's ACCOUNT is the stable account identity shared across the boundary, not a shared writable financial row. The accepted engine represents a transfer as source/destination fields; TRANSFER_LEG explains balanced accounting rather than claiming a separate implemented entity. Issuance is a ledger counterparty, not a normal customer account. Trades have exactly two order participations in the business model; Mermaid cardinality alone does not enforce that constraint. See [state ownership](../state-ownership.md) and [ledger](../ledger.md).
