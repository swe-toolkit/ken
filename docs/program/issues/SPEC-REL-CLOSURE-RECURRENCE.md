---
id: SPEC-REL-CLOSURE-RECURRENCE
title: "spec 58 §7 (CAT-4, Fork B) fixes the outer transitive-closure formula R+ x y := IsTrue(reachableWithin N x y), N := size(dom R), but NEVER defines what one unit of reachableWithin fuel recognizes (zero = base step? reflexive+step? direct-edge?) -- and its 58:368-370 'simple path length <= N-1' bound is off by one when dom R is the outer-key set and the endpoint is a target-only sink; Foundation cannot choose the recurrence in code, so the draft must state the exact fuel convention and correct N-1 to N"
status: active
owner: spec
size: S
gate: none
depends_on: []
blocks: [CAT-REL-TRANSITIVE-CLOSURE]
github: null
origin: "Steward cut 2026-09-12 from the Architect HS1 ruling on CAT-REL-TRANSITIVE-CLOSURE (evt_16w35x7zxj442, classification BOUNDED_CLOSURE_FUEL_CONVENTION_UNSPECIFIED_AND_BOUND_PROSE_OFF_BY_ONE, grounded at exact main b7c829c9f against draft spec 58 + Map.ken.md). The Architect ruled the computational node HELD until this small spec correction lands, and stated the exact contract to write (below). It gates CAT-REL-TRANSITIVE-CLOSURE (now held on it). Draft-spec-currency correction only: no TCB, no scope fork, no new capability -- fenced, spec-enclave-owned."
---

> # SPEC CORRECTION owed to the enclave (Architect HS1 evt_16w35x7zxj442). It
> # GATES the computational closure node [[CAT-REL-TRANSITIVE-CLOSURE]], held
> # until this lands. Draft-v0 currency fix (define the recurrence + correct an
> # off-by-one); no TCB, no operator touch. Architect is the design authority
> # (the contract below is the Architect's) + CV; Steward M1-M4 -> lieutenant.

## What this is

`spec/50-stdlib/58-maps-sets-relations.md §7` (CAT-4, Fork B) fixes the OUTER
closure formula (58:362) but leaves TWO things underspecified that Foundation
must not decide in code:

1. **The `reachableWithin` fuel recurrence is undefined.** 58:362 gives
   `R+ x y := IsTrue(reachableWithin N x y)`, `N := size(dom R)`, but never says
   what one unit of fuel recognizes. Zero could be a base step, reflexive+step,
   or a direct edge -- these are different relations.
2. **The `N - 1` simple-path bound (58:368-370) is off by one** for the
   outer-key-set interpretation of `dom R`. Counterexample: `r = {a -> {b}}`
   with `b` a target-only sink -- `size(dom r) = 1`, but the valid simple path
   `a -> b` has one edge, not zero.

## The exact contract to state (Architect ruling)

**Signatures** (existing snake-case Map API):

```text
size            : (k v : Type) -> Tree k v -> Nat
dom             : (k v : Type) -> Tree k v -> Tree k Unit
reachable_within: (k : Type) -> (leq : k -> k -> Bool) -> (fuel : Nat)
                  -> (x y : k) -> (r : Tree k (Tree k Unit)) -> Bool
reachable_plus  : (k : Type) -> (leq : k -> k -> Bool)
                  -> (x y : k) -> (r : Tree k (Tree k Unit)) -> Prop
```

**`dom`** is exactly the outer key set: structurally replace every node value by
`MkUnit`, preserving `Leaf`/`Node`, key, and subtree shape. It does NOT union
relation targets into the domain and needs no comparator.

**`size`** counts raw `Tree` nodes: `Leaf ↦ Zero`;
`Node l _ _ r ↦ Suc (add (size l) (size r))`, reusing canonical
`Data.Numeric.Nat.Arithmetic.add`.

**Fuel recurrence.** With `edge_R x y = set_member k leq y (succ k leq x r)`:

```text
reachable_within 0       x y r = False
reachable_within (Suc n) x y r =
  edge_R x y
  OR
  fold OR False { reachable_within n z y r | z ∈ succ k leq x r }
```

Every recursive call decreases exactly `n`, including in the fold step. The
existential may use the landed `fold` + `cat4_bool_or`; no general
bounded-iteration interface, no duplicate Bool disjunction.

**Semantics: fuel is the maximum positive path length.** Fuel 0 recognizes no
path; fuel 1 exactly a direct edge; fuel 2 additionally a two-edge path; `x = y`
is NOT accepted by reflexivity, but IS accepted when a positive self-loop or
cycle exists. That is `R+`, not reflexive-transitive `R*`. Reject BOTH
alternatives the draft leaves open: `Zero = direct edge` (shifts every bound by
one) and `Zero = (x == y)` (changes the relation to `R*`).

**Public predicate:**

```text
reachable_plus k leq x y r =
  IsTrue (reachable_within k leq (size k Unit (dom k (Tree k Unit) r)) x y r)
```

`IsTrue` may unfold to the existing `Equal Bool _ True`; it carries no path
witness and introduces no proof-relevant closure inductive.

## The bound-prose correction

With `dom r` the outer-key set, the correct bound is **`length <= N`**, not
`N - 1`: in a positive simple path `v0 -> v1 -> ... -> vm`, each of `v0 ...
v(m-1)` has an outgoing edge and so is in the outer domain; those sources are
distinct, so `m <= |dom r|` (the terminal vertex may be outside the domain). The
computational bound `N := size(dom r)` is unchanged; only the `N - 1`
explanatory/law premise (58:368-370) becomes `N`.

**Alternative the enclave may take instead (materially different, NOT
recommended):** keep the familiar `N - 1` by counting the complete vertex
support `sources ∪ targets`, naming that op `vertices` (not `dom`), and changing
the public formula. The Architect recommends the smaller `N` correction above,
which matches the current frame and adjacency API.

## Deliverables

1. Edit 58 §7 to state the `reachable_within` fuel recurrence and semantics
   exactly as above, and the `size`/`dom`/`reachable_plus` signatures.
2. Correct 58:368-370 `N - 1` -> `N` (outer-domain interpretation), or take the
   `vertices` alternative and change the public formula accordingly.
3. Keep the closure encoding Ω-sound (Π-into-Ω over a decidable bounded `Bool`;
   no raw `data ... : Ω`).

## Acceptance criteria

- A reader can implement `reachable_within` from 58 §7 alone, with no remaining
  choice between base-step / reflexive+step / direct-at-zero.
- The bound premise is internally consistent with `N := size(dom r)` (no
  off-by-one against the target-only-sink case).
- No TCB growth, no new capability -- a currency/definitional correction only.
- CV confirms the closure form (58:362) and Ω-soundness (58:428-431) are
  unchanged; only the recurrence and the bound premise are stated/corrected.

## Not this node

- The `.ken` computational realization ([[CAT-REL-TRANSITIVE-CLOSURE]], which
  consumes this correction).
- The faithfulness/saturation LAWS (the deferred fast-follow).

## Sizing / tier

**Size S, tier T1.** A small edit, but it fixes a definitional gap and an
off-by-one in a normative contract; reviewed as a correctness statement.
