# C7 — quotient type + the `respect` obligation

**Axis:** observational / quotient fragment (K2). **Status:** prepared
challenge, not a current conformance oracle. The current `.ken` sketches fail
before the respect check: they omit the checked `e : IsEquiv A R` required by
`16 §5`. **Couples with C1** — a lawful quotient is the principled alternative
to C1's naive `DecEq Decimal`.

## Why this is a blind spot

C1 shows that a value-equality over a non-canonical carrier cannot back a lawful
`DecEq` (it inhabits `Bottom`). The **principled resolution** ADR 0010 names is
a **quotient / setoid**: make the carrier canonical by quotienting the raw pairs
by denotational equality, `Decimal := DPair / denoteEq / e` for
`e : IsEquiv DPair denoteEq`. The quotient
eliminator `elim_/` then **demands a `respect` proof** — that any function you
lift out of the quotient gives equal results on related inputs. That obligation
is exactly the check C1's unsound instance skipped. VAL2 never touched the
observational/quotient fragment at all.

## Intended pair

A future C7 pair uses a genuine equivalence relation and checked witness `e`.
A respecting `isZero` lift should be admitted; a `coeff` lift that separates
related representatives should be refused. These are design expectations,
not current conformance verdicts: the existing sketches do not reach the
respect check.

## Current reachability and fixture status

The frontend has no quotient construction: `/` is parsed as built-in integer
division, not as `Term::Quot`. The legacy expressions therefore do not reach
kernel quotient formation; that is a surface-gap result only. The current
sources also use `DPair / denoteEq` without the required third
argument `e`. At `x = y`, the displayed exponent clause requires
`eq_int (add_int ex 1) ex`, and no checked reflexivity witness is supplied.
Thus spelling alone cannot make this a formed quotient or reach `elim_/`.

The `isZeroRespect` `Axiom` is a placeholder assumption, not a discharged
proof. It cannot show that the sound arm is proven, or that the pair
discriminates the respect gate. Do not cite either fixture as a current C7
respect verdict. A future C7 oracle needs a valid equivalence relation, a
checked `e`, and a respecting/non-respecting pair that reaches Quot-Elim.

## Surface-expressibility note

Quotients are kernel-level (`Term::Quot`/`QuotElim`); formation is
`A / R / e` with checked `e : IsEquiv A R` (`11-syntax.md`, `16 §5`).
The frontend has no quotient production; `/` is integer division. That
surface gap is not evidence about the `respect` gate.
A future C7 input must first provide a genuine equivalence and checked `e`;
the current sketches do neither.
