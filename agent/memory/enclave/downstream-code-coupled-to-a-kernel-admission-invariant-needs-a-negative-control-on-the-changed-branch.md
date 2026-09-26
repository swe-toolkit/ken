---
name: downstream-code-coupled-to-a-kernel-admission-invariant-needs-a-negative-control-on-the-changed-branch
description: >-
  Two layers of one nested-former recursion feature (2026-09-10): a whnf added
  to the kernel's trusted All-lift builder, and an elaborator Pi-guard that
  surfaces the kernel's nested IH. Each is sound only while it agrees with an
  upstream kernel admission invariant (the positivity checker's normalization;
  the kernel building every function-IH as a head Pi), and nothing pins the
  agreement. Localize admission vs generation, establish the direction, name
  the seam and the deferred increment that could invert it, and demand a
  negative control that reaches the changed branch in the exact enabled shape;
  one that reds upstream does not test the change.
metadata:
  type: feedback
---

# Downstream code coupled to a kernel admission invariant needs a negative control on the changed branch

**Measured 2026-09-10 on two layers of one feature.** Kernel layer
`486e9f33` (KERNEL-INTRINSIC-ALL-LIFT-NESTED-POSITIVE, a TCB-adjacent
post-merge hunt on the strict-positivity checker; NO DEFECT, concurring with
the Architect's positivity-preservation gate; one low-severity forward note
routed `evt_47gwqd7q70kfc`). Elaborator layer `87e2149a`
(LANG-ELAB-NESTED-FORMER-RECURSION, a required pre-merge review; the Steward
held Decision `dec_40m33a7erdajc` on the Adversary sign-off; verdict
`evt_6fhmp4t76gftf`; NO DEFECT + one forward seam note).

- Kernel: `guest_params_from_shape` now takes `env` and runs
  `whnf(env, &Context::new(), field_type)` before shape-matching, so the
  All-lift builder sees through a definition that hid a nested positive former
  (`List (Pair String Self)`).
- Elaborator: `check_match_with_lift`'s `result_ordinal == None` branch
  (+29/-1) now surfaces the evidence binder as the selectable recursive result
  iff `whnf(evidence_type)` is not a head `Term::Pi`. The `Some(ordinal)` arm
  is byte-identical, so the direct `List Self` path is untouched.

## 1. Localize what the changed code decides: admission or generation

A whnf near positivity reads as scary, since reduction can move an
occurrence between positions. Do not stop there.

1. **Is the admission decision touched?** `check_positivity` /
   `check_pos_arg` (the `PositivityViolation` emitter) was unchanged, and it
   already whnf-normalizes at every level with delta-aware `occurs_delta`
   guards. Admission already decides on the definition-seen-through form.
2. **Is the changed function admission or generation?** Grep its callers.
   `guest_params_from_shape`'s only callers are `intrinsic_former_lift_type`
   and `intrinsic_former_lift_term`, which run after admission and consume a
   pre-computed `RecursiveShape` produced only for checked-positive positions.
3. **Establish the direction of the mismatch it fixes.** Before, the builder
   normalized *less* than the checker, so it rejected folds positivity had
   admitted ("intrinsic All lift has no guest path"): over-strict, safe. The
   fix uses the same whnf as the checker and admits nothing positivity
   rejects.

## 2. Know whether a recheck backstops the downstream code

**The kernel's generated lift is trusted.** Its correctness rests on the
positivity check plus the builder faithfully reflecting the validated shape,
not on the kernel rechecking the generated term. So the safe direction is
load-bearing: the builder must never normalize *more* or *differently* than
the checker, or it would build a trusted lift for a relation admission never
validated.

**The elaborator's output is rechecked.** Verify it empirically, not from the
commit's word: the lowered global's body is a real `Term::Elim { .. }`
(`ac_nested_fold_lowers_to_a_checked_elimination`), and `env.env.trusted_base()`
is equal before and after as a `BTreeSet`. With no kernel file in the diff, the
surfaced binder can only ever be a kernel eliminator-provided IH domain (from
`all_support_evidence_positions`, positivity-gated in the kernel). The
elaborator cannot fabricate a non-IH binder or invent a decrease, so a guard
misclassification is bounded to rejection or a kernel type error, never admitted
non-termination. See
[[an-elaborator-change-clears-on-soundness-structurally-when-the-kernel-is-unchanged-and-trust-delta-is-zero]].

## 3. The negative control must reach the changed branch

**Elaborator.** `ac_totality_preserved_negative_position_is_rejected` reds
`PositivityViolation` on `List (Pair (BadRose -> Bool) String)`, but at the
data-declaration positivity check, upstream of the new branch. The negative
type never forms, so the changed branch never runs on it. It is not the
totality control for this change. The new branch's own axis is the Pi-guard:
a value-IH for a positive nested former (an `All`-evidence data application,
non-Pi, admit) versus a function / W-style IH (`(Bool -> a)` gives
`(b:Bool) -> Motive (k b)`, head Pi, reject). That axis was pinned by the
pre-existing, unmodified
`d2_wstyle_without_a_trailing_result_binder_is_exactly_out_of_scope`
(`lang_structural_result_elab.rs`), which hits the exact changed branch and
still reds `StructuralResultOutOfScope`; a no-op guard would have flipped it
to success or `NotAFunction`. No function-IH whose type whnfs to non-Pi could
be constructed.

**Kernel.** The reaching, discriminating control is a negative occurrence in
the exact enabled shape: `List (Pair (Self -> Empty) Unit)` (Self in a Pi
domain inside the same former-nested structure), asserting the specific
`"non-strictly-positive occurrence"` message. A direct-negative control
(`(Self -> Empty) -> Self`) does not exercise the through-a-former path and is
only a baseline.

## 4. Name the seam; it is not structurally enforced

**Kernel seam (low severity).** Builder and checker now share "normalize the
field the same way before shape work", but each calls `whnf(env,
&Context::new(), field)` independently, with nothing pinning that they agree. A
future change to one side's reduction discipline (delta/opaque handling,
strategy) that misses the other reopens the gap, and not necessarily on the safe
side. Route it: factor the normalization into one shared point, or add a test
asserting builder and checker see the same head for a def-hidden former
([[a-hand-written-de-bruijn-traversal-restates-the-kernels-binder-list-so-diff-it-and-pin-it-against-shift]]
builds such an oracle).

**Elaborator seam.** The Pi-guard admits on a **negative property** (`not
Term::Pi`), correct only while the kernel's `carrier_lift_type` builds every
function-IH as a head Pi and every value-IH non-Pi. The commit defers
higher-order / W-style recursion as a separate capability; when it lands,
nothing in `ken-elaborator` reddens if it builds a value-IH behind a Pi or a
function-IH behind a data wrapper, and the guard silently inverts. Soundness
stays kernel-backstopped, so the worst case is a completeness regression. Route
it so the deferred frame carries an AC to revisit the classification, or replace
the negative-property guard by asking the kernel directly whether an IH is
value- or function-valued. Same name-the-future-AC posture as
[[exhaustiveness-comes-from-an-unguarded-arm-not-from-the-match]] and
[[a-newly-reachable-allocation-producer-must-be-censused-against-its-sibling-allocators-declared-capacity-governor]].
