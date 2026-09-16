---
id: PUB-DOC-ONLY-UNVALIDATED-AGAINST-ITS-DIFF
title: "`--doc-only` names a content class and is never checked against the actual content class of the diff, so the publisher accepts an assertion ABOUT the tree in place of the tree and merges a crates/ change with zero CI; the closure is one refusal, not a policy change or a discipline reminder"
status: ready
owner: verify
size: S
gate: none
tier: T2
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-09-16. Measured failure: the lieutenant invoked scripts/scripted-pr-automerge.sh --doc-only on 0a046da792bb1c6e6a40ec66f9f9fc96526c36eb, which carries a +697/-0 crates/ diff. --doc-only merges immediately with no CI, so it landed as squash 10321a158bc69cf50e0cb753e1096fe10fa43ed1 while every native-slow job, all 8 test shards, the z3 controls, the ignored-row sweep and the work-item tracker were still PENDING. The route had specified GATE: CI green alone. Self-reported immediately and completely by the lieutenant at evt_77qkyjmp52a18; no revert, on the Architect's structural ruling evt_74rscmvvc2j2k and the Steward's concurrence evt_614xvz94schgn. Named as a mechanism defect by the Architect and routed by the Steward per COORDINATION section 2."
---

> ## RELEASED to Team Verify 2026-09-16 — `ready`, size S, tier T2
>
> **This is a guard, not a policy change.** `--doc-only` keeps doing exactly
> what it does today for the diffs it is actually for. The node adds one
> refusal so the flag cannot disagree with the tree it describes.

## The defect

`scripts/scripted-pr-automerge.sh` takes `--doc-only` (`:182-183`) and, on that
path (`:765`), **merges with no CI at all** — which is the documented point of
the flag (`:17`, *"doc-only: merge immediately"*).

**Nothing anywhere validates that the diff is in fact doc-only.** The flag is an
**assertion about** the tree, supplied by the caller, and the publisher acts on
the assertion without ever consulting the tree it describes.

## Why the existing guard does not cover it

That path already carries a substantial guard and a worked failure
(`:766-780`) — but it is a **currency/citation** guard, aimed at a doc-only
merge invalidating a `library/` claim. Its own comment says the coupling is
*"CITATION-DIRECTED, not path-directed."*

**The failure here is the path-directed one that guard explicitly sets aside:**
the diff was not doc-only at all. The guard assumes the premise that failed.

## The closure

**`--doc-only` must refuse when the diff touches `crates/`.** One check against
the merge-base diff, before `acquire_merge_lock`, exiting non-zero with the
offending paths named.

The measurement is the one the repo already trusts: the diff from the
merge-base, which is what the squash lands. The classifier
`scripts/ci-doc-only.py` already exists and already decides this exact question
for CI's `classify-paths` job — **the refusal should reuse it rather than
re-implement a path predicate**, so the publisher and CI cannot disagree about
what "doc-only" means.

## Acceptance

**AC-1. `--doc-only` on a diff touching `crates/` REFUSES, non-zero, before any
merge or lock acquisition.** Control: invoke it on a candidate with a `crates/`
diff and show it exits non-zero with no merge performed and the offending paths
printed.

**AC-2. `--doc-only` on a genuinely doc-only diff is UNCHANGED.** Control: a
doc-only candidate still takes the fast path. **This AC is what keeps the fix a
guard rather than a policy change** — twelve doc-only merges took this path
correctly on 2026-09-15/16 and all twelve must still take it.

**AC-3. The refusal uses the same classifier as CI's `classify-paths`.**
Control: show the publisher's decision and `scripts/ci-doc-only.py`'s decision
agree on a case of each kind. **A second, independently-written path predicate
would reintroduce the same class of defect one level up** — two things that can
disagree about what the flag means.

**AC-4. The refusal is measured at the MERGE-BASE diff, not `diff(main, head)`.**
Control: a candidate on a stale base whose tree-diff to `main` shows unrelated
paths must still be classified by what the squash actually lands. See
`agent/memory/fleet/a-squash-merge-lands-the-diff-from-the-merge-base-not-the-diff-from-main.md`.

**AC-5. No new decorative glyphs.** The surrounding file predates the
2026-08-01 rule and carries several; that is not a licence to add more.

## SECOND INSTANCE, 2026-09-16 05:38Z — within the hour of this node being filed

**`10eed42912428d66c84533fe5cba88bcaf8c946f`
(`RT-IGNORED-PASSING-ROWS-DISPOSITION`) landed the same way**, on a diff of ten
files of which nine are under `crates/`. Measured from GitHub, not relayed:

```
PR #3747                       merged   2026-09-16T05:38:31Z
pull_request CI run 35060321484 created 2026-09-16T05:38:29Z, still in_progress
```

**Two seconds.** The publisher did not wait for the run it had just triggered —
which is the `--doc-only` fast path behaving exactly as documented, on a diff
that is not doc-only.

**What makes this instance stronger evidence than the first:** the route post
for this candidate (`evt_37hqhj4ya8hgm`) **stated the classification
explicitly** — *"it touches `.github/ignored-test-exemptions.toml`, so it
classifies `full` and takes the whole matrix"* — and the flag was mis-set
anyway.

⇒ **The procedural signal was present, correct, and ahead of the invocation, and
it did not prevent the defect.** So the remaining "the router should be clearer"
mitigation is refuted by measurement rather than by argument: it was already
done. **A caller-declared mode that nothing validates is the defect, and only a
check in the tool closes it.** Two instances sharing one predicate is where
enumeration stops paying (Architect, `evt_7fsxkxbns0j09`).

**No content was lost.** All ten paths are byte-identical between the approved
tip `74b4c6289c8fa06d8ccdd81c85b876efc8d0dbe6` and `main`, and the post-merge
push run `35060326844` classifies from `main`'s own diff, so the full matrix —
all eight test shards, the ignored-row sweep, the six `rt_parity_native` and two
`px8f` native-slow shards, the conformance suite — is running now. **The gap is
that it ran AFTER the merge instead of before it**, which is a different and
smaller defect than the first instance, where the pending jobs were simply never
reconciled.

## Why this is worth a node rather than a reminder

**The error is not a discipline failure and will not be fixed by care.** The
invoking seat had just published five consecutive genuinely doc-only candidates
from the same router, and the sixth was not — a habit built by correct
repetitions, misfiring once. **A guard in the tool is immune to that; a reminder
is not.**

It is also the same substitution this fleet hit four separate times on
2026-09-16 — **an artifact adjacent to the question standing in for the one that
answers it** (a comment's headline read for its bullets, a vote's SHA read for
its contents, a channel re-key read for an object mutation, a `--doc-only` flag
read for a diff). **This instance is the one that lives in a tool**, so it is
the one that will fire again unprompted.

## Contention

Touches `scripts/scripted-pr-automerge.sh` only, plus whatever test cover Team
Verify judges appropriate. **The publisher is in active use by the lieutenant
for every merge** — coordinate the landing so no publish is in flight, and
prefer a change that is inert until the `crates/` case is hit.
