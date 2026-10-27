# Partial derivatives commute in evaluation — seed cases

Format: `../../README.md`.
Spec: `spec/50-stdlib/66-symbolic-algebra.md` §6.

The §6 export is absent from the landed base of this spec candidate.
These source-shaped clients use the exact ruled public theorem; their
closed fixtures do not prove its quantifiers. Architect D0 checked the
law generically in a separate catalog-only scratch development. Do not
interpret that check as shipped §6 seed execution.

## Accepted client

Let `x` and `y` be variables zero and one, and let `xy` be their
product. The law is applied with `w = 1` first and `v = 0` second
on the left, then in the opposite order on the right, under any
environment. Under `env23 v = Suc (Suc v)` each order independently
evaluates to one. The accepted structural `Refl` theorem uses the
**same left-hand derivative expression on both sides**, controlling
the proof form of the refused claim below on the same `xy` and
variable indices.

```ken
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EVar, EMul, eval, deriv, deriv_commute)

const x : Expr = EVar Zero
const y : Expr = EVar (Suc Zero)
const xy : Expr = EMul x y
fn env23 (v : Nat) : Nat = Suc (Suc v)

theorem xy_commute (env : Nat → Nat) :
  Equal Nat (eval env (deriv Zero (deriv (Suc Zero) xy)))
    (eval env (deriv (Suc Zero) (deriv Zero xy))) =
  deriv_commute Zero (Suc Zero) xy env

theorem xy_left_value :
  Equal Nat (eval env23 (deriv Zero (deriv (Suc Zero) xy)))
    (Suc Zero) = Proved

theorem xy_right_value :
  Equal Nat (eval env23 (deriv (Suc Zero) (deriv Zero xy)))
    (Suc Zero) = Proved

theorem xy_left_self :
  Equal Expr (deriv Zero (deriv (Suc Zero) xy))
    (deriv Zero (deriv (Suc Zero) xy)) = Refl
```

### `stdlib/symbolic-algebra/derivative-commutes-in-evaluation`

- spec: `spec/50-stdlib/66-symbolic-algebra.md` §6.
- promise class: durable invariant.
- given: `xy` and both derivative orders, with every environment for
  the law instance and the closed `env23` for two value controls.
- expect: the generic public law application accepts; each order
  computes to `Suc Zero` at `env23` independently. Reflexivity of the
  unchanged left tree also accepts.
- measured: the law application, two direct exact-value proofs and
  same-tree structural `Refl` control, only on the stated operands.
- claimed: the public theorem equates the evaluated results of the
  two orders at every environment, not their derivative syntax.
- gap: one product and one environment cannot establish the generic
  proof or the absence of a trust delta; those need separate catalog
  verification.
- why: the independent values check the numerical consequence, while
  the accepted `Equal Expr ... = Refl` control makes the false
  structural-equality refusal discriminate on the same expression.

## Refused cross-order structural equality

Runner assembly: accept the **entire positive Ken fence** above.
Append only the following theorem to a *fresh copy* of that accepted
client. The expected failure is in this new declaration, not an
unbound import, unsupported fixture or earlier accepted theorem.

Although both derivative orders evaluate to one at `env23` and the
public §6 law equates their values at every environment, §5's
product-rule constructor order puts the middle terms in different
positions. The two resulting `Expr` trees are not structurally
identical. The positive `xy_left_self` uses the same `xy`, the same
variables, the same structural `Equal Expr` form and the same `Refl`
proof; only the right expression changes to the opposite derivative
order here.

### `stdlib/symbolic-algebra/reject-derivative-tree-commutation`

- spec: `spec/50-stdlib/66-symbolic-algebra.md` §6.
- promise class: durable invariant.
- given: a fresh copy of the full accepted prefix, followed by one
  false `Equal Expr` proposition on that same `xy` and two indices.
- expect: rejection at `refuse_derivative_tree_commutation`, not in
  an import, fixture or accepted prefix.
- measured: the accepted prefix's true same-tree `Refl` and both
  one-valued reductions isolate the attempted **structural** claim.
- claimed: pointwise evaluation equality does not identify the
  literal expression trees produced by the two derivative orders.
- gap: this one product witnesses a failed structural equality; it
  does not characterize all expressions or refute semantic equality.
- why: confusing `Equal Nat` after `eval` with `Equal Expr` would
  accept this false claim despite the unchanged §5 constructor order.

```ken
theorem refuse_derivative_tree_commutation :
  Equal Expr (deriv Zero (deriv (Suc Zero) xy))
    (deriv (Suc Zero) (deriv Zero xy)) = Refl
```

## Evidence boundary

Architect D0 on `2f78d45b8` checked the generic §6 proof with
`ken check` and `ken fmt --check` rc 0, two probe tests passed,
no new `Axiom`, and identical scratch `trusted_base()` identity
sets 114 → 114. Its root-loaded client accepted the same generic
`xy` application, independent closed value proofs and same-tree
`Refl`, then rejected the false cross-order structural claim with
`TypeMismatch` at `Refl`. These are isolated D0 checks, not checks against shipped §6
catalog exports. The authored fences were checked separately in an
isolated worktree at the same exact base. Only the catalog path of
Architect's D0 patch (SHA-256
`9be6778006eb50954e476ca3690273dee329db6e4ec1e8dde71c72ce8cfb345d`)
was applied; its test path was not applied. Architect's locally
built scratch CLI (SHA-256 at this check:
`6bfd12654f508fc96f3f4d4a44214a15306b6b1ab95e21e9635be36f597adcc2`)
accepted the full positive client, rc 0. A fresh full-positive-prefix
client with only `refuse_derivative_tree_commutation` appended
returned rc 1/E0038 `TypeMismatch`, `Refl expects an Eq-shaped goal`,
at appended `Refl` bytes 932–936, inside the declaration starting
at byte 798. The accepted prefix checks the same-operand,
same-form structural `Refl` control and both exact-value rows,
so this refusal is at the intended false cross-order claim.
Logs: `/tmp/spec-author-deriv-commute-{Positive,StructuralRefusal}.log`.
These are D0-overlay checks, not shipped-seed execution, official
conformance, CI, publication, landing or landed trust.
