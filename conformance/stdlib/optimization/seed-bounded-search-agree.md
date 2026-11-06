# Predicates that agree below the bound have the same search result

Format: `../../README.md`.
Spec: `spec/50-stdlib/72-optimization.md` §2.7.

This seed targets the public `find_below_ext` law. Its accepted client
uses `above_two` and `never` at bound three, where both predicates are
false at every index below the bound. A direct `Proved` control accepts
the same result claim. The refused claim keeps the predicates and result
form but raises the bound to ten: the first disagreement is at three,
where `above_two` is true and `never` is false. These closed cases do
not prove the generic law. The seed has not been run by the official
conformance runner; no CI, publication, landing or trust measurement
is claimed.

## Accepted client and same-form control

`above_two n` is true exactly when `2 < n`; `never n` is always false.
Below three, the predicates agree, so both bounded searches return
`None`. The law client uses the pointwise-below-bound premise from the
public type. The direct `Proved` control checks the same result claim
without the law.

```ken
import Algorithm.Optimization.BoundedSearch (find_below, find_below_ext)

import Data.Numeric.Nat.Order (lt_nat)

const two : Nat = Suc (Suc Zero)

const three : Nat = Suc two

const ten : Nat = Suc (Suc (Suc (Suc (Suc (Suc (Suc three))))))

fn above_two (n : Nat) : Bool = lt_nat two n

fn never (n : Nat) : Bool = False

theorem above_two_agrees_with_never_below_three
    : (m : Nat) → Equal Bool (lt_nat m three) True → Equal Bool (above_two m) (never m) =
  λm.
    match m {
      Zero ↦ λwithin. Proved;
      Suc m1 ↦
        match m1 {
          Zero ↦ λwithin. Proved;
          Suc m2 ↦
            match m2 {
              Zero ↦ λwithin. Proved;
              Suc m3 ↦ λwithin. absurd within
            }
        }
    }

theorem agree_below_three_by_law
    : Equal (Option Nat) (find_below above_two three) (find_below never three) =
  find_below_ext above_two never three above_two_agrees_with_never_below_three

theorem agree_below_three_by_proved
    : Equal (Option Nat) (find_below above_two three) (find_below never three) =
  Proved
```

### `stdlib/optimization/bounded-search-agreement-law-client`

- spec: §2.7.
- promise class: durable invariant.
- given: `above_two`, `never`, bound three and a proof that the predicates
  agree at every index strictly below three.
- expect: the public law accepts the claim that both searches return the
  same result.
- measured: the client supplies the exact premise required by the public
  theorem type; the direct `Proved` control checks the same closed result.
- claimed: predicates that agree strictly below a bound give the same
  `find_below` result at that bound.
- gap: this closed instance does not establish the generic law.
- why: the client consumes the public theorem, while the direct control
  checks only the finite result claim.

### `stdlib/optimization/bounded-search-agreement-same-form-control`

- spec: §2.1 and §2.7.
- promise class: durable invariant.
- given: the same predicates and bound-three equality claim, proved by
  `Proved` directly.
- expect: the claim is accepted.
- measured: this control uses the identical equality proposition asserted
  by the law client.
- claimed: the closed result claim itself is true at bound three.
- gap: the finite control does not establish the generic law.
- why: it controls for a malformed or false equality as the reason the
  law client might be rejected.

## Refused claim

For the refusal, use a fresh copy of the entire accepted client above
and append only the declaration below. Require refusal at this appended
theorem with `KernelRejected TypeMismatch`; do not count a refusal in
an import or earlier declaration as a pass. At bound ten, index three
is below the bound and separates the predicates: `above_two` is true
there, while `never` is false. The accepted bound-three control shows
that the equality proposition and `Proved` form are not themselves the
reason for refusal.

### `stdlib/optimization/bounded-search-agreement-disagreement-refusal`

- spec: §2.1 and §2.7.
- promise class: durable invariant.
- given: the same predicates and equality proposition, with bound ten.
- expect: refuse the claim that both searches return the same result.
- measured: the range now includes index three, where the predicates
  disagree.
- claimed: the law does not equate search results when its agreement
  premise fails within the bound.
- gap: this finite refusal does not prove the generic law.
- why: `find_below above_two ten` finds three, while `find_below never
  ten` returns `None`; the same-form bound-three control accepts.

```ken

-- Refused: above_two holds at three, below ten, and never does not.
theorem reject_agree_below_ten_by_proved
    : Equal (Option Nat) (find_below above_two ten) (find_below never ten) =
  Proved
```

## Coverage boundary

The seed covers one closed application of the public law, its explicit
pointwise agreement premise, a direct same-form acceptance control and
a same-form refusal when the bound includes a disagreement. The finite
instances do not prove the universally quantified law or specify a
search strategy, predicate-call count or performance guarantee. No
package test or official conformance execution is claimed.
