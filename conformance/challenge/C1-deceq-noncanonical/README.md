# C1 — `DecEq` over a non-canonical carrier (the Bottom edge)

**Axis:** lawful typeclasses + the canonical-carrier soundness precondition.
**Flavor:** A (soundness-boundary — the unsound arm must REJECT; acceptance =
a hole). **Couples with C7** (the quotient is the principled fix).

## Why this is a blind spot

`DecEq a`'s law `sound : (x y : a) → IsTrue (eq x y) → Equal a x y` makes `eq`
**decide the kernel's propositional equality**. That is a *theorem* only when
the carrier is **canonical** — one representation per semantic value. Over a
non-canonical carrier the statement is **false**, and a postulated `sound`
inhabits `Bottom` (ADR 0010; the Decimal/Char DEMOTE arc). VAL2 used `Ord`/
`DecEq` as comparators but never stressed the canonicity precondition — and the
existing machinery (`51 §5`, "laws PROVED not postulated") **guards deception,
not falsehood**: it checks that a postulate's proof-chain reaches an honest
audited leaf, never whether the *statement being postulated is even true*. So a
naive lawful `DecEq DecimalModel { sound = Axiom }` is the construction this
fixture aims to test, once it reaches the law boundary.

## The pair

- **Sound arm — `sound-deceq-char.ken` — should-PASS.** `DecEq Char` over the
  canonical carrier `Char = {c : Int | isScalar c}` (one Int per codepoint under
  erasure). `sound = Axiom` here is an **honest, true** postulate: `Equal Char`
  coincides with `Equal Int`, and `eqChar` really does decide it. Behaves
  exactly like the landed `DecEq Int` (also `Axiom`, also sound — `Int` is a
  primitive with no induction principle, so its true laws are honest visible
  postulates).
- **Unsound arm — `unsound-deceq-decimal.ken` — should-REJECT, currently
  non-oracular.** `DecEq DecimalModel` over the local
  `DecimalModel = MkDecimalPair Int Int` is non-canonical:
  `MkDecimalPair 10 (-1)` and `MkDecimalPair 1 0` both denote `1`.
  `decimalEq` returns `True` on those structurally distinct pairs, so
  `sound` would yield
  `Equal DecimalModel (MkDecimalPair 10 (-1)) (MkDecimalPair 1 0)`;
  `MkDecimalPair` injectivity (K7 no-confusion) refutes it:
  `Equal Int 10 1 → Bottom`. The source sketches the exploit up to its
  `falseEqDecimal` claim, but the constructor collision rejects the data
  declaration before the law or exploit is checked.

## Expected behavior (exact)

- Sound arm: **PASS** — `DecEq Char` elaborates; `sound`/`complete` are honest
  visible `Axiom`s over a canonical carrier (grounded: `lawful_classes.ken`
  `DecEq Int` is the identical honest-`Axiom` shape; `Char` canonicity per ADR
  0010 §2).
- Unsound arm: **should-REJECT at the `DecEq` law boundary** — the intended
  outcome is that the lawful `DecEq DecimalModel` instance is **not
  deliverable** (ADR 0010 decision pt1/§90 `OQ-decimal-eq`). Under the
  current prelude, the source instead refuses with E0030
  `DuplicateConstructorSpelling` at `MkDecimalPair`, before `DecEq.sound`.
  This pre-existing collision makes the arm non-oracular for the `DecEq` law:
  its rejection is **not credited** as evidence of canonicity enforcement.
  Repairing the constructor so the arm reaches the law is a carry, not work
  in this node.

## Discriminates

The intended discriminator asks whether lawful-class machinery **enforces
carrier canonicity**, or admits a Bottom-inhabiting `sound = Axiom` over a
non-canonical carrier. Char (canonical, PASS) vs DecimalModel (non-canonical,
must REJECT at the law) is the intended flip. The current E0030 constructor
refusal does not reach that boundary; the unsound arm supplies no live
`DecEq`-law oracle.

## Surface-expressibility note

The `sound` field type (`IsTrue (eq x y) → Equal a x y`) and `Axiom` fields are
landed surface (`lawful_classes.ken`). The exploit's final step needs
`MkDecimalPair` injectivity (K7 no-confusion, landed) and `Bottom`-elim
(`absurd`, K5, landed). If the surface cannot yet *state* the injectivity
projection after the constructor carry, document that step as
reached-but-not-surface-expressible. An instance accepted at the law boundary
would itself be the finding; the current E0030 refusal makes no such claim.
