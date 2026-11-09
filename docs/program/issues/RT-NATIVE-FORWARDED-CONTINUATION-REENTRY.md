---
id: RT-NATIVE-FORWARDED-CONTINUATION-REENTRY
title: "A pure-forward carried clause at a function's outermost carried position jumps its own header with k in place, so a body-tail request forwarded out of a recursive worker never reaches the enclosing genuine handler, and RNSPK traps that edge fail-closed. Lower the boundary arm as a return of Vis(op, K2') through the function's normal-return path, with K2' one generated continuation specialization per forwarding chain that re-runs the request's K body and the chain's clauses, so PX10 B1a's F rows reach native parity"
status: ready
owner: runtime
size: L
tier: T1
gate: architect
depends_on: [RT-NATIVE-SETTLEMENT-PLACEMENT-KEY]
blocks: [PX10-NATIVE-SPAWN-PARITY]
github: null
origin: "Architect S0b ruling (RE-ENTRY-ABSENT, one composed re-entry per chain) on implementer evt_6cww96v9b0vf2, RNSPK thread; Steward recut 14:50Z 2026-11-09 moved F native parity out of RT-NATIVE-SETTLEMENT-PLACEMENT-KEY into this node. L1 objective: the Linux ABI program. Steward-filed per COORDINATION section 2."
---

# Forward a request out of its function, then re-enter

## Objective

F's body-tail request reaches the enclosing genuine handler and resumes
through the forwarding chain, so the three F rows match the interpreter.
This replaces the named trap that RNSPK places at the boundary arm.

## Settled inputs (S0b, on F base `0d4199e26` plus the C-28 patch)

- **The consumer is present (M1).** fn72's single argv caller,
  fn48/Predeclared18 at 8 cloned sites, feeds every normal return into
  C571's checked Ret/Vis header. A returned Vis reaches S571's genuine
  Vis arm S957.
- **No re-entry unit exists (M3).** Context5 is (Spec12, S1771). The only
  S1665 worker is Spec1, consuming S1165. `intern_generated_contexts`
  (`continuations.rs` ~7100) refuses S1665 under Spec12, because Spec12's
  selected worker body is S1771.
- **The chain has two clauses (M4).** In argv, all in Spec12/fn72:
  request S1687's word `0x1609` enters S1165 and takes pure-forward Vis
  case S1172; the rebuilt Vis `0x1809` enters C986 and takes Vis case
  S993; field1 `0x1509` re-enters C986, and the fallback fires. Live has
  the same shape: S606, then C427.
  - Forwarding between clauses inside one function already works by
    value. Only the outermost clause's in-place IH edge is wrong.
  - `0x1509` is the inner clause's IH value, not the request's K. C-26
    left it unclassified, so nothing may call it as a K.
- **Cross-function body-tail consumption already works** in committed
  rows: two-print (`rt_selected_pending_call_admission.rs:303`) and span
  freeze (`rt_span_prov_native.rs:358`).

Treat anchors as perishable. If a settled input is false on the landed
RNSPK base, stop and report the mismatch.

## Deliverable

One candidate on the landed RNSPK. It is the Architect's planner
obligation, one composed re-entry per chain:

1. **Where.** A pure-forward arm (`brackets.rs:429`
   `forwarding_vis_arm`) whose clause is the outermost carried
   ComputationalMatch of its emitting function. Inner pure-forward
   clauses keep their by-value forward.
2. **Lowering.** The boundary arm returns Vis(op, K2') through the
   function's normal-return path (the Result slot load, then the caller's
   carried header), never an in-place jump. It replaces RNSPK's trap.
3. **K2'** is a new generated continuation specialization, keyed in
   `ContinuationSpecializationKey` vocabulary (`continuations.rs` ~6438):
   - worker body: the request K's body (S1665, from LexicalClosure
     S1677 of request S1687);
   - consumer: the innermost forwarding clause entered (S1165);
   - a new component: the ordered clause stack from that consumer to the
     boundary ([S1165, S986]);
   - emission owner: the boundary specialization (Spec12), with
     producer owner, result and construct from the request, and recursive
     position as today;
   - a new (enclosing_specialization, worker_body_origin) context that
     `intern_generated_contexts` admits as distinct.
   K2' x runs S1665 on x with k's captures, feeds the result to S1165's
   header and then S986's, and returns whatever reaches the boundary to
   its own caller. A further Vis takes the same return route.
4. **Representation.** K2' is callable through the lowering S571's
   genuine arm already uses to call its k (IH body `Call Var(0) [r]`,
   block2394 onward).
5. **Captures.** k's captures plus what the clause stack needs to resume,
   including region R0's settlement state at C986 and C1165's bind
   continuation. Measure them; do not assume them.
6. **Admission.** Only the exact pure-forward shape, at a function
   boundary, with a single static clause stack. Anything else is a named
   refusal and keeps the trap.

## Acceptance

- **Step 0 (before product code).** Name the S571 IH call site
  (file:line) and show it can call a generated specialization. If it
  cannot, stop back to the Architect.
- **Posture.** Until B2 opens the Process gate, A1, A2 and A4 are
  measured under the uncommitted C-28 diagnostic overlay, as in RNSPK's
  AC-1, and their committed rows are promoted in B2. A3 and A6 are
  committed and production-gated.
- **A1, parity.** argv exits 0 with 19 events, v-null exits 73 with 14,
  and live exits 0 with 17. Each equals the interpreter, and event 6 is
  the field2 ProcessPlanPush (Effect S724 in argv, S165 in live).
- **A2, route,** in order: the request enters S1165; C986 takes the Vis
  case; fn72 returns to fn48; C571's Vis case S957 executes on the
  forwarded op; after its response, K2' runs S1665, then S1165, then
  S986. The fallback count and the trap count are 0. Live has the same
  order with S606 and C427.
- **A3, controls unchanged:** smoke 9/9, G x2, px8ta 3/3, checked_ih 9,
  the two-print parity row and span freeze 2/2.
- **A4, mutations,** each reddening A1 or A2:
  - (i) restore the in-place boundary edge;
  - (ii) K2' re-enters only S986 and skips S1165;
  - (iii) K2' omits region R0's settlement state.

  A mutation that does not redden is reported as non-discriminating, not
  replaced.
- **A5, one-clause row.** If a fixture can put a single pure-forward
  clause between a body-tail request and a genuine handler across a
  function boundary, commit it. If not, record the gap.
- **A6.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- Step 0 fails.
- A kernel or trust change, or a change to the Process gate,
  `HostOpV1::availability` or AC-2g (B2's).
- PX10 §1a reaching 21 (it is 18).

Checks: 4 fired: the obligation, key components and call path are the
Architect's, written in the planner's vocabulary at the cited sites, and
step 0 measures the one unmeasured enabler. 5 fired: `0x1509` is not
treated as a callable K. 6 fired: the population is the three F rows plus
the A5 gap; no claim is made beyond them.
