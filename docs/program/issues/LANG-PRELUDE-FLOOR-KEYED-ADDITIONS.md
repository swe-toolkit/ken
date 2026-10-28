---
id: LANG-PRELUDE-FLOOR-KEYED-ADDITIONS
title: "Admit to the reserved strict prelude floor each identity that a whole-ledger D0 proves keyed by a native mechanism and independently required by the surface contract (spec 30 §4), starting with the entrypoint-ABI family (ExitCode, ProcessInput, ProgramCaps, HostIO, the Prod/MkProd disposition), with the spec roster and count changing atomically with PRELUDE_FLOOR_NAMES"
status: merged
owner: language
size: M
tier: T1
gate: architect
depends_on: [LANG-PRELUDE-FLOOR-FIFTEEN, LANG-QUALIFIED-CONSTRUCTORS, LANG-MODULE-PATH-OWNER-GATE, LANG-PROVE-DECL-MODULE-IDENTITY]
blocks: []
github: null
origin: "Operator 2026-09-25: one resolution mode, with the minimal prelude and an ever-present intrinsic's name reserved; no intrinsic module; the Prod/zip disposition belongs to the floor additions. Slice L2-3 of the single-mode program, named by the Architect (evt_3zm3021fs1ess). Steward-filed per COORDINATION section 2."
---

# The strict floor admits the keyed, source-required identities

## Objective

Strict resolution admits, as reserved floor names, exactly those prelude
identities that are both keyed by a native mechanism and independently
required by the surface contract (spec 30 §4). The spec roster and count
and `PRELUDE_FLOOR_NAMES` agree on one branch. This WP lands the first
family, entrypoint ABI.

## Settled inputs (Architect `evt_3zm3021fs1ess`, main `118d83f51`)

- **The rule.** Spec 30 §4 forbids admission "until D0 proves both
  witness clauses for a candidate". A ledger row alone, or registration
  in `globals`, proves neither. Each name is decided separately, there is
  no intrinsic module, and the final list is informational (operator
  Q-B).
- **The mechanism exists.** `PRELUDE_FLOOR_NAMES`
  (`crates/ken-elaborator/src/modules.rs:190`, `[&str; 15]`), floor-id
  admission (`:346`, `:459`) and `install_prelude_floor` (`:278`) admit
  exact pre-source identities (LANG-PRELUDE-FLOOR-FIFTEEN). The scoped
  `T.C` property is from LANG-QUALIFIED-CONSTRUCTORS.
- **The keying population.** `canonical_runtime_roles` (`prelude.rs:43-233`,
  109 role rows), `PreludeEnv` (`:337`, 88 captured ids) and
  `pre_source_names` (`:363`).
- **The spec side.** The roster and count are in
  `spec/30-surface/30-taxonomy.md` §4 (about `:183-280`). The candidate
  rows and the per-type scoped-constructor table are in
  `spec/30-surface/30-intrinsic-ledger.md`. The ledger's note that
  Empty's fixed-ID capture has not landed is stale: `empty_id` is at
  `prelude.rs:388`, used at `elab.rs:20897`.
- **The first family.** `main`'s signature is source-named (33 §3.2.1),
  so this family has the strongest source clause:
  - ExitCode, scoped, `Success`/`Failure`;
  - ProcessInput, unscoped, `MkProcessInput`;
  - ProgramCaps, scoped, `MkProgramCaps`;
  - HostIO;
  - Prod/MkProd, reserved or retargeted to the floor Pair.
- **Consumers of the count** (Check 3). These are `modules.rs`,
  `elab.rs` and `conformance/surface/taxonomy/minimality.md`, plus these
  tests:
  - `cat_ord_nat_canonical_owner`, `cat_tier_d_posix_import`,
    `cat_transport_pub_export` and `lang_mod_catalog_evidence_frontier`,
    under `r_layer_tests`;
  - `lang_mod_nat_floor_realization`, `lang_mod_pair_floor_realization`,
    `lang_mod_strict_resolution_d0`, `rt_dasm_d1b_role_a_role_authority`
    and `cat_symbolic_algebra_nat_polynomial`;
  - `ken-runtime` `lowering/core/tests/effects.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. **AC-0 D0 (Architect), on main.** One table over every ledger row and
   every roles, `PreludeEnv` and `pre_source_names` identity outside B.
   Each row gives:
   - the name and its exact GlobalId producer;
   - every reader, traced from the producer rather than grepped by
     spelling;
   - the source-need citation, or "none";
   - a disposition: floor (scoped or not), compiler-internal, package,
     or remove-after-reach.
   It also closes the FLOOR-FIFTEEN residual: the 222 `NO_READER_FOUND`
   names get a producer and reader closure. It must reproduce four
   controls:
   - the Pair family is keyed and floor;
   - Empty is keyed with no source need, so internal-only;
   - leqChar and charToInt have no identity reader, so package;
   - eqChar is keyed by `decimal_char` `set_eq_entry`, with source need
     open.
   The Architect rules the keying column. The Spec ring rules source
   need and spelling.
2. **Build, on a base with MPOG and PROVE-DECL landed.** One
   spec+crates branch admits the entrypoint-ABI family. It changes the
   30 §4 roster and count, the ledger rows and `PRELUDE_FLOOR_NAMES`
   together, with any 35/38 spelling rename. The Spec co-turn on the
   same branch owns the source-need column, the spelling-specificity
   check and any rename, and CV votes on the `spec/` paths. The later
   families are successor slices from the same D0 table, each landing
   before the flip:
   - Console and effects;
   - IOError and FileError;
   - the Resource family;
   - Instant, CreatePolicy, the Buffer family and TransferCount.

## Acceptance

- **AC-1.** In both strict and legacy resolution modes, each admitted
  type resolves to its exact registered id. A scoped family resolves
  only as `T.C`, so `ExitCode.Success`, `ExitCode.Failure` and
  `ProgramCaps.MkProgramCaps` resolve to their ids, and bare `Success`,
  `Failure` and `MkProgramCaps` are refused. A same-name source
  declaration is refused (Architect `evt_4k303wsmrmj4z`).
- **AC-2.** The spec count equals `PRELUDE_FLOOR_NAMES.len()`. The census
  diff is deletion-only, with predicted pairs.
- **AC-3.** `trusted_base()` is unchanged as an identity set.
- **AC-4 (mutation, QA).** Each mutant fails and is restored:
  - removing one admitted name from `PRELUDE_FLOOR_NAMES` reddens its
    AC-1 row;
  - admitting a scoped constructor unscoped reddens the bare-name
    refusal row in both modes.

## Stop conditions

- A name with an open L3 prelude-move WP. Exclude it rather than add
  it.
- An admission that the D0 table does not support with both witness
  clauses.
- Any kernel or trust change, or the flip carries (those stay with
  L2-4).

Checks: 4 fired: the admission work is named in the code's own terms
(`PRELUDE_FLOOR_NAMES :190`, the floor-id map `:346`, the role roster
`:43-233`) from the Architect's read on `118d83f51`. 3 fired: the 13
consumers of the fifteen-name roster are listed from the same read. 8
fired: the AC-4 mutants pick out the property each AC-1 row claims, and
the D0's Empty and leqChar controls are names a census-reach proxy would
wrongly admit.
