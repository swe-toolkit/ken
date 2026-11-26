---
id: RT-NATIVE-AGGREGATE-DATA-NOT-CODE
title: "Native lowering transfers a run-time-selected value into a carrier with an inline case split over the whole alternative tree of the value's type, at every transfer site, so per-site code grows with the type's width. PX11 Inc2's exchange reaches one function, u2:59, of about 1.49M instructions, in which carrier allocation and transfer are 75% of the values, and Cranelift refuses it as CodeTooLarge. Emit a transfer whose static template contains a run-time selection once, as an out-of-line helper per (owner, seat, template), and call it from every site"
status: ready
owner: runtime
size: L
tier: T1
gate: architect
depends_on: []
blocks: [PX11-NATIVE-STREAM-SOCKET-PARITY]
github: null
origin: "Architect recut evt_1teeny8aaxj7y (PX11 Inc2 §1b entries 13-15), corrected by the AC-0 measurement evt_2ry7fef3x4b9r and the shape ruling evt_18xe7x3rjaqj5, after research advisories evt_3c0j7pd627wvn and evt_5x0v73wpgth68. L1 Linux ABI program, native parity. Steward-filed per COORDINATION section 2. The id predates the AC-0 finding that the hot values are selected at run time."
---

# A carrier transfer is bounded by its site, not its type

## Objective

The code that a carrier transfer emits at a site is bounded independently
of the width of the transferred value's type. A transfer whose static
template contains a run-time selection is emitted once per (owner, seat,
template) as an out-of-line helper, and every site calls it.

## Settled inputs (Architect AC-0 and shape ruling; anchors re-checked at release)

- **The failing function.** On PX11 Inc2 WIP `34cc2134f`, F = `u2:59` has
  990,992 values and 1,493,949 instructions. No other function exceeds
  31,052 values. Carrier allocation is 0.41 of F's values and transfer
  0.34.
- **The cap.** `CodegenError::CodeTooLarge` on x86_64 Linux comes only from
  `VRegAllocator::alloc` reaching regalloc2's `VReg::MAX` (2^21 - 1). It is
  a per-function cap (cranelift-codegen 0.113.1, regalloc2 0.10.2).
- **The hot values are selected at run time.** At origins 1364 and 3598,
  each of the 198 top-level transfers is a HostResult whose error is a
  DynamicConstructor over ResourceError's 12 alternatives, with nested
  IOError and ResourceKind alternatives: 65 to 66 branches per transfer,
  each with its own checked allocation and stores. One transfer costs
  about 1,700 to 2,000 values. Compile-time constructor templates cost
  99 to 145.
- **Two multipliers.**
  - Each lowering of the enclosing bracket match makes 11 top-level
    transfers per origin, one per arm.
  - The bracket body under ComputationalMatch 706 is lowered 18 times
    inside F, once per path through main's enclosing matches. This is
    duplication, not an unrolled recursion, and it is out of scope here
    (sibling `RT-NATIVE-CONTINUATION-DUPLICATION-BUDGET`).
- **Provenance is already exact for sharing.** Each origin has 722
  allocation occurrences and 39 constructors. Every occurrence is emitted
  exactly 18 times, and each constructor has exactly one lane. So code
  shared across the copies needs no re-keying.
- **The seat.** `emit_carrier_transfer` (`lowering/aggregates.rs:1868`) and
  its DynamicConstructor and HostResult arms. Inc2's WIP does not touch
  `aggregates.rs`.
- **The probe.** The patch (`ac0-probe.patch`, with `px11_size.rs`) and
  three run logs are in `/workspaces/ken/tmp/arch-agg-ac0/`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Rulings (Architect `evt_18xe7x3rjaqj5`)

1. **The shape.** One out-of-line transfer helper per static transfer
   template, keyed by (owner, transfer seat origin, template signature). The
   signature is static and planner-issued: the preorder shape, each node's
   constructor identity and allocation occurrence, and each leaf's kind.
   The parameters are the run-time leaves in preorder, plus the allocation
   context the carrier choke already reads, and the helper returns the
   carried word. Its body is the existing emitter, scoped to the helper's
   FuncId. A descriptor-driven generic routine is rejected.
2. **The threshold is structural.** Outline if and only if the template
   contains a run-time selection. Other templates stay inline.
3. **The base is main.** Inc2 rebases onto main after this lands.
4. **Duplication is not in this WP.**

## Scope

The transfer seat in `aggregates.rs`, and where module functions are
declared and defined. Retained as proved: the boundary screening
(`boundary_transfer_admissibility`, `source_aggregate_preflight` and the
slot-store obligation check), the static disposition table and the D2
identity authority, the carried `list_to_bytes` arm, the folded literal
case, and the RT-NATIVE-FNSPLIT activation-identity closure.

## Acceptance

- **AC-0.** Done: `evt_2ry7fef3x4b9r` and `evt_18xe7x3rjaqj5`.
- **AC-1, size.** Measure on a detached merge of the candidate with
  `34cc2134f`, running the exchange once (cap 1800 s, with the probe patch
  applied for measurement only).
  - The build passes ObjectEmission.
  - The largest emitted function is at most 400,000 values.
  - At origins 1364 and 3598, every top-level transfer in the calling
    function is charged at most 100 values.
  - Report the number of helpers and the largest helper's values.
- **AC-2, behavior.** A native root transfers a run-time-selected host
  result through the helper. It runs twice, once with an Ok outcome and
  once with an Err carrying an IOError alternative, and each run is paired
  with the interpreter on the same root. The exit code distinguishes the
  selected alternative, and the two lanes agree. The runtime-leader picks
  the fixture, and AC-3 shows that it reaches a helper.
- **AC-3, mutation.** Force the inline path for selection templates. AC-1's
  per-seat row reddens, at about 1,700 values or more per transfer. Restore
  byte-identically.
- **AC-4.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- **H-dup.** The per-seat bound holds, but ObjectEmission still fails or
  the largest function exceeds 400,000 values. The duplication sibling
  then blocks Inc2.
- **H-ledger.** The D7 relation, the ownership check or the event scoping
  refuses an allocation made in a helper's FuncId for an owner-33
  occurrence.
- **H-context.** The allocation choke needs caller state that cannot be
  passed as a helper parameter.
- The repair needs a change to the boundary screening, the disposition
  table or the identity authority.
- An emitted helper needs a new relocation or object-format class that the
  Architect did not rule.

Checks: 4 fired. I read `emit_carrier_transfer`,
`boundary_transfer_admissibility`, `source_aggregate_preflight` and
`partition_function_units` on main and at `34cc2134f`. 6 fired. AC-0
counted the population at the seam: 198 top-level transfers per origin, 6
template shapes, every one with a selection. 8 fired. A build that leaves
the transfers inline fails AC-1's per-seat row, and AC-3 shows that the
row detects it.
