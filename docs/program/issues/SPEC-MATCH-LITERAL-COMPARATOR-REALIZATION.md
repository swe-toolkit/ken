---
id: SPEC-MATCH-LITERAL-COMPARATOR-REALIZATION
title: "spec 34 §3.2 pins each literal-pattern comparison RESULT but is silent on the IMPLEMENTATION FLOOR: whether exact, total, already-landed lossless views (fixed-width `*_to_int`, `string_to_list_char`, `bytes_to_list`) may realize the comparison internally, or whether direct per-carrier comparators are required -- and the current 'does not widen or narrow' sentence reads as forbidding the view composition the emitted checked core would contain, so the contained literal-pattern core cannot be built without the enclave stating which realization is admitted"
status: active
owner: spec
size: S
gate: none
depends_on: [SPEC-MATCH-PATTERN-PINS]
blocks: [LANG-MATCH-LITERAL-PATTERN]
github: null
origin: "Steward cut 2026-09-12 from the Architect HS1 ruling on LANG-MATCH-LITERAL-PATTERN (evt_1wc0m1xbtk4r, classification PINNED_VALUE_COMPARATOR_HAS_NO_TOTAL_CORE_REALIZATION, re-derived at exact main 2d35fd3b6). The Architect ruled the literal-pattern node must HOLD and recut into three parts; this is part (1), a behavioral-contract correction the Architect explicitly named as OWNED BY SPEC, not an Architect waiver of the present words. It gates the contained Language core (LANG-MATCH-LITERAL-PATTERN, now blocked on this)."
---

> # SPEC CORRECTION owed to the enclave (Architect HS1 evt_1wc0m1xbtk4r). It
> # GATES the contained literal-pattern core [[LANG-MATCH-LITERAL-PATTERN]],
> # which is BLOCKED until this lands. Zero-TCB path (option 1) is the
> # Architect's recommendation; option 2 grows the TCB and would return to the
> # operator. Architect is the design authority; CV reviews conformance-row
> # currency; Steward M1-M4 -> lieutenant.

## What this is

`spec/30-surface/34-data-match.md §3.2` (the literal-kind-to-comparator table,
`:385-420`) pins the RESULT of each literal comparison. It does not state the
admitted IMPLEMENTATION FLOOR. The Architect HS1 measured that the prelude has
no total `Bool` core realization for the whole pinned population at base
`2d35fd3b6`:

- direct total comparators exist only for `Int` (`eq_int`), `Float`
  (`eq_float`), `Float32` (`eq_float32`), and `Char` (`eqChar`)
  (`NumericEnv::eq_table`, `numbers.rs:557-560`; `decimal_char.rs:195-211,266-268`);
- fixed-width carriers have only their exact lossless views `<snake>_to_int`
  (`conversions.rs:143-166`);
- `String` has `string_to_list_char` (+ `eqChar` per codepoint); `Bytes` has
  `bytes_to_list` (+ `uint8_to_int`/`eq_int` per octet);
- `decimalEq` is registered but NOT total: it aligns exponents through
  `decimalPow10`, and beyond `MAX_SHIFT = 30` `decimalPow10Unbounded` is
  deliberately stuck (`decimal_char.rs:41-66,110-141,179-188`).

So a compiler-owned, literal-shaped comparison plan CAN realize `String` (and,
structurally, `Bytes` and fixed-width) from these exact total views WITHOUT any
new primitive or global comparator. The obstruction is a spec-contract one, not
a capability one: **34 §3.2 currently says fixed-width comparison "does not ...
widen or narrow," and the emitted checked core for the view-composition route
would actually contain `int8_to_int`/etc.** The Architect will not redefine that
sentence by fiat ("no source-visible coercion") -- the `uint8_deceq_eq` lawful
instance shows the mechanism distinction is meaningful -- so the enclave must
state which realization is admitted before the Language core can build the
fixed-width and Bytes rows.

## The decision this node owes

**Option 1 (Architect-recommended, zero new TCB).** State in 34 §3.2 that exact,
total, already-landed lossless view composition (`*_to_int`, `string_to_list_char`
+ `eqChar`, `bytes_to_list` + octet `eq_int`) is an ALLOWED INTERNAL comparator
realization, while the source-level carrier selection is unchanged -- i.e.
"does not widen or narrow" governs the SOURCE carrier of the literal and its
scrutinee, not the internal checked-core realization, which may compose exact
lossless views. The literal-shaped structural plan is finite source-literal
unrolling, not a duplicate general comparator, not an implicit catalog import,
and not a new primitive.

**Option 2 (larger, returns to the operator).** Require DIRECT per-carrier
comparators. That is a new TCB / runtime capability (new trusted-base entries +
interpreter + native reduction) and CANNOT stay in the Language slice; if the
enclave concludes option 2 is necessary, HARD-STOP to the Steward, who returns
it to the operator (TCB growth is an operator call).

## Deliverables

1. Edit `34 §3.2` to state the admitted comparator realization (option 1 unless
   the enclave rules otherwise), scoping the "does not widen or narrow" contract
   to source carrier selection.
2. SPLIT the compound literal-comparator conformance row into one row per carrier
   (`Int`, each fixed-width, `Float`, `Float32`, `Char`, `String`, `Bytes`,
   `Decimal`), so a phased Language delivery can be conformance-honest about
   which carriers are supported now and which are deferred.
3. State `Decimal` as explicitly DEFERRED: no unrestricted Decimal literal
   pattern is total under the present comparator (the scrutinee exponent is
   unbounded, `decimalPow10Unbounded` is demanded under CBV even for a zero
   coefficient), so the sound present admission predicate is `false`; point the
   deferral at the 18a §5.6.1(2) unbounded-alignment forward obligation.
4. If option 2 is chosen instead, do NOT edit the spec; hard-stop to the Steward.

## Acceptance criteria

- `34 §3.2` states, unambiguously, whether internal lossless view composition is
  an admitted comparator realization; a reader can decide the fixed-width/Bytes
  question from the text alone.
- The compound conformance row is split per carrier; each row is marked
  supported-now or deferred with its reason.
- `Decimal` is marked deferred with the totality reason and the 18a §5.6.1(2)
  pointer.
- No TCB growth (option 1). Option 2, if chosen, is not landed here -- it
  escalates.
- CV confirms the conformance-row split is currency-consistent with the pinned
  results (the RESULT semantics from SPEC-MATCH-PATTERN-PINS are unchanged; only
  the realization floor and row granularity are stated).

## Not this node

- The Language elaborator implementation ([[LANG-MATCH-LITERAL-PATTERN]] -- the
  contained core that consumes this correction).
- Any Decimal total comparator or bounded-scrutinee contract (18a §5.6.1(2)).
- The 35 §4.2 user-type literal mechanism (unstaged; such literals stay
  fail-closed in the Language core).

## Sizing / tier

**Size S, tier T1.** The edit is small but it is a behavioral-contract decision
about the admitted realization floor (the source-carrier vs internal-realization
distinction the `uint8_deceq_eq` instance shows is meaningful), reviewed as a
correctness statement, not a mechanical wording tidy.
