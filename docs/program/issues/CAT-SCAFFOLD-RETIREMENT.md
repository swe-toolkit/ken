---
id: CAT-SCAFFOLD-RETIREMENT
title: "Umbrella: retire the fixture-scaffolding-authored catalog modules and migrate every kept module off scaffolding onto the real prelude + module/import, so no catalog module depends on fixture scaffolding. Executed as the Architect's bottom-up 5-tier DAG (publish provider export surfaces, then repoint consumers to imports, tier by tier)."
status: active
owner: foundation
size: L
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Operator ruling 2026-09-02 (L3 re-scope from module-system semantics-fix to scaffolding retirement) + Architect decomposition evt_2e0pee5jxzv07. See lanes.md OPERATOR RULING 2026-09-02 block (commit 43f1c20a2) for the durable operator text and the Flag-2 resolution."
---

> # The scaffold-retirement migration. Umbrella node; the work lands as tier WPs.
>
> This tracks the arc; it does not itself carry a candidate. Each tier is a
> separate WP framed and released one at a time (one release ahead), reviewed by
> the Architect (required soundness/design reviewer per tier) plus Foundation QA
> + CV, then Steward M1-M4.

## Objective (operator, 2026-09-02)

Type classes are uniform across all modules in a compilation BY DESIGN — the
wholesale class-install side effect is the intended enforcement, not a missing
semantics. So the "standalone elaboration failure" measured the wrong thing;
there is no new module-system semantics to build. Fixture scaffolding is
TRANSIENT: modules authored against it are not the production language surface.
End state = **zero catalog dependence on fixture scaffolding**, reached by (1)
retiring the scaffolding-authored modules and (2) migrating every kept module off
scaffolding onto the real prelude + module/import.

## Partition axis (Architect evt_2e0pee5jxzv07)

The cut is the MEASURED value-dependency DAG over catalog VALUE edges, NOT the
directory tree and NOT the class mechanism. Directory home does not dictate
migration order (Core.Logic.Transport and Data.Collections.Derived are
dependency-BOTTOM providers; Core.Classes sits ABOVE them; Data.Serialization.Json
is a top-tier CONSUMER of Capability.Parsing.Cursor). Each tier's WP publishes its
OWN modules' export surfaces AND repoints those same modules' consumption to
already-published lower tiers — no consumer-only WP ever references an unpublished
provider.

## The five tiers

- **Tier A — primitive providers** ([[CAT-MIGRATE-TIER-A-PROVIDERS]], `merged`
  — squash `f1d7d4133`, 2026-09-03).
  Pure export publication, no consumer edits. Census folded it to
  `Derived::{nth, bytes_nat_length}`; Transport/Compare/Arithmetic/Nat.Order are
  already sufficiently published.
- **Tier B — Core.Classes + primitive-instance consolidation.** RECUT after the
  foundation D0 hard-stop (Architect evt_21c0cdvnmv3f3 + Steward evt_7bzffq1q90rr8)
  into a provider-publication predecessor + a relocation successor, honoring the
  DAG "no consumer-only WP references an unpublished provider" axis:
  - P = [[CAT-MIGRATE-TIER-B-PROVIDERS]] (`active`, released 2026-09-03): LC
    publishes its own `class DecEq` + `fn bool_eq`; StringBijection (a
    Tier-A-missed Data.Text provider) clean-ifies + publishes
    `string_to_list_char_injective`.
  - [[CAT-MIGRATE-TIER-B-CLASSES]] (`draft`, gated on P): the WHOLE DecEq
    UInt8/Bytes/String relocation into LC + EmptyDec consolidation + BytesKeys /
    StringKeys / EmptyDec standalone.
  - Off the critical path (census error — EC was NOT clean):
    [[CAT-MIGRATE-EC-FUNCTOR-IMPORT]] (`ready`, framed 2026-09-03,
    `depends_on: []`) clean-ifies EC (LF publishes `Functor`/`comp`/`idf`/
    `list_map`; EC imports them → standalone-green). Runs in parallel, not a
    regression fix (EC elaborates in the full-catalog build via ambient today).
    Do NOT preempt P with it. LF (LawfulFunctors) itself is genuinely clean via
    `CAT-LAWFULFUNCTORS-STANDALONE-IMPORT`.

  **The class-owner relocation lands here and gates every DecEq/class consumer
  below.** Operator ruling 2026-09-02
  (Flag 2, verbatim "Move DecEq UInt8, etc to LC"): the orphaned primitive-type
  `DecEq` instances — `DecEq UInt8` and `DecEq Bytes` (from BytesKeys),
  `DecEq String` (from StringKeys) — RELOCATE with their eq/sound/complete wiring
  to LC (the class owner), NOT a new prelude; BytesKeys/StringKeys become pure
  consumers. EmptyDec's DUPLICATE local class `DecEq` / `fn bool_eq` /
  `instance DecEq Bool` CONSOLIDATE into LC (import, retire the local
  redeclarations). The retired-as-standalone-artifact generated instance
  dictionaries (EC `Functor_instance_Identity`, Validation
  `Functor_instance_Validation`, NonEmpty `Semigroup_instance_NonEmpty`) resolve
  ambiently in the whole-catalog build (Flag 1, resolved by the operator's
  class-uniformity ruling) — they carry no WP work and are not published as
  ordinary exports.
- **Tier C — Data value modules.** SB -> StringKeys; BK; NE; Map; Codec;
  Validation; Deque; Vector; Sums.Combinators. Gated on Tier B (all consume
  classes).
- **Tier D — Capability**, internally ordered: DC + Doc -> Cursor -> Decoder ->
  Numeric / Parsing / Process.Arguments; Diagnostics.Render; Filesystem.Path.Posix
  (gated on BK's DecEq relocation, Tier B). System.IO carries a one-module
  theorem-rename erratum (rename the `write_all_all_success` theorem so it does
  not shadow its subject fn; keep the subject fn), folded into its Tier D step.
- **Tier E — Serialization + Application + Algorithms.** Json (imports Cursor —
  now available; THIS is where Json belongs) -> Application.Input.Schema ->
  Application.CommandLine.ArgParse + Application.Configuration.Decoder;
  InsertionSort, OrderedSearch, Gcd, Property.

Intra-tier ordering inside D and E is real (Cursor before Decoder before Parsing;
Input.Schema before ArgParse/Config.Decoder) and must be respected within
whatever WP granularity each tier's frame chooses.

## Non-collapse (Architect, carries; no new ruling)

The differing-signature insert/sort/compare/compose/empty/lookup/sorted_tail
homonyms and Nat.Order's deliberately package-local `OrdResult` are correctly NOT
collapsed. Duplicate-map attached-owner work (Gcd subst/cong/sym/trans -> T;
Property ByteCursor -> Parsing; Map/LF bool_and families -> LC) lands with its
tier, not as a blind delete.

## Sequencing

Tier A is `ready` and released. Each subsequent tier is framed one release ahead
as its predecessor lands, with fixed inputs re-measured at that SHA. The Architect
is the required reviewer on each migration WP.
