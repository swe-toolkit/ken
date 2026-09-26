---
name: a-validator-whose-expected-value-is-its-own-builder-re-run
description: >-
  A gate that computes its expected value by re-running the builder that
  produced the value it checks cannot see a defect in that derivation — it is
  self-oracled, and the word "independently" in the comment beside it is the
  tell. Its severity is decided elsewhere: it is a defect only if nothing
  else (construction upstream, or an independent guard on this same path)
  upholds the invariant; otherwise it is a quality finding.
metadata:
  type: feedback
---

# A validator whose expected value is its own builder re-run

Three planner gates landed in the shape

```rust
plan.case_emissions = build_case_emission_plan(&plan)?;
validate_case_emission_plan(&plan, &plan.case_emissions)?;   // checks
                                    // records != build_case_emission_plan(plan)
```

so the comparison is `build_X(plan)` against `build_X(plan)`. The builder is
the answer key for its own output.

**Establish it by closure, not by reading one site.** Two facts make it a
tautology and both are greppable: the population has **exactly one writer**
(so the stored value is always the builder's), and the builder **reads only
plan state that is final before the assignment** (so a later re-run cannot
differ). Check the second one specifically against whatever runs *between* the
two call sites — here an ABI installer ran in between, and its only look at
`plan.abi` turned out to be inside a `#[cfg(test)]` block. Without that check
the second call site could have been genuine and the finding overstated.

**Measure it with a defect in the derivation, not by deleting the gate.**
Removing a correct gate leaves a green suite, so it measures nothing. Inject a
compile-valid defect into the builder instead and watch two numbers: 114 of 611
tests reddened, and the gate's own message appeared **zero** times. That
separates "the defect is caught" from "this gate catches it" — the coverage was
entirely downstream behavioural tests.

**The tell is a comment claiming independence.** *"The two populations are
independently re-derived and checked"* is what makes it costly: a reader
budgeting trust sees two authorities agreeing. Re-derived, yes; independent,
no. Report the word, not just the code — and price the repeat cost, since each
builder now runs three times per plan.

**Be exact about the blast radius.** In the same commit a sibling gate joined
two *separately derived* populations by origin — genuinely load-bearing. Naming
what is unaffected is what keeps the finding from being discounted whole.

Fourth instance of this class, after a join validator that derived `required`
from the very sets it then validated. When you find one, grep the file for
every other `validate_*` and ask what supplies its expectation.
Related: [[a-ruling-that-widens-a-shared-map-names-only-the-consumer-it-was-about]],
CHECKS.md check 1,
the `mutation-prove-a-pin` skill (mutate the natural producer),
[[differential-oracle-is-blind-to-a-shared-premise]],
[[audit-a-detector-against-the-one-case-whose-answer-you-already-know]].

## Severity: a self-oracled check is a bug only if nothing else upholds its invariant on this path

Finding the tautology does not decide its severity. A check that verifies
nothing is a live defect only if **nothing else** upholds the invariant it
pretends to enforce. Before filing, look in two places:

1. **By construction upstream.** Trace the two things the check should have
   related and see whether how they are built already aligns them.
2. **An independent guard elsewhere**, and then **confirm it covers the exact
   path the vacuous check is on**, not merely that it exists in the crate.

Both uphold it: a **quality** finding (misleading guard, overclaiming comment;
direction: annotate it as redundant, or make it a real check against an
independent authority). Neither upholds it and a concrete divergent input
exists: then a correctness or soundness defect. Neither can be confirmed and no
input can be built: a bounded open question with a direction, not a soundness
verdict. A sibling arm that **does** read an independent authority is the tell
that the vacuous arm is anomalous rather than a considered "cannot be checked"
design. The same rule decides a closed-derivation validator's same-state re-run
half: a non-finding when it is defense-in-depth over a discriminator already
safe in both directions
([[a-guarded-new-path-plus-old-residual-codegen-specialization-clears-when-the-dangerous-direction-is-fail-closed-and-the-closed-derivation-validators-two-halves-have-different-oracle-strength]]).

Instance, 2026-08-24 on `e9c9f8be` (M6 RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION
defunctionalization, Steward section 10a gate): a finder called the
`WorkerCaptureOperand` arm of `reconcile_declared_children`
(`cranelift_backend/lowering/aggregates.rs:3382-3393`) a silent-field-swap
soundness bug. The tautology was real: the emitter sets
`origin = checked_ih_capture_origin(owner, seat, ordinal)` from the enumerate
position, and the reconciler recomputes the same call on `declared_ordinal`
after checking `ordinal == declared_ordinal`; the arm passes
`ClaimedEffectSeats::none()` (its `SiteOperand` sibling reads the independent
`ClaimedEffectSeats`), while its comment claims "three independent sources
meet here". The independent guard `validate_checked_ih_capture_suffix` is
reorder-tested but runs on the transport
(`call_checked_ih_transport_from_case_environment`) and continuation-assembly
(`assemble_continuation_call_operands`) paths, not the
`materialize_checked_ih_static_worker_application ->
emit_checked_ih_captured_environment` path. So the emit path rested on
construction alone: `worker.captures` follows the closure's declared capture
order and the plan's run is position-keyed on the same closure, so a reorder
is not constructible there. The finder's repro was hypothetical. Filed as
quality; verdict CLEAN. evt_7gdg9ge89002j (thread thr_5cnzr5sj3rdn0).
