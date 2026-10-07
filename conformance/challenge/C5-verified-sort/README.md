# C5 — verified `sort` (`isSorted ∧ Perm`, `Perm` at the right universe)

**Axis:** proof-carrying / verified programs. **Flavor:** B (should-PASS on
emission; full discharge is a known-gap). **Coupled with C2** — the `Perm`
conjunct must sit at the universe C2 establishes (`‖Perm‖` / count-equality,
never a proof-relevant Ω inductive).

**W5 staging:** obligation emission of both conjuncts remains the C5 target;
the checked subset-Σ result and `Pair` core shape are **deferred — W5** while
the transitional carrier-only implementation remains.

## Why this is a blind spot

VAL2's `merge-sort` was an ordinary `Ord` sort with **no verification**. Ken's
distinctive value is that a `sort` can carry its own correctness: the result
type is the W1 target subset Σ
`Σ(ys : List a).isSorted leq ys ∧ Perm ys xs`, whose W5 pair's proof
component carries the conjoined obligation. The load-bearing
subtlety: `Perm` forces `sort` to be a sort. `isSorted` alone is
**vacuous** (`const Nil` satisfies "the output is sorted"; the empty list is
sorted). Dropping `Perm` is a verification-soundness omission the kernel does
not catch.

## The pair

- **Sound arm — `sound-verified-sort.ken` — should-PASS (emission).** The
  explicit-comparator `sort (a) (leq) (xs)` has result type
  `Σ(ys : List a).And (isSorted a leq ys) (Perm a ys xs)` (the W1 subset-Σ
  target of the surface refinement). The pair shape is **deferred — W5**;
  the emitted obligation carries **both** conjuncts. (Grounded:
  `isSorted`/`Perm` are real definitions that unfold, not postulates.)
- **Unsound/stub arm — `unsound-const-nil.ken` — remains unproved.**
  `sortBad _ _ _ = Nil` claims the **same** W1 subset-Σ result type. Its pair
  shape is **deferred — W5**; `isSorted leq Nil` holds vacuously, but
  `Perm Nil xs` is false for non-empty `xs`, so that proof obligation remains
  open.

## Expected behavior (exact)

- Sound arm: **PASS on emission** — the obligation is
  `And (isSorted a leq (sort …)) (Perm a (sort …) xs)`, with **both conjuncts
  present**. The checked result pair at subset-Σ type is **deferred — W5**.
  Full proof discharge (the prover closing `Perm (insert …) …`) remains a
  known gap; emission with both conjuncts is the checkable property today.
- Unsound arm: `sortBad` emits an open proof hole; the checked subset pair is
  **deferred — W5**. It is **not proved** because `Perm a Nil xs` cannot be
  discharged for non-empty `xs` (count mismatch / no permutation witness).
  Open holes are admitted typed postulates (`21 §6.5`), not elaboration
  rejection. If the verifier reports `proved`, that is the finding: `Perm` was
  dropped or the obligation was not enforced.

## Discriminates

Is the `Perm` conjunct present and enforced? The sound fixture emits both
conjuncts; `const Nil` leaves the `Perm` proof open although its `isSorted`
conjunct is satisfiable. This is the flip. A `sort` test that
asserted only `isSorted` would be green-vs-green — `const Nil` passes it. This
is the refinement-completeness discriminator,
promoted from the `sort-emits-issorted-and-perm` seed to a full verified
program.

## Surface-expressibility note

The surface refinement `{ ys : List a | And (isSorted a leq ys) (Perm a ys xs) }`
has the W1 subset-Σ target with an explicit proof component (`21 §2`); its
pair output is **deferred — W5**, and transitional lowering remains
carrier-only. Whether the emitted obligation is **discharged** (vs merely
emitted) depends on the prover; if discharge isn't reachable, record "emits
both conjuncts; discharge deferred" — the emission completeness (both
conjuncts, `Perm` present) is the load-bearing check either way. `Perm` must be
the C2 `‖Perm‖`/count-equality form, not a proof-relevant Ω inductive.
