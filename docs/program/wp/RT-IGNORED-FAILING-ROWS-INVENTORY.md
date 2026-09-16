# WP frame — `RT-IGNORED-FAILING-ROWS-INVENTORY`

**Owner:** Team Runtime · **Size:** S · **Risk:** low (no production diff) ·
**Tier:** T2 · **Gate:** none · **Deps:** none

**Origin:** operator directive 2026-09-15: *"The other tests should be fixed."*
Fixing sixteen rows is a **program**, not a work package. This WP produces the
ledger that says how many programs it actually is, so the cut is made on
measured failure signatures instead of on a guess.

## 1. Objective

For each of the sixteen `#[ignore]`d rows that the sweep runs and that **fail**,
record its actual failure signature and the node that owns the defect. Produce
a ledger. **Repair nothing.**

## 2. Fixed inputs, measured

Same measurement as `RT-IGNORED-PASSING-ROWS-DISPOSITION` §2: PR #3676 head
`0f71ab5b9267781ae1d91bc654011cad42b926af`, `ignored-row sweep` job id
`104224384914`. 28 selected, 12 passed, **16 failed**.

    ken-cli::px7f_resource_native
      linked_public_right_denial_preserves_exact_masks
      linked_public_second_release_is_closed_and_the_handle_closes_once
    ken-cli::px7l_checked_host_recursive_bind
      delayed_capturing_generic_bind_agrees_across_real_executors
      runtime_selected_non_unit_response_is_consumed_across_real_executors
    ken-cli::px7m_hostresult_computational_match
      dynamic_ok_payload_selects_a_multistep_tree_across_real_executors
      dynamic_err_payload_selects_a_multistep_tree_across_real_executors
    ken-cli::px7n_nested_computational_eliminator
      nested_ok_payload_reaches_both_real_executors
      nested_err_payload_reaches_both_real_executors
    ken-cli::px8ta_oriented_subcontinuation
      public_two_three_level_brackets_finish_and_release_lifo
    ken-cli::px8f_buffer_native
      discharge_ledger_refuses_one_word_discharging_two_obligations   (macro-generated)
    ken-cli::rt_escape_second_resource_native
      escaped_buffer_used_by_fanning_host_op_matches_interpreter
      escaped_resource_used_by_fanning_host_op_matches_interpreter
      nat_fanout_escaped_resource_matches_interpreter
      r2_cross_buffer_freeze_fails_closed_with_invalid_bounds
    ken-cli::rt_span_prov_native
      sp_a_foreign_span_freeze_rejects_own_span_succeeds_on_both_engines
    ken-runtime  cranelift_backend::lowering::core::tests::constructors
      c2_ac4_runtime_host_result_selects_a_separately_generated_nested_payload

## 3. The hypothesis this WP exists to CONFIRM OR KILL — do not assume it

Twelve of the sixteen are **interp/native differential** rows by name: eight in
the `px7*` family named `..._across_real_executors` /
`..._reaches_both_real_executors`, and four in `rt_escape_second_resource_native`
named `..._matches_interpreter`. If those twelve share one defect, the repair
program is two or three nodes rather than sixteen.

**That is a naming-pattern observation and nothing more.** It has not been
checked against a single failure signature. A shared name is consistent with a
shared cause and equally consistent with twelve unrelated defects in one test
family. **The ledger decides it; the frame must not pre-empt it** — and an
explanation that fits is exactly what stops the census being run.

## 4. Deliverables

A ledger, one row per failing test, checked in under `docs/program/evidence/`:

| field | content |
|---|---|
| identity | `package::binary test_name`, as nextest prints it |
| ignore reason | the current `#[ignore = "..."]` text verbatim |
| observed signature | the actual assertion text, panic, trap message, or signal from the sweep run — **not** the ignore label |
| label agrees? | does the observed signature match what the label claims |
| owning node | the node id, with its current `status` |
| framed? | is there a live frame that would close this row |

## 5. Acceptance

**AC-1. Every one of the sixteen has a row. The ledger's population is the
sweep's failing set, not a source grep.** Control: ledger row count is 16, and
each identity appears in the sweep's selected listing.

**AC-2. The observed signature is READ FROM A RUN, never copied from the ignore
label.** Nine of the *passing* rows already carry labels asserting failures that
no longer happen, so a label is not evidence about current behaviour. Cite the
job or local run each signature came from.

**AC-3. The clustering question is ANSWERED, in either direction, from the
signatures.** State how many distinct failure signatures the sixteen exhibit
and which rows share each. "They look related" does not satisfy this; identical
or demonstrably common signatures do, and so does finding they are all
different.

**AC-4. No production diff, and no row changes disposition.** This WP reads.
A diff touching `crates/**/src/` fails it.

**AC-5. Rows whose owning node is absent, `merged`, or does not in fact cover
the observed signature are called out explicitly** — those are the ones with
nowhere to route, and they are the reason this ledger exists.

## 6. Why an inventory rather than starting the repairs

§4c: the constraint is real and it is measured. `RT-SITEOP-CARRIED-WITNESS` is
`merged` while four *passing* rows still cite its `D2`, and the labels in this
family cite base `21fd46dce`, 2338 commits behind `origin/main`. **The labels
in this test population are known to be stale**, so sequencing repair work off
them would cut nodes against defects that may not exist. One cheap read of
sixteen signatures replaces that guess.

## 7. Contention

Read-only over `crates/ken-cli/tests/` and one `ken-runtime` test module; writes
only a new evidence file. No contention with any open candidate.
