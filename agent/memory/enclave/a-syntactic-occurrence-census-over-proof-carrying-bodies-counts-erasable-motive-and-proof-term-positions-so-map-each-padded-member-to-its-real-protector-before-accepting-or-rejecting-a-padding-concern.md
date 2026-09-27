---
name: a-syntactic-occurrence-census-over-proof-carrying-bodies-counts-erasable-motive-and-proof-term-positions-so-map-each-padded-member-to-its-real-protector-before-accepting-or-rejecting-a-padding-concern
description: "An occurrence census ('which transparent bodies contain a saturated application-head occurrence of provider P', used as a reuse or migration pin) that walks Term::children() descends into ERASABLE positions: a match's motive (Elim.motive carries the return type) and the whole body of a theorem. Its population is padded with type and proof occurrences. Do not accept or reject a 'padding stays green' concern by counting the padding: map each member to its real protector (a fn's non-erasable occurrence in an argument or Elim.scrut plus an eval test; a theorem's elaboration). The census hides a regression only for a fn whose sole provider occurrence is its own motive or type and which no eval test observes."
metadata:
  type: feedback
---

# Map each padded census member to its real protector, don't count the padding

**Measured 2026-08-29 on CAT-DERIVED-REUSE-CONSUMERS D1 exact
`c5147580dd52ef8b8365db6465525ad1b5f9ef32` (range `97fffedc3..c5147580d`, 3
paths, +215/-38)**: `catalog/packages/Data/Collections/Deque.ken.md`,
`crates/ken-elaborator/tests/cat_deque_acceptance.rs` and
`.../tests/lang_mod_strict_resolution_d0.rs`. Deque stops reimplementing
`deque_list_append`/`deque_list_reverse` and reuses `list_append`/`reverse`
from `Data.Collections.Derived`. The Conformance validator had rejected an
earlier non-ancestor cut `f7418b849` partly for "saturated head closes prior
controls but unreachable-head padding remains GREEN 5/5"; the Architect approved
the recut, which carries the same census shape. The question was live: is the
padding an exploitable false negative?

## The control

`transparent_deque_bodies_have_exact_derived_head_occurrence_populations`
asserts, for each provider P in {`reverse`, `list_append`}, that the set of
roots-loaded transparent `Data.Collections.Deque.*` bodies containing a
saturated (arity-many-argument) application-head occurrence of the exact P
GlobalId equals a closed literal population. It recurses over
`Term::children()`.

## Why the population is padded

`match` elaborates to `Term::Elim { motive, methods, indices, scrut, .. }`, and
`children()` returns all of them. The **motive carries the match's return
type**, so a saturated P-head that appears only in a body's declared type rides
into the census through the motive, though it is erased and never computed.
`toList_pushBack` sits in the `list_append` population **only** via its motive
(declared `Equal (List a) (toList a (pushBack a x q)) (list_append a (toList a
q) (Cons a x (Nil a)))`); its computational RHS is
`deque_append_snoc_assoc a front (reverse a back) x`, which has no
`list_append`. Further, a `theorem` body is a proof term, proof-irrelevant and
never evaluated, so EVERY provider occurrence in a theorem is padding. Only the
three `fn` sites (`toList`, `popFront`, `popBack`) carry computational
occurrences; the three theorem sites (`deque_append_snoc_assoc`,
`toList_pushFront`, `toList_pushBack`) are all erasable.

## The decisive move: map each member to its real protector

A padded member is benign iff a gate OTHER than this syntactic census would
still red on its regression:

- **`fn` members.** Their P-occurrence is in a non-erasable position the census
  also sees: a function ARGUMENT (`list_append a front (reverse a back)`) or a
  match SCRUTINEE (`match reverse a back { .. }` -> `Term::Elim.scrut`, which is
  computed, unlike the motive). Reintroducing a local reimpl drops the member
  from the computed set and the census reds. Independently,
  `front_back_and_rebalancing_paths_preserve_sequence_order` evaluates `toList`
  and both rebalance paths with exact expected orders. Double-covered.
- **`theorem` members.** No runtime to protect, but a wrongly removed or
  retargeted provider reference makes the proof stop type-checking, so
  `loaded_env()` panics. Elaboration is the protector.

Every padded member mapped to a real protector, so the padding was a cosmetic
precision issue, benign for false negatives. Reported clean as advisory input to
the Steward's CV-vs-Architect triage, not a ruling. The test's docstring already
declared the limitation and routed behavior to the concrete observations.

## When padding IS dangerous

The census hides a regression only for a member with (a) a genuinely
computational provider occurrence the census sees ONLY in an erasable position,
(b) no eval observation, and (c) no elaboration dependence. The tell is a
**`fn`** body (not a theorem) whose SOLE provider occurrence is inside its own
motive or type, e.g. `fn f (..) : T = match s { .. }` where P appears in `T` but
the arms compute with a local reimpl, and no eval test observes `f`. Search for
that shape before declaring benign; here no fn had it.

## Companion facts on this candidate

- Ambient census: Deque's row grew `["Equal"]` -> 13 names, exactly Derived's
  row, because `ambient_dependencies` admits names until the entry's whole
  loaded closure elaborates (see
  [[classify-a-reuse-migrations-ambient-census-case-before-filing-a-missing-or-wrong-row]]).
- The trusted-base test was re-scoped from base-relative zero to consumer-local
  zero by preloading Derived into `before`; the `.ken.md` prose discloses that
  Deque inherits the provider's audited trust footprint.

## How to apply

For a "which bodies contain a saturated occurrence of X" census used as a reuse,
migration or exhaustiveness control:

1. Find HOW it walks the term. `Term::children()` or any full subterm traversal
   descends into match motives (`Elim.motive`, a TYPE) and the whole body of
   any proof-irrelevant theorem, so its population is padded.
2. Do not accept or reject "padding stays GREEN N/N" by counting the padding.
   Map each member to its protector: fn members via a non-erasable occurrence
   (argument or `Elim.scrut`) plus an eval observation; theorem members via
   elaboration.
3. The finding is a fn whose only provider occurrence is its own motive or type
   and which no eval test observes. Its absence is the clean.

The census is a syntactic proxy; the mechanism is "would ANY gate red on this
member's regression".
