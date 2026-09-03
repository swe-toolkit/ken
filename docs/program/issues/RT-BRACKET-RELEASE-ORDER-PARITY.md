---
id: RT-BRACKET-RELEASE-ORDER-PARITY
title: "Interp vs native diverge on multi-resource bracket TEARDOWN order: on a two-resource (file + buffer) bracket the interpreter releases resource 1 then 2, native releases 2 then 1 — outcome-independent, in bracket teardown, orthogonal to the composed-return repair (R3). Pre-existing; newly EXPOSED once the five composed-return fixtures run past the base ResourceBodyResult trap. First task: determine the spec-correct release order (which side is the bug), then fix that side."
status: draft
owner: runtime
size: S
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-09-03. Scope-call disposition of the Q2 finding the Architect routed to the Steward (Architect evt_66q0njbd8qjf1, runtime thread thr_13yeftxjnxz2z). While building R3 for RT-COMPOSED-RETURN-FORWARD-RET-EDGE (b2), the runtime-implementer found the parity oracle still fails all five composed-return fixtures on RESOURCE RELEASE ORDER: interp releases resource1 then resource2, native releases 2 then 1. A decisive STRUCTURAL check (runtime-implementer evt_5merj95jgakap; Architect CONCUR evt_66q0njbd8qjf1) EXONERATES R3: the captured-environment carrier is built as worker.captures in POSITION order (emit_checked_ih_captured_environment aggregates.rs:3848-3865, field ordinal N = capture N) and R3 projects emit_carrier_field(carrier, ordinal) at that same ordinal (the landed Direct route's convention, core.rs:7893), so R3 threads the file/buffer handles in PLANNER order and is NOT a capture-ordinal permutation. The divergence is downstream in bracket teardown, outcome-independent, pre-existing (these five fixtures previously base-trapped before reaching teardown, so it was invisible), and orthogonal to the composed-return object. The differential control (same divergence on a two-resource bracket OFF the R3 path) was UNRUNNABLE — the suite has no completing two-resource-bracket fixture off the R3 path; the structural check was the sharper of the two the Architect offered and it decided it. This is a DISTINCT component object from R3 (bracket-teardown release-order parity vs the composed-return outcome repair); per the runtime lane's repeated bundling defect it does NOT belong inside b2. Coordinates re-measure at the build SHA; b2 branch WIP was 430350cff at the finding."
---

> # OPERATIVE (Steward, 2026-09-03) — SCOPE RULING: this is filed as a DISTINCT
> # runtime node, NOT folded into RT-COMPOSED-RETURN-FORWARD-RET-EDGE (b2). b2 lands
> # its composed-return repair on OUTCOME parity (exact InvalidOffset on the
> # fs-read/write-at-offset witnesses); release-order parity is EXCLUDED for those
> # five fixtures and tracked HERE. This is the runtime lane's NEXT deliverable after
> # b2 lands (draft now; the Steward flips it ready and releases on the b2 landing).
> #
> # WHY DISTINCT, NOT INSIDE b2: R3 is exonerated by the structural check (handles
> # threaded in planner order, no permutation — origin above). The release-order
> # divergence is outcome-independent, pre-existing, and downstream in bracket
> # teardown. Bundling an orthogonal component object into b2 is the exact defect the
> # runtime lane hit repeatedly (a downstream property carried into the wrong WP).
> #
> # WHY b2 IS NOT A FALSE GREEN when it lands partial: b2's five fixtures assert the
> # OUTCOME (the composed-return repair the operator funded under dec_7d5aet2hhc1w1),
> # and the release-order property is NOT silently blessed — the fixtures carry an
> # explicit "release-order parity excluded, tracked in RT-BRACKET-RELEASE-ORDER-
> # PARITY" marker (an xfail/disabled release-order assertion with this node cited),
> # NOT an expected-order pinned to whichever side native happens to produce. Landing
> # the correct outcome while explicitly naming and tracking an excluded property is
> # an accepted partial, not a false green.
> #
> # FIRST TASK (D0, before any fix): DETERMINE WHICH SIDE IS CORRECT. Does
> # the spec mandate resource release order for nested / multi-resource
> # brackets (LIFO / reverse-acquisition, or unspecified)? This is a
> # spec-grounded question — pull the Spec enclave / Architect. Only after the
> # correct order is established does "fix the divergence" have a direction: fix
> # the side that violates it. Do NOT assume native is wrong (or that interp is)
> # — the structural check localized the divergence to teardown, it did not
> # adjudicate correctness. If the spec leaves release order unspecified, that
> # is itself a disposition (either pick and enforce one side in both engines,
> # or the parity oracle stops asserting order) — a Steward scope follow-up,
> # surfaced from the D0.
> #
> # SEQUENCING: draft; released as the runtime lane's next deliverable after
> # b2 lands. Architect is the required design reviewer; runtime QA gate;
> # Steward M1-M4 -> lieutenant. Coordinates re-measure at the release SHA.

## Objective

On a two-resource bracket (file handle + buffer), the Ken interpreter and the
native backend tear the resources down in OPPOSITE order (interp: resource 1
then 2; native: 2 then 1). The behavior is outcome-independent (both produce the
correct `InvalidOffset` result on the fs-read/write-at-offset witnesses) and the
divergence is purely in the release/teardown sequence. Bring interp and native
into agreement on the spec-correct order.

## What is established (do not re-derive)

- The divergence is NOT in the composed-return repair (R3). R3 threads the
  file/buffer handles in planner order (capture-ordinal -> frame-slot =
  planner order; no permutation). Structural check decisive; Architect concurs.
- It is pre-existing, exposed only now that the five composed-return fixtures run
  past the base `ResourceBodyResult` trap into teardown.
- The five composed-return fixtures are currently the ONLY two-resource-bracket
  fixtures, and all previously base-trapped — which is why the differential
  (reproduce off the R3 path) is unrunnable today and why this went unseen.

## Deliverables (to be framed on release)

D0. Establish the spec-correct nested/multi-resource-bracket release order
    (Spec enclave / Architect input). Named outcome: the correct order + which
    engine violates it, OR a finding that the spec leaves it unspecified (which
    routes back to the Steward as a scope follow-up).
D1. Fix the violating engine so interp and native agree on the established order.
    Add a completing two-resource-bracket parity fixture OFF the R3 path (the
    missing differential control), so this property is covered independently of
    the composed-return path.
D2. Re-enable release-order parity on the five composed-return fixtures (remove
    the b2 exclusion marker) once agreement holds.

## Acceptance (to be sharpened on release)

- A two-resource-bracket parity fixture that does NOT route through the R3
  collapse closeout asserts identical interp/native release order (the
  differential control that is missing today).
- The five composed-return fixtures assert release-order parity again (D2).
- The fix is grounded in the spec-correct order (D0), not in matching whichever
  engine was easier to change.
