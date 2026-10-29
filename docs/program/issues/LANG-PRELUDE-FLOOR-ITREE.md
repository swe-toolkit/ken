---
id: LANG-PRELUDE-FLOOR-ITREE
title: "Admit ITree to the reserved strict prelude floor as the third keyed-additions slice, with its unchanged constructors scoped as ITree.Ret and ITree.Vis, bare Ret and Vis freed rather than reserved, and the spec roster, count and PRELUDE_FLOOR_NAMES changing together"
status: ready
owner: language
size: L
tier: T1
gate: architect
depends_on: [LANG-PRELUDE-FLOOR-KEYED-EFFECTS]
blocks: []
github: null
origin: "Slice 3 of the keyed floor additions, split out of LANG-PRELUDE-FLOOR-KEYED-EFFECTS. Scope: Spec ruling evt_6g6zj2k2vzp5e, confirmed by the Spec Leader in evt_5fw7p5jnrxq3x. Keying: Architect evt_a9zrvmkrazcr. Steward-filed per COORDINATION section 2."
---

> **AMENDED 2026-10-29 (Architect `evt_2vg7zkq5yh2va`, hard stop
> `evt_4zsf5m00r95g7`).** Scoping made the constructor's artifact symbol
> `ctor:<pkg>::ITree::ITree.Ret`, because `stable_symbols_for_env` takes
> the post-scoping spelling. Scoping is a surface binding property (spec 34
> §1.1), so the symbol must stay `T::C`. The fix is in this WP: deliverable
> item 3 and the restated AC-2 and AC-3 below. Everything else stands.

# The strict floor admits ITree with a scoped constructor family

## Objective

Strict resolution admits `ITree` as a reserved floor type that resolves to
its exact registered id. `ITree.Ret` and `ITree.Vis` resolve to their checked
constructor ids. Bare `Ret` and `Vis` are no longer prelude names and are not
reserved: a source program may bind them. The spec 30 §4 roster and count,
the ledger rows and `PRELUDE_FLOOR_NAMES` agree on one branch.

## Settled inputs (main `ac5b0a002`)

- **Scope (Spec `evt_6g6zj2k2vzp5e`, confirmed in `evt_5fw7p5jnrxq3x`).**
  - Admit `ITree` as a floor type.
  - Scope its constructors unchanged, as `ITree.Ret` and `ITree.Vis`.
  - Do not reserve bare `Ret` or `Vis`, and do not rename either.
- **Keying (Architect `evt_a9zrvmkrazcr`).** `ITree` keys through
  `env:itree_id`, which is `PreludeEnv.itree_id` in
  `crates/ken-elaborator/src/prelude.rs`. `Ret` and `Vis` key through their
  role and environment ids. That D0 measured on `7014158ff` and checked reader
  stability only through `9d4ba1e83`.
- **The mechanism is slices 1 and 2.** It is `PRELUDE_FLOOR_NAMES`, floor-id
  admission and `scope_constructor_family`. That function keys the family on
  the parent's checked `GlobalId` and the complete constructor-id set. It
  removes root-scope locals, so no bare alias survives (Architect review
  `evt_d0e1paptg7jj` of slice 2).
- **Consumer population, a ceiling only.** `git grep -w` outside
  `crates/*/src`, over `crates/`, `examples/`, `conformance/` and `catalog/`,
  finds `Ret` in 126 files (979 lines) and `Vis` in 96 files (916 lines). It
  was counted on main `ac5b0a002`, with slice 2 landed. It counts Rust
  identifiers and legal user-declared constructors, such as
  `rt_dasm_d1b_role_a`'s `data ShadowSpine = Ret | Vis | InL | InR`, as well
  as Ken uses of the prelude constructors. The real migration
  population is the first act's census, not this number. Two catalog files
  carry hits: `Capability/Console/Text.ken.md` and
  `Core/Classes/EffectfulClasses.ken.md`.
- **Sequencing.** `ABI-S6-HS18-MAIN-BASED-CLOSURE` increment A has landed
  (`bd5c64ed1`), so the earlier gate on its paths is met.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

1. **Recheck, then census.** On the cut base:
   - Re-read the `itree_id` producer and the `Ret`/`Vis` role readers. If the
     checked-id basis differs from `evt_a9zrvmkrazcr`, stop for an Architect
     refresh (Spec Leader `evt_5fw7p5jnrxq3x`).
   - Count, token-aware, the Ken-source uses of the prelude `Ret` and `Vis`
     that scoping breaks. Exclude Rust identifiers and user-declared
     same-spelling constructors. State the corpus and its membership rule.
2. **One spec+crates branch.**
   - Admit `ITree`.
   - Scope `Ret`/`Vis`.
   - Change the 30 §4 roster and count, the ledger rows and
     `PRELUDE_FLOOR_NAMES` (23 to 24) together.
   - Migrate the census population token-aware, as slices 1 and 2 did.

   The Spec co-turn on the same branch owns the `spec/` text. CV votes on the
   `spec/` and `conformance/` paths.
3. **Artifact symbols from the checked leaf (amended).** As ruled:
   - add `ModuleState::scoped_constructor_leaves(parent_id)` in
     `modules.rs`, which maps each constructor id to its bare leaf through
     `constructor_members`;
   - in `compiler_driver.rs` `stable_symbols_for_env`, name each constructor
     of a scoped family by that leaf, keyed by `ctor.id`.

   This applies to every scoped family, not only ITree. Runtime code stays
   out of this WP; the backend's spelling keys for `Ret`/`Vis` are
   `RT-ITREE-ROLE-IDENTITY-NOT-SPELLING`.

## Acceptance

- **AC-1.** In both strict and legacy resolution modes:
  - `ITree` resolves to `itree_id`;
  - `ITree.Ret` and `ITree.Vis` resolve to their checked constructor ids;
  - bare `Ret` and `Vis` are refused as prelude references, each in its own
    independently executable row, with the mode's exact error;
  - a source declaration that binds `Ret` or `Vis` is accepted;
  - a same-name source declaration of `ITree` is refused.
- **AC-2 (amended).** The spec count equals `PRELUDE_FLOOR_NAMES.len()`,
  and `trusted_base()` is unchanged as an identity set.
  - Every moved hash is named, with the reason "scoped constructor symbols
    take the family leaf". This covers every scoped family (ExitCode,
    ProgramCaps, Coproduct, Instant, ITree, and ResourceKind if it moves).
    Comments that cite "qualified checked constructor symbols" are
    corrected.
  - "No other hash moves" is shown by a control, not asserted: the same
    qualified sources built with the scoped loop emptied measure the same
    hashes.
  - **Neutrality pin.** For each family in the scoped loop, plus
    ResourceKind, each constructor's stable symbol is looked up by its
    `GlobalId`. Its final component equals the kernel leaf and contains no
    `.`. One row fixes the ITree `Ret` symbol's `Display` to end
    `::ITree::Ret`.
- **AC-3 (mutation, QA).** Each mutant fails and is restored:
  - removing `ITree` from `PRELUDE_FLOOR_NAMES` reddens its AC-1 row;
  - admitting `ITree` unscoped reddens both bare-constructor refusal rows in
    both modes;
  - reverting the `compiler_driver.rs` hunk reddens both
    `ken-verify --test px8f_write_partition` and the neutrality pin.

## Stop conditions

- The checked-id basis has changed since `evt_a9zrvmkrazcr`. Stop for an
  Architect refresh.
- A migrated consumer whose meaning changes, rather than only its spelling,
  for example a runtime role read by spelling after the scoping point.
- Any kernel or trust change, or the strict-mode flip, which stays with
  L2-4.

Checks: 4 fired: `itree_id` exists in `prelude.rs` (`:287`, `:431`, and the
`(itree_id, ret_id, vis_id)` producer at `:1033`) on `ac5b0a002`, and
`PRELUDE_FLOOR_NAMES` has 23 entries (`modules.rs:239`).
`scope_constructor_family` is the slice 2 mechanism the Architect reviewed.
6 fired: the consumer count is stated as a word-grep ceiling, with its
corpus named and its known over-count classes listed. The migration
population is left to the deliverable's token-aware census.
