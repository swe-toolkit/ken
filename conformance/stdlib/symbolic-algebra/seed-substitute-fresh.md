# Substitution keeps a variable fresh

Format: `../../README.md`.
Spec: `spec/50-stdlib/66-symbolic-algebra.md` §23.
Frame: `docs/program/issues/SPEC-SYMBOLIC-ALGEBRA-SUBSTITUTE-FRESH.md`
AC-2; Architect AC-0: `evt_6x7mbs8q441fb`.

The seed is authored but unexecuted. Architect's D0 measured the generic
law and these client rows in scratch on base
`f14bb647933dea460bb9790dd0945d7b56d3a634`. That is not execution of
this seed or the official conformance runner, CI, publication, landing, or a
landed trust measurement. No external references were consulted; the seed is
grounded in spec 66 §22, the frame, and AC-0.

Every case has promise class **durable invariant**.

## Accepted client: variable one stays absent

The fixture contains variables zero and one. `moved` replaces zero with
variable two, replaces one with constant three, and leaves later variables
unchanged. Thus the replacements at occurring variables contain no one, but
the substituted expression still contains two.

```ken
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EConst, EVar, EAdd, occurs, substitute, substitute_fresh)

const one : Nat = Suc Zero

const two : Nat = Suc one

const three : Nat = Suc two

fn moved (w : Nat) : Expr =
  match w {
    Zero ↦ EVar two;
    Suc a ↦
      match a {
        Zero ↦ EConst three;
        Suc b ↦ EVar (Suc (Suc b))
      }
  }

const fixture : Expr = EAdd (EVar Zero) (EVar one)

theorem one_value : Equal Bool (occurs one (substitute moved fixture)) False = Proved

theorem two_value : Equal Bool (occurs two (substitute moved fixture)) True = Proved

theorem fresh_premise_one
      (w : Nat)
    : Equal Bool (occurs w fixture) True → Equal Bool (occurs one (moved w)) False =
  match w {
    Zero ↦ λh. Proved;
    Suc a ↦
      match a {
        Zero ↦ λh. Proved;
        Suc b ↦ λh. absurd h
      }
  }

theorem fresh_after_one : Equal Bool (occurs one (substitute moved fixture)) False =
  substitute_fresh one moved fixture fresh_premise_one
```

### `stdlib/symbolic-algebra/substitute-fresh-on-occurring-replacements`

- spec: §23, the occurrence-restricted freshness law.
- promise class: durable invariant.
- given: `fixture = EAdd (EVar Zero) (EVar one)` and `moved` as above.
- expect: the two value controls and `fresh_after_one` are accepted.
- measured: the result lacks one but contains two, and the public law
  applies using the premise for every variable occurring in the fixture.
- claimed: when every occurring replacement lacks one, substitution leaves
  one absent from this fixture's result.
- gap: this closed application does not establish the generic theorem or
  its proof body.
- controls: `one_value` witnesses absence; `two_value` prevents the case
  from passing because substitution erased all replacement variables.
- why: the two source variables reach different replacements, so the
  premise is exercised at both occurring indices.

## Refused client: variable two is introduced by a replacement

This is a separate input with a fresh accepted prefix. It repeats the same
fixture, replacement, and value controls, changing the queried variable from
one to two. The false premise is checked before the law application. Do not
pin a numeric source span or GlobalId.

```ken
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EConst, EVar, EAdd, occurs, substitute, substitute_fresh)

const one : Nat = Suc Zero

const two : Nat = Suc one

const three : Nat = Suc two

fn moved (w : Nat) : Expr =
  match w {
    Zero ↦ EVar two;
    Suc a ↦
      match a {
        Zero ↦ EConst three;
        Suc b ↦ EVar (Suc (Suc b))
      }
  }

const fixture : Expr = EAdd (EVar Zero) (EVar one)

theorem one_value : Equal Bool (occurs one (substitute moved fixture)) False = Proved

theorem two_value : Equal Bool (occurs two (substitute moved fixture)) True = Proved

theorem fresh_premise_two
      (w : Nat)
    : Equal Bool (occurs w fixture) True → Equal Bool (occurs two (moved w)) False =
  match w {
    Zero ↦ λh. Proved;
    Suc a ↦
      match a {
        Zero ↦ λh. Proved;
        Suc b ↦ λh. absurd h
      }
  }

theorem fresh_after_two : Equal Bool (occurs two (substitute moved fixture)) False =
  substitute_fresh two moved fixture fresh_premise_two
```

### `stdlib/symbolic-algebra/reject-substitute-fresh-for-occurring-replacement`

- spec: §23's premise requires freshness at every source variable that
  occurs.
- promise class: durable invariant.
- given: a fresh client prefix with the same fixture, `moved`, and both
  value controls as the accepted client.
- expect: `fresh_premise_two` refuses with `KernelRejected TypeMismatch`
  at the client's premise; do not assert a finer span or wording.
- measured: `one_value` and `two_value` pass, then the zero arm of
  `fresh_premise_two` cannot prove its conclusion.
- claimed: the premise cannot be supplied when a replacement of an
  occurring source variable introduces the queried variable.
- gap: this refusal is at the client's premise, before `fresh_after_two`
  applies the law; it does not test the generic theorem's proof body.
- controls: the accepted `fresh_premise_one`/`fresh_after_one` client shows
  the same form succeeds when the queried variable is fresh; `two_value`
  establishes that two occurs after substitution.
- why: variable zero occurs in `fixture`, `moved Zero` is `EVar two`, and
  `occurs two (EVar two)` is True, contradicting the premise's required
  False. The accepted and refused clients differ in the queried variable.

## Evidence boundary

The D0 measurement is scratch evidence for the exact client rows and public
law; it is not an execution of this seed. The separate implementation and
spec children are required for an assembled check. The rows do not establish
generic proof coverage, official conformance, CI, publication, landing, or a
landed trust result.
