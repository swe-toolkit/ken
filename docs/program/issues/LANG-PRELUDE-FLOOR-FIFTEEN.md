---
id: LANG-PRELUDE-FLOOR-FIFTEEN
title: "Realize the spec's fifteen-member prelude floor in strict resolution (add Bottom, Equal, Prop, Proved, Top to PRELUDE_FLOOR_NAMES), opened by the D0 that measures what the legacy fall-through and local binders reach outside the minimal built-in set B across every unit class; first slice of the single-mode, minimal-prelude program"
status: active
owner: language
size: M
gate: architect
tier: T1
depends_on: []
blocks: []
github: null
origin: "Operator rulings 2026-09-25: built-ins cannot be overridden; 'they should be the minimal set required'; 'any convenience names that are not required by the prelude rules above (or the kernel built-ins) should be considered technical debt and moved to packages'; local binders included; the elaborator work is L2's. Program design Architect evt_1ymf3p94jygx3 and evt_4s5he6tnf3xs4 (slices L2-1 and the floor half of L2-2). Steward-filed per COORDINATION section 2."
---

# Realize the fifteen-member floor, measured first

## Objective

Strict resolution admits exactly the spec's closed floor of fifteen types
(`spec/30-surface/30-taxonomy.md §4`), and the program knows, by measurement,
every place outside the catalog that still leans on the legacy fall-through or
binds a name in B.

## Settled inputs -- Architect, measured at `50966beb2`

- **B**, the built-in set: the fifteen floor types {Auth, Bool, Bottom, Char,
  Equal, List, Nat, Option, Pair, Prop, Proved, ResourceKind, Result, Top,
  Utf8Error}, their constructors, `mk_pair`/`pair_fst`/`pair_snd`, and kernel
  vocabulary (native trusted base, `Omega`, reserved sugar). About 135 of the
  503 registered globals.
- `modules.rs::PRELUDE_FLOOR_NAMES` (`:123`) still holds the old ten; `Proved`
  is special-cased into `strict_builtin_names` (`:251`). 30 §4 already fixes
  fifteen; 33 §3 is stale and is reconciled by Spec S0.
- For catalog roots, legacy reach outside B equals the census `expected` (47
  rows, 87 names at `50966beb2`). Other unit classes are unmeasured.
- Inventory of all 503 names: `architect/work` `24f413275`,
  `notes/prelude-global-inventory-50966beb2.md`. Its keying column is a lower
  bound, not a measurement.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

`PRELUDE_FLOOR_NAMES` becomes the fifteen, admitted at their exact
pre-source identities, with no second identity and no fallback route. The
census `expected` shrinks by deletion only. `space` desugaring uses an `Empty`
identity captured before source elaboration and compared by exact ID, not a
by-spelling read of the live globals map.

## Acceptance

- **AC-0 (D0, scratch only, post to the WP thread, then proceed).** With
  B-only resolution and the no-binding rule installed in scratch:
  - (a) Fall-through reach over every unit class (catalog roots, isolated
    files, examples, conformance, `ken-cli` fixtures, elaborator suites), as
    (unit, suite, name). The catalog part must reproduce the census; that is
    the positive control.
  - (b) Binder reach, by form. It must find `modules.rs:4461` `fn keep
    (Proved : Nat)` and `lang_mod_pair_floor_realization.rs:364` `fn lexical
    (pair_fst : Bool)`; that is the positive control.
  - (c) Per-name keying for every registered global outside B, by producer
    reading (registration to every reader), not by grep.
  - (d) Every non-source path into `globals`: `declare_postulate_raw`,
    sequential `elaborate_decl`, REPL sessions, tests' `globals.insert`.
- **AC-1.** Each of Bottom, Equal, Prop, Proved, Top resolves in strict mode
  to its exact registered `GlobalId`. A source declaration of the same name
  and shape is still rejected. A strict unit naming a non-floor prelude global
  (for example `And`) still fails `UnboundName`.
- **AC-1a (`Empty` capture, Architect `evt_2esx3xsv7sm4g`).** At
  `ae5cce97f`, `elaborate_space_decl` reads `globals.get("Empty")` by spelling
  at each `space` (`elab.rs:13450`), so a same-program `data Empty` rebinds it.
  Capture the prelude `Empty` in a `PreludeEnv` field before source
  elaboration, as `Proved`'s fixed identity is (`modules.rs:258-261`), and use
  it there. Controls (Architect `evt_6drg78c0zz7dz`), each red on base:
  - `data Empty (a : Type) : Type where {}` followed by a `space` checks
    instead of failing `TypeMismatch`;
  - with `data Empty : Type where { Oops : Empty }` before a `space`, the
    `GlobalId` in the residual position of the elaborated `Counter.get` type
    equals the `PreludeEnv` capture and differs from the source `Empty`'s;
  - with `"Empty"` removed from the globals map after `ElabEnv::new()` (as
    `modules.rs:4534` removes `"Proved"`), a `space` still elaborates, with
    the captured identity in the residual position; on base it fails
    `Internal("space desugaring requires the prelude Empty type")`;
  - reverting the capture reddens all three.
- **AC-2.** The census diff is deletion-only. The deleted (row, name) pairs are
  predicted before the build and measured after it; an unpredicted deletion is
  a finding. If `CAT-LOGIC-PRELUDE-MOVE` lands first, its row
  `(Core.Logic.Not, [Bottom])` moves to the clean set here and is predicted.
  No `trusted_base()` change. Targeted builds only, through
  `scripts/ken-cargo`; no-regression means green in CI.

## Stop conditions

- Spec S0's reconcile of 33 §3 to fifteen has not landed when this is ready to
  route: hold the route, not the work.
- A floor member cannot be admitted at its exact pre-source identity.
- Any `trusted_base()` growth.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the child-2 checkpoint.

## After landing

AC-0 (d) is the population for the session-scope slice. AC-0 (c) is the input
to the per-name routing of keyed names.
