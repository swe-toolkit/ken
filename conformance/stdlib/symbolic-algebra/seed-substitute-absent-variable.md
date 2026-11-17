# Substitution leaves an expression unchanged when a variable is absent

Format: `../../README.md`.
Spec: `spec/50-stdlib/66-symbolic-algebra.md` §20, reviewed Spec child
`cb7a231cd75e8c188bd035e9b98712a0bf246eee` (not in this seed's base).
Frame criterion: AC-2. Seed base: `4d1cbe1ca285a6db53561493fdb047ea1b30a31a`.

The public law requires both syntactic absence of `v` in `e` and that
`s` leave every other variable unchanged. Its result is structural
`Equal Expr`, not equality of evaluations. The accepted client applies
the law to an open expression and to a sum and product with `v = 1`
absent. The present-variable control computes the actual substituted
expression. The refusal has the same structural equality form as that control,
but claims the original tree as the result.

## Accepted client

The function `s1` maps variable one to seven and leaves every other
variable unchanged. The two closed absent fixtures exercise the sum and
product recursive cases separately. `fixed_off_one` supplies the law's
second premise; its open successor arm uses `Refl`, while its closed
arms use the terminals checked in Architect's D0 client.

```ken
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EConst, EVar, EAdd, EMul, occurs, substitute, substitute_absent)

const one : Nat = Suc Zero
const two : Nat = Suc one
const seven : Nat = Suc (Suc (Suc (Suc (Suc two))))

fn s1 (w : Nat) : Expr =
  match w {
    Zero ↦ EVar Zero;
    Suc a ↦
      match a {
        Zero ↦ EConst seven;
        Suc b ↦ EVar (Suc (Suc b))
      }
  }

theorem fixed_off_one (w : Nat)
    : (Equal Nat one w → Bottom) → Equal Expr (s1 w) (EVar w) =
  match w {
    Zero ↦ λne. Proved;
    Suc a ↦
      match a {
        Zero ↦ λne. absurd (ne Proved);
        Suc b ↦ λne. Refl
      }
  }

theorem open_law
      (v : Nat) (s : Nat → Expr) (e : Expr)
      (absent : Equal Bool (occurs v e) False)
      (fixed : (w : Nat) → (Equal Nat v w → Bottom) → Equal Expr (s w) (EVar w))
    : Equal Expr (substitute s e) e =
  substitute_absent v s e absent fixed

const sum_fixture : Expr = EAdd (EVar Zero) (EConst two)
const prod_fixture : Expr = EMul (EVar Zero) (EVar two)
const present_fixture : Expr = EAdd (EVar one) (EConst two)

theorem sum_absent : Equal Bool (occurs one sum_fixture) False = Proved
theorem prod_absent : Equal Bool (occurs one prod_fixture) False = Proved

theorem sum_unchanged : Equal Expr (substitute s1 sum_fixture) sum_fixture =
  substitute_absent one s1 sum_fixture sum_absent fixed_off_one

theorem prod_unchanged : Equal Expr (substitute s1 prod_fixture) prod_fixture =
  substitute_absent one s1 prod_fixture prod_absent fixed_off_one

theorem present_value
    : Equal Expr (substitute s1 present_fixture) (EAdd (EConst seven) (EConst two)) = Refl
```

### symbolic-algebra/substitute-absent-open

- spec: §20's generic `substitute_absent` law.
- promise class: durable invariant.
- given: arbitrary `v`, `s`, and `e`, with the law's absence and
  off-`v` premises.
- expect: the public law application checks at its structural
  `Equal Expr` result type.
- measured: `open_law` leaves `v`, `s`, and `e` open and applies the
  law with exactly its two stated premises.
- claimed: the law's public type and premise order are usable without
  fixing the expression or substitution.
- gap: applying a public law does not verify its generic proof body.
- why: this pins the public application shape separately from the
  closed fixtures.

### symbolic-algebra/substitute-absent-sum

- spec: §20's syntactic-absence premise and structural substitution law.
- promise class: durable invariant.
- given: `sum_fixture = EAdd (EVar Zero) (EConst two)`, `v = one`,
  and `s1`.
- expect: `occurs one sum_fixture` is false and the law proves
  `substitute s1 sum_fixture` structurally equal to `sum_fixture`.
- measured: `sum_absent` and `sum_unchanged` use the public Boolean
  decision and public substitution law.
- claimed: substitution at an absent variable leaves this sum unchanged.
- gap: this closed instance does not establish the generic theorem.
- why: the expression exercises the `EAdd` case with variable zero
  and a constant while the selected variable is absent.

### symbolic-algebra/substitute-absent-product

- spec: §20's syntactic-absence premise and structural substitution law.
- promise class: durable invariant.
- given: `prod_fixture = EMul (EVar Zero) (EVar two)`, `v = one`,
  and `s1`.
- expect: `occurs one prod_fixture` is false and the law proves
  `substitute s1 prod_fixture` structurally equal to `prod_fixture`.
- measured: `prod_absent` and `prod_unchanged` use the public Boolean
  decision and public substitution law.
- claimed: substitution at an absent variable leaves this product
  unchanged when two other variables occur.
- gap: this closed instance does not establish the generic theorem.
- why: the expression exercises the `EMul` case and distinguishes
  variable absence from an empty or constant-only fixture.

### symbolic-algebra/substitute-present-value-control

- spec: §20's substitution behavior, with the selected variable present.
- promise class: normative compatibility vector.
- given: `present_fixture = EAdd (EVar one) (EConst two)` and `s1`.
- expect: substitution yields `EAdd (EConst seven) (EConst two)`.
- measured: `present_value` closes the exact structural equality by
  `Refl`.
- claimed: the substitution fixture actually changes variable one.
- gap: one fixed result is a control, not a proof of the general law.
- why: it checks the real result on the same input used by the refusal.

## Refused false equality at a present variable

Append only the following declaration to a fresh complete copy of the
accepted client. The expected refusal is at the new `Refl` in
`present_unchanged`, not in the accepted prefix. It has the same
input, substitution, `Equal Expr` form, and proof terminal as
`present_value`; only the claimed right-hand expression differs.

```ken
theorem present_unchanged : Equal Expr (substitute s1 present_fixture) present_fixture = Refl
```

### symbolic-algebra/reject-substitute-unchanged-at-present-variable

- spec: §20's absence boundary and structural substitution behavior.
- promise class: durable invariant.
- given: the complete accepted client and `present_fixture`.
- expect: reject the appended claim at its `Refl`; the substitution
  result is not the original expression tree.
- measured: the accepted `present_value` and refused
  `present_unchanged` have the same input and proof form.
- claimed: substitution is not identity on a variable that `s1`
  changes.
- gap: this one present-variable case does not characterize every
  non-absent expression.
- why: `s1` replaces `EVar one` with `EConst seven`, so the true result
  is the `EAdd` of seven and two, not `present_fixture`.

## Companion package-test boundary

Architect's D0 did not measure `trusted_base()` identity sets. The
Foundation package test owns that gate and must compare exact
`GlobalId` sets before and after loading `NatPolynomial`; it also
checks law ownership and transparency, the owner constant in `open_law`,
and the accepted and refused clients. The refused span must lie inside
`present_unchanged` and cover its `Refl`. This seed makes no trusted-base
claim or package-test claim.

## Evidence boundary

This seed is authored from AC-2 and Architect's D0 on landed base
`4d1cbe1ca285a6db53561493fdb047ea1b30a31a`, not from the unmerged
Spec child. D0 reports that the accepted client checked and that a
fresh accepted prefix plus `present_unchanged` refused at its own
`Refl` with `TypeMismatch`. The D0 scratch binary's source commit was
not identified. These are D0 observations, not execution of this seed.
No Ken check or formatter check of this seed, package test, official
conformance-runner execution, CI, publication, landing, or measured
landed-trust result is claimed. No external or `local/refs` material
was consulted.
