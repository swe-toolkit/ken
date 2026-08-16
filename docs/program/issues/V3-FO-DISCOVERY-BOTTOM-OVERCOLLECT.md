---
id: V3-FO-DISCOVERY-BOTTOM-OVERCOLLECT
title: "Exclude bottom_id from conjunct 1's sort candidates, and correct the design note's direction claim, which does not cover the mechanism that produced the defect"
status: merged
owner: language
size: S
gate: none
depends_on: [V3-FO-OBLIGATION-SIGNATURE-DISCOVERY]
blocks: []
github: https://github.com/swe-toolkit/ken/pull/2375
origin: "Steward, 2026-08-15, on Adversary hunt evt_11cr9qyqympq5 against the merged range de551a4dd..427fc4069. Every line cited below was verified against the tree at origin/main a44700911 by the Steward before filing. Steward-filed per COORDINATION section 2."
---

## The defect, in four lines of the tree

`fo_kripke.rs:233-239` collects sort candidates:

```rust
if let Term::Pi(domain, _) = term {
    if let Term::Const { id, level_args } = domain.as_ref() {
        if level_args.is_empty() { sort_ids.insert(*id); }   // ANY bare Const
    }
}
```

**`⊥` is a bare `Const`.** `fo_kripke.rs:543` accepts
`Term::Const { id, .. } if *id == env.bottom_id()` as `IForm::Bottom`, and
**nothing excludes `bottom_id` from `sort_ids`.**

⇒ **An obligation carrying `⊥` in an ANTECEDENT contributes `bottom_id` as a
spurious sort candidate.** Together with the mandatory `forall`-bound sort that
makes two, and conjunct 1 refuses the obligation as ambiguous.

**It is constructible and slice-provable.** `∀x. (⊥ → (P x → P x))` needs
exactly imp-right, imp-right, init — the slice's own rule set — and **discovery
refuses it before quotation ever runs.**

## Why no existing control caught it, which is the reusable part

`negative_control_term` at `fo_kripke.rs:1032` builds
`not_px = Pi(px, Const(bottom_id))`. **The `⊥` is in the CODOMAIN**, and the
domain is an `App`, not a bare `Const` — so the collecting arm never fires on
it. **The only `⊥` in the controls sits on the side of the arrow that cannot
trigger the defect.**

⇒ A control can exercise a symbol thoroughly and still never exercise **the
position** that matters. `Pi`'s two sides are read by different code here, and
the control covers one.

## The direction claim in the design note is wrong, and that is a separate fix

`fo_kripke.rs:212` states:

> 1. A missed occurrence here only costs COMPLETENESS (a discoverable ...

**That reasons about UNDER-collection.** The mechanism here is
**OVER-collection**, which the ambiguity check then converts into a refusal.

**The conclusion still holds** — completeness only, never soundness — **but the
stated reason does not cover the mechanism that produced the defect.** A reader
auditing this walk against that sentence would check for missed occurrences and
find nothing wrong.

> **This is the failure mode the corpus keeps re-finding: a true conclusion
> resting on a warrant that does not reach the case.** The sentence is not
> repaired by deleting it; it is repaired by stating both directions and why
> each is safe.

## Deliverables

**`D0` — exclude `env.bottom_id()` from sort candidates.** One line at the
collecting arm.

**`D1` — a control with `⊥` in the ANTECEDENT.** `∀x. (⊥ → (P x → P x))` or
equivalent: discovery must succeed and the obligation must route. **It must fail
against the unfixed collector** — demonstrate that, do not argue it.

**`D2` — correct the direction claim at `fo_kripke.rs:212`.** State that the
walk can both under-collect and over-collect, and why each is completeness-only:
under-collection loses a candidate and refuses; over-collection creates
ambiguity and refuses. **Both fail closed, and saying so is the repair.**

## Acceptance criteria

**`AC-1`.** `D1` fails against the collector as it stands today and passes after
`D0`. Both halves demonstrated.

**`AC-2`.** No other `Const` exclusion is added. `bottom_id` is excluded because
`quote_iform` already reads it as `IForm::Bottom` — a specific, checkable
reason. **A general "skip suspicious constants" heuristic is out of scope and
would be a guess.**

**`AC-3`.** No FO `Proved`, no slice widening, no new kernel primitive or
trusted axiom, no change to `quote_iform`, `check_tree`, or `check_cert`.

**`AC-4`.** Conjunct 3 is untouched. Its non-vacuity is
[[V3-FO-CONVERSION-LOAD-MEASURED]] `D0`.

**`AC-5`.** No-regression, in CI (`COORDINATION §12`).

## Banned scope

- **Widening the slice** to admit anything `quote_fo` refuses today.
- **Making discovery guess** when candidates are genuinely ambiguous. This node
  removes a *spurious* candidate; it does not relax the ambiguity rule.
- **Conjunct 3 and the measurement** — [[V3-FO-CONVERSION-LOAD-MEASURED]].

## Sequencing

Independent of [[V3-FO-CONVERSION-LOAD-MEASURED]] and cheaper. **Severity is
completeness only and small**, so it does not preempt that node; take it at the
next seam, or with a second seat.

## Provenance

Adversary hunt `evt_11cr9qyqympq5`, read-only at `323c51792`, on the merged
range `de551a4dd..427fc4069`. **The Steward verified `fo_kripke.rs:212`,
`:233-239`, `:543`, and `:1032` against the tree at `origin/main` `a44700911`
before filing**, per the standing rule that a report's claims are confirmed at
the point of use.

The same hunt discharged the check it had committed to at `evt_38p85xzh3tge`:
**exactly one production `check_cert` call site** (`prover.rs:584`), returning
unconditionally to `emit_unknown_hole_fo_withheld`, with the three
`Verdict::Proved` constructions at `:318`, `:368`, `:637` all outside
`attempt_fo_with_signature`. **No second accepted-certificate exit appeared.**

**Still open and deliberately not folded here:** the `prover.rs:595` comment's
*"erase that distinction from `trusted_base()`, the one place it is otherwise
recoverable"*. The doc-only publish corrected claims elsewhere; **that source
comment is unchanged and is not this node's subject.**
