---
id: CAT-EXPORT-CENSUS-DERIVES-LOADER-PREDICATE
title: "Derive the export-census population from the loader's own publication predicate instead of a parallel hand-maintained Decl match, so the exactly-six equality control cannot silently narrow as new publishable declaration kinds are added."
status: draft
owner: foundation
size: S
gate: none
tier: T2
depends_on: [CAT-DERIVED-PUB-EXPORT]
blocks: []
github: null
origin: "Adversary M8 pre-publication hunt on CAT-DERIVED-PUB-EXPORT rebased 84d836e39, evt_2rnmt4yt8n6xa, 2026-08-29. Verdict CLEAN; this is the single labeled LATENT note, which does NOT reproduce at that SHA. Filed by the Steward after independently verifying both cited coordinates and enumerating the divergence."
---

> # LATENT. NOT A REGRESSION, AND NOT A REASON TO REOPEN THE PREDECESSOR.
>
> There is no live defect at `84d836e39`. `Derived.ken.md` contains only
> `fn`/`theorem`/`proof`/`class`/`instance`/`import` declarations, all six
> exports are `fn`, and both sides enumerate those. **The predecessor's control
> is sound for the file it guards today.** What is latent is that the control
> narrows silently as the declaration surface grows.

## The defect

The acceptance test's population helper `top_level_publication_queries()`
enumerates publishable declaration kinds with its own `match` ending in
`_ => None`. The loader decides the same question with `is_qualifiable`
(`crates/ken-elaborator/src/modules.rs:1638`). **The two are maintained
independently and they already disagree.**

Measured at `84d836e39`:

| kind | loader `is_qualifiable` | census match | consequence |
|---|---|---|---|
| `SpaceDecl` | publishable | falls through `_ => None` | **over-export invisible to the equality** |
| `ClassDecl` | not publishable | queried | harmless: loader refuses it anyway |
| `AttachedProofDecl` | publishable | separate dedicated pass | covered, not a gap |

`SpaceDecl` is the real one. A `pub space` — or any new `Decl` variant added to
`is_qualifiable` — becomes an export that the "exactly six and nothing else"
assertion **cannot see**, because the name never enters the population it
compares. The control stays green while the export set grows.

## Why this matters more than its size suggests

**This is the same class as the defect that rejected the predecessor's first
candidate**, one level in. That rejection was a `^pub` source census standing in
as a proxy for loader visibility. The repair correctly moved the **verdict** to
the loader — but the **population** is still a hand-maintained parallel
enumeration. **Half the proxy was removed.**

⇒ **A control has two sides, and moving one of them to the authority is not
moving the control to the authority.** Ask of every such repair: is the
population derived from the same authority as the verdict, or only compared
against it?

This also corrects a residual gap in the predecessor's own amended
`AC-EXCLUDED-UNMARKED`, which required the population be derived "mechanically
from the module's own definitions". Parsing the file **is** mechanical, so the
candidate satisfied it — mechanical derivation from the artifact is not the same
as derivation from the deciding predicate.

## Deliverables

- **D1 — derive the census population from `is_qualifiable`.** The test's
  publishable-kind set must come from the loader's predicate rather than a
  parallel match, so the two cannot drift. If `is_qualifiable` is not reachable
  from the test's crate boundary, expose it narrowly rather than duplicating it;
  duplicating it is the defect.

## Acceptance criteria, each with its control

- **AC-NO-PARALLEL-MATCH.** No hand-maintained enumeration of publishable
  declaration kinds remains in the test. Control: adding a new publishable
  variant to `is_qualifiable` must reach the census with no edit to the test.
- **AC-SPACEDECL-VISIBLE.** A `pub space` in the guarded module is caught by the
  equality assertion. Control: add one, observe RED, byte-restore. **This is the
  arm that is currently blind — it must be exhibited, not argued.**
- **AC-SIX-UNCHANGED.** The existing six-name equality and both predecessor RED
  mutations (`list_append` withdrawal; leading-space `pub Perm`) still behave
  identically. Control: re-run them.
- **AC-AFFECTED-CLOSURE.** Cover every target that loads any module whose
  closure this changes. Scope by which PATHS changed, never which VALUES
  changed. Targeted via `scripts/ken-cargo`, never `--workspace`.

## Sequencing

`draft`, queued behind [[CAT-DERIVED-PUB-EXPORT]] on file contention — it edits
that node's acceptance test. Flip `ready` and release once the predecessor is
confirmed `merged` by blob. **Do not fold this into the predecessor:** that node
is gated, approved on an exact SHA, and routed; reopening it to add a criterion
would void two exact-SHA votes for a latent finding with no live reproduction.
