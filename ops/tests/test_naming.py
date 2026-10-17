"""Representative repository/language path policy and layout migration regression checks."""
import importlib.util
from pathlib import Path
import unittest

SCRIPT = Path(__file__).resolve().parents[1] / "scripts" / "check_naming.py"
SPEC = importlib.util.spec_from_file_location("cinis_check_naming", SCRIPT)
POLICY = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(POLICY)


class NamingTests(unittest.TestCase):
    def test_existing_tool_and_versioned_document_names(self):
        for path in ("Cargo.toml", "src/engine/Cargo.lock", "README.md", "AGENTS.md",
                     "docs/product/scope-v0.1.md", ".github/ISSUE_TEMPLATE/bug_report.md"):
            with self.subTest(path=path):
                self.assertEqual(POLICY.validate_path(path), [])

    def test_native_language_paths(self):
        for path in ("src/platform/Cinis.Gateway/Features/Orders/OrderRequest.cs",
                     "src/platform/Cinis.Gateway/Cinis.Gateway.csproj",
                     "src/engine/src/market_data/order_book.rs",
                     "ops/scripts/check_docs.py", "ops/scripts/Get-CinisLogs.ps1",
                     "ops/scripts/Cinis.Operations.psm1",
                     "src/platform/Cinis.Access/packages.lock.json",
                     "src/platform/Cinis.Platform.slnx",
                     "src/platform/Cinis.Platform/Properties/launchSettings.json"):
            with self.subTest(path=path):
                self.assertEqual(POLICY.validate_path(path), [])

    def test_wrong_casing_and_language_style(self):
        for path in ("docs/operations/RunLocal.md", "ops/scripts/check-docs.py",
                     "src/engine/src/OrderBook.rs", "ops/scripts/run-local.ps1",
                     "docs/operations/run_local.md"):
            with self.subTest(path=path):
                self.assertTrue(POLICY.validate_path(path))

    def test_retired_automation_and_documentation_paths(self):
        self.assertTrue(POLICY.validate_path("scripts/check_docs.py"))
        self.assertTrue(POLICY.validate_path("docs/ops/logging.md"))
        self.assertEqual(POLICY.validate_path("docs/operations/logging.md"), [])


if __name__ == "__main__":
    unittest.main()
