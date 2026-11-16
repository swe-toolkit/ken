# Evaluation is invariant under change to an absent variable

Format: `../../README.md`.
Spec: `spec/50-stdlib/66-symbolic-algebra.md` §19,
child `eb757d55826c85cd34a16b04aa75e9f431b74642`.

The public law says that changing an environment only at a variable
that does not occur in an expression preserves its evaluation. The
accepted client checks the open law application and its use as §7's
`deriv_independent` premise. The closed fixture then checks the values
3, 3, and 11 independently of the law application. Its refusal uses
the same law-application form as the accepted instance, but supplies
`Proved` for the false absence premise at variable one.

## Accepted client

The expression contains variables zero and one. `ones` maps every
variable to one. `changed_at_two` changes only the absent variable
from one to seven; `changed_at_one` changes the occurring variable
from one to five. The agreement proofs establish the corresponding
pointwise environment premises.

```ken
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EConst, EVar, EAdd, EMul, eval, occurs, deriv,
   deriv_independent, eval_absent)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const five : Nat = Suc (Suc three)
const seven : Nat = Suc (Suc five)
const eleven : Nat = Suc (Suc (Suc (Suc seven)))

const fixture : Expr = EAdd (EVar Zero) (EMul (EConst two) (EVar one))

fn ones (w : Nat) : Nat = one

fn changed_at_two (w : Nat) : Nat =
  match w {
    Zero ↦ one;
    Suc k ↦
      match k {
        Zero ↦ one;
        Suc j ↦
          match j {
            Zero ↦ seven;
            Suc i ↦ one
          }
      }
  }

fn changed_at_one (w : Nat) : Nat =
  match w {
    Zero ↦ one;
    Suc k ↦
      match k {
        Zero ↦ five;
        Suc j ↦ one
      }
  }

theorem agree_off_two
      (w : Nat)
    : (Equal Nat two w → Bottom) →
      Equal Nat (changed_at_two w) (ones w) =
  match w {
    Zero ↦ λne. Proved;
    Suc k ↦
      match k {
        Zero ↦ λne. Proved;
        Suc j ↦
          match j {
            Zero ↦ λne. absurd (ne Proved);
            Suc i ↦ λne. Proved
          }
      }
  }

theorem agree_off_one
      (w : Nat)
    : (Equal Nat one w → Bottom) →
      Equal Nat (changed_at_one w) (ones w) =
  match w {
    Zero ↦ λne. Proved;
    Suc k ↦
      match k {
        Zero ↦ λne. absurd (ne Proved);
        Suc j ↦ λne. Proved
      }
  }

theorem eval_absent_open
      (v : Nat) (e : Expr) (env : Nat → Nat)
      (absent : Equal Bool (occurs v e) False)
    : (env2 : Nat → Nat) →
      ((w : Nat) → (Equal Nat v w → Bottom) →
        Equal Nat (env2 w) (env w)) →
      Equal Nat (eval env2 e) (eval env e) =
  eval_absent v e env absent

theorem deriv_independent_from_occurs
      (v : Nat) (e : Expr) (env : Nat → Nat)
      (absent : Equal Bool (occurs v e) False)
    : Equal Nat (eval env (deriv v e)) Zero =
  deriv_independent v e env (eval_absent v e env absent)

theorem occurs_two_is_false
    : Equal Bool (occurs two fixture) False =
  Proved

theorem occurs_one_is_true
    : Equal Bool (occurs one fixture) True =
  Proved

theorem eval_ones_is_three
    : Equal Nat (eval ones fixture) three =
  Proved

theorem eval_changed_at_two_is_three
    : Equal Nat (eval changed_at_two fixture) three =
  Proved

theorem eval_changed_at_one_is_eleven
    : Equal Nat (eval changed_at_one fixture) eleven =
  Proved

theorem eval_absent_at_two
    : Equal Nat (eval changed_at_two fixture) (eval ones fixture) =
  eval_absent two fixture ones Proved changed_at_two agree_off_two
```

### symbolic-algebra/eval-absent-open

- spec: §19's generic `eval_absent` law.
- promise class: durable invariant.
- given: arbitrary `v`, `e`, `env`, an absence proof, and any `env2`
  with the stated agreement premise.
- expect: the law application checks at its public type, with no
  premise on `env2 v`.
- measured: `eval_absent_open` leaves the expression, variable, and
  environments open and supplies only the law's stated premises.
- claimed: evaluation is equal under any environment change outside
  an absent variable.
- gap: an application does not establish the generic proof body.
- why: the test applies the named public law rather than re-proving a
  closed instance.

### symbolic-algebra/eval-absent-derivative-premise

- spec: §19 and §7's `deriv_independent` premise.
- promise class: durable invariant.
- given: arbitrary `v`, `e`, and `env`, plus
  `Equal Bool (occurs v e) False`.
- expect: `eval_absent v e env absent` supplies the exact environment-
  agreement function required by `deriv_independent`.
- measured: `deriv_independent_from_occurs` uses that partial
  application directly, without an eta-expanded wrapper.
- claimed: the public syntactic-absence decision discharges the
  semantic-independence premise for evaluation.
- gap: one client does not prove either generic theorem.
- why: the result checks the ruled binder order and premise shape.

### symbolic-algebra/eval-absent-closed-fixture

- spec: §19's stated closed fixture.
- promise class: normative compatibility vector.
- given: `fixture`, `ones`, and the two changed environments.
- expect: `occurs two fixture` is false; `occurs one fixture` is
  true; the three evaluations are respectively 3, 3, and 11.
- measured: separate `Proved` claims check each Boolean and value,
  independently of the new law.
- claimed: the expression ignores variable two and changes value when
  its occurring variable one changes.
- gap: these values do not establish the law's generic quantifiers.
- why: the arithmetic follows §1's `eval` equations directly:
  `1 + 2·1 = 3` and `1 + 2·5 = 11`.

### symbolic-algebra/eval-absent-at-two

- spec: §19's `eval_absent` law.
- promise class: durable invariant.
- given: `fixture`, `ones`, `changed_at_two`, and `agree_off_two`.
- expect: the public law proves equality of the two evaluations.
- measured: one closed law application at the same inputs as the
  independent value rows.
- claimed: changing only absent variable two preserves this
  expression's evaluation.
- gap: one instance does not establish the generic theorem.
- why: the environment agreement proof excludes only variable two,
  while the independently checked values are both three.

## Refused false absence application

Append only the following declaration to a **fresh complete copy** of
the accepted client. It has the same equality and law-application form
as `eval_absent_at_two`; it changes the selected variable and its
matching changed environment and agreement proof. The refusal must be
at the supplied `Proved` absence argument, whose expected type is
`Equal Bool (occurs one fixture) False`. The separate accepted
`occurs_one_is_true` row confirms that this is a false premise. Do
not substitute a value-only `Equal Nat 11 3` claim: that would refuse
on the differing result, not test the absence premise.

### symbolic-algebra/reject-eval-absent-at-present-variable

- spec: §19's syntactic-absence premise.
- promise class: durable invariant.
- given: the complete accepted client and the same `fixture`.
- expect: refuse at `Proved` in the new `eval_absent` application;
  the failed argument is the false absence premise.
- measured: the accepted variable-two application and this variable-one
  application use the same result proposition shape and proof form.
- claimed: the law does not accept a present variable as absent.
- gap: this case does not establish that absence is necessary for
  semantic independence.
- why: `occurs one fixture` is true, so `Proved` cannot establish
  `Equal Bool (occurs one fixture) False`.

```ken
theorem refuse_eval_absent_at_one
    : Equal Nat (eval changed_at_one fixture) (eval ones fixture) =
  eval_absent one fixture ones Proved changed_at_one agree_off_one
```

## Companion trusted-identity test-design gate

Architect's AC-0 did not measure `trusted_base()` identity sets. The
companion package-owned test must take a baseline set, load the
providers first, then require the exact `GlobalId` sets after
`Data.Collections.Derived` and after `NatPolynomial` to equal that
baseline. It must also verify that `eval_absent` is owned by
`NatPolynomial` and is `Decl::Transparent`. This is a separate package-
test gate, not a behavior measured by these conformance cases; no
trusted-identity result is claimed here.

## Evidence boundary

The expected behavior follows §19 and §1.1. Architect's AC-0 reports a
scratch accepted open client, the §7 consumer application, and a false
absence application refused at its premise; this is not execution of
this committed seed. No external or `local/refs` material was
consulted. No Ken check of this seed, package test, official
conformance-runner execution, CI, publication, landing, or measured
landed-trust result is claimed.
