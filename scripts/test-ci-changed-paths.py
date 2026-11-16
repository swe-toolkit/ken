#!/usr/bin/env python3
"""Exercise git's newline-delimited diff through all path-list consumers."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

HERE = Path(__file__).resolve().parent
CONVERTER = HERE / "ci-changed-paths.py"
CLASSIFIERS = {
    "z3": HERE / "ci-z3-relevant.py",
    "formatter": HERE / "ci-formatter-corpus-relevant.py",
    "doc": HERE / "ci-doc-only.py",
}
PATH_CASES = {
    ".github/workflows/ci.yml": ("true", "true", "full"),
    "Cargo.toml": ("true", "true", "full"),
    "Cargo.lock": ("true", "true", "full"),
    "scripts/ci-changed-paths.py": ("true", "true", "full"),
    "catalog/example.ken": ("true", "false", "full"),
    "crates/ken-cli/src/main.rs": ("false", "false", "full"),
    "docs/guide.md": ("false", "false", "doc-only"),
    "docs/ends_in_n": ("false", "false", "doc-only"),
}


def metadata():
    packages = []
    for name in ("ken-elaborator", "ken-cli"):
        packages.append({
            "id": f"path+file:///w/crates/{name}#0.0.0",
            "name": name,
            "manifest_path": f"/w/crates/{name}/Cargo.toml",
            "dependencies": [],
        })
    return {"workspace_root": "/w", "workspace_members": [p["id"] for p in packages],
            "packages": packages}


class ChangedPathsTests(unittest.TestCase):
    def test_real_git_diff_feeds_all_three_classifiers(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            def git(*args):
                return subprocess.run(["git", "-C", str(root), *args],
                                      check=True, text=True, capture_output=True).stdout

            git("init", "-q")
            git("config", "user.name", "CI path fixture")
            git("config", "user.email", "ci-path-fixture@example.invalid")
            for path in PATH_CASES:
                target = root / path
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_text("before\n")
            git("add", ".")
            git("commit", "-qm", "before")
            base = git("rev-parse", "HEAD").strip()
            for path in PATH_CASES:
                (root / path).write_text("after\n")
            git("add", ".")
            git("commit", "-qm", "after")
            meta = root / "metadata.json"
            meta.write_text(json.dumps(metadata()))
            path_list = root / "paths.json"

            def convert(pathspec=None):
                args = ["git", "-C", str(root), "diff", "--name-only", "--no-renames", base, "HEAD"]
                if pathspec is not None:
                    args += ["--", pathspec]
                diff = subprocess.run(args, check=True, capture_output=True, text=True).stdout
                self.assertTrue(diff.endswith("\n"))
                output = subprocess.run([sys.executable, str(CONVERTER)], input=diff,
                                        capture_output=True, text=True, check=True).stdout
                path_list.write_text(output)
                return json.loads(output)

            self.assertEqual(set(convert()), set(PATH_CASES))
            for path, expected in PATH_CASES.items():
                self.assertEqual(convert(path), [path], path)
                values = []
                for name in ("z3", "formatter", "doc"):
                    arguments = [sys.executable, str(CLASSIFIERS[name]), str(path_list)]
                    if name != "doc":
                        arguments.append(str(meta))
                    result = subprocess.run(arguments, capture_output=True, text=True, check=True)
                    values.append(result.stdout.strip())
                self.assertEqual(tuple(values), expected, path)

    def test_truncated_path_listing_fails_before_classification(self):
        result = subprocess.run([sys.executable, str(CONVERTER)], input="Cargo.lock",
                                capture_output=True, text=True)
        self.assertEqual(result.returncode, 2)
        self.assertIn("not newline-terminated", result.stderr)


if __name__ == "__main__":
    unittest.main()
