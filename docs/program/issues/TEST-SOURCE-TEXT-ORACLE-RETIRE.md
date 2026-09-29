---
id: TEST-SOURCE-TEXT-ORACLE-RETIRE
title: "Retire the remaining source-text test oracles: 21 test functions in 16 files read Rust source with include_str! and assert on its text, so behaviour-neutral edits redden them; the Architect disposes each function first, then each is dropped, replaced by a behavioural control, or retired to review"
status: ready
owner: verify
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-09-29 ('concur with rec. 1 WP owned by verify in L2'), on the Architect's carry evt_4zwm5v35gnk7j after RT-BACKEND-SOURCE-CENSUS-RETIRE. The rule is the operator's 2026-07-26 prohibited-subject ruling. Steward-filed per COORDINATION section 2."
---

# Retire the remaining source-text test oracles

## Objective

No test in the workspace fails because a Rust source file was edited,
renamed or re-commented without a change in behaviour. Each property these
tests guarded stays protected by a behavioural control or is owned by
Architect review.

## Fixed inputs (read at `22093cfcd`)

- **Rule.** Operator 2026-07-26: "Test oracles that assert facts about source
  code, catalog, or documentation lines are an invitation for failure and
  delay. Tests should focus on behavior."
  (`agent/playbooks/build/qa-test-design.md`).
- **Population.** `grep -rnE 'include_str!\("[^"]*\.rs"\)' crates` finds 32
  uses in 21 test functions across 16 files:

  | Crate | Test functions |
  |---|---|
  | ken-runtime | `boundary_value_clif.rs` `b2v_every_class_guard_call_site_takes_its_set_from_the_plan`; `lowering/boundary.rs` `b2v_ac3_the_lowered_boundary_disposition_has_no_wildcard_arm`; `lowering/core/tests/control.rs` `LOWERING_IMPL_SOURCES` and its consumer; `lowering/core/tests/host_call_carrier.rs` `every_generated_root_and_unit_signature_is_two_pointers_to_one_word`; `lowering/core/tests/mod.rs` `every_source_term_carrier_holds_an_occurrence_and_never_a_bare_expression`, `retained_closures_carry_a_static_origin_and_no_body_term`; `static_transition/closure.rs` `b2r_ac6_the_abi_plane_declares_no_emission_construct`, `b2r_ac7_the_abi_plane_adds_no_parser_and_no_dependency_edge`; `static_transition/occurrences.rs` `the_semantic_seed_api_accepts_only_occurrence_origins` |
  | ken-elaborator | `tests/b2_acceptance.rs` `no_modal_construct_in_kernel`, `inert_to_conversion`, `obligation_not_dischargeable_in_ken`; `tests/decimal_char_acceptance.rs` `char_deceq_pin1_structural_encoding`; `r_layer_tests/ds1_empty_dec_acceptance.rs` `ac3_trusted_base_delta_is_ordinary_inductive_admission_only` |
  | ken-host | `lib.rs` `producer_inventory_is_bidirectional_and_sync_drift_is_discriminating`, `public_surface_contains_only_ken_owned_semantic_types`; `effect_v1.rs` `resource_owner_and_close_allowance_are_structurally_confined` |
  | ken-cli | `tests/px4b_native_production.rs` `linked_console_broken_pipe_reaches_ken_instead_of_signal_termination`, `naked_process_ir_helpers_are_not_public_production_api`; `tests/console_exec.rs` `closed_stdout_is_an_io_failure_not_sigpipe_termination` |
  | ken-interp | `tests/f2f3_acceptance.rs` `f3_legacy_add_sub_mul_unregistered_in_elaborator` |
  | ken-verify | `filesystem.rs` `canonical_snapshot_projects_mode_but_never_owner_namespace` |

- **Stale guard comments.** Four comments name retired census tests as
  live guards (Adversary `evt_7m4dbphvp53vh`): `semantic_ir.rs:2182-2186`,
  `closure.rs:1456-1459` and `:6464-6466`, `host_call_carrier.rs:681-682`.
  A corrected draft is kept unmerged at
  `archive/RT-FRAME-MARKER-ONCE-comments-eab5` (`eab5a45c1`).
- **Precedent.** `RT-BACKEND-SOURCE-CENSUS-RETIRE` (landed `43015b699`)
  disposed five such consumers this way.

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## Deliverable

Every listed function is dropped, replaced or retired per its AC-0
disposition, and no `include_str!` of a `.rs` file remains as a test
oracle. The four stale comments are corrected. Increments are per crate,
ken-runtime first.

## Acceptance

- **AC-0 (disposition; no edit).** For each function, name the property it
  guards and propose one disposition:
  1. drop the text assertion, because the test already runs the behaviour
     (for example, `console_exec.rs` checks `main.rs` text before a real
     broken-pipe run);
  2. replace it with a named behavioural control;
  3. retire it to Architect review;
  4. keep the read, because it is input data rather than an oracle.
  The Architect rules before any increment is built. A property that
  touches the kernel or `trusted_base()` is ruled individually.
- **AC-1.** A behaviour-neutral edit to each file a retired oracle read,
  such as an added comment line on a scratch branch, leaves every test in
  the owning crate green.
- **AC-2 (controls).** Each replacement control reddens under a mutation
  that breaks the property it now carries, and each mutation is restored.
  If a replacement stays green under its mutation, stop: it does not carry
  the property.

## Stop conditions

- Any production, kernel, spec or `trusted_base()` change (an operator
  question).
- A replacement needs production code: stop to the Architect.
- A disposition 4 (input data) claim that the Architect does not accept.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`, or the parked
  `wp/RT-NATIVE-TREE-MATCH-RUNTIME-SCRUTINEE`.
