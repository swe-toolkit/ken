# WP — KERNEL-CONV-TRUNC-CONGRUENCE: truncation-former congruence pair in conv_struct

Lane-2 (language / FO) critical-path PREREQUISITE, built by the kernel ring. One
WP, one branch `wp/kernel-conv-trunc-congruence`, one PR. Owner: kernel. Size: S.
Gate: none. Depends on: none. Blocks: V3-FO-EMBEDDING-ADEQUACY (the
quotation-preservation Or arm).

Source: Architect ruling `evt_579jhptqfzcgn` (D1 Trunc-congruence hard stop,
language-implementer `evt_4x7zr3jepz62t`; language-qa test-drive
`evt_2rdx75e35bbhb`). Full spec + soundness analysis + scope decision:
`docs/program/issues/KERNEL-CONV-TRUNC-CONGRUENCE.md`.

Fixed inputs measured at `origin/main a7603b31b`.

## Objective

The ordinary kernel conversion gate (whnf + congruence, `conv_struct`) rejects
convertible-but-not-syntactically-identical truncation types because `conv_struct`
has no `(Term::Trunc, Term::Trunc)` congruence arm. Add the truncation-former
congruence PAIR — Trunc AND its eliminator TruncProj — so the gate recognises
`‖A‖ ≡ ‖B‖` whenever `A ≡ B`. Completeness-only, fail-closed; unblocks
V3-FO-EMBEDDING-ADEQUACY's fixed five-form quotation-preservation slice (the Or
arm).

## Fixed inputs (SETTLED, at `origin/main a7603b31b`)

1. `crates/ken-kernel/src/conv.rs` — `conv_struct` fn (`:404`), whnf both sides
   (`:434-435`), `a==b` syntactic fast path (`:436`), the congruence-arm match
   (Type/Var/Const/IndFormer/Constructor/Pi/Lam/Sigma/Pair/App/Proj1/Proj2/Elim/
   Ascript/Absurd/Eq/IntLit), catch-all `_ => false` (`:578`). No Trunc arm.
2. `normalize` (`:288-289`) recurses `Term::Trunc(a) => Trunc(normalize a)` and
   TruncProj — the asymmetry that makes full normal forms equal while ordinary
   conversion returns false.
3. The proof-irrelevance boundary to NOT cross: `convert` Omega-shortcut
   (`:345-347`, keyed on `is_omega_type`) is element-level Trunc irrelevance;
   this arm is reached via `convert_type` (`:397`) which bypasses that shortcut.
   Different judgment — do not implement "all inhabited props are equal".
4. Precedent: the Eq-congruence arm re-landed as a previously-missing closure
   (comment `:560-565`) — same direction and justification.

## The change (Architect-settled)

    (Term::Trunc(p1), Term::Trunc(p2)) => conv_struct(env, ctx, p1, p2),

plus the analogous TruncProj arm matching TruncProj's real formation/elimination
rule. Nothing else in conv.rs; no change to `normalize`, `convert`, the
Omega-shortcut, or any trusted-base surface.

## Acceptance criteria

- AC-ACCEPT (per arm). `‖A‖`/`‖B‖` with `A ≡ B` (not syntactically identical)
  accepted by `convert_type`; same for TruncProj on convertible interiors.
- AC-DISCRIMINATE (per arm). `‖A‖` vs `‖B‖` with `A ≢ B` still REJECTED; same
  for TruncProj — proven not to over-accept (COORDINATION section 7, both
  directions, kernel-level cases owned by this WP).
- AC-CONSUMER (downstream, verified by language). The held D1 branch
  `wp/V3-FO-EMBEDDING-ADEQUACY-D1` Or cell greens once this lands.
- AC-ZERO-TRUST. Zero `trusted_base()` delta.
- AC-NO-REGRESSION. Whole-suite green in CI; local targeted `-p ken-kernel`.

## Scope

Truncation PAIR only (Trunc + TruncProj) — the Architect-settled floor. The
remaining missing arms {Quot, QuotClass, QuotElim, Cast, Omega-level, Refl} are
the filed follow-up `KERNEL-CONV-CONGRUENCE-CLOSURE` (named now, held). Do NOT
blanket-add — each arm needs its own equality rule + conformance pair.

## Reviewers / contention

kernel-QA + Adversary (over-accept hunt) + Architect (as-implemented pass).
Trust-root change; the Steward routes the merge only on all three + a resolved
Decision. Touches conv.rs only — contention-free with both lanes. Code change ⇒
full CI on merge.

## Capability tier

T1 (trust-root soundness; small diff, argument-carried review). Size S.
