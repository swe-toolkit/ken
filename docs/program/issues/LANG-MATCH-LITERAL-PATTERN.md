---
id: LANG-MATCH-LITERAL-PATTERN
title: "literal patterns -- slice 6 (final) of 34 §3's six absent pattern forms: a VALUE-LEVEL branch-selection form, not a proof-carrying one (34 §3.2:385-420). A literal pattern always takes the scrutinee's type as its expected type (checked there by 35 §4), and comparator selection uses the resulting carrier via the 34 §3.2 table (Int->eq_int, fixed-width->exact eq, Float->eq_float, Float32->eq_float32, Decimal->decimalEq, String->codepoint-wise eq, Char->eqChar, bytes->byte-sequence eq); the comparator returns Bool and constructs NO Equal proof and adds NO equality hypothesis. Lawful DecEq may license a comparator but is not required where normative value semantics fixes a total comparison. Fail-closed: a literal the expected carrier does not admit, or a numeric literal at a user-defined carrier (35 §4.2 unstaged), is a TYPE ERROR, never a silent accept"
status: ready
owner: language
size: M
gate: none
depends_on: [SPEC-MATCH-PATTERN-PINS]
blocks: []
github: null
origin: "Steward cut 2026-09-12 from the umbrella LANG-MATCH-PATTERN-FORMS-ABSENT on the spec-enclave disposition evt_12qrtnp7237dn (the prerequisite-ordered six-slice cut) and the Architect L2 decomposition ruling evt_127n516pkvtnb. Slices 1-5 landed: as-patterns (LANG-MATCH-AS-PATTERN e6645d7c2), tuple/pair (LANG-MATCH-TUPLE-PATTERN af2b36dc8), record (LANG-MATCH-RECORD-PATTERN a42b90454), or-patterns (LANG-MATCH-OR-PATTERN 7f80228c2), guards (LANG-MATCH-GUARDS, merged). This is slice 6, the final form. The umbrella's earlier 'BLOCKED on DecEq + Float/Decimal + expected-type' status is SUPERSEDED: the Architect ruled (evt_127n516pkvtnb, re-derived from c1ef0b004) that literal comparison is VALUE-LEVEL per the 34 §3.2 comparator table -- lawful DecEq is OPTIONAL, not a blocker; the separately-staged DecEq Char/String instances (already landed, LawfulClasses.ken.md:625/2359/2409) neither supply nor replace the value comparators. Prerequisite pin discharged: SPEC-MATCH-PATTERN-PINS (34fd01c1) landed the literal-kind-to-comparator table + the expected-type-is-scrutinee's pin (34 §3.2). Anchors measured by the Steward at origin/main d750d694d before framing (PatKind ast.rs:178; top-level Wild/Var refusal elab.rs 15772/15859/15999/16133; comparators eq_int, eq_float/eq_float32 numbers.rs, decimalEq/eqChar decimal_char.rs -- re-measure at cut). The umbrella stays draft and flips its slice-6 row when this lands."
---

> # RELEASED 2026-09-12 to the language ring (lane-2), slice 6 (FINAL) of the
> # enclave's six-slice cut (evt_12qrtnp7237dn) and the Architect L2 decomposition
> # ruling evt_127n516pkvtnb. depends_on satisfied: SPEC-MATCH-PATTERN-PINS merged
> # (34fd01c1) -- it discharged the literal-kind-to-comparator table pin and the
> # expected-type-is-the-scrutinee's pin (34 §3.2). Base = current origin/main
> # d750d694d. RE-MEASURE every anchor at your cut -- five prior slices advanced
> # the elab.rs match spine (arm_used / NoInhabitants / top-level refusal / the
> # PatKind and RPatKind enums) repeatedly; escalate a false fixed input rather
> # than building around it.
> #
> # The Architect is the REQUIRED reviewer alongside language-qa + CI: the
> # value-level-not-proof comparator boundary, the Float/Float32/Decimal
> # value-vs-lawful-DecEq distinction, and the fail-closed rejection of
> # unsupported carriers are a correctness argument about the elaborator's
> # comparator selection, not a mechanical edit. Tier T1. Standing Adversary hunt
> # independent -> Steward M1-M4 -> lieutenant.

## What this is

The **literal** form -- slice 6, the final one, of the six absent forms in
`spec/30-surface/34-data-match.md §3` (form list `§3.1` bullet 3; the
comparator contract at `§3.2:385-420`). A literal pattern is matched by a
**value-level branch-selection comparator**, not a carrier-bearing or
proof-bearing pattern.

Two properties are the whole content of this slice, and both come straight from
`34 §3.2` (re-affirmed by the Architect ruling `evt_127n516pkvtnb`):

1. **Expected-type-then-comparator.** A literal pattern always has the
   **scrutinee's type** as its expected type. The literal is first checked at
   that expected type by `35 §4`, and comparator selection uses the **resulting
   carrier**. Numeric defaulting therefore does not choose the comparator. A
   literal form the expected carrier does not admit is a **type error**.

2. **Value-level, not proof-level.** The comparator returns a `Bool`; it
   **constructs no `Equal` proof and adds no equality hypothesis** to the
   branch. A lawful `DecEq` certificate *may* license a comparator but is **not
   required** when the type's normative value semantics already fixes a total
   comparison -- the interpreter ring's "a wrong value, never a false proof."
   This is why `Float`/`Float32`/`Decimal` participate at all: their value
   comparators are deliberately **not** lawful `DecEq` operations.

The comparator table is normative and self-contained in `34 §3.2`; implement
exactly it:

| expected carrier | comparator | note |
|---|---|---|
| `Int` | `eq_int` | exact integer value equality + registered `Int` certificate |
| `Int8/16/32/64`, `UInt8/16/32/64` | exact fixed-width value equality | no arithmetic/wrap/widen/narrow |
| `Float` | `eq_float` | IEEE-754 binary64 `==` (NaN unequal to all, +0.0==-0.0); not proof equality |
| `Float32` | `eq_float32` | IEEE-754 binary32 `==`, same boundary; not proof equality |
| `Decimal` | `decimalEq` | exact base-10 by exponent alignment; **reject** rather than emit a non-selecting match on the unbounded-alignment stuck case |
| `String` (ordinary or raw) | codepoint-wise `String` `eq` | canonical `String` value; independent of lawful `DecEq String` |
| character | `eqChar` | Unicode-scalar equality via the `Int` projection; independent of lawful `DecEq Char` |
| byte string / bracketed hex bytes | byte-sequence content equality | immutable length-determined bytes; no new surface op name |

**Fail-closed is a hard requirement of the cut** (enclave: every remainder
stays fail-closed until its slice lands). A numeric literal at a **user-defined
carrier** stays outside this table while the `35 §4.2` user-type literal
mechanism is unstaged -- it must **reject**, never silently accept. Any
literal/carrier pair not in the table above rejects with a type error.

## The composition discriminator this slice owes

The enclave rule: every **inner** pattern form must add a composition
discriminator, because literals compose under constructors (`MkWrap 3`) and
inside the other landed forms. Concretely:

- A **redundant literal arm** -- the same literal value after an earlier
  identical literal, or a literal already covered by an earlier arm -- must get
  the **subsumption** dead-cause, **never `NoInhabitants`**. `NoInhabitants` is
  honest today only because the populations that would falsify it are refused
  upstream; a literal arm over an inhabited carrier is dead by *coverage*, not
  by emptiness. Do **not** widen `Subsumed` to carry an empty winner set (that
  is the shape `LANG-REACHABILITY-SUBSUMING-ARMS` was cut to forbid).
- A finite set of literal arms over a carrier with more values than are listed
  is **non-exhaustive**, and that is reported honestly (missing cases), not
  papered over. Making a literal match ergonomic with a trailing catch-all is
  **out of scope** -- see "Not this slice."

## Deliverables

1. A literal `PatKind` (AST) + parser production for the literal forms `34 §3.2`
   admits, threaded through `RPatKind` (`resolve.rs:66`) into the match spine.
2. Elaborator comparator selection implementing the `34 §3.2` table exactly:
   expected-type check at the scrutinee type (`35 §4`), carrier-driven
   comparator, value-level `Bool` result with no `Equal`/hypothesis emission.
3. Fail-closed rejection for unadmitted carriers and user-defined-carrier
   numeric literals (`35 §4.2` unstaged), with a diagnostic that names the
   carrier and the unsupported row.
4. The composition discriminator: a redundant literal arm routes to the
   subsumption cause; a partially-covered literal set stays reported as
   non-exhaustive.
5. Tests: a reaching positive per supported table row (at minimum `Int`, one
   fixed-width, `Float`, `Decimal`, `String`, character, bytes); a fail-closed
   control for an unadmitted/user carrier; a redundant-literal-arm control
   asserting the subsumption cause not `NoInhabitants`; a control keeping the
   five landed slices green.

## Acceptance criteria

- Each supported `34 §3.2` row compiles and selects by value (a genuine
  differential, not vacuous green-vs-green): the matching literal takes its arm,
  a non-matching one falls through.
- `Float`/`Decimal` rows match by their value comparator with **no** `Equal`
  proof or equality hypothesis introduced into the branch (assert the branch
  context is unrefined by the match).
- An unadmitted carrier and a user-defined-carrier numeric literal **reject**
  with a type error (fail-closed), not a silent accept or a non-selecting match;
  the `Decimal` unbounded-alignment stuck case rejects.
- A redundant literal arm reports the **subsumption** dead-cause; `NoInhabitants`
  is never emitted for a coverage-dead literal arm.
- The five landed slices (as/tuple/record/or/guards) stay green; top-level
  `_`/`Var` refusal is **unchanged** (this slice does not lift it).
- No kernel/TCB/spec change. **Architect required reviewer** + language-qa +
  standing Adversary hunt.

## Not this slice

- **Top-level `_`/`Var` catch-all and its `ArmDeadCause` obligations.** A literal
  match without full finite coverage is non-exhaustive and honestly reported;
  making it ergonomic with a trailing catch-all is the separate Wild/Var-at-top
  -level work (with the `ArmDeadCause` third-cause + `NoInhabitants`-clause
  obligations the umbrella describes). This slice keeps the top-level refusal
  (`elab.rs` 15772/15859/15999/16133) intact.
- **The `35 §4.2` user-type literal mechanism** -- staged separately; here such
  literals are fail-closed.
- **[[LANG-CONVOY-ENCLOSING-FIELD]]** (nested-match re-typing at `34 §3.2`),
  orthogonal to which pattern forms exist.
- **[[LANG-DECEQ-CHAR-LAWFUL-INSTANCES]]** (closed already-satisfied): the lawful
  instances exist but are independent of these value comparators.

## Contention

One serial language seat; this shares `ast.rs`, `parser.rs`, `resolve.rs`, and
the `elab.rs` match spine with the five landed slices. Base = current
origin/main `d750d694d`. No cross-lane contention (L1 runtime is on
`crates/ken-lowering`; L3 is down). Re-measure every anchor at the cut.

## Sizing / tier

**Size M, tier T1.** The diff is moderate (a PatKind + parser + comparator
dispatch + two exhaustiveness/reachability interactions), but the review turns
on an argument: the value-level-not-proof boundary, the value-vs-lawful-DecEq
distinction for Float/Decimal, and fail-closed carrier rejection. Not a
mechanical edit; the Architect reviews it as a correctness claim about
comparator selection.
