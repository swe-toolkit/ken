---
id: RT-IH-MARKER-PRODUCER-COMPLETE
title: "Resolve the OrientedSubcontinuationPlanV1 'computational IH invocation marker does not wrap a complete application' refusal on the checked-program family (both programs, the terminal of the four cap41_* pins, on RT-FSREADAT's AC-4/AC-5 critical path) by CORRECTING THE PRODUCER, never the validator: diagnose which producer step made the plan and the marked expression disagree about completeness, then resolve to ONE of the two lawful non-interchangeable representations -- (i) it IS complete => fix the producer so plan and marker agree on a full Call of the checked arity, or (ii) it is genuinely partial => the distinct closure/PAP form with its own later apply. NEVER pad the call, infer missing arguments at emission, or reinterpret a full-call node as partial. The completeness checker stays byte-untouched (GHC join-point / Lean IR-checker / typed-CPS precedent, all fail-closed). Carries a MANDATORY post-fix re-enumeration gate: this family is depth-1 and cannot self-bound, so after the fix lands, re-run the existing checked-family enumeration end-to-end and read what surfaces."
status: active
owner: language
size: M
gate: none
depends_on: [RT-COLD-LOWERING-CHECKED-FAMILY-ENUMERATION]
blocks: []
github: null
origin: "Bounded successor cut by the Steward from the two enumeration reports (RT-COLD-LOWERING-PATH-ENUMERATION report 1 evt_1m6eg23vnbj4n found the IH-marker has ZERO entries in rt_parity; RT-COLD-LOWERING-CHECKED-FAMILY-ENUMERATION report 2 evt_7rg8mye0bbfse confirmed it is the checked-program family's single terminal over both programs). Architect ruling point 3 + IH-marker addendum evt_1a8tf8776fd6m (producer-fix, validator untouched) and the report-2 review evt_4jcnbhx8nqwdy (the depth behind the marker is UNKNOWN, possibly zero; the cheap sound instrument is a post-fix re-enumeration, not speculative fixtures). Steward-filed per COORDINATION section 2."
---

# WHAT THIS NODE IS

> OUTCOME 2026-08-22 (producer fix DELIVERED + effective; deeper layer ruled).
> Language's producer fix landed on the co-land branch at `64019430c` and WORKED:
> both checked-family programs cleared the marker seam (supplied 0 == arity 0,
> method_binder_ordinal 0 == binder_index 0). This node's producer-side
> deliverable is CLOSED (Architect confirmed effective, evt_3amhmvyd0sr9t). The
> mandatory AC-REENUM then STOPPED at a deeper refusal ONE LAYER BELOW the
> marker seam -- the static-worker ABI supply layer (template arity 0 vs
> `worker.declared_arity` 1) -- a genuine NEW mechanism (HS=4, Steward-confirmed),
> NOT a further checked-family layer, so the DEFERRED DECISION POINT below did NOT
> fire. The Architect ruled the closure fork CLOSURE A (evt_3tspjkw7dhh6x): the
> marker is a saturated nullary closure-style invocation, template arity 0 is
> CORRECT, and the recursive value is an environment operand the ported route must
> SEAT -- owner Runtime. That work is the successor
> [[RT-CHECKED-IH-RECURSIVE-OPERAND-SEAT]], which now carries the terminal
> AC-REENUM gate. This node co-lands (§8) as a closed producer deliverable with
> that successor + the join half (`6a45ae1a7`); it does not land alone.
>
> UPDATE 2026-08-22: Closure A was subsequently WITHDRAWN (Architect
> evt_hftfnn4mh8jk) after the operand-seat mechanism was refuted at the type
> level -- the recursive value at the firing seam is a live nested-recursor
> StaticWorker,
> not a placeable operand (HS=5). The successor is HELD, re-framed pending a
> reframed section-1a ruling (leading hypothesis: a nullary FORCE of a lazy IH
> thunk, fixed upstream of the arity gate). This node's producer fix still STANDS
> as a closed deliverable; the terminal AC-REENUM gate moves with the reframed
> successor. Nothing lands until the reframed ruling and AC-REENUM green.

> UPDATE 2026-08-22 (RESOLVED -- ESCAPING; co-land re-scoped). The reframed
> ruling landed (Architect evt_2f4bbmt7qfde1): the marker is a NULLARY_FORCE
> (realize the specialized IH function VALUE unapplied), and the fork is an
> ESCAPE discriminator.
> The decisive measurement came back ESCAPING (runtime-implementer
> evt_79jd1nxamqd95): the realized IH value is stored straight into a Construct
> on both checked-family programs -- a CAPABILITY GAP, not a seam fix. Steward
> scoping
> ruling (evt_5pmk273zg5paa): the co-land set lands WITHOUT the functional-IH
> piece; the two checked-family programs are re-pointed to documented ADVANCING
> REFUSALS
> (they refuse at the correct deeper point now). This node's producer fix
> (`64019430c`) still STANDS as a closed deliverable. The terminal AC-REENUM gate
> and the "green both checked-family programs" goal move to the new deliverable
> [[RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION]]; the operand-seat successor
> RT-CHECKED-IH-RECURSIVE-OPERAND-SEAT is CLOSED (superseded). HS=5 (ESCAPING is a
> scoping decision, not an increment). The co-land can now land on its own greenness
> with the refusal encoded.

> RE-SCOPED + RE-OWNED 2026-08-22 (Architect shape ruling evt_7rsy01s7k1d7x;
> Steward ownership finalization). The shape is PRODUCER-SIDE, case (i) genuinely
> complete but mis-spelled: at arity 0 the producer emits bare
> `RuntimeExpr::Var(0)` (the un-applied IH), where the marker's contract is an
> application node and a complete arity-0 invocation is `Call{ func, args: [] }`
> -- which the consumer `enter_checked_computational_ih_invocation` ALREADY
> accepts (0 == 0). The fix is in `ken-elaborator/src/erasure.rs` (sites 2327 /
> 2863 / 3302) -- Language's crate -- so OWNER is now `language`. The id keeps
> its RT- prefix only to preserve the [[RT-FSREADAT-REPLY-BUFFER-GATE-REMOVAL]] /
> [[RT-MATERIALIZED-DEAD-JOIN-RECONCILE]]
> / enumeration links; the body is authoritative on ownership. Do NOT relax the
> consumer (the Steward's arity-0-admitted counter was attributed to the wrong
> mechanism -- that admission is BoundaryCarrier's carried-word at core.rs:3758,
> a different subsystem, not the checked-IH marker; relaxing the
> unconditional-Call demand would silently admit a bare-Var mis-emission that the
> demand refuses fail-closed, stepping back toward the pre-D5a "wraps anything"
> hole). The RUNTIME side narrows to VERIFICATION only: confirm the consumer
> stays byte-untouched and accepts the emitted `Call{args:[]}`, the terminal pins
> advance/green, then run AC-REENUM. Producer fix authored by the Language seat
> on the co-land branch `wp/RT-FSREADAT-REPLY-BUFFER-GATE-REMOVAL`; Runtime
> authors no ken-elaborator edit. Architect required reviewer on both halves;
> Adversary hunts. HS stays 3.

Resolve the `OrientedSubcontinuationPlanV1` "computational IH invocation marker
does not wrap a complete application" refusal -- the checked-program family's
single terminal, tripped by BOTH programs in the family (the source three of the
four `cap41_*` pins share, and the distinct `rt_branched_scrutinee_unit_body_port`
source), on RT-FSREADAT's own AC-4/AC-5 critical path. The fix is on the
PRODUCER; the completeness checker stays byte-untouched.

# THE MEASURED FACT (both enumeration reports; Architect evt_4jcnbhx8nqwdy)

- Report 1: the IH-marker has ZERO entries in `RT_PARITY_SOURCE` -- unreachable
  from that population.
- Report 2: it is the checked-program family's ONE mechanism, tripped by both
  programs (measured by content hash, not filed name -- the fourth pin's source
  shares the entry NAME `rt_branched_stage` with a distinct body).
- Structural finding (Architect confirms binding): this family is DEPTH-1 and
  cannot self-bound. Each program is a population of one; a single compile
  returns a single refusal, so whatever sits behind the IH-marker is INVISIBLE
  until the marker refusal is cleared. The depth behind it is UNKNOWN, possibly
  zero.

# MECHANISM (Architect ruling point 3 + addendum evt_1a8tf8776fd6m; producer-fix, checker untouched)

The refusal is the STANDARD compiler-IR well-formedness boundary, and prior art
is unanimous it must NOT be relaxed (GHC join points: every occurrence a
saturated same-arity tail call or invalid Core; Lean IR checker: a full
application supplies exactly the declaration arity, recursor content gets no
exception; typed-CPS: continuation application is a dedicated saturated form).
Ken already matches this precedent (`CheckedComputationalIHCallTemplateV1` is an
exact complete application; entry validation refuses a body that is not one
`Call` of the checked arity before emitting). So the refusal is CORRECT and
aligned -- it is evidence the producer-side PLAN and the marked EXPRESSION
disagree about completeness, never evidence to accept an incomplete application.

Diagnose which producer step created the disagreement, then resolve to ONE of the
two lawful, non-interchangeable representations:

- (i) it IS complete => fix the producer so plan and marker agree on a full
  `Call` of the checked arity;
- (ii) it is GENUINELY partial => represent it as the distinct closure/PAP form
  with its own later apply operation.

NEVER pad the call, let emission infer missing arguments, or reinterpret a
full-call node as partial -- each weakens the IR contract. Keep the checker;
correct the producer. The CPS/subcontinuation-planning prior art (research
addenda evt_3p0rwsjw51mjq) is a fix-time input for the builder.

# ACCEPTANCE

- **AC-1 (both programs clear the marker).** After the fix, both checked-family
  programs no longer stop at the `OrientedSubcontinuationPlanV1` completeness
  check. Report, per program, which producer step disagreed and which lawful
  representation resolved it.
- **AC-2 (producer-corrected, one of the two lawful shapes).** The resolution is
  a full `Call` of the checked arity OR the distinct closure/PAP form -- never a
  pad, an inferred argument, or a full-call node reinterpreted as partial.
- **AC-SOUNDNESS (checker byte-untouched).** The `OrientedSubcontinuationPlanV1`
  completeness check is unchanged -- it stays the fail-closed boundary. A marked
  expression that is not a complete application of the checked arity must STILL
  trip it. Ship one durable test in that shape.
- **AC-REENUM (MANDATORY post-fix re-enumeration gate).** Immediately after the
  fix lands, RE-RUN the existing checked-family enumeration end-to-end (the
  runner and red-until-green coverage test from
  [[RT-COLD-LOWERING-CHECKED-FAMILY-ENUMERATION]] already exist -- this is a
  RERUN, not authoring). Read what surfaces: if BOTH programs go green, the
  checked family is bounded and the co-land can green. If a FURTHER
  checked-family refusal surfaces, STOP and report it -- do NOT silently proceed;
  that is the trigger for the deferred decision point below. This is the cheap,
  sound instrument for an unknown-depth space (Architect review evt_4jcnbhx8nqwdy):
  clear one refusal, re-run, read -- one bounded step at a time, at near-zero
  cost, so the family cannot resume serial discovery silently.
- **AC-NO-REGRESSION.** No lowering on `main` regresses; whole-suite green in CI
  (`COORDINATION §12`). Local targeted `-p` only, never `--workspace`; runtime
  respin gate `-p ken-runtime` all-binaries + `-p ken-cli` + `-p ken-verify`.
- **Required reviewer.** Architect (producer-direction soundness + checker
  untouched, AND the post-fix re-enumeration result). The Adversary hunts the
  three over-accept shapes: a padded call, an emission-inferred argument, and a
  full-call node reinterpreted as partial.

# DEFERRED DECISION POINT (contingency, NOT now)

Fixture provisioning -- giving these sources a substitution site and additional
admissible entries so the checked-program space becomes enumerable -- is NEW
fixture work, not measurement, and is HELD as a contingency. Invest in it ONLY
IF the AC-REENUM re-enumeration surfaces a further checked-family layer, i.e.
the family demonstrates genuine serial depth rather than depth-1. Do NOT pay this
expense speculatively against a depth we cannot yet show is nonempty (Architect
review evt_4jcnbhx8nqwdy: the refinement that governs the investment). If it
fires, it comes back to the Steward as a fresh cut.

# CO-LANDING

Part of the RT-FSREADAT co-landing set (§8) on
`wp/RT-FSREADAT-REPLY-BUFFER-GATE-REMOVAL` (thread `thr_3j5ew8rhy35nh`). This
node's fix + its AC-REENUM green (both checked programs) are what let RT-FSREADAT's
AC-4/AC-5 and the NHC `D-final` close. The whole set lands as ONE green candidate
once the `cap41_*`/terminal rows go all-green.

# NOT IN SCOPE

- Relaxing or bypassing the `OrientedSubcontinuationPlanV1` completeness check.
- The materialized-dead join reconciliation
  ([[RT-MATERIALIZED-DEAD-JOIN-RECONCILE]]), the BoundaryCarrier layer-1 witness
  (folded into [[RT-FSREADAT-REPLY-BUFFER-GATE-REMOVAL]]), and the elaboration
  TypeMismatch (separate disposition) -- each its own ledger line.
- Fixture provisioning now -- held as the contingency above.
- Any kernel / TCB edit. `ken-runtime` cranelift lowering + planning
  (`crates/ken-runtime/src/cranelift_backend/planning/`,
  `oriented_subcontinuation_plan.rs`); no operator authorization is in play.

# SEQUENCING / CONTENTION / CAPABILITY TIER

`depends_on: [RT-COLD-LOWERING-CHECKED-FAMILY-ENUMERATION]` -- bounded by its
report 2. Same branch, ring, and thread as the co-landing set. Proceeds
regardless of the report-1 successors -- it is a known refusal on RT-FSREADAT's
critical path and must be fixed either way.

Tier T1: the producer-side diagnosis (which step disagrees, and which of two
non-interchangeable representations is correct) is the load-bearing judgment, and
the depth-1 re-enumeration gate makes this the node that governs whether the
checked family is bounded. Size M.
