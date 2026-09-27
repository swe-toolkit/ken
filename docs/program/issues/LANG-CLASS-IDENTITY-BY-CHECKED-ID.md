---
id: LANG-CLASS-IDENTITY-BY-CHECKED-ID
title: "Resolve every class reference by its checked identity, so two modules that each declare a same-spelled class cannot collide through the bare-spelling class index or globals entry"
status: ready
owner: language
size: M
gate: architect
tier: T2
depends_on: [LANG-SESSION-SCOPE]
blocks: []
github: null
origin: "Architect ruling 2026-09-27 (evt_ax2kbwvatm8f, LANG-SESSION-SCOPE stop 10): class declarations register their record type in env.globals and class_env.current_names under the bare spelling, not module-qualified -- a Check 10 hazard. Operator L2 objective 2026-09-25 (one resolution mode). Steward-filed per COORDINATION section 2."
---

# Class identity by checked id

## Objective

A class reference resolves to the class its scope selects, keyed on the
checked `GlobalId`, never on a bare spelling. Two modules that each declare
`class View` both elaborate, and each client gets the one it imported.

## Settled inputs (measured on `4a8f27022`)

- **Registration is spelling-keyed.** `ClassEnv::register_class` and
  `register_record` (`crates/ken-elaborator/src/classes.rs:275-290`) write
  `current_names: HashMap<String, GlobalId>` under the bare name, and the
  class record type enters `env.globals` the same way (Architect
  `evt_ax2kbwvatm8f`: only `View`/`IndexedView`/`RefinementView` keys found).
  The id-keyed store `named_field_owners` keeps both entries.
- **Three production readers fall back to the spelling index**
  (`ClassEnv::class`, `classes.rs:257`):
  - `constraint_instance_id` (`elab.rs:10612`), when `class_id` is `None`;
  - the `RType::RCon(name, _)` arm (`elab.rs:10735`);
  - `checked_class_id` (`elab.rs:12740`), when `selected` is `None`.
  Import-selected references already carry an id.
- **No catalog collision today.** `grep` over `catalog/` finds each class
  spelling declared once (`Functor`, `Eq` and `Traversable` repeat only in
  `conformance/` fixtures). The hazard is latent, so this is a guard, not a
  repair of a live wrong answer.

Re-measure before building. If a settled input is false, stop and report.

## Deliverable

Each of the three readers gets its id from the checked selection: an import
selection, or the current unit's own checked class id. The spelling fallback
is deleted. The bare-spelling `globals` entry for a class record type is
module-qualified or removed, whichever the Architect rules at AC-0.
`ClassEnv::class` remains only as a diagnostic view, or is deleted if it
has no remaining consumers.

## Acceptance

- **AC-0.** A two-module fixture: A and B each declare `pub class View`,
  and client C imports A's. Record the base behaviour: the id C's
  constraint, instance head and `RCon` reference each select. Enumerate
  every consumer of `current_names` and of class-record `globals` keys,
  across `crates/*/src`, `crates/*/tests`, `r_layer_tests`, `conformance/`
  and `examples/` (Check 3). The Architect rules the `globals` disposition.
- **AC-1.** On the candidate, C's three references select A's id, and a
  C' importing B's selects B's. Restoring any one spelling fallback turns
  its row red.
- **AC-2.** The `conformance/` class seeds and the catalog class suites
  stay green (Full CI).

## Stop conditions

- Any kernel, `trusted_base()` or spec change.
- A consumer whose only route to a class is a bare spelling with no
  checked selection: stop and return it to the Architect.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the child-2 checkpoint.
