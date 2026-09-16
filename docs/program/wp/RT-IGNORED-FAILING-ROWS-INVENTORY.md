# WP frame — `RT-IGNORED-FAILING-ROWS-INVENTORY`

**Owner:** Team Runtime · **Size:** S · **Risk:** low (no production diff) ·
**Tier:** T2 · **Gate:** none · **Deps:** none

**Origin:** operator directive 2026-09-15: *"The other tests should be fixed."*
Fixing the whole failing set is a **program**, not a work package. This WP
produces the
ledger that says how many programs it actually is, so the cut is made on
measured failure signatures instead of on a guess.

## 1. Objective

For each `#[ignore]`d row that the sweep runs and that **fails at the
implementation base**, record its actual failure signature and the node that
owns the defect. Produce a ledger. **Repair nothing.** Every count in this
frame keyed to "the sixteen" means that failing set, not the listing in §2 —
which is the sixteen as measured at `0f71ab5b9`.

## 2. Fixed inputs, measured

Same measurement as `RT-IGNORED-PASSING-ROWS-DISPOSITION` §2: PR #3676 head
`0f71ab5b9267781ae1d91bc654011cad42b926af`, `ignored-row sweep` job id
`104224384914`. 28 selected, 12 passed, **16 failed**.

> **THE `12 / 16` SPLIT IS NOT A FIXED INPUT — RE-DERIVE IT AT THE
> IMPLEMENTATION BASE.** See the boxed note in
> `RT-IGNORED-PASSING-ROWS-DISPOSITION` §2 for the full argument and the
> verification. In short: the population (`34`, `-6`, `28`) carries to `main`,
> but the split is a property of the tree the sweep ran on, and that tree is 36
> commits ahead of `origin/main` and **53 behind** it. **`px8f_buffer_native.rs`
> is the one named test file that differs between the two trees, and it holds
> one of the sixteen rows listed below.** The listing is therefore the sixteen
> **at `0f71ab5b9`**; treat it as the starting hypothesis for the ledger's
> population, never as the population itself.

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

**TEN of the sixteen are interp/native differential rows**, under a predicate
stated first and counted second: **the row's assertion is that native and
interpreted execution AGREE.** Four name patterns carry it, and the count
follows from them rather than the other way round:

    _across_real_executors          4
    _reaches_both_real_executors    2
    _matches_interpreter            3
    _succeeds_on_both_engines       1
                                   10  of 16  (62%)

If those ten share one defect, the repair program is two or three nodes rather
than sixteen.

> **CORRECTED 2026-09-16 — the first version said TWELVE (eight `px7*`, four
> `rt_escape_second_resource_native`), and both sub-counts were wrong in one
> way.** Against this frame's own listing: six `px7*` rows match those two
> patterns, not eight, and three `rt_escape_second_resource_native` rows end
> `_matches_interpreter`, not four.
>
> **EACH SUB-COUNT WAS THE ROW COUNT OF THE CONTAINING BINARY, NOT THE COUNT
> MATCHING THE PREDICATE.** Eight is *every* `px7*` row; four is *every*
> `rt_escape_second_resource_native` row. `linked_public_right_denial_...`,
> `linked_public_second_release_...` and
> `r2_cross_buffer_freeze_fails_closed_with_invalid_bounds` are the three that
> were swept in by their container. **I counted the box and reported it as the
> contents.**
>
> `sp_a_foreign_span_freeze_rejects_own_span_succeeds_on_both_engines` is the
> tenth: the two originally-stated patterns exclude it and the **meaning**
> includes it, which is why the predicate is now stated in words and the
> patterns are listed as its carriers. **The sizing argument moves with the
> count** — twelve of sixteen is 75%, ten is 62%, nine under the narrow
> patterns — so `AC-3` gets the predicate rather than the number, and the
> implementer re-derives both at the implementation base.

**That is a naming-pattern observation and nothing more.** It has not been
checked against a single failure signature. A shared name is consistent with a
shared cause and equally consistent with ten unrelated defects in one test
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

**AC-1. Every row in the FAILING SET AS MEASURED AT THE IMPLEMENTATION BASE has
a ledger row. The population is that sweep's failing set, not a source grep and
not the listing in §2.** Control: ledger row count equals that sweep's failing
count, and each identity appears in its selected listing. **Sixteen is what
`0f71ab5b9` gave; an implementer on `main` who measures fifteen or seventeen
satisfies this AC with that number.** A count pinned from another tree is not
satisfiable honestly, and forcing it is likelier than failing it.

**AC-2. The observed signature is READ FROM A RUN, never copied from the ignore
label.** Nine of the *passing* rows already carry labels asserting failures that
no longer happen, so a label is not evidence about current behaviour. Cite the
job or local run each signature came from.

**AC-3. The clustering question is ANSWERED, in either direction, from the
signatures.** State how many distinct failure signatures the failing set
exhibits and which rows share each — **and re-derive the differential count
from §3's predicate at the implementation base rather than carrying `10`.**
"They look related" does not satisfy this; identical
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
family cite base `21fd46dce`, **2364 commits behind `origin/main` as measured at
`507bd4bd1`** — an anchored figure, because it drifts by construction and an
unanchored one invites a reader who re-measures to doubt the frame. **The labels
in this test population are known to be stale**, so sequencing repair work off
them would cut nodes against defects that may not exist. One cheap read of
the failing set's signatures replaces that guess.

## 7. Contention

Read-only over `crates/ken-cli/tests/` and one `ken-runtime` test module; writes
only a new evidence file. No contention with any open candidate.
