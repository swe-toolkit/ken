---
id: LANG-CLASS-FORWARD-REFERENCE-STALE-IDENTITY
title: "A module that uses its own class before declaring it binds that use to a different class under the same key when one is already loaded: checked_class_id falls back to globals.get(name), so a file-loaded A.View answers a forward reference to the module's own View. The same source on a fresh environment is refused. Make the outcome depend on the module's scope, not on the global table's history"
status: merged
owner: language
size: S
tier: T2
gate: architect
depends_on: [LANG-CLASS-IDENTITY-BY-CHECKED-ID]
blocks: []
github: null
origin: "Adversary finding on LANG-CLASS-IDENTITY 1030ee887 (evt_4mynvxwxacaq9), severity correctness: it over-accepts against the wrong class identity. It is a residual of that WP's objective (spec 33 §3.3), present on the parent too. Serves the L2 objective. Steward-filed per COORDINATION section 2."
---

# A forward class reference never reaches another module's class

## Objective

Inside a module, a reference to that module's own class resolves by the
module's scope. It never reaches another declaration that happens to
hold the same globals key.

## Settled inputs (main `051377eec`)

- **The fallback.** `checked_class_id` takes
  `selected.or_else(|| globals.get(name))`
  (`crates/ken-elaborator/src/elab.rs:19880`). The type position takes
  the same `globals` route.
- **The forward reference.** A local class prebound in `modules.rs`
  (`bind_local`, near `:4393`) carries no checked id before its
  barrier, so the use falls through to `globals["A.View"]`.
- **The rows (Adversary `evt_4mynvxwxacaq9`).** In one `ElabEnv`, first
  load file `A.ken` (`pub class View a { get : a -> a }`, with
  `instance View Nat`) through `elaborate_module_from_roots_strict`.
  Then elaborate in memory:
  - row F: `module A { fn early (d : View Nat) : View Nat = d` followed
    by `pub class View a { other : Nat } }`. It is accepted, typed at
    the file's class.
  - row W: the same with `const early : Nat where View Nat = d.get Zero`.
    It is accepted and uses the file's `get` field, which the module's
    own `View` does not have.
  - The control is row F on a fresh env, which is refused with
    `UnresolvedCon { name: "A.View" }`.
- **The intended outcome.** The merged test
  `local_class_is_prebound_for_collision_and_type_use`
  (`lang_class_identity_checked_id.rs:305`) records the GAP that "a
  reference to a class before its ordered barrier stays refused".

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

Rows F and W get the fresh-env outcome whatever the session already
holds, with the rows pinned in `lang_class_identity_checked_id.rs`.
Before the implementer starts, the Architect confirms that the outcome
is refusal rather than resolution to the local class.

## Acceptance

- **AC-0 (Architect).** Refusal or local resolution, ruled on the
  landed base.
- **AC-1.** Rows F and W, after the file load, have the same outcome as
  on a fresh env. Each class reference names the module's identity in
  the refusal or in the elaborated type.
- **AC-2 (control).** An imported class reference and a reference after
  the barrier still resolve to their checked ids, and the merged
  `namespace_effect_tests` and LANG-CLASS-IDENTITY rows are unchanged.
- **AC-3 (mutation, QA).** Restoring the `globals.get(name)` fallback
  for a prebound local name makes row F accept again.

## Stop conditions

- The repair needs a change to the module barrier order or to spec 33.
- Any kernel or trust change.
