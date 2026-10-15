"""Check maintained path names; language identifiers and contract compatibility need separate review."""
from pathlib import Path, PurePosixPath
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
KEBAB = re.compile(r"[a-z][a-z0-9]*(?:-[a-z0-9]+)*")
SNAKE = re.compile(r"[a-z][a-z0-9]*(?:_[a-z0-9]+)*")
PASCAL = re.compile(r"[A-Z][A-Za-z0-9]*")
DOTNET = re.compile(r"Cinis(?:\.[A-Z][A-Za-z0-9]*)+")
ROOT_FILES = {
    "README.md", "AGENTS.md", "LICENSE", "CHANGELOG.md", "CONTRIBUTING.md",
    "CODE_OF_CONDUCT.md", "SECURITY.md", "GIT_GUIDELINES.md", "COMMIT_MESSAGE_GUIDELINES.md",
    ".editorconfig", ".gitattributes", ".gitignore", ".env.example", "global.json", "NuGet.config",
    "Directory.Build.props", "Directory.Build.targets", "Directory.Packages.props",
}
TOOL_FILES = {"Cargo.toml", "Cargo.lock", "lib.rs", "main.rs", "mod.rs", "__init__.py", "Dockerfile", "Makefile"}
GITHUB_FILES = {"CODEOWNERS", "SUPPORT.md", "pull_request_template.md", "bug_report.md", "work_item.md"}


def validate_path(name):
    path = PurePosixPath(name)
    parts = path.parts
    errors = []
    if parts[0] == "scripts" or parts[:2] == ("docs", "ops"):
        errors.append("use ops/scripts/ for automation and docs/operations/ for operational docs")
    for index, folder in enumerate(parts[:-1]):
        if folder in {".github", ".vscode"} and index == 0:
            continue
        if folder == "ISSUE_TEMPLATE" and parts[0] == ".github":
            continue
        language_directory = (path.suffix in {".rs", ".py"} or parts[:2] == ("tests", "fixtures"))
        dotnet_directory = parts[:2] == ("src", "platform") and index >= 2
        if KEBAB.fullmatch(folder):
            continue
        if language_directory and SNAKE.fullmatch(folder):
            continue
        if dotnet_directory and (PASCAL.fullmatch(folder) or DOTNET.fullmatch(folder)):
            continue
        errors.append(f"nonconforming directory: {folder}")
    filename = path.name
    if (len(parts) == 1 and filename in ROOT_FILES) or filename in TOOL_FILES or filename == "README.md":
        return errors
    if parts[0] == ".github" and filename in GITHUB_FILES:
        return errors
    suffix = path.suffix
    stem = path.stem
    if suffix in {".rs", ".py"} or parts[:2] == ("tests", "fixtures"):
        valid = SNAKE.fullmatch(stem)
    elif suffix == ".ps1":
        valid = re.fullmatch(r"[A-Z][A-Za-z]+-Cinis[A-Za-z0-9]*", stem)
    elif suffix in {".psm1", ".psd1"} and stem.startswith("Cinis."):
        valid = DOTNET.fullmatch(stem)
    elif suffix in {".cs", ".csproj", ".sln", ".slnx"}:
        valid = PASCAL.fullmatch(stem) or DOTNET.fullmatch(stem)
    elif suffix == ".md" and len(parts) > 2 and parts[:3] == ("docs", "architecture", "decisions"):
        valid = re.fullmatch(r"\d{4}-[a-z][a-z0-9]*(?:-[a-z0-9]+)*", stem)
    else:
        valid = KEBAB.fullmatch(stem) or re.fullmatch(r"[a-z][a-z0-9]*(?:-[a-z0-9]+)*-v[0-9]+(?:\.[0-9]+)+", stem)
    if not valid:
        errors.append(f"nonconforming filename: {filename}")
    return errors


def main():
    inventory = subprocess.run(
        ["git", "ls-files", "--cached", "--others", "--exclude-standard", "-z"],
        cwd=ROOT, capture_output=True, check=True,
    ).stdout.decode("utf-8").split("\0")
    names = sorted({name for name in inventory if name and (ROOT / name).is_file()})
    errors = []
    spelling = {}
    for name in names:
        errors.extend(f"{name}: {message}" for message in validate_path(name))
        parts = PurePosixPath(name).parts
        for length in range(1, len(parts) + 1):
            prefix = "/".join(parts[:length])
            previous = spelling.setdefault(prefix.casefold(), prefix)
            if previous != prefix:
                errors.append(f"case-only collision: {previous} / {prefix}")
    if errors:
        print("\n".join(sorted(set(errors))))
        return 1
    print(f"Naming checks passed ({len(names)} maintained files; path rules and case collisions).")
    return 0


if __name__ == "__main__":
    sys.exit(main())
