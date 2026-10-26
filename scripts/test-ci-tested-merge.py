#!/usr/bin/env python3
"""Controls for scripts/ci-tested-merge.py: one accepting case, each guard rejects."""
import importlib.util
import json
import os
import subprocess
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("ci_tested_merge", os.path.join(HERE, "ci-tested-merge.py"))
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)

MERGE = "a" * 40
TREE = "b" * 40
HEAD = "c" * 40
BASE = "e" * 40
ENV = {
    "GITHUB_EVENT_NAME": "pull_request",
    "GITHUB_SHA": MERGE,
    "GITHUB_RUN_ID": "77",
    "PR_NUMBER": "5",
    "PR_HEAD_SHA": HEAD,
}


class FakeGit:
    def __init__(self, head=MERGE, parents=(BASE, HEAD)):
        self.head, self._parents = head, list(parents)

    def rev(self, rev):
        return self.head

    def tree(self, rev):
        return TREE

    def parents(self, rev):
        return self._parents


rec = m.build_record(ENV, FakeGit(), "full")
assert rec == {"schema": 2, "event": "pull_request", "run_id": 77, "pr_number": 5, "mode": "full",
               "head_sha": HEAD, "sha": MERGE, "base_sha": BASE, "merge_head_sha": HEAD,
               "tree": TREE}, rec


def rejects(env=ENV, git=None):
    try:
        m.build_record(env, git or FakeGit(), "full")
    except m.Reject:
        return
    raise AssertionError("expected Reject")


rejects(env={**ENV, "GITHUB_EVENT_NAME": "push"})
rejects(git=FakeGit(head="f" * 40))
rejects(env={**ENV, "PR_HEAD_SHA": "short"})
rejects(git=FakeGit(parents=(BASE,)))
rejects(git=FakeGit(parents=(BASE, HEAD, "f" * 40)))
rejects(git=FakeGit(parents=(BASE, "f" * 40)))
rejects(git=FakeGit(parents=("nothex", HEAD)))

# End to end against a real merge commit, as a pull_request checkout sees it.
with tempfile.TemporaryDirectory() as d:
    def git(*args):
        return subprocess.run(["git", "-C", d, "-c", "user.email=t@x", "-c", "user.name=t", *args],
                              check=True, capture_output=True, text=True).stdout.strip()
    git("init", "-q", "-b", "main")
    git("commit", "-q", "--allow-empty", "-m", "root")
    git("checkout", "-q", "-b", "pr")
    git("commit", "-q", "--allow-empty", "-m", "pr")
    head = git("rev-parse", "HEAD")
    git("checkout", "-q", "main")
    git("commit", "-q", "--allow-empty", "-m", "main moved")
    base = git("rev-parse", "HEAD")
    git("merge", "-q", "--no-ff", "-m", "merge", "pr")
    merge = git("rev-parse", "HEAD")
    out = os.path.join(d, "out", "record.json")
    summary = os.path.join(d, "summary.md")
    env = {**os.environ, **ENV, "GITHUB_SHA": merge, "PR_HEAD_SHA": head,
           "GITHUB_STEP_SUMMARY": summary}
    subprocess.run(["python3", os.path.join(HERE, "ci-tested-merge.py"), "record", "doc-only", out],
                   check=True, capture_output=True, text=True, cwd=d, env=env)
    got = json.load(open(out))
    assert (got["sha"], got["base_sha"], got["merge_head_sha"], got["mode"]) == (merge, base, head, "doc-only"), got
    assert f"base={base}" in open(summary).read()
    bad = subprocess.run(["python3", os.path.join(HERE, "ci-tested-merge.py"), "record", "full", out],
                         capture_output=True, text=True, cwd=d, env={**env, "PR_HEAD_SHA": base})
    assert bad.returncode != 0, "a merge whose second parent is not the PR head must fail the run"

print("ci tested-merge controls passed")
