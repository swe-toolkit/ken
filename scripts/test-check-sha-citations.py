#!/usr/bin/env python3
"""Controls for scripts/check-sha-citations, in a throwaway git repository."""
import os
import subprocess
import tempfile

SCRIPT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "check-sha-citations")


def git(repo, *args):
    return subprocess.check_output(["git", "-C", repo, *args], text=True).strip()


with tempfile.TemporaryDirectory() as repo:
    git(repo, "init", "-q")
    git(repo, "-c", "user.name=t", "-c", "user.email=t@t", "commit", "-q",
        "--allow-empty", "-m", "base")
    commit = git(repo, "rev-parse", "HEAD")
    tree = git(repo, "rev-parse", "HEAD^{tree}")
    doc = os.path.join(repo, "lesson.md")
    with open(doc, "w") as fh:
        fh.write(f"landed as `{commit[:9]}`\n")          # commit: silent
        fh.write(f"squash `{tree[:9]}` per the notice\n")  # planted tree cite
        fh.write("cited 0123abc4567 nowhere\n")          # missing
        fh.write("a decade of x_{0}abc and deadbeef\n".format(commit[:9]))
    out = subprocess.run([SCRIPT, "lesson.md"], cwd=repo, capture_output=True,
                         text=True)
    assert out.returncode == 0, out
    rows = out.stdout.splitlines()
    assert f"lesson.md:2: {tree[:9]} tree" in rows, rows    # the planted control
    assert "lesson.md:3: 0123abc4567 missing" in rows, rows
    assert not any(commit[:9] in r for r in rows), rows      # commit is silent
    assert len(rows) == 2, rows                              # no word or identifier hits
print("check-sha-citations controls passed")
