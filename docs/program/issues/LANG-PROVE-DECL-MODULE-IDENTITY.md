---
id: LANG-PROVE-DECL-MODULE-IDENTITY
title: "After LANG-FOREIGN-LAW-TEMPORAL, a prove declaration is the only named source declaration left outside is_qualifiable, so a prove inside a module likely keeps a bare globals key and the same spelling in two modules is refused as a duplicate. Measure the leak on the landed base; if it reproduces, give prove the module-qualified, scope-bound identity the other kinds have"
status: active
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
- **Not measured.** Whether an unimported `prove` name or its
  obligation id is reachable from another module, by mode.

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
- **AC-1.** The same `prove p` in two modules is admitted with
  distinct owners. An unimported use from another module gives the
  per-mode surface class, and an in-module reference still resolves.
- **AC-2 (control).** Top-level `prove` obligations keep their ids.
  The LANG-FOREIGN-LAW-TEMPORAL rows are unchanged, and
  `trusted_base()` is unchanged as an identity set.
- **AC-3 (mutation, QA).** Removing the `ProveDecl` arm from the
  qualification restores the duplicate refusal.

## Stop conditions

- A spec change, or a kernel or trust change.
- `pub prove` eligibility, which is not in scope.
