---
id: CAT-IDF-TRUST-FREE-PROVIDER
title: "Trust-closure repair after Vec: move the canonical idf (and comp) to a small trust-free function-combinator provider, so loading Data.Vector.Vector cold no longer pulls five unrelated trusted assumptions into its manifest; pinned by a cold-load trusted_base set-equality check"
status: ready
owner: foundation
size: M
gate: architect
tier: T1
depends_on: [CAT-CONCAT-MAP-APPEND-DISTRIBUTIVITY-LAW]
blocks: []
github: null
origin: "Adversary M8 on Vec squash 33a2f27c2 (evt_3ywnxtkq61fcb). Architect disposition (b) evt_3v28sw4kvdq4c: re-home the identity to a trust-free provider rather than pin the inherited axioms. Sequenced after Concat on L3 (Steward). Steward-filed per COORDINATION section 2."
---

# Vector inherits five axioms to get `idf`

## Fixed inputs -- measured at `33a2f27c2`

- `pub fn idf (a : Type) (x : a) : a = x` and the pure `pub fn comp` live
  in `catalog/packages/Core/Classes/LawfulFunctors.ken.md` (`:149`,
  `:151`). LawfulFunctors imports `Data.Collections.Derived` and
  `Core.Classes.LawfulClasses`. LawfulClasses imports
  `Data.Text.StringBijection`.
- Vec added `import Core.Classes.LawfulFunctors (idf)` to Vector. The
  Adversary measured a cold roots load of Vector at 112 trusted items against
  a compiler base of 107, where the parent was 107 = 107. The five are the
  four `Ord Int` `Axiom` fields and `string_to_list_char_retraction`. The
  Vec proof uses none of them. This is the Adversary's measurement;
  re-establish it.
- The Vec trust tests (`cat_vec_acceptance.rs`, `cat_vector_closeout.rs`)
  preload the providers before comparing, so they cannot see the closure
  delta. `Vector.ken.md` says imported trust is visible "if any".
- Other direct users of `idf`: `EffectfulClasses`, `Validation`, and tests
  under `crates/ken-elaborator` and `conformance/stdlib/classes`.

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## Deliverable

- Move the single existing `pub fn idf`, and `pub fn comp` with it, into the
  new trust-free provider `Core.Function.Combinators` at
  `catalog/packages/Core/Function/Combinators.ken.md` (name settled by the
  Architect, `evt_1x2e11te39vmz`). Its closure has no trusted assumptions.
  LawfulFunctors keeps no copy. Do not copy or redeclare a second `idf`, add
  a private Vector substitute, or put generic functions into
  `Core.Logic.Transport` because it is already imported.
- LawfulFunctors imports the moved functions and still states and checks
  its Functor laws.
- Migrate every direct user (Vector, EffectfulClasses, Validation, and any
  checked consumer) to the new provider's identity. No compatibility
  duplicate is kept; there are no external users.
- `vec_map_identity`'s proposition is unchanged. Its raw-type pin names the
  one new `idf` GlobalId.
- Replace Vector's "if any" prose with the true closure. Record the five as
  historical inherited trust in the repair note only.
- No new operation, axiom or kernel feature.

## Acceptance

- **AC-1 (cold closure, base red first).** Roots-load Vector into a fresh
  `ElabEnv` without preloading providers. Compare its `trusted_base()` **as
  a set** with a separately fresh compiler base. At `33a2f27c2` this fails
  (112 against 107); on the candidate the sets are equal. A count or a
  relative delta over preloaded providers does not satisfy this AC.
- **AC-2 (mutation).** In a scratch candidate, add exactly
  `import Core.Classes.LawfulFunctors (Functor)` to Vector (its
  `pub class Functor` stays public), keeping the new canonical `idf` import.
  The mutated Vector must still roots-load and its proof must still check;
  the cold-closure set-equality assertion **alone** reddens, showing the
  actual provider closure. Restore and recheck. The unmutated cold Vector
  check is the positive control.
- **AC-3 (consumer sweep).** Sweep every provider consumer, import cycle and
  conformance reference to the changed canonical ids. Do not infer
  completeness from a green Vec-only suite. The LawfulFunctors laws and
  every migrated user stay green.
- **AC-4.** No lasting assertion that allows five inherited items, and no
  permanent pin keyed to the anonymous `Ord Int` field labels that
  `CAT-ORD-INT-AXIOM-NAMING` will rename. Targeted builds only, through
  `scripts/ken-cargo`. No-regression means green in CI.

## Stop conditions

Stop if the new provider's cold closure is not trust-free, if a migration
needs a new operation or a second `idf`, or if a consumer cannot move
without changing its proved statements.
