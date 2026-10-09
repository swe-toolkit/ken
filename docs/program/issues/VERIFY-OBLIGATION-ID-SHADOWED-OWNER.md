---
id: VERIFY-OBLIGATION-ID-SHADOWED-OWNER
title: "A package obligation id is built from its owner's spelling ({def_name}.requires.{n}), so when a later file shadows or lawfully rebinds that spelling (a user const x over x, or a second Pick_instance_Foo on a rebound Foo), the two owners mint one id and semantic.obligations.insert silently drops one owner's requires premise, in a source-order-dependent way. Key obligation ids on the owner's stable checked identity so every live premise survives"
status: active
owner: verify
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Adversary M8 finding evt_mxa6e26p25e2 on 176646c5f (VERIFY-NAMED-HEAD-INSTANCE-IDENTITY); pre-existing, not introduced by that merge. Spec 46 §3.2: obligation ids derive from stable inputs. Steward-filed per COORDINATION section 2."
---

# Every live premise keeps its own obligation id

## Objective

Every `requires` premise of a retained checked declaration appears in the
package's obligations under its own id, and the premise set does not depend
on source order.

## Settled inputs (Adversary `evt_mxa6e26p25e2`, at `176646c5f`)

- **The mechanism.** `extract.rs:165` builds the id from the owner's
  spelling: `format!("{}.requires.{}", def_name, obl.id)`. Then
  `compiler_driver.rs:3410` runs `semantic.obligations.insert(obligation,
  goal)`. Two owners with distinct GlobalIds and distinct stable symbols
  (`X` and `X#1`) mint one id, and the insert overwrites one goal.
- **The witnesses** (`compile_ken_package_sources`, target `k`,
  NonRuntime). `src/c.ken` declares `class Pick carrier { sel : carrier ->
  Int }`, `const need1 : Int requires Equal Int 0 1 = 0`, `const need2 :
  Int requires Equal Int 0 2 = 0` and `const k : Nat = Zero`.
  - **Q3 (minted rebind).** `src/a.ken` has `data Foo` (`MkOld`),
    `instance Pick Foo { sel = \x. need1 }` and `const z : Int where Pick
    Foo = d.sel MkOld`. `src/b.ken` has `data Foo` (`MkNew`) and
    `instance Pick Foo { sel = \x. need2 }`. It admits with one
    obligation, `obl:Pick_instance_Foo.requires.0`, which holds `need2`'s
    goal, although `z` uses the old dictionary.
  - **Q1.** a then b keeps only `need2`; b then a keeps only `need1`. The
    hashes are `0xf385bb5369f36a55` and `0x72f2ee57ca5bad2c`.
  - **Q4 (user shadow).** `a.ken`: `const x : Int = need1` and `const y :
    Int = x`. `b.ken`: `const x : Int = need2`. One obligation,
    `obl:x.requires.0`, holds `need2`'s goal, while `y` depends on the
    shadowed `x`.
  - **P4.** A second `class Pick` plus `instance Pick Foo` in a later file
    admits with one obligation.
  - **Controls.** `a.ken` alone and `b.ken` alone each give one
    obligation, and the two differ.
- **The probe** is `zz_adv_rebind.rs` in the Adversary's scratchpad.
- **Not covered elsewhere.** `VERIFY-GLOBALS-IDENTITY-CHECKED-INSERT`
  keeps lawful shadows as class (c), so these rows stay admitted under it.
  `VERIFY-OBLIGATION-STABLE-IDENTITY` (merged `fe21d61c4`) made ids
  session-independent, not owner-distinct.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. **D0 (measure only).** Census every producer of an obligation id (each
   `ObligationKind` arm in `extract.rs`, and any other site that inserts
   into `semantic.obligations`). Report each id's key inputs, and the
   catalog packages whose obligation ids or hashes would move if the id
   were keyed on the owner's stable symbol. The Architect rules the key and
   any hash movement from the D0.
2. **The ruled repair** (Architect `evt_70cf80pvp8gek`, key K2). The id
   is minted at the emitter from the owner's stable symbol:
   `obl:{origin.components[1..].join(".")}.{suffix}`, from
   `symbols[owner]` (compiler_driver.rs:3383).
   - `extract.rs` factors its one exhaustive `ObligationKind` match into
     `obligation_id_suffix`, with no `_ =>` arm. `lift_obligation` keeps
     its id strings byte-for-byte, and `v2_extract_with_suffixes` pairs
     each triple with its suffix.
   - `owned_v2_obligations` returns `(GlobalId, String, ObligationTriple)`.
     Its three callers and both signatures take the 3-tuple.
   - `add_obligation_metadata` refuses any re-insert with
     `DuplicateObligationId { obligation, first_owner, second_owner }`,
     including the same owner and goal. Do not loosen it to "identical is
     fine".

## Acceptance

- **AC-1.** In Q1 (both orders), Q3, Q4 and P4, both owners' premises
  appear, each under its own id. In each order the id set is identical:
  `{X#1.<suffix>, X.<suffix>}`. Each id's goal is the premise of the
  declaration its stable symbol names, pinned from source text by hand:
  a then b gives `#1` to need1 and plain to need2; b then a gives `#1` to
  need2 and plain to need1. Each id's `obligation_metadata.origin` equals
  that declaration's symbol. The goal pin identifies the premise from its
  content (the need1/need2 predicate), not from the minted id or `origin`,
  since both come from `symbols[owner]`.
- **AC-2 (control).** The single-file controls, the
  `VERIFY-NAMED-HEAD-INSTANCE-IDENTITY` suite, `modules::namespace_effect_tests`
  and `VERIFY-OBLIGATION-STABLE-IDENTITY`'s pins keep their results. Every
  one of the 61 measured catalog images keeps its obligation-id set and
  core hash; any movement is a stop. The full `scripts/ken-cargo test -p
  ken-elaborator --lib` passes and is part of the QA gate.
- **AC-3 (mutations, QA).** Each reddens its test and is restored:
  - (a) minting from the spelling (`triple.id.0`) at the emitter makes Q1
    refuse through the checked insert, so AC-1 reddens;
  - (b) replacing the checked insert with a plain `insert` reddens an
    in-crate `#[cfg(test)]` unit test of `add_obligation_metadata`: two
    GlobalIds mapped to one StableSymbol, with one suffix, expect
    `Err(DuplicateObligationId { .. })` naming both owners.

## Stop conditions

- A catalog or corpus package is newly refused, or a hash moves that the
  Architect has not ruled: stop with the list.
- Any kernel, `trusted_base()` or spec change.

## Carry

Declaration stable symbols depend on declaration order for a shadowed name
(`stable_symbols_for_env`), and obligation ids inherit that. A
source-independent declaration identity is a separate redesign.
