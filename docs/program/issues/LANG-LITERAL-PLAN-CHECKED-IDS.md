---
id: LANG-LITERAL-PLAN-CHECKED-IDS
title: "A literal arm must be selected by its literal's value under the carrier's own view and comparator, but plan_literal_comparator still finds the String, Bytes and fixed-width views and the eq_int comparator by spelling in the flat globals map, so a source fn eq_int or fn string_to_list_char changes which arm a match selects and a checked theorem certifies the wrong result. Key every literal plan id on the checked identity registered for its carrier, and pin the takeover from source"
status: ready
owner: language
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Adversary evt_39engjhksagqs on LANG-PRELUDE-FLOOR-CONSOLE-AND-CHAR (05a15edee): the merge keyed the String plan's eq_char on Char's registered id, and the sibling fields still read cx.globals by spelling. L2 objective: one resolution mode, no bare name resolved through the flat globals map (operator 2026-09-25). Steward-filed per COORDINATION section 2."
---

# Literal patterns use their carrier's own comparator

## Objective

Every id that `plan_literal_comparator` puts into a literal plan is the
checked identity registered for that carrier. No source declaration,
on any route, changes which arm a literal match selects.

## Settled inputs (main `05a15edee`)

- **The lookup.** `literal_builtin` is `cx.globals.get(name)`
  (`elab.rs:23849`). Its 11 call sites read:
  - the FixedWidth plan's eight `*_to_int` views and `eq_int`
    (`:23936`, `:23937`);
  - the String plan's carrier `String` and view `string_to_list_char`
    (`:23981`, `:23991`);
  - the Bytes plan's carrier `Bytes`, view `bytes_to_list`,
    `uint8_to_int` and `eq_int` (`:24023`, `:24039` to `:24041`);
  - `List`, `Nil` and `Cons` (`:24181` to `:24183`).
- **The precedent.** The String plan's `eq_char` and the Int, Float,
  Float32 and Char comparators already come from
  `numeric_env.classify_eq` on the carrier id (`:23909`, `:23994`).
- **Measured defect** (Adversary evt_39engjhksagqs, `ken check` on
  `05a15edee`). A file declaring `fn string_to_list_char` returning
  `Nil`, or `fn eq_int` returning `True`, checks a theorem that a
  literal match selects an arm its literal does not match. Without the
  declaration the same theorem is refused with `TypeMismatch`. Through
  `ElabEnv::elaborate_decl`, a source `fn eq_int` replaces
  `globals["eq_int"]`. The roots loader is not affected.
- **The existing pin is blind to the route.**
  `string_literal_comparison_uses_char_registered_id_despite_flat_name_takeover`
  (`tests/lang_match_literal_patterns.rs:91`) injects the takeover with
  `env.globals.insert`, not from source.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

One language branch. Each of the 11 sites takes the checked id from the
registration that owns it (the numeric environment, the prelude's
recorded ids, or a new recorded field next to them), never from
`cx.globals` by spelling. `literal_builtin` is deleted or no longer
reads `globals`. The Architect reviews `crates/`.

## Acceptance

- **AC-1 (source takeover, both routes).** For each of `eq_int`,
  `string_to_list_char`, `bytes_to_list`, `uint8_to_int` and one
  fixed-width view, a source declaration of that name, then a literal
  match and a theorem that its arm is the one the literal does not
  match, is refused on `ken check` and through `elaborate_decl`. A
  control without the declaration gives the same refusal, and the true
  theorem checks with the declaration present.
- **AC-2 (mutation, QA).** Restoring `literal_builtin`'s spelling
  lookup for `eq_int` makes AC-1's `eq_int` row accept.
- **AC-3 (sweep).** The handoff lists every remaining
  `globals.get("<literal>")` in `crates/ken-elaborator/src` outside
  tests, and states for each whether source can reach it. Each reachable
  one is repaired here or named for the flip.

## Stop conditions

- A site has no registration that records its id, and adding one needs
  a kernel or trust change.
- A repair changes a literal plan's shape or a match's result on a
  program without a takeover.

Checks: 10 fired: `literal_builtin` reads the flat globals table by
spelling at 11 sites; the String `eq_char` repair keyed only one field.
3 fired: `globals.get("…")` in elaborator src outside tests also has 9
hits in `modules.rs`, `decimal_char.rs` and `lib.rs`; AC-3 classifies
them. 8 fired: AC-1 injects the takeover from source on both routes, so
a repair that only guards `globals.insert` fails it, and AC-2's mutant
reddens the `eq_int` row.
