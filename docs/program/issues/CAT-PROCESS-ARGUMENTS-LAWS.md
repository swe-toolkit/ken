---
id: CAT-PROCESS-ARGUMENTS-LAWS
title: "give argument_slice_location the two-way characterization its package's round_trip already models for process_arguments: an ArgLocation is produced exactly when the argument exists and the endpoints are ordered and in bounds, and the produced location carries index/start/end unchanged -- the next of the seventeen proof-backfill follow-ons, selected while CAT-PARSING-CURSOR-LAWS is parked"
status: ready
owner: foundation
size: S
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "One of the seventeen proof-backfill follow-ons named by docs/program/CATALOG-PROOF-COMPLETENESS-SURVEY.md, under operator ruling 2026-09-13 / PRINCIPLES #16. Selected and framed by the Steward 2026-09-20 on CAT-COMPARE-LAWS landing, while CAT-PARSING-CURSOR-LAWS stays parked at 6065b4993 behind the non-local same-GlobalId resolver precursor. Chosen because it is the smallest genuine gap in the seventeen whose precursors are all landed: nth::some_below_length and nth::at_or_beyond_is_none arrived with CAT-COLLECTIONS-NTH-LAWS. All current-code facts measured at origin/main ecf1850766cd497fd92c3bc5739a69a900216bbb."
---

# Process argument location laws

`Capability/Process/Arguments.ken.md` proves `process_arguments::round_trip`
and nothing else. Its second public operation, `argument_slice_location`,
carries its entire guarantee in an elaborator acceptance test. Give it the
general law, over the representation the package already ships.

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## Settled inputs — measured at `ecf185076`. Do not re-derive.

The function is three guards and four outcomes
(`catalog/packages/Capability/Process/Arguments.ken.md:67`):

```ken
pub fn argument_slice_location
      (index : Nat) (start : Nat) (end : Nat) (arguments : List Bytes)
    : Option ArgLocation =
  match argument_at index arguments {
    None ↦ None ArgLocation;
    Some argument ↦
      match leq_nat start end {
        False ↦ None ArgLocation;
        True ↦
          match leq_nat end (bytes_nat_length argument) {
            False ↦ None ArgLocation;
            True ↦ Some ArgLocation (MkArgLocation index start end)
          }
      }
  }
```

**The precursors are landed, which is why this node is selected now.**
`argument_at index arguments` is `nth Bytes index arguments`, and
`CAT-COLLECTIONS-NTH-LAWS` merged `nth::some_below_length` and
`nth::at_or_beyond_is_none` (`Data/Collections/Derived.ken.md:177,191`). The
first guard is therefore discharged against landed general `List` facts rather
than re-derived in this package.

**`bytes_nat_length` is ordinary checked Ken, not a primitive** —
`pub fn bytes_nat_length (bs : Bytes) : Nat = length UInt8 (bytes_to_list bs)`
(`Derived.ken.md:928`). The third guard needs no TCB contract and no deferral.

**`ArgLocation`'s projections are PRIVATE and field identity must go through
the constructor.** `Capability/Parsing/Cursor.ken.md:69` declares
`data ArgLocation = MkArgLocation Nat Nat Nat` and exports exactly
`ArgLocation, MkArgLocation`; `arg_location_index`, `arg_location_start`, and
`arg_location_end` are unexported helpers of that package. State every field
claim as `MkArgLocation index start end`. Do not reach for a projection, and
do not export one.

**The external evidence being replaced** lives in
`crates/ken-elaborator/tests/cc6a_process_arguments_exit_acceptance.rs`, in
`structural_slice_location_keeps_nonzero_argument_and_range`, which runs index
2, start 3, end 5 against a 6-byte argument plus one out-of-bounds probe. That
test stays; it stops being the guarantee.

**The in-package exemplar** is `pub proof round_trip for process_arguments`
(`Arguments.ken.md:33`) — attached, public, transparent. Match its posture.

## Deliverable

Attached public proofs on `argument_slice_location` stating the two-way
characterization, in the shape `pair_compare::eq` / `eq_cases` already
establishes in the corpus:

- **soundness** — given `argument_at index arguments = Some argument`,
  `leq_nat start end = True`, and
  `leq_nat end (bytes_nat_length argument) = True`, the call equals
  `Some ArgLocation (MkArgLocation index start end)`.
- **completeness** — given the call equals `Some ArgLocation loc`, recover all
  three premises **and** `loc = MkArgLocation index start end`. The three
  impossible branches close with `absurd`.
- **refusal** — each guard independently returns `None ArgLocation` with the
  other two premises satisfied.

No new function, carrier, import, module, instance, primitive, postulate,
`Axiom`, or trusted entry. `trusted_base()` delta stays zero, and
`Cursor.ken.md` is not touched.

## Acceptance criteria

**AC-1 — the success law pins each field separately, and a field swap reds.**
A checked witness instantiates the law at three *distinct, nonzero* values with
`start < end < bytes_nat_length argument` (the acceptance test's 2/3/5 on a
6-byte argument qualifies), deriving `Some ArgLocation (MkArgLocation 2 3 5)`
through the law. Mutation, restored byte-exact: swap `start` and `end` in the
law's stated constructor and it must RED. A law that carried the right *count*
of `Nat` fields in the wrong order passes a fixture and must not pass this.

**AC-2 — each of the three guards is independently load-bearing.** Three
separate refusal witnesses, each satisfying the other two premises: an index at
or beyond the length (via `nth::at_or_beyond_is_none`); ordered endpoints
violated with the argument present and the end in bounds; the length bound
violated with the argument present and the endpoints ordered. A law proving
only the conjunction of the three satisfies a single combined negative and
fails here.

**AC-3 — the completeness direction genuinely reaches `absurd`.** Removing
`absurd` from any one of the three impossible arms reds that proof locally.
Report which arm was mutated and its diagnostic.

## Stop condition

Hand back rather than work around if either holds:

- the completeness direction cannot be stated without a projection function or
  a new export/import. Do not export `arg_location_*`, do not add an import,
  and do not restate the function in this package.
- any `catalog_ambient_passthrough_migration_census` row other than
  `Capability.Process.Arguments` moves. This package already imports from
  `Core.Classes.LawfulClasses`, so a moved OTHER row is a Steward stop, not a
  mirroring update.

## Not this node

- General laws for `process_argument_at`, `argument_at`, or the private
  `argument_bytes_at`. Adjacent, and not the surveyed gap.
- Any edit to `Capability/Parsing/Cursor.ken.md`.
- `CAT-PARSING-CURSOR-LAWS`, which stays parked.
- Machine-checked complexity bounds.
