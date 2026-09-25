---
id: CAT-LOGIC-PRELUDE-MOVE
title: "Move the prelude's logic conveniences into catalog packages: And, and_intro, and_fst, and_snd and Not into a Core/Logic home, and Dec/Yes/No/Empty and decide declared as checked definitions in Core.Logic.EmptyDec, with every consumer importing them; opened by the identity guard that proves no compiler code or other prelude definition holds each moved identity; first L3 slice of the minimal-prelude program"
status: active
owner: foundation
size: M
gate: architect
tier: T1
depends_on: []
blocks: []
github: null
origin: "Operator rulings 2026-09-25: the prelude is the minimal set required; 'any convenience names that are not required by the prelude rules above (or the kernel built-ins) should be considered technical debt and moved to packages'. Program design Architect evt_1ymf3p94jygx3 and evt_4s5he6tnf3xs4 (slice L3-1, arms C and D). Steward-filed per COORDINATION section 2."
---

# Move the logic conveniences out of the prelude

## Objective

Every catalog module that uses `And`, `and_intro`, `and_fst`, `and_snd`,
`Not`, `Dec`, `Yes`, `No`, `Empty` or `decide` resolves it through an import
of a catalog package, never through the prelude fall-through.

## Settled inputs -- AC-0 measured at `17147272c`, Architect `evt_6dvt9fgjppwbk`

**Corrected 2026-09-25.** The first cut claimed EmptyDec already defined
`Empty`/`Dec` and that its row listed `Not`; both were false (AC-0,
`evt_4a5672hbnreeb`).

- **Transparent, movable:** `And`, `and_intro`, `and_fst`, `and_snd`, `Not`
  are transparent `declare_def`s (`prelude.rs:930-1029`, `:1153-1170`). A
  catalog copy with the same body delta-unfolds to a convertible term, so a
  prelude body that mentions one (`is_sorted`, `:1277-1305`) does not hold its
  identity. `PreludeEnv.and_id` and `issorted_id` are storage with no reader.
- **Inductive, one unit:** `Dec`, `Yes`, `No` and prelude `decide`
  (`:1464-1515`). `No`'s type embeds `Empty`. No compiler code reads any
  member beyond registration. Their only census consumer is the EmptyDec row.
- **`Empty` is keyed and internal-only.** `elaborate_space_decl`
  (`elab.rs:13449-13456`) bakes the prelude `Empty` into space desugaring. No
  source contract names it (`36 §4.1-4.2`), so under the operator's Q-B rule
  the compiler keeps that identity internally and the source name `Empty` is a
  package. The catalog `Empty` is a lawful second identity. The crates change
  (hold the id in a `PreludeEnv` field) is L2's, not this WP's.
- **EmptyDec today:** `data Empty`/`data Dec` sit in a `ken ignore`
  illustrative fence (`EmptyDec.ken.md:47-56`); its checked fences consume
  the prelude names. `Not` is on the `Data.Collections.Map` row.
- `is_sorted` stays in the prelude here; it moves in the L3 collections slice,
  which lands before the L2 flip.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

- A `Core/Logic` home for `And`, its three helpers and `Not`, with the same
  statements (the Architect picks the module).
- EmptyDec's illustrative fence becomes checked declarations: `Empty : Type0`
  with no constructors, `Dec (P : Omega) = Yes P | No (P -> Empty)` over that
  `Empty`, and `fn decide`, all with the prelude's statements.
- Every consumer imports them. The prelude registrations stay until the L2
  flip. No `crates/` change except test expectations.

## Acceptance

- **AC-0 (done, `evt_4a5672hbnreeb`).** Identity guard as ruled above.
- **AC-0a (before any consumer edit).**
  - (i) Count catalog modules that both declare a `space` and name `Empty`.
    Measured zero at `17147272c`; non-zero is a STOP.
  - (ii) Elaborate a module declaring a name the prelude still registers
    (`data Empty`, `data Dec`, `And`). If the elaborator refuses to redeclare
    a prelude-registered name, STOP: every move assumes it is admitted.
- **AC-1.** Every catalog module that used a moved name imports it and the
  catalog checks. Attached proofs keep their statements. In particular,
  `Data.Collections.Derived`'s four sort-law proofs, which consume prelude
  `is_sorted` and would now build with catalog `and_intro`/`and_fst`, still
  check. If they do not, STOP: `is_sorted` would have to move here.
- **AC-2.** The census diff is deletion-only. The deleted (row, name) pairs
  are predicted before the build and measured after; an unpredicted deletion
  is a finding. No `trusted_base()` change. Targeted builds only, through
  `scripts/ken-cargo`; no-regression means green in CI.

## Stop conditions

- AC-0a (i) is non-zero, AC-0a (ii) refuses, or AC-1's sort-law proofs fail.
- A consumer's statement changes, or a proof needs the prelude identity.
- Any `trusted_base()` growth, or any new ambient name in the census.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the child-2 checkpoint.
