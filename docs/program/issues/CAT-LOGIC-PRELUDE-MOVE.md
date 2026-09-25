---
id: CAT-LOGIC-PRELUDE-MOVE
title: "Move the prelude's Not and its Dec/Yes/No/decide unit into catalog packages: Not into Core.Logic.Not, and Empty, Dec, Yes, No and decide declared as checked definitions in Core.Logic.EmptyDec, with every consumer importing them and an import-removal control per moved inductive name; first L3 slice of the minimal-prelude program"
status: merged
owner: foundation
size: M
gate: architect
tier: T1
depends_on: []
blocks: []
github: null
origin: "Operator rulings 2026-09-25: the prelude is the minimal set required; 'any convenience names that are not required by the prelude rules above (or the kernel built-ins) should be considered technical debt and moved to packages'. Program design Architect evt_1ymf3p94jygx3 and evt_4s5he6tnf3xs4 (slice L3-1, arms C and D); move units evt_6dvt9fgjppwbk, evt_7pqxp142fg39s, evt_16ve7jwxbzax3. Steward-filed per COORDINATION section 2."
---

# Move Not and the Dec unit out of the prelude

## Objective

Every catalog module that uses `Not`, `Dec`, `Yes`, `No`, `Empty` or `decide`
resolves it through an import of a catalog package, never through the prelude
fall-through.

## Settled inputs -- Architect, measured at `ae5cce97f`

- **`Not`** is a transparent `declare_def` (Pi to `Bottom`). Nothing reads it
  and no other prelude body uses it. A catalog copy is convertible with it.
- **`Dec`, `Yes`, `No` and prelude `decide`** (`prelude.rs:1464-1515`) are
  inductive, one unit. `No`'s type embeds `Empty`. No compiler code reads any
  member beyond registration.
- **`Empty` is keyed and internal-only.** `elaborate_space_decl` bakes the
  prelude `Empty` into space desugaring; no source contract names it, so under
  Q-B the compiler keeps that identity internally. The catalog `Empty` is a
  lawful second identity. The crates change is L2's.
- **EmptyDec today:** `data Empty`/`data Dec` sit in a `ken ignore` fence
  (`EmptyDec.ken.md:47-56`); its checked fences consume the prelude names.
- **The package path admits the redeclaration** (`evt_7pqxp142fg39s`). A file
  under `catalog/packages/` gets module-qualified constructors, so
  `guard_constructor_spelling` does not fire. A loose file does not, and
  refuses `Yes`. The guard is right and must not be weakened.
- **Consumers:** EmptyDec's checked fences, the `Not` user on the
  `Data.Collections.Map` row, and `Algorithm.Searching.OrderedSearch`
  (`Dec`, `Yes`, `No`, `Empty`). OrderedSearch is the census's single
  baseline-red residual (`lang_mod_strict_resolution_d0.rs:1389`), so the
  census cannot see its names.
- **Out of this slice:** `And`, `and_intro`, `and_fst`, `and_snd` and
  `is_sorted`. `And`'s body `λa b. Σ(_:a).b` at Ω has no catalog spelling
  until expression-position Σ lands (`LANG-EXPRESSION-SIGMA`); they move
  together in the L3 collections slice. OrderedSearch keeps its ambient
  `And`/`and_fst`/`and_snd` here. Do not create `Core.Logic.And`.
  `PriorityQueue`'s own `Empty` constructor stays untouched.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

- `Core.Logic.Not` with the prelude's statement.
- EmptyDec's illustrative fence becomes checked declarations: `Empty : Type0`
  with no constructors, `Dec (P : Omega) = Yes P | No (P -> Empty)` over that
  `Empty`, and `fn decide`, all with the prelude's statements.
- Every consumer imports them. The prelude registrations stay until the L2
  flip. No `crates/` change except test expectations.

## Acceptance

- **AC-0a (before any consumer edit).** On the real package path, not a loose
  file:
  - (i) zero catalog modules both declare a `space` and name `Empty`;
  - (ii) `Core/Logic/EmptyDec.ken.md` and `Core/Logic/Not.ken.md` with the
    moved declarations check.

  Either failing is a STOP.
- **AC-1.** Every consumer imports what it uses, and the catalog checks.
  Attached proofs keep their statements.
- **AC-2 (controls).** For each moved inductive name, on OrderedSearch and on
  EmptyDec's consumers, removing that name's import while keeping `Dec`
  imported reddens with a kernel `TypeMismatch`. A consumer that checks green
  without its import proves nothing.
- **AC-3 (census, Architect `evt_5mqbentknbf4c`).** The census diff deletes
  (row, name) pairs from existing rows, plus exactly one added row
  `(Core.Logic.Not, [Bottom])`: the leaf's statement `a -> Bottom` forces it
  until the L2 floor admits `Bottom`. Every name in it must already be ambient
  in an existing row. Any other addition is a STOP.
  - Predicted before the build and measured after: `Map` loses only `Not`;
    `EmptyDec` loses `Dec`, `Empty`, `No`, `Yes`, and whether its `Bottom`
    survives is predicted by source reading; no importer row moves.
  - OrderedSearch's names are predicted separately by source reading. If it
    leaves the residual set, that sentinel change is predicted.
  - Whichever of this slice and `LANG-PRELUDE-FLOOR-FIFTEEN` lands second
    re-predicts the `Core.Logic.Not` row.
  - No `trusted_base()` change. Targeted builds only, through
    `scripts/ken-cargo`; no-regression means green in CI.
- **AC-4 (non-catalog consumers, Architect `evt_1kkygg9f7psn8`).**
  - Enumerate every non-catalog source that loads or exposes OrderedSearch,
    `Data.Collections.Map`, `Core.Logic.EmptyDec` or `Core.Logic.Not`,
    transitive importers included. Look in `crates/*/tests`, `r_layer_tests`,
    `ken-cli` fixtures, `examples/` and `conformance/`.
  - Classify each loose source there that names `Dec`, `Yes`, `No`, `Empty`,
    `decide` or `Not` bare:
    - (a) consumes the moved value, so it exposes the catalog module;
    - (b) is independent;
    - (c) is a labelled prelude-identity transition sentinel.

    Post the list with the candidate. For `Not`, which is transparent, this
    sweep is the only detector.

## Stop conditions

- AC-0a fails, or an AC-2 control stays green.
- A consumer's statement changes, or a proof needs the prelude identity.
- Any `trusted_base()` growth, or any census addition beyond AC-3's one row.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the child-2 checkpoint.

## Symptom inventory

Append one line per hard stop; never rewrite history.

1. `And`'s transparent body `λa b. Σ(_:a).b` at Ω has no catalog spelling --
   keyed on a surface-grammar gap (Σ only in type position).
2. A non-catalog loose source (`cat_bsearch_acceptance.rs:198`) names bare
   `Dec` against OrderedSearch's catalog `Dec` -- keyed on bare-name
   resolution to the still-registered prelude identity during the L3→L2-4
   dual-identity window; invisible to the catalog-roots census and AC-2's
   catalog-scoped controls (Architect `evt_1kkygg9f7psn8`).
