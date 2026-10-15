"""Validate GitHub Actions with a pinned, checksum-verified actionlint binary."""
import hashlib
import io
import os
from pathlib import Path
import platform
import subprocess
import sys
import tarfile
import urllib.request
import zipfile

ROOT = Path(__file__).resolve().parents[2]
VERSION = "1.7.12"
ARCHIVES = {
    "Windows": ("windows_amd64.zip", "6e7241b51e6817ea6a047693d8e6fed13b31819c9a0dd6c5a726e1592d22f6e9"),
    "Linux": ("linux_amd64.tar.gz", "8aca8db96f1b94770f1b0d72b6dddcb1ebb8123cb3712530b08cc387b349a3d8"),
}


def main():
    system = platform.system()
    if system not in ARCHIVES or platform.machine().lower() not in {"amd64", "x86_64"}:
        print("Pinned actionlint installer supports Windows/Linux x86_64 only.")
        return 1
    suffix, digest = ARCHIVES[system]
    url = f"https://github.com/rhysd/actionlint/releases/download/v{VERSION}/actionlint_{VERSION}_{suffix}"
    # Revalidate the cached archive before extracting only the expected executable.
    directory = ROOT / "artifacts" / "tools" / f"actionlint-{VERSION}-{system.lower()}"
    directory.mkdir(parents=True, exist_ok=True)
    archive_path = directory / suffix
    if not archive_path.exists() or hashlib.sha256(archive_path.read_bytes()).hexdigest() != digest:
        request = urllib.request.Request(url, headers={"User-Agent": "Cinis-workflow-check"})
        with urllib.request.urlopen(request, timeout=60) as response:
            data = response.read()
        if hashlib.sha256(data).hexdigest() != digest:
            raise RuntimeError("actionlint archive checksum mismatch")
        archive_path.write_bytes(data)
    data = archive_path.read_bytes()
    filename = "actionlint.exe" if system == "Windows" else "actionlint"
    if suffix.endswith(".zip"):
        with zipfile.ZipFile(io.BytesIO(data)) as archive:
            binary = archive.read(filename)
    else:
        with tarfile.open(fileobj=io.BytesIO(data), mode="r:gz") as archive:
            member = archive.extractfile(filename)
            if member is None:
                raise RuntimeError("Missing actionlint executable")
            binary = member.read()
    executable = directory / filename
    executable.write_bytes(binary)
    if system != "Windows":
        os.chmod(executable, 0o755)
    workflows = sorted(str(p.relative_to(ROOT)) for p in (ROOT / ".github/workflows").glob("*.yml"))
    print(f"Validating {len(workflows)} workflows with actionlint {VERSION}", flush=True)
    # Optional shellcheck/pyflakes are disabled to keep the same mandatory gate on both hosts.
    return subprocess.run([str(executable), "-shellcheck=", "-pyflakes=", *workflows], cwd=ROOT).returncode


if __name__ == "__main__":
    sys.exit(main())
