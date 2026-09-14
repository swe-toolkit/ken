---
id: CAT-ORD-INT-AXIOM-NAMING
title: "Name the four anonymous term-level Axioms in instance Ord Int (refl/antisym/trans/total) as named catalog `axiom` declarations in LawfulClasses.ken.md, consumed by the instance fields, copying the `string_to_list_char_retraction` shape. Architect-ruled scope: the defect is NAMABILITY, not placement — anonymous Axioms mint indistinguishable trusted_base() entries that fail PRINCIPLES #17's complete-manifest requirement. NOT routed through a kernel certificate: Ord's laws are catalog vocabulary (IsTrue/bool_or) and building them kernel-side would expand the TCB. trusted_base() cardinality must not change (4 anonymous -> 4 named)."
status: draft
owner: foundation
size: S
gate: none
tier: T2
depends_on: []
blocks: []
github: null
origin: "Architect ruling PRIMITIVE-CONTRACT-AXIOM-PLACEMENT (2026-09-14, evt_45tk37v830x9b, thread thr_11dcwfhxcsbdw), durable artifact docs/program/PRIMITIVE-CONTRACT-AXIOM-PLACEMENT.md (routed by Steward at 0af0e9f29). The routing posed an axiom-home question over a four-item inventory; the Architect measured that only ONE item is the real defect. Items 1 (string_to_list_char_retraction, already a named catalog axiom), 3 (Eq/DecEq Int, already proved from kernel-issued int_eq_sound/int_eq_complete), and 4 (BytesRoundTripLaw, caller-supplied across FFI) are out of scope, each for a stated reason. The defect is item 2: instance Ord Int's four law fields are anonymous term-level `Axiom`s. Per AX-2 each mints a trusted_base() entry named only by owner_label (audit metadata, may repeat, not identity), so the four cannot be told apart, cited, or discharged incrementally. The ruling's home rule: an axiom over a TCB export is kernel-issued and co-located IFF its statement is expressible in kernel vocabulary plus the registration's ids; otherwise a named catalog axiom; in neither case an anonymous term-level Axiom. Ord Int fails the kernel-vocabulary test (IsTrue/bool_or), so it is a named catalog axiom, NOT a kernel certificate. Steward-framed as foundation-lane (L3) work per the Architect's explicit handoff; no lane is blocked."
---

# Objective

Give the four anonymous term-level `Axiom`s in `instance Ord Int`
(`LawfulClasses.ken.md`) names, as named catalog `axiom` declarations consumed
by the instance's law fields — copying the `string_to_list_char_retraction`
shape. This is a namability fix, not a relocation and not a soundness change:
the four assumptions are correct in kind (`leq_int` is a `reg_prim`
kernel-`Neutral`, so PRINCIPLES #17 governs), they are simply unnameable.

The full design, grounding, and the four-item inventory are in the ruling doc
`docs/program/PRIMITIVE-CONTRACT-AXIOM-PLACEMENT.md` — read it as the design
authority; this node states the bounded deliverable and its controls.

# Fixed inputs (at origin/main `664691b83`; re-measure by symbol at the cut —
`LawfulClasses.ken.md` is a high-contention file)

- Defect site: `instance Ord Int { leq = int_leq; refl = Axiom; antisym =
  Axiom; trans = Axiom; total = Axiom }` (near `LawfulClasses.ken.md:215-220`
  at this SHA — locate by the `instance Ord Int` declaration, not the line).
- Law-field types (from the `Ord` class def in the same file): `refl : (x : a)
  → IsTrue (leq x x)`; `antisym : (x : a) → (y : a) → IsTrue (leq x y) →
  IsTrue (leq y x) → Equal a x y`; `trans : (x) → (y) → (z) → IsTrue (leq x y)
  → IsTrue (leq y z) → IsTrue (leq x z)`; `total : (x) → (y) → IsTrue
  (bool_or (leq x y) (leq y x))`. For the `Int` instance `a = Int`, `leq =
  int_leq`.
- Template shape: `axiom string_to_list_char_retraction : <type>`
  (`catalog/packages/Data/Text/StringBijection.ken.md`), then consumed by name.
- Negative-control baseline (measured at `664691b83`): `IsTrue` and `bool_or`
  are each at ZERO occurrences in `crates/ken-kernel/src/`.

# Deliverables

- Four named catalog `axiom` declarations in `LawfulClasses.ken.md` (e.g.
  `ord_int_refl`/`ord_int_antisym`/`ord_int_trans`/`ord_int_total`), each with
  the law-field type above instantiated at `Int`/`int_leq`.
- The `instance Ord Int` law fields rewired to consume those names
  (`refl = ord_int_refl; …`) — no remaining `= Axiom` on `Ord Int`.
- The doc prose in the file that describes the `Ord Int` `Axiom` fields updated
  to the named form where it enumerates them.

# Acceptance criteria

- `instance Ord Int` has no `= Axiom` field; each of `refl`/`antisym`/`trans`/
  `total` consumes a named catalog `axiom`.
- `trusted_base()` cardinality is UNCHANGED: four anonymous entries become four
  NAMED entries. A changed count means the statements changed, which is NOT
  authorized. (This is the primary control and it must be measurable.)
- NEGATIVE CONTROL, the one that fails in the damaging direction: `IsTrue` and
  `bool_or` gain NO kernel-side construction — both stay at ZERO occurrences in
  `crates/ken-kernel/src/`. An implementer reaching for symmetry with `DecEq
  Int` would try to build `Ord`'s statements kernel-side; that is a real TCB
  expansion and is exactly what this design forbids.
- The four axioms use kernel-`Neutral` `leq_int` vocabulary (`IsTrue`,
  `bool_or`) at the CATALOG level only, matching the `string_to_list_char_
  retraction` named-axiom shape; NOT a kernel certificate / `declare_postulate`
  path.
- Out of scope and untouched: `Eq Int`/`DecEq Int` (item 3, already proved),
  `string_to_list_char_retraction` (item 1), `BytesRoundTripLaw` (item 4), and
  `string_ord_leq`'s laws — `string_ord_leq` is a Ken-level `fn` so its laws
  are PROVED (`pub proof … for string_ord_leq`), #16 territory, and must NOT be
  converted to axioms.
- No-regression in CI (workspace-green in CI, per COORDINATION §12).

# Sequencing / contention — READ BEFORE RELEASING

`LawfulClasses.ken.md` is a HIGH-contention file: it is the center of the
active CAT migration campaign and is currently touched by in-flight foundation
work (`foundation-implementer/work`, `foundation-leader/work`,
`foundation-qa/work`) and recent `main` commits land there continuously. This
WP must be SERIALIZED — released only when no other in-flight WP holds
`LawfulClasses.ken.md`, to avoid a merge conflict on the same file. It is held
`draft` for that reason, not for any unresolved design question: the design is
fully ruled. The foundation-leader (or the Steward) flips it `ready` and
frames the implementer WP when the file is contention-free; no lane is blocked
by holding it.

# Sizing / tier

Size S, tier T2. A bounded, mechanical-with-judgment edit: four named axiom
declarations copying an existing shape plus a four-field rewire in one file.
The reasoning is entirely in the Architect's ruling doc; the application is
careful editing under the cardinality and negative controls.
