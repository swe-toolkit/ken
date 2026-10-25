#!/usr/bin/env python3
"""Fail-closed check that a push to main repeats an already-passed PR run.

A squash merge pushes a commit whose tree is usually byte-identical to the
tree the PR run tested: the PR merge commit `refs/pull/N/merge`. When that
tree passed a full-mode pull_request run of this same workflow, the push run
would repeat identical work, so classify-paths may report `pr-verified`.

Two subcommands:

  record MODE OUT   (pull_request runs) write the tested merge commit, its
                    tree and the classified mode to OUT, for upload as the
                    `ci-tested-tree` artifact.
  check             (push runs) print `pr-verified` only when every lookup
                    and comparison below succeeds; otherwise print `full`.

`check` accepts only when all of the following hold:
  - exactly one merged same-repository PR into main has this push as its
    merge commit;
  - a completed, successful pull_request run of this workflow file exists
    for that PR's final head sha, from the same repository;
  - that run's `ci-tested-tree` record names mode `full`, the run itself and
    the PR head;
  - the recorded merge commit, fetched from origin, is a two-parent merge
    whose second parent is the PR head and whose first parent is an ancestor
    of the pushed commit;
  - that merge commit's tree equals the pushed commit's tree.

Any exception, missing object or mismatch yields `full`. The reason is
logged to stderr as one `CI_PR_VERIFIED` line.
"""
import io
import json
import os
import subprocess
import sys
import urllib.request
import zipfile

ARTIFACT = "ci-tested-tree"
RECORD = "record.json"
SCHEMA = 1
MAX_RUNS = 5
MAX_ARTIFACT_BYTES = 65536


class Reject(Exception):
    pass


def require(cond, reason):
    if not cond:
        raise Reject(reason)


def is_sha(value):
    return isinstance(value, str) and len(value) == 40 and all(c in "0123456789abcdef" for c in value)


def select_pr(pulls, push_sha, repo):
    require(isinstance(pulls, list), "commit pulls response is not a list")
    merged = [
        p for p in pulls
        if p.get("merged_at")
        and p.get("merge_commit_sha") == push_sha
        and p.get("base", {}).get("ref") == "main"
        and p.get("base", {}).get("repo", {}).get("full_name") == repo
        and p.get("head", {}).get("repo", {}).get("full_name") == repo
    ]
    require(len(merged) == 1, f"expected one merged same-repo PR for {push_sha}, found {len(merged)}")
    head = merged[0].get("head", {}).get("sha")
    require(is_sha(head), "PR head sha is malformed")
    return merged[0]


def candidate_runs(runs, head_sha, repo, workflow_path):
    require(isinstance(runs, list), "workflow runs response is not a list")
    ok = [
        r for r in runs
        if r.get("event") == "pull_request"
        and r.get("status") == "completed"
        and r.get("conclusion") == "success"
        and r.get("head_sha") == head_sha
        and r.get("path") == workflow_path
        and r.get("head_repository", {}).get("full_name") == repo
        and isinstance(r.get("id"), int)
    ]
    require(ok, f"no successful pull_request run of {workflow_path} for head {head_sha}")
    ok.sort(key=lambda r: (r.get("created_at") or "", r["id"]), reverse=True)
    return ok[:MAX_RUNS]


def newest_artifact(artifacts):
    require(isinstance(artifacts, list), "artifacts response is not a list")
    live = [
        a for a in artifacts
        if a.get("name") == ARTIFACT
        and not a.get("expired")
        and isinstance(a.get("size_in_bytes"), int)
        and 0 < a["size_in_bytes"] <= MAX_ARTIFACT_BYTES
        and a.get("archive_download_url")
    ]
    require(live, f"run has no live {ARTIFACT} artifact")
    live.sort(key=lambda a: (a.get("created_at") or "", a.get("id", 0)), reverse=True)
    return live[0]


def read_record(archive):
    with zipfile.ZipFile(io.BytesIO(archive)) as z:
        require(z.namelist() == [RECORD], f"{ARTIFACT} archive must hold only {RECORD}")
        return json.loads(z.read(RECORD))


def verify_record(record, run_id, head_sha, head_tree):
    require(isinstance(record, dict), "record is not an object")
    require(record.get("schema") == SCHEMA, "record schema mismatch")
    require(record.get("event") == "pull_request", "record event is not pull_request")
    require(record.get("run_id") == run_id, "record run_id does not name this run")
    require(record.get("mode") == "full", f"PR run mode was {record.get('mode')!r}, not full")
    require(record.get("head_sha") == head_sha, "record head_sha is not the PR head")
    require(is_sha(record.get("sha")), "record merge sha is malformed")
    require(record.get("tree") == head_tree, "recorded tree differs from the pushed tree")
    return record["sha"]


def verify_merge(git, merge_sha, head_sha, head_tree):
    git.fetch(merge_sha)
    parents = git.parents(merge_sha)
    require(len(parents) == 2, f"tested commit {merge_sha} is not a two-parent merge")
    base, pr_head = parents
    require(pr_head == head_sha, "tested merge second parent is not the PR head")
    require(git.is_ancestor(base, "HEAD"), "tested merge base is not an ancestor of the pushed commit")
    tree = git.tree(merge_sha)
    require(tree == head_tree, f"tested tree {tree} differs from pushed tree {head_tree}")
    return base


def decide(env, api, download, git):
    """Return (mode, reason). Raises nothing: every failure is `full`."""
    try:
        require(env.get("GITHUB_EVENT_NAME") == "push", "event is not push")
        require(env.get("GITHUB_REF") == "refs/heads/main", "ref is not main")
        repo = env.get("GITHUB_REPOSITORY") or ""
        require(repo.count("/") == 1, "GITHUB_REPOSITORY is malformed")
        push_sha = env.get("GITHUB_SHA")
        require(is_sha(push_sha), "GITHUB_SHA is malformed")
        workflow_ref = env.get("GITHUB_WORKFLOW_REF") or ""
        prefix = repo + "/"
        require(workflow_ref.startswith(prefix) and "@" in workflow_ref, "GITHUB_WORKFLOW_REF is malformed")
        workflow_path = workflow_ref[len(prefix):].split("@", 1)[0]
        require(workflow_path.startswith(".github/workflows/"), "workflow path is malformed")
        workflow_file = workflow_path.rsplit("/", 1)[1]

        require(git.rev("HEAD") == push_sha, "checked-out HEAD is not GITHUB_SHA")
        head_tree = git.tree("HEAD")

        pr = select_pr(api(f"/repos/{repo}/commits/{push_sha}/pulls"), push_sha, repo)
        pr_head = pr["head"]["sha"]
        runs = api(
            f"/repos/{repo}/actions/workflows/{workflow_file}/runs"
            f"?event=pull_request&head_sha={pr_head}&status=success&per_page=10"
        )
        failures = []
        for run in candidate_runs(runs.get("workflow_runs") if isinstance(runs, dict) else None, pr_head, repo, workflow_path):
            try:
                listing = api(f"/repos/{repo}/actions/runs/{run['id']}/artifacts?name={ARTIFACT}")
                artifact = newest_artifact(listing.get("artifacts") if isinstance(listing, dict) else None)
                record = read_record(download(artifact["archive_download_url"]))
                merge_sha = verify_record(record, run["id"], pr_head, head_tree)
                base = verify_merge(git, merge_sha, pr_head, head_tree)
            except Exception as exc:  # try the next older successful run
                failures.append(f"run {run['id']}: {exc}")
                continue
            return "pr-verified", (
                f"skip: push {push_sha} tree {head_tree} equals the tree of tested merge "
                f"{merge_sha} (base {base}, head {pr_head}) from PR #{pr.get('number')} "
                f"successful full run {run['id']} {run.get('html_url', '')}".rstrip()
            )
        raise Reject("; ".join(failures))
    except Exception as exc:
        return "full", f"full: {type(exc).__name__}: {exc}"


class Git:
    def _out(self, *args):
        return subprocess.run(["git", *args], check=True, capture_output=True, text=True).stdout.strip()

    def rev(self, rev):
        return self._out("rev-parse", "--verify", f"{rev}^{{commit}}")

    def tree(self, rev):
        return self._out("rev-parse", "--verify", f"{rev}^{{tree}}")

    def parents(self, rev):
        return self._out("rev-list", "--parents", "-n", "1", rev).split()[1:]

    def is_ancestor(self, a, b):
        return subprocess.run(["git", "merge-base", "--is-ancestor", a, b], capture_output=True).returncode == 0

    def fetch(self, sha):
        subprocess.run(["git", "fetch", "--no-tags", "--quiet", "origin", sha], check=True, capture_output=True)


class _NoAuthRedirect(urllib.request.HTTPRedirectHandler):
    # Artifact downloads redirect to signed blob storage; do not forward the token.
    def redirect_request(self, req, fp, code, msg, headers, newurl):
        return urllib.request.Request(newurl)


def http_client(token, api_url):
    opener = urllib.request.build_opener(_NoAuthRedirect)
    headers = {"Accept": "application/vnd.github+json", "Authorization": "Bearer " + token,
               "X-GitHub-Api-Version": "2022-11-28"}

    def fetch(url, limit):
        with opener.open(urllib.request.Request(url, headers=headers), timeout=30) as resp:
            body = resp.read(limit + 1)
        require(len(body) <= limit, f"response from {url} exceeds {limit} bytes")
        return body

    def api(path):
        return json.loads(fetch(api_url.rstrip("/") + path, 8 << 20))

    def download(url):
        return fetch(url, MAX_ARTIFACT_BYTES)

    return api, download


def record(mode, out):
    sha = os.environ["GITHUB_SHA"]
    require(Git().rev("HEAD") == sha, "checked-out HEAD is not GITHUB_SHA")
    data = {
        "schema": SCHEMA,
        "event": os.environ["GITHUB_EVENT_NAME"],
        "run_id": int(os.environ["GITHUB_RUN_ID"]),
        "mode": mode,
        "head_sha": os.environ["PR_HEAD_SHA"],
        "sha": sha,
        "tree": Git().tree("HEAD"),
    }
    os.makedirs(os.path.dirname(out) or ".", exist_ok=True)
    with open(out, "w") as f:
        json.dump(data, f, sort_keys=True)
        f.write("\n")
    print(f"CI_TESTED_TREE {json.dumps(data, sort_keys=True)}")


def main(argv):
    if len(argv) == 4 and argv[1] == "record":
        record(argv[2], argv[3])
        return 0
    if len(argv) == 2 and argv[1] == "check":
        token = os.environ.get("CI_PR_VERIFIED_TOKEN", "")
        api, download = http_client(token, os.environ.get("GITHUB_API_URL", "https://api.github.com"))
        mode, reason = decide(os.environ, api, download, Git()) if token else ("full", "full: no token")
        line = f"CI_PR_VERIFIED mode={mode} {reason}"
        print(line, file=sys.stderr)
        # A pr-verified green skips the test jobs; say why on the run page,
        # so it is not read as this run having executed them.
        summary = os.environ.get("GITHUB_STEP_SUMMARY")
        if summary:
            try:
                with open(summary, "a") as f:
                    f.write(line + "\n")
            except OSError:
                pass
        print(mode)
        return 0
    print("usage: ci-pr-verified.py record MODE OUT | check", file=sys.stderr)
    return 2


if __name__ == "__main__":
    sys.exit(main(sys.argv))
