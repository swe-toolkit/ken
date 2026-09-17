---
id: LANG-STANDARD-INFIX-CALL-COMPLETION
title: "A1 of the reserved-infix-glyph objective: the reusable use-site standard-call completion adapter that spec 39 section 6.9 specifies and that ships un-implemented, co-landing its first standard consumers ∧ ∨ ≤ ≥ ≠ bound to ordinary checked functions; ∧ ∨ ≤ bind to the ALREADY-LANDED `Core.Classes.LawfulClasses` bindings rather than to a second copy in a new package, because 6.9 keys completion on ONE defining GlobalId; ≥ and ≠ are the genuinely new bindings; the completion resolver is factored so later membership (6.10) reuses the SAME scoped/coherent dictionary resolver, not a second operator dispatcher; results Bool, single left-to-right eval, no short-circuit; the definition-time NoInstance gap for a generic `where Ord a` binding is repaired or explicitly split before the generic case is claimed delivered"
status: ready
owner: language
size: L
gate: none
depends_on: [LANG-RESERVED-INFIX-NAMES, SPEC-STANDARD-INFIX-BINDING]
blocks: [LANG-MEMBERSHIP-OPERATOR-SURFACE]
github: null
tier: T1
origin: "Steward cut 2026-09-13 from the Architect final-A decomposition (evt_784ge2nq65dfy), grounded at main 4fdd4f0ad. A1: the reusable completion link, made its own node per the Architect (do not ship an unused framework nor a third wiring node -- co-land ∧ ∨ ≤ ≥ ≠). RELEASED 2026-09-17 by the Steward, re-grounded at main 8f0f3753270b2d7639fc7aed457b759e6e6a57a1: both prerequisites are merged, and the re-grounding refuted one of the stub's own scope lines (see the RELEASED banner). IN-LANE: bounded elaborator + catalog design; Architect required reviewer; no new TCB (Architect: no new Eq/DecEq instance, Float-equality law, or TCB entry needed)."
---

> # RELEASED 2026-09-17 (Steward), re-grounded at main `8f0f37532`.
>
> Both prerequisites are `merged`: A0 [[LANG-RESERVED-INFIX-NAMES]] (landed
> `a631e4fb2`, closed `757ec6049`) and the A1 spec contract
> [[SPEC-STANDARD-INFIX-BINDING]]. The normative text this node implements is
> landed and is the authority for every question below:
> **`spec/30-surface/39-elaboration.md`, section 6.9 "Standard-operator call
> completion"** (its sibling section 6.10 is the B track, not this node).
>
> **Treat every anchor here as perishable. If a fixed input turns out false
> against the landed code, say so and escalate — do not quietly build around
> it.** One of them already was: see FI-2.

## What this is (Architect A1)

The reusable use-site call-completion adapter — the omitted-prefix / dictionary
completion that `39 section 6.9` specifies but that does not ship today —
co-landing its first real standard consumers `∧ ∨ ≤ ≥ ≠` so it is not an
unused framework.

## Fixed inputs, measured at main `8f0f37532`

Cited by grep-able phrase. Every line number is an anchor to re-find at your
own cut, never a value to check.

**FI-1. The contract is landed and it is the authority.**
`spec/30-surface/39-elaboration.md` section 6.9 fixes the completion order
(infer the operand carrier; resolve the required dictionary by the ordinary
instance search of section 6.2, or for `≠` select the registered comparator for
that carrier per `33 section 6.2`; then check the saturated application), and
fixes that a failure at any step is **an ordinary elaboration error at the
occurrence, never a silent fallback to a different meaning**. Do not re-derive
this from the stub's prose; read section 6.9.

**FI-2. THE STUB'S "new catalog package (proposed `Core.Operators`)" IS
REFUTED FOR `∧ ∨ ≤`, AND THIS CHANGES THE DELIVERABLE.**
Three of the five standard bindings already exist, are already `pub`, and
already carry the exact telescope section 6.9 keys on:

    pub fn ord_leq_at (a : Type) (d : Ord a) (x : a) (y : a) : Bool = d.leq x y
    pub fn bool_or    (a : Bool) (b : Bool) : Bool
    pub fn bool_and   (a : Bool) (b : Bool) : Bool

all in `catalog/packages/Core/Classes/LawfulClasses.ken.md`, alongside
`pub class Ord a` and `pub class DecEq a`. `catalog/packages/Core/` holds
exactly `Classes` and `Logic`; there is no `Operators` package.

⇒ **Authoring a second copy in a new package would mint a SECOND `GlobalId`
for a meaning that already has a canonical one, and section 6.9's entire policy
is that completion binds to THE defining `GlobalId` and its checked telescope,
never to glyph text.** Two homes for one meaning is not untidiness here; it is
a direct contradiction of the rule this node exists to implement, and it would
make section 6.9's own stated consequence — *"a renamed standard binding still
completes"* — ill-defined.

**So: `∧ ∨ ≤` bind to the landed `Core.Classes.LawfulClasses` bindings.** The
genuinely new bindings are **`≥`** (reverses already-evaluated VALUES) and
**`≠`** (negation of the exact NumericEnv `==` comparator on the same
supported carriers). Their home is `Core.Classes.LawfulClasses` too, unless
the Architect rules otherwise at review — **this is the Steward's call on a
fork the stub left open with the word "proposed", resolved from FI-1 rather
than escalated, and it is the first thing a reviewer should attack.**

**FI-3. The definition-time `NoInstance` gap is real and located.**
`crates/ken-elaborator/src/prelude.rs` states in its module header that
`instance_search` runs **before the body elaborates**, emitting `NoInstance`;
the construction sites are in `crates/ken-elaborator/src/elab.rs` and the
variant is `ElabError::NoInstance { class, ty, span }` in
`crates/ken-elaborator/src/error.rs`. A generic `where Ord a` binding therefore
fails at definition time against the abstract tyvar `a`. **Repair it or split
it explicitly — see AC-6. Do not claim the generic case delivered either way
without discharging AC-6.**

**FI-4. `membership_member_at` is NOT a catalog binding today.** It appears
only as a test fixture string in
`crates/ken-elaborator/tests/lang_type_projection_surface_form.rs`. That is
correct and expected: it is the B track. Do not author it here.

## Scope boundaries

- Standard bindings are ordinary checked functions reached by ordinary imports:
  `∧`/`∨` = the landed `bool_and`/`bool_or`; `≤` = the landed `ord_leq_at`
  (type + actual `Ord` dictionary); `≥` reverses already-evaluated VALUES; `≠`
  = negation of the exact NumericEnv `==` comparator on the SAME supported
  carriers. See FI-2 for where each lives.
- The common call elaborator completes the omitted prefix (infer carrier;
  resolve dictionary via the existing class resolver OR select the existing
  equality comparator; check the saturated application in the kernel), bound to
  the defining `GlobalId` + checked telescope, NOT glyph text.
- Factor dictionary completion so comparisons AND later membership reuse the
  SAME scoped/coherent resolver (not two operator dispatchers). Preserve
  class/head ownership, provenance, overlap/refusal, and the explicit
  named-dictionary escape. Thread a genuine local `where` dictionary as the
  given (do not resolve it as a global instance named "a").
- Standard fixities: `∧ infixr 3`, `∨ infixr 2`, `≤ ≥ ≠ infix 4`. Results Bool;
  no automatic Ω connective/equality refinement/proof-witness conversion;
  ordinary application evaluation (no invented short-circuit); single
  evaluation, left-to-right operand order (especially `≥`).
- Do NOT claim general implicit inference, operator sections, or arbitrary
  partial application are complete. `ord_leq_at Nat d` stays a valid partial
  application; an unrelated local `≤` stays an ordinary function.

## Deliverables

- **D1 — the completion adapter.** One use-site completion path in
  `crates/ken-elaborator`, keyed on the defining `GlobalId` + checked
  telescope, implementing section 6.9's ordering exactly.
- **D2 — the five standard bindings wired.** `∧ ∨ ≤` bound to the landed
  `Core.Classes.LawfulClasses` bindings (FI-2); `≥` and `≠` authored and bound.
- **D3 — the resolver factored once.** The dictionary completion that D1 uses
  is the same scoped/coherent resolver the B track will reuse for section 6.10,
  exposed so that reuse needs no second dispatcher.
- **D4 — the `NoInstance` disposition.** Either the repair, or an explicit
  split into its own node with the generic case named as NOT delivered here.

## Acceptance criteria

Each AC names the observation that discharges it. An AC whose evidence you
would have to manufacture is the one to raise, not to skip.

- **AC-1 (completion order).** For each of the five glyphs, a passing case
  shows the saturated application reaching the kernel, and a negative case at
  each of section 6.9's three steps (carrier un-inferable / dictionary
  unresolvable / application ill-typed) produces an **ordinary elaboration
  error at the occurrence**. Positive control: the error must name the
  occurrence's span, not a synthesized one.
- **AC-2 (identity, not glyph) — three pairs, each a non-degenerate pair on a
  shared input.** (a) a renamed / re-exported standard binding still completes;
  (b) an unrelated local `≤` with a different `GlobalId` gets NO completion and
  elaborates per its own declaration; (c) `ord_leq_at Nat d` stays a valid
  partial application and is not rewritten into a four-argument call. Each pair
  must FLIP if completion is keyed on glyph text instead of identity.
- **AC-3 (one resolver, not two).** Name the single resolver entry point D1
  calls and show, by reading, that no second operator-dispatch path exists.
  **The satisfying act is a structural property of the tree, not an authored
  list:** state the entry point and show every completion site reaches it.
- **AC-4 (`≥` operand order and single evaluation).** A case whose operands
  have observable evaluation shows each operand evaluated EXACTLY ONCE,
  left-to-right, with `≥` reversing already-evaluated values. Positive control:
  a mutation swapping the reversal or double-evaluating an operand must redden
  this case.
- **AC-5 (`≠` carrier inventory).** Enumerate the FULL NumericEnv carrier
  inventory measured at your own cut — including Float32 and Decimal's
  registered representation, not the Int/Float/Char vs Nat/Bool/String sample —
  and for each carrier show `≠` complementary to `==`, IEEE edges included, and
  the SAME unsupported-carrier refusal. **Report the inventory you measured and
  the SHA you measured it at.** No automatic DecEq fallback and no
  numeric-order substitute for `Ord`.
- **AC-6 (the `NoInstance` disposition, FI-3).** EITHER the generic
  `where Ord a` binding elaborates with the local `where` dictionary threaded
  as the given — with a case proving it is threaded as a given and NOT resolved
  as a global instance named "a" — OR the gap is split into its own node and
  this frame's generic case is recorded as NOT DELIVERED. **Report which.
  A silent third outcome is the failure this AC exists to catch.**
- **AC-7 (Boolean truth tables).** Complete truth tables for `∧` and `∨` at the
  stated fixities, plus a case proving no short-circuit was invented (both
  operands evaluate even when the first settles the result).
- **AC-8 (no regression).** Workspace-green **in CI**, never a local
  `--workspace` run (`COORDINATION` section 12). Enumerate the globbed catalog
  oracles your change reaches and name each in your handoff — a catalog-touching
  change trips oracles in crates this WP never edits, and they surface as red CI
  at publish, which is the most expensive place to find them.

## Not this node

- Membership's standard binding / `class Membership` and section 6.10's
  carrier-first rule — the parallel B track
  ([[LANG-MEMBERSHIP-OPERATOR-SURFACE]]), which reuses this node's resolver
  when it lands. FI-4.
- The name/fixity admission (A0) and the spec contracts.

## Sizing / tier

**Size L, tier T1.** Soundness-adjacent completion machinery plus catalog
design; the review turns on the resolver-factoring, the FI-2 placement call,
and the `NoInstance` repair/split. Architect required reviewer.

## Contention

Language ring: `crates/ken-elaborator/src` + tests, AND
`catalog/packages/Core/Classes/LawfulClasses.ken.md` (not a new package — FI-2).
A1 is NOT parser-only; enumerate the globbed catalog consumers at the cut.
CODE merge -> full CI, M8/M8a Adversary. Re-measure the NumericEnv carrier
inventory and the class-resolver anchors at the cut.

**`LawfulClasses.ken.md` is actively contended** — the foundation ring has been
landing in it (for example `CAT-LAWFULFUNCTORS-STANDALONE-IMPORT`, migrating
`list_append` attached proofs to `Derived`). The foundation seats are currently
walled on their provider quota, so the file is quiet right now, but **re-check
contention at your cut rather than inheriting this sentence.**
