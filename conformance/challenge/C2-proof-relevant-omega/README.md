# C2 — a proof-relevant inductive declared at `Ω`

**Axis:** the Ω / strict-prop boundary. **Flavor:** A
(soundness-boundary — the unsound arm must reject; acceptance is a consistency
hole). **Couples with C5**, whose `Perm` uses the Ω-native count-equality form
pinned in `57 §3.1`.

## Why this is a blind spot

`Ω` (`Prop`) is definitionally proof-irrelevant: any two inhabitants of an
Ω-proposition are equal by fiat. A proof-relevant multi-constructor inductive
cannot be declared directly at Ω: unrestricted `Type → Ω` admits `Bool`, and
Ω proof irrelevance would then force `True ≡ False`, collapsing consistency
(`16 §1.4`).

The pinned `Perm` is not a truncation of a permutation relation. It is
count-equality:

```text
Perm a eqf xs ys = (x : a) → Equal Nat (count a eqf x xs) (count a eqf x ys)
```

For verified sort, `eqf` is `eq_from_ord a leq`, derived from the sort's own
comparator. `‖Perm_rel‖` remains the named fallback for carriers without
decidable element equality; it is not the `Perm` pin (`57 §3.1`).

## The pair

- **Sound arm — `sound-perm-count.ken` — should pass.** It exhibits the
  count-equality shape as an Ω proposition. The fixture names a witness
  element (`PermAt`) to keep the example concrete; it is not the full public
  `Perm`, which quantifies over every element.
- **Unsound arm — `unsound-perm-omega.ken` — should reject.** It declares a
  proof-relevant permutation relation directly at Ω using four constructors.
  This is forbidden by the Ω proof-irrelevance gate.

## Expected behavior

The count-equality proposition is natively Ω-valued because it is a function
into equality of `Nat` values (`16 §1.1`). The C5 sort law specializes that
form with `eq_from_ord` and imports `Perm` from `Data.Collections.Derived`.

The direct Ω inductive must not be accepted. The recorded C2 probe currently
rejects at the parser because Ω-data constructors do not parse; that is a
coincidental rejection, not evidence that the Ω sort-check was reached. If a
surface path admits the declaration and the kernel accepts it, that is a
consistency stop.

## Discriminator

The property is the representation boundary: count-equality is the current
Ω-native `Perm`; a proof-relevant multi-constructor relation is not an Ω
inductive. A successful count-equality example alone would not test the
forbidden declaration.

## Surface-expressibility note

If the surface cannot write an explicit Ω sort annotation on `data`, record the
parser boundary rather than crediting the kernel gate. The C2 arm is a prepared
challenge, not an official conformance-runner result.
