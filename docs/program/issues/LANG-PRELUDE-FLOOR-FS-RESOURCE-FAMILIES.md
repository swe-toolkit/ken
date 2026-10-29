---
id: LANG-PRELUDE-FLOOR-FS-RESOURCE-FAMILIES
title: "Admit to the reserved strict prelude floor the FS and resource identities that have both witnesses, keyed by a native reader of the exact id and required by name in source, each type with its ledger scope choice, and keep every other ledger candidate in this group internal or package, with the roster, count and PRELUDE_FLOOR_NAMES changing together"
status: ready
owner: language
size: L
tier: T1
gate: architect
depends_on: [LANG-PRELUDE-FLOOR-ITREE]
blocks: []
github: null
origin: "Slice 4 of the keyed floor additions; lanes.md L2 names the FS/resource families as next after LANG-PRELUDE-FLOOR-ITREE. Membership rule: spec 30 §4 (both witnesses admit; operator 2026-09-25, no intrinsic module). Candidate rows: spec 30-intrinsic-ledger.md. Steward-filed per COORDINATION section 2."
---

# The strict floor admits the witnessed FS and resource identities

## Objective

Every FS or resource identity that is both keyed by a native reader of its
exact id and required by name in source resolves as a reserved floor name, with
its ledger scope choice. Every other candidate in the group stays internal or
package material. The spec 30 §4 roster and count, the ledger rows and
`PRELUDE_FLOOR_NAMES` agree on one branch.

## Settled inputs (spec 30 §4 and the intrinsic ledger)

- **The rule.** Both witnesses admit a type to the floor with its whole
  constructor family, per its scope choice. Keyed without source need means
  compiler-internal. Unkeyed but promised means an explicit-import package.
  Witnesses are per name: a family label proves nothing for its members.
- **The candidate group** (ledger, "Reserved entrypoint names"):
  - `FS`, `read_bytes`, `write_file`;
  - `FileError`, and `IOError` with its thirteen causes;
  - `CreatePolicy` with three cases;
  - `BufferSpan`, `BufferWindow`, `TransferCount`;
  - `ResourceError` with twelve constructors;
  - `ResourceBodyResult` and `ResourceBracketResult`.
- **Scope choices are already ruled** (ledger, "Per-type constructor scope
  choices"). `IOError`, `ResourceError`, the two bracket results,
  `CreatePolicy`, `BufferSpan` and `TransferCount` are scoped if admitted.
  `FileError` and `BufferWindow` are unscoped. Constructor-private visibility
  never widens.
- **The mechanism is slice 1's.** It uses `PRELUDE_FLOOR_NAMES`, floor-id
  admission and `scope_constructor_family`. Constructor artifact leaves come
  from the checked id, per LANG-PRELUDE-FLOOR-ITREE.
- **Out of this slice.** `IO`, `Stream` and its cases, and `write` are the
  Console group. `Stream` is already ruled internal
  (LANG-PRELUDE-FLOOR-KEYED-EFFECTS).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

1. **AC-0 first: the witness table.**
   - The Architect's D0 names, for each candidate row, the native producer or
     reader of the exact id, or records that none exists.
   - The Spec Leader rules each row's independent source need and any
     specificity rename (`NotFound`, `Other` and `Unsupported` are the
     operator's probes).
   - Rows that have both witnesses are the admission set. The others get the
     disposition the rule gives them.
2. **Census, on the cut base.** List the bare consumers of every admitted
   scoped family's constructors, measured by `git grep -w` outside
   `crates/*/src`, plus the source declarations that collide with admitted
   names.
3. **One spec+crates branch.** It changes the 30 §4 roster and count, the
   ledger rows and `PRELUDE_FLOOR_NAMES` together, and migrates the census
   token-aware. The Spec co-turn owns the `spec/` text, and CV votes on
   `spec/` and `conformance/`.

## Acceptance

- **AC-1.** In strict and legacy modes:
  - each admitted type resolves to its exact registered id;
  - each scoped family's `T.C` resolves to its checked constructor id, and the
    bare spelling is refused;
  - a same-name source declaration of an admitted type is refused;
  - one row the table keeps internal is refused as a source name.
- **AC-2.** The spec count equals `PRELUDE_FLOOR_NAMES.len()`. The census diff
  is deletion-only, with predicted pairs.
- **AC-3.** `trusted_base()` is unchanged as an identity set.
- **AC-4 (mutation, QA).** Removing one admitted name from
  `PRELUDE_FLOOR_NAMES` reddens its AC-1 row. Admitting `IOError` unscoped,
  if it is admitted, reddens the bare `NotFound` refusal row.

## Stop conditions

- An admission that the AC-0 table does not support with both witnesses.
- A rename that changes a spec 38 operation's public type.
- Any kernel or trust change, or the flip, which stays with L2-4.

Checks: 6 fired: the candidate group is the ledger's FS and resource rows, and
the slice claims no member's witness. AC-0 measures each row individually,
because the ledger states that a shared reader is not a keying proof. 3 fired:
the migration population is the deliverable's census, not a count in this
frame, since ITree's landing moves it. 8 fired: AC-1's internal-row refusal
fails if the slice admits by family label instead of by the table.
