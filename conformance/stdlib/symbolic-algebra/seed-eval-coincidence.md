# Evaluation depends on the environment at occurring variables

Format: `../../README.md`.
Spec: `spec/50-stdlib/66-symbolic-algebra.md` §21, settled Spec child
`d92e579068b13be018fddb5f508f138bdded797b`; evaluation is defined in §1.1.

The law compares two total environments only at indices occurring in the
expression. The accepted client changes two absent indices at once; the
refused client changes one occurrence in the fixture while keeping the
environment pair and agreement-proof form fixed.

## Accepted client

```ken
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EConst, EVar, EAdd, EMul, eval, occurs, eval_coincidence)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const five : Nat = Suc (Suc three)
const seven : Nat = Suc (Suc five)
const nine : Nat = Suc (Suc seven)

const fixture : Expr = EAdd (EVar Zero) (EMul (EConst two) (EVar one))
const fixture_two : Expr = EAdd (EVar two) (EMul (EConst two) (EVar one))

fn ones (w : Nat) : Nat = one

fn changed_at_two_three (w : Nat) : Nat =
  match w {
    Zero ↦ one;
    Suc k ↦
      match k {
        Zero ↦ one;
        Suc j ↦
          match j {
            Zero ↦ seven;
            Suc i ↦
              match i {
                Zero ↦ five;
                Suc n ↦ one
              }
          }
      }
  }

theorem agree_on_fixture
      (w : Nat)
    : Equal Bool (occurs w fixture) True
      → Equal Nat (changed_at_two_three w) (ones w) =
  match w {
    Zero ↦ λh. Proved;
    Suc k ↦
      match k {
        Zero ↦ λh. Proved;
        Suc j ↦
          match j {
            Zero ↦ λh. absurd h;
            Suc i ↦
              match i {
                Zero ↦ λh. absurd h;
                Suc n ↦ λh. Proved
              }
          }
      }
  }

theorem eval_coincidence_at_two_absent_indices
    : Equal Nat
        (eval changed_at_two_three fixture)
        (eval ones fixture) =
  eval_coincidence fixture changed_at_two_three ones agree_on_fixture

theorem occurs_two_in_fixture_is_false
    : Equal Bool (occurs two fixture) False =
  Proved

theorem occurs_three_in_fixture_is_false
    : Equal Bool (occurs three fixture) False =
  Proved

theorem occurs_two_in_fixture_two_is_true
    : Equal Bool (occurs two fixture_two) True =
  Proved

theorem eval_ones_fixture_is_three
    : Equal Nat (eval ones fixture) three =
  Proved

theorem eval_changed_fixture_is_three
    : Equal Nat (eval changed_at_two_three fixture) three =
  Proved

theorem eval_ones_fixture_two_is_three
    : Equal Nat (eval ones fixture_two) three =
  Proved

theorem eval_changed_fixture_two_is_nine
    : Equal Nat (eval changed_at_two_three fixture_two) nine =
  Proved
```

### stdlib/symbolic-algebra/eval-coincidence-two-absent-indices

- spec: §21's `eval_coincidence` law.
- promise class: durable invariant.
- given: `fixture`, `ones`, `changed_at_two_three` and
  `agree_on_fixture`.
- expect: the public law application checks and proves equality of the
  evaluations under environments that differ at both indices two and three.
- measured: one concrete application of `eval_coincidence` with a pointwise
  proof over every `w : Nat`.
- claimed: evaluation is equal when the environments agree at each index
  occurring in the expression, even when they differ at two absent indices.
- gap: this closed application does not prove the generic theorem body.
- why: the fixture contains only indices zero and one; the pointwise premise
  proves equality at those indices while permitting both other differences.
  A single §19 `eval_absent` application can exclude only one selected index:
  choosing two leaves the difference at three, and choosing three leaves the
  difference at two. It cannot discharge both changes at once.

### stdlib/symbolic-algebra/eval-coincidence-known-values

- spec: §21's stated fixture and §1.1 evaluation equations.
- promise class: normative compatibility vector.
- given: the two fixtures and both environments in the accepted client.
- expect: indices two and three do not occur in `fixture`; index two occurs
  in `fixture_two`. The evaluations of `fixture` under `ones` and
  `changed_at_two_three` are both three. For `fixture_two`, the corresponding
  evaluations are respectively three and nine.
- measured: separate `Proved` equations check occurrence and evaluation
  values without invoking `eval_coincidence`.
- claimed: changing the environment at absent indices preserves the fixture's
  value, while the second fixture exposes the changed value at an occurring
  index.
- gap: the closed values do not establish the generic agreement theorem.
- why: the independent `Proved` value and occurrence rows make the expected
  control values observable without deriving them from `eval_coincidence`.

## Refused occurring-index agreement

Append only the following declaration to a fresh complete copy of the accepted
client. The environment pair is unchanged. The fixture changes only its first
variable from zero to two, and the agreement-proof body has the same form.
The diagnostic owner is the declaration `agree_on_fixture_two`; its
source-site discriminator is the `absurd h` expression in the index-two arm.
No candidate-local numeric byte span is claimed.

```ken
theorem agree_on_fixture_two
      (w : Nat)
    : Equal Bool (occurs w fixture_two) True
      → Equal Nat (changed_at_two_three w) (ones w) =
  match w {
    Zero ↦ λh. Proved;
    Suc k ↦
      match k {
        Zero ↦ λh. Proved;
        Suc j ↦
          match j {
            Zero ↦ λh. absurd h;
            Suc i ↦
              match i {
                Zero ↦ λh. absurd h;
                Suc n ↦ λh. Proved
              }
          }
      }
  }
```

### stdlib/symbolic-algebra/eval-coincidence-present-index-refuses

- spec: §21's agreement premise, contrasted with its stated refusal fixture.
- promise class: durable invariant.
- given: the complete accepted client and the refused declaration above.
- expect: reject with `KernelRejected TypeMismatch` within the
  `agree_on_fixture_two` declaration, whose span contains the index-two
  `absurd h`; the found type is the `occurs` equation at index two. This is
  not a later `eval_coincidence` application. In that arm `h` has type
  `Equal Bool True True`, not `Bottom`. The accepted controls establish that
  index two occurs in `fixture_two` and its evaluations are nine and three.
- measured: the diagnostic span is within `agree_on_fixture_two`'s declaration
  and contains the index-two `absurd h`; the found type is the `occurs`
  equation at index two. No numeric byte span is asserted.
- claimed: the law's agreement premise cannot be supplied at an index that
  occurs when the two environments disagree there.
- gap: this fixture does not characterize every invalid agreement premise.
- why: changing only the fixture's first variable makes the same index-two
  arm move from the absent case to a true occurrence; the same `absurd h`
  proof then fails for the specified reason.

## Evidence boundary

The contract source is the settled Spec child
`d92e579068b13be018fddb5f508f138bdded797b` and Architect AC-0
`evt_7k0fe3tsc76s6`. The initial seed was authored against landed base
`24f47951f6f417923f24c29c031dc39d50a32879`; this wording-only revision is
based on `origin/main` `831bd875faaf8ee1d710850395a46e0d74c9a5d1`. This seed does
not change Spec, catalog or crates. No external or `local/refs` material
was consulted.
No seed execution, official conformance, CI, publication, landing or measured
trusted-base result is claimed. The assembled package test owns AC-T's exact
trusted-base identity check.
