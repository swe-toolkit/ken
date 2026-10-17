---
id: RT-NESTED-RELAY-PERFORMING-ROUTE
title: "After RT-NESTED-RELAY-VIS-NATIVE-SETTLEMENT a forwarded host op with no performing route stops native at a planned relay trap where the interpreter performs it (FsReadAt), so native refuses a valid program the interpreter runs. Emit an outer member that performs the forwarded op and retire the traps (route C)"
status: merged
owner: runtime
size: L
tier: T1
gate: architect
depends_on: [RT-NESTED-RELAY-VIS-NATIVE-SETTLEMENT]
blocks: []
github: null
origin: "Increment (ii), route (C), of RT-NESTED-RELAY-VIS-NATIVE-SETTLEMENT, which names it a successor node framed from the D0 table (evt_3y63h26apw108, evt_6jxckz0adhrbm). Architect approval of the increment (i) respin evt_7vdy3sx322062 records its coverage obligation here. Steward-filed per COORDINATION section 2."
---

# Forwarded host ops are performed natively

## Objective

A relay pair whose forwarded host op has no route today performs that op
natively, in the interpreter's order, instead of stopping at the planned
relay trap.

## Settled inputs (increment (i), exact `22d5f0956`)

- **The traps.** Increment (i) plans a typed relay trap for each route-none
  (pair, identity) (`MissingRuntimeMetadata`, "relay: forwarded host op
  <op> from source <origin> has no performing route"). On `5c0afeffa` five
  callsites consume it (terminal transport, active transport, claim,
  fusion and detached result), plus the ownerless early guard. The runtime
  ring's AC-0 measured source 768 / Spec(2) / FsReadAt reaching the claim
  consumer (`lowering/core.rs:7737`), so the earlier three-seat list was
  not exhaustive (`evt_77jsfk2hdrn9g`). R4 makes an admitted pair with no
  planned call edge a planner refusal.
- **AC-MAIN is a transition sentinel.**
  `branched_scrutinee_forwarded_read_is_performed_or_refused`
  (`crates/ken-cli/tests/rt_branched_scrutinee_unit_body_port.rs`) asserts
  that native stops at exactly the planned trap with no `FsReadAt`. This
  WP turns it into parity, with `FsReadAt` performed.
- **The D0 table** (SHA-256 `c2b23461`) is the population. Row 36's 11
  pair-builds already leave by `deferred_handler_owned`, a performing
  route.
- **Coverage obligation (Architect `evt_7vdy3sx322062`).** QA found no
  discriminating witness for the three seat guards, nor any
  `Excluded{Underived}` owner, in the targeted rows (`evt_3a4mkedqb5v3c`).
  This WP supplies one witness per seat and one Underived-owner fixture.
  The Architect restated it on the respin `b2590d9b8`
  (`evt_22vjqdcd1btgk`): the R1, R2 and R3 seat mutants stayed green
  because no selected row reaches those seats. It adds one more: the
  px8ta relay owner0/Vis676/relay370 sits inside a Closure-refused
  artifact, so it is unexercised, not cleared.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

Route (C) as ruled at D0 (`evt_5pktec6dv6wxx`, base `5c0afeffa`): the
emission point is the source's own execute-then-resume response owner,
reached by promoting its P2 demand in `static_response_phase_b_split_over`.
- (a) Promote a demand iff it is a transport source that would otherwise
  go P2 `UnconsumedTransportCaller`, its `vis_origin` is the source of an
  admitted relay pair, no Deferred handler owner exists for it, and its
  stage is exclusively predeclared. Key on the pair and edge identity,
  never an origin number.
- (b) A relay-promoted owner passes `validate_response_owner_call_coverage`
  iff its selected caller is verified, or a lowering ledger over every
  construct-lowering path records its source construct as never lowered.
  `disposition=None` is not that ledger.
- (c) A relay source in a mixed-owner stage is a typed planner refusal
  naming the source.
- (d) A route-none at `build_relay_routes` after promotion is a typed
  planner refusal. Delete `branch_unrouted_relay_at_consumer`, its call
  sites, and the trap interning at construction.rs:1556-1562.
- (e) Relay promotion is separate from execute-then-resume suppression.
  px8f's suppressed expectation becomes the measured
  `[BufferAllocate, FsOpen]`.

## Acceptance

- **AC-0 (measure).** On the landed base, a complete census of the
  route-none (pair, identity) population in the D0 table and the consumer
  each reaches, over all five callsites and the ownerless guard. It ends
  as a reached/not-reached table per consumer, with the record counts and
  the units that produced them; for px8ta, the Closure refusal point and
  its record count. Do not build fixtures for not-reached consumers: the
  table is the D0 input, and the Architect rules per consumer whether a
  fixture is constructible or the consumer is unreachable by a cited
  guard (`evt_234a783np26tf`). Instrumentation is disposable and not
  committed. Done: `evt_dm6hsc4mm1ng`.
- **AC-0' (measure, before any build).** The relay-source set for (a) is
  derived at construction.rs:1447 from inputs final there, as a least
  fixpoint over the would-be owner population. After pairs exist, the
  plan checks the pre-schema promoted set equals the pair-derived set; a
  mismatch is a planner error. One reorder is authorized (operator
  2026-10-16, "a, with the Architect's review first"): moving
  `build_host_effect_seat_plan` and `validate_host_effect_seat_plan` from
  :1459-1460 to just after `join_results`, once the Architect has reviewed
  it. Any other reorder is a stop with the exact diff.
- **AC-1.** AC-MAIN asserts parity. Native performs `FsReadAt`, and its
  non-release events equal the interpreter's in order, with releases
  compared as a set through the shared helpers.
- **AC-2 (witnesses).** The traps are gone, so the per-seat rows become:
  - planner refusal: the relay-promotion suppression mutant turns AC-MAIN
    back red, and a mixed-owner relay source compiles to the typed
    refusal;
  - coverage discharge: cap41_endpoint, cap41_offset_endpoint and
    read_window compile and keep their terminals, and a mutant that
    lowers the source without entering its owner trips the gate;
  - an Underived-owner witness by classification forced at the protocol
    producer (`returned_vis_protocol`) for each installed AC-MAIN owner,
    native against the interpreter (Architect `evt_5fevwxrfy8knk`). A
    natural user-source Underived owner was not constructed: the
    measured installed owners' K worker bodies are prelude wrappers
    (`evt_3e2bxfp79n9yc`). If native drops an op the interpreter
    performs, stop to the Architect;
  - the ignored px8ta real-same-depth row, run once under route (C): a
    witness if it reaches route assembly, otherwise named unreached with
    that measurement.
- **AC-3 (control).** Rows 01-36 and the four AC-MAIN controls keep
  parity under the fleet release rule.

## Hard-stop inventory (§1b)

§1a: the Underived-owner settlement chain is at 1 (Architect
`evt_27ernsj0ttshz`); the route chain (emission point, promotion
authority) is at 1 (Architect `evt_7ve4vnpycf74t`).

1. An `Excluded{Underived}` owner is emitted on the Ret-only arm although
   its K may return a Vis: the arm is keyed on "derivation failed", which
   the emitter treats as "no returned Vis" (units.rs:4663). Forced owner
   890 on AC-MAIN drops `FsReadAt` silently, with promotion suppressed as
   well, so main has the defect. Ruled: refuse the plane at plan
   construction, after `build_pending_vis_settlements` (construction.rs
   :1530), with `planner_capacity_error` "underivable returned-Vis
   protocol"; forced 890 and forced 768 both refuse; deleting the refusal
   is QA's AC-3 mutation for this guard. A native row green on main that
   newly refuses with that text is a stop to the Architect.
2. A single-authority suppression control kept a retired runtime-trap
   oracle after a second authority, relay promotion, came to overlap it.
   The overlap is keyed on test-seam authority. On `8cdada507`, CI run
   `39002598310` reddened two native rows at `differential(
   "fs-write-at-offset-single", ...)`: the ETR-only artifact returns the
   untyped `-1` of the pending-Vis unknown-member exit (units.rs:3377).
   Ruled: no production guard. Test-only changes in `rt_parity_native.rs`:
   - `suppressing_both_materialization_authorities_refuses_at_the_relay_edge`
     builds with both suppressions and expects the Packaging refusal
     carrying `EXECUTE_THEN_RESUME_SUPPRESSED_RELAY_TRAP`, with relay
     applications = 1;
   - an ETR-only sibling row builds Ok, and a relay-only sibling row builds
     Ok with 0 applications;
   - the outer-carried control keeps only its residual sentinel,
     `site_a_none_arrivals == 3`, on an ETR-only build.
   A relay-only refusal or nonzero applications, or a different sentinel
   count, is a stop to the Architect. The untyped `-1` is carried, out of
   scope.

## Stop conditions

- The repair restructures the planner's build order: return to the
  operator to size (operator 2026-09-21).
- Any kernel, trust or spec change.
- A red native CI row on increment (i)'s SHA is a stop to the Architect
  before this WP starts.
