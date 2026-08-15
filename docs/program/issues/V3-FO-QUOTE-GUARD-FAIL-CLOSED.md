---
id: V3-FO-QUOTE-GUARD-FAIL-CLOSED
title: "Make quote_fo's pre-quotation guards fail closed: Pair is not a binder, and a proof-variable-occurrence test must default to true"
status: ready
owner: language
size: S
gate: none
depends_on: [V3-FO-KRIPKE-SLICE]
blocks: []
github: null
origin: "Steward, 2026-08-15, dispositioning Adversary finding evt_235vyn7za92ry on the merged range 75a91d2ba...6ec9694fa. Filed as its own node rather than folded into V3-FO-OBLIGATION-SIGNATURE-DISCOVERY, which is already released and is about a different surface. Steward-filed per COORDINATION section 2."
---

## `Pair` is classified as a binder. The kernel says twice that it is not.

`mentions_var0` in `fo_kripke.rs`:

```rust
Term::Pi(a, b) | Term::Lam(a, b) | Term::Sigma(a, b) | Term::Pair(a, b)
    => go(a, depth) || go(b, depth + 1),
```

`ken_kernel::subst` on the same constructor:

```
subst.rs:36   Sigma(a, b) => sigma(shift(a, d, cutoff), shift(b, d, cutoff + 1))   // binder
subst.rs:44   Pair (a, b) => pair (shift(a, d, cutoff), shift(b, d, cutoff))       // NOT
subst.rs:147  Pair (a, b) => pair (subst_var(a, j, u), subst_var(b, j, u))         // NOT
```

⇒ **`Pair` is a term former, not a binder**, and grouping it with `Sigma` makes
the guard wrong **in both directions**:

| input | `mentions_var0` says | truth |
|---|---|---|
| `Pair(_, Var(0))` — the proof variable | **false** | it IS mentioned |
| `Pair(_, Var(1))` | **true** | it is not |

**The false positive is safe** — an extra `DependentProofUse` refusal. **The
false negative is the unsound direction:** it certifies the proof hypothesis
absent, and `shift(codomain, -1, 0)` then runs on a term that references it.

## The same root, wider: `_ => false` is fail-open

`Elim`, `Let`, `Ascript`, `Refl`, `Cast`, `J`, `Quot`, `QuotClass`, `QuotElim`,
and `Absurd` all carry term subterms and all fall to `_ => false`, so a `Var(0)`
inside any of them reads as absent.

> **A guard answering *"does the proof variable appear?"* must default to
> `true`.** Conservative `true` yields `Err(DependentProofUse)` — a refusal, the
> safe direction. `false` yields an erasure.

## THE STRUCTURAL DEFECT, which is the part that is not one line

`quote_iform`'s `_ => Err(UnsupportedTermShape)` **is** a construction-level
refusal, and out-of-slice shapes genuinely cannot survive it. **But
`mentions_var0` and `shift` run BEFORE that refusal, on unvalidated input.**

What makes them safe today is that `quote_iform`'s accepted grammar happens to be
a **subset** of the constructors `mentions_var0` traverses.

⇒ **Two enumerations, in two functions, maintained separately, with nothing tying
them together.** The slice is explicitly built to grow — `top`/`and`/`exists` and
a real atom theory are on its roadmap — and **the first accepted shape that
`mentions_var0` does not traverse turns a latent bug into a live one.**

**Fixing only the two arms leaves the coupling unfixed.** Prefer a structure where
adding an accepted shape cannot silently outrun the guard.

## Severity, stated without inflation

**Latent. There is no current soundness exposure.** Route FO is unreachable in
production, `23 §4.4` forbids `proved`, and both exits converge on
`emit_unknown_hole`. The `Pair` misclassification is **already wrong today and
merely unreachable.**

This is a defect in a guard inside a slice built to grow, filed so it is fixed
while it is cheap rather than when it is load-bearing.

## Deliverables

**`D0` — `Pair` is not a binder.** Traverse both subterms at the same depth,
matching `subst.rs:44`.

**`D1` — the default becomes `true`.** No constructor carrying term subterms may
reach a `false` default. Prefer exhaustive matching over a wildcard, so a new
kernel constructor is a compile error rather than a silent fail-open.

**`D2` — tie the two enumerations together, or state why they cannot be.** A
recorded mechanism explaining why the coupling is not expressible is a complete
deliverable; a silent restatement of the status quo is not.

**`D3` — two comment corrections where the conclusion holds and the stated reason
does not.** Both were checked and both conclusions stand; only the justifications
are wrong, and a later author will rely on them.

- **`shift`'s underflow guard prevents corruption, not capture.** `Var(0)` at
  `d = -1, cutoff = 0` is left unchanged, so no panic and no wrapped index — but
  the un-shifted `Var(0)` aliases whatever the next enclosing binder becomes once
  the other indices shift down. **Fail-safe at the representation level, not the
  semantic one.** Say so at the site.
- **The `ForallRight` sort collapse is not exploitable, but its stated reason is
  self-contradicting.** The module doc justifies the absent `QSort` by *"every
  relation and quantifier already fixes which sort each slot is"* — and
  `ForallRight` is the one site that consumes a quantifier **without recording
  which**. Freshness plus `Init`'s syntactic equality is what actually blocks
  confusion. **That is the sentence a second sort would be added against.**

## Acceptance criteria

**`AC-1`.** `D0` and `D1` are each demonstrated by a test that **fails before the
fix** — a `Pair(_, Var(0))` case and at least one wildcard-defaulted constructor
carrying `Var(0)`.

**`AC-2`.** No accepted `IForm` shape is traversable by `quote_iform` but not by
the occurrence guard. **Demonstrate the property, not the current instance** —
the point is that growth cannot outrun it.

**`AC-3`.** No behavior change for any obligation the slice accepts today. This
is a guard correction, not a widening.

**`AC-4`.** `proved` is still not returned for FO, and the slice is not widened.

**`AC-5`.** No-regression, in CI (`COORDINATION §12`).

## Banned scope

- **Widening the slice.** `top`/`and`/`exists` and a real atom theory are later
  nodes; this node makes the guard safe **for** that growth, it does not perform
  it.
- **Signature discovery.** [[V3-FO-OBLIGATION-SIGNATURE-DISCOVERY]].
- **The theorem home and the evaluator posture.** Neither is touched here.

## Provenance

Adversary finding `evt_235vyn7za92ry`, hunt on `75a91d2ba...6ec9694fa`,
read-only. The kernel citations (`subst.rs:36`, `:44`, `:147`) are the Adversary's
and are the basis of the `Pair` claim; **the implementer should confirm them
against the tree before building**, per the standing rule that a cited line is
verified at the point of use.

The Adversary also recorded three things as checked and sound, so a later pass
need not re-run them: `shift` is total, `IndFormer`/`Constructor` carry no term
subterms, and `check_tree`'s three rules enforce linearity rather than assuming
it.
