---
id: SPEC-STANDARD-INFIX-BINDING
title: "A1's spec prerequisite: a bounded standard-binding + use-site call-completion contract grounded in 33.5.4 (implicit argument/dictionary completion) and 39 (elaboration), specifying the STANDARD meanings of ∧ ∨ ≤ ≥ ≠ as ordinary checked standard-package functions (∧/∨ = bool_and/bool_or; ≤/≥ = Ord via an actual dictionary, ≥ reversing already-evaluated values; ≠ = negation of the exact NumericEnv `==` comparator on the SAME supported carriers), their standard fixities (∧ infixr 3, ∨ infixr 2, ≤ ≥ ≠ infix 4), and the completion policy bound to defining GlobalId + checked telescope; results Bool, no Ω connective, single left-to-right evaluation, no short-circuit guarantee; NO new Eq/DecEq instance, Float-equality law, or TCB entry"
status: merged
owner: spec
size: S
gate: none
depends_on: [SPEC-RESERVED-INFIX-NAMES]
blocks: [LANG-STANDARD-INFIX-CALL-COMPLETION]
github: null
tier: T1
origin: "Steward cut 2026-09-13 from the Architect final-A decomposition (evt_784ge2nq65dfy), grounded at main 4fdd4f0ad. A1's Spec-owned prerequisite. Draft: released after SPEC-RESERVED-INFIX-NAMES lands (the enclave may pipeline). Full A1 design detail, including the exact binding shapes the Architect probed and the NumericEnv `==` carrier inventory to enumerate (Int/Float/Char accept, Nat/Bool/String reject in the sample -- include Float32 and Decimal's actual registered representation when framing the full inventory), is in evt_784ge2nq65dfy. IN-LANE: bounded normative surface, no new trust-root/TCB (Architect: no new Eq/DecEq instance, Float-equality law, or TCB entry needed). Architect is the design authority and reviewer."
---

> ## MERGED 2026-09-16 at `9a56f81b9a4c095140d85b30a03fb10c25e46a85`
>
> **Verified by blob, not by ancestry** — the publisher squashes, so a routed
> commit is an ancestor of nothing. All four touched paths
> (`conformance/README.md`,
> `conformance/surface/operators/seed-standard-infix-binding.md`,
> `spec/30-surface/33-declarations.md`, `spec/30-surface/39-elaboration.md`)
> are byte-identical between the approved candidate
> `653c3444d107a6daf016d0d6ffc21f536642134e` and `main`, landed via PR #3787
> with full-mode CI green. Decision `dec_6bhhzhqd8kwnr` resolved; Architect
> and CV both approved with independent re-derivation. The `∈` half of
> `SPEC-MEMBERSHIP-CLASS-CONTRACT` (B-track) is unblocked from this landing.

> # READY. Frame:
> `docs/program/wp/SPEC-STANDARD-INFIX-BINDING.md`.
>
> **The release condition this node set itself is MET.**
> [[SPEC-RESERVED-INFIX-NAMES]] is `merged`; so is
> [[SPEC-RESERVED-INFIX-APP-BOUNDARY-CORRECTION]]. Design authority remains the
> Architect's final-A decomposition `evt_784ge2nq65dfy`.
>
> **This is the lane-2 critical path.** A1
> ([[LANG-STANDARD-INFIX-CALL-COMPLETION]]) is held on this node, and
> [[LANG-MEMBERSHIP-OPERATOR-SURFACE]] is held on A1. Lane 2 has one `ready`
> node and it is in flight; when it lands the language ring goes idle unless
> this contract has moved.
>
> **The `≠` carrier inventory is MEASURED, and the census closes** — frame §2b.
> Five rows, and two of them are not what the prose above implies: **Decimal is
> keyed on `decimalpair_id`, not `decimal_id`**, and there are **two distinct
> refusals** (a table miss, and a head that is neither `Const` nor `IndFormer`),
> which one "unsupported carrier" sentence would collapse. Read frame §2c before
> writing the normative text.

## What this settles

The STANDARD meanings and the use-site completion contract for the operators A0
admits as names, grounded in `spec/30-surface/39-elaboration.md` and `33.5.4`
(implicit argument / dictionary completion, currently specified but not shipped):

1. **Standard bindings** as ordinary checked standard-package functions (a
   proposed `Core.Operators` home), reached through ordinary imports:
   - `∧` = `bool_and`, `∨` = `bool_or` (Bool, no invented short-circuit);
   - `≤` / `≥` = `Ord` via an ACTUAL dictionary; `≥` reverses already-evaluated
     VALUES inside its wrapper (not by reversing operand ASTs);
   - `≠` = negation of the exact comparator selected by the existing NumericEnv
     `==` path, on the SAME supported carriers, with the same unsupported-carrier
     refusals (it is NOT universal disequality and NOT DecEq).
2. **Completion policy.** The common call elaborator completes the omitted prefix
   for the NEW standard bindings: infer the operand carrier, resolve the required
   dictionary via the existing class resolver OR select the existing equality
   comparator, then check the fully saturated ordinary application in the kernel.
   The policy binds to the defining GlobalId + checked telescope, NOT to the
   occurrence's glyph text. A renamed standard binding must not be lost; an
   unrelated local `≤` stays an ordinary function; `ord_leq_at Nat d` stays a
   valid partial application (do not reinterpret every 2-arg call as a 4-arg
   helper).
3. **Standard fixities** (of the bindings, not the tokens): `∧ infixr 3`,
   `∨ infixr 2`, `≤ ≥ ≠ infix 4`. Legacy `==` unchanged. Results Bool; no
   automatic Ω connective, equality refinement, or proof-witness conversion.
   Single evaluation, left-to-right operand order (especially `≥`).

## Not this node

- The elaborator implementation — [[LANG-STANDARD-INFIX-CALL-COMPLETION]] (A1).
- The name/fixity-target admission — [[SPEC-RESERVED-INFIX-NAMES]] (A0's prereq).
- Membership's standard binding / class — the parallel B track.

## Sizing / tier

**Size S, tier T1.** Short contract; T1 because it fixes the completion policy
and the `≠`/`==` carrier coherence that A1's soundness review turns on.
Spec-enclave-owned; Architect design authority and reviewer.

## Contention

Spec enclave, `spec/30-surface/`. No cross-lane contention. Re-measure anchors
(33.5.4, 39, the NumericEnv carrier inventory) at the cut.
