---
id: CAT-EXPORT-CENSUS-DERIVES-LOADER-PREDICATE
title: "Derive the export-census population from the loader's own publication predicate instead of a parallel hand-maintained Decl match, so the exactly-six equality control cannot silently narrow as new publishable declaration kinds are added."
status: closed
owner: foundation
size: S
gate: none
tier: T2
depends_on: [CAT-DERIVED-PUB-EXPORT]
blocks: []
github: null
origin: "Adversary M8 pre-publication hunt on CAT-DERIVED-PUB-EXPORT rebased 84d836e39, evt_2rnmt4yt8n6xa, 2026-08-29. Verdict CLEAN; this is the single labeled LATENT note, which does NOT reproduce at that SHA. Filed by the Steward after independently verifying both cited coordinates and enumerating the divergence."
---

> # WITHDRAWN 2026-08-31 — BOTH OPERATIVE CLAIMS FALSE (Architect ruling).
>
> Closed without landing. Architect ruling `evt_mgwssa5xvckb` (grounded at base
> `978b05dd29`, tree `0f846696…`, `modules.rs` blob `36503cf7…`, test blob
> `624a33aa…`, Foundation `pub space` log/diff, and an independent same-base
> `pub class ExportCensusProbe a {}` scratch run):
>
> **No extant `Decl` variant is omitted by `top_level_publication_queries()`
> while being publishable by the real loader.** The classification is
> mechanically closed at `parser.rs::pub_eligibility` with **no catch-all** — a
> new `Decl` variant creates a compile-time classification obligation, so the
> census cannot silently narrow. The test helper's population already equals the
> complete current pub-eligible set (nine ordinary kinds in the direct arm,
> `AttachedProofDecl` in its own arm).
>
> **The frame named the wrong authority.** `is_qualifiable` classifies
> module-local qualification/shadowing, not publication, and two counterexamples
> prove it cannot be the export predicate: `SpaceDecl` is `is_qualifiable==true`
> yet `pub space` is refused (`parse_pub_decl`, and redundantly `expand_scope`'s
> placement gate); `ClassDecl` is `is_qualifiable==false` yet a public class IS
> published (dedicated `expand_scope` class arm). The real publication authority
> is successful insertion via `publish_identity` into `exports_here`, persisted
> as `ModuleState.exports[module]` after `load_unit` — the interface required by
> `spec/30-surface/33-declarations.md §4.1`. There is no single declaration-kind
> predicate equivalent to it.
>
> **The existing control is already sound.** At the same base, inserting a valid
> `pub class ExportCensusProbe a {}` loads successfully and the existing equality
> test ALREADY REDs (left set gains `ExportCensusProbe`). It bites on a real
> over-export via the class arm — the arm `is_qualifiable` is false for. So there
> is no live defect and no reproducer: `pub space` is not a valid over-export
> mutation (accepting an earlier RED would test placement refusal, not census
> completeness).
>
> **Symptom-inventory entry 1** (recorded per the ruling): SpaceDecl was treated
> as publishable because qualification membership was mistaken for successful
> public-placement plus export-table insertion — keyed on `is_qualifiable`
> rather than `ModuleState.exports`. (Same static-fact-as-runtime-authority
> family as the runtime LIVE-K chain: a taxonomy predicate mistaken for the
> live decision.)
>
> **No successor node.** Optional future robustness — should it ever be wanted —
> is honest no-live-defect structural work: drop the source-kind population
> helper and compare the literal six directly against a narrow read-only sorted
> view of `ModuleState.exports[DERIVED]` after roots loading; pin with an
> accepted public form (the class above), never claimed as a regression
> reproducer (the present test also REDs), and expose the export table, never
> the qualification predicate. NOT framed as a node here: the `pub_eligibility`
> no-catch-all already forecloses silent narrowing, so the constraint is not a
> grounded live defect (§4c). Do NOT expose/copy `is_qualifiable`, alter space
> placement, or edit the acceptance test under this node. Foundation owed no
> product move.
>
> Everything below is the ORIGINAL frame, retained as the withdrawn record.

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

`ready` — ON DECK, not released. The predecessor is confirmed `merged` by blob
at `origin/main` `04e157a450a0d17f9fab5437c8f1f60c523ff052`, so the file
contention that held this at `draft` is discharged: the acceptance test it edits
is now on `main`.

**What it now queues behind is the seat, not the file.**
[[CAT-DERIVED-REUSE-CONSUMERS]] was released ahead of it at 03:47 UTC because it
is the lane's objective (draining census group 4) rather than a robustness
repair, and lane 3 has one implementer. The two do **not** contend on content —
this node edits `crates/ken-elaborator/tests/cat_derived_pub_export.rs`, that one
edits five catalog packages (Deque, Parsing, EffectfulClasses, Cursor, Property)
and neither touches the other's paths. Release this the moment the foundation
implementer is free. **Do not fold this into the predecessor:** that node
is gated, approved on an exact SHA, and routed; reopening it to add a criterion
would void two exact-SHA votes for a latent finding with no live reproduction.
