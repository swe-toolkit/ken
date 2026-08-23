---
id: KERNEL-CONV-CONGRUENCE-CLOSURE
title: "Complete conv_struct's congruence closure for the remaining formers with no arm — Quot, QuotClass, QuotElim, Cast, Omega (level_eq), Refl — each matching that former's real formation/equality rule with its own acceptance + discrimination conformance case; no blanket add"
status: draft
owner: kernel
size: M
gate: none
depends_on: [KERNEL-CONV-TRUNC-CONGRUENCE]
blocks: []
github: null
origin: "Architect kernel-level section 1b in evt_579jhptqfzcgn: conv_struct's congruence closure is incomplete for a SET of formers, not just Trunc; adding arms reactively one-per-consumer is the anti-pattern. This node captures the DELIBERATE structural closure of the remainder after the truncation pair, so the gap is named and tracked now rather than tripped later. Steward-filed per COORDINATION section 2, 2026-08-23."
---

> # FILED to name the full gap deliberately — HELD (no current blocked consumer)
>
> Sequenced AFTER the floor WP [[KERNEL-CONV-TRUNC-CONGRUENCE]] (truncation
> pair). Held draft: no live consumer is blocked on these arms today, and each
> remaining arm's equality rule needs Architect design confirmation before build
> (unlike the truncation pair, whose spelling+soundness the Architect settled).
> Filing now keeps the closure deliberate, not reactive.

# The remaining gap (Architect evt_579jhptqfzcgn)

`conv_struct` (`crates/ken-kernel/src/conv.rs:404`, catch-all `_ => false` at
`:578`) has congruence arms for Type, Var, Const, IndFormer, Constructor, Pi,
Lam, Sigma, Pair, App, Proj1, Proj2, Elim, Ascript, Absurd, Eq, IntLit. After
the truncation pair (Trunc + TruncProj) lands via the floor WP, the formers still
lacking a congruence arm are:

- Quot, QuotClass, QuotElim — the quotient family (each with its own formation
  and equality rule; QuotClass/QuotElim are not plain structural recursion).
- Cast — its own formation/equality rule.
- Omega — needs a level equality (`level_eq`) like the existing Type arm, not a
  bare structural recurse.
- Refl — mostly shadowed by the Omega-shortcut (`convert` `:345-347`), so its
  arm may be a no-op in practice; confirm whether it is reachable via
  `convert_type`.
- (Let is fine — whnf reduces it; not in scope.)

# Deliverable

Add each missing congruence arm, matching that former's REAL formation/equality
rule — NOT a blanket structural recurse. The Architect's bar: "each added arm
must match that former's real formation/equality rule and carry its own
conformance case; do not blanket-add." Each arm requires an Architect design
confirmation of its equality rule before build (the truncation pair was settled
in evt_579jhptqfzcgn; these are not).

# Acceptance criteria

- AC-ACCEPT / AC-DISCRIMINATE (per arm). Both directions per COORDINATION section
  7: an acceptance case (two convertible instances accepted) AND a discrimination
  case (non-convertible instances still rejected — proven not to over-accept).
- AC-ZERO-TRUST. Zero `trusted_base()` delta (completeness-only).
- AC-NO-REGRESSION. Whole-suite green in CI; local targeted `-p ken-kernel` only.

# Reviewers

kernel-QA + the Adversary (over-accept hunt per arm) + the Architect (equality
rule confirmation per arm + as-implemented pass). Trust-root change; Steward
routes the merge.

# Capability tier

T1 (multiple trust-root soundness arms, several with non-trivial equality rules —
quotients, Cast, level equality). Size M. The truncation pair is NOT here (that
is the floor WP).
