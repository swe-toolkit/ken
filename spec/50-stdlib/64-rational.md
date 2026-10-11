# Exact non-negative rational values

> **Status: DRAFT v0 (SPEC-NUMERIC-RATIONAL-CONTRACT).** This chapter is
> normative for the ordinary `Data.Numeric.Rational` package, its Nat
> prerequisites, and their required checked laws. It specifies an exact
> rational **value** with explicit equivalence, not structural equality
> between different representations. A checked development establishes
> the carrier, positive-denominator construction, equivalence
> transitivity, and addition respect as feasible; it does not deliver
> the package or discharge every law below. No kernel rule, surface
> syntax, primitive, `Axiom`, `Float`, or `trusted_base()` entry is added.

## 1. Carrier and value equality

`Data.Numeric.Rational` is an optional, explicitly imported standard
package over the existing inductive `Nat`. The numerator is any
natural, including `Zero`; the second constructor field is the
**predecessor of the positive denominator**. Thus `MkRational n k`
means `n/(Suc k)`, not `n/k`. No caller supplies an unchecked positivity
proof, and the denominator cannot be zero.

```ken
pub data Rational : Type where {
  MkRational : (num : Nat) → (den_tail : Nat) → Rational
}
export MkRational

pub fn numerator (x : Rational) : Nat
pub fn denominator (x : Rational) : Nat
pub const rational_zero : Rational
pub const rational_one : Rational
pub fn rational_of_nat (n : Nat) : Rational
```

`numerator (MkRational n k) = n` and `denominator (MkRational n k) =
Suc k`. `rational_zero` is `MkRational Zero Zero`, `rational_one`
is `MkRational (Suc Zero) Zero`, and `rational_of_nat n` is
`MkRational n Zero`. The latter is construction from naturals;
`MkRational` itself admits any natural numerator and any positive
denominator via its predecessor. There is no normalization in the
constructor.

For `x = n/d` and `y = m/e`, where `d` and `e` are the projected
positive denominators, **value equivalence** is the proposition that
`mul n e = mul m d`. It is not `Equal Rational x y`: for example,
`MkRational 1 1` and `MkRational 2 3` denote the same value
`1/2 = 2/4` but remain different carrier terms. Likewise, `0/1`
and `0/7` are equivalent without being identical representations.
Clients that assert a rational equality must use `rational_equiv`
and its checked laws, not `Refl` on arbitrary spellings.

```ken
pub fn rational_equiv (x : Rational) (y : Rational) : Prop
```

Its defining proposition is `Equal Nat (mul (numerator x)
(denominator y)) (mul (numerator y) (denominator x))`. Both the
carrier and the proposition live at level-zero `Type` and `Omega`,
respectively. The structural positivity proof and the equivalence
laws are ordinary checked Ken:

```ken
pub proof positive for denominator (x : Rational) :
  Equal Bool (lt_nat Zero (denominator x)) True

pub proof refl for rational_equiv (x : Rational) :
  rational_equiv x x

pub proof sym for rational_equiv
  (x : Rational) (y : Rational)
  (h : rational_equiv x y) : rational_equiv y x

pub proof trans for rational_equiv
  (x : Rational) (y : Rational) (z : Rational)
  (xy : rational_equiv x y) (yz : rational_equiv y z) :
  rational_equiv x z
```

Transitivity is not definitional equality of the pair. The checked
D0 development multiplies the two cross equations, reorders the
natural products, then cancels the **positive middle denominator**
using §4's proved `mul::cancel_r`. Neither proof irrelevance nor an
unprovided quotient or Setoid mechanism asserts this law.

## 2. Total arithmetic, order, and congruence

Addition and multiplication are total and exact. For pairs `n/d`
and `m/e`, `rational_add` forms numerator `add (mul n e) (mul m d)`
and denominator `mul d e`; `rational_mul` forms numerator `mul n m`
and the same denominator `mul d e`. The denominator's predecessor
is computed by a **package-local** `positive_product_tail`; it is
not an exported operation. The checked relation
`mul (Suc a) (Suc b) = Suc (positive_product_tail a b)` keeps the
result positive by construction. No gcd, division, or rounding is
used in either operation.

The Boolean order compares the same cross products by the existing
canonical `leq_nat`. It returns a decision about the **values**,
not lexicographic order on `(numerator,den_tail)` and not floating
comparison.

```ken
pub fn rational_add (x : Rational) (y : Rational) : Rational
pub fn rational_mul (x : Rational) (y : Rational) : Rational
pub fn rational_leq (x : Rational) (y : Rational) : Bool
```

For `x=n/d` and `y=m/e`, `rational_leq x y = leq_nat (mul n e)
(mul m d)`. The exposed denominator equations ensure callers can
reason from the two operations' actual values rather than a
separately asserted or cached denominator:

```ken
pub proof denominator_product for rational_add
  (x : Rational) (y : Rational) :
  Equal Nat
    (denominator (rational_add x y))
    (mul (denominator x) (denominator y))

pub proof denominator_product for rational_mul
  (x : Rational) (y : Rational) :
  Equal Nat
    (denominator (rational_mul x y))
    (mul (denominator x) (denominator y))
```

All algebraic equations in this section use `rational_equiv`.
Neither a commuting sum nor a unit law can generally claim
`Equal Rational`: an unnormalized operation may produce a different
numerator/denominator spelling for the same value. An operation
must also **respect** equivalence in each operand; the following
proved laws make the quotient-independent interface usable even
without a quotient type. In particular, `rational_leq` gives
identical Boolean results on equivalent inputs.

```ken
pub proof respect for rational_add
  (x : Rational) (xp : Rational) (y : Rational) (yp : Rational)
  (hx : rational_equiv x xp) (hy : rational_equiv y yp) :
  rational_equiv (rational_add x y) (rational_add xp yp)

pub proof respect for rational_mul
  (x : Rational) (xp : Rational) (y : Rational) (yp : Rational)
  (hx : rational_equiv x xp) (hy : rational_equiv y yp) :
  rational_equiv (rational_mul x y) (rational_mul xp yp)

pub proof respect for rational_leq
  (x : Rational) (xp : Rational) (y : Rational) (yp : Rational)
  (hx : rational_equiv x xp) (hy : rational_equiv y yp) :
  Equal Bool (rational_leq x y) (rational_leq xp yp)
```

The checked D0 addition-respect development covers **arbitrary four
rationals and both real hypotheses**; its independent weakened
second-premise mutation kernel-rejects. The multiplication and order
respect bodies remain required CAT proofs, not consequences of that
one measurement.

### 2.1 Additive and multiplicative laws

The additive laws include both units, both orderings of a sum, and
associativity; the multiplicative laws include both units, a zero
annihilator, and left distributivity over addition. Their stated
orientation is an equality of **values** through `rational_equiv`.
The symmetric right distributivity follows from the checked
commutativity and left distributivity; it is not a second public
law or a separately trusted rule.

```ken
pub proof comm for rational_add (x : Rational) (y : Rational) :
  rational_equiv (rational_add x y) (rational_add y x)

pub proof assoc for rational_add
  (x : Rational) (y : Rational) (z : Rational) :
  rational_equiv
    (rational_add x (rational_add y z))
    (rational_add (rational_add x y) z)

pub proof zero_l for rational_add (x : Rational) :
  rational_equiv (rational_add rational_zero x) x

pub proof zero_r for rational_add (x : Rational) :
  rational_equiv (rational_add x rational_zero) x

pub proof comm for rational_mul (x : Rational) (y : Rational) :
  rational_equiv (rational_mul x y) (rational_mul y x)

pub proof assoc for rational_mul
  (x : Rational) (y : Rational) (z : Rational) :
  rational_equiv
    (rational_mul x (rational_mul y z))
    (rational_mul (rational_mul x y) z)

pub proof one_l for rational_mul (x : Rational) :
  rational_equiv (rational_mul rational_one x) x

pub proof one_r for rational_mul (x : Rational) :
  rational_equiv (rational_mul x rational_one) x

pub proof zero_l for rational_mul (x : Rational) :
  rational_equiv (rational_mul rational_zero x) rational_zero

pub proof distrib_l for rational_mul (x : Rational) (y : Rational)
  (z : Rational) :
  rational_equiv
    (rational_mul x (rational_add y z))
    (rational_add (rational_mul x y) (rational_mul x z))
```

Zero and one are **values**, not normal forms for every equivalent
spelling. A client may compare `0/7` to `rational_zero` with
`rational_equiv`, and multiplication's annihilator still applies
under the explicit respect law. The D0 scratch checked the four
generic zero/one unit bodies; associativity, commutativity,
distributivity, and zero annihilation are further required checked
CAT bodies, not claimed complete from those unit probes.

### 2.2 Exact order

The order is reflexive, transitive, antisymmetric **up to value
equivalence**, and total. `False` is an actual strict opposite
witness for a particular pair, not an `unknown` or an exceptional
case. The exact propositions use the Boolean relation:

```ken
pub proof refl for rational_leq (x : Rational) :
  Equal Bool (rational_leq x x) True

pub proof trans for rational_leq
  (x : Rational) (y : Rational) (z : Rational)
  (xy : Equal Bool (rational_leq x y) True)
  (yz : Equal Bool (rational_leq y z) True) :
  Equal Bool (rational_leq x z) True

pub proof antisym for rational_leq
  (x : Rational) (y : Rational)
  (xy : Equal Bool (rational_leq x y) True)
  (yx : Equal Bool (rational_leq y x) True) :
  rational_equiv x y

pub proof total for rational_leq
  (x : Rational) (y : Rational)
  (xy_false : Equal Bool (rational_leq x y) False) :
  Equal Bool (rational_leq y x) True
```

`rational_leq::antisym` does **not** return `Equal Rational x y`:
`1/2` and `2/4` order each way and remain distinct pair terms.
The generic order proofs rely on checked natural-order monotonicity
and cancellation by positive factors from §4. The D0 checked
`rational_leq::refl` and unequal closed outcomes; no D0 claim
covers the generic transitivity, antisymmetry, totality, or order
respect bodies.

## 3. Package derivation and trust boundary

`Rational` is an ordinary `data` over the prelude `Nat`; all its
computations are ordinary total Ken definitions. Its exported
constructor has no proof field, because the positive denominator
is syntactic `Suc den_tail`. `rational_equiv` is an `Equal Nat`
proposition; `rational_leq` consumes the existing Boolean natural
order. The arithmetic uses only checked Nat addition,
multiplication, and the proved natural-algebra and order lemmas in
§4. No new compiler identity, quotient rule, typeclass instance,
`Float` primitive, or `trusted_base()` entry is needed.

The **catalog completion obligation** is the entire public inventory
in §§1–2, with every proof body kernel-checked and no `Axiom`,
postulate or open proof obligation. A scratch check demonstrates
feasibility for the named D0 and AC-0b subset only. Tests and the
accompanying seed are independent behavioral oracles; neither
substitutes for any public proof. Until the CAT package and its
Nat prerequisites are delivered, this chapter is a normative target,
not a claim that imports currently succeed on `main`.

## 4. Nat prerequisite surface

The defining package `Data.Numeric.Nat.Arithmetic` already has
checked `add`, `mul`, public addition laws, and public
`mul_add_distrib_r`. It also has the following **checked but private**
multiplication laws. The CAT work makes these six existing laws
public under their existing identities; it does not redefine `mul`,
create competing proofs in Rational, or duplicate the canonical Nat
class dictionary.

```ken
pub proof zero_l for mul (n : Nat) :
  Equal Nat (mul Zero n) Zero

pub proof one_l for mul (n : Nat) :
  Equal Nat (mul (Suc Zero) n) n

pub proof one_r for mul (n : Nat) :
  Equal Nat (mul n (Suc Zero)) n

pub proof comm for mul (a : Nat) (b : Nat) :
  Equal Nat (mul a b) (mul b a)

pub proof assoc for mul (a : Nat) (b : Nat) (c : Nat) :
  Equal Nat (mul a (mul b c)) (mul (mul a b) c)

pub theorem mul_add_distrib_l (a : Nat) (b : Nat) (c : Nat) :
  Equal Nat (mul (add a b) c) (add (mul a c) (mul b c))
```

The same Arithmetic package gains a new **checked** cancellation
law for multiplication by a positive factor. Its actual proof in
the AC-0b scratch uses Nat case analysis, `Suc` injectivity, and
cancellation of addition on the right. The existing private
`mul::suc_l` proof can support that implementation without becoming
a seventh exported algebra law.

```ken
pub proof cancel_r for mul
  (a : Nat) (b : Nat) (k : Nat)
  (h : Equal Nat (mul a (Suc k)) (mul b (Suc k))) :
  Equal Nat a b
```

`Data.Numeric.Nat.Order` additionally exports two checked laws for
the canonical `leq_nat` under the same positive multiplier. Their
Boolean propositions are directional: monotonicity carries a
non-strict bound **into** multiplication, and cancellation brings a
multiplied bound **back**. Both are needed to transport exact order
through different rational representations; neither follows from
`mul::cancel_r`, which accepts an **equality**, not an inequality.

```ken
pub theorem leq_nat_mul_right_mono
  (a : Nat) (b : Nat) (k : Nat)
  (ab : Equal Bool (leq_nat a b) True) :
  Equal Bool
    (leq_nat (mul a (Suc k)) (mul b (Suc k))) True

pub theorem leq_nat_mul_right_cancel
  (a : Nat) (b : Nat) (k : Nat)
  (scaled : Equal Bool
    (leq_nat (mul a (Suc k)) (mul b (Suc k))) True) :
  Equal Bool (leq_nat a b) True
```

These are **Nat provider obligations** in the Rational CAT node or a
landed prerequisite, never assumptions locally re-proved or
postulated by the Rational package. The D0/AC-0b scratch checked the
proposed `mul::cancel_r`, but did **not** check either new order
lemma. `Algorithm.Numeric.Gcd` retains no public surface in this WP:
no ruled rational declaration consumes it.

## 5. Deliberate exclusions and next consumers

This contract specifies **non-negative exact rational values** only.
Signed values, subtraction, reciprocals and division, reduced or
canonical spellings, gcd normalization and Gcd exports, `Float`,
approximate statistics, natural-construction homomorphism laws, and
monotonicity of rational addition or multiplication remain separate
work. A normalized subtype can be built over this carrier later;
this chapter does not pretend every equivalent spelling is identical
now.

`spec/50-stdlib/63-statistics.md` §§1–2 currently interpret event
mass and score sum over a positive total as numerator/denominator
pairs, without returning a `Rational`. `MkRational n k` supplies the
value-level representation of `n/(Suc k)` needed by a future
statistics adapter, but **no value-typed probability, expectation or
mean operation** is added to either statistics tier by this chapter.
Those re-statements, including any required compatibility bridge to
§63's existing numerators, need their own checked CAT contract and
conformance. The denominator-proof obligations in §63 are not
silently replaced by this constructor.
