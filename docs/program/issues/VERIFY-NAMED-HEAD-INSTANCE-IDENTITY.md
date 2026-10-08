---
id: VERIFY-NAMED-HEAD-INSTANCE-IDENTITY
title: "Since VERIFY-INSTANCE-OWNER-KEY a named-head instance's owner is its synthesized dictionary name {class}_instance_{head}, which is not injective and which a user declaration can spell, so two distinct instances (or an instance and a const) share one owner: one requires premise drops out of the package's obligations, the second falls to a #n fallback, and the hash depends on order. Give named-head instances an identity no other instance or declaration can take"
status: active
owner: verify
size: M
tier: T1
gate: architect
depends_on: [VERIFY-STRUCTURAL-HEAD-INSTANCE-IDENTITY]
blocks: []
github: null
origin: "Adversary finding evt_4n1d4dwq571kx on 06c037f92: a reporting regression of VERIFY-INSTANCE-OWNER-KEY (soundness-adjacent: a live premise is unreported). Separate from the structural-head WP, whose D0 ruling evt_407cerfqyf2yn covers structural heads only. Steward-filed per COORDINATION section 2."
---

# A named-head instance has its own identity

## Objective

Distinct named-head instances, and an instance and any user declaration,
never share a declaration symbol, owner or obligation id, in any
declaration order.

## Settled inputs (Adversary `evt_4n1d4dwq571kx`, at `06c037f92`)

- **The mechanism.** The owner is `synthesized_dictionary_name(..).canonical`
  (`modules.rs:4633-4646`), built as
  `format!("{resolved_class}_instance_{resolved_head}")` (`:3308`).
  `Scope::bind_local` accepts a re-bind of the same surface to the same
  canonical name (`:639-640`).
- **Shared prelude.** `class A carrier { la : carrier -> Int }`,
  `class A_instance_B carrier { lb : carrier -> Int }`,
  `data B_instance_C : Type where { MkBC : B_instance_C }`,
  `data C : Type where { MkC : C }`,
  `const need : Int requires Equal Int 0 1 = 0`, `const k : Nat = Zero`.
  Package `zz_adv_ownerkey`, target `k`, NonRuntime.
- **R1.** `instance A B_instance_C { la = \x. need }` and
  `instance A_instance_B C { lb = \x. need }` both give
  `A_instance_B_instance_C`. Result: one obligation
  (`obl:A_instance_B_instance_C.requires.0`) where there should be two,
  declarations `[A_instance_B_instance_C, A_instance_B_instance_C#1]`, and
  an order-dependent hash (`0x946d9d38a813e883` vs `0xf4bb8d2a1b7a530`).
  Same in one file and across two files in either order.
- **R2.** `class Lbl carrier { mark : carrier -> Int }`,
  `instance Lbl A { mark = \x. need }` and
  `const Lbl_instance_A : Int = need` give one obligation and an
  order-dependent hash.
- **Pre-WP control.** With `modules.rs` from `e5ec530dc`, every row gives
  two obligations.
- The dictionary name is also an importable surface binding, so the
  repair must keep or deliberately replace that binding.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

**D0 ruling (Architect `evt_2wbp7vr08k3pf`, at `b5ba6619c`): refuse the
collision.** The identity stays `{resolved_class}_instance_{resolved_head}`,
which is also the importable binding (10 catalog files depend on it). A
declaration whose identity key is already bound in `elab.globals` to a
different checked GlobalId is refused with a new typed
`DeclarationIdentityCollision`. The error names both declarations, sorted,
so the message does not depend on declaration order. The guard sits at the
module-path choke point `elaborate_checked_as`. The structural WP's
same-instance-key predicate is the only exemption.

1. **AC-0 (measure and report before building).**
   - For named `instance`, `derive` and `const`, the key each inserts into
     `elab.globals` equals the `owner` passed to `elaborate_checked_as`.
   - R1 and R2 refuse in all 12 layouts and orders with the guard in
     place.
   - A data constructor spelled like a dictionary
     (`data D : Type where { Lbl_instance_A : D }`), in both orders.
     Constructors enter globals by another path. Report the result; if
     it collapses too, it goes to the Steward as a residual and is not
     folded in.
2. **The guard and its typed error.**
3. **The rebind exemption (Architect `evt_210gpmk42hsys`, after CI red on
   `9a6e77e11`).** A minted name is a function of the canonical
   `(class, head)` spelling pair (`modules.rs:3602`). Record that pair for
   each minted identity and compare it:
   - **same pair, new checked ids:** a later owner lawfully rebinding the
     spelling. **Admit.** The `globals` key moves; the old dictionary stays
     reachable through `instances_by_id` and the provider's
     `file_export_ids`;
   - **different pairs minting one key (R1):** **refuse**;
   - **a prior binding that was never minted** (R2 const before instance,
     or a class, data or const prior): **refuse**, as now;
   - **the same instance key:** the existing exemption.

   The edit is the Architect's sketch:
   - `head_canonical` on `SynthesizedDictionaryName`;
   - a `MintedSpelling { class, head }` type;
   - `ModuleState.minted_spellings: HashMap<GlobalId, MintedSpelling>`,
     scrubbed in `scrub_global_ids`;
   - `minted: Option<MintedSpelling>` in place of `minted_identity` at the
     `expand_scope` caller. `elaborate_checked` passes `None`;
   - `rebound_spelling = previous_minted.is_some() && previous_minted ==
     minted` added to the refusal condition, and the spelling recorded on
     admission.

   No further exemption is added here. Any later over-refusal of a lawful
   rebind is closed by `VERIFY-GLOBALS-IDENTITY-CHECKED-INSERT`.

## Acceptance

Scope narrowed by Architect `evt_7r6p7bj8wj22y`: the guard reads `prior`
only for an identity the elaborator mints (a dictionary canonical or a
structural symbol). A user-spelled key that a later unit rebinds is lawful
shadowing. A user declaration arriving after a minted identity, at its
spelling, is the residual of `VERIFY-GLOBALS-IDENTITY-CHECKED-INSERT`.

- **AC-1.** These nine rows, across one-, two- and three-file layouts, are
  refused with the typed error naming both declarations: R1 in both
  orders, and R2 with the const before the instance. R2 with the instance
  before the const still admits with one obligation, a `#1` fallback and
  order-dependent hashes, as do a class, data or const declared after a
  minted dictionary. Those rows are recorded as the successor's residual,
  not as passes. The non-colliding controls give one obligation per live
  premise, no `#n` fallback and an order-independent hash.
- **AC-2 (control).** The predecessor's named-head and `derive` rows and
  the structural-head rows keep their owners and obligation ids, and no
  catalog package is refused or changes hash.
- **AC-3 (mutation, QA).** Deleting the guard reddens the nine AC-1
  refusal rows: they admit again with one obligation, a `#1` fallback and
  order-dependent hashes.
- **AC-6 (control).** `lang_instance_registry_identity_key` passes 5/5
  with no changes to the test.
- **AC-6c (control).** `modules::namespace_effect_tests` passes with no
  changes to the test, including the four rebind rows that reddened
  `9a6e77e11` in CI:
  `generated_instance_import_refuses_stale_provider_in_either_order`,
  `generated_derive_import_refuses_stale_provider_in_either_order`,
  `same_head_class_instances_search_by_selected_provider_identity` and
  `derive_keeps_selected_class_and_data_ids_after_shadow`. The full
  `scripts/ken-cargo test -p ken-elaborator --lib` passes and is part of
  the QA gate.
- **AC-7.** On one shared environment, a second `data Foo` admits, and
  `instance Pick Foo` on the rebound `Foo` also admits:
  - `globals["Pick_instance_Foo"]` is the new id;
  - the old dictionary is still in `instances_by_id` under its checked
    (Pick, old Foo) key;
  - a `where Pick Foo` search on the new `Foo` selects the new
    dictionary.
- **AC-8 (mutation, QA).** Passing a minted spelling at the user-spelled
  caller reddens AC-6.
- **AC-9 (mutation, QA).** Forcing `rebound_spelling = false` reddens
  exactly the four AC-6c rows and AC-7, while AC-1 stays green.

## Stop conditions

- A catalog or corpus package is newly refused or changes hash: stop with
  the list.
- Any kernel, `trusted_base()` or spec change.
