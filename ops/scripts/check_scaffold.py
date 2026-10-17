"""Check repository-foundation inventory and representative Git ignore contracts."""
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
REQUIRED = (
    "global.json", "Directory.Build.props", "Directory.Packages.props", "NuGet.config",
    "src/platform/Cinis.Platform.slnx", ".github/workflows/platform.yml",
    "README.md", "LICENSE", "AGENTS.md", "CONTRIBUTING.md", "CODE_OF_CONDUCT.md",
    "SECURITY.md", "GIT_GUIDELINES.md", "COMMIT_MESSAGE_GUIDELINES.md", "CHANGELOG.md",
    ".gitignore", ".gitattributes", ".editorconfig", ".env.example",
    ".github/CODEOWNERS", ".github/SUPPORT.md", ".github/dependabot.yml",
    ".github/pull_request_template.md", ".github/ISSUE_TEMPLATE/config.yml",
    "docs/README.md", "docs/development/naming-conventions.md", "docs/development/ci.md",
    "docs/development/scaffold.md", "ops/config/projects.psd1",
)
IGNORED = (".env", ".env.local", "artifacts/logs/example/run.json", "src/engine/target/debug/engine.exe",
           "src/platform/Cinis.Gateway/bin/Debug/app.dll", "src/platform/Cinis.Gateway/obj/project.assets.json",
           "ops/scripts/__pycache__/example.pyc", ".venv/example", "private.key")
TRACKABLE = (".env.example", ".vscode/settings.json", ".github/workflows/ops.yml",
             "src/engine/Cargo.lock", "src/engine/src/bin/matching_demo.rs", "src/platform/Cinis.Gateway/appsettings.json",
             "src/platform/Cinis.Gateway/Migrations/Initial.cs", "deploy/compose.yaml")


def is_ignored(path):
    result = subprocess.run(["git", "check-ignore", "--no-index", "--quiet", "--", path], cwd=ROOT)
    if result.returncode not in (0, 1):
        raise RuntimeError("Unable to evaluate Git ignore policy")
    return result.returncode == 0


def main():
    errors = [f"Missing scaffold file: {name}" for name in REQUIRED if not (ROOT / name).is_file()]
    errors += [f"Local/generated path must be ignored: {name}" for name in IGNORED if not is_ignored(name)]
    errors += [f"Source/config path must remain trackable: {name}" for name in TRACKABLE if is_ignored(name)]
    if errors:
        print("\n".join(errors))
        return 1
    print("Scaffold inventory and Git ignore contracts passed.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
