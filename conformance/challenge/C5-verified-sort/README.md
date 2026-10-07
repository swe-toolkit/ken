# C5 — verified `sort` (`isSorted ∧ Perm`, `Perm` at the right universe)

**Axis:** proof-carrying / verified programs. **Flavor:** B (should-PASS on
emission; full discharge is a known-gap). **Coupled with C2** — the `Perm`
conjunct must sit at the universe C2 establishes (`‖Perm‖` / count-equality,
never a proof-relevant Ω inductive).

## Why this is a blind spot

VAL2's `merge-sort` was an ordinary `Ord` sort with **no verification**. Ken's
distinctive value is that a `sort` can carry its own correctness: the result
type is the checked subset Σ
`Σ(ys : List a).isSorted leq ys ∧ Perm ys xs`, and the elaboration introduces
a pair whose proof component carries the conjoined obligation. The load-bearing
subtlety: `Perm` forces `sort` to be a sort. `isSorted` alone is
**vacuous** (`const Nil` satisfies "the output is sorted"; the empty list is
sorted). Dropping `Perm` is a verification-soundness omission the kernel does
not catch.

## The pair

- **Sound arm — `sound-verified-sort.ken` — should-PASS (emission).** The
  explicit-comparator `sort (a) (leq) (xs)` has result type
  `Σ(ys : List a).And (isSorted a leq ys) (Perm a ys xs)` (the subset-Σ
  elaboration of the surface refinement). Result introduction constructs the
  pair; its proof component carries **both** conjuncts. (Grounded: this exact
  view type-checks in `es2_acceptance.rs`; `isSorted`/`Perm` are real defs that
  unfold, not postulates.)
- **Unsound/stub arm — `unsound-const-nil.ken` — remains unproved.**
  `sortBad _ _ _ = Nil` claims the **same** subset-Σ result type. It elaborates
  to a pair whose proof component carries both conjuncts; `isSorted leq Nil`
  holds vacuously, but `Perm Nil xs` is false for non-empty `xs`, so that proof
  obligation remains open.

## Expected behavior (exact)

- Sound arm: **PASS on emission** — the result is a checked pair at the subset
  Σ type; its proof component emits `And (isSorted a leq (sort …))
  (Perm a (sort …) xs)`, with **both conjuncts present**. Full proof discharge
  (the prover closing `Perm (insert …) …`) is a **known-gap** — the verified-
  sort proof term / prover discharge is not fully landed; emission-with-both-
  conjuncts is the checkable property today.
- Unsound arm: `sortBad` still **elaborates** with a checked pair and an open
  proof hole; it is **not proved** because `Perm a Nil xs` cannot be discharged
  for non-empty `xs` (count mismatch / no permutation witness). Open holes are
  admitted typed postulates (`21 §6.5`), not elaboration rejection. If the
  verifier reports `proved`, that is the finding: `Perm` was dropped or the
  obligation was not enforced.

## Discriminates

Is the `Perm` conjunct present and enforced? The real `sort` has a discharged
proof with both conjuncts; `const Nil` leaves the `Perm` proof open although its
`isSorted` conjunct is satisfiable. This is the flip. A `sort` test that
asserted only `isSorted` would be green-vs-green — `const Nil` passes it. This
is the refinement-completeness discriminator,
promoted from the `sort-emits-issorted-and-perm` seed to a full verified
program.

## Surface-expressibility note

The surface refinement `{ ys : List a | And (isSorted a leq ys) (Perm a ys xs) }`
elaborates to the subset `Σ` with an explicit proof component (`21 §2`). It is
landed surface (`es2_acceptance.rs`). Whether the emitted obligation is
**discharged** (vs merely emitted) depends on the prover; if discharge isn't
reachable, record "emits both conjuncts; discharge deferred" — the emission
completeness (both conjuncts, `Perm` present) is the load-bearing check either
way. `Perm` must be the C2 `‖Perm‖`/count-equality form, not a proof-relevant Ω
inductive.
