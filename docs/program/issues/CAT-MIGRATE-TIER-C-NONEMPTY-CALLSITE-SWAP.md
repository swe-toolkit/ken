---
id: CAT-MIGRATE-TIER-C-NONEMPTY-CALLSITE-SWAP
title: "Tier-C staging P2: swap the five raw NonEmptyCons client call sites (Schema.ken.md:123,:130; ArgParse.ken.md:252; Validation.ken.md:77,:86) to nonempty_cons, still ambient. After this ZERO consumer outside NonEmpty's defining module names a raw NonEmpty constructor, which is what makes the P3 abstract flip safe (no window where a live consumer names a hidden ctor)."
status: merged
owner: foundation
size: S
gate: none
tier: T2
depends_on: [CAT-MIGRATE-TIER-C-NONEMPTY-SMART-CTORS]
blocks: [CAT-MIGRATE-TIER-C-NONEMPTY-VALIDATION]
github: null
origin: "Steward cut 2026-09-06, second step of the Architect's 3-step additive staging DAG (evt_5fxtzhk104q96) for the held Tier-C {NonEmpty, Validation} abstract-export migration. Once P1 has added the smart constructors, the ambient raw-ctor call sites can swap to nonempty_cons with identical denotation, still under ambient loading. This removes the last raw-ctor consumer BEFORE the P3 flip hides the raw ctor, satisfying the atomicity invariant: no window where a live consumer names a hidden ctor. CENSUS CORRECTED at cut (Steward on the P1-landed tree 0e439f1e5, evt_3agz16819vtge; Architect Option A evt_n71wk0sc0pww; foundation-leader evt_1km5h84bm138c): the raw-consumer census is FIVE sites, not three -- the original count used the wrong predicate ('outside NonEmpty/Validation') and missed the two raw NonEmptyCons applications in Validation.ken.md's checked example (:77/:86); Validation is a CLIENT of NonEmptyCons, not a co-definer. All five are the same ambient mechanical swap (nonempty_cons a x rest is definitionally NonEmptyCons a x rest, ambiently available post-P1), so all five belong in P2 and P3 stays a pure logic-free flip."
---

> # RELEASED 2026-09-06 to the foundation ring (lane-3). Step P2 of the
> # Architect's 3-step additive staging (evt_5fxtzhk104q96). ADDITIVE and
> # AMBIENT-SAFE: nonempty_cons resolves in the flat ambient namespace exactly
> # like the raw ctor did, identical denotation, so every harness stays green.
> # Base = the P1-landed main (0e439f1e5 or later; re-measure the exact call-site
> # anchors at cut -- lines drift). Seat foundation gpt-5.6-terra/medium (T2)
> # correct. CENSUS = FIVE sites (Architect Option A, owns the earlier 3-count
> # miss): the two raw NonEmptyCons uses in Validation.ken.md's checked example
> # (:77/:86) are clients and belong in P2. Architect is the required reviewer on
> # the candidate (additive/mechanical); foundation-leader confirmed the scope.

## What this is

The second additive step of the Tier-C scaffold retirement. With the smart
constructors in place (P1), swap the five remaining raw-`NonEmptyCons` call sites
in NonEmpty's ambient clients to the smart constructor. Still ambient -- no module
boundary, no imports. Its purpose is to make the P3 flip safe: after P2, no source
outside NonEmpty's defining region names a raw constructor, so hiding the raw ctor
at P3 breaks nothing.

## Fixed inputs (census re-measured at cut on the P1-landed tree 0e439f1e5; re-measure again at your cut -- lines drift)

Exactly five raw `NonEmptyCons` call sites outside NonEmpty's defining module:
- `Application.CommandLine.ArgParse` (ArgParse.ken.md:252)
- `Application.Input.Schema` (Schema.ken.md:123 and :130)
- `Data.Sums.Validation` (Validation.ken.md:77 and :86) -- two raw NonEmptyCons
  applications inside the checked `ken example` (const name_failure/age_failure :
  NonEmpty String = NonEmptyCons String (...) (Nil String)).

No other catalog/examples source names raw `NonEmptyCons` (examples/ has none,
verified on 0e439f1e5). Every other NonEmptyCons hit is INSIDE NonEmpty.ken.md
(the defining module) -- transparent, not a consumer. Invalid/Valid have no
external raw consumers. Forge and Configuration.Decoder name the Validation/
NonEmpty TYPES only (no raw ctor) -- they are P3's import concern, NOT this node.

## Deliverables

- **D1 -- swap the five call sites** `NonEmptyCons a x rest` -> `nonempty_cons a x
  rest` at Schema.ken.md:123/:130, ArgParse.ken.md:252, and Validation.ken.md:77/
  :86. Identical denotation. Still ambient: add NO import lines, NO module
  boundary. Re-measure the exact lines at cut.

## Acceptance criteria

- **AC-SWAP.** Exactly those five call sites change from the raw ctor to
  `nonempty_cons`; the elaborated/reduced result is unchanged (control: the
  Schema/ArgParse checked examples/harnesses AND the Validation checked example
  produce the same values as before).
- **AC-NO-RAW-CONSUMER (the P3-enabling control).** After this node, a census
  over catalog/ + examples/ finds NO raw `NonEmptyCons` reference outside
  NonEmpty's own defining module. This is the invariant that makes P3 safe; assert
  it as a behavioral/grep control the node carries.
- **AC-NO-REGRESSION.** Full `-p ken-elaborator` green in CI; cc1/cc7/cc8 remain
  green (still ambient, no harness change). Targeted via `scripts/ken-cargo`,
  never `--workspace`.

## Gate, reviewer, sequencing

`gate: none`. Reviewer per candidate: **Architect** (additive/mechanical) +
**Foundation QA** + CI -> Steward M1-M4 -> lieutenant. Second of the 3-step DAG;
depends_on CAT-MIGRATE-TIER-C-NONEMPTY-SMART-CTORS, blocks the P3 flip
CAT-MIGRATE-TIER-C-NONEMPTY-VALIDATION.
