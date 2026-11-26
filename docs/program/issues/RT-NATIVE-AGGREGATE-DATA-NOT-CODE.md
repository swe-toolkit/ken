---
id: RT-NATIVE-AGGREGATE-DATA-NOT-CODE
title: "Native lowering emits a compile-time aggregate as per-node inline code at every boundary transfer: emit_carrier_transfer's Constructor arm allocates and stores each node in place, so the size of a function tracks the size of the data. PX11 Inc2's exchange reaches one function, u2:59, of about 1.49M instructions, of which transfer is 75%, and Cranelift refuses it as CodeTooLarge. Bound the code emitted at a transfer, and at any consumer of a compile-time aggregate, independently of the aggregate's node count, by materializing a large aggregate as data rather than code"
status: ready
owner: runtime
size: L
tier: T1
gate: architect
depends_on: []
blocks: [PX11-NATIVE-STREAM-SOCKET-PARITY]
github: null
origin: "Architect recut evt_1teeny8aaxj7y (PX11 Inc2 §1b entries 13-15: data size is creating code size), after the §1a 3rd stop evt_2fktc4teb9wd0 and the research advisory evt_3c0j7pd627wvn. L1 Linux ABI program, native parity. Steward-filed per COORDINATION section 2."
---

# A compile-time aggregate is data, not code

## Objective

The code that native lowering emits at a boundary transfer, or at any
consumer of a compile-time aggregate, is bounded independently of that
aggregate's node count. An aggregate above a small fixed node count is
materialized as data, not as code. Below that count, inline
materialization stays as it is.

## Settled inputs (Architect `evt_1teeny8aaxj7y`; anchors re-checked at release)

- **The measurement.** Probe `045cad009`, on PX11 Inc2 WIP `34cc2134f`,
  sized every emitted function in the exchange.
  - One function, F = `u2:59`, has 990,992 values, 1,493,949 instructions
    and 503,733 blocks. No other function exceeds 31,052 values.
  - Of F's values, `emit_carrier_transfer` accounts for 0.7485, Construct
    for 0.0026, `bytes_to_list` for 0.0006 and `list_to_bytes` for 0.0009.
  - Transfer origins 1364 and 3598 each make 3366 calls, at about 105
    values per call. One ComputationalMatch origin, 706, makes 18 calls,
    and 3366 is 18 times 187. Every listed origin is owned by
    `PredeclaredFunctionId(33)`.
- **The cap.** `CodegenError::CodeTooLarge` on x86_64 Linux comes only from
  `VRegAllocator::alloc` reaching regalloc2's `VReg::MAX` (2^21 - 1), so it
  is a per-function cap (cranelift-codegen 0.113.1, regalloc2 0.10.2).
- **The mechanism.** `emit_carrier_transfer` (`lowering/aggregates.rs:1868`)
  materializes a `Lowered::Constructor` tree by recursing into each child.
  Each node gets its own aggregate alloc, tag store and field stores,
  emitted inline at the transfer site.
- **The partition does not bound size.** `partition_function_units`
  (`planning/static_transition/semantic_ir.rs:1312` on main) forms units by
  structure, so nothing in it limits a unit's emitted size.
- **The population is unmeasured.** Nothing yet shows which constructors
  origins 1364 and 3598 carry, so the closure covers compile-time aggregates
  in general, not lists.
- **Prior art.** Research `evt_3c0j7pd627wvn`: constant data emitted as code
  is the classic way to hit a per-function cap (javac's oversized static
  initializers), and placing constant data as data is the known way out.
- **The probe patch.** It is at `/tmp/px11-entry15-probe/probe.patch`, and
  `/tmp` is RAM-backed. Copy it to `/workspaces/ken/tmp/` before
  AC-0 uses it.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Replaced: only the per-node inline emission in `emit_carrier_transfer`'s
Constructor arm and its callers, and the compile-time unrolling that feeds
it, as AC-0 attributes them.

Retained as proved:

- the boundary screening before any transfer
  (`boundary_transfer_admissibility`, `source_aggregate_preflight` and the
  slot-store obligation check);
- the static disposition table and the D2 artifact-static identity
  authority;
- the carried `list_to_bytes` arm and its rows at `34cc2134f`;
- the folded literal case;
- the RT-NATIVE-FNSPLIT activation-identity closure.

## Acceptance

- **AC-0 (Architect, before code).** One detached measurement on
  `34cc2134f`, extending probe `045cad009`. At origins 1364, 3598 and 706,
  record the transferred `Lowered` variant and its constructor, its node
  count and depth, and each origin's source location. Also report whether
  the 18 ComputationalMatch calls unroll a source recursion over data fixed
  at compile time. Call research for the specific shapes before ruling. The
  ruling fixes:
  - the threshold;
  - the shape: a static node image emitted once into the object and
    installed by one shared out-of-line helper, or one shared out-of-line
    transfer loop over a data description;
  - the base: main, or a stack under PX11 Inc2's WIP.
- **AC-1.** A size row on the exchange's native build. No emitted function
  exceeds a bound that AC-0 states, and the build passes ObjectEmission. F's
  per-origin emitted values at 1364 and 3598 no longer scale with node
  count.
- **AC-2.** Paired behavior rows, built and run natively and run in the
  interpreter. One aggregate is above the threshold and one is below; both
  give the same observed value in both lanes. The pair differs only in the
  aggregate's node count.
- **AC-3.** One mutation: force the inline path above the threshold. AC-1's
  size row reddens.
- **AC-4.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- AC-0 finds that the hot aggregates are not fixed at compile time.
- The repair needs a change to the boundary screening, the disposition
  table or the identity authority.
- An emitted static image needs a new relocation or object-format class
  that AC-0 did not rule.

Checks: 4 fired. I read `emit_carrier_transfer`,
`boundary_transfer_admissibility`, `source_aggregate_preflight` and
`partition_function_units` on main `200c0124c` and at `34cc2134f`. 6 fired.
The probe sized every emitted function, and only F is large. The aggregate
population behind it is left to AC-0. 8 fired. AC-2's pair differs only in
node count, so a fix that changes behavior above the threshold fails it.
AC-3 shows that AC-1 detects the inline path.
