#!/usr/bin/env python3
"""Record exactly which merge commit a pull_request CI run tested.

A pull_request run checks out `refs/pull/N/merge`: a two-parent merge commit
whose first parent is the `main` tip GitHub merged against and whose second
parent is the PR head. CI no longer runs after a merge lands on main, so the
publisher (scripts/scripted-pr-automerge.sh) must know which base a green run
tested before it may merge: it merges only when that first parent is the
current origin/main tip and the second parent is the routed SHA.

  record MODE OUT   write the tested merge commit, both of its parents, its
                    tree, the PR number and head, the run id and the classified
                    mode to OUT, for upload as the `ci-tested-tree` artifact.

Recording fails, and so fails the run, when HEAD is not GITHUB_SHA, is not a
two-parent merge, or its second parent is not the PR head. A run without a
record cannot authorize a merge, so a recording failure must be red.
"""
import json
import os
import subprocess
import sys

SCHEMA = 2


class Reject(Exception):
    pass


def require(cond, reason):
    if not cond:
        raise Reject(reason)


def is_sha(value):
    return isinstance(value, str) and len(value) == 40 and all(c in "0123456789abcdef" for c in value)


class Git:
    def _out(self, *args):
        return subprocess.run(["git", *args], check=True, capture_output=True, text=True).stdout.strip()

    def rev(self, rev):
        return self._out("rev-parse", "--verify", f"{rev}^{{commit}}")

    def tree(self, rev):
        return self._out("rev-parse", "--verify", f"{rev}^{{tree}}")

    def parents(self, rev):
        return self._out("rev-list", "--parents", "-n", "1", rev).split()[1:]


def build_record(env, git, mode):
    sha = env["GITHUB_SHA"]
    require(env.get("GITHUB_EVENT_NAME") == "pull_request", "event is not pull_request")
    require(git.rev("HEAD") == sha, "checked-out HEAD is not GITHUB_SHA")
    head_sha = env["PR_HEAD_SHA"]
    require(is_sha(head_sha), "PR_HEAD_SHA is malformed")
    parents = git.parents("HEAD")
    require(len(parents) == 2, f"tested commit {sha} is not a two-parent merge")
    base, merge_head = parents
    require(is_sha(base) and is_sha(merge_head), "tested merge parents are malformed")
    require(merge_head == head_sha, f"tested merge second parent {merge_head} is not the PR head {head_sha}")
    return {
        "schema": SCHEMA,
        "event": env["GITHUB_EVENT_NAME"],
        "run_id": int(env["GITHUB_RUN_ID"]),
        "pr_number": int(env["PR_NUMBER"]),
        "mode": mode,
        "head_sha": head_sha,
        "sha": sha,
        "base_sha": base,
        "merge_head_sha": merge_head,
        "tree": git.tree("HEAD"),
    }


def record(mode, out):
    data = build_record(os.environ, Git(), mode)
    os.makedirs(os.path.dirname(out) or ".", exist_ok=True)
    with open(out, "w") as f:
        json.dump(data, f, sort_keys=True)
        f.write("\n")
    line = (f"CI_TESTED_MERGE sha={data['sha']} base={data['base_sha']} "
            f"head={data['merge_head_sha']} mode={mode}")
    print(f"CI_TESTED_TREE {json.dumps(data, sort_keys=True)}")
    print(line)
    # Say on the run page which base this run tested, so a reader can tell a
    # green run on a stale base from a green run on the current one.
    summary = os.environ.get("GITHUB_STEP_SUMMARY")
    if summary:
        try:
            with open(summary, "a") as f:
                f.write(line + "\n")
        except OSError:
            pass


def main(argv):
    if len(argv) == 4 and argv[1] == "record":
        record(argv[2], argv[3])
        return 0
    print("usage: ci-tested-merge.py record MODE OUT", file=sys.stderr)
    return 2


if __name__ == "__main__":
    sys.exit(main(sys.argv))
