---
id: LANG-MATCH-LITERAL-PATTERN
title: "literal patterns -- slice 6 (final) of 34 §3's six absent pattern forms: a VALUE-LEVEL branch-selection form (34 §3.2:385-420), the comparator returning Bool with NO Equal proof and NO equality hypothesis. RECUT under Architect HS1 (evt_1wc0m1xbtk4r) to a CONTAINED CORE: direct Int/Float/Float32/Char rows + a literal-shaped String structural plan (string_to_list_char + eqChar) + fixed-width and Bytes via the now-LANDED lossless view composition (option 1, SPEC-MATCH-LITERAL-COMPARATOR-REALIZATION merged ceea742831f2), everything else FAIL-CLOSED. Zero new TCB: no hidden primitive, no new PrimReduction::Op, no new trusted-base entry. A sealed LiteralComparatorPlan (derived from the expected carrier after elab_num_lit_checked) selects the comparison; unknown carrier/literal pairs have no catch-all success arm and reject. Decimal is deferred (no unrestricted Decimal literal pattern is total under the present comparator)"
status: ready
owner: language
size: M
gate: none
depends_on: [SPEC-MATCH-PATTERN-PINS, SPEC-MATCH-LITERAL-COMPARATOR-REALIZATION]
blocks: []
github: null
origin: "Steward cut 2026-09-12 from the umbrella LANG-MATCH-PATTERN-FORMS-ABSENT on the spec-enclave disposition evt_12qrtnp7237dn (the prerequisite-ordered six-slice cut) and the Architect L2 decomposition ruling evt_127n516pkvtnb. Slices 1-5 landed: as-patterns (LANG-MATCH-AS-PATTERN e6645d7c2), tuple/pair (LANG-MATCH-TUPLE-PATTERN af2b36dc8), record (LANG-MATCH-RECORD-PATTERN a42b90454), or-patterns (LANG-MATCH-OR-PATTERN 7f80228c2), guards (LANG-MATCH-GUARDS, merged). This is slice 6, the final form. RECUT 2026-09-12 under Architect HS1 evt_1wc0m1xbtk4r (classification PINNED_VALUE_COMPARATOR_HAS_NO_TOTAL_CORE_REALIZATION, re-derived at exact 2d35fd3b6): the earlier all-row frame assumed every 34 §3.2 row had a total kernel-checkable Bool comparator; it does not. See the READ-FIRST banner. Anchors measured by the Steward at framing (PatKind ast.rs:178; top-level Wild/Var refusal elab.rs 15772/15859/15999/16133; comparators eq_int/eq_float/eq_float32 numbers.rs, eqChar/decimalEq decimal_char.rs; lossless views conversions.rs:143-166; string_to_list_char/bytes_to_list) -- RE-MEASURE at the cut. RE-RELEASED 2026-09-12 (status draft->ready) after SPEC-MATCH-LITERAL-COMPARATOR-REALIZATION landed at ceea742831f2 (option 1: internal lossless view composition is an admitted realization; conformance split 14 supported-now + Decimal deferred), which promotes fixed-width and Bytes from conditional into this core. New base current main e34597a5a; the 2d35fd3b6 anchors above are superseded -- re-measure every anchor at the implementer's cut."
---

> # RE-RELEASED 2026-09-12 (Steward): gating spec correction landed. READ FIRST.
>
> **The gate is cleared: [[SPEC-MATCH-LITERAL-COMPARATOR-REALIZATION]] merged at
> `ceea742831f2` (closeout `e34597a5a`), choosing OPTION 1 -- exact, total,
> already-landed lossless view composition is an admitted INTERNAL comparator
> realization, the conformance row is split 14 supported-now + Decimal deferred.
> This node flips `draft -> ready` and is released to the language ring on new
> base current main `e34597a5a`.** The Architect HS1 that produced this recut
> (`evt_1wc0m1xbtk4r`, `PINNED_VALUE_COMPARATOR_HAS_NO_TOTAL_CORE_REALIZATION`)
> is now resolved by that landing: spec 34 §3.2 pins each comparison RESULT, the
> prelude has no total `Bool` core realization for every row directly, and the
> spec now admits realizing fixed-width/Bytes/String through their exact total
> views -- so this core covers them with ZERO new TCB.
>
> **What the landing changed for this node:** fixed-width and Bytes move from
> conditional into the core (their `<snake>_to_int` / `bytes_to_list` + octet
> views are the admitted realization). Direct Int/Float/Float32/Char and the
> String structural plan are unchanged. Decimal stays deferred (the landed spec
> marks it carrier-wide deferred; no total comparator yet). The old "all-row ACs
> vs no-TCB AC cannot both hold" tension is gone: option 1 makes every
> supported row realizable without a new primitive.
>
> **ZERO new TCB is authorized. There are no intended hidden names to supply.**
> A value-only comparator primitive is safer than a proof-producing one, but it
> is still trusted surface: `declare_primitive` installs a trusted declaration
> before `ElabEnv::empty()` captures `prelude_env.native_trusted_base`
> (`lib.rs:240-267`), and it would need matching interpreter + native reduction.
> This WP authorizes exactly zero new `PrimReduction::Op` symbols and zero new
> trusted-base entries; adding one after the pre-source capture is not an escape,
> it falls outside the admitted native roster.
>
> **The recut is a bounded three-part decomposition (Architect):**
> 1. A spec 34 §3.2 realization correction, owned by the enclave
>    ([[SPEC-MATCH-LITERAL-COMPARATOR-REALIZATION]]) -- decide whether exact,
>    total, already-landed lossless view composition is an admitted internal
>    comparator realization, split the compound conformance row per carrier,
>    defer Decimal. **This node BLOCKS on it.**
> 2. THIS node: the contained Language core below.
> 3. Decimal -- a separate prerequisite/consumer pair AFTER a total Decimal
>    comparator or a bounded-scrutinee contract is selected (18a §5.6.1(2)); the
>    owning team is NOT pre-assigned from current evidence. Not in this node.
>
> **The Architect is the REQUIRED reviewer** alongside language-qa + CI: the
> value-level-not-proof boundary, the sealed `LiteralComparatorPlan` mechanism,
> and fail-closed carrier rejection are a correctness argument, not a mechanical
> edit. Tier T1. Standing Adversary hunt independent -> Steward M1-M4 ->
> lieutenant. **Now released: cut a fresh branch from current main
> `e34597a5a` and re-measure every anchor at that cut (five prior slices moved
> the elab.rs match spine, the PatKind/RPatKind enums, and the top-level refusal
> repeatedly, and the landed spec correction touched `34`/`18a`).**

## What this is

The **literal** form -- slice 6, the final one, of the six absent forms in
`spec/30-surface/34-data-match.md §3` (form list `§3.1` bullet 3; the comparator
contract at `§3.2:385-420`). A literal pattern is matched by a **value-level
branch-selection comparator**, not a carrier-bearing or proof-bearing pattern.

Two properties are unchanged from the original cut and survive HS1 (they are the
Architect L2 decomposition ruling `evt_127n516pkvtnb`):

1. **Expected-type-then-comparator.** A literal pattern always has the
   **scrutinee's type** as its expected type. The literal is first checked at
   that expected type by `35 §4` (`elab_num_lit_checked`), and comparator
   selection uses the **resulting carrier**. Numeric defaulting does not choose
   the comparator. A literal form the expected carrier does not admit is a
   **type error**.
2. **Value-level, not proof-level.** The comparator returns a `Bool`; it
   **constructs no `Equal` proof and adds no equality hypothesis** to the branch.
   A lawful `DecEq` certificate *may* license a comparator but is **not required**
   where normative value semantics fixes a total comparison. This is why
   `Float`/`Float32` participate at all: their value comparators are deliberately
   **not** lawful `DecEq` operations.

**What HS1 changed:** the frame may not assume a total comparator for every 34
§3.2 row. This node's core is the sub-population with a zero-new-TCB total
realization; the rest is fail-closed and deferred.

## The comparator population this core implements

Direct total comparators, already available to the match compiler (zero-TCB,
`numbers.rs:557-560`, `decimal_char.rs:195-211,266-268`):

| expected carrier | comparator | note |
|---|---|---|
| `Int` | `eq_int` | exact integer value equality + registered `Int` certificate |
| `Float` | `eq_float` | IEEE-754 binary64 `==` (NaN unequal to all, +0.0==-0.0); not proof equality |
| `Float32` | `eq_float32` | IEEE-754 binary32 `==`, same boundary; not proof equality |
| character | `eqChar` | Unicode-scalar equality via the `Int` projection; independent of lawful `DecEq Char` |

Literal-shaped **String** structural plan (zero-TCB; no new global `String.eq`,
no catalog import, no primitive): lower the tested scrutinee through
`string_to_list_char`, then emit an N+1-deep `List` eliminator for a String
literal of N codepoints -- at each `Cons` test the head with `eqChar`, after N
successes require `Nil`. This is finite source-literal unrolling.

**Licensed by the landed correction (option 1), IN this core**
[[SPEC-MATCH-LITERAL-COMPARATOR-REALIZATION]]: fixed-width (apply the
already-landed lossless `<snake>_to_int` view to both operands, then `eq_int`)
and Bytes (`bytes_to_list`, octets compared through the existing `UInt8` view).
The merged spec admits exact, total, already-landed lossless view composition as
an internal realization, so these are supported-now rows here -- not conditional,
not split off. Zero new TCB: the views and `eq_int` are already trusted surface.

**Deferred, NOT in this node:** `Decimal`. No unrestricted Decimal literal
pattern is total under `decimalEq` -- the scrutinee exponent is unbounded, and
under CBV `decimalPow10Unbounded` is demanded before `eq_int` even for a zero
coefficient, so `MkDecimalPair c (e_lit + 31)` is stuck for every `e_lit`. The
only sound present static admission predicate is `false`. Decimal waits for a
total comparator or a bounded-scrutinee contract (18a §5.6.1(2)); see the READ-
FIRST banner part 3.

## The comparator-plan mechanism (Architect-directed)

Do **not** extend `NumericEnv::eq_table` with synthesized pseudo-operations --
that table selects actual binary operator identities. Add a separate **sealed
`LiteralComparatorPlan`**, derived only from the expected carrier after
`elab_num_lit_checked`, with explicit plan variants for the direct ops and the
literal-shaped structural comparison. The plan yields only a `Bool` test; it
never constructs `Equal`, mutates the branch context, or copies a catalog
dictionary into the prelude. **Unknown carrier/literal pairs have no catch-all
success arm and reject.**

## The composition discriminator this slice owes

Every **inner** pattern form must add a composition discriminator, because
literals compose under constructors (`MkWrap 3`) and inside the landed forms:

- A **redundant literal arm** -- the same literal value after an earlier
  identical literal, or a literal already covered by an earlier arm -- must get
  the **subsumption** dead-cause, **never `NoInhabitants`**. A literal arm over
  an inhabited carrier is dead by *coverage*, not by emptiness. Do **not** widen
  `Subsumed` to carry an empty winner set (the shape
  `LANG-REACHABILITY-SUBSUMING-ARMS` forbids). Duplicate/subsumed accounting
  uses **comparator semantics** -- Float signed zero, String canonical
  codepoints -- **not Rust spelling equality**.
- A finite set of literal arms over a carrier with more values than are listed
  is **non-exhaustive**, reported honestly (missing cases), not papered over.
  The trailing catch-all is out of scope (see "Not this slice").

The matrix compiles a literal column as **ordered value-test branches plus an
unguarded residual fallback, not as an inductive constructor split**.

## Deliverables

1. A literal `PatKind` (AST) + parser production for the literal forms this core
   admits, threaded through `RPatKind` (`resolve.rs:66`) into the match spine.
2. The sealed `LiteralComparatorPlan` + elaborator selection: expected-type
   check at the scrutinee type (`35 §4`), carrier-driven plan, value-level
   `Bool` result with no `Equal`/hypothesis emission. Direct Int/Float/Float32/
   Char rows + the String structural plan + fixed-width and Bytes through the
   landed lossless views (option 1, [[SPEC-MATCH-LITERAL-COMPARATOR-REALIZATION]]).
3. Fail-closed rejection for every carrier/literal pair not in the implemented
   population -- Decimal, unadmitted carriers, and user-defined-carrier numeric
   literals (`35 §4.2` unstaged) -- with a diagnostic naming the carrier and the
   unsupported row. No silent accept, no non-selecting match.
4. The composition discriminator: a redundant literal arm routes to the
   subsumption cause (by comparator semantics); a partially-covered literal set
   stays reported as non-exhaustive.
5. Tests: a reaching positive per supported row (at minimum `Int`, `Float`,
   `Char`, `String`, a fixed-width carrier, and `Bytes`); a fail-closed control for
   Decimal AND for an unadmitted/user carrier; a redundant-literal-arm control
   asserting the subsumption cause not `NoInhabitants`; a control keeping the
   five landed slices green.

## Acceptance criteria

- Each implemented row compiles and selects by value (a genuine differential,
  not vacuous green-vs-green): the matching literal takes its arm, a non-matching
  one falls through.
- The `Float` rows match by their value comparator with **no** `Equal` proof or
  equality hypothesis introduced into the branch (assert the branch context is
  unrefined by the match).
- Decimal, an unadmitted carrier, and a user-defined-carrier numeric literal
  **reject** with a type error (fail-closed), not a silent accept or a
  non-selecting match.
- A redundant literal arm reports the **subsumption** dead-cause;
  `NoInhabitants` is never emitted for a coverage-dead literal arm.
- The five landed slices (as/tuple/record/or/guards) stay green; top-level
  `_`/`Var` refusal is **unchanged** (this slice does not lift it).
- **No kernel/TCB/spec change in this node** (the spec change is
  [[SPEC-MATCH-LITERAL-COMPARATOR-REALIZATION]], a separate WP). Zero new
  `PrimReduction::Op`, zero new trusted-base entries. **Architect required
  reviewer** + language-qa + standing Adversary hunt.

## Not this slice

- **Direct per-carrier comparators for fixed-width/Bytes** -- option 1 landed,
  so these rows are realized through the lossless views (in this core), not by a
  new comparator primitive. A direct comparator would be new TCB and is out.
- **Decimal** -- deferred (18a §5.6.1(2)); the owning team is not pre-assigned.
- **Top-level `_`/`Var` catch-all and its `ArmDeadCause` obligations.** This
  slice keeps the top-level refusal (`elab.rs` 15772/15859/15999/16133) intact.
- **The `35 §4.2` user-type literal mechanism** -- staged separately;
  fail-closed here.
- **[[LANG-CONVOY-ENCLOSING-FIELD]]** (nested-match re-typing at `34 §3.2`).
- **[[LANG-DECEQ-CHAR-LAWFUL-INSTANCES]]** (closed already-satisfied): the lawful
  instances exist but are independent of these value comparators.

## Symptom inventory

Append one entry per Architect hard stop; never rewrite history.

1. P5 assigns fixed-width, String, Bytes, and Decimal comparison outcomes, but
   the current prelude has no total `Bool` core realization for the whole
   population: fixed-width and byte comparison have only the explicitly named
   widening views, String has only its structural view, and `decimalEq` remains
   stuck beyond the bounded exponent-alignment cascade -- keyed on a normative
   comparator result without a complete implementation floor.

## Contention

One serial language seat; this shares `ast.rs`, `parser.rs`, `resolve.rs`, and
the `elab.rs` match spine with the five landed slices. Base = `e34597a5a`;
re-measure every anchor at the cut (five prior slices advanced the elab.rs match
spine, the PatKind/RPatKind enums, arm_used/NoInhabitants/top-level refusal
repeatedly). No cross-lane contention (L1 runtime is on `crates/ken-lowering`).

## Sizing / tier

**Size M, tier T1.** The diff is moderate (a PatKind + parser + the sealed
`LiteralComparatorPlan` + two exhaustiveness/reachability interactions), but the
review turns on an argument: the value-level-not-proof boundary, the zero-TCB
sealed-plan mechanism, comparator-semantics duplicate accounting, and fail-closed
carrier rejection. Not a mechanical edit.
