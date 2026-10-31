---
id: PX10-NATIVE-SITE-IDENTITY
title: "The native planner reads a site's identity (bracket settlement owner, response leaf, transport stage class, region-exit owner, call target) from the unit that emits the site's code, not from the site's static owner. The two diverge wherever a predeclared higher-order prelude function such as withChild has its callbacks emitted inside another unit, so PX10's three fixtures stop at four walls. Key every plane on the static owner, projecting to the emitting unit through one site-identity map"
status: ready
owner: runtime
size: L
tier: T1
gate: architect
depends_on: [PX10-SPAWN-PLAN-WAIT]
blocks: [PX10-NATIVE-SPAWN-PARITY]
github: null
origin: "Architect AC-0 round 2 on PX10-NATIVE-SPAWN-PARITY Increment B (evt_1jd8yzwewmtf6): five walls, four sharing one §1b predicate. Steward recut evt_6kmhp6k18ypq5 under the operator's 2026-09-21 restructuring direction. L1 objective: the Linux ABI program. Steward-filed per COORDINATION section 2."
---

# Native sites keep their static identity

## Objective

Every native planning plane keys ownership and selection on a site's
static owner. A site emitted in another unit (a callback body, a
finalizer closure, a curried push chain inside a specialization or
inlined into its caller) projects to that unit through one map, so each
static site has one identity wherever its code is emitted.

## Settled inputs (Architect probes on `6869c29bc`, evt_1jd8yzwewmtf6)

- **The predicate.** A site's identity is read from the unit that emits
  its code rather than from its static owner. The two coincide wherever
  native already works.
- **The walls**, each keyed on emission:
  - W1, the returned-Vis settlement is checked against the protocol's
    root owner, not the Vis that returned it. R3 closes it.
  - W2, `responses.rs:2030`: a `CheckedBracketFinalizer` Vis emitted
    inside a closure cannot reach its selecting match by an up-walk.
  - W3, `responses.rs:3825`: the transport stage of producer call 158
    is classed "mixed" from its emitting workers. Re-keying that one
    plane makes `brackets.rs:165` mint two owners for one static exit,
    in all three fixtures.
  - W4, `lowering/calls.rs:1861`: a curried push chain's innermost body
    is called from unit 20 while its static caller is unit 18.
- **The one existing projection.** `emittable_call_edges` already
  duplicates source calls under a retained worker's function id. The
  map generalizes that rule; it is not a fifth special case.
- **Ruled already, carried by whichever WP lands first:** R2 (return-fold
  up-walk), R3 (settlement keyed on the returning Vis), and the
  `host_effect_recipe_tree` rows for the seven PX10 operations
  (evt_1jd9djf6thfea, evt_1jd8yzwewmtf6).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

1. **AC-0 (Architect, D0 on the cut base).** A scratch prototype of the
   site-identity map that brings the three slice-1 fixtures to a native
   build, then a verbatim ruling of the map and of each plane's key.
2. **One runtime branch** carrying the ruled map and the plane re-keys.
   The process-op promotion, the availability-gate deletion and native
   parity stay in PX10-NATIVE-SPAWN-PARITY Increment B.

## Acceptance

- **AC-1.** Smoke, argv_env_pipe and live_child build natively, with the
  Process operations admitted in the test configuration.
- **AC-2 (one owner).** Each static region exit has exactly one owner in
  every plane. A pin on argv_env_pipe and live_child names the exit and
  its owner.
- **AC-3 (no regression).** The return-fold equivalence holds on every
  existing region build. `rt_native_bracket_body_once`,
  `px7f_resource_native`, `rt_escape_second_resource_native` and
  `rt_bracket_control_region_d0` stay green.
- **AC-4 (negative, QA).** A settlement or response whose static owner
  lies outside the region still refuses. A mutant that keys one plane on
  the emitting unit reddens a named row.

## Stop conditions

- A kernel or trust change, or a post-fork callback.
- A third advancing stop on the native-lowering chain (§1a count 2).

Checks: 6 fired: the wall population is the Architect's probe
enumeration across all three fixtures, bypassing each wall to reach the
next. 7 fired: the naive W3 re-key was re-run on all three fixtures and
fails at `brackets.rs:165`, which is why the scope is the map rather
than per-wall fixes. 8 fired: AC-2 counts owners per static exit, which
the probe-7 failure (two owners) does not satisfy.
