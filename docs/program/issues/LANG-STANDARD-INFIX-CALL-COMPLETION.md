---
id: LANG-STANDARD-INFIX-CALL-COMPLETION
title: "A1 of the reserved-infix-glyph objective: the reusable use-site standard-call completion adapter (the omitted-prefix / dictionary completion that 33.5.4 specifies but ships un-implemented), co-landing its first standard consumers ∧ ∨ ≤ ≥ ≠ bound to ordinary checked functions in a new Core.Operators package; the completion resolver is factored so later membership reuses the SAME scoped/coherent dictionary resolver, not a second operator dispatcher; results Bool, single left-to-right eval, no short-circuit; the definition-time NoInstance gap for a generic `where Ord a` binding is repaired or explicitly split before the generic case is claimed delivered"
status: draft
owner: language
size: L
gate: none
depends_on: [LANG-RESERVED-INFIX-NAMES, SPEC-STANDARD-INFIX-BINDING]
blocks: []
github: null
tier: T1
origin: "Steward cut 2026-09-13 from the Architect final-A decomposition (evt_784ge2nq65dfy), grounded at main 4fdd4f0ad. A1: the reusable completion link, made its own node per the Architect (do not ship an unused framework nor a third wiring node -- co-land ∧ ∨ ≤ ≥ ≠). Draft: HELD until A0 (LANG-RESERVED-INFIX-NAMES) and the A1 spec contract (SPEC-STANDARD-INFIX-BINDING) land; then the Steward frames the full ACs from evt_784ge2nq65dfy and releases. FULL A1 SCOPE + CONTROLS are in evt_784ge2nq65dfy -- this stub carries the boundaries so the DAG is complete; expand at release ('one ahead'). IN-LANE: bounded elaborator + catalog design; Architect required reviewer; no new TCB (Architect: no new Eq/DecEq instance, Float-equality law, or TCB entry needed)."
---

> # HELD (Architect A1, evt_784ge2nq65dfy). Draft; `depends_on` A0
> # [[LANG-RESERVED-INFIX-NAMES]] + the A1 spec contract
> # [[SPEC-STANDARD-INFIX-BINDING]]. Full ACs/controls are in evt_784ge2nq65dfy;
> # the Steward expands this frame and releases once both prerequisites land.

## What this is (Architect A1)

The reusable use-site call-completion adapter — the omitted-prefix / dictionary
completion that `33.5.4` specifies but that does not ship today — co-landing its
first real standard consumers `∧ ∨ ≤ ≥ ≠` so it is not an unused framework.

## Scope boundaries (full detail: evt_784ge2nq65dfy)

- Standard bindings are ordinary checked functions in a new catalog package
  (proposed `Core.Operators`), via ordinary imports: `∧`/`∨` = `bool_and`/
  `bool_or`; `≤`/`≥` take a type + actual `Ord` dictionary (`≥` reverses
  already-evaluated VALUES); `≠` = negation of the exact NumericEnv `==`
  comparator on the SAME supported carriers.
- The common call elaborator completes the omitted prefix for the NEW standard
  bindings (infer carrier; resolve dictionary via the existing class resolver OR
  select the existing equality comparator; check the saturated application in the
  kernel), bound to the defining GlobalId + checked telescope, NOT glyph text.
- Factor dictionary completion so comparisons AND later membership reuse the
  SAME scoped/coherent resolver (not two operator dispatchers). Preserve
  class/head ownership, provenance, overlap/refusal, and the explicit
  named-dictionary escape. Thread a genuine local `where` dictionary as the given
  (do not resolve it as a global instance named "a"). REPAIR or explicitly SPLIT
  the present definition-time `NoInstance { Ord, a }` gap before claiming the
  generic case delivered.
- Standard fixities: `∧ infixr 3`, `∨ infixr 2`, `≤ ≥ ≠ infix 4`. Results Bool;
  no automatic Ω connective/equality refinement/proof-witness conversion;
  ordinary application evaluation (no invented short-circuit); single evaluation,
  left-to-right operand order (especially `≥`).
- Do NOT claim general implicit inference, operator sections, or arbitrary
  partial application are complete. `ord_leq_at Nat d` stays a valid partial
  application; an unrelated local `≤` stays an ordinary function.

## Controls (expand from evt_784ge2nq65dfy at release)

Typed missing / ambiguous / unadmitted-instance refusals; real nontrivial `Ord`
choices and generic-given threading; exact kernel applications vs explicitly
supplied dictionary/comparator adapters; alias/shadowing isolation; `≥`
operand-order / single-evaluation; complete Boolean truth tables; `≠`
complementary to `==` on the same supported carriers incl. IEEE edges, with the
same unsupported-carrier refusals (enumerate the FULL NumericEnv carrier
inventory, incl. Float32 and Decimal's registered representation, not just the
Int/Float/Char vs Nat/Bool/String sample). No automatic DecEq fallback or
numeric-order substitute for `Ord`.

## Not this node

- Membership's standard binding / `class Membership` — the parallel B track
  (reuses this node's resolver when it lands).
- The name/fixity admission (A0) and the spec contracts.

## Sizing / tier

**Size L, tier T1.** Soundness-adjacent completion machinery + a new catalog
package; the review turns on the resolver-factoring and the NoInstance-gap
repair/split. Architect required reviewer.

## Contention

Language ring: `crates/ken-elaborator/src` + tests, AND a new catalog path
(`Core.Operators`) — A1 is NOT parser-only; enumerate the globbed catalog
consumers at the cut. CODE merge -> full CI, M8/M8a Adversary. Re-measure the
NumericEnv carrier inventory and the class-resolver anchors at the cut.
