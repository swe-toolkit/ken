---
id: SPEC-MODULE-PATH-SINGLE-OWNER
title: "Specify that a qualified module path has exactly one owning compilation unit, and that a foreign module declaration into an owned path is a hard owner-clash surface error in either load order; seed conformance for it"
status: ready
owner: spec
size: S
gate: architect
tier: T1
depends_on: []
blocks: []
github: null
origin: "Architect ruling 2026-09-27 (evt_1f4vmg3gtqq9) choosing (a) on the policy fork Spec returned (evt_4gdjdqwghtfss), raised by the Adversary on b541ca0ef (evt_7e34ja43kz2ev). Steward-filed per COORDINATION section 2."
---

# One owner per module path

## Objective

`/spec` states who owns a qualified module path, and conformance pins it,
so that `P.f` always means "f from the one unit that owns P".

## Settled inputs

- **The gap** (spec-author, `evt_4gdjdqwghtfss`): the 33 §1 duplicate rule
  is unit-local. 33 §3.1-3.2 and 39 §2.0 constrain file and import path
  resolution, not a foreign inline block. ADR 0014 MRES-3(a)/MRES-5 do not
  settle cross-unit ownership.
- **The rule** (Architect `evt_1f4vmg3gtqq9`, binding):
  1. A qualified module path has exactly one owning compilation unit. A
     file unit owns its own path and the inline descendant paths it declares
     itself; an in-memory unit owns the inline paths it declares first.
  2. A module declaration whose path is owned by a different unit is a hard
     owner-clash surface error, at declaration or load, in either order and
     whatever the member names. It admits no second checked declaration or
     interface.
  3. Ownership is of the exact path. A strict child `P.Q` that P's unit did
     not declare is a different path, governed by rule 1 on its own.
  4. Last-writer-wins on the host `globals` table is not a semantics.
- **As built** (measured on `b541ca0ef`): `Decl::ModuleDecl` at
  `modules.rs:3873-3878` qualifies the child prefix with no ownership check,
  and the later writer silently rebinds the qualified `globals` key.

## Deliverable

A 33 §3 clarification carrying rules 1-4, plus a conformance seed.

## Acceptance

- **AC-1.** The clarification states rules 1-4 in spec vocabulary and names
  the diagnostic class.
- **AC-2.** The conformance seed has:
  - a disjoint-member foreign inline block that rejects in both load orders;
  - a distinct-path control that accepts;
  - a strict-child control (`module P.Q { ... }` in unit X, where Y owns P
    and declares no `P.Q`) that accepts;
  - after a rejection, no new declaration in the checked environment or the
    interface.
- **AC-3.** The conformance validator votes on the exact tip, and the
  Architect approves.

## Stop conditions

- Any kernel or `trusted_base()` change, or a rule outside 1-4.
- The implementation (the ownership check and migrating the two
  coexistence tests `file_root_import_uses_its_own_exports_after_memory_shadow`
  and `file_facade_uses_source_file_exports_not_memory_shadow`) is Language
  work in the flip frame's `globals`-writes carry, not this node.
