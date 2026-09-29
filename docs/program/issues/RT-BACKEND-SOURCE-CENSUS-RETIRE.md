---
id: RT-BACKEND-SOURCE-CENSUS-RETIRE
title: "Retire the backend's source-text census family in core/tests/control.rs: the five consumers of BACKEND_PRODUCTION_SOURCES assert facts about Rust source text, so any behaviour-neutral module addition reddens them; replace each with its named behavioural control or retire it to Architect review, per the Architect's per-consumer ruling"
status: ready
owner: runtime
size: M
gate: architect
tier: T1
depends_on: []
blocks: [RT-FRAME-MARKER-ONCE]
github: null
origin: "Architect ruling 2026-09-29 (evt_2ssnqgwmj2bce) on the RT-FRAME-MARKER-ONCE QA block (evt_3cbg2t9f5r8wx, evt_4t0sefwbcs22f): the census family conflicts with the operator's prohibited-subject rule (2026-07-26, 'Tests should focus on behavior'). A module added anywhere reddens it, as FRAME-MARKER's frame_validation.rs did in PR #4346. Steward-filed per COORDINATION section 2 so FRAME-MARKER can land without a text-census row."
---

# Retire the backend source-text census

## Objective

No test in `ken-runtime` fails because a production module was added,
renamed or re-commented without a change in behaviour. The properties the
census guarded stay protected, each by its named replacement.

## Fixed inputs (read at `46b12fade`)

- **The roster** is `BACKEND_PRODUCTION_SOURCES`
  (`crates/ken-runtime/src/cranelift_backend/lowering/core/tests/control.rs:3553`),
  a list of `include_str!` source texts. It has five consumers. The
  Architect's disposition for each (`evt_2ssnqgwmj2bce`):

  | Consumer | Property | Disposition |
  |---|---|---|
  | A `correspondence_adds_no_emitted_unit_to_the_production_census` (`:2709`) | emission only at census-listed units | replace with the behavioural counters `units::b2f_last_unit_emission` (`units.rs:210`) and `seed_material::b2f_last_seed_material_emission` (`seed_material.rs:118`). Its own doc calls the text census fail-open |
  | B `the_backend_production_surface_inventory_is_closed` (`:3869`) | the roster is the whole surface | retire; it exists only to close A, C, D and E |
  | C `the_entry_carrying_types_are_module_private` (`:4297`) | `PlannedExpr` and `StaticNodeId` stay private to `static_transition` | retire to Architect review (AC-0 ruling `evt_170pr1hbj2e8s`); no compile-fail pin |
  | D `no_collection_is_keyed_by_a_scheduling_entry` (`:4373`) | no body is selected, and no occurrence filed, by scheduling entry | narrowed to that hazard and carried by the planner's (b) `keying_selection_by_the_scheduling_entry_does_not_resolve_the_body` and (c) `filing_two_occurrences_under_one_origin_is_refused`, once AC-2 shows each reddens (`evt_170pr1hbj2e8s`) |
  | E `the_owner_classification_has_a_closed_production_naming_inventory` (`:4499`) | closed inventory of files naming `SemanticOwner` | retire; ownership moves to Architect review |

- **Rule.** Operator 2026-07-26: "Test oracles that assert facts about source
  code, catalog, or documentation lines are an invitation for failure and
  delay. Tests should focus on behavior." (`agent/playbooks/build/qa-test-design.md`).

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## Deliverable

`BACKEND_PRODUCTION_SOURCES` and its five consumers are deleted. A becomes a
behavioural test over the emission counters. D is carried by the planner's
(b) and (c) controls. B, C and E retire, and each retirement is recorded in
the test file's comments with a pointer to this frame. There is no
production change.

## Acceptance

- **AC-0 (done).** Measured at `18d969847` (`evt_7x68hrt71693a`): (b) and
  (c) stay green when `StaticNodeId` is widened and when an unused
  entry-keyed map is added. Architect ruling `evt_170pr1hbj2e8s`: C retires
  to review; D narrows to its behavioural hazard.
- **AC-1.** A behaviour-neutral production module addition, such as an empty
  `mod` in `lowering/` on a scratch branch, leaves every `ken-runtime` test
  green.
- **AC-2 (controls).**
  - A new production emitter outside the census reddens A's behavioural
    replacement.
  - Routing body selection through a map keyed by `StaticNodeId` reddens
    (b).
  - Filing occurrences by entry instead of by origin, so two collide under
    one entry, reddens (c).
  - If either stays green, stop: the planner controls do not carry D.

## Limitations

Owned by Architect review, not by any test (`evt_170pr1hbj2e8s`):
- `PlannedExpr`/`StaticNodeId` module privacy is a review-owned property:
  widening either is a design change the Architect reviews on the merge
  Decision.
- A production collection keyed by a scheduling entry that neither selects a
  body nor files an occurrence is not test-observed; review owns it.

## Stop conditions

- Any production, kernel, spec or `trusted_base()` change (an operator
  question).
- The D replacement needs production code: stop to the Architect.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`, or the parked
  `wp/RT-NATIVE-TREE-MATCH-RUNTIME-SCRUTINEE`. `wp/RT-FRAME-MARKER-ONCE` is
  in review: do not move it here.
