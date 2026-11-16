#!/usr/bin/env python3
"""Behavioural controls for the fail-safe formatter implementation gate."""
import copy
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

SCRIPT = Path(__file__).with_name("ci-formatter-corpus-relevant.py")
spec = importlib.util.spec_from_file_location("ci_formatter_corpus_relevant", SCRIPT)
gate = importlib.util.module_from_spec(spec)
spec.loader.exec_module(gate)


def package(name, deps=()):
    return {
        "id": f"path+file:///w/crates/{name}#0.0.0",
        "name": name,
        "manifest_path": f"/w/crates/{name}/Cargo.toml",
        "dependencies": [
            {"name": dep, "kind": None, "path": f"/w/crates/{dep}"}
            for dep in deps
        ],
    }


PACKAGES = [package("ken-elaborator", ["ken-kernel"]),
            package("ken-kernel"), package("ken-cli")]
METADATA = {
    "workspace_root": "/w",
    "workspace_members": [pkg["id"] for pkg in PACKAGES],
    "packages": PACKAGES,
}


class GateTests(unittest.TestCase):
    def test_implementation_inputs_and_corpus_only(self):
        for path in (
            "crates/ken-elaborator/src/layout.rs",
            "crates/ken-elaborator/src/lexer.rs",
            "crates/ken-elaborator/src/lossless.rs",
            "crates/ken-elaborator/src/resolve.rs",
            "crates/ken-elaborator/src/literate.rs",
            "crates/ken-kernel/src/lib.rs",
            "crates/ken-elaborator/Cargo.toml",
            "crates/ken-cli/Cargo.toml",
            ".github/workflows/ci.yml",
            "scripts/ci-duration-shard.py",
        ):
            self.assertTrue(gate.classify([path], METADATA)[0], path)
        self.assertFalse(gate.classify(["catalog/packages/Core/Logic/Or.ken.md"], METADATA)[0])
        self.assertFalse(gate.classify(["crates/ken-cli/src/main.rs"], METADATA)[0])
        self.assertTrue(gate.classify(["catalog/anything", "crates/ken-elaborator/src/layout.rs"], METADATA)[0])

    def test_new_dependency_is_picked_up(self):
        metadata = copy.deepcopy(METADATA)
        metadata["packages"].append(package("ken-layout"))
        metadata["workspace_members"].append(metadata["packages"][-1]["id"])
        metadata["packages"][0]["dependencies"].append(
            {"name": "ken-layout", "kind": None, "path": "/w/crates/ken-layout"}
        )
        self.assertTrue(gate.classify(["crates/ken-layout/src/lib.rs"], metadata)[0])

    def test_unknown_or_malformed_inputs_fail_open_to_running(self):
        for paths, metadata in (
            ([], METADATA), ("catalog/x", METADATA), ([], None),
            (["catalog/x"], None), (["catalog/x"], {"packages": []}),
            (["unknown/new.rs"], METADATA), (["catalog/../layout.rs"], METADATA),
        ):
            self.assertTrue(gate.classify(paths, metadata)[0])
        with tempfile.TemporaryDirectory() as directory:
            paths = Path(directory) / "paths.json"
            metadata = Path(directory) / "metadata.json"
            metadata.write_text(json.dumps(METADATA))
            paths.write_text(json.dumps(["catalog/packages/Core/Logic/Or.ken.md"]))
            command = [sys.executable, str(SCRIPT), str(paths), str(metadata)]
            self.assertEqual(subprocess.run(command, capture_output=True, text=True).stdout.strip(), "false")
            paths.write_text("broken json")
            self.assertEqual(subprocess.run(command, capture_output=True, text=True).stdout.strip(), "true")
            paths.write_text(json.dumps(["catalog/x"]))
            metadata.unlink()
            self.assertEqual(subprocess.run(command, capture_output=True, text=True).stdout.strip(), "true")


if __name__ == "__main__":
    unittest.main()
