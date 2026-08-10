---
id: CI-ROW-CLAIM-NAMESPACE
title: "verify-row-claims hardcodes surface/ in both its claim and heading patterns, so eight of the nine conformance namespaces are structurally invisible to it -- a claim it cannot see is indistinguishable from a claim that does not exist"
status: active
owner: verify
size: S
gate: none
depends_on: []
blocks: []
github: null
origin: "Found by the conformance-validator at evt_3z26bwcjhcwtj while authoring CONF-EVAL-COMPUTED-BOOL-ELIM's four runtime/evaluation rows at exact base 376d495d: the checker reported 30 resolved claims on both an untouched detached base worktree and the edited tree, against an expected 34. Cause grounded to scripts/ci-ignored-sweep.py:36-37 and independently re-measured by the Steward at 376d495d. Steward-filed (agents cannot create tracked work per COORDINATION §2)."
---

> ## Frame: `docs/program/wp/CI-ROW-CLAIM-NAMESPACE.md`. `ready`, shovel-ready.
>
> No dependencies. **Sequence after `CI-DOCTEST-UNEXECUTED`** — same team, same
> ring, and the two touch different files but Verify runs one node at a time.

## The defect in one line

`scripts/ci-ignored-sweep.py:36-37`:

```python
ROW_CLAIM_RE   = re.compile(r"^\s*//(?:/)?\s+(?P<row>surface/\S+)")
ROW_HEADING_RE = re.compile(r"^###\s+(?P<row>surface/\S+)(?:\s.*)?$")
```

**Both patterns hardcode `surface/`.** `conformance/` has **nine** top-level
namespaces — `behavioral`, `challenge`, `fs`, `kernel`, `runtime`, `security`,
`stdlib`, `surface`, `verify`. The checker governs **one**.

## This is the third axis of the same defect, and the principle is now proven

`CI-L1-EXECUTING-COVER` built the checker. `CI-ROW-CLAIM-COMMENT-FORM` widened
its **comment-marker** axis after the Adversary showed a `//` claim was
invisible. This is the **namespace** axis, and it is the same sentence again:

> **A claim the extractor cannot see is indistinguishable from a claim that does
> not exist.**

**It stayed invisible because nobody had authored a non-`surface/` claim until
the conformance-validator did today.** The gate has been reporting green over
eight untested namespaces since it landed.

## What it cost, concretely

`CONF-EVAL-COMPUTED-BOOL-ELIM`'s `AC-1` was written by the Steward as "the
checker's resolved count rises" and **could not be discharged at all**. That
node's four `runtime/evaluation/` rows now land governed by no automated
checker, with the gap stated as its residual. This node is what closes it.

## The trap that makes the obvious fix wrong

**Do not replace `surface/` with a permissive pattern.** A generic
`(?P<row>\w+/\S+)` re-admits exactly the population `CI-ROW-CLAIM-COMMENT-FORM`
measured and excluded: **file-path citations in doc comments**
(`spec/30-surface/35-numbers.md`, `conformance/surface/numbers/seed-numbers.md`)
look identical to row ids under any pattern loose enough to span namespaces.
That census found 29 such citations. Re-admitting them would manufacture false
unresolved claims and red the gate for the wrong reason.

**Derive the namespace set from the filesystem** — the directories under
`conformance/` — rather than hardcoding a list of nine. A hardcoded list is the
same defect with a longer literal, and it drifts the first time a namespace is
added.
