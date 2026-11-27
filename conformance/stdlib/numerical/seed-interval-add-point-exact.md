# Shifting by a point is exact

Format: `../../README.md`.
Spec: `spec/50-stdlib/68-numerical.md` §24, per AC-0.
Frame: `docs/program/issues/SPEC-NUMERICAL-INTERVAL-ADD-POINT-EXACT.md`.
Architect AC-0: `evt_1b1n2fsh3bvws`.

This seed is authored but unexecuted. No claim is made that the official
conformance runner has executed it. Its discriminating accepted row is the
open-`x` converse witness. The closed `x = 0` exclusion is only a usage
example: D0 measured that it computes without the law.

The fixtures set `base = [1, 2]`, `t = 1`, and
`shifted = base + [1, 1] = [2, 3]`. The law says membership of `x + 1`
in `shifted` is equivalent to membership of `x` in `base`.

## Accepted client

The accepted converse witness is open in `x`. It uses symmetry of the
public law to turn membership in the shifted interval into membership in
the base interval. The closed exclusion illustrates law use but does not
discriminate the law: the same closed result is derivable without it.

```ken
import Data.Numeric.Rational
  (Rational, MkRational, rational_add, rational_zero)
import Algorithm.Numeric.RationalInterval
  (Interval, MkInterval, interval_add, interval_point,
   interval_contains, interval_add_point_exact)

const one : Rational = MkRational (Suc Zero) Zero
const two : Rational = MkRational (Suc (Suc Zero)) Zero
const base : Interval = MkInterval one two
const shifted : Interval = interval_add base (interval_point one)

theorem shifted_member_unshifts (x : Rational)
    : IsTrue (interval_contains shifted (rational_add x one))
      → IsTrue (interval_contains base x) =
  λmember.
    trans
      Bool
      (interval_contains base x)
      (interval_contains shifted (rational_add x one))
      True
      (sym
        Bool
        (interval_contains shifted (rational_add x one))
        (interval_contains base x)
        (interval_add_point_exact base one x))
      member

theorem shifted_excludes_one
    : Equal Bool (interval_contains shifted (rational_add rational_zero one)) False =
  trans
    Bool
    (interval_contains shifted (rational_add rational_zero one))
    (interval_contains base rational_zero)
    False
    (interval_add_point_exact base one rational_zero)
    Proved
```

### stdlib/numerical/interval-add-point-exact-open-converse

- spec: §24, `interval_add_point_exact`.
- promise class: durable invariant.
- given: `base = [1, 2]`, `shifted = base + [1, 1]`, and open `x`.
- expect: `shifted_member_unshifts` accepts.
- measured: the term applies symmetry of the public law at open `x`.
- claimed: membership of `x + 1` in the shifted interval implies
  membership of `x` in the base interval.
- gap: this client does not test the law's proof body.
- controls: the closed `x = 0` exclusion is a usage example, not this
  case's discriminator.
- why: the open witness uses the converse direction; enclosure alone
  supplies only the forward direction.

### stdlib/numerical/interval-add-point-exact-closed-usage

- spec: §24, `interval_add_point_exact`.
- promise class: durable invariant.
- given: `x = 0`, `base = [1, 2]`, and `shifted = [2, 3]`.
- expect: `shifted_excludes_one` accepts with `False`.
- measured: the law is applied to rewrite the membership test to
  membership of zero in `base`.
- claimed: this is a closed usage example for the law.
- gap: it does not discriminate the law; D0 measured the same closed
  result accepted without applying the law.
- why: every fixture term is closed, so computation alone can close it.

## Isolated refused applications

For each refusal below, start from a fresh complete copy of the accepted
client above and append only that refusal. Require a declaration-level
`KernelRejected(TypeMismatch)` within the refusal's own theorem span.
Do not pin a token span or exact diagnostic wording. The two refusals
must be tested in separate fresh prefixes.

### stdlib/numerical/reject-shifted-one-from-base-one

- spec: §24, `interval_add_point_exact`.
- promise class: durable invariant.
- given: the accepted client with `base = [1, 2]` and `shifted = [2, 3]`.
- expect: the appended theorem refuses inside its declaration with
  `KernelRejected(TypeMismatch)`.
- measured: the claim drops the shift from the queried value; the law
  applied at `x = 1` concerns `shifted` membership of `1 + 1`, not `1`.
- claimed: the shifted interval does not inherit membership of the
  unshifted point.
- gap: one closed value does not establish all shifts.
- controls: `shifted_member_unshifts` accepts with the shift retained;
  the refused theorem has its own fresh accepted prefix.
- why: `base` contains `1`, but `shifted` does not contain `1`.

Append only:

```ken
theorem shifted_contains_one : IsTrue (interval_contains shifted one) =
  trans
    Bool
    (interval_contains shifted one)
    (interval_contains base one)
    True
    (interval_add_point_exact base one one)
    Proved
```

### stdlib/numerical/reject-open-dropped-shift

- spec: §24, `interval_add_point_exact`.
- promise class: durable invariant.
- given: a fresh accepted client with open `x`.
- expect: the appended theorem refuses inside its declaration with
  `KernelRejected(TypeMismatch)`.
- measured: the claim drops the shift while applying the law at open `x`.
- claimed: membership of `x` in the base does not imply membership of
  that same `x` in the shifted interval.
- gap: this one theorem does not prove the law's body.
- controls: the accepted `shifted_member_unshifts` uses the same open
  variable and conclusion shape with the required `x + 1` argument.
- why: the public law relates `shifted` membership at `x + 1` to base
  membership at `x`; it does not justify `shifted` membership at `x`.

Append only:

```ken
theorem shifted_contains_unshifted (x : Rational)
    : IsTrue (interval_contains base x)
      → IsTrue (interval_contains shifted x) =
  λmember.
    trans
      Bool
      (interval_contains shifted x)
      (interval_contains base x)
      True
      (interval_add_point_exact base one x)
      member
```

## Evidence boundary

Architect AC-0's scratch development checked the law and the seed's
client forms. It measured the open converse witness accepted, both dropped-shift
twins refused with `KernelRejected(TypeMismatch)` at their declarations,
and the closed exclusion accepted without the law. Those checks do not
execute this Markdown seed. No official conformance run, assembled AC-T,
CI, publication, or landing claim is made.
