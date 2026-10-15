@{
    # Trusted repository configuration. Never put credentials in argument lists.
    Engine = @{
        Kind = 'Job'
        Description = 'Volatile matching demo; no background service or financial state.'
        Tasks = @{
            Build = @(@{ Tool = 'cargo'; Arguments = @('build', '--locked', '--manifest-path', 'src/engine/Cargo.toml') })
            Test = @(@{ Tool = 'cargo'; Arguments = @('test', '--locked', '--manifest-path', 'src/engine/Cargo.toml') })
            Run = @(@{ Tool = 'cargo'; Arguments = @('run', '--locked', '--manifest-path', 'src/engine/Cargo.toml', '--bin', 'cinis-demo') })
            Format = @(@{ Tool = 'cargo'; Arguments = @('fmt', '--manifest-path', 'src/engine/Cargo.toml', '--check') })
            Lint = @(@{ Tool = 'cargo'; Arguments = @('clippy', '--locked', '--manifest-path', 'src/engine/Cargo.toml', '--all-targets', '--', '-D', 'warnings') })
            Workflows = @(@{ Tool = 'python'; Arguments = @('ops/scripts/check_workflows.py') })
            Docs = @(
                @{ Tool = 'python'; Arguments = @('ops/scripts/check_docs.py') }
                @{ Tool = 'python'; Arguments = @('ops/scripts/check_naming.py') }
            )
            Verify = @(
                @{ Tool = 'cargo'; Arguments = @('fmt', '--manifest-path', 'src/engine/Cargo.toml', '--check') }
                @{ Tool = 'cargo'; Arguments = @('clippy', '--locked', '--manifest-path', 'src/engine/Cargo.toml', '--all-targets', '--', '-D', 'warnings') }
                @{ Tool = 'cargo'; Arguments = @('build', '--locked', '--manifest-path', 'src/engine/Cargo.toml') }
                @{ Tool = 'cargo'; Arguments = @('test', '--locked', '--manifest-path', 'src/engine/Cargo.toml') }
                @{ Tool = 'python'; Arguments = @('ops/scripts/check_docs.py') }
                @{ Tool = 'python'; Arguments = @('ops/scripts/check_naming.py') }
                @{ Tool = 'python'; Arguments = @('ops/scripts/check_workflows.py') }
                @{ Tool = 'python'; Arguments = @('ops/scripts/check_scaffold.py') }
            )
        }
    }
}
