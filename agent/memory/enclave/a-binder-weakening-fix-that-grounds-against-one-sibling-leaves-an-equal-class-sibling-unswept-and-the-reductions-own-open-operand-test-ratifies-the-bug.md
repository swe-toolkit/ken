---
name: a-binder-weakening-fix-that-grounds-against-one-sibling-leaves-an-equal-class-sibling-unswept-and-the-reductions-own-open-operand-test-ratifies-the-bug
description: >-
  A de Bruijn binder-hygiene fix on a kernel reduction arm (a Pi/Sigma whose
  codomain is an outer-context term needing weaken(_,1)) that grounds itself
  against one sibling arm leaves the equal-class siblings unswept; here
  eq_at_omega weakened nothing. Closed operands hide it, and the reduction's own
  open-operand test ratified the bug because its expected term was hand-encoded
  with the same off-by-one. Sweep the defect class across every sibling arm;
  derive the correct de Bruijn value independently; a mis-weakening shows first
  as completeness, so do not claim soundness without a witness.
metadata:
  type: feedback
---

# A binder-weakening fix that grounds against one sibling leaves an equal-class sibling unswept; the reduction's own open-operand test ratifies the bug

**Measured 2026-09-02 while returning the advisory verdict on the binder-hygiene
M8 hunt (`LANG-RECORD-INDEX-REFINEMENT-KERNEL`, landed squash `8080fe26`, sole
production file `crates/ken-kernel/src/obs.rs`).** The landed fix was correct and
well-pinned; the finding is a **pre-existing, equal-class defect in a sibling the
fix did not sweep**, surfaced by attacking the fix's *blast radius* (the defect
class) rather than only the changed line. Filed to the Steward, thread
`thr_3kffa05kvafe0`, as a HIGH kernel/TCB finding.

## The shape

`obs.rs::eq_reduce` (the observational-equality whnf) dispatches by the whnf'd
type to one arm per type former. Several arms build a Π or Σ whose **codomain is
an outer-context term**, which must therefore be `weaken(_, 1)` past the new
binder. The established, correct convention in the file:

- `eq_at_pi` weakens `f`,`g` by 1 (obs.rs:130, explicit comment).
- `eq_at_sigma` weakens `second` by 1 (obs.rs:167).
- `eq_at_inductive` **now** weakens `acc` by 1 (obs.rs:308) — the just-landed fix.

The fix's commit message grounded its correctness against `eq_at_sigma`
verbatim ("exactly as `eq_at_sigma` does with `weaken(&second, 1)`"). It did
**not** audit `eq_at_omega` (obs.rs:171-174), the propext arm
`Eq Ω P Q ⇝ (P→Q) ∧ (Q→P)`, which weakens **nothing**:
`Term::pi(p, q)`, `Term::pi(q, p)`, and `Term::sigma(p_to_q, q_to_p)` all place
the outer-context props `p`,`q` under new binders with no lift.

## Why it survived, and the two detectors

**Reachability + hiding.** `eq_reduce:79` calls `eq_at_omega` unconditionally
whenever the type whnf's to Ω — no canonicity guard on the operands — so any
`Eq Ω P Q` with an **open** (free-variable) `P` or `Q` reaches it. Closed props
are unaffected because `weaken` is the identity on closed terms, which is why the
closed-prop conformance corpus never tripped it.

**Detector 1 — sibling-class sweep.** A binder-weakening fix that *names its
precedent sibling* is a signal to audit the family's **other** siblings: a
commit that reasons from one precedent tends to check only that precedent. Same
family as auditing your own new lines for the defect class you just diagnosed,
and as [[a-fix-that-closes-the-named-counterexample-need-not-close-the-class]]
(a divergence in one function does not generalize to a sibling you never opened)
— here with a concrete kernel manifestation. The clearing-direction twin of this
sweep is
[[an-elaborator-acceptance-widening-clears-by-construction-and-the-payoff-is-sweeping-every-consumer]].

**Detector 2 — vacuous reduction oracle.** The suite *did* have an open-operand
test, `k2_propext_definitional` (acceptance.rs:1421-1438), and it **ratified the
bug**: it hand-encodes `expected = Σ(Π(var1,var0), Π(var0,var1))` — the
identical off-by-one as production — so `assert_eq!(whnf(...), expected)` passes
green-vs-green, and the fix's own no-regression run over `acceptance` read
green. The tell: **the expected term is authored by the same hand and the same
(wrong) mental model as the reduction**, so a reduction's own round-trip test is
worthless as a no-regression oracle for a binder bug. This is the reduction-arm
face of
[[a-pin-cannot-disagree-with-its-own-source]].

## Grounding technique

Derive the CORRECT de-Bruijn value **independently**, and cross-check the
convention against a **known-correct sibling test** — `k2_quotient_eq_is_relation`
(acceptance.rs:1529-1532) builds `A→A→Ω` and explicitly weakens the inner `A` to
`var(1)` ("Inner A weakens past each binder"). For `ctx=[P:Ω(var1),Q:Ω(var0)]`
the correct reduct is `Σ(Π(var1,var1), Π(var1,var3))`, not the produced
`Σ(Π(var1,var0), Π(var0,var1))` (which is `Σ(Π(x:P,x), …)` — ill-typed as a Π).
The **executable repro** is: replace the vacuous test's `expected` with the
correct value and confirm it reds against current code. (Reason-verified, not
executed — the box was memory-saturated, §12; state that honestly.)

## Direction

A mis-weakening in the TCB manifests **first** as completeness — open-operand
reducts go ill-typed, so valid `Eq Ω A A`-style equalities become unprovable
(the safe direction). Do **not** claim the over-acceptance (soundness) direction
unless you construct a witness; report it as an un-ruled-out latent hazard and
name the residual. Sibling in the "de-escalate the finder-lens's soundness read
to what the kernel backstop actually leaves open" move applied the same day to
the W-style IH arm (elab.rs), where the kernel's `method_type` convertibility
check closed the soundness gap to a false-reject.
