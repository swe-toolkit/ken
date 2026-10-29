---
id: LANG-PRELUDE-FLOOR-CONSOLE-AND-CHAR
title: "Give the last keyed candidates in the intrinsic ledger, the Console group and the three Char operations, a ruled disposition from both witnesses, admitting to the strict floor only an identity with a native exact-id reader and an independent source need, so that no ledger candidate row is unruled before the flip"
status: ready
owner: language
size: M
tier: T1
gate: architect
depends_on: [LANG-PRELUDE-FLOOR-FS-RESOURCE-FAMILIES]
blocks: []
github: null
origin: "Slice 5, the last of the keyed floor additions; LANG-PRELUDE-FLOOR-KEYED-ADDITIONS deliverable 2 lists Console and effects as a successor slice before the flip, and lanes.md L2 orders floor additions, then flip and enforce. Membership rule: spec 30 §4 (operator 2026-09-25, no intrinsic module). Steward-filed per COORDINATION section 2."
---

# The last keyed ledger candidates get a ruled disposition

## Objective

Every remaining keyed candidate in the intrinsic ledger has a ruled
disposition: floor, compiler-internal, or package. Only an identity with
both witnesses joins the floor. The spec 30 §4 roster and count, the
ledger rows and `PRELUDE_FLOOR_NAMES` agree on one branch. After this
slice, no ledger candidate row is unruled, which is the flip's entry
condition.

## Settled inputs (main after FS-RESOURCE, `c0b3b082a` tree)

- **The rule.** It is the same as slices 1 to 4. Both witnesses admit a
  type with its whole constructor family, per its scope choice. Keyed
  without source need means compiler-internal. Unkeyed but promised means
  an explicit-import package. Witnesses are per name.
- **The unruled rows.** Nine candidate rows in the ledger's "Reserved
  entrypoint names" table still carry no disposition: `IO`, `write`,
  `Stream`, `Stdin`, `Stdout`, `Stderr`, `eqChar`, `leqChar` and
  `charToInt`. Every other row in that table is floor, internal or
  package.
- **Already ruled, to be recorded rather than re-ruled.**
  - `Stream` and its three cases are keyed with no source need (37 §5).
    They are internal (LANG-PRELUDE-FLOOR-KEYED-EFFECTS).
  - `Console` is not admitted. The keyed family is `ConsoleOp`, which is
    compiler-internal (same frame).
  - In the slice 1 D0, `leqChar` and `charToInt` had no identity reader,
    so their default is package. `eqChar` is keyed by `decimal_char`
    `set_eq_entry`, and its source need is open
    (LANG-PRELUDE-FLOOR-KEYED-ADDITIONS).
- **Not yet a row.** `print_line` is a checked Console helper in
  `prelude.rs` with no ledger row. AC-0 gives it one.
- **Source use** (`git grep -w`, catalog, examples, conformance and
  `crates/*/tests`, on `c0b3b082a`):

  | Name | Lines | Files |
  |---|---|---|
  | `IO` | 30 | 15 |
  | `write` | 100 | 24 |
  | `eqChar` | 19 | 5 |
  | `leqChar` | 17 | 5 |
  | `charToInt` | 50 | 6 |

  These counts are spelling hits, not consumers. The census in
  deliverable 2 replaces them.
- **The mechanism is slice 1's.** It is `PRELUDE_FLOOR_NAMES`, floor-id
  admission and `scope_constructor_family`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

1. **AC-0 first: the witness table.** The Architect's D0 names the native
   producer or reader of the exact id for each of the ten names, or
   records that none exists. For `IO`, that means stating whether the
   effect reifier selects the alias id or only the underlying `ITree`
   constructors. The Spec Leader rules each row's independent source
   need and any specificity rename. `write` is one of the operator's
   probes.
2. **Census, on the cut base.** Measure, by `git grep -w` outside
   `crates/*/src`:
   - the consumers of every name whose disposition moves it out of bare
     resolution;
   - the source declarations that collide with an admitted name.
3. **One spec+crates branch.** It changes the 30 §4 roster and count (if
   anything is admitted), the ten ledger rows and `PRELUDE_FLOOR_NAMES`
   together. It migrates the census token-aware. A package-disposition
   name gets its explicit-import home in the same branch, so no client
   is stranded. The Spec co-turn owns the `spec/` text, and CV votes on
   `spec/` and `conformance/`.

## Acceptance

- **AC-1.** In strict and legacy modes:
  - each admitted name resolves to its exact registered id, and a
    same-name source declaration of it is refused;
  - each internal or package name is absent from `PRELUDE_FLOOR_NAMES`,
    and a same-name source declaration of it is accepted in strict mode;
  - a package name resolves through its explicit import to the id its
    clients used on the cut base.
- **AC-2.** The spec count equals `PRELUDE_FLOOR_NAMES.len()`. In the
  ledger's "Reserved entrypoint names" table, every candidate row carries
  a disposition. The census diff is deletion-only, with predicted pairs.
- **AC-3.** `trusted_base()` is unchanged as an identity set.
- **AC-4 (mutation, QA).**
  - Removing an admitted name from `PRELUDE_FLOOR_NAMES` reddens its AC-1
    refusal row.
  - Adding `Stream` to `PRELUDE_FLOOR_NAMES` reddens its strict
    acceptance row.

## Stop conditions

- An admission that the AC-0 table does not support with both witnesses.
- A rename that changes a spec 36, 37 or 38 operation's public type.
- Any kernel or trust change, or the flip itself, which stays with L2-4.

Checks: 6 fired: the 104 rows on `c0b3b082a` were filtered for a floor,
internal or package marker. 35 lack one. Two are private floor
constructors, 24 are helpers whose row prose places them in a package or
deletes them, and the remaining nine are this slice. 3 fired: the
spelling counts span catalog, examples, conformance and `crates/*/tests`.
They are not the migration population, which deliverable 2 measures on
the cut base. 8 fired: AC-1's strict acceptance row for an internal name
fails if the slice admits by family label, and AC-4's `Stream` mutant
makes that failure observable.
