---
id: LANG-MODULE-PATH-OWNER-GATE
title: "Enforce 33 §3.1's one-owner-per-module-path rule in the elaborator (a hard ModuleOwnerClash in either load order, keyed on unit-ownership records), and re-home the ~36 tests that build same-spelling collisions through the now-forbidden foreign same-path claim without losing their identity coverage"
status: ready
owner: language
size: M
gate: architect
tier: T1
depends_on: [LANG-SESSION-SCOPE]
blocks: []
github: null
origin: "Spec 33 §3.1 as merged at 10cc33ba3 (SPEC-MODULE-PATH-SINGLE-OWNER; Architect evt_1f4vmg3gtqq9). Consumer census from the Adversary's post-merge probe (evt_3z13fah0hsn1g). Operator L2 objective 2026-09-25 (one resolution mode). Steward-filed per COORDINATION section 2."
---

# Module path owner gate

## Objective

The elaborator refuses a module declaration whose exact path another unit
owns, in either load order. Every test that used a foreign same-path claim
to build a same-spelling collision keeps what it pinned.

## Settled inputs

- **The rule** is 33 §3.1 at `10cc33ba3`: one owning unit per exact path, a
  hard owner-clash surface error in either order and whatever the member
  names. It is keyed on unit-ownership records, never on `globals`
  (Check 10). A strict child path is a distinct path.
- **The sites:** the inline `ModuleDecl` expansion (`modules.rs:3873`) and
  the file-load entry (`modules.rs:1753`).
- **The consumers are about 38, not the 2 the spec closeout named.** This
  is the Adversary's probe on `10cc33ba3`: a panic whenever a different unit
  claims an owned exact path, then
  `scripts/ken-cargo test -p ken-elaborator --no-fail-fast`. 38 failures,
  every one the probe panic:
  - 36 file-versus-in-memory claims that clash under any definition of
    in-memory unit identity (28 file-then-memory, 8 memory-then-file), all
    in `modules::namespace_effect_tests`, most labelled "Promise class:
    durable invariant (spec 33 §§3.1–3.3)";
  - 1 that depends on the in-memory unit identity choice
    (`in_memory_root_import_keeps_earlier_declared_child`,
    `modules.rs:6011`);
  - 1 probe artifact (`lang_mod_pair_floor_realization.rs:160`, no file
    unit).
  Other crates (ken-cli, the catalog harness) were not probed.
- **Why it is more than a rename.** These tests are the main identity-keyed
  coverage for classes, instances, derive, effect rows, fixity, prop-intro
  and re-export under a same-spelling collision. Turning them into clash
  assertions alone would make their resolver arm unreachable and retire that
  coverage.

## Deliverable

The owner gate, plus a disposition for every consumer: a
`ModuleOwnerClash` assertion, or a re-homed test that builds its collision
by a still-legal route and keeps its identity pin.

## Acceptance

- **AC-0 (D0).** Re-measure the consumer census across every crate (Check
  3). Define in-memory unit identity. Propose the still-legal collision
  construct for re-homing (for example distinct paths that import the same
  spelling). The Architect rules on both before any build.
- **AC-1.** The seed cases in `conformance/surface/modules/seed-modules.md`
  hold: a disjoint-member foreign block rejects in both orders, and the
  distinct-path and strict-child controls accept.
- **AC-2.** Each re-homed test still reddens under the resolver mutation it
  guarded before. List old test, new form, and the mutation.
- **AC-3.** Removing the gate turns each clash assertion green-accepting, so
  the gate is the observation. Full CI is the breadth gate.

## Stop conditions

- A consumer whose collision cannot be built by any legal route: stop to the
  Architect with the test.
- Any kernel, `trusted_base()` or spec change.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the child-2 checkpoint.
