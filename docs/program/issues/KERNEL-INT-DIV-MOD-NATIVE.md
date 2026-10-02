---
id: KERNEL-INT-DIV-MOD-NATIVE
title: "Int has no division: spec 18a lists div_int and mod_int as a GAP with a NATIVE verdict, and the l1_acceptance division row stays ignored. Register both as neutral kernel Ops, give raw / and % on Int a non-zero-divisor obligation, and un-ignore the row"
status: active
owner: kernel
size: M
tier: T1
gate: architect
depends_on: [KERNEL-OBS-SIGMA-QUOT-CAST-GATE]
blocks: []
github: null
origin: "Operator approved 2026-10-02 ('approve 1 and 2 as recommended') the TCB addition asked in evt_stpqbjc1ky5h, from LANG-L1-ACCEPTANCE-ROWS row 2 (Architect evt_6bg0x6s5w300n). Placed on the kernel ring after KERNEL-REFL-ENDPOINT-TYPED-CONVERSION and KERNEL-OBS-SIGMA-QUOT-CAST-GATE. Steward-filed per COORDINATION section 2."
---

# Int division with a non-zero-divisor obligation

## Objective

`fn f (a : Int) (b : Int) = a / b` elaborates to `div_int a b` and emits one
obligation that `b` is non-zero. At `b = 0` that obligation is unsatisfiable,
and an undischarged one degrades to a runtime fault, never a value.

## Settled inputs (read at `c9f931538`)

- **The verdict.** Spec 18a's `div_int` `mod_int` row (`18a-primitive-
  registry.md:270`) reads: GAP, `Int → Int → Int`, NATIVE face (b) iff
  degrade-not-wrap.
  - `div x 0` gives an unsatisfiable `NonZeroDivisor` obligation, never a
    value. `div x 0 → 0` is the blocked F1/F4 shape.
  - `mod` is truncated: `(-7) mod 3 ≡ -1`, machine `%`.
  - The oracle is the div-mod identity `a = (a div b)·b + (a mod b)`. Its
    boundaries are a zero divisor and a negative dividend, where truncated
    and floored division differ.
- **The surface.** Spec 35 §3.1: a raw `/` or `%` on a possibly-zero `Int`
  divisor emits a non-zero side-condition obligation at the operation site
  (spec 22 §2.4).
- **The machinery.**
  - Int ops are registered in `numbers.rs` (`:383`, `reg_binop!` →
    `declare_primitive` with `PrimReduction::Op`).
  - `BinOp` (`ast.rs:598`) has no `/` or `%` variant.
  - `elab.rs:10001` emits a `PartialPrim` obligation for a bare
    fixed-width `+`. That is the template for this obligation.
  - The interpreter's `prim_reduce` arms are at `eval.rs:1999`, and its
    arity table at `:6897`.
- **The predicate is definable at zero TCB.** The kernel decides
  `Eq Int` between two literals (ADR 0013 Layer 2, `obs.rs:132`). So
  `NonZeroDivisor b ≡ Eq Int b 0 → ⊥` is an ordinary definition in `Ω`, and
  at `b = 0` it is `⊤ → ⊥`.
- **The row.** `sec31_int_div_zero_emits_obligation`
  (`crates/ken-interp/tests/l1_acceptance.rs:377`) is ignored with reason
  "needs operator-approved div_int/mod_int registration (18a GAP)". Its
  body asserts nothing yet.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. **The Ops.** Register `div_int` and `mod_int` as primitive Ops of type
   `Int → Int → Int`, with no conversion rule, so both stay neutral in the
   kernel. `trusted_base()` grows by exactly these two.
2. **The surface: `/` and `%` become fixed arithmetic tokens, like `+`**
   (Architect `evt_3jj0h45bmcdp6`, which carries the design; operator
   2026-10-02: "approve / and % as built-ins"). Today they are
   generic user operators. A fallback is rejected, because the meaning of
   one spelling would depend on declaration order.
   - The lexer claims exactly `/` and `%`, so `<`, `>`, `<+>` and `/\` stay
     generic.
   - Add `BinOp::Div` and `BinOp::Mod` at `infixl 7`, in `builtin_fixity`,
     `parser.rs:5498` and `layout.rs:1823`, with no `_ =>` arm.
   - `elab_binop` dispatches them through a new `NumericEnv::classify_div`,
     whose only entry is Int. On Int it builds `div_int`/`mod_int` and emits
     one `PartialPrim` obligation, `NonZeroDivisor` of the divisor, from the
     `:10046` template. Any other type gives `TypeMismatch` naming the
     operator.
   - Declaring `fn /` or `fn %` is refused with the diagnostic class
     `fn +` gets. Measure that diagnostic first.
   - **Spec piece**, on the same branch and Decision (COORDINATION §14 (4)):
     - spec 31 §2 adds `/` and `%` to the fixed set and drops them from the
       generic list;
     - spec 32 §6's level-7 row becomes "`*`, `/`, `%`".

     The spec ring authors it, and the conformance validator votes.
3. **Evaluation.** The interpreter reduces both truncated on nonzero
   divisors. A zero divisor faults, never yields a value.
4. **The row.** Un-ignore `sec31_int_div_zero_emits_obligation`. It asserts
   the emitted obligation's goal, and that the goal at a literal `0` is
   uninhabited.

## Acceptance

- **AC-0 (measure; no build).** Record what `a / b` and `a % b` on `Int` do
  on base, and the delivered names for `⊥` and the obligation kind. Write
  `NonZeroDivisor` in those names before any edit.
- **AC-1 (behavior).**
  - `7 / 2 = 3`, `(-7) / 2 = -3`, `7 % 3 = 1` and `(-7) % 3 = -1`.
  - The div-mod identity holds on operands across 2¹²⁷ and on every sign
    pairing. The oracle is independent of the interpreter path (18a §3).
  - `x / 0` and `x % 0` fault at runtime. A test that returns any value
    there is red.
- **AC-1b (surface; each pair on a shared input).**
  - Dispatch: the same `a / b` (and `a % b`) at `Int` gives `div_int a b`
    with exactly one `PartialPrim` obligation of goal `NonZeroDivisor b`. At
    `Nat` it gives `TypeMismatch` naming `/`, not `UnboundName`.
  - Reservation: `fn / (x : Nat) (y : Nat) : Nat = x` is refused with the
    `fn +` diagnostic class. In `generic_and_fixed_operator_paths_remain_distinct`
    (`lang_reserved_infix_names.rs:536`), `/` and `%` move to the fixed-token
    assertions, and `<` and `>` stay generic.
  - Precedence: `a + b / c` is `a + (b / c)`, and `a / b * c` is
    `(a / b) * c`.
  - Restoring `/` to the generic path reddens the reservation pair.
- **AC-2 (falsifiers).**
  - Removing the obligation emission reddens the row.
  - Replacing the zero-divisor fault with `0` reddens AC-1.
  - Swapping truncated `mod` for floored `mod` reddens `(-7) % 3`.
- **AC-3.**
  - `trusted_base()` grows by exactly `div_int` and `mod_int`. Name both in
    the handoff.
  - `NonZeroDivisor` is a definition, not a postulate.
  - The 57-package census shows no verdict change, and the `l1_acceptance`
    rows that pass today stay green.
- **Gates.** Kernel QA, the Architect, and the conformance-validator for
  the NATIVE oracle (18a: the two gates are a conjunction) and the spec
  piece.

## Stop conditions

- A third trusted entry, such as an opaque `NonZeroDivisor` postulate or a
  conversion rule for either Op: an operator question.
- Native runtime lowering that accepts `div_int` without the zero check.
  Stop and name the path. Native lowering is out of scope, and refusing the
  Op there is acceptable.
- A surface name beyond `/` and `%`, which would grow the fixed prelude.
- Any change to what the kernel accepts beyond the two Ops: stop to the
  Architect.
