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
     whose only entry is Int. On Int it builds `div_int`/`mod_int`. Any
     other type gives `TypeMismatch` naming the operator.
   - **The obligation, only on a possibly-zero divisor** (spec 35 §3.1,
     22 §2.4; Architect `evt_20wykc13b8916`). At `3aca62b5d` the body
     elaborates before `requires` (`elab.rs:14952` vs `:15020`), and
     `RRefine` erases φ (`:989`), so `cx.ctx` holds neither, and an
     unconditional push closed over `cx.ctx` is unprovable under `requires`.
     - `ElabCtx` gains elaborator-only `assumptions` (prop, depth); they
       never enter a kernel term. `elaborate_view_with_spec` elaborates each
       `requires` before the body and pushes it at the parameter depth.
       `requires` only.
     - **Refined parameters are not assumptions** (Architect audit
       `evt_27wngyard37xa` withdraws that half of `evt_5ma7hzg4a0e6`).
       `{x:A|φ}` lowers to its carrier and no call site establishes φ, so
       using φ as an assumption would remove the only check: measured, `f
       (d : {z : Int | Not (Equal Int z 0)}) = n / d` then `g = f 1 0` gave
       0 obligations and 0 new trust. Refined φ enters neither recognition
       nor the goal telescope. `install_refined_param_assumptions` and its
       call sites go.
     - At the `/` or `%` site, a direct assumption that the kernel's
       conversion finds `≡ NonZeroDivisor rhs` gives no hole, no obligation
       and no counter bump. No spelling match and no search beyond a direct
       assumption.
     - Otherwise one `PartialPrim` obligation, closed over the context with
       each assumption inserted at its depth, through one telescope builder.
     - Introducing a refinement keeps its spec 34 §5 use-site obligation.
     - **One obligation sequence per declaration** (Architect
       `evt_5ma7hzg4a0e6`). Today both Phase 1 branches and
       `elab_in_ctx_at_omega` drop their `ElabCtx` obligations, so every
       body obligation under `requires` is lost (the fixed-width `+` one
       too, `:10067`). `elaborate_view_with_spec` absorbs every `ElabCtx`'s
       obligations in elaboration order (requires, body, ensures),
       renumbering ids on absorption; ensures ids follow from the sequence
       length. Anything reading an `Obligation.id` before absorption is a
       stop to the Architect, never a second counter.
     - The Ω acceptance predicate for `requires` and `ensures` keeps the
       disjunction it had: accept an Ω-shaped type, or else a prop that
       kernel-checks at Ω (`evt_27wngyard37xa` R2).
   - Declaring `fn /` or `fn %` is refused with the diagnostic class
     `fn +` gets. Measure that diagnostic first.
   - **Spec piece**, on the same branch and Decision (COORDINATION §14 (4)):
     - spec 31 §2 adds `/` and `%` to the fixed set and drops them from the
       generic list;
     - spec 32 §6's level-7 row becomes "`*`, `/`, `%`".
     - spec 31 §4's post-freeze fixed-token list adds `/` and `%`, so it
       agrees with §2 (spec-leader `evt_1v7fmwtpvbn0w`).
     - spec 32 §3's `fixed_binop` grammar route adds `/` and `%`, so it
       agrees with §31 §§2, 4 and §32 §6 (CV block `evt_6f3jfs4j5q2tx`).
     - Conformance, by the CV: `surface/operators/generic-symbolic-names-remain-live`
       in `seed-reserved-infix-names.md` stops expecting `/` and `%` as
       generic names and states the fixed-token observation for them, with
       `<` and `>` still generic; `seed-numbers.md` §3.1 covers `%`'s
       `PartialPrim` obligation beside `/` (spec 35 §3.1).
     - `seed-obligations.md` gains the non-direct row: `requires Equal Int
       d 5` gives one obligation whose Γ carries the hypothesis (the CV
       writes it in the corpus's form).
     - Sweep `spec/` and `conformance/` for any other place that lists the
       fixed or generic operator spellings, and fold each one here. Report
       the sweep as a list with the gate handoff.

     The spec ring authors it, and the conformance validator votes.
3. **Evaluation.** The interpreter reduces both truncated on nonzero
   divisors. A zero divisor faults, never yields a value.
4. **The row.** Un-ignore `sec31_int_div_zero_emits_obligation`. It asserts
   the emitted obligation's goal, and that the goal at a literal `0` is
   uninhabited.

## Acceptance

- **AC-0 (measure; no build).** Record what `a / b` and `a % b` on `Int` do
  on base, and the delivered names for `⊥` and the obligation kind. Write
  `NonZeroDivisor` in those names before any edit. `≠` is Bool (33 §6.1),
  so the direct proposition is `Not (Equal Int d 0)`, which reaches
  `NonZeroDivisor d` by δβ (`evt_5ma7hzg4a0e6`).
- **AC-1 (behavior).**
  - `7 / 2 = 3`, `(-7) / 2 = -3`, `7 % 3 = 1` and `(-7) % 3 = -1`.
  - The div-mod identity holds on operands across 2¹²⁷ and on every sign
    pairing. The oracle is independent of the interpreter path (18a §3).
  - `x / 0` and `x % 0` fault at runtime. A test that returns any value
    there is red.
- **AC-1b (surface; each pair on a shared input).**
  - Dispatch: the same `a / b` (and `a % b`) at `Int` gives `div_int a b`.
    At `Nat` it gives `TypeMismatch` naming `/`, not `UnboundName`.
  - Obligations, for both `/` and `%` on one body: a possibly-zero divisor
    gives exactly 1 `NonZeroDivisor` obligation; `(d : {z : Int | Not
    (Equal Int z 0)})` gives 1, whose goal telescope has no φ; `requires
    Not (Equal Int d 0)` gives 0;
    `requires Equal Int d 5` gives 1, whose closed goal's leading Π
    telescope holds `Eq Int d 5` at param depth; `requires Not (Equal Int
    e 0)` with divisor `d` gives 1.
  - Caller pair: `f` with the refined divisor carries 1 open hole (pinned),
    and `g = f 1 0` adds no obligation.
  - Consumer sweep: run every suite that elaborates a `requires` (about 82
    lines in `crates/*/tests`; the catalog has none). Each changed
    obligation count is a census row with its cause, not a silent pin edit.
  - Reservation: `fn / (x : Nat) (y : Nat) : Nat = x` is refused with the
    `fn +` diagnostic class. In `generic_and_fixed_operator_paths_remain_distinct`
    (`lang_reserved_infix_names.rs:536`), `/` and `%` move to the fixed-token
    assertions, and `<` and `>` stay generic.
  - Precedence: `a + b / c` is `a + (b / c)`, and `a / b * c` is
    `(a / b) * c`.
  - Restoring `/` to the generic path reddens the reservation pair.
- **AC-2 (falsifiers).**
  - Removing the obligation emission reddens the row.
  - Restoring the unconditional push reddens the `requires Not (Equal Int
    d 0)` zero row. Dropping the assumptions from the closure reddens the
    `Equal Int d 5` goal-shape pin. Recognizing any assumption of shape `_ ≠
    0` reddens the `e` row. Installing a refined φ as an assumption reddens
    the caller-pair hole pin.
  - Replacing the zero-divisor fault with `0` reddens AC-1.
  - Swapping truncated `mod` for floored `mod` reddens `(-7) % 3`.
- **AC-3.**
  - `trusted_base()` grows by exactly `div_int` and `mod_int`. Name both in
    the handoff.
  - `NonZeroDivisor` is a definition, not a postulate.
  - The census over every tracked package card (55 at `710458002`) shows
    no verdict change, and the `l1_acceptance`
    rows that pass today stay green.
- **Gates.** Kernel QA, the Architect, and the conformance-validator for
  the NATIVE oracle (18a: the two gates are a conjunction) and the spec
  piece.

## Stop conditions

- The `Not (Equal Int d 0)` rows do not give 0 once obligations are
  absorbed: stop to the Architect; do not add a spelling match.
- A changed obligation count has no explained cause: stop to the
  Architect, never a skip.

- A third trusted entry, such as an opaque `NonZeroDivisor` postulate or a
  conversion rule for either Op: an operator question.
- Native runtime lowering that accepts `div_int` without the zero check.
  Stop and name the path. Native lowering is out of scope, and refusing the
  Op there is acceptable.
- A surface name beyond `/` and `%`, which would grow the fixed prelude.
- Any change to what the kernel accepts beyond the two Ops: stop to the
  Architect.
- Not this WP (Architect carries): the fixed-width `+`/`-`/`*` obligation's
  Γ; the other declaration aggregators that return `obligations: vec![]`;
  the spec 21 examples and seeds that write a Bool `≠` in an Ω position;
  ensures goals closing without the `requires` premises; the refined-argument
  introduction obligation (an AC row in
  `LANG-REFINEMENT-INTRODUCTION-OBLIGATION`, `evt_1ytv0fc4j1c1j`). Refined
  recognition in the callee needs a proof-carrying parameter encoding, a
  spec-lane decision. Until then, spec 21 §6.3 / 22 §3 and the seed
  `refined-param-is-hypothesis-not-obligation` (refined φ enters Γ) diverge
  from this WP's retained hole. That divergence is pre-existing and does not
  block this WP's votes; it goes to the operator as a spec question
  (Steward, on CV `evt_5sfpt076hm8r9`).
