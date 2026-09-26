---
name: certify-a-kernel-totality-guard-that-adds-an-early-false-refusal-by-proving-the-refusal-predicate-is-the-divergence-certificate-then-auditing-caller-polarity
description: >-
  A kernel totality guard that fixes a non-termination hole by adding an early
  `false` refusal changes results, so byte-inertness is the wrong instrument.
  Certify it by proving the refusal predicate is the divergence certificate
  (it fires only where the old code did not terminate, so the only new
  behavior is hang to a spec-mandated `false`), then auditing caller polarity
  across the whole trust root (conjunctive combinators, negated sites
  fail-closed, no reduction fires on `false`), and by-value threading of any
  path-local ledger. The termination proof is the enclave's lane; a residual
  the spec defines as non-convertible is not a defect.
  KERNEL-CONV-RECURSIVE-HEAD-TOTALITY, 2026-08-30.
metadata:
  type: feedback
---

# Certify a kernel totality guard (early-`false` refusal) by proving the refusal predicate is the divergence certificate, then auditing caller polarity

**Measured 2026-08-30 on `KERNEL-CONV-RECURSIVE-HEAD-TOTALITY` landed squash
`81b83fb83a1d74f1e2ac7970f10f5498ecf0f04c` (2 paths, +1319/-74; sole production
file `crates/ken-kernel/src/conv.rs` +907/-74; the other path
`crates/ken-kernel/tests/recursive_head_totality_d0.rs` +412/-0, an integration
test).** Reviewed candidate `b442502d`; both product blobs byte-identical
landed == reviewed == origin/main. Reported no adversarial objection
(`evt_5ypf3v55fp53r`, thread `thr_1m61egmwhftcn`). This is `crates/ken-kernel` —
the trust root, the highest-value product-scope lane.

## The change shape — and why byte-inertness is the WRONG instrument here

The change fixes a non-termination (totality) hole in the conversion checker:
`convert`/`convert_type` on two DISTINCT transparent recursive definitions can
δ-unfold forever when each eliminator sits on a neutral scrutinee (no ι ever
fires to bottom out). The fix ADDS a path-local no-progress δ-origin ledger
threaded through the private recursion; when the same distinct-transparent-`Const`
head pair recurs on one descent with no intervening ι-progress, it returns
`false`.

This is NOT a byte-inert seam. It genuinely CHANGES results — it turns some prior
outcomes into `false`. So the byte-inert-seam tools (emission grep,
all-underscore-field non-consumption tell) do not apply. The certification is a
different argument.

## The decisive invariant: the refusal predicate IS the divergence certificate

The soundness of the whole change reduces to one claim: **the refusal fires ONLY
on inputs where the old code did not terminate.** Prove it by characterising
exactly when the guard's trigger can recur:

- The trigger is a distinct-transparent-`Const` head pair `(f, g)`, `f != g`,
  both δ-transparent, captured pre-whnf.
- Such a pair RECURS at a deeper structural edge only when both heads are
  recursive and unfold to bodies re-headed by themselves — and with NO ι-progress
  in between, that descent is infinite.
- Therefore on every input where the old checker TERMINATED, the pair cannot
  recur without ι-progress, the guard never fires, and the new result is
  byte-identical to the old.

Corollary (the trust-root conclusion): no caller that previously got `true` can
now get `false`, and none can get a spurious `true`. The ONLY new observable
behavior is **hang -> `false`**, and that `false` is sound because spec 17 §3.5
MANDATES it ("conversion MUST halt with `false` where two distinct self
`GlobalId`s meet beneath a stuck eliminator"). Cite the clause — a hang-to-false
that the spec does NOT bless would be a real question.

Adversary move: try to construct a TERMINATING input on which the refusal fires.
If the invariant holds you cannot — the refusal is gated on the divergence
certificate itself. Failing to build one is the confirmation.

## The caller-polarity audit (the actual unsoundness vector)

The vector to rule out is a sub-comparison's `false` flipping to a `true`
upstream. Grep EVERY call site of the checker across EVERY kernel file, plus
every negated `!checker(...)` use:

- **Conjunctive structural combinators** (`&&`) inside the checker: a `false`
  sub-result propagates to `false`, never negates. Confirm no arm is disjunctive
  over a sub-conversion in a way that makes `false` permissive.
- **Negated call sites** (`!convert_type`, `!convert`): each must take the
  fail-closed early-out (reject / skip / leave the eliminator stuck). Here all 11
  did (check.rs 391/418/444/450/708/741/798; obs.rs 484/508/519; conv.rs 369).
- **Positive-guard sites that FIRE a reduction** — the dangerous ones: OTT
  `Eq`/`cast`/`J` reduction in obs.rs (293/345/535/600) only reduces on `true`;
  a `false` leaves the term NEUTRAL (under-reduction = sound). The unsound shape
  would be a reduction that fires on `false` (["if NOT convertible, then
  reduce"]) — grep specifically for that and confirm it is absent.

Given the divergence-certificate invariant, results are unchanged on all
previously-terminating inputs, so these sites behave identically except that a
previously-hanging call now returns the spec-correct `false` and takes the
correct "not convertible" branch.

## Threading hygiene (path-local ledger)

A path-local ledger must be threaded BY VALUE — immutable slices, a fresh child
Vec per edge — never shared mutable state. Then sibling comparisons (Pair halves,
Pi domain/codomain) cannot contaminate each other: each gets its own copy, each
first-sighting is independent. Verify every recursive call receives its own
child path, and that the discharge/record/refuse decision is computed per edge.
(Here `child_storage: Vec<ConstPair>` + `child_path: &[ConstPair]`, correct.)

## What is NOT yours, and what is not a defect

- **Termination/decidability proof** (the guard actually bounds the descent —
  here SCT (17 §4) bounding the ι/progress count, so the ledger cannot grow
  unboundedly): that is the enclave / CV / Architect lane. Name it un-re-derived
  and move on. A failure there is a HANG (incompleteness / DoS), never an
  unsoundness — the kernel never accepts a bad proof by looping.
- **The residual completeness boundary** — genuinely definitionally-equal inputs
  the guard refuses (two distinct recursive heads that converge only via
  ι-progress AFTER two no-progress laps). If the spec DEFINES that boundary as
  declared-non-convertible for decidability (17 §3.5), it is the specified trade,
  NOT a defect. Do NOT file it. Disclose it as a residual and stop; by
  construction no program is admitted unsoundly, and no concrete regressing Ken
  program is reproducible.

## Ancillary checks that stayed cheap

- **whnf refactor behavior-preserving**: `pub whnf` became a thin wrapper
  `whnf_progress(..).0`; the reduction logic (cur assignments, unfold,
  `iota_reduct`, continue) is unchanged — only return tuples + an `iota |=`
  accumulation added, so the `Term` output is provably identical and every
  existing caller is unaffected. Confirm the reducer body, not just the wrapper.
- **Instrumentation inert in production**: the `probe_*` observation hooks are
  `#[cfg(not(test))] #[inline(always)] {}` no-ops, called as BARE statements
  (results never gate control flow); the counters are `#[cfg(test)]` only.
  Production path carries zero instrumentation.

Sibling of the byte-inert-seam lessons
[[a-byte-inert-plane-still-regresses-if-its-unconditional-build-can-err]] (that
shape changes NO result and is certified by an emission grep; THIS shape changes
results to `false` and is certified by the divergence-certificate argument) and
of
[[certify-a-merge-block-unification-refactor-by-enumerating-the-environment-divergence-axes-and-proving-each-vacuous-in-tree-or-the-intended-fix]]
(another refactor-certification shape keyed on characterising exactly when a
delta can bite).
