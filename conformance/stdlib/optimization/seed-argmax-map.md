# Argmax commutes with a map

Format: `../../README.md`.
Spec: `spec/50-stdlib/72-optimization.md` §3.9.
Frame: `docs/program/issues/SPEC-OPTIMIZATION-ARGMIN-MAP.md`.
Architect AC-0: `evt_1f2gafx118qj2`.

This seed is authored but unexecuted. No claim is made that the official
conformance runner has executed it.

The fixture uses `a = b = Nat`, identity cost,
`xs = [0, 1, 2]`, and `reverse_three` mapping `0 ↦ 2`, `1 ↦ 1`,
`2 ↦ 0`. The mapped argmax and the composed-cost route both yield `2`;
the uncomposed-cost route yields `0`. These closed values exercise the
law's cost composition but do not establish the generic theorem.

Use `Proved` for closed `Equal Nat` goals. AC-0 measured that these
goals reduce successfully with `Proved`; `Refl` refuses even true closed
`Equal Nat` goals here.

## Accepted client

The complete block accepts. It checks the three images, the mapped
argmax, the composed-cost result, and the independent uncomposed result
before applying the public law.

```ken
import Algorithm.Optimization.Argmin (argmax, argmax_map)
import Data.Collections.NonEmpty (NonEmpty, nonempty_cons, nonempty_map)
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

const xs : NonEmpty Nat =
  nonempty_cons Nat Zero (Cons Nat one (Cons Nat two (Nil Nat)))

theorem image_zero : Equal Nat (reverse_three Zero) two = Proved
theorem image_one : Equal Nat (reverse_three one) one = Proved
theorem image_two : Equal Nat (reverse_three two) Zero = Proved

theorem mapped_result :
    Equal Nat
      (argmax Nat identity_cost (nonempty_map Nat Nat reverse_three xs))
      two = Proved

theorem composed_result :
    Equal Nat
      (reverse_three
        (argmax Nat
          (comp Nat Nat Nat identity_cost reverse_three)
          xs))
      two = Proved

theorem uncomposed_result :
    Equal Nat
      (reverse_three (argmax Nat identity_cost xs))
      Zero = Proved

theorem accepted_map :
    Equal Nat
      (argmax Nat identity_cost (nonempty_map Nat Nat reverse_three xs))
      (reverse_three
        (argmax Nat
          (comp Nat Nat Nat identity_cost reverse_three)
          xs)) =
  argmax_map Nat Nat reverse_three identity_cost xs
```

### stdlib/optimization/argmax-map-values

- spec: §3.9, selection of mapped candidates under the composed cost.
- promise class: durable invariant.
- given: `[0, 1, 2]`, identity cost, and `reverse_three` mapping the
  candidates to `[2, 1, 0]`.
- expect: the mapped argmax and composed-cost route both equal `2`; the
  uncomposed-cost route equals `0`; `accepted_map` accepts.
- measured: the three images, both sides of the law, the uncomposed
  result, and an application of the public law.
- claimed: for this fixture, mapping the argmax agrees with selecting
  under the cost composed with the map.
- gap: this closed instance does not establish the generic law for
  arbitrary carriers, maps, costs, or nonempty inputs.
- controls: the separate composed and uncomposed result claims expose the
  cost distinction; the image claims pin the map on each candidate.
- why: the reverse map makes the greatest source candidate become the
  least image, while composition selects the source whose image has
  greatest cost.

## Refuse an uncomposed cost

On a fresh full copy of the accepted client, append only the false law
application below. Its RHS uses the uncomposed identity cost instead of
`comp Nat Nat Nat identity_cost reverse_three`. The accepted prefix must
remain successful; this declaration is expected to refuse with
`KernelRejected(TypeMismatch)` at the `argmax_map` application. Do not
pin a numeric span or GlobalIds.

```ken
theorem refused_uncomposed_cost :
    Equal Nat
      (argmax Nat identity_cost (nonempty_map Nat Nat reverse_three xs))
      (reverse_three (argmax Nat identity_cost xs)) =
  argmax_map Nat Nat reverse_three identity_cost xs
```

### stdlib/optimization/reject-argmax-map-uncomposed-cost

- spec: §3.9; the right-hand selection uses `comp cost f`.
- promise class: durable invariant.
- given: the accepted fixture and its full accepted prefix, followed by
  `refused_uncomposed_cost`.
- expect: the prefix accepts; the appended law application refuses with
  `KernelRejected(TypeMismatch)`.
- measured: the law application with the composed cost succeeds in the
  prefix; only the appended RHS drops composition.
- claimed: the mapped-selection law requires the composed cost.
- gap: this one finite fixture does not characterize every incorrect law
  type or implementation.
- controls: `accepted_map` applies the law with composition; the prefix
  independently establishes that the uncomposed result is `0`, not `2`.
- why: the uncomposed RHS has a different result from the mapped argmax.

## Refuse a false closing value

On another fresh full copy of the accepted client, append only the false
closed claim below. The expected refusal is in this declaration, after
the accepted controls; do not pin a numeric span or GlobalIds.

```ken
theorem refused_false_value :
    Equal Nat
      (argmax Nat identity_cost (nonempty_map Nat Nat reverse_three xs))
      Zero = Proved
```

### stdlib/optimization/reject-argmax-map-false-value

- spec: §3.9; the mapped selection has the value fixed by the accepted
  controls.
- promise class: durable invariant.
- given: the same accepted client, followed by `refused_false_value`.
- expect: the accepted prefix succeeds; the false closing value claim
  refuses.
- measured: the mapped result is independently established as `2`,
  while the appended claim asserts `0`.
- claimed: the mapped argmax is not the uncomposed route's value.
- gap: this closed result is not a proof of the generic mapping law.
- controls: `mapped_result` establishes the same expression as `2`, and
  `uncomposed_result` establishes the distinct value `0`.
- why: the closing claim uses the same expression and differs only in
  asserting the uncomposed result.

## Evidence boundary

The cases are authored expectations based on Architect AC-0, not executed
seed results. There is no official conformance-runner, CI, publication,
landing, or landed-trust claim.
