---
id: CAT-BOOL-REUSE-CONSUMERS
title: "Drain catalog-reuse census group 6 (Boolean computational reuse) — replace three reimplementations (Derived#bool_and, Derived#bool_leq, Map#option_is_some) with selective imports of the now-public LC.bool_and, LC.bool_leq and SC.is_some. The consumer half of CAT-BOOL-PUB-EXPORT, shaped on the landed CAT-DERIVED-REUSE-CONSUMERS per-package increment pattern."
status: active
owner: foundation
size: S
gate: none
tier: T2
depends_on: [CAT-BOOL-PUB-EXPORT]
blocks: []
github: null
origin: "Steward, 2026-08-29, filed on the CAT-BOOL-PUB-EXPORT landing (providers public at 4faa97bfb, PR #3108) so lane 3 does not idle. Group 6 membership quoted verbatim from docs/program/cat-reuse-census.md §4.4 item 6 (lines 317-320) at origin/main 4faa97bfb; the three [low] consume tags read from §3 rows 36 (Derived) and 37 (Map). The two providers CAT-BOOL-PUB-EXPORT just published (LC.bool_and, LC.bool_leq, SC.is_some) are exactly the three names group 6 consumes, so the prerequisite covers the consumers with nothing left over. Steward-filed per COORDINATION section 2."
---

> # RELEASED — lane 3, the group-6 consumer drain. `ready`.
>
> The provider prerequisite `CAT-BOOL-PUB-EXPORT` is MERGED (`4faa97bfb`,
> PR #3108); the three names are `pub` and loader-visible. This node replaces the
> three duplicate reimplementations with selective imports. Two consumer modules,
> three sites, all census-tagged `[low]`.

## Fixed inputs (re-measured at `origin/main` `4faa97bfb`)

Census group 6, **"Boolean computational reuse"**, quoted verbatim from
`docs/program/cat-reuse-census.md` §4.4 item 6 — three `[low]` consumer sites
across two packages:

| consumer site (§4.4) | reimplements | provider (now `pub`) | provider module |
|---|---|---|---|
| `Data/Collections/Derived.ken.md#bool_and` | `bool_and` | `LC.bool_and` | `Core/Classes/LawfulClasses.ken.md` |
| `Data/Collections/Derived.ken.md#bool_leq` | `bool_leq` | `LC.bool_leq` | `Core/Classes/LawfulClasses.ken.md` |
| `Data/Collections/Map.ken.md#option_is_some` | `is_some` | `SC.is_some` | `Data/Sums/Combinators.ken.md` |

Census consume tags (§3): Derived row 36 — `bool_and->LC.bool_and [low]`,
`bool_leq->LC.bool_leq [low]`; Map row 37 — `option_is_some->SC.is_some [low]`.
All three are `[low]`. **The other Map boolean consumes in row 37
(`bool_and`, `cat4_bool_or`, `bool_dichotomy`, `leq_nat`, `total_leq_nat`) are
`[higher]` and are NOT group 6 — do NOT touch them here.**

**The Map site is a RENAME, the Derived sites are not.** Derived's local
`bool_and`/`bool_leq` share the provider's names, so an unqualified selective
import leaves internal call sites unchanged. Map's local is named
`option_is_some` while the provider is `is_some`, so draining it renames the
reference at every internal call site (or aliases the import) — measure the call
sites before deleting the local.

## Deliverable

Two per-package increments, released/verified one at a time (the
`CAT-DERIVED-REUSE-CONSUMERS` pattern), each replacing the named
reimplementation(s) with a selective import of the now-public provider and
deleting the local definition:

- **D1 — Derived** (`Data/Collections/Derived.ken.md`): import `bool_and` and
  `bool_leq` from `Core.Classes.LawfulClasses`; delete the two local
  reimplementations; internal references unchanged (same names).
- **D2 — Map** (`Data/Collections/Map.ken.md`): import `is_some` from
  `Data.Sums.Combinators`; delete the local `option_is_some`; update its
  internal call sites to the imported name (or alias the import). Touch ONLY
  `option_is_some` — leave the `[higher]` Map consumes alone.

## Acceptance criteria (each increment)

- **AC-CENSUS-ROW-DRAINED** — the increment's census §4.4 group-6 row(s) no
  longer name a reimplementation: the local definition is deleted and the
  selective import names the provider. Control: the selective import names the
  provider and the named local `fn` is gone.
- **AC-NO-EQUIVALENT-LOCAL** (the load-bearing causal control; amended
  2026-08-29, Steward disposition on the CV's second reject) — after the
  drain the consumer module defines NO function kernel-equivalent (identical type
  AND identical transparent body) to the imported provider, so the drained
  computation cannot be served by a renamed reimplementation. This is a CLOSED
  relation over the module's own definitions — the consumer-side MIRROR of the
  provider-side loader-visible inventory the CV required on `CAT-BOOL-PUB-EXPORT`
  (population from own definitions, closed equality, NOT an occurrence census).
  It must be CAUSAL/equivalence-based, not occurrence-based: a control that
  passes merely because the provider `Const` APPEARS somewhere in a type/body has
  not measured reuse. Required reddening mutation (CV's counterexample): reroute
  the consumers to a renamed kernel-equivalent local (e.g. `local_bool_leq`),
  with or without an unused provider alias `let _ = bool_leq` padding, MUST
  redden. Spelling-agnostic — ban the equivalence, never a name or the padding
  spelling. **If a sound closed mechanism cannot be built (a legitimately-needed
  local is kernel-equivalent to the provider, or the equivalence check has a real
  gap), that is a HARD STOP to the Architect — not another occurrence-census
  respin.**
- **AC-SAME-BEHAVIOUR** — the consumer module elaborates to the same result
  through the imported provider as through the deleted local. Control: the
  module's existing checked declarations and any dependent headline
  (Derived's sort/derived string-byte headlines; Map's `Tree` map operations)
  still elaborate; a mutation that imports the WRONG provider name reddens.
- **AC-STANDALONE-GREEN** — the consumer module still elaborates standalone
  (exit 0) after the drain. If the import pulls the module non-standalone that is
  a HARD STOP to the Architect, not a workaround.
- **AC-NO-OTHER-DRAIN** (D2 only) — Map's `[higher]` boolean consumes are
  untouched; only `option_is_some` is drained. Control: those local definitions
  and their call sites are byte-unchanged.

## Reviewers

Foundation QA (the census row is drained, the module stays standalone-green, and
the same-behaviour control reddens on the wrong provider) + conformance-validator
(the loader actually resolves the selective import to the public provider, not a
shadowing local — this is the consumer mirror of the loader-visibility inventory
the CV owns on the provider side). A drain that turns a consumer module
non-standalone HARD-STOPS to the Architect.

## Capability tier

T2 — a mechanical, precedent-shaped catalog reuse drain (three sites, two files),
reviewed differentially on census-row-drained + standalone-green, not on an
argument. Size S (smaller than group 4's six sites / five packages).

## Sequencing

Lane 3 (foundation). Released 2026-08-29 on the `CAT-BOOL-PUB-EXPORT` landing so
the lane does not idle. `depends_on: [CAT-BOOL-PUB-EXPORT]` (merged). This closes
census group 6. Groups 1, 5 and 7 are not re-measured and are not framed here
(§4c — frame on need, not ahead of it); group 1's provider is the compiler
prelude and may need no prerequisite, but that is a measurement for when this
lands, not now.
