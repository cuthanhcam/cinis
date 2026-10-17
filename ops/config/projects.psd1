@{
    # Trusted repository configuration. Never put credentials in argument lists.
    Engine = @{
        Kind = 'Job'
        Description = 'Volatile synthetic financial execution; no background service or recovery.'
        Tasks = @{
            Build = @(@{ Tool = 'cargo'; Arguments = @('build', '--locked', '--manifest-path', 'src/engine/Cargo.toml') })
            Test = @(@{ Tool = 'cargo'; Arguments = @('test', '--locked', '--manifest-path', 'src/engine/Cargo.toml') })
            Run = @(@{ Tool = 'cargo'; Arguments = @('run', '--locked', '--manifest-path', 'src/engine/Cargo.toml', '--bin', 'cinis-demo') })
            RunMatching = @(@{ Tool = 'cargo'; Arguments = @('run', '--locked', '--manifest-path', 'src/engine/Cargo.toml', '--bin', 'cinis-matching-demo') })
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
    Platform = @{
        Kind = 'Application'
        Description = 'Fail-closed platform scaffold; Run is a finite smoke; Serve is bounded foreground debugging.'
        Tasks = @{
            Build = @(
                @{ Tool = 'dotnet'; Arguments = @('restore', 'src/platform/Cinis.Platform.slnx', '--locked-mode') }
                @{ Tool = 'dotnet'; Arguments = @('build', 'src/platform/Cinis.Platform.slnx', '--no-restore', '-c', 'Release') }
            )
            Test = @(@{ Tool = 'dotnet'; Arguments = @('test', 'src/platform/Cinis.Platform.slnx', '--no-restore', '-c', 'Release') })
            Run = @(@{ Tool = 'dotnet'; Arguments = @('run', '--project', 'src/platform/Cinis.Platform', '--no-launch-profile', '-c', 'Release', '--', '--smoke-test') })
            Serve = @(@{ Tool = 'dotnet'; Arguments = @('run', '--project', 'src/platform/Cinis.Platform', '--no-launch-profile', '-c', 'Release', '--', '--urls', 'http://127.0.0.1:5080') })
            Format = @(@{ Tool = 'dotnet'; Arguments = @('format', 'src/platform/Cinis.Platform.slnx', 'whitespace', '--no-restore', '--verify-no-changes') })
            Lint = @(@{ Tool = 'dotnet'; Arguments = @('build', 'src/platform/Cinis.Platform.slnx', '--no-restore', '-c', 'Release') })
            Docs = @(
                @{ Tool = 'python'; Arguments = @('ops/scripts/check_docs.py') }
                @{ Tool = 'python'; Arguments = @('ops/scripts/check_naming.py') }
            )
            Workflows = @(@{ Tool = 'python'; Arguments = @('ops/scripts/check_workflows.py') })
            Verify = @(
                @{ Tool = 'dotnet'; Arguments = @('restore', 'src/platform/Cinis.Platform.slnx', '--locked-mode') }
                @{ Tool = 'dotnet'; Arguments = @('format', 'src/platform/Cinis.Platform.slnx', 'whitespace', '--no-restore', '--verify-no-changes') }
                @{ Tool = 'dotnet'; Arguments = @('build', 'src/platform/Cinis.Platform.slnx', '--no-restore', '-c', 'Release') }
                @{ Tool = 'dotnet'; Arguments = @('test', 'src/platform/Cinis.Platform.slnx', '--no-build', '--no-restore', '-c', 'Release') }
                @{ Tool = 'dotnet'; Arguments = @('run', '--project', 'src/platform/Cinis.Platform', '--no-build', '--no-restore', '-c', 'Release', '--', '--smoke-test') }
                @{ Tool = 'python'; Arguments = @('ops/scripts/check_docs.py') }
                @{ Tool = 'python'; Arguments = @('ops/scripts/check_naming.py') }
                @{ Tool = 'python'; Arguments = @('ops/scripts/check_scaffold.py') }
            )
        }
    }
}
