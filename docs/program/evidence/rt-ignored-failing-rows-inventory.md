# RT-IGNORED-FAILING-ROWS-INVENTORY — the ledger

Measured at implementation base `c041de7c3` (`== origin/main` at the time of
the run, and the same base the Steward's census used). Every observed
signature below was READ FROM A RUN on that tree, one process per row, and
never copied from the row's `#[ignore]` label — `AC-2`.

## 1. Population, derived two ways

    28  raw `#[ignore` textual hits under crates/
    23  REAL attributes          (5 prose, separated by a code-only scan)
    -8  registry run-exemptions  (.github/ignored-test-exemptions.toml)
    15  SELECTED

The compiler's own `--ignored --list` independently reports 15 for `ken-cli`,
which matters: `scripts/ci-ignored-sweep.py` states that a source grep is NOT
the authority because it cannot see a macro-leading `#[ignore]`, and nextest
is unavailable on this box. Two instruments with different blind spots
agreeing is what stands in for the one the sweep uses.

**`AC-1`: all 15 selected rows were run, and all 15 FAILED.** The ledger
population is therefore 15, not the sixteen of `§2`. That listing is the
sixteen at `0f71ab5b9`; `px8f_buffer_native.rs` carries no `#[ignore]` at
this base at all, so its row is simply gone — which is what `§2`'s own boxed
note predicted and why it said fifteen or seventeen satisfies the AC.

## 2. The ledger

| identity | ignore label's node | status | sig | label agrees? |
|---|---|---|---|---|
| `ken-runtime c2_ac4_runtime_host_result_selects_a_separately_generated_nested_payload` | `RT-CARRIER-PRODUCER-OCCURRENCE` | ready | S4 | yes |
| `ken-cli::px7l_checked_host_recursive_bind delayed_capturing_generic_bind_agrees_across_real_executors` | `RT-SITEOP-CARRIED-WITNESS` | merged | S1 | yes |
| `ken-cli::px7m_hostresult_computational_match dynamic_err_payload_selects_a_multistep_tree_across_real_executors` | `RT-SITEOP-CARRIED-WITNESS` | merged | S1 | yes |
| `ken-cli::px7m_hostresult_computational_match dynamic_ok_payload_selects_a_multistep_tree_across_real_executors` | `RT-SITEOP-CARRIED-WITNESS` | merged | S1 | yes |
| `ken-cli::rt_escape_second_resource_native escaped_buffer_used_by_fanning_host_op_matches_interpreter` | `RT-SITEOP-CARRIED-WITNESS` | merged | S6 | NO |
| `ken-cli::rt_escape_second_resource_native escaped_resource_used_by_fanning_host_op_matches_interpreter` | `RT-CLOSURE-BOUNDARY-LANE` | merged | S2 | NO |
| `ken-cli::px7f_resource_native linked_public_right_denial_preserves_exact_masks` | `RT-SITEOP-CARRIED-WITNESS` | merged | S3 | NO |
| `ken-cli::px7f_resource_native linked_public_second_release_is_closed_and_the_handle_closes_once` | `RT-SITEOP-CARRIED-WITNESS` | merged | S3 | NO |
| `ken-cli::rt_escape_second_resource_native nat_fanout_escaped_resource_matches_interpreter` | `RT-CLOSURE-BOUNDARY-LANE` | merged | S2 | NO |
| `ken-cli::px7n_nested_computational_eliminator nested_err_payload_reaches_both_real_executors` | `RT-FRAME-MARKER-ONCE` | draft | S2 | NO |
| `ken-cli::px7n_nested_computational_eliminator nested_ok_payload_reaches_both_real_executors` | `RT-FRAME-MARKER-ONCE` | draft | S2 | NO |
| `ken-cli::px8ta_oriented_subcontinuation public_two_three_level_brackets_finish_and_release_lifo` | `RT-CLOSURE-BOUNDARY-LANE` | merged | S5 | NO |
| `ken-cli::rt_escape_second_resource_native r2_cross_buffer_freeze_fails_closed_with_invalid_bounds` | `RT-PROCESS-EXIT-STATUS` | draft | S7 | NO |
| `ken-cli::px7l_checked_host_recursive_bind runtime_selected_non_unit_response_is_consumed_across_real_executors` | `RT-SITEOP-CARRIED-WITNESS` | merged | S1 | yes |
| `ken-cli::rt_span_prov_native sp_a_foreign_span_freeze_rejects_own_span_succeeds_on_both_engines` | `RT-COMPMATCH-TREE-SCRUTINEE` | draft | S8 | NO |

## 3. Observed signatures

**S1** — 4 row(s)

    unsupported runtime-IR lowering: BoundaryCarrier: a carried recursive hypothesis is an eliminated value, not a callable, so it takes no arguments, but the call provides 1

**S2** — 4 row(s)

    Cranelift backend failure: native static transition planner invariant failed; please report this compiler bug: two host response cases claim one operation constructor

**S3** — 2 row(s)

    UnclassifiedRuntimeTrap { terminal_value: -1 }

**S4** — 1 row(s)

    a source aggregate reached the carrier with no planner-issued producer occurrence, so it would name no ownership record and could only be given the authority of wherever it happened to be transferred

**S5** — 1 row(s)

    depth 2 releases must be strict LIFO left: [ResourceTraceIdentityV1(1), ResourceTraceIdentityV1(2)] right: [ResourceTraceIdentityV1(2), ResourceTraceIdentityV1(1)]

**S6** — 1 row(s)

    Cranelift backend failure: native static transition planner invariant failed; please report this compiler bug: source-specific inheritances at one generated entry disagree on their typed consumer projection, including the fresh-result route

**S7** — 1 row(s)

    unsupported runtime-IR lowering: StaticResponseDeferred: a deferred host response is compiler control and can only enter its exact response owner

**S8** — 1 row(s)

    Cranelift backend failure: native static transition planner invariant failed; please report this compiler bug: an exact detached required consumer has no computational occurrence

## 4. `AC-3` — the clustering question, answered

**Fifteen rows exhibit EIGHT distinct observed signatures**, clustered
4 / 4 / 2 / 1 / 1 / 1 / 1 / 1.

**The labels cluster 7 / 3 / 2 / 1 / 1 / 1 across six nodes, and the two
partitions do not align.** Neither is a refinement of the other:

- `RT-SITEOP-CARRIED-WITNESS` names 7 rows that exhibit **three** different
  signatures (S1 x4, S3 x2, S6 x1).
- **S2 spans two different nodes** — 4 rows with one byte-identical planner
  message, two labelled `RT-CLOSURE-BOUNDARY-LANE` and two
  `RT-FRAME-MARKER-ONCE`. The labels disagree with each other exactly where
  the runtime agrees.

**The §3 differential predicate, re-derived at this base.** Ten of the 15
rows assert that native and interpreted execution agree. They do **not**
share one defect: those ten carry S1, S2, S3, S6 and S8 — five distinct
signatures. **The naming-pattern hypothesis is refuted as a defect claim.**
A shared name meant a shared test family, not a shared cause.

**Seven of the 15 labels are byte-identical strings.** That is one diagnosis
copied seven times, not seven measurements — which is how it can be right
about four rows and wrong about three.

## 5. `AC-5` — rows with nowhere to route

**Ten of the 15 cite a node that is already `merged`:**
`RT-SITEOP-CARRIED-WITNESS` (7 rows, MERGED 2026-08-17, *"D2 LANDED"*, PR
#2557) and `RT-CLOSURE-BOUNDARY-LANE` (3 rows, MERGED 2026-08-15, PR #2322).
Those rows name a landed deliverable as their blocker.

**Three cite a `draft` node** (`RT-FRAME-MARKER-ONCE` 2,
`RT-PROCESS-EXIT-STATUS` 1, `RT-COMPMATCH-TREE-SCRUTINEE` 1) — filed, not
dispatchable. **One cites a `ready` node** (`RT-CARRIER-PRODUCER-OCCURRENCE`).

**So exactly one of fifteen rows names a node that is both live and
dispatchable**, and its label is one of the five that the signature confirms.

**Ten of fifteen labels do not describe what the row actually does.** Only
S1's four rows and S4's one match their label's mechanism; the other ten
name a mechanism the run does not exhibit.

## 6. A measured input the sweep cannot reach: the `ken-runtime` lib baseline

    origin/main   ken-cargo test -p ken-runtime --lib
                  1035 passed   0 FAILED   2 ignored

Recorded here on the Steward's instruction because nothing else holds it. **The
ignored-row sweep runs `--run-ignored=only`**, so the ordinary suite is outside
its population by construction, and no CI job publishes this number.

**Why it belongs in THIS ledger rather than a note somewhere.** Whoever sizes
the repair program off the eight signatures will be changing planner and
lowering code that those 1035 tests exercise. **They need to know the baseline
is clean**, because a repair that regresses part of it is otherwise
indistinguishable from one that does not: a red appearing during the repair
could be pre-existing debt, and this says it cannot be.

It was measured by running the suite on `origin/main` at `c041de7c3`, which
took about 30 seconds. The two ignored rows in that count are the registry
exemptions inside `ken-runtime`, not part of the 15.

## 6. What this ledger does NOT establish

- **It does not say the six nodes are wrong about the code.** A merged node
  can have landed its deliverable while a row it was cited on fails for
  another reason. What is measured is that the LABEL does not predict the
  SIGNATURE, not that the node did not do its work.
- **It does not size the repair program.** Eight signatures is an upper
  bound on distinct causes and a lower bound on nothing: two signatures can
  share a root, and S2's four rows spanning two labels is the case most
  likely to collapse. Sizing is the next cut's, from these signatures.
- **No row changed disposition and no production file was touched** —
  `AC-4`. This node reads.
