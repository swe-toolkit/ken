---
id: LANG-STANDARD-INFIX-CALL-COMPLETION
title: "A1 of the reserved-infix-glyph objective: the reusable use-site standard-call completion adapter that spec 39 section 6.9 specifies and that ships un-implemented, co-landing its first standard consumers ∧ ∨ ≤ ≥ ≠ bound to ordinary checked functions; ∧ ∨ ≤ bind to the ALREADY-LANDED `Core.Classes.LawfulClasses` identities, reached through a separate standard-operator facade that RE-EXPORTS them rather than defining second copies, because 6.9 keys completion on ONE defining GlobalId and 33 section 6.1 requires exactly that home; ord_geq_at and ≠ are the genuinely new bindings; the completion resolver is factored so later membership (6.10) reuses the SAME scoped/coherent dictionary resolver, not a second operator dispatcher; results Bool, single left-to-right eval, no short-circuit; the definition-time NoInstance gap for a generic `where Ord a` binding is repaired or explicitly split before the generic case is claimed delivered"
status: ready
owner: language
size: L
gate: none
depends_on: [LANG-RESERVED-INFIX-NAMES, SPEC-STANDARD-INFIX-BINDING]
blocks: [LANG-MEMBERSHIP-OPERATOR-SURFACE]
github: null
tier: T1
origin: "Steward cut 2026-09-13 from the Architect final-A decomposition (evt_784ge2nq65dfy), grounded at main 4fdd4f0ad. A1: the reusable completion link, made its own node per the Architect (do not ship an unused framework nor a third wiring node -- co-land ∧ ∨ ≤ ≥ ≠). RELEASED 2026-09-17 by the Steward, re-grounded at main 8f0f3753270b2d7639fc7aed457b759e6e6a57a1: both prerequisites are merged, and the re-grounding refuted one of the stub's own scope lines (see the RELEASED banner). AMENDED same day with FI-2a after language-implementer attacked FI-2 as instructed (evt_70jn1q9r56ttr): FI-2 refuted second copies correctly but over-refuted the home, and spec 33 section 6.1 requires a re-exporting standard-operator module and fixes ≥'s binding as ord_geq_at. Architect then upheld the facade and overturned the Steward on ord_geq_at's home (evt_4jhrgeqse8k13, FI-2b): define it in LawfulClasses beside ord_leq_at, re-exported by the facade, because it is fully generic and section 2a puts only package-specific content in a package; the same ruling corrected the frame's stale 'actively contended' claim about LawfulClasses.ken.md to UNCONTENDED. IN-LANE: bounded elaborator + catalog design; Architect required reviewer; no new TCB (Architect: no new Eq/DecEq instance, Float-equality law, or TCB entry needed)."
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
>
> ## AMENDED 2026-09-17 — FI-2a, and the instruction above working as intended.
>
> **The implementer attacked FI-2 first, as asked, and FI-2 was incomplete.**
> It refuted *second copies* (correct, unchanged) but as written it read as
> refuting *any new module*, and `33 section 6.1` positively requires a
> **re-exporting standard-operator home**. `39 section 6.9` was the wrong
> authority to settle placement from on its own; `33 section 6.1` is the
> authority for the names and the home, and it also FIXES `≥`'s binding as
> `ord_geq_at`. **FI-2a is the amendment; D2, the scope boundaries, AC-2,
> AC-7 and the contention section were re-cut to match** — the Steward's error
> is corrected in the operative text rather than noted beside it. See FI-2a.
>
> **The Architect then UPHELD the facade and OVERTURNED the Steward on
> `ord_geq_at`'s home** (`evt_4jhrgeqse8k13`): it is defined in `LawfulClasses`
> beside `ord_leq_at` and re-exported by the facade, because it is fully generic
> and `section 2a` puts only what is SPECIFIC to a package in it. **FI-2b is the
> operative text on placement.** It also corrects this frame's earlier
> "actively contended" claim about `LawfulClasses.ken.md` — measured
> UNCONTENDED, twice, independently. See FI-2b and the Contention section.
>
> **AMENDED AGAIN — FI-5 rules how the elaborator learns the five identities**
> (`evt_51m59k0zh8z5t`), discharging a `section 1a` hold taken on a research
> prior-art advisory. **Three layers: role vocabulary, acquisition from the
> facade's existing export table, and a required-roles check WITH A SHAPE
> CONTRACT.** The implementer's stop was correct — a bare runtime string
> comparison fails open — and the advisory's finding is that **acquisition
> without a contract lands in the silent tier no matter how few strings it
> names**, so fewer paths was never the fix. D1 is now D1a/D1b/D1c and AC-9
> discharges the hardness layer. **The fixity half is UNBLOCKED.**

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

**FI-2. THE STUB'S "new catalog package" MAY NOT DEFINE SECOND COPIES OF
`∧ ∨ ≤`, AND THIS CHANGES THE DELIVERABLE.**
Three of the five standard bindings already exist, are already `pub`, and
already carry the exact telescope section 6.9 keys on:

    pub fn ord_leq_at (a : Type) (d : Ord a) (x : a) (y : a) : Bool = d.leq x y
    pub fn bool_or    (a : Bool) (b : Bool) : Bool
    pub fn bool_and   (a : Bool) (b : Bool) : Bool

all in `catalog/packages/Core/Classes/LawfulClasses.ken.md`, alongside
`pub class Ord a` and `pub class DecEq a`. `catalog/packages/Core/` holds
exactly `Classes` and `Logic`; there is no `Operators` package.

⇒ **Authoring a second copy in a new package would mint a SECOND
`GlobalId` for a meaning that already has a canonical one, and section
6.9's entire policy is that completion binds to THE defining `GlobalId`
and its checked telescope, never to glyph text.** Two homes for one
meaning is not untidiness here; it is a direct contradiction of the rule
this node exists to implement, and it would make section 6.9's own stated
consequence — *"a renamed standard binding still completes"* —
ill-defined.

**So: `∧ ∨ ≤` bind to the landed `Core.Classes.LawfulClasses` identities.** The
genuinely new bindings are **`≥`** (reverses already-evaluated VALUES) and
**`≠`** (negation of the exact NumericEnv `==` comparator on the same
supported carriers).

**FI-2a. THE HOME IS A SEPARATE RE-EXPORTING FACADE MODULE, AND `33 section
6.1` REQUIRES IT. This amends FI-2 as first released** (`language-implementer`,
`evt_70jn1q9r56ttr`, which attacked FI-2 as instructed and found it
incomplete).

FI-2 refuted *second copies*, and that holds. As first written it refuted
*the package*, which reads as **"no home at all"** — and `33 section 6.1`
positively requires one:

> *"`bool_and`, `bool_or` and `ord_leq_at` **already exist** as ordinary
> public functions in the standard package. The standard-operator home
> **re-exports** them; it does not define second copies."*

By `33 section 4.3` an `export` republishes the existing `GlobalId` and never
mints another, so a re-exporting home is **not** the thing FI-2 rules out. The
two claims are different and only one of them is true.

**The home is a separate module, not an attachment inside `LawfulClasses`**, on
four grounds, none of them aesthetic:

1. **Section 6.1's verb settles it.** A module cannot re-export what it defines
   — `4.3` states that for a locally defined name `export foo` has the same
   interface effect as `pub foo`, no second identity. The three are already
   `pub` in `LawfulClasses`, so `LawfulClasses` cannot be the re-exporting home.
2. **`section 6.3` puts `∈` in the same home** — `membership_member_at` is an
   ordinary top-level binding *"exactly as section 6.1's other four are"* — and
   its provider is `50-stdlib/58b`'s `Membership`, not a lawful class. A
   `LawfulClasses` home cannot host the B track without dragging `Membership`
   into the classes module.
3. **Fixity travels with identity** (`33 section 6`), so declaring the five
   fixities at the facade is correct and needs no access to the defining module.
4. **In-tree precedent for exactly this shape:**
   `catalog/packages/Data/Numeric/Nat/Order.ken.md` is a facade that
   re-exports `Ord`, `IsTrue` and `bool_or` without redeclaring them.

⇒ **The stub's `Core.Operators` was right in SHAPE and wrong only in what it
put there.** Author it as a facade: `export` the three landed identities,
declare the five fixities, and define only what is SPECIFIC to it (FI-2b: not
`ord_geq_at`, which is generic).

**`≥`'s binding is `ord_geq_at (a : Type) (d : Ord a) (x : a) (y : a) :
Bool`** — the name and telescope are FIXED by `33 section 6.1`, not yours
to choose. It is *"derived over `leq`, not a class field"*: `class Ord a`
declares `leq, refl, antisym, trans, total` and no `geq` member is added.
It reverses its two **already-evaluated argument values inside its own
body** and never reverses the operand expressions at the use site.

**FI-2b. `ord_geq_at` IS DEFINED IN `LawfulClasses`, NOT IN THE FACADE.
ARCHITECT RULING, `evt_4jhrgeqse8k13` — it OVERTURNS the Steward's
recommendation and is the operative text.** The facade `export`s it along with
the other three. The facade shape itself is UPHELD.

The spec fixes `ord_geq_at`'s name and telescope and is silent on its defining
module, so this was a factoring call, and the factoring rule is `section 2a` —
in force on this node (operator, 2026-08-22), with Architect review as its
design backstop.

**The precedent read whole answers it.** `Data/Numeric/Nat/Order.ken.md`
re-exports the GENERIC (`Ord`, `IsTrue`, `bool_or`, `leq_nat`) and defines the
SPECIFIC (`min`, `max`, `sub`, `compare` — everything keyed on `Nat`). A facade
defining things is fine; that one defines five. What it never does is define a
generic. **`ord_geq_at` is fully generic over any `Ord a` — exactly as generic
as `ord_leq_at`, which lives in `LawfulClasses`.** The facade's specific content
is the glyph surface and the five fixity declarations, and an `Ord`-derived
operation is not specific to an operator facade.

**Two Steward errors this ruling corrects, recorded because inheriting either
would be worse than the misplacement:**

- **The contention reason was measurably FALSE**, and the refuting measurement
  was in the very post the recommendation replied to. `LawfulClasses.ken.md`:
  last landed touch `7663ad9b9` (2026-09-13); `wp/CAT-CAPEX` and
  `wp/CAT-CC-ORACLE-BEHAVIORALIZE` touch it zero times; both foundation seat
  branches are not ahead of main. **Uncontended.** And even if it had been
  contended, contention is a SCHEDULING fact and this is a FACTORING question —
  `section 2a` exists because soundness gates do not check factoring, and a
  contention gate does not get to decide it either.
- **"Cheap to move" was wrong about what moves.** The line is cheap; the
  IDENTITY is not. By `33 section 4.3` identity is owned by the **defined-at**
  module, so this choice permanently assigns `ord_geq_at`'s canonical owner, and
  relocating it later re-homes a `GlobalId` — the exact property FI-2 exists to
  protect — after consumers exist. They will: `ord_leq_at` already carries 376
  uses in `PriorityQueue`, 66 in `InsertionSort`, 16 in `OrderedSearch`.

⇒ **Put it where it still belongs in a year: beside `ord_leq_at`, in the module
that owns the `Ord` vocabulary.** A reader opening `LawfulClasses` for the `Ord`
surface must not find `ord_leq_at` with no `ord_geq_at` and nothing pointing
onward.

**The Architect ruled placement and factoring ONLY.** `section 6.1`'s telescopes
against the elaborator seam, and `ord_geq_at`'s body (reversing two
already-evaluated argument values rather than the operand expressions), are
correctness questions he takes at candidate review.

**`section 2a(b)` binds the facade, flagged now rather than at review:** lead
with the headline — the operator surface and its fixities — and descend to
plumbing, most-fundamental last. A module arranged bottom-up is a block at
review.

**FI-3. The definition-time `NoInstance` gap is real and located.**
`crates/ken-elaborator/src/prelude.rs` states in its module header that
`instance_search` runs **before the body elaborates**, emitting
`NoInstance`; the construction sites are in `crates/ken-elaborator/src/elab.rs`
and the variant is `ElabError::NoInstance { class, ty, span }` in
`crates/ken-elaborator/src/error.rs`. A generic `where Ord a` binding
therefore fails at definition time against the abstract tyvar `a`. **Repair
it or split it explicitly — see AC-6. Do not claim the generic case
delivered either way without discharging AC-6.**

**FI-4. `membership_member_at` is NOT a catalog binding today.** It appears
only as a test fixture string in
`crates/ken-elaborator/tests/lang_type_projection_surface_form.rs`. That is
correct and expected: it is the B track. Do not author it here.

**FI-5. HOW THE ELABORATOR LEARNS THE FIVE IDENTITIES IS RULED — THREE LAYERS,
AND THE HARDNESS LAYER IS THE ONE THAT WAS MISSING.** Architect
`evt_51m59k0zh8z5t`, discharging a `section 1a` hold taken on a research
prior-art advisory (`evt_34sa9cvyags83`). See D1a/D1b/D1c.

The implementer's stop was correct and the fork it raised was real: **a bare
runtime string comparison FAILS OPEN.** The advisory's finding is that
acquisition without a contract lands in the silent tier **no matter how few
strings it names** — so fewer paths was never the fix.

    layer 1  ROLE VOCABULARY   compiler-owned, closed, names no paths
    layer 2  ACQUISITION       the facade's existing `export` table
    layer 3  HARDNESS          required-roles check + SHAPE contract

**The shape contract is the load-bearing part: a role is not filled by whatever
happens to sit under the glyph, it is filled by a binding of the required
form.** Presence-checking alone would accept a wrong-but-present binding.

**The residual coupling is stated, not buried: the elaborator holds ONE string,
the standard-operator home's module path.** That is this design's minimum.
Under layer 3 it fails **closed and loud** — a moved home is a hard error
naming every unfilled role, never a silent completion miss.

**A self-declaring home would remove even that string. It is a DEFERRED
improvement and explicitly NOT a blocker — do not build it here.** It moves
where one string lives without changing any of the three layers, so it can land
later without re-opening this ruling.

**Fixity falls out and needs no widening.** Once layer 3 certifies the five
roles, `section 6.1`'s fixities attach to the `GlobalId`s just certified. No
locality gate (nothing is declared at a module), and **no first-writer race
over an identity's fixity, because there is exactly one home.**
`collect_scope_fixities`' `GlobalId` keying carries it unchanged.

## Scope boundaries

- Standard bindings are ordinary checked functions reached by ordinary imports:
  `∧`/`∨` = the landed `bool_and`/`bool_or`; `≤` = the landed `ord_leq_at`
  (type + actual `Ord` dictionary); `≥` = `ord_geq_at`, reversing
  already-evaluated VALUES; `≠` = negation of the exact NumericEnv `==`
  comparator on the SAME supported carriers. They are reached through a
  separate re-exporting facade module. See FI-2 and FI-2a for where each lives
  and which module defines it.
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

- **D1 — the completion adapter, in THREE LAYERS (FI-5, Architect ruling).**
  One use-site completion path in `crates/ken-elaborator`, keyed on the
  defining `GlobalId` + checked telescope, implementing section 6.9's ordering
  exactly — built as:
  - **D1a — role vocabulary.** A compiler-owned, closed enumeration of the
    standard operator ROLES (`∧ ∨ ≤ ≥ ≠`, and `∈` when the B track lands). It
    names roles only: **never a module path, never a qualified identifier.**
    This is `31 section 1c`'s admission made available to elaboration —
    *"admission fixes names, not meanings"* — so it concedes nothing new to
    the compiler.
  - **D1b — acquisition, from the facade's EXISTING export table. No new
    surface form.** `export Core.Classes.LawfulClasses (bool_and as ∧, …)`
    already states glyph-to-identity, in Ken, at the home `33 section 6.1`
    designates, and by `4.3` it republishes the existing `GlobalId`. **The
    acquisition statement exists; do not invent one.** The elaborator resolves
    the home and reads its export table.
  - **D1c — a separate REQUIRED-ROLES check, with a SHAPE CONTRACT, at a
    defined point in elaboration.** Hardness does not come from making the
    lookup total; it comes from an independently maintained statement of what
    must be present. **Role absent from the export table: hard error naming the
    role. Role present but the binding has the WRONG SHAPE: hard error naming
    the role and the mismatch.** `section 6.1` supplies both inputs verbatim —
    the role list and each role's signature. A catalog rename is a non-event
    (the glyph binding travels with the `export`); a wrong-but-present binding
    is caught at the moment of binding, which is the case a bare string
    comparison cannot see.
- **D2 — the five standard bindings wired, through a re-exporting facade.**
  `∧ ∨ ≤` bound to the landed `Core.Classes.LawfulClasses` **identities**,
  reached via `export` from a separate standard-operator module that does NOT
  redeclare them (FI-2, FI-2a); `ord_geq_at` defined in `LawfulClasses`
  beside `ord_leq_at` and re-exported by the facade (FI-2b, Architect ruling);
  `≠` authored; each defined exactly once; the five fixities declared at the
  facade, which is arranged headline-first per `section 2a(b)`. The facade is the
  surface the B track's `∈` attaches to.
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
  **FI-2a makes (a) degenerate unless you take care.** The facade's own
  re-export is now the production path, so "a re-exported binding completes" is
  satisfied by the delivery itself and controls nothing. (a) needs a witness
  the delivery does not already produce: a **renaming** hop, or a second
  re-export path, reaching the same identity under a different surface name and
  completing identically. Separately, show the facade **republishes** rather
  than mints — the identity behind the facade path and the identity behind
  `Core.Classes.LawfulClasses`'s own path are the SAME `GlobalId`, which is the
  property `33 section 4.3` promises and the one FI-2 exists to protect.
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
  operands evaluate even when the first settles the result). **The case is
  about OPERANDS and `18a section 5.4` is about the body's ARMS — `33 section
  6.1` names the conflation explicitly.** `bool_and`'s body matches on its first
  argument and forces one arm; under call-by-value both operands are already
  evaluated before the body runs. An answer that cites arm laziness to explain
  operand behaviour has answered a different question, and one that reports
  no-short-circuit is CORRECT rather than a defect to fix.
- **AC-9 (the hardness layer FAILS CLOSED — D1c, FI-5).** Three observations,
  and **all three require evidence you must manufacture, which by this frame's
  own standard makes them the ones to raise rather than skip.**
  (a) **Role absent:** remove or rename a role out of the facade's export table
  and show a HARD ERROR naming the role — not a silent completion miss, not a
  fallback to glyph text. (b) **Role present, WRONG SHAPE:** bind a role to a
  binding of the wrong telescope and show a hard error naming the role AND the
  mismatch. **This is the case a presence-only check accepts, and it is why the
  contract is on shape.** (c) **Home moved:** relocate the standard-operator
  home and show every unfilled role named. Positive control for all three: a
  build that is green before the perturbation and red after, with the role name
  in the message — **a red build alone does not discharge this; the diagnostic
  must name the role.**
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
- **A self-declaring standard-operator home** — the surface form that would let
  the facade announce itself and remove the elaborator's one remaining module
  path. Architect-deferred and explicitly not a blocker (FI-5): it relocates a
  string without touching any of D1's three layers, so it lands later without
  re-opening the ruling. **Do not build it here, and do not treat its absence
  as a gap in D1c** — under the hardness layer the residual string already
  fails closed and loud.

## Sizing / tier

**Size L, tier T1.** Soundness-adjacent completion machinery plus catalog
design; the review turns on the resolver-factoring, the FI-2a/FI-2b placement call,
and the `NoInstance` repair/split. Architect required reviewer.

## Contention

Language ring: `crates/ken-elaborator/src` + tests, AND the new
standard-operator facade module under `catalog/packages/Core/` (FI-2a), AND
`catalog/packages/Core/Classes/LawfulClasses.ken.md`, which gains
`ord_geq_at`'s definition per FI-2b.
A1 is NOT parser-only; enumerate the globbed catalog consumers at the cut.
CODE merge -> full CI, M8/M8a Adversary. Re-measure the NumericEnv carrier
inventory and the class-resolver anchors at the cut.

**`LawfulClasses.ken.md` measured UNCONTENDED at `775c9823b`** — by
`language-implementer` and independently re-derived by the Architect:

    last landed touch               7663ad9b9  2026-09-13  CAT-PRIORITY-QUEUE-LAWS
    wp/CAT-CAPEX                    ahead 2, LawfulClasses touches = 0
    wp/CAT-CC-ORACLE-BEHAVIORALIZE  ahead 3, LawfulClasses touches = 0
    foundation-implementer/work     NOT ahead of main
    foundation-leader/work          NOT ahead of main

**This replaces an earlier "actively contended" sentence in this frame, which
was wrong.** Two probes to know apart: `git diff origin/main...<branch>` answers
a DIFFERENT question — a squash-merged branch still shows its own old changes
against the merge-base, which reported 32 spurious hits. Sweep only branches
genuinely ahead of `main` and held by a worktree. **Re-measure at your cut
rather than inheriting either sentence.**
