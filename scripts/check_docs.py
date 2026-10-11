"""Check local Markdown file targets and basic text hygiene."""
from pathlib import Path
import re
import sys
from urllib.parse import unquote, urlsplit

ROOT = Path(__file__).resolve().parents[1]
errors = []
for path in ROOT.rglob("*"):
    if not path.is_file() or ".git" in path.relative_to(ROOT).parts or path.name == "init.md":
        continue
    if path.suffix not in {".md", ".py", ".yml"}:
        continue
    text = path.read_text(encoding="utf-8")
    label = path.relative_to(ROOT)
    if not text.endswith("\n"):
        errors.append(f"{label}: missing final newline")
    if "\ufffd" in text:
        errors.append(f"{label}: replacement character")
    if path.suffix != ".md":
        continue
    # Remote URLs and heading fragments are outside this file-target check.
    for target in re.findall(r"\[[^\]]*\]\(([^)]+)\)", text):
        target = target.strip().strip("<>")
        parsed = urlsplit(target)
        if parsed.scheme or target.startswith("#"):
            continue
        local = unquote(parsed.path)
        if local and not (path.parent / local).exists():
            errors.append(f"{label}: missing link target {target}")
if errors:
    print("\n".join(errors))
    sys.exit(1)
print("Documentation checks passed (local file links, UTF-8, final newlines).")
