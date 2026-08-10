# RT-LEXICAL-RECURSOR-CONSUMERS — `D0`/`D1` checkpoint

Node: `docs/program/issues/RT-LEXICAL-RECURSOR-CONSUMERS.md` (rows 1-5 only).
Frame: `docs/program/wp/RT-LEXICAL-RECURSOR-CONSUMERS.md`.

## 0. Identity — the candidate base and the measurement base are the SAME

| coordinate | exact |
|---|---|
| candidate merge-base | `9adeb30f` |
| **base every number below was measured at** | **`9adeb30f`** |

⛔ **Read this before any denominator.** An earlier revision of this record
opened `Base: exact origin/main 8fae87de…` and then called its figures *"at this
base"* — while the candidate had been re-anchored to `9adeb30f`. That is a false
candidate identity: it presents measurements taken at one tree as evidence about
another, and it does so in the one field a reader uses to decide what the
numbers are about. `8fae87de` was the **original** measurement base and is
recorded here as history, not as this candidate's base.

**The resolution is a re-measurement, not an argument.** Every figure in §1 and
§2 was re-taken on `9adeb30f` and reproduces the earlier run exactly —
denominators, distribution, activation counts, the five failing tests, and the
per-test abort pattern. The measured surface also did not move: `core.rs` is
byte-identical across the two bases (sha256 `d75c2f09…a4f3` at both), which is
why the reproduction is expected rather than lucky.

**If a later re-anchor moves this candidate again, this section is what goes
stale first.** Re-run the census rather than carrying these numbers forward: the
instrument is described in §1.2 and takes one suite run per leg.

> # THE PARTITION IS RETURNED. `D2` IS NOT STARTED.
>
> `D1` finds **three distinct authorities across four causal classes**, and one
> row that is not a lowering refusal at all. The frame's §6 hard stop —
> *"return the partition before coding if `D1` finds materially distinct
> authorities rather than downstream symptoms of one root"* — **fires**.
>
> The other two triggers do **not** fire: the population is exactly **16
> compiles**, not materially above it; and no class shares an exact production
> root with [[RT-MATCH-RECURSOR-CONSUMERS]].

## 1. `D0` — the population, closed by measurement

### 1.1 The production predicate, read from production

`recursive_descent_residual` and `collect_recursive_descent_residuals` carry the
**same** predicate, byte-for-byte in shape
(`lowering/core.rs:1389-1400`, `:1544-1555`):

> a `Call` whose callee is a `LexicalClosure`, with **some** argument a
> `ComputationalMatch` **some** of whose cases has a non-empty
> `recursive_positions`.

The population is that predicate's, not any fixture list's.

### 1.2 The instrument, and why at this seam

`select_body_emission_authority` has exactly one call site,
`compile_expr_into_module_with_root_projection` (`core.rs:1888`), and both
public entries delegate to it. A census there sees **every** compilation that
can supply the predicate.

Rows were taken at **function entry** and **at the selector**, keyed by libtest
thread name, one `write_all` of a pre-formatted buffer — the sibling node
measured that `writeln!` issues a syscall per fragment and lets concurrent test
threads interleave mid-record. **Malformed rows in the run below: 0.**

The instrument was temporary and is **not** in this candidate; the tree is
byte-identical to `9adeb30f` (`core.rs` sha256 `d75c2f09…a4f3`). The measurement
is the durable artifact.

### 1.3 Denominators, `ken-runtime --lib`, measured at `9adeb30f`

| quantity | value |
|---|---|
| compilations reaching function entry | **683** |
| compilations reaching the selector | **668** |
| refused before the selector | **15** |
| residual set of all 15 pre-selector refusals | **`none`**, every one |
| distinct tests performing at least one compilation | **287** |
| malformed census records | **0** |

**The 15 invisible compiles are empty of B**, so the population below is not
short by construction. That gap is closed by measurement, not by argument.

### 1.4 The measured distribution at the selector

| enumerated set | compilations |
|---|---|
| `none` | 637 |
| **`{LexicalCallArgumentRecursor}`** | **16** |
| `{MatchScrutineeRecursor}` | 15 |
| both variants together | **0** |

**Both-together is zero**, so no simultaneous exclusion is ever needed — the
frame's claim, re-measured rather than inherited.

### 1.5 The B population — 16 compiles across 10 tests

| test | compiles | production authority |
|---|---|---|
| `control::px8j_owned_scope_deletion_fails_closed_before_another_frame_is_emitted` | **2** | `RecursiveDescent` |
| `control::px8j_all_three_producer_paths_reach_real_consumers` | 1 | `RecursiveDescent` |
| `control::px8j_siblings_share_an_origin_and_nested_ih_gets_a_child_origin` | 1 | `RecursiveDescent` |
| `control::px8j_one_two_three_scope_segments_reach_selection_hole_and_unwind` | 3 | `RecursiveDescent` |
| `control::px8j_selected_scope_partitions_differ_across_the_real_return_hole` | 2 | `RecursiveDescent` |
| `constructors::recursive_computational_aggregate_traverses_ordinary_frame` | 1 | `RecursiveDescent` |
| `control::d8_every_required_join_plan_is_consumed_exactly_once` | 2 | `RecursiveDescent` |
| `effects::recursive_computational_host_result_keeps_established_dynamic_lane` | 1 | `RecursiveDescent` |
| `control::rt_d1_the_exact_position_b_witness_carries_without_a_port` | 2 | 1 `FunctionizedUnits`, 1 `RecursiveDescent` |
| `control::rt_d2_exact_counts_and_the_suppression_ab` | 1 | `FunctionizedUnits` |

### 1.6 The floor was a floor — two ways

The frame's table is **eight expressions across five families**. Measured:

1. **Row 1 is 2 compiles, not 1.** `px8j_owned_scope_deletion…` supplies the
   predicate twice.
2. **Three tests outside rows 1-5 are in the population** — 4 compiles:
   `recursive_computational_aggregate_traverses_ordinary_frame`,
   `d8_every_required_join_plan_is_consumed_exactly_once` (2), and
   `recursive_computational_host_result_keeps_established_dynamic_lane`.

Rows 1-5 are therefore **9** of the 16, not 8. **All four extra compiles stay
green under B-only exclusion** (§2.3), so they widen the *population* without
widening the *repair*.

**16 is not materially above 16.** The sibling's census at `bcf3218b` put B at
16 across 10 tests; `9adeb30f` measures the same figure. The re-size trigger does
**not** fire. (`MatchScrutineeRecursor` moved 8 → 15 over the same interval, but
that is the sibling's population, not this node's.)

### 1.7 Candidate selectors, and what each missed

| selector | what it would have given | what it missed |
|---|---|---|
| `px8j_` name prefix | rows 1-5 | all four non-`px8j` compiles, and the second `owned_scope_deletion` compile |
| `BodyEmissionAuthority::RecursiveDescent` assertions | a superset spanning both variants | cannot distinguish A from B; would have pulled row 6 in |
| grep for `LexicalCallArgumentRecursor` | the two production sites and the exclusion controls | zero fixtures — the predicate is structural, and no fixture names the variant |

**No grep reproduces this population**, which is `AC-1`'s point: a name tells you
what to open, never what a program enumerates.

### 1.8 Bounds on this closure, stated rather than implied

1. **`ken-runtime --lib` only.** The census hooks are `#[cfg(test)]`, so
   `rt_parity_native`, `px8f_buffer_native` and `px8f_write_partition` are not
   covered, and those compile real Ken programs.
2. **The selector has two populations; this census sees one.** Programs that
   compile through `select_body_emission_authority`, and controls that call it
   directly without compiling. Production reaches the selector only from the
   compile path, so the compile-keyed closure is the right one **for a repair** —
   but this is not a claim about everything that consults the selector.

## 2. `D1` — activation and attribution

### 2.1 The seam

B-only exclusion, the existing one-variant hook used as designed, set **only on
compiles that enumerate B**. The branch carries
`debug_assert!(was_present)` — *"the exclusion was set for a variant this program
does not fire"* — so a blanket set would panic on the other 652 compiles rather
than measure them. Every B compile that reached the selector under exclusion
selected **`FunctionizedUnits`**; production, unexcluded, keeps
`RecursiveDescent`. The seam is confirmed at `9adeb30f`.

### 2.2 Activation denominators — the guard against a credited-but-unreached refusal

| test | unexcluded | B-only |
|---|---|---|
| `px8j_owned_scope_deletion…` | 2 | **1** |
| `px8j_all_three_producer_paths…` | 1 | 1 |
| `px8j_siblings_share_an_origin…` | 1 | 1 |
| `px8j_one_two_three_scope_segments…` | 3 | **1** |
| `px8j_selected_scope_partitions…` | 2 | 2 |
| `d8_every_required_join_plan…` | 2 | 2 |
| `recursive_computational_aggregate…` | 1 | 1 |
| `recursive_computational_host_result…` | 1 | 1 |
| `rt_d1_the_exact_position_b_witness…` | 2 | 2 |
| `rt_d2_exact_counts_and_the_suppression_ab` | 1 | 1 |
| **total** | **16** | **13** |

**The three missing compiles are accounted for, not lost.** `owned_scope_deletion`
and `scope_segments` abort on their *first* activated compile, so their later
compiles never run. ⇒ **rows 1 and 4 are attributed on their first compile only**;
their remaining 3 are unmeasured under activation and must not be reported as
passing or failing.

### 2.3 Outcome: 836 passed, 5 failed — exactly rows 1-5

The five reds are exactly the frame's five rows. **Every other B test stays
green on `FunctionizedUnits`** — five tests, seven compiles — and these are the
same-family positive controls `D1` requires:

| positive control | compiles | result under B-only exclusion |
|---|---|---|
| `rt_d1_the_exact_position_b_witness_carries_without_a_port` | 2 | green |
| `rt_d2_exact_counts_and_the_suppression_ab` | 1 | green |
| `d8_every_required_join_plan_is_consumed_exactly_once` | 2 | green |
| `recursive_computational_aggregate_traverses_ordinary_frame` | 1 | green |
| `recursive_computational_host_result_keeps_established_dynamic_lane` | 1 | green |

⇒ The activation is not globally destructive: B programs **do** run on the
functionized lane today. That is what makes the five reds attributable to their
own shapes rather than to the lane change as such.

### 2.4 The causal partition — three authorities, four classes

| class | rows | first refusal | owner | boundary |
|---|---|---|---|---|
| **R1** | 1, 4 | `ComputationalMatch` — *"source scrutinee is not a constructor value"* | `lower_source_machine_with_continuation_inner`, `core.rs:6870` | lowering refusal |
| **R2** | 3 | `Closure` — *"a closure cannot cross the boundary: it is runtime-local and live-domain only, and it has no durable lane"* | `boundary_transfer_admissibility`, `mod.rs:9874` arm `Lowered::Closure \| DeclarationClosure` | lowering refusal |
| **R3** | 5 | `ComputationalMatch` — *"a computational recursor closure names an in-flight activation, not a transferable value"* | `boundary_transfer_admissibility`, `mod.rs:9886` arm `Lowered::ComputationalRecursorClosure` | lowering refusal |
| **R4** | 2 | **no refusal — the compile returns `Ok`.** The trace lacks a `Px8jSourceTraceEvent::Mint` with `siblings > 0` | not a lowering authority; a post-compile trace observation at `control.rs:1448` | post-compile |

**Four rendered strings, five rows, three authorities — and the frame's warning
was right.** Rows 1 and 4 share a reason *and* an owner. Rows 3 and 5 share an
**owner** but sit on **different `Lowered` variant arms**, so they are one
authority and two operand kinds. **Row 2 is not a refusal at all**, and no
count of refusal strings would have shown that.

### 2.5 The finding that most constrains `D2`

**Every one of R1-R4 is a guard `AC-3` forbids weakening**, by name:

| class | `AC-3` guard it lands on |
|---|---|
| R1 | 3 — *an actual non-constructor computational scrutinee still refuses* |
| R2 | 2 — *a closure is never made boundary-transferable* |
| R3 | 1/2 — the protocol-only in-flight activation, never a transferable value |
| R4 | 5 — *a missing recursive-IH authority still refuses* |

⇒ **No `D2` here may be written at the refusal site.** The frame's lawful shape
is the only one available: *make the protocol or fact get consumed or
represented at its owner before the guards.* A repair that taught any of these
four to accept its input would be a guard weakening, and each of the four is
also the only probe the campaign has for that guard.

### 2.6 Hard stops

| trigger | fires? | evidence |
|---|---|---|
| population materially above 16 compiles | **no** | exactly 16, the sibling's figure |
| authorities materially distinct | **YES** | three owners, four classes, one non-refusal boundary |
| shares an exact production root with `RT-MATCH-RECURSOR-CONSUMERS` | **no** — but see the correction below | ruled by the Architect, not inferred here |

**⇒ The partition is returned and `D2` is not begun.** Sizing is the Steward's
call: `M` was a scoping figure from a symptom count, and the symptom count was
five while the causal count is four across three authorities — with R4 in a
different phase from the other three.

## 3. What this checkpoint does not cover

- **No repair, no control, no fixture change.** `crates/` is byte-identical to
  `9adeb30f`; this candidate is this document.
- **Row 6 and the `MatchScrutineeRecursor` population** — not touched.
- **Rows 1 and 4 beyond their first activated compile** — 3 compiles unmeasured
  under activation, stated in §2.2 rather than assumed harmless.
- **The two population bounds in §1.8**, neither of which this node closes.

## 4. CORRECTION — the shared-root verdict, re-derived on measurement

⛔ **§2.6's original wording was wrong, and this section supersedes it.** It read
*"row 6 refuses on `RecursiveBackedge`; no class here does"* — a verdict keyed on
the refusal **string**. A later probe measured the R1 operand itself:

```
kind=RecursiveBackedge  actual_constructor=None  route=DirectScrutinee  frame=None
```

⇒ **R1's refused value IS `RecursiveBackedge`.** The strings differ because the
two sites report different things about the same protocol value, which is
exactly why a string-keyed comparison was the wrong instrument. The original
sentence was false as written.

**The verdict itself survives, but on different grounds, and they are the
Architect's rather than mine** (`evt_k64pjherb9x0`): the two share a **protocol
representation**, not a repair root. `#6c`'s owner is `carried_join_arm`, which
R1 **never traverses**; R1's first missing owner is source-machine
`SourceContinuation::ComputationalMatchScrutinee` on the `DirectScrutinee` route
with no frame. **Same marker, distinct first missing owner and route ⇒ no shared
repair root**, and no subsumption proposal is owed.

**The lesson worth carrying past this node:** a shared-root question is about the
**value and its owner**, and a refusal string is a rendering of neither. I
reported "no shared root" from the strings and had to correct it from the
operand.
