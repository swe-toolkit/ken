#!/usr/bin/env python3
"""Controls for scripts/ci-pr-verified.py: one accepting case, each guard rejects."""
import copy
import importlib.util
import io
import json
import os
import subprocess
import tempfile
import zipfile

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("ci_pr_verified", os.path.join(HERE, "ci-pr-verified.py"))
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)

REPO = "o/r"
PUSH = "a" * 40
TREE = "b" * 40
HEAD = "c" * 40
MERGE = "d" * 40
BASE = "e" * 40
RUN = 77
ENV = {
    "GITHUB_EVENT_NAME": "push",
    "GITHUB_REF": "refs/heads/main",
    "GITHUB_REPOSITORY": REPO,
    "GITHUB_SHA": PUSH,
    "GITHUB_WORKFLOW_REF": f"{REPO}/.github/workflows/ci.yml@refs/heads/main",
}


def zipped(record, name="record.json"):
    buf = io.BytesIO()
    with zipfile.ZipFile(buf, "w") as z:
        z.writestr(name, json.dumps(record))
    return buf.getvalue()


def world():
    return {
        "pulls": [{
            "number": 5, "merged_at": "t", "merge_commit_sha": PUSH,
            "base": {"ref": "main", "repo": {"full_name": REPO}},
            "head": {"sha": HEAD, "repo": {"full_name": REPO}},
        }],
        "runs": {"workflow_runs": [{
            "id": RUN, "event": "pull_request", "status": "completed", "conclusion": "success",
            "head_sha": HEAD, "path": ".github/workflows/ci.yml", "created_at": "t",
            "head_repository": {"full_name": REPO}, "html_url": "https://x/runs/77",
        }]},
        "artifacts": {"artifacts": [{
            "id": 1, "name": "ci-tested-tree", "expired": False, "size_in_bytes": 300,
            "archive_download_url": "https://x/zip", "created_at": "t",
        }]},
        "record": {"schema": 1, "event": "pull_request", "run_id": RUN, "mode": "full",
                   "head_sha": HEAD, "sha": MERGE, "tree": TREE},
        "archive": None,
        "git": {"HEAD": PUSH, "trees": {"HEAD": TREE, MERGE: TREE},
                "parents": {MERGE: [BASE, HEAD]}, "ancestors": {BASE}, "fetchable": {MERGE}},
    }


class FakeGit:
    def __init__(self, g):
        self.g = g

    def rev(self, rev):
        return self.g["HEAD"]

    def tree(self, rev):
        return self.g["trees"][rev]

    def parents(self, rev):
        return self.g["parents"][rev]

    def is_ancestor(self, a, b):
        return a in self.g["ancestors"]

    def fetch(self, sha):
        if sha not in self.g["fetchable"]:
            raise RuntimeError("fetch failed")


def run(w, env=ENV):
    def api(path):
        if "/commits/" in path:
            return w["pulls"]
        if "/workflows/" in path:
            return w["runs"]
        if "/artifacts" in path:
            return w["artifacts"]
        raise AssertionError(path)

    def download(url):
        return w["archive"] if w["archive"] is not None else zipped(w["record"])

    return m.decide(env, api, download, FakeGit(w["git"]))


mode, reason = run(world())
assert mode == "pr-verified", reason
assert MERGE in reason and TREE in reason and str(RUN) in reason, reason


def rejects(mutate, env=ENV):
    w = copy.deepcopy(world())
    mutate(w)
    mode, reason = run(w, env)
    assert mode == "full", (mode, reason)
    return reason


def set_in(path, value):
    def mutate(w):
        target = w
        for key in path[:-1]:
            target = target[key]
        target[path[-1]] = value
    return mutate


# Event, ref and environment guards.
rejects(lambda w: None, {**ENV, "GITHUB_EVENT_NAME": "pull_request"})
rejects(lambda w: None, {**ENV, "GITHUB_REF": "refs/heads/other"})
rejects(lambda w: None, {**ENV, "GITHUB_SHA": "short"})
rejects(lambda w: None, {**ENV, "GITHUB_WORKFLOW_REF": "other/repo/.github/workflows/ci.yml@x"})
rejects(set_in(["git", "HEAD"], "f" * 40))
# PR association.
rejects(set_in(["pulls"], []))
rejects(lambda w: w["pulls"].append(copy.deepcopy(w["pulls"][0])))
rejects(set_in(["pulls", 0, "merged_at"], None))
rejects(set_in(["pulls", 0, "merge_commit_sha"], "f" * 40))
rejects(set_in(["pulls", 0, "base", "ref"], "dev"))
rejects(set_in(["pulls", 0, "head", "repo", "full_name"], "fork/r"))
rejects(set_in(["pulls"], {"message": "Not Found"}))
# Run selection.
rejects(set_in(["runs", "workflow_runs"], []))
rejects(set_in(["runs", "workflow_runs", 0, "conclusion"], "failure"))
rejects(set_in(["runs", "workflow_runs", 0, "status"], "in_progress"))
rejects(set_in(["runs", "workflow_runs", 0, "event"], "push"))
rejects(set_in(["runs", "workflow_runs", 0, "head_sha"], "f" * 40))
rejects(set_in(["runs", "workflow_runs", 0, "path"], ".github/workflows/other.yml"))
rejects(set_in(["runs", "workflow_runs", 0, "head_repository", "full_name"], "fork/r"))
# Artifact and record.
rejects(set_in(["artifacts", "artifacts"], []))
rejects(set_in(["artifacts", "artifacts", 0, "expired"], True))
rejects(set_in(["artifacts", "artifacts", 0, "size_in_bytes"], 10 ** 7))
rejects(set_in(["record", "mode"], "doc-only"))
rejects(set_in(["record", "run_id"], RUN + 1))
rejects(set_in(["record", "head_sha"], "f" * 40))
rejects(set_in(["record", "tree"], "f" * 40))
rejects(set_in(["record", "schema"], 2))
rejects(set_in(["archive"], b"not a zip"))
rejects(set_in(["archive"], zipped(world()["record"], name="other.json")))
# Git verification of the tested merge commit.
rejects(set_in(["git", "fetchable"], set()))
rejects(set_in(["git", "trees", MERGE], "f" * 40))
rejects(set_in(["git", "parents", MERGE], [HEAD]))
rejects(set_in(["git", "parents", MERGE], [BASE, "f" * 40]))
rejects(set_in(["git", "ancestors"], set()))
# A failing newest run falls through to an older verified run.
w = world()
newer = dict(w["runs"]["workflow_runs"][0], id=RUN + 1, created_at="u")
w["runs"]["workflow_runs"].insert(0, newer)
mode, reason = run(w)
assert mode == "pr-verified" and f"run {RUN} " in reason, reason

# The real Git adapter against a scratch repository: a merge commit whose tree
# equals a squash commit's tree verifies; the CLI with no token prints `full`.
with tempfile.TemporaryDirectory() as d:
    def git(*args):
        return subprocess.run(["git", "-C", d, *args], check=True, capture_output=True, text=True).stdout.strip()
    git("init", "-q", "-b", "main")
    git("config", "user.email", "t@example.invalid")
    git("config", "user.name", "t")
    git("commit", "-q", "--allow-empty", "-m", "base")
    git("switch", "-q", "-c", "pr")
    with open(os.path.join(d, "f"), "w") as f:
        f.write("x\n")
    git("add", "f")
    git("commit", "-q", "-m", "pr")
    git("switch", "-q", "main")
    git("merge", "-q", "--no-ff", "-m", "merge", "pr")
    merge = git("rev-parse", "HEAD")
    git("reset", "-q", "--hard", "HEAD~1")
    git("merge", "-q", "--squash", "pr")
    git("commit", "-q", "-m", "squash")
    cwd = os.getcwd()
    os.chdir(d)
    try:
        real = m.Git()
        assert real.tree(merge) == real.tree("HEAD")
        assert real.parents(merge)[1] == git("rev-parse", "pr")
        assert real.is_ancestor(real.parents(merge)[0], "HEAD")
        assert not real.is_ancestor(merge, "HEAD")
    finally:
        os.chdir(cwd)
out = subprocess.run(["python3", os.path.join(HERE, "ci-pr-verified.py"), "check"],
                     capture_output=True, text=True, env={"PATH": os.environ["PATH"]})
assert out.returncode == 0 and out.stdout.strip() == "full", out
print("ci pr-verified controls passed")
