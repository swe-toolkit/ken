---
id: SURF-gadt-elaboration
title: "Tracker node for the landed dependent-constructor elaboration slice (`34 §8`): points at `docs/program/wp/SURF-gadt-elaboration.md`, filed retroactively for the same tracker-generation reason as its parser/AST predecessor"
status: merged
owner: language
size: M
gate: none
depends_on: [SURF-gadt-parser-ast]
blocks: []
github: null
origin: "[[LANG-GADT-SEQUENCE-TRACKER-GAP]] D1 -- Steward release evt_6jb0p5w0zx69p, thread thr_11nhrpk58gkqt. Frame is `docs/program/wp/SURF-gadt-elaboration.md`; landed at squash-merge `3e3e8f41` (\"Reject recursive explicit data result indices\")."
---

## What this is

The tracker node for `SURF-gadt-elaboration`, the second dependent-constructor
build slice. Full design content lives in
`docs/program/wp/SURF-gadt-elaboration.md` -- not restated here.

## Status, measured at `origin/main` `f78b486d`

**Landed.** Squash-merge `3e3e8f41` is an ancestor of `f78b486d`. Test file:
`crates/ken-elaborator/tests/explicit_data_elaboration.rs`, 16 tests (covers
this slice, its `SURF-gadt-coverage-diagnostics` successor, and
`SURF-gadt-field-sugar`'s elaboration-side cases together -- see those nodes).

Per-AC discharge, obligation-keyed:

- **AC1 (non-indexed explicit family elaborates).** Search: a zero-index
  `data ... : Type where` declaration usable as a real constructor, not a
  postulate. Found:
  `non_indexed_explicit_family_elaborates_and_constructor_is_usable`
  (`explicit_data_elaboration.rs:24`) -- `Box`, asserts `ind.indices.is_empty()`
  and constructs `Boxed Int 3` through the ordinary introduction path.
- **AC2 (indexed `Vec` declaration elaborates).** Search: an indexed family
  with constructor targets at distinct index instances, re-checked by kernel
  positivity/universe. Found:
  `indexed_vector_family_records_indices_and_constructor_targets`
  (`explicit_data_elaboration.rs:79`) -- `Vector`, asserts one index, both
  constructors' `target_indices`, and that `VCons`'s target carries `n` in
  scope.
- **AC3 (proof-carrying constructor telescope elaborates).** Search: earlier
  binders in scope for later proof-field types, no CAT-5-specific behavior.
  Found: `proof_carrying_constructor_telescope_elaborates_with_prior_binders_
  in_scope` (`explicit_data_elaboration.rs:113`) -- `CheckedSource`'s 5-arg
  telescope over three auxiliary proof families.
- **AC4 (bad result-target diagnostics).** Search: the four negative shapes
  (wrong head, changed parameter, too few/too many indices, non-family
  result) each rejecting with a named diagnostic before install. Found:
  `bad_constructor_result_targets_are_surface_errors`
  (`explicit_data_elaboration.rs:151`) -- table-driven over exactly those five
  cases (wrong head, changed parameter, too few, too many, non-family),
  asserting `"bad constructor result target"` plus the constructor and family
  names.
- **AC5 (kernel admission remains the positivity authority).** Search: a
  positive family accepted, a negative recursive occurrence rejected through
  the kernel verdict (not a parallel elaborator check). Found:
  `negative_recursive_occurrence_rejects_through_kernel_gate`
  (`explicit_data_elaboration.rs:228`) -- asserts `"kernel rejected"` and
  `"non-strictly-positive occurrence"`, i.e. the kernel's own wording, not a
  surface-invented one. `same_family_occurrence_in_result_index_rejects_
  before_install` (`:192`) is the companion same-family-in-target-index case,
  asserting `ElabError::KernelRejected { error: KernelError::
  PositivityViolation(_), .. }` by pattern match on the error variant, not
  string-matching.
- **AC6 (existing behavior and scope remain stable).** Search: legacy
  `data`/`match` tests still passing, and touched-area confinement. Found:
  `legacy_simple_data_still_elaborates` (`:280`) still exercises the legacy
  path end to end including a `match`. `git show --stat 3e3e8f41` shows the
  touched area is `crates/ken-elaborator/{ast,data,elab,modules,resolve}.rs`
  plus tests and the frame doc -- no `ken-kernel`, `Cargo.lock`, `packages`,
  `spec`, or `conformance` movement.
- **AC7 (durable naming hygiene).** Search: same as `SURF-gadt-parser-ast`'s
  AC6 -- `grep -rniE '\bsurf\b|gadt'` over non-test `src/*.rs` turns up no
  `SURF-gadt-*` identifiers; symbols are domain-named (`Vector`,
  `CheckedSource`, `ConstructorUniverseViolation`).

**One test beyond the stated AC list**,
`same_level_universe_constructor_rejects_before_decoder_can_form`
(`:241`), is a `ConstructorUniverseViolation` regression from a different WP
(`KTR-1`/`KTR-2`, see `git log -- explicit_data_elaboration.rs`); it exercises
this slice's declaration path but discharges no `SURF-gadt-elaboration` AC, so
it is not cited above as a discharge.

## Not this node

No repair, no reframing of the linked frame.
