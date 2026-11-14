# C5 — verified `sort` and count-equality `Perm`

**Axis:** proof-carrying / verified programs. **Flavor:** B. The two original
sources keep C5's emission contract; a separate closed pair supplies the
kernel-decided accept/refuse split required by AC-2. The checked subset-Σ pair
shape remains deferred to W5 while carrier-only lowering is current.

## Contract and imports

The `Perm` pin follows `57 §3.1`: count equality over an explicit comparator,
not the old prelude truncation relation. The original C5 sources import `Perm`,
`is_sorted`, and `eq_from_ord` from `Data.Collections.Derived`; for sort, the
equality comparator is `eq_from_ord a leq`. They also import `And` from
`Core.Logic.Conjunction`.

The sort refinement is
`{ ys : List a | And (is_sorted a leq ys)
(Perm a (eq_from_ord a leq) ys xs) }`.
`is_sorted` asserts ordering; `Perm` asserts that output and input have equal
element counts under the comparator. `is_sorted` alone is vacuous for `const
Nil`.

## Original source pair: emission and never proved

- **Sound source — `sound-verified-sort.ken`.** `ken check` returns rc 0 with
two open refinement obligations. The emitted obligations contain both
`is_sorted` and count-equality `Perm`; neither is reported proved. Full proof
discharge remains a known gap.
- **Unsound source — `unsound-const-nil.ken`.** `ken check` returns rc 0 with
one open refinement obligation containing `Perm a (eq_from_ord a leq) Nil xs`.
It is never proved. The open hole is not an elaboration refusal or a proof of
the sort claim (`21 §6.5`).

These are the C5 contract's emission-plus-never-proved observations. In the
local CLI, rc 0 does not distinguish them: open obligations are reported as
`status unknown`. Do not treat that exit code as PASS or REFUSE.

## Closed kernel-decided pair

Both files use `always`, a constant-`True` comparator. Thus
`eq_from_ord Bool always x y` reduces to `True` for every `x` and `y`.

- **Positive — `sound-closed-count.ken`.** `Perm` compares the counts of the
  same one-element list. Each count reduces to `Suc Zero`; the equality
  reduces to `Top`, and `λx. Proved` closes it. Expected: `ken check` rc 0 and
  no open obligations.
- **Negative — `unsound-closed-const-nil.ken`.** `Perm` compares `Nil` with a
  one-element list. The counts reduce to `Zero` and `Suc Zero`; their equality
  reduces to `Bottom`. Expected: `ken check` rc 1 with
  `KernelRejected TypeMismatch` inside `const_nil_is_perm`.

This closed pair carries the accept/refuse split. A `Perm` implementation that
ignored counts would accept the negative arm, so the pair discriminates the
count property rather than imports or spelling.

## Mutation control

In a scratch copy of `unsound-closed-const-nil.ken`, change only its second list
from `Cons Bool True (Nil Bool)` to `Nil Bool`. The theorem then compares two
empty lists and is expected to pass with rc 0. This mutation checks that the
negative arm refuses because its counts differ. Do not alter the repository
source for this control; preserve and verify its bytes before and after the
scratch run.

## Surface-expressibility note

The imported identities, argument order, and comparator are part of the
fixtures. Do not substitute the old prelude `Perm`, which has a different
signature. The open-obligation source pair and the closed theorem pair make
different claims and must retain separate verdicts.
