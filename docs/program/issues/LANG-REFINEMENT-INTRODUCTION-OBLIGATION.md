---
id: LANG-REFINEMENT-INTRODUCTION-OBLIGATION
title: "A value introduced at a def-named refinement (Char, PosInt) emits no obligation, so const c : Char = 55296 elaborates with isScalar 55296 untracked; spec 34 section 5 requires every introduction at a refinement, literal or named, to emit phi a. Record the named predicate by GlobalId and emit at the one expected-type check"
status: ready
owner: language
size: M
tier: T1
gate: architect
depends_on: [KERNEL-LEQ-INT-LITERAL-REDUCTION]
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
- `inRangeBool` (`decimal_char.rs:234-237`) is respelled by transparent Bool
  elimination, as `inLowerScalarBool`, `inUpperScalarBool` and a `match`
  over them, with the same value on all inputs (Architect
  `evt_pawgvbeevyg2`). `isScalar 55295` and `57344` then close, and `55296`
  stays `Equal Bool False True`. The conformance row
  `conformance/surface/numbers/seed-decimal-char-demote.md` (`:361`,
  `:420-422`) is amended to "value-level Bool composition inside `IsTrue`,
  here by transparent Bool elimination", which takes the
  conformance-validator's Spec vote.
- The design is ruled at AC-0 (Architect `evt_y1v2wbnhx9ej`, R3 and R4):
  - a predicate table keyed on GlobalId;
  - a literal refinement becomes an anonymous named one;
  - one emission helper, with arm path conditions.

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
  - An argument at a literal refined parameter (Architect
    `evt_1ytv0fc4j1c1j`): given `fn f (n : Int) (d : {z : Int | Not (Equal
    Int z 0)}) : Int = n / d`, `fn g (u : Int) : Int = f 1 0` carries 1 open
    refinement obligation, `Not (Equal Int 0 0)`. `f` keeps its own
    `PartialPrim` hole. This does not license refined-parameter recognition
    in the callee: a higher-order use through the carrier Pi bypasses every
    introduction.
- **AC-2 (consumers and controls).**
  - The prelude's `intToChar` discharges its `Some Char n` from the arm's
    `inRangeBool n = True`.
  - The guide's `const five : PosInt = 5` (`surface-reference.ken.md:108`)
    discharges.
  - The seeds `seed-def-refinement`, `seed-data-match`, `seed-obligations`
    and `seed-spec-syntax` are swept.
  - **Int terms introduced at Char elsewhere** (grep for `(N : Int)` at
    `0699d6e90`; AC-0 extends the census by mechanism, not by spelling):
    - Closed literals, each of which should discharge: `Console/Text.ken.md`
      `:18` and `:28`; `Formatting/Doc.ken.md` `:106`-`:107`; the
      `char_to_digit` arguments at `Parsing/Numeric.ken.md` `:626`-`:630`;
      and `ds9_json_codec_acceptance.rs` `:458`, `:811` and `:831`.
    - One open Int expression, `(48 : Int) + natToInt d`, in `digitChar`.
      Its obligation `isScalar (48 + natToInt d)` over `d : Nat` is false
      at `d = 55248`, so the consumer over-promises (Architect AC-0 ruling
      `evt_y1v2wbnhx9ej`, R2). **In scope:** rewrite each `digitChar`
      through the prelude's checked `intToChar`, with outputs unchanged for
      `d < 10`. There are six sites:
      - `crates/ken-interp/tests/rtp1_elim_reduce_ih_perf_acceptance.rs`
        (`:199`); QA reruns its perf thresholds;
      - `examples/rosetta/{ackermann,factorial,fibonacci,fizzbuzz,gcd}/`
        `*.ken`.
  - `char-expected-integer-literal-scalar-boundary`
    (`seed-numbers.md:286`) states its observable as the obligation:
    `55295` leaves no open refinement obligation, and `55296` elaborates
    with `isScalar 55296` undischarged. It no longer says "rejects"
    (Architect `evt_1a3kmh4jf12ga`, spec leader `evt_1x72bcq61qfq3`). The
    conformance validator reviews this row delta at the exact SHA.
  - `ac7_plain_carrier_no_obligation` stays at zero.
  - **The forgetful direction emits nothing** (`{x:A|φ} ≤ A` is free, 34
    §5). The prelude's `charToInt`, `eqChar` and `leqChar`, and
    `add_int n p` with `p : PosInt` in the guide's `add_to_pos_int`, add
    zero obligations.
  - **Re-use at the same refinement emits nothing new.** In the guide's
    `const ten : Int = add_to_pos_int five five`, passing `five : PosInt` at
    a `PosInt` parameter adds none; only `const five : PosInt = 5` carries
    one (Architect `evt_3svpdbv1cjmwx`). These zeros are what separate
    emitting at introduction from emitting on the type's name.

## Stop conditions

- A new `trusted_base()` entry in the prelude or catalog.
- Any kernel change.
- A checked program gains an obligation it cannot discharge.

## Hard-stop inventory (§1b)

§1a count: 1 (Architect `evt_5qg2098zmhd20` on stop `evt_4g6d57z5x6wqz`).

1. A closed Int refinement obligation stays open. The kernel has no
   reduction for primitive `leq_int` on `IntLit` (keyed on primitive-Op
   computation), so `isScalar 55295` and PosInt `5` have no proof term. The
   route through `checked_char_literal` is refused: it misses PosInt and is
   a second scalar derivation. Recut: prerequisite
   `KERNEL-LEQ-INT-LITERAL-REDUCTION`. This WP holds its WIP until that
   merges, then resumes with AC-1 and AC-2 unchanged. WIP parked at
   `f4edc461c` on `wp/LANG-REFINEMENT-INTRODUCTION-OBLIGATION` (base
   `c4f1812f4`).
