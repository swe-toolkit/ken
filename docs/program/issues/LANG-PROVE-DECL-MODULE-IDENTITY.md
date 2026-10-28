---
id: LANG-PROVE-DECL-MODULE-IDENTITY
title: "After LANG-FOREIGN-LAW-TEMPORAL, a prove declaration is the only named source declaration left outside is_qualifiable, so a prove inside a module likely keeps a bare globals key and the same spelling in two modules is refused as a duplicate. Measure the leak on the landed base; if it reproduces, give prove the module-qualified, scope-bound identity the other kinds have"
status: merged
owner: language
size: S
tier: T1
gate: architect
depends_on: [LANG-FOREIGN-LAW-TEMPORAL-MODULE-IDENTITY]
blocks: []
github: null
origin: "Architect residual 6 in the LANG-FOREIGN-LAW-TEMPORAL-MODULE-IDENTITY AC-0 (evt_4dc9n9em4kctp), Check 7 run on the next consumer of the same predicate: not measured there. Serves the L2 objective of one resolution mode. Steward-filed per COORDINATION section 2."
---

# A prove declaration has module identity

## Objective

A `prove` declaration inside a module is bound by the module's scope
under its qualified name, and its obligation id follows that name. A
client that does not import it cannot see it in any resolution mode.

## Settled inputs (main after LANG-FOREIGN-LAW-TEMPORAL)

- **The predicate.** `is_qualifiable`
  (`crates/ken-elaborator/src/modules.rs:3432`) lists every named
  source kind except `ProveDecl` once foreign, law and temporal join it.
  `decl_namespace_effect` (`:4230`) still gives `ProveDecl` a name.
- **The form.** `prove name : φ` (`parser.rs:2855`). `pub_eligibility`
  refuses it as "a `prove` obligation" (`parser.rs:5786`).
- **The sentinel.** The `prove p` row of
  `other_unqualified_kinds_refuse_same_spelling_in_two_modules`
  (`tests/verify_instance_owner_key.rs:183`) refuses the same spelling
  in two modules with DuplicateDefinition. Its doc comment says to
  retire the refusal once scoping admits the pair.
- **Measured and ruled (Architect AC-0 `evt_3ypcqjr9t2q4s`, base
  `372a6a1a6`).** The leak reproduces. `qualify_decl_name`
  (`modules.rs:2250`) has no `ProveDecl` arm, so a module `prove` keeps
  its bare globals key. The anchors moved: `is_qualifiable` is at `:3478`,
  and the sentinel is `prove_decl_still_refuses_same_spelling_in_two_modules`
  (`verify_instance_owner_key.rs:203`). The obligation id becomes
  `M.p.prove` with no id-specific change, because every id consumer
  derives from the qualified name. The repair is four arms in
  `modules.rs` plus two test migrations (the sentinel, and
  `lang_mod_nat_floor_realization.rs:759` to `Entry.LocalProve`), in
  `/workspaces/ken/local/architect-shared/prove-module-identity-d0/repair.patch`
  (sha256 `15f71dfa…`). Format only the new hunks.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The Architect measures the leak first. If it does not reproduce, this
node closes without repair. Otherwise the change in `modules.rs`, with
the sentinel row migrated to two admitted owners `M.p` and `N.p`.

## Acceptance

- **AC-0 (Architect).** The four-mode measurement on the landed base
  (Strict roots, Legacy roots, the direct API after roots, inline
  modules), and a census of the consumers of the prove name and its
  obligation id, by name and by mechanism. It rules whether the
  obligation id becomes module-qualified.
- **AC-1.** In a new `tests/lang_prove_decl_module_identity.rs`,
  modelled on `lang_foreign_law_temporal_module_identity.rs`, and in
  each of the four modes: the same `prove p` in two modules is admitted
  with distinct owners; an unimported use gives the per-mode surface
  class; and an in-module reference resolves. The D0 probe
  `arch_prove_module_identity_d0.rs` prints rather than asserts, so
  convert it; do not ship it as is.
- **AC-2 (control).** Top-level `prove` obligations keep their ids.
  The LANG-FOREIGN-LAW-TEMPORAL rows are unchanged, and
  `trusted_base()` is unchanged as an identity set.
- **AC-3 (mutation, QA).** Each mutant fails and is restored:
  - removing the `ProveDecl` arm from `qualify_decl_name` turns the
    sentinel red, with the duplicate refusal back;
  - removing `ProveDecl` from `is_qualifiable` turns the AC-1 in-module
    pin red in each mode. It leaves the duplicates admitted, so the
    sentinel alone does not catch it (Architect measurement);
  - the `record_checked_local` arm is unguarded parity (Architect
    `evt_1tays25fx53yn`): no fixture can observe it while `pub prove` is
    ineligible. The WP that admits `pub prove` must add the same-expansion
    re-import pin that reddens it.

## Stop conditions

- A spec change, or a kernel or trust change.
- `pub prove` eligibility, which is not in scope.
