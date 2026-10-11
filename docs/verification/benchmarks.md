# Benchmark specification

Status: methodology only; no measurements or latency promises.

Measure separately: parsed command to core transition; journal admission to durable result; client send to response. State whether risk, serialization, network, storage sync, and event delivery are included.

Workloads: resting insertion, cancel, single fill, partial fill, many fills. Record depth, accounts, sizes, offered load, seeds, capacity, warm-up, interval, rejection/backpressure counts, throughput and p50/p99/p99.9. Include queueing and timeout outcomes; avoid a load model that hides delays by waiting for every preceding response.

Record commit, toolchains, build profile, CPU/RAM/OS, power settings, affinity if any, native/container topology, exact durability policy, raw data, and reproduction commands. Verify optimized output against the reference first.

No-allocation work is later research. Claims must specify measured path, warm-up, capacity, and instrumentation. Core latency does not represent system latency.
