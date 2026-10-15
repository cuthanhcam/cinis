# Runbook plan

Status: requirements for M4/M5, not executable procedures. No deployed service exists.

Before v0.1 provide verified startup/shutdown, journal backup, snapshot validation, restore to a separate directory, projection rebuild, and market-data resync commands. Include expected output and failure handling.

Recovery drill: stop admission, preserve storage, validate snapshot/journal, replay offline, compare invariants and cursors, rebuild projections if necessary, reopen ingress. Never overwrite the only journal copy. Committed-history corruption requires halt/investigation, never silent truncation.

Incident reports include revision, workload, durability policy, symptoms, affected sequences, evidence, recovery steps, and follow-up tests. Measure restore time only after a performed drill.
