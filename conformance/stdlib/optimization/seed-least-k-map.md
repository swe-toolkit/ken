# Least-k selection commutes with map

Format: `../../README.md`.
Spec: `spec/50-stdlib/72-optimization.md` §4.8, Spec child
`d4cb1f112b9fbea71b6928091d0ce7ddb09495a8`. Its law and discriminator
match the seed rows below. Architect AC-0: `evt_r9yxp2tff2kp`.

The fixture uses `a = b = Nat`, identity cost, `f = reverse_three`,
`xs = [0, 1, 2]`, and `k = 1`. The mapped selection and the
selection under the composed cost both yield `[0]`; selection under
uncomposed identity cost and then mapping yields `[2]`. These closed
values distinguish the stated law from omitting `f` in its cost, but
do not establish the generic law.

## Accepted client

The whole block accepts. Its three value controls independently pin
both law sides to `[0]` and the uncomposed route to `[2]`. The final
theorem applies `least_k_map` with its composed cost.

```ken
import Algorithm.Optimization.Argmin (least_k, least_k_map)
import Data.Collections.Derived (map)
import Core.Function.Combinators (comp)

const one : Nat = Suc Zero
const two : Nat = Suc one

fn identity_cost (n : Nat) : Nat = n

fn reverse_three (n : Nat) : Nat =
  match n {
    Zero ↦ two;
    Suc m ↦
      match m {
        Zero ↦ one;
        Suc r ↦ Zero
      }
  }

const xs : List Nat = Cons Nat Zero (Cons Nat one (Cons Nat two (Nil Nat)))
const zero_only : List Nat = Cons Nat Zero (Nil Nat)
const two_only : List Nat = Cons Nat two (Nil Nat)

theorem v_mapped : Equal (List Nat) (least_k Nat identity_cost one (map Nat Nat reverse_three xs)) zero_only = Refl
theorem v_composed : Equal (List Nat)
    (map Nat Nat reverse_three (least_k Nat (comp Nat Nat Nat identity_cost reverse_three) one xs)) zero_only = Refl
theorem v_uncomposed : Equal (List Nat)
    (map Nat Nat reverse_three (least_k Nat identity_cost one xs)) two_only = Refl

theorem accepted_map : Equal (List Nat)
    (least_k Nat identity_cost one (map Nat Nat reverse_three xs))
    (map Nat Nat reverse_three (least_k Nat (comp Nat Nat Nat identity_cost reverse_three) one xs)) =
  least_k_map Nat Nat reverse_three identity_cost one xs
```

### `stdlib/optimization/least-k-map-values`

- spec: §4.8, mapped least-k selection under the composed cost.
- promise class: durable invariant.
- given: `[0, 1, 2]`, `k = 1`, identity cost, and
  `reverse_three` mapping `0 ↦ 2`, `1 ↦ 1`, `2 ↦ 0`.
- expect: the mapped selection and the composed-cost route both yield
  `[0]`; the uncomposed route yields `[2]`; `accepted_map` accepts.
- measured: three closed value equations and an application of the
  public law.
- claimed: for this fixture, `least_k` commutes with mapping when the
  source selection uses the composed cost.
- gap: this closed instance does not establish the generic law for
  arbitrary types, costs, mappings, bounds, or lists.
- controls: `v_mapped` and `v_composed` pin the two law sides;
  `v_uncomposed` independently pins the result when composition is
  omitted.
- why: the three distinct costs select `2` before mapping in the
  composed route, but select `0` before mapping in the uncomposed route.

## Refuse an uncomposed cost

Use a fresh copy of the complete accepted client above and append only
the false claim below. Its right-hand `least_k` uses `identity_cost`
instead of `comp Nat Nat Nat identity_cost reverse_three`; everything
else in the client is unchanged. The expected result is
`KernelRejected TypeMismatch` at the law application, within the
`refused_uncomposed_cost` declaration and including `least_k_map`.
Do not pin a numeric span or GlobalIds.

```ken
import Algorithm.Optimization.Argmin (least_k, least_k_map)
import Data.Collections.Derived (map)
import Core.Function.Combinators (comp)

const one : Nat = Suc Zero
const two : Nat = Suc one

fn identity_cost (n : Nat) : Nat = n

fn reverse_three (n : Nat) : Nat =
  match n {
    Zero ↦ two;
    Suc m ↦
      match m {
        Zero ↦ one;
        Suc r ↦ Zero
      }
  }

const xs : List Nat = Cons Nat Zero (Cons Nat one (Cons Nat two (Nil Nat)))
const zero_only : List Nat = Cons Nat Zero (Nil Nat)
const two_only : List Nat = Cons Nat two (Nil Nat)

theorem v_mapped : Equal (List Nat) (least_k Nat identity_cost one (map Nat Nat reverse_three xs)) zero_only = Refl
theorem v_composed : Equal (List Nat)
    (map Nat Nat reverse_three (least_k Nat (comp Nat Nat Nat identity_cost reverse_three) one xs)) zero_only = Refl
theorem v_uncomposed : Equal (List Nat)
    (map Nat Nat reverse_three (least_k Nat identity_cost one xs)) two_only = Refl

theorem accepted_map : Equal (List Nat)
    (least_k Nat identity_cost one (map Nat Nat reverse_three xs))
    (map Nat Nat reverse_three (least_k Nat (comp Nat Nat Nat identity_cost reverse_three) one xs)) =
  least_k_map Nat Nat reverse_three identity_cost one xs

theorem refused_uncomposed_cost : Equal (List Nat)
    (least_k Nat identity_cost one (map Nat Nat reverse_three xs))
    (map Nat Nat reverse_three (least_k Nat identity_cost one xs)) =
  least_k_map Nat Nat reverse_three identity_cost one xs
```

### `stdlib/optimization/reject-least-k-map-uncomposed-cost`

- spec: §4.8, the right-hand selection uses `comp cost f`.
- promise class: durable invariant.
- given: the same fixture and complete accepted prefix as
  `least-k-map-values`, followed by `refused_uncomposed_cost`.
- expect: the prefix accepts; the added claim is rejected with
  `KernelRejected TypeMismatch` at its `least_k_map` application.
- measured: the accepted law application and all three value controls
  succeed before the false claim.
- claimed: the mapped selection law requires the composed source cost.
- gap: this one closed counterexample does not characterize every
  incorrect law type or body.
- controls: `accepted_map` accepts with the composed cost;
  `v_uncomposed` independently fixes the false claim's right-side
  value as `[2]` rather than `[0]`.
- why: only the right-side cost argument in the added claim differs
  from `accepted_map`, and the fixture makes that difference visible.

## Evidence boundary

The accepted and refused rows are transcribed from Architect AC-0's
measured seed rows. This seed has not been checked against the
Foundation implementation child or run by the official conformance
runner. Its Ken fences may be formatter-checked, but that does not
establish semantic execution. No package test, CI, publication,
landing, or landed-trust measurement is claimed.
