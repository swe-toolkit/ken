# Multiplication by a positive point preserves membership

Format: `../../README.md`.
Spec: `spec/50-stdlib/68-numerical.md` §25, per Architect AC-0.
Frame: `docs/program/issues/SPEC-NUMERICAL-INTERVAL-MUL-POINT-EXACT.md`.
Architect AC-0: `evt_3s9zqsqyw97w9`.
Base: `88a611593ec6dbe4cded42d1ad43847d6a12eaed`.

The law requires the point `t` to have nonzero numerator. It says that
membership of `x * t` in `i * [t, t]` is equivalent to membership of
`x` in `i`. The accepted client keeps `x` open; the twin at `t = 0`
does not satisfy the law's premise.

The fixtures use `base = [1, 2]`, `t = 2`, `scaled = [2, 4]`,
`zero_scaled = [0, 0]`, and `five = 5`.

## Accepted client and closed controls

The accepted converse witness is open in `x`. The two closed controls
do not cite the law: zero scaling makes `5 * 0` a member of
`zero_scaled`, while `5` is not a member of `base`.

```ken
import Core.Classes.LawfulClasses (IsTrue)
import Core.Logic.Transport (sym, trans)
import Data.Numeric.Rational
  (Rational, MkRational, rational_mul, rational_zero)
import Algorithm.Numeric.RationalInterval
  (Interval, MkInterval, interval_mul, interval_point,
   interval_contains, interval_mul_point_exact)

const one : Rational = MkRational (Suc Zero) Zero
const two : Rational = MkRational (Suc (Suc Zero)) Zero
const five : Rational = MkRational (Suc (Suc (Suc (Suc (Suc Zero))))) Zero
const base : Interval = MkInterval one two
const scaled : Interval = interval_mul base (interval_point two)
const zero_scaled : Interval = interval_mul base (interval_point rational_zero)

theorem scaled_member_unscales (x : Rational)
    : IsTrue (interval_contains scaled (rational_mul x two))
      → IsTrue (interval_contains base x) =
  λmember.
    trans
      Bool
      (interval_contains base x)
      (interval_contains scaled (rational_mul x two))
      True
      (sym
        Bool
        (interval_contains scaled (rational_mul x two))
        (interval_contains base x)
        (interval_mul_point_exact base two x Proved))
      member

theorem zero_scaled_contains_five_times_zero
    : IsTrue (interval_contains zero_scaled (rational_mul five rational_zero)) =
  Proved

theorem base_excludes_five
    : Equal Bool (interval_contains base five) False =
  Proved

theorem premise_free_scaling_is_false
    : ((t : Rational) → (x : Rational)
        → Equal Bool
          (interval_contains (interval_mul base (interval_point t)) (rational_mul x t))
          (interval_contains base x))
      → Equal Bool True False =
  λclaim.
    trans
      Bool
      True
      (interval_contains base five)
      False
      (trans
        Bool
        True
        (interval_contains zero_scaled (rational_mul five rational_zero))
        (interval_contains base five)
        (sym
          Bool
          (interval_contains zero_scaled (rational_mul five rational_zero))
          True
          zero_scaled_contains_five_times_zero)
        (claim rational_zero five))
      base_excludes_five
```

### stdlib/numerical/interval-mul-point-exact-open-converse

- spec: §25, `interval_mul_point_exact`.
- promise class: durable invariant.
- given: `base = [1, 2]`, `scaled = [2, 4]`, `t = 2`, and open `x`.
- expect: `scaled_member_unscales` accepts and applies the public law.
- measured: Architect AC-0 checked this open client in scratch.
- claimed: membership of `2 * x` in `[2, 4]` implies membership of
  `x` in `[1, 2]`.
- gap: this fixed interval and point do not check the generic law's body.
- why: the law supplies the converse direction for open `x`; the
  premise-free claim is separately refuted below.

### stdlib/numerical/interval-mul-point-exact-zero-controls

- spec: §25 and the zero-point boundary.
- promise class: durable invariant.
- given: `base = [1, 2]`, `zero_scaled = base * [0, 0]`, and `five = 5`.
- expect: `zero_scaled_contains_five_times_zero` accepts, and
  `base_excludes_five` accepts with `False`; neither theorem cites
  `interval_mul_point_exact`.
- measured: Architect AC-0 checked both closed values in scratch.
- claimed: zero scaling admits `5 * 0` in the zero interval, while the
  base interval excludes `5`.
- gap: these closed values do not check the generic positive-point law.
- why: they form the counterexample used by the premise-free falsity
  arm and do not rely on a refusal diagnostic.

### stdlib/numerical/interval-mul-point-exact-premise-free-falsity

- spec: §25, `interval_mul_point_exact` premise.
- promise class: durable invariant.
- given: the accepted block with `premise_free_scaling_is_false`.
- expect: the client accepts and derives `Equal Bool True False` from
  the hypothetical premise-free scaling claim, without citing the law.
- measured: Architect AC-0 checked this client in scratch.
- claimed: the nonzero-numerator premise is necessary for the equality.
- gap: the counterexample fixes `base` and uses the point `0`.
- controls: the closed zero controls establish the opposite membership
  results used in the derivation.
- why: the hypothetical claim at `t = 0`, `x = 5` contradicts the
  independent closed values.

## Refuse the zero-point twin

Start from a fresh copy of the accepted block above and append only the
twin below. Its failed premise diagnoses that the law cannot be applied
at `t = 0`; the refusal itself does not establish the claimed conclusion
false. The closed controls and premise-free falsity arm carry that
counterexample.

```ken
theorem zero_scaled_member_unscales (x : Rational)
    : IsTrue (interval_contains zero_scaled (rational_mul x rational_zero))
      → IsTrue (interval_contains base x) =
  λmember.
    trans
      Bool
      (interval_contains base x)
      (interval_contains zero_scaled (rational_mul x rational_zero))
      True
      (sym
        Bool
        (interval_contains zero_scaled (rational_mul x rational_zero))
        (interval_contains base x)
        (interval_mul_point_exact base rational_zero x Proved))
      member
```

### stdlib/numerical/interval-mul-point-exact-zero-premise-refusal

- spec: §25, especially the nonzero-numerator premise.
- promise class: durable invariant.
- given: a fresh accepted block with `zero_scaled_member_unscales`.
- expect: `KernelRejected(TypeMismatch)` within the declaration, at the
  unsatisfied premise argument `Proved`; do not pin a token span.
- measured: Architect AC-0 observed this refusal in scratch.
- claimed: `interval_mul_point_exact` cannot be applied at `t = 0`.
- gap: the refusal does not prove the conclusion false.
- controls: the zero closed values and `premise_free_scaling_is_false`
  independently establish the counterexample.
- why: `t = 0` does not satisfy the law's nonzero-numerator premise;
  falsity comes from the controls, not the diagnostic.

## Evidence boundary

Architect AC-0 (`evt_3s9zqsqyw97w9`) reports that the law and generic
open client checked in scratch on landed base
`88a611593ec6dbe4cded42d1ad43847d6a12eaed`, with no added `Axiom` and
equal exact trusted `GlobalId` sets (114 → 114). D0 also measured the
open client accepted, the zero controls accepted, the premise-free
falsity arm accepted, and the zero-point twin refused with
`KernelRejected(TypeMismatch)` at its premise argument.

These are D0 observations, not execution of this authored Markdown seed.
CV ran no tests. The seed has not been executed by the official
conformance runner. No assembled AC-T, CI, publication, landing, or
measured landed-trust result is claimed.

Clean-room provenance: no `local/refs/` or external reference
implementations were consulted. This seed is grounded in the settled
AC-0 ruling and first principles.
