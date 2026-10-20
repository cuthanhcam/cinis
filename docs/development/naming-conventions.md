# Naming conventions

Status: Accepted repository convention, 2026-10-15. Owner: repository maintainer. Scope: maintained Cinis source, automation, documentation, contracts, and generated-output identities.

## Principle and precedence

Use one domain vocabulary across languages, with each language's idiomatic casing. Consistency means the same concept and responsibility, not forcing PascalCase or snake_case everywhere. Product name: Cinis. Repository/CLI prefix: cinis. Do not introduce Cineres into new maintained names.

Apply, in order: fixed tool/framework filenames; versioned external contracts; the language rules below; repository defaults. A naming change to an exposed contract needs compatibility review. This standard does not authorize renaming existing public APIs or future components merely for style.

## Repository paths

| Category | Rule | Examples |
| --- | --- | --- |
| Shared/top-level directories | Lowercase; kebab-case for multiple words; clear responsibility | src, tests, docs, ops, research, deploy |
| Documentation topic directories | Full, clear lowercase names; kebab-case if multiple words | docs/operations, docs/architecture, docs/development |
| Automation directories | Short root ops; descriptive children | ops/scripts, ops/config, ops/tests |
| .NET project/type directories | PascalCase and dot-separated project identity | src/platform/Cinis.Gateway, Features/Orders |
| .NET application lockfiles | Preserve NuGet filename; commit per project | packages.lock.json |
| Rust/Python module directories | snake_case according to language | market_data, order_book |
| Markdown files | kebab-case; one topic per file | state-ownership.md, naming-conventions.md; versioned scope-v0.1.md |
| ADR files | Four-digit immutable number plus kebab-case decision | 0004-naming-and-automation-layout.md |
| Shared matching fixtures | snake_case and stable scenario prefix | mc01_partial_cancel.txt |
| Workflow/config filenames | kebab-case except tool-required names | engine.yml, projects.psd1 |
| Tool-required files | Preserve exact ecosystem spelling | README.md, LICENSE, AGENTS.md, Cargo.toml, Cargo.lock, global.json, Directory.Build.props, .editorconfig |

Use docs/operations/ for operational documentation and ops/ for executable automation. There is no root scripts/ directory: both Python quality checks and PowerShell task runners belong under ops/scripts/. Keep tools/ for application-facing research tools rather than repository wrappers.

Do not create paths distinguished only by case, spaces, unexplained abbreviations, timestamps for source versions, or names such as final, final2, new, misc, common, and helpers without a precise responsibility. Established short roots src/ops/docs and standard technical terms API, CLI, HTTP, TCP, UUID, GTC are explicit exceptions to the abbreviation rule. Existing root community files keep their standard uppercase filenames, including GIT_GUIDELINES.md and COMMIT_MESSAGE_GUIDELINES.md. Standard GitHub CODEOWNERS/SUPPORT.md and explicitly shared .vscode settings/extensions files are also recognized exceptions.

## .NET / C# (when projects exist)

| Item | Rule | Example |
| --- | --- | --- |
| Solution/project/assembly/root namespace | Cinis.<Component>[.<Responsibility>] | Cinis.Platform.slnx, Cinis.Gateway.csproj, Cinis.Gateway |
| C# file | Main type name, PascalCase | OrderRequest.cs, OrderServiceTests.cs |
| Types, namespaces, public members | PascalCase | OrderRequest, OrderId, PlaceOrder |
| Interfaces | I plus PascalCase | IOrderGateway |
| Parameters/local variables | camelCase | orderId, remainingQuantity |
| Private instance fields | _camelCase | _orderGateway |
| Constants | PascalCase | MaxOrderCount |
| Async Task/ValueTask methods | Async suffix | PlaceOrderAsync |
| Boolean members | Predicate wording | IsOpen, HasRemainingQuantity, CanCancel |
| Tests | <Member>_<Condition>_<ExpectedBehavior> | CancelOrder_WrongOwner_ReturnsNotOwner |

Use Id, Api, Http in composed identifiers: OrderId, ApiKey, HttpClient; avoid ORDER_ID or APIKey in C# members. Match namespace/project responsibility instead of embedding the full architecture into every class name. Introduce test projects as Cinis.<Component>.Tests or Cinis.<Component>.IntegrationTests only when real tests exist. These are project choices aligned with [Microsoft C# conventions](https://learn.microsoft.com/en-us/dotnet/csharp/fundamentals/coding-style/identifier-names).

## Rust

| Item | Rule | Example |
| --- | --- | --- |
| Cargo package and binary target | lowercase kebab-case with cinis prefix | cinis-engine, cinis-demo |
| Crate import | snake_case | cinis_engine |
| Source/module file, function, method, variable, test | snake_case | order_book.rs, place_order, remaining_quantity |
| Types, traits, enum variants | UpperCamelCase | OrderId, OrderGateway, PartiallyFilled |
| Constants/statics | SCREAMING_SNAKE_CASE | MAX_QUANTITY |
| Boolean methods | Predicate wording | is_open, has_remaining_quantity |

Preserve conventional lib.rs, main.rs, mod.rs, Cargo.toml, and Cargo.lock. Rust src/bin/ is maintained source and must remain trackable despite .NET bin/ ignore patterns. Cinis chooses kebab-case Cargo package names; Rust imports normalize hyphens to underscores. Use ordinary getters such as orders(), not get_orders(), where the method simply exposes a value. Follow [Rust API naming guidance](https://rust-lang.github.io/api-guidelines/naming.html). Do not add crate prefixes to every internal symbol.

## Python

Modules/files, functions, methods, variables, and test functions use snake_case: check_docs.py, check_naming.py, test_runner.py, repository_root. Classes use CapWords: RunnerTests. Constants use UPPER_SNAKE_CASE: ROOT. Test modules start with test_; internal implementation names can start with one underscore. Use readable short lowercase package names, without hyphens in importable modules. These follow [PEP 8](https://peps.python.org/pep-0008/).

Standalone checks are executed with python ops/scripts/<name>.py. Keep file stems importable even if a script is initially only a CLI. Do not name a module json.py, subprocess.py, pathlib.py, or another imported standard-library module. Generalize into a Python package only when shared code warrants it; no empty package hierarchy is required.

## PowerShell

Public scripts and functions use an approved Verb plus PascalCase Noun, with a Cinis noun prefix for shared commands: Invoke-Cinis.ps1, Get-CinisLogs.ps1, Invoke-CinisLoggedProcess. Use Get for retrieval, Test for validation, Invoke for execution, Start/Stop for actual lifecycle actions. A finite demo run remains Invoke-Cinis; do not label it Start-CinisService before a service exists. Check verbs using Get-Verb; see [approved verbs](https://learn.microsoft.com/en-us/powershell/scripting/developer/cmdlet/approved-verbs-for-windows-powershell-commands).

Modules use Cinis.<Responsibility>.psm1; a corresponding module manifest, if introduced, matches its stem. Existing Cinis.Operations.psm1 retains the full responsibility name even though its parent folder is ops/. Parameters use PascalCase; internal variables use camelCase. Registry/config data files use descriptive kebab-case: projects.psd1. Avoid aliases and positional magic in maintained automation. Public script parameters and exported functions are interfaces; rename deliberately and update all callers.

## Other executable and configuration files

Future shell scripts use kebab-case action names with .sh, such as verify-engine.sh, and snake_case internal functions/variables. Do not create shell equivalents until supported and tested. Preserve Dockerfile, compose.yaml, Makefile, and other required framework filenames when introduced.

Application executables use cinis-<role> for Rust/native CLI tools. .NET project/assembly identity stays Cinis.<Role>; published host filenames may follow platform defaults. Public endpoint/service identities can use cinis-gateway; they must not depend on the source filename. Current cinis-demo is a finite demo, not a service.

YAML/workflow file stems use kebab-case; workflow display names are readable English; job/step IDs use kebab-case. Environment variables use CINIS_UPPER_SNAKE_CASE when Cinis owns them; retain required tool variables such as RUST_BACKTRACE. Secrets never belong in filenames or command arguments.

Future SQL tables/columns use snake_case, explicit singular entities, and descriptive constraints; finalize migration tooling before creating schema. Financial units must appear in names where ambiguity exists: price_ticks, quantity_lots, balance_atoms, timeout_seconds. Keep currency/asset identity explicit rather than assuming all integers have one scale.

## Cross-language contracts and domain vocabulary

Use OrderId/order_id/orderId for the same identity; use remaining quantity, reservation, fill, trade, and ledger consistently with the glossary. Do not casually substitute booking, payment, transaction, or position for these concepts.

JSON field policy is lowerCamelCase, such as orderId, accountId, remainingQuantity; enum/error wire values must be explicitly versioned, not inferred from Rust Debug output or C# enum names. Existing MC fixture tokens and CLI debug output are test/demo formats, not JSON contracts. Freeze actual protocol names and serialization behavior in protocol.md before network implementation. SQL/projections can use snake_case with explicit mapping. Changing casing in code must not silently change serialized names.

FR/QR/BR/INV/SC/MC IDs keep uppercase stable prefixes. Branches follow type/kebab-case-description, such as chore/naming-and-script-layout and test/m1-reference-model. Commits follow COMMIT_MESSAGE_GUIDELINES.md. Generated run IDs/log filenames follow the operations logging contract rather than source-file naming rules.

## Enforcement and migration

Read this page before adding a language or project. Review semantic names and units in PRs; formatters do not judge terminology. Rust compiler/Clippy checks language identifiers; Python/PowerShell checks currently focus on behavior, not comprehensive identifier linting. .NET analyzers/formatting are introduced with real .NET projects.

check_naming.py checks maintained path spelling and case collisions against the supported current layout, not every source identifier or future external contract. It reads Git's tracked/unignored inventory and excludes generated logs/builds through .gitignore. Root/community/tool filenames and language-specific paths are explicit exceptions. When adding a new language, extend the checker and document its rules together rather than enforcing an unrelated language's casing.

A directory/script migration updates CI, task registry, docs, relative roots, tests, and contributors' commands together. No silent compatibility wrappers or duplicate document trees. Historical ADR decisions remain historical; superseding decisions explain the current contract.

ASP.NET Core keeps framework configuration names: appsettings.json and appsettings.<Environment>.json (for example appsettings.Development.json). Personal appsettings.Local.json overrides remain ignored; Program does not automatically load that custom suffix. Use standard environment configuration or user secrets deliberately.
