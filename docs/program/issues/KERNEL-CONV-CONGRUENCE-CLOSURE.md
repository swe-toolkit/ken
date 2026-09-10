---
id: KERNEL-CONV-CONGRUENCE-CLOSURE
title: "Complete conv_struct's congruence closure for the remaining formers with no arm — Quot, QuotClass, QuotElim, Cast, Omega (level_eq), Refl — each matching that former's real formation/equality rule with its own acceptance + discrimination conformance case; no blanket add"
status: ready
owner: kernel
size: M
gate: none
depends_on: [KERNEL-CONV-TRUNC-CONGRUENCE]
blocks: []
github: null
origin: "Architect kernel-level section 1b in evt_579jhptqfzcgn: conv_struct's congruence closure is incomplete for a SET of formers, not just Trunc; adding arms reactively one-per-consumer is the anti-pattern. This node captures the DELIBERATE structural closure of the remainder after the truncation pair, so the gap is named and tracked now rather than tripped later. Steward-filed per COORDINATION section 2, 2026-08-23."
---

> # FRAMED + SCHEDULED 2026-09-10 (operator ruling). Floor WP
> # [[KERNEL-CONV-TRUNC-CONGRUENCE]] is MERGED; the operator directed this node
> # DONE, with SYNTHESIZED live consumers for the arm-less formers.
> #
> # The earlier "HELD, no blocked consumer" rationale is SUPERSEDED. The operator
> # ruled (2026-09-10) the missing arms be closed PROACTIVELY and the WP itself
> # SYNTHESIZE the live consumers that exercise each one — rather than wait for a
> # quotient/cast/level-eq program to surface and trip the `_ => false` catch-all.
> # See the new consumer-synthesis deliverable and AC-CONSUMER-REACHES below.
> #
> # SEQUENCING: inserted at the FRONT of the L2 queue and run via a TEMPORARY
> # KERNEL RESEAT of the L2 slot (KERNEL-NESTED-IND precedent). The language ring
> # resumes its own queue (head LANG-SCT-OPAQUE-THROUGH-HELPER-RETURN) after this
> # lands. Each arm STILL needs an Architect design confirmation of its REAL
> # equality rule BEFORE build — that per-arm design pass is the first step, and
> # it is where a genuinely-unreachable former (e.g. Refl, shadowed by the
> # Omega-shortcut) is discharged by a documented unreachability proof instead of
> # a synthesized consumer.

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

**Plus — SYNTHESIZE a live consumer per arm (operator ruling 2026-09-10).** For
each arm-less former, author a ken-source artifact (a `.ken` fixture or a catalog
module) that FORCES `conv_struct` to decide convertibility with that former as
the head of the comparison, and that elaborates/type-checks ONLY once the arm
exists. This turns each arm's acceptance case from a conv-level unit probe into a
REACHING consumer: it fails on the pre-arm tree (the catch-all rejects the
genuine convertibility) and passes once the arm lands. Where the per-arm design
pass finds a former UNREACHABLE (Refl shadowed by the Omega-shortcut), that arm
is discharged by a documented unreachability proof, not a synthesized consumer.

# Acceptance criteria

- AC-ACCEPT / AC-DISCRIMINATE (per arm). Both directions per COORDINATION section
  7: an acceptance case (two convertible instances accepted) AND a discrimination
  case (non-convertible instances still rejected — proven not to over-accept).
- AC-ZERO-TRUST. Zero `trusted_base()` delta (completeness-only).
- AC-NO-REGRESSION. Whole-suite green in CI; local targeted `-p ken-kernel` only.
- AC-CONSUMER-REACHES (per arm). The synthesized ken-source consumer for each
  former FAILS to elaborate on the arm-less base (cite the exact refusal) and
  ELABORATES / type-checks once the arm lands — a reaching, discriminating
  witness that the arm is exercised by real source, not only a conv-level probe.
  A consumer that passes on the base (never reaches the missing arm) does not
  discharge this; an arm proven unreachable is discharged by that proof instead.

# Reviewers

kernel-QA + the Adversary (over-accept hunt per arm) + the Architect (equality
rule confirmation per arm + as-implemented pass). Trust-root change; Steward
routes the merge.

# Capability tier

T1 (multiple trust-root soundness arms, several with non-trivial equality rules —
quotients, Cast, level equality). Size M. The truncation pair is NOT here (that
is the floor WP).
