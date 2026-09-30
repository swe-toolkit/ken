---
id: LANG-REFINEMENT-INTRODUCTION-OBLIGATION
title: "A value introduced at a def-named refinement (Char, PosInt) emits no obligation, so const c : Char = 55296 elaborates with isScalar 55296 untracked; spec 34 section 5 requires every introduction at a refinement, literal or named, to emit phi a. Record the named predicate by GlobalId and emit at the one expected-type check"
status: ready
owner: language
size: M
tier: T1
gate: architect
depends_on: []
blocks: [TEST-CHAR-CONSTRUCTION-ROUTES-SCALAR]
github: null
origin: "Architect ruling evt_1a3kmh4jf12ga on the TEST-CHAR-CONSTRUCTION-ROUTES-SCALAR (b)/(c) stop: routes (b) and (c) are accepted at 944ff08ca because a named refinement emits no introduction obligation; an elaborator defect against spec 34 section 5, not a design fork. Steward-filed per COORDINATION section 2."
---

# Every introduction at a refinement emits its obligation

## Objective

Spec 34 §5: a value introduced at a refinement `{x:A|φ}`, whether the type
is written literally or through a `def` name, emits the obligation `φ a`.

## Settled inputs (Architect `evt_1a3kmh4jf12ga`, measured on `944ff08ca`)

- `def Char = { c : Int | isScalar c }` (`decimal_char.rs:243`) goes through
  the `RDeclKind::TypeAlias` arm (`elab.rs:12205`). `elab_type`'s `RRefine`
  arm returns only the carrier (`elab.rs:912`). The result is a transparent
  `Char := Int` with no obligations, and `φ` is recorded nowhere.
- The kernel sees Char as a transparent alias convertible to Int
  (`check.rs:1338`). No core-level check can see the predicate, and the
  carrier encoding is normative.
- The only refinement obligation emitted today is `Ensures`, from a return
  type that is syntactically an `RRefine` (`innermost_refine_pred`,
  `elab.rs:13733`, used at `:13262` and `:14460`). `ObligationKind`
  (`elab.rs:43`) has no kind for an argument, let or field introduction.
- **The pinned literal case** is `ac7_refinement_intro_emits_obligation`
  (`l2_acceptance.rs:757`). Row `surface/declarations/def-refinement`
  (`seed-def-refinement.md:17`) claims the named case on a core-decl proxy.
- **Soundness today.** Nothing trusted asserts that every Char is a scalar.
  While this is open, no postulate going List Char → String → List Char
  may be admitted.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

- The elaborator records a named refinement's predicate in a side table
  keyed on the def's `GlobalId`, not its spelling.
- It emits the obligation at the one place a term is checked against an
  expected type whose head, after alias resolution, is a refinement. Syntax
  sites are not enumerated.
- No kernel change.

## Acceptance

- **AC-0 (measure first; no build).** Tabulate obligation counts via
  `elaborate_decl_v1`.
  - Rows are the introduction-site kinds: return type, const annotation,
    argument at an application, let annotation, constructor or record
    field, and match-arm result.
  - Columns are a literal refinement against a `def`-named one.
  - Only the literal return and const cells should be nonzero today. The
    Architect rules the design before any edit.
- **AC-1.**
  - Every table cell emits the obligation.
  - `55296` at Char leaves `isScalar 55296` undischarged, and `55295` leaves
    none.
  - `toC`'s body emits `isScalar n` with `n` free.
- **AC-2 (consumers and controls).**
  - The prelude's `intToChar` discharges its `Some Char n` from the arm's
    `inRangeBool n = True`.
  - The guide's `const five : PosInt = 5` (`surface-reference.ken.md:108`)
    discharges.
  - The seeds `seed-def-refinement`, `seed-data-match`, `seed-obligations`
    and `seed-spec-syntax` are swept.
  - `ac7_plain_carrier_no_obligation` stays at zero.

## Stop conditions

- A new `trusted_base()` entry in the prelude or catalog.
- Any kernel change.
- A checked program gains an obligation it cannot discharge.
