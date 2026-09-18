# `RT-IGNORED-FAILING-ROWS-INVENTORY` — the ledger

**Implementation base: `origin/main` at
`04d4dd38a9cfb40da9b5d68bfeb42aaddbb76661`.** Every number and every signature
below was measured at that base. This file reads; it repairs nothing and
changes no row's disposition.

## How the population was established

`AC-1` requires the failing set **as measured at the implementation base**, not
§2's listing and not a source grep. Three independent instruments agree on
**15**:

    libtest `--ignored --list` per crate, default features, this box      15
      ken-cli 15 listed - 1 exempt (px8ds)                              = 14
      ken-runtime 2 listed - 1 exempt (b2v_ac10)                        =  1
      ken-elaborator 2 / ken-interp 3 / ken-kernel 1, all 6 exempt      =  0

    CI `ignored-row sweep`, run 35265440149 on this exact base,
    `cargo nextest --workspace --locked --run-ignored=only`, feature UNION
      "Ignored-row sweep completed: 15 selected; 0 passed."             = 15

    that run's per-binary FAIL breakdown                                = 15

The 8-row exemption registry reconciles exactly: `1 + 1 + 6 = 8`, no exemption
unmatched and no listed row unexplained.

**The feature axis is measured, not assumed.** A row gated behind `ken-cli`'s
`dasm-c2-observation` would be invisible to a default-feature `-p` build and
visible to CI's union build. CI ran the union and also returned 15, so that
residual is empty rather than untested.

**Signatures below are from local runs on this box**, cited per row. CI's log
is a second signature source; a per-row local-versus-CI comparison was **not
performed** — no GitHub credential is held at this seat — so where this ledger
says a signature, it means the local run.

### Why sixteen became fifteen

§2's sixteenth row,
`px8f_buffer_native::discharge_ledger_refuses_one_word_discharging_two_obligations`,
**has never existed on `main`.**

    absent from px8f_buffer_native.rs at this base           0 occurrences
    absent from that binary's full test listing              5 tests, none it
    commits introducing it   7f408cbcb, b1575f536            NOT on main
    commit that #[ignore]s it   0d94d58b6                    NOT on main

`RT-DISCHARGE-LEDGER-COLLISION-SOURCE-REACHABILITY` records it as *"`#[ignore]`d
at `0d94d58b6`"*, and that commit is not an ancestor of `origin/main`. The row
lives only on unlanded work. Note that a source grep can neither confirm nor
deny it — the row is macro-generated, so the name appears only under `docs/` at
this base and at the sweep's older tree alike. **Only the harness listing
answers.**

## The ledger

Signature is the observed failure text, read from a run. Label is the current
`#[ignore]` text at this base.

> **THE TABLE BELOW IS A RECORD OF `04d4dd38a` AND IS NOT EDITED WHEN A ROW
> CLOSES.** Rewriting a cell to say "cleared" would destroy the measurement this
> file exists to hold. Closures are appended to **"Rows cleared since this
> base"** after the table, each with its own base. **Read both.** Row 15 has
> closed; every other row's disposition here still stands.

| # | identity | observed signature (measured) | label agrees? | owning node | status | framed? |
|---|---|---|---|---|---|---|
| 1 | `ken-cli::px7f_resource_native linked_public_right_denial_preserves_exact_masks` | `UnclassifiedRuntimeTrap { terminal_value: -1 }` | **NO** — label predicts the BoundaryCarrier arity refusal | `RT-SITEOP-CARRIED-WITNESS` | **merged** | yes |
| 2 | `ken-cli::px7f_resource_native linked_public_second_release_is_closed_and_the_handle_closes_once` | `UnclassifiedRuntimeTrap { terminal_value: -1 }` | **NO** — same | `RT-SITEOP-CARRIED-WITNESS` | **merged** | yes |
| 3 | `ken-cli::px7l_checked_host_recursive_bind delayed_capturing_generic_bind_agrees_across_real_executors` | `BoundaryCarrier: a carried recursive hypothesis is an eliminated value, not a callable, so it takes no arguments, but the call provides 1` | symptom yes; **mechanism refuted** | `RT-CARRIED-RESIDUAL-IH-ARITY` | ready | yes |
| 4 | `ken-cli::px7l_checked_host_recursive_bind runtime_selected_non_unit_response_is_consumed_across_real_executors` | same as 3 | symptom yes; **mechanism refuted** | `RT-CARRIED-RESIDUAL-IH-ARITY` | ready | yes |
| 5 | `ken-cli::px7m_hostresult_computational_match dynamic_ok_payload_selects_a_multistep_tree_across_real_executors` | same as 3 | symptom yes; **mechanism refuted** | `RT-CARRIED-RESIDUAL-IH-ARITY` | ready | yes |
| 6 | `ken-cli::px7m_hostresult_computational_match dynamic_err_payload_selects_a_multistep_tree_across_real_executors` | same as 3 | symptom yes; **mechanism refuted** | `RT-CARRIED-RESIDUAL-IH-ARITY` | ready | yes |
| 7 | `ken-cli::px7n_nested_computational_eliminator nested_ok_payload_reaches_both_real_executors` | `native static transition planner invariant failed: two host response cases claim one operation constructor` | yes | `RT-HOST-RESPONSE-ROUTE-KEY-COLLISION` | ready | yes |
| 8 | `ken-cli::px7n_nested_computational_eliminator nested_err_payload_reaches_both_real_executors` | same as 7 | yes | `RT-HOST-RESPONSE-ROUTE-KEY-COLLISION` | ready | yes |
| 9 | `ken-cli::px8ta_oriented_subcontinuation public_two_three_level_brackets_finish_and_release_lifo` | `assertion left == right failed: depth 2 releases must be strict LIFO` — `[Id(1), Id(2)]` vs `[Id(2), Id(1)]` | **NO** — label predicts a closure-lane refusal; this is an ordering assertion inside one engine | `RT-CLOSURE-BOUNDARY-LANE` | **merged** | **NO** |
| 10 | `ken-cli::rt_escape_second_resource_native escaped_buffer_used_by_fanning_host_op_matches_interpreter` | `source-specific inheritances at one generated entry disagree on their typed consumer projection, including the fresh-result route` | **NO** — label predicts the BoundaryCarrier arity refusal | `RT-SITEOP-CARRIED-WITNESS` | **merged** | yes |
| 11 | `ken-cli::rt_escape_second_resource_native escaped_resource_used_by_fanning_host_op_matches_interpreter` | same as 7 | yes | `RT-HOST-RESPONSE-ROUTE-KEY-COLLISION` | ready | yes |
| 12 | `ken-cli::rt_escape_second_resource_native nat_fanout_escaped_resource_matches_interpreter` | same as 7 | yes | `RT-HOST-RESPONSE-ROUTE-KEY-COLLISION` | ready | yes |
| 13 | `ken-cli::rt_escape_second_resource_native r2_cross_buffer_freeze_fails_closed_with_invalid_bounds` | `StaticResponseDeferred: a deferred host response is compiler control and can only enter its exact response owner` | **NO** — label predicts a Persistent-child/NoReferent ownership failure | `RT-PROCESS-EXIT-STATUS` | **draft** | **NO** |
| 14 | `ken-cli::rt_span_prov_native sp_a_foreign_span_freeze_rejects_own_span_succeeds_on_both_engines` | `an exact detached required consumer has no computational occurrence` | **NO** — label predicts a tree-producing match scrutinee | `RT-COMPMATCH-TREE-SCRUTINEE` | **draft** | **NO** |
| 15 | `ken-runtime cranelift_backend::lowering::core::tests::constructors::c2_ac4_runtime_host_result_selects_a_separately_generated_nested_payload` | `a source aggregate reached the carrier with no planner-issued producer occurrence, so it would name no ownership record` | yes, near-verbatim | `RT-CARRIER-PRODUCER-OCCURRENCE` | ready | yes |

**Label agreement: 9 agree, 6 do not.** Rows 3-6 are counted as agreeing on the
symptom only — the observed text is what the label predicts, while the label's
*mechanism* and its readmission condition were refuted by
`RT-CONTEXT-FRAME-SLOT-HOLDS-ONE-PER-FUNCTION` and are rewritten by
`RT-CONTEXT-FRAME-LABEL-CORRECTION`, landed at
`fb414cb78de6cc82cdf247be85270a2da0e47f3e`. **This column therefore has a known
expiry with a named cause**, and the quotations above remain a correct record of
`04d4dd38a` rather than of `main`.

## Rows cleared since this base — APPEND ONLY

    row 15   ken-runtime ... constructors::c2_ac4_runtime_host_result_selects_
             a_separately_generated_nested_payload
             cleared by  RT-CARRIER-PRODUCER-OCCURRENCE D4
             squash      9c3a5f588d7b58152c1c2c38e6e4f999f056ba8a  (PR #3937)
             POPULATION  15 -> 14

**The first of the fifteen to close.** The `#[ignore]` is gone from
`crates/ken-runtime/src/cranelift_backend/lowering/core/tests/constructors.rs`
— the file now carries none — and the row is live at `:3316`. **It passes on
CI's green at D4's landing gate, not on a local run**; per `COORDINATION §12`
this box does not run the workspace, and no credential here reads CI logs, so
"passes" here means "the gate that merged it was green."

**The row closed by measuring its property again, not by relabelling.** D4
replaced a hardcoded `BoundaryTag::ImmediateBool` expectation with one read
from the plan's ruled allocation lane. That distinction is the whole point of
the exercise and is the thing to check on every subsequent closure.

### The population is MEASURED at 14, not 15 minus one

Measured at `a503a5a971f8d673248317f1ab8023ee924b055c`. Two instruments:

    attribute-only #[ignore] sites in crates/     23 at 04d4dd38a -> 22 now
      the ONLY per-file delta is constructors.rs   1 -> 0
    exemption registry .github/ignored-test-       8 entries, unchanged
      exemptions.toml
                                    22 - 8 = 14, reconciling exactly as
                                    23 - 8 = 15 did at the original base

> ### COUNT THE ATTRIBUTE, NOT THE TEXT. THE WRONG PATTERN RETURNED THE INVERSE.
>
> A first pass counted `#[ignore` as a **substring** across `crates/` and got
> **28 at both bases** — which reads as *"the population did not move."*
>
> **Five of those 28 are prose:** comments that mention `#[ignore]` while
> discussing a row. `4eb3dc4c6` added one such comment to
> `px8ta_oriented_subcontinuation.rs` in the same window, so the text count
> showed that file **gaining** a row it did not gain — exactly cancelling the
> row that did close.
>
> ⇒ **The wrong instrument did not blur the answer. It produced the INVERSE
> one, and the inverse was plausible**: a new fixture landing beside a closure
> is an ordinary thing to see, and "one cleared, one added" is a story that
> explains itself. **Anchor the pattern at line start — `^\s*#\[ignore` — so it
> matches the attribute and not the discussion of it.**
>
> **What caught it was the PER-FILE delta, not the total.** A total can only be
> believed or disbelieved; a delta names a file you can open. **Never take a
> population count as a bare number when the per-item delta costs one more
> flag.**

### What this closure settles about row 15's second refusal

`RT-HOST-RESULT-ARM-SHAPE-DISAGREEMENT`'s frame asserted that row 15 carries
**two independent refusals**, that its own was downstream of the carrier one,
and that the `#[ignore]` must therefore stay after D4 because the row still
could not run. **D4 removed it and the gate was green.** ⇒ Only one of the two
claimed refusals actually blocked this row. The arm-shape question survives on
its own merits — the Architect ruled it NARROWED, and the borrow it names is a
measured coverage gap — but **not as a blocker on this row**, and that node is
`draft` pending its re-cut.

## `AC-3` — the clustering question, MEASURED AT SIGNATURE DEPTH

**Eight distinct failure signatures across fifteen rows.**

    A  BoundaryCarrier arity: eliminated value, not a callable        4   rows 3-6
    B  two host response cases claim one operation constructor        4   rows 7,8,11,12
    C  UnclassifiedRuntimeTrap { terminal_value: -1 }                 2   rows 1,2
    D  source-specific inheritances disagree on typed consumer proj.  1   row 10
    E  StaticResponseDeferred: only its exact response owner          1   row 13
    F  exact detached required consumer has no computational occ.     1   row 14
    G  releases must be strict LIFO (in-engine ordering assertion)    1   row 9
    H  source aggregate reached carrier with no producer occurrence   1   row 15

**The frame's hypothesis is UNCONFIRMED AT SIGNATURE DEPTH.** §3 proposed that
ten differential rows might share one defect, in which case *"the repair
program is two or three nodes rather than sixteen."* **At first-refusal depth
they do not cluster:** the differential rows span **five** distinct signatures
(A, B, D, E, F), and the two largest clusters in the whole set are four rows
each.

**This does not refute a shared upstream cause.** One defect can produce five
different first stops depending on what each row exercises, so per the caveat
below this evidence is *consistent with* §3's hypothesis rather than against
it. **It bounds nothing about the node count in either direction.**
`RT-CONTEXT-FRAME-REFUSAL-DEPTH-CENSUS` is what settles it.

### The differential predicate, re-derived at this base

§3's predicate is *"the row's assertion is that native and interpreted
execution AGREE"*, carried by four name patterns. Applying the **predicate**
rather than the patterns:

    _across_real_executors                                    4
    _reaches_both_real_executors                              2
    _matches_interpreter                                      3
    _succeeds_on_both_engines                                 1
                                                             10  by name
    r2_cross_buffer_freeze_fails_closed_with_invalid_bounds  +1  by BODY
                                                             11  of 15

**`r2_cross_buffer_freeze_fails_closed_with_invalid_bounds` calls
`assert_native_matches_interpreter`** — the same helper as the confirmed
`_matches_interpreter` rows — so it asserts the predicate while its name
advertises none of the four patterns. §3's correction lists it among three rows
*"swept in by their container"* and excludes it. **That exclusion is wrong on
this row**, and it is wrong the same way the original over-count was: both
decided membership from the name. The first version counted the containing
binary; the correction excluded on the name not matching. The predicate lives
in the body.

The other four non-pattern rows were checked the same way and are **not**
differential: rows 1, 2 and 9 name `ken_runtime::EffectObservation` only as a
helper's return type, and row 15 contains no comparison mechanism at all.

⇒ **11 of 15 (73%) are differential**, and being differential does not predict
the first-stop signature.

## `AC-5` — rows with nowhere to route

**Six of fifteen have an owning node that cannot close them as it stands.**

    rows 1, 2, 10   RT-SITEOP-CARRIED-WITNESS     merged
    row 9           RT-CLOSURE-BOUNDARY-LANE      merged, no frame
    row 13          RT-PROCESS-EXIT-STATUS        draft, no frame
    row 14          RT-COMPMATCH-TREE-SCRUTINEE   draft, no frame

**All three `RT-SITEOP-CARRIED-WITNESS` rows also fail `label agrees?`**, and
that is one fact rather than two: the node merged, the rows kept its label, and
what they now do is unrelated to what the label predicts. Rows 1 and 2 trap at
runtime; row 10 refuses on typed consumer projection. **A merged node with a
stale label is the exact state that makes a row look owned and leaves it
unroutable.**

The remaining nine rows have a `ready` owning node with a live frame.

## What this ledger does not establish

- **No per-row local-versus-CI signature comparison.** No credential at this
  seat. Where CI's machine might differ from this box, that difference is
  unmeasured rather than absent.
- **Signature depth is one layer, and it bounds the node count in NEITHER
  direction.** Each row's signature is the first refusal that stops it.
  `RT-CONTEXT-FRAME-REFUSAL-DEPTH-CENSUS` exists because forcing past the first
  refusal on four of these rows reveals a second one behind it, so a shared
  signature here is not evidence of a shared *root cause*, only of a shared
  first stop. **That cuts both ways and only one side is obvious:** two rows
  with *different* first stops may still share one cause upstream of both. So
  **eight signatures is not a floor of eight repairs and not a ceiling on
  shared causes** — two rows behind one signature may have different causes, so
  the true count can exceed eight; five rows behind five signatures may share
  one cause, so it can fall below. This ledger does not say the repair program
  is eight nodes, and must not be read as saying it.
- **`RT-SITEOP-CARRIED-WITNESS` is recorded as merged, not as re-verified.**
  Whether it once covered rows 1, 2 and 10 is not measured here; what is
  measured is that it does not describe their current behaviour.
