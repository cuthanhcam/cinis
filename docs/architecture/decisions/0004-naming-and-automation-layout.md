# ADR-0004: Language-aware names and one automation tree

Date: 2026-10-15. Status: Accepted. Supersedes ADR-0003 for documentation-topic naming and root scripts placement; its process/logging design remains applicable.

## Context and options

The maintainer wants short top-level automation naming, clear documentation-topic names, and idiomatic conventions for a multi-language project. A single casing rule would conflict with C#, Rust, Python, and PowerShell conventions. Keeping root scripts/ alongside ops/scripts/ splits repository automation.

## Decision

Keep ops/ for automation, restore docs/operations/ for operational documentation, and move the portable checker from scripts/check_docs.py to ops/scripts/check_docs.py. No root scripts/ directory remains. Use the naming-conventions.md policy: shared path defaults, tool-required exceptions, language-specific names, and explicit wire-contract mapping.

## Consequences and verification

Update runner registry, CI, documentation links, and script-relative repository resolution together. Introduce a narrow Git-inventory path check; keep language identifier linting and contract compatibility separate. Verify the logged task runner and direct checks after the move. No trading behavior changes, compatibility alias, new service, or additional language scaffold is required.

## Revisit

Extend rules and check coverage when actual .NET, shell, SQL, or other language files are added. Revisit public naming changes only with explicit compatibility review.
