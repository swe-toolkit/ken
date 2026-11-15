# Interval width under multiplication

Format: `../../README.md`.
Spec: `spec/50-stdlib/68-numerical.md` §17.
Frame: `docs/program/issues/SPEC-NUMERICAL-INTERVAL-WIDTH-MUL.md` AC-2.

These closed clients instantiate the public ordered-product width law.
They do not establish its generic proof. `MkRational n Zero` denotes the
natural rational `n`. The ordered product `[1,2]·[1,2]` has width three:
its endpoint product is `[1,4]`, while the selected right-hand side is
`2·1 + 1·1 = 3`. The product of the two operand widths is only one.
`IsTrue` is already available in the client prelude; this client does
not import it a second time.

## Accepted client

Check this entire client before appending one isolated refusal. W-APPLY
uses the public theorem, W-THREE independently checks the closed value,
and W-FORM independently checks the selected expression with the same
proposition form and proof terminal as X-DROP-FIRST.

```ken
import Data.Numeric.Rational
  (Rational, MkRational, rational_add, rational_mul, rational_equiv, rational_leq)
import Algorithm.Numeric.RationalInterval
  (Interval, MkInterval, interval_mul, interval_width, interval_width_mul)

const r1 : Rational = MkRational (Suc Zero) Zero
const r2 : Rational = MkRational (Suc (Suc Zero)) Zero
const r3 : Rational = MkRational (Suc (Suc (Suc Zero))) Zero

-- W-APPLY: both intervals are ordered.
theorem width_product_application
  : rational_equiv
      (interval_width (interval_mul (MkInterval r1 r2) (MkInterval r1 r2)))
      (rational_add
        (rational_mul r2 (interval_width (MkInterval r1 r2)))
        (rational_mul r1 (interval_width (MkInterval r1 r2)))) =
  interval_width_mul r1 r2 r1 r2 Proved Proved

-- W-THREE: the product's width has the independently checked value three.
theorem width_product_value
  : rational_equiv
      (interval_width (interval_mul (MkInterval r1 r2) (MkInterval r1 r2))) r3 =
  Proved

-- W-FORM: the selected expression has the product's width on this input.
theorem width_product_selected_form
  : rational_equiv
      (interval_width (interval_mul (MkInterval r1 r2) (MkInterval r1 r2)))
      (rational_add
        (rational_mul r2 (interval_width (MkInterval r1 r2)))
        (rational_mul r1 (interval_width (MkInterval r1 r2)))) =
  Proved
```

### stdlib/numerical/interval-width-product-application

- spec: §17, `interval_width_mul`.
- promise class: durable invariant.
- given: two ordered intervals `[1,2]`.
- expect: the public theorem applies with two true order premises.
- measured: W-APPLY instantiates the theorem at the four closed endpoints.
- claimed: ordered products obey the selected-form width law.
- gap: an instance is not a proof of the generic theorem.
- why: W-THREE and W-FORM separately fix nonzero product and sum values;
  the theorem application alone could not distinguish a bad width operation.

### stdlib/numerical/interval-width-product-value-and-form

- spec: §17, `interval_width` and the selected product form.
- promise class: durable invariant.
- given: `[1,2]·[1,2]` and the value three.
- expect: W-THREE and W-FORM both accept by closed computation.
- measured: independent `Proved` conclusions, not uses of
  `interval_width_mul`.
- claimed: the product's width is three, matching the selected sum.
- gap: these two closed reductions do not characterize arbitrary
  intervals or prove that both order premises are necessary.
- why: the result differs from the operand-width product of one.

## Isolated refusals

Append exactly one negative declaration to a fresh complete copy of
the accepted client for each row. The accepted controls remain in the
prefix. Both false claims must refuse at their own `Proved` after
that prefix; a failure to import, parse or check an earlier theorem
is not the intended refusal.

### stdlib/numerical/reject-width-product-of-widths

- spec: §17, selected product width form.
- promise class: durable invariant.
- given: the ordered `[1,2]·[1,2]` product.
- expect: X-PRODUCT refuses a width equal to the product of widths.
- measured: W-THREE accepts for the same product; X-PRODUCT changes
  only the claimed value to `1·1` in a `rational_equiv`/`Proved` claim.
- claimed: the selected law is not multiplication of operand widths.
- gap: one instance excludes this competing form on these values, not
  every alternative law or a mutant implementation.
- controls: W-THREE checks the same product, proposition and terminal.
- why: the computed width is three, whereas the width product is one.

```ken
-- X-PRODUCT: refuse after W-THREE on a fresh accepted prefix.
theorem reject_width_product_of_widths
  : rational_equiv
      (interval_width (interval_mul (MkInterval r1 r2) (MkInterval r1 r2)))
      (rational_mul
        (interval_width (MkInterval r1 r2))
        (interval_width (MkInterval r1 r2))) =
  Proved
```

### stdlib/numerical/reject-width-product-inverted-first

- spec: §17, necessity of the first ordered-endpoint premise.
- promise class: durable invariant.
- given: `[3,1]·[1,2]`; only the first interval is inverted.
- expect: X-DROP-FIRST refuses the premise-free selected expression.
- measured: W-FORM accepts the same kind of `rational_equiv`/`Proved`
  claim for ordered inputs; X-DROP-FIRST changes the first interval.
- claimed: the selected product-width formula needs its first premise.
- gap: a closed value refusal is not a generic premise-consumption proof;
  it also does not test necessity of the second premise. Architect's
  separate D0 X-DROP-SECOND checked `[1,2]·[3,1]` at its own `Proved`.
- controls: W-FORM uses the same operation, expression form and terminal
  with the first operand ordered.
- why: truncated product width is zero, while the selected sum is one.

```ken
-- X-DROP-FIRST: refuse after W-FORM on a fresh accepted prefix.
theorem reject_width_product_inverted_first
  : rational_equiv
      (interval_width (interval_mul (MkInterval r3 r1) (MkInterval r1 r2)))
      (rational_add
        (rational_mul r1 (interval_width (MkInterval r1 r2)))
        (rational_mul r1 (interval_width (MkInterval r3 r1)))) =
  Proved
```

## Evidence boundary

Architect's AC-0 ruling `evt_19ygq87cxn5hh` records the detached D0
on `287abf7ed`, run by Foundation Implementer. In that scratch D0,
W-APPLY, W-THREE and W-FORM accepted, and X-PRODUCT and X-DROP-FIRST
refused with `KernelRejected(TypeMismatch)` at their own `Proved` on
fresh prefixes following W-THREE and W-FORM respectively. That is
not execution of these newly authored fences, a generic mutant-law
check, official conformance, CI, publication, landing or a measured
landed-trust delta. No external or `local/refs` material was used.
