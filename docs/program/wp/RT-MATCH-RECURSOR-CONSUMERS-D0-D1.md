# RT-MATCH-RECURSOR-CONSUMERS — `D0`/`D1` checkpoint

> **SUPERSEDED ON ITS OPERATIVE QUESTION — read section 6 first.** This section
> records the measurement at base `89aa1550`, where the population refused. Five
> successor authorities have merged since, and **at base `166641c8` every row
> the `ken-runtime --lib` census reaches compiles.** The enumeration and causal
> work below stand as the record of that base; the answer to *"does it
> refuse?"* has flipped **over the censused domain**, and section 6 is the
> current one.
>
> **Sections 1 and 6 do not close the node's population.** `AC-1` is
> unqualified and both of their censuses are `ken-runtime --lib` only — see
> 1.6.1 and 6.7. **Section 7 is the cross-crate census that closes the domain
> those two left open**, and it is the current statement of `AC-1`'s coverage.

**Base: `89aa15502d6f76e6d42aac1b97ea3ff5032cd889`** (`origin/main`, the merged
corrected `RT-RECURSOR-TRANSPORT` `D0`-`D2`). Branch
`wp/RT-MATCH-RECURSOR-CONSUMERS`.

**This candidate changes no code.** `git diff 89aa1550 -- crates/` is empty at
the checkpoint SHA. Every instrument below was temporary, was run, and was
reverted; nothing in `crates/` differs from the base by a single byte. That
discharges `AC-5` (no new `#[ignore]`) and `AC-6` (both residual variants, both
classifier and collector insertions, and the per-variant exclusion hook present
and unchanged) trivially rather than by inspection.

> # THE HEADLINE, BECAUSE IT IS THE THING THIS NODE EXISTS TO ESTABLISH
>
> **`d8d` is not the perimeter.** The `MatchScrutineeRecursor` population
> contains a second, previously unnamed member:
> `px8j_all_three_producer_paths_reach_real_consumers`.
>
> **And the two are one root, not two.** Their refusal backtraces are identical
> frame for frame and line for line. Neither frame hard stop fires.

## 1. `D0` — the `ken-runtime --lib` population, by measurement

> **Scoped 2026-08-09.** This heading read *"the population, closed by
> measurement"*. It is a `ken-runtime --lib` census (see 1.6.1) and closes no
> unqualified `AC-1`; a heading is what a reader quotes, so the claim is
> corrected here rather than only in the bounds section below.

### 1.1 The instrument, and why it is at this seam

The production predicate has exactly **one** consumer:
`select_body_emission_authority` is called from exactly one site,
`compile_expr_into_module_with_root_projection` (`lowering/core.rs:1276`), and
both public entries (`compile_expr_into_module`,
`compile_expr_into_object_module`) delegate to it. So a census placed at that
function sees **every** compilation that can supply the predicate, and no other
path can route a program to `RecursiveDescent` behind its back.

The census recorded, for **every** compilation, the complete
`enumerate_recursive_descent_residuals` set and the lane actually selected,
keyed by the libtest thread name (which is the test's full path). **Every
compile was recorded, not only the firing ones**, so the population has a real
denominator rather than a filtered numerator.

Two instrument defects were found and fixed before the numbers below were
trusted, both by measurement rather than by review:

| defect | how it surfaced | fix |
|---|---|---|
| `writeln!` issues one syscall per format fragment, so concurrent test threads interleaved mid-record | 2 malformed rows in the first run | one `write_all` of a pre-formatted buffer |
| a compile refused by `validate_oriented_subcontinuation_transport` returns **before** the selector, so the census could not see it | suspected, then measured | a second census at function **entry**; entry minus at-selector is exactly the invisible set |

### 1.2 Denominators

| quantity | value |
|---|---|
| compilations reaching function entry | **632** |
| compilations reaching the selector | **617** |
| compilations refused before the selector | **15**, across 7 tests |
| residual set of all 15 pre-selector refusals | **`none`**, every one |
| distinct tests performing at least one compilation | **271** |
| malformed census records, final run | **0** |

**The 15 invisible compiles matter and they are empty.** Had any of them
enumerated `MatchScrutineeRecursor`, the population below would have been short
by construction and no control over it could have said so. They do not, so the
gap is closed by measurement, not by argument.

### 1.3 The measured distribution at the selector

| enumerated set | compilations |
|---|---|
| `none` | 593 |
| `{LexicalCallArgumentRecursor}` | 16 |
| `{MatchScrutineeRecursor}` | **8** |
| both variants together | **0** |

### 1.4 The `MatchScrutineeRecursor` population, as censused (`AC-1` NOT discharged)

Eight compilations across five tests. Every one enumerates exactly
`{MatchScrutineeRecursor}`; none also carries `LexicalCallArgumentRecursor`.

| test | compiles | complete residual set | production lane, unhooked |
|---|---:|---|---|
| `d8d_the_composed_binding_site_is_live_and_neither_landed_population_installs_a_target` | 1 | `{MatchScrutineeRecursor}` | `RecursiveDescent` |
| `px8j_all_three_producer_paths_reach_real_consumers` | 1 | `{MatchScrutineeRecursor}` | `RecursiveDescent` |
| `rt_d2_exact_counts_and_the_suppression_ab` | 3 | `{MatchScrutineeRecursor}` | `RecursiveDescent` (1 of 3 unhooked) |
| `rt_d2_the_exact_position_a_witness_executes_on_both_lanes_and_agrees` | 2 | `{MatchScrutineeRecursor}` | `RecursiveDescent` (1 of 2 unhooked) |
| `rt_d2_trace_shows_the_marker_propagated_and_never_reaching_the_composed_consumer` | 1 | `{MatchScrutineeRecursor}` | via the committed hook |

**Read the last column carefully.** Several A-firing compilations select
`FunctionizedUnits` in an ordinary suite run. That is **not** production
behaviour — those tests arm the committed one-variant hook themselves. The
production, unhooked answer for every A-firing program is `RecursiveDescent`.
Crediting those hooked rows to production would be the inverse of the frame's
"do not reinterpret a retained `RecursiveDescent` run as activation," and it is
the reason the lane was recorded per compilation rather than per test.

**Three of the five are `RT-RECURSOR-TRANSPORT` `D2`'s own controls.** The
population that exists independently of `D2`'s witness is `d8d` and
`px8j_all_three_producer_paths_reach_real_consumers`.

### 1.5 Candidate selectors used, and what each missed

The frame required stating which selectors were candidates. A grep was used to
generate candidates; **none of them closed the population**, and the table
records the gap rather than implying agreement.

| candidate axis | hits | relation to the measured population |
|---|---:|---|
| CamelCase variant name `MatchScrutineeRecursor` | 23 | over-generates: matches the classifier, collector, enum and hook, none of which are programs |
| snake_case fixture spelling `match_scrutinee_recursor` | 13 | **misses `d8d` and `px8j_all_three` entirely** — neither names the class |
| the lane, `BodyEmissionAuthority::RecursiveDescent` | 17 | over-generates and still misses both |
| the enumerator / observation API | 29 | over-generates |
| the exclusion hook `set_selector_variant_exclusion` | 3 | finds only `D2`'s own controls |

**The two members that matter are invisible to every one of the five axes.**
`d8d` and `px8j_all_three` reach the class through
`px8j_deferred_recursive_field_fixture`, a helper named after neither the class
nor the lane. A grep list is a candidate set; only the enumeration is closure.

### 1.6 The bounds on this closure, stated rather than implied

1. **The census covers the `ken-runtime` lib suite.** The instrument is
   `#[cfg(test)]`, so the lib compiled as a dependency of another crate carries
   no census. `crates/ken-cli/tests/rt_parity_native.rs`,
   `px8f_buffer_native.rs` and `ken-verify`'s `px8f_write_partition.rs` are
   therefore **not** covered here.
2. **Every member of the measured population is a hand-built `RuntimeExpr`.**
   Campaign Trap 1 names this exactly: such a witness proves the classifier
   **sees** the class, and says nothing about whether any real Ken program
   exhibits it. Nothing in this checkpoint claims otherwise.
3. **The four `#[ignore]`d tests never run**, so the census cannot see them.
   Two were run under `--ignored` and compile with `none`. One
   (`b2v_ac10_...`) performs no backend compilation. The fourth,
   `c2_ac4_runtime_host_result_selects_a_separately_generated_nested_payload`,
   never reaches a compilation at all, so it was classified **by inspection
   against the exact predicate**: its `Match` scrutinees are `Var(0)`, never a
   `ComputationalMatch`, and its one `Call` has an empty argument list, so
   neither residual predicate can fire. It is not in the population.
4. **A compile-keyed census cannot see a test that calls the selector
   directly.** This is not hypothetical — see 2.4.

## 2. `D1` — activation and attribution

### 2.1 The seam

A-only exclusion, via the existing one-variant hook, used as designed: enumerate
the full set, remove exactly `MatchScrutineeRecursor`, let the remainder decide.
Both variants were never excluded together and the hook was not generalized.

### 2.2 Retained run, and the regression baseline

**`D0`'s Trap-2 obligation — the target suite on the base with no delta applied:
816 passed, 0 failed, 4 ignored.** That is the regression baseline; every row
green in it is a row `AC-1b` must keep green.

The ordinary retained run stays green at that same 816 with the census attached,
so the instrument is not itself perturbing the suite.

### 2.3 Under A-only exclusion: 810 passed, 6 failed

**Only two of the six are members of this node's repair population.** The other
four fail because the probe was applied suite-wide, which the committed hook
never is. Attributing all six would have overstated the node by a factor of
three.

| row | first refusal / assertion | class |
|---|---|---|
| `d8d_the_composed_binding_site_is_live_...` (`control.rs:18257`) | `Unsupported(UnsupportedLowering { construct: "RecursiveBackedge", reason: "protocol machinery is never a source value at a boundary" })` | **repair population** |
| `px8j_all_three_producer_paths_reach_real_consumers` (`control.rs:1482`) | **identical refusal, verbatim** | **repair population** |
| `d5_c3_a_second_residual_retains_recursive_descent` (`control.rs:12939`) | expected `RecursiveDescent`, got `FunctionizedUnits` | probe artifact |
| `retained_authority_residual_is_the_typed_selector_accounting` (`control.rs:5290`) | expected `RecursiveDescent`, got `FunctionizedUnits` | probe artifact |
| `the_body_authority_selector_narrows_only_completed_ports_and_stays_fail_closed` (`control.rs:5171`) | expected `RecursiveDescent`, got `FunctionizedUnits` | probe artifact |
| `rt_d2_exact_counts_and_the_suppression_ab` (`control.rs:11371`) | `(1,1,1)` where `(1,0,0)` expected | probe artifact |

**The four artifacts were classified by reading the code, not by their names.**
`rt_d2_exact_counts_and_the_suppression_ab:11370` runs the witness through
`rt_run` — the deliberately **unhooked** leg whose whole point is to contrast
with the hooked one. A suite-wide exclusion activates that leg too and destroys
the contrast, which is exactly the `(1,0,0)` to `(1,1,1)` shift observed. All
four pass unhooked on this tree.

### 2.4 The blind spot this exposed, which is worth more than the artifacts

`d5_c3_a_second_residual_retains_recursive_descent`,
`retained_authority_residual_is_the_typed_selector_accounting` and
`the_body_authority_selector_narrows_only_completed_ports_and_stays_fail_closed`
**never appear in the census at all**, yet they reacted to the probe.

They call `select_body_emission_authority` **directly**, without compiling. A
census placed at the compile entry is structurally incapable of seeing them.

⇒ **The selector has two populations, not one:** programs that compile through
it, and controls that call it directly. The production predicate's population is
the first — production reaches the selector only from the compile path, which is
why the closure in section 1 is the right one for a repair. But any future claim
about "everything that consults the selector" needs both, and a compile-keyed
instrument answers only half.

### 2.5 Activation denominators — the guard against a credited-but-unreached refusal

Recorded per compilation, at selection time, before lowering runs:

| row | lane under A-only exclusion | outcome |
|---|---|---|
| `d8d_...` | **`FunctionizedUnits`** | refused in lowering |
| `px8j_all_three_...` | **`FunctionizedUnits`** | refused in lowering |

Both refusals are therefore credited to a path the selector **actually reached**
and routed onto the functionized lane. Neither is an unvisited seat, and neither
is a retained `RecursiveDescent` run misread as activation.

### 2.6 Positive control

`rt_d2_the_exact_position_a_witness_executes_on_both_lanes_and_agrees` is in the
**same family** — it is in the A population, both its compilations reach
`FunctionizedUnits` under A-only exclusion — and it **passes**. So A-only
exclusion plus the functionized lane is not broken for A programs generally; the
two refusals discriminate a specific shape rather than the lane.

### 2.7 Cross-check against the sanctioned seam

The suite-wide probe is not the committed hook, so the refusal was re-measured
with the **committed** `set_selector_variant_exclusion` armed around `d8d`'s own
expression — no reconstruction, the same expression object, the mechanism the
frame names:

```
Err(Unsupported(UnsupportedLowering {
  construct: "RecursiveBackedge",
  reason: "protocol machinery is never a source value at a boundary" }))
```

Identical. The refusal is the seam's answer, not an artifact of the probe.

### 2.8 The causal chain, to the first mis-consumed static fact

Both rows produce the **same backtrace, frame for frame and line for line**:

```mermaid
graph TD
  A[compile_expr_into_module_with_root_projection core.rs:1540] --> B[define_unit_body units.rs:3640]
  B --> C[lower_carried_computational_match core.rs:12160]
  C --> D[lower_source_carried_match core.rs:7147]
  D --> E[lower_source_carried_leaf core.rs:6730]
  E --> F[lower_forked_branch core.rs:6380]
  F --> G[lower_source_machine_with_continuation_inner core.rs:5163]
  G --> H[carried_join_arm core.rs:10813]
  H --> I[transfer_into_carrier mod.rs:4985]
  I --> J[emit_carrier_transfer mod.rs:7242 REFUSES]
```

> **Every line number above is stated against this candidate's base,
> `89aa1550`, and was re-derived there by name.** The backtrace itself was
> captured with temporary instrumentation in the worktree, which displaced
> `core.rs` by a uniform **+78** lines and the refusal arm in `mod.rs` by **+7**.
> An earlier revision of this section carried those displaced numbers —
> `carried_join_arm` at `10842` rather than `10764` — which resolve to real but
> **wrong** lines in the tree a reader actually has. The attribution never
> depended on them; **the function names are the load-bearing handle** and they
> are unchanged.

**This is not `D2`'s seat.** `RT-RECURSOR-TRANSPORT` `D2` propagates the marker
at `resume_active_continuation`, and that function is nowhere on this path. The
`D2` mechanism is sound and is doing its job; it simply does not own this
crossing. That is precisely the completeness defect the node was filed for.

**The first mis-consumed static fact.** `carried_join_arm` (`core.rs:10764`)
classifies a join arm into three cases:

- `Carried(word)` — pass through;
- `Specialized(Lowered::Trap(_))` — **refused, explicitly and honestly**,
  because a compile-time trap arm returns instead of reaching the merge, so the
  merge would have fewer predecessors than the case chain has arms, and that
  control-flow shape "this route does not build yet";
- `Specialized(anything else)` — transfer into the carrier as a **value**.

`Lowered::RecursiveBackedge` falls into the third case. But a backedge arm has
**exactly the property the second case exists to recognise**: the tail-recursive
edge has already been emitted as a CFG jump, the block is predecessor-free, and
the arm contributes no value and no predecessor to the merge.

⇒ The mis-consumed fact is **the arm's already-left-the-block status**.
`carried_join_arm` keys on the operand's representation (`Carried` versus
`Specialized`) when the property that decides whether an arm can be a join
predecessor is *whether control already departed*. `Trap` encodes that property
and is consulted; `RecursiveBackedge` has it and is not.

**Owner: `carried_join_arm`, `lowering/core.rs:10764`**, the carried-match join
arm consumer — with the protocol's producer being the source-machine fork
(`lower_forked_branch` / `lower_source_carried_leaf`) that yielded a backedge
branch. The guard at `emit_carrier_transfer` is **correct and stays**: it is
refusing a genuine category error, and teaching it to accept the marker is the
banned shape.

### 2.9 Sizing signal for `D2`, not a `D2` decision

The `Trap` arm's own comment states the reduced-predecessor merge "is a control
flow shape this route does not build yet." A backedge arm needs that same
missing shape. ⇒ The lawful repair — represent or consume the protocol at its
owner, before the guard — may require building the reduced-predecessor merge
rather than only re-routing an operand. **This is flagged, not decided**; `D2`
is out of scope for this checkpoint.

## 3. Hard stops — neither fires

1. **Materially distinct authorities?** No. Two rows, **one** root: identical
   refusal text, identical owner, identical backtrace frame for frame. This is
   one authority with two call sites, not two symptoms mistaken for one.
2. **Does any repair need a new planner or ABI population?** Not on the evidence
   here. The mis-consumed fact is a lowering-local control-flow property of a
   join arm. No new planner record and no ABI payload is implicated by the
   trace. `D2` must re-confirm this before coding, since a repair not yet
   written cannot be fully priced.

The sibling-specific third stop does not fire either: nothing here proposes
folding with [[RT-LEXICAL-RECURSOR-CONSUMERS]], and no shared root is claimed.

## 4. Observation routed to the sibling node, not acted on

The same census measured the `LexicalCallArgumentRecursor` population at
**16 compilations across 10 tests**. The campaign record and
[[RT-LEXICAL-RECURSOR-CONSUMERS]]'s frame describe that population as *"eight
expressions across five test families."*

**That earlier figure was measured on the six named hard-stop fixtures under
B-only exclusion, and this one is measured over the whole lib suite**, so they
are not the same quantity and neither refutes the other. But the wider number is
the one a sizing decision should see. **Recorded and routed; rows 1-5 were not
touched**, per this node's banned scope.

Two tests compile programs in **both** populations
(`px8j_all_three_producer_paths_reach_real_consumers`,
`rt_d2_exact_counts_and_the_suppression_ab`). **No single compilation enumerates
both variants**, so the two repair populations remain disjoint at the unit that
matters, but a test-keyed reading of either population will double-count.

## 5. What this checkpoint does not cover

- **No `D2` work.** No repair code, no fixture reshaping, no change to the
  `RecursiveBackedge` guard, and no `#[ignore]` — `crates/` is byte-identical
  to the base.
- **Rows 1-5 and the `LexicalCallArgumentRecursor` population were not
  touched.**
- **`10369776252861e8b15e613576256a3682c70066` was not resumed, cherry-picked
  or consulted** as anything but held evidence.
- **Nothing outside the `ken-runtime` lib suite was censused** — see 1.6.1. The
  native parity suites in `ken-cli` and `ken-verify` compile real Ken programs
  and are the population most relevant to Trap 1's caveat; measuring them needs
  an instrument that survives outside `#[cfg(test)]`, which this checkpoint did
  not build.
- **CI has not run.** `AC-8` is a CI claim and no local `--workspace` run was
  performed, per `COORDINATION §12`.

# 6. THE RESUMPTION — a `ken-runtime --lib` `D0`/`D1` PARTIAL census at base `166641c8`

**Record only. No production change.** `git diff 166641c8 -- crates/` is empty
at this checkpoint SHA; every instrument below was disposable and is reverted.
No `D2` code was written. Suite: `-p ken-runtime --lib`, **838 passed, 0 failed,
5 ignored** (the 5 are the base's).

> ## THE RESULT, STATED AT THE SCOPE IT WAS MEASURED AT
>
> **This is a `ken-runtime --lib` `D0`/`D1` PARTIAL census with zero measured
> red roots.** Every program the census reaches reaches `FunctionizedUnits`
> under A-only exclusion and compiles `Ok`, so **no `D2` repair is owed on any
> root measured here.**
>
> Section 2.3 measured 6 failures at base `89aa1550`. The five merged successor
> authorities closed them over this domain.
>
> ### WHAT THIS IS NOT
>
> **It does NOT discharge `AC-1`, and it does not close the node's
> population.** `AC-1` is unqualified — the frame requires *every compilation
> entry that can supply the production predicate* — while both censuses are
> `#[cfg(test)]` in `ken-runtime` and therefore reach only that crate's own
> unit tests. `ken-cli`, `ken-verify` and elaborator-driven entries are
> **uncensused** (6.7). **A closed subset cannot discharge an unqualified AC**,
> and narrowing the AC to fit the subset is not this record's to do.
>
> An earlier revision of this section and of commit `f9539c00`'s message said
> this discharged `AC-1` "for the in-crate domain" and was "the event `AC-1` was
> written to close on." **Both claims are withdrawn and are corrected here
> rather than annotated.** Whether the residual is measured here or framed
> elsewhere is an Architect/Steward scoping ruling, not a build call.

## 6.1 The gate, re-derived rather than inherited

Section 1.1's closure argument is re-measured at this base, because `D2`-`D6`
moved this file underneath it and a line number does not survive that.

- `select_body_emission_authority` has **exactly one production call site**:
  `lowering/core.rs:1487`, inside
  `compile_expr_into_module_with_root_projection`. (Section 1.1 recorded
  `:1276`; the seam is unchanged, the line moved.)
- That function's only production callers are `compile_expr_into_module` and
  `compile_expr_into_object_module`.
- **Bypass check:** the sole production construction of a `BodyEmissionAuthority`
  *value* is that same line; every other construction is in a `#[cfg(test)]`
  support file building a `Lowering` directly.

## 6.2 Three censuses, and what each one is evidence about

Each row records thread name (libtest names the thread after the test), pid,
exclusion state, and the **complete** residual set.

| census | placed at | classifications | `MatchScrutineeRecursor` hits |
|---|---|---|---|
| compile-site | `core.rs:1484`, beside the selector | 665 | 15 |
| selector-site | `select_body_emission_authority` entry | 731 | 50 |

**The two censuses answer two different questions, and section 2.4 already said
so.** The compile-site census answers *which programs compile through the
selector*; the selector-site census answers *which programs the corpus
classifies at all*, which is the wider set because controls call the selector
directly without compiling. Three tests do exactly that, and the compile-keyed
census cannot see them.

**An earlier revision of this subsection said the compile-keyed census
"under-reports by a whole root" and was "measurably short". That overstates it
and contradicted section 2.4, so it is withdrawn.** `AC-1`'s population is
*compilation entries* — section 2.4's reading, that production reaches the
selector only from the compile path, is correct and stands. R3 is a selector
control, not a compilation entry, so its absence from the compile census is not
an under-report of `AC-1`'s population.

**Running the selector census was still worth it, for a different reason:** it
is how R3 was found at all, and finding it is what let 6.6 establish by
measurement that it has no well-formed compilable form rather than leaving it
an unexamined name. An instrument that answers a wider question than the AC
asks is not thereby measuring the AC.

**All 50 hits carry the complete set `{MatchScrutineeRecursor}`.** No program in
the corpus fires it alongside another variant, so the one-variant exclusion is
valid on every row and always leaves an empty remainder.

### 6.2a The pre-selector gap — MEASURED HERE, not inherited

Section 1.1's second instrument defect still binds: `validate_oriented_-`
`subcontinuation_transport` returns `?` **before** the selector, so a compile it
refuses is invisible to both censuses above. Section 1.2 measured that set at
base `89aa1550` and found it empty of this class. **That is a premise about a
different base, so it was re-measured here rather than carried.**

A third census at function **entry**, above the validator:

| quantity | value |
|---|---|
| compilations reaching entry | **680** |
| compilations reaching the selector | **665** |
| refused before the selector | **15** |
| residual set of all 15 | **`{}`**, every one |
| `MatchScrutineeRecursor` hits, entry | **15** |
| `MatchScrutineeRecursor` hits, at-selector | **15** |

**The two hit counts are equal, so no member of this population is refused
before the selector**, and the enumeration is not short by construction. The
conclusion agrees with section 1.2; the evidence is this base's.

The other instrument defect section 1.1 records — `writeln!` issuing one syscall
per fragment, interleaving concurrent test threads mid-record — was avoided by
construction: every row is one `write_all` of a pre-formatted buffer. Zero
malformed records across all three censuses.

## 6.3 The censused population, by root

| root | carrier | tests | compiled? |
|---|---|---|---|
| **R1** `rt_match_scrutinee_recursor_executable` over `rt_closed_active_recursor` | ordinary `Match` | 3 | yes |
| **R2** ordinary `Match` over `px8j_deferred_recursive_field_fixture` | 6 wrapper spellings | 6 | yes |
| **R3** `d1_match_scrutinee_recursor_witness` over `d1_active_recursor` | bare, `Let`-wrapped, declaration body | 3 | **never** |

This is wider than section 1.4's two members. R2 is that population. R1 is the
`RT-RECURSOR-TRANSPORT` `D2` witness, in the population by the predicate and
omitted there because it was already closed; it is enumerated here because the
population is defined by the predicate, not by what is still open.

**R2's six wrappers are one program — measured, not assumed.** All six build the
identical `Match` over the same scrutinee, same two constructors, `binders: 1`,
same `EXIT_SUCCESS` body, differing **only** in the default trap's message and
the object symbol. All six were compiled separately, with identical results.

## 6.4 `D1` — activation and outcome, over the censused domain

**The activation denominator is measured, not derived:**
`select_body_emission_authority` was called directly under the exclusion and its
answer recorded per row, so no row is credited with an outcome the selector
never routed.

| row | complete residuals | authority, plain | authority, A-only | A-only outcome | plain outcome |
|---|---|---|---|---|---|
| R1 `rt_match_scrutinee_recursor_executable` | `{MatchScrutineeRecursor}` | `RecursiveDescent` | `FunctionizedUnits` | **`Ok`** | `Ok` |
| R2a `D8d composed site` | `{MatchScrutineeRecursor}` | `RecursiveDescent` | `FunctionizedUnits` | **`Ok`** | `Ok` |
| R2b `CED D1 AC-7 witness` | `{MatchScrutineeRecursor}` | `RecursiveDescent` | `FunctionizedUnits` | **`Ok`** | `Ok` |
| R2c `CCR D3 witness` | `{MatchScrutineeRecursor}` | `RecursiveDescent` | `FunctionizedUnits` | **`Ok`** | `Ok` |
| R2d `COC D3 witness` | `{MatchScrutineeRecursor}` | `RecursiveDescent` | `FunctionizedUnits` | **`Ok`** | `Ok` |
| R2e `SAR D3 witness` | `{MatchScrutineeRecursor}` | `RecursiveDescent` | `FunctionizedUnits` | **`Ok`** | `Ok` |
| R2f `direct deferred HostResult default` | `{MatchScrutineeRecursor}` | `RecursiveDescent` | `FunctionizedUnits` | **`Ok`** | `Ok` |

**The frame's fixed input is superseded by measurement, as the frame instructed.**
At `8efdfdb3` the `d8d` row refused with `Unsupported(RecursiveBackedge,
"protocol machinery is never a source value at a boundary")`. At this base it
compiles. Nothing here reverts any successor.

## 6.5 The positive control — why the all-`Ok` table is not vacuous

**A negative check passes for any reason.** Re-arming each landed wall's own
suppression, on the **exact `d8d` witness**, through the **same harness**,
restores a distinct refusal:

| wall re-armed | refusal restored |
|---|---|
| `set_ccr_d2_suppress_active_route` | `Unsupported(BoundaryCarrier, "a carried scrutinee reached a continuation frame that resumes a compile-time value rather than eliminating one")` |
| `set_coc_d2_suppress_continuation` | `Unsupported(BoundaryCarrier, "a carried producer-call scrutinee reached an ordinary eliminator with further composed eliminators behind it; the carried elimination consumes exactly one frame, so the remainder would be silently dropped")` |
| `set_sar_d2_suppress_route` | `Unsupported(ComputationalMatch, "scrutinee is not a constructor value after ordinary expression lowering")` |

Three suppressions, three **distinct** constructs and reasons: three mechanisms
proven live and consumed on this witness, not one probe run three times.

## 6.6 R3, and a control of mine that was wrong

R3 is classified by three tests and compiled by none. Ruling it out as "not a
compilation entry" alone would be a judgement, so it was measured.

**My first control was wrong and is recorded rather than dropped.**
`d1_active_recursor` scrutinises `Var(0)`, **free at the root** — the corpus says
so in the doc comment on its closed counterpart: *"perfectly good for asking the
classifier a question and cannot be compiled or run."* I "closed" it with a `Let`
binding index 0 to a `Bool`, got `Unsupported(ComputationalMatch, "scrutinee is
not a constructor value after ordinary expression lowering")`, and paired it with
a control that removed the recursive position — which **varied case-less-ness
while holding the ill-typed binding fixed**. Wrong axis, and the refusal briefly
looked like an eighth wall.

Holding the shape fixed and varying only the binding's well-typedness:

| binding | outcome |
|---|---|
| `Bool` (ill-typed) | `Unsupported(ComputationalMatch, "scrutinee is not a constructor value after ordinary expression lowering")` |
| `ctor:fixture::d1::Node` (well-typed) | `Unsupported(Match, "scrutinee is not a constructor value")` |

Correcting the binding **moved** the refusal to the outer `Match` rather than
removing it: the fixture's case body is `Var(0)`, which under
`argument_binders: 1` yields the bound argument, so the outer `Match` receives a
closure. Both refusals are attributable to my construction.

⇒ **R3 has no well-formed compilable form via either route tried**, and the
corpus already supplies the closed counterpart of this exact shape —
`rt_closed_active_recursor`, which **is** R1 and compiles. Stated as *"no witness
via the two bindings tried"*, never as a property of R3.

## 6.7 The domain still not covered

**Unchanged from section 1.6.1 and the closing bullets, and re-stated because it
still binds.** Both censuses are `#[cfg(test)]` in `ken-runtime`, so they see
only that crate's own unit tests. The native parity suites in `ken-cli` and
`ken-verify`, and elaborator-driven programs, are **not** in this measurement.

One member of that domain is measured and is **outside** this population. The
merged `RT-SPECIALIZED-MATCH-ATTRIBUTION` `D0` record (`f8250c5a`) measured the
`nc14` refusal's scrutinee occurrence as `RuntimeExpr::Var(0)` at
`StaticOriginId(52)`. The predicate requires the scrutinee to be **syntactically**
a `ComputationalMatch`; a `Var` cannot match it. That family is owned by
`RT-TERMINAL-ALL-ELIM-AUTHORITY` — framed, explicitly **not released**, off this
lane.

**What would close it:** re-gate the census to
`#[cfg(any(test, feature = "px8-ds-test-support"))]` and run those suites with
the feature. I have not made that run and am not claiming closure over it.

## 6.8 Hard stops, and what is owed next

**No hard stop fires on the censused domain.** `D1` found no materially distinct
authorities because it found **no red roots there**; no repair needs a new
planner or ABI population because no repair is needed on any measured root; no
shared root with `RT-LEXICAL-RECURSOR-CONSUMERS` arises, since nothing measured
here is left to attribute. The fourth stop (reduced-predecessor merge shape) is
not reached for the same reason. **A hard stop could still fire on the
uncensused domain; nothing here is evidence either way about it.**

**The open question is a scoping call, not a mechanism one, and it is not
mine.** `D2` and `D3` as framed presuppose a repair. With zero red roots *over
the measured domain* there is no root to repair and no
mutation-at-a-repaired-root for `D3` to key on — but that is an argument about
what was measured, not a licence to declare the node complete. **The residual
in 6.7 has to be measured or reassigned by a ruling before any completion
claim is available to anyone.**

`D3`'s standing obligation to give the `D2` counters a consumer (or state at the
declaration site that they are unread) is **not** discharged by this and
survives independently.

## 6.9 Bans held

`RecursiveBackedge` untouched and still protocol-only. No `RecursiveDescent`
fallback, no `BoundaryUse`, no `PlannedEffectSeat` widening, no lowering-minted
token, no invocation-local state in ABI data. Zero new `#[ignore]`. No tracker
`status:` change. No resume of `10369776`. Both residual variants, both
classifier insertions, both collector insertions and the per-variant hook
present and unchanged — all discharged by the empty `crates/` diff rather than
by inspection.

# 7. `AC-1` CLOSURE — the cross-crate census, at base `1729ef34`

**Record only. No production change.** `git diff origin/main -- crates/` is
**empty** at this checkpoint SHA: every instrument below was disposable and is
reverted. No `D2` code was written, no activation seam was widened, no fixture
was reshaped.

Branch `wp/RT-MATCH-RECURSOR-CONSUMERS`, cut at exactly
`origin/main` `1729ef34`, so no rebase was required.

> ## THE RESULT
>
> **Over the entire cross-crate domain — 47 compilation entries across
> `ken-cli`, `ken-verify`, `ken-interp` and `ken-elaborator`, including their
> `#[ignore]`d population and their child `ken native-build` processes — the
> `MatchScrutineeRecursor` residual fires ZERO times.** Every entry carries the
> empty residual set and reaches production's own `FunctionizedUnits`.
>
> **This is the domain `AC-1` named as remaining** (frame `AC-1`: *"the
> cross-crate `px8-ds-test-support` census authorized in section 4a —
> in-process entries under 4a items 1-5, and child-process native-compile
> entries under 4a.1"*). Frame section 4a's outcome routing states that no
> additional `MatchScrutineeRecursor` row closes `AC-1`'s population coverage;
> none was found, so nothing routes through the `D1`/hard-stop path.

## 7.1 The committed 4a/4a.1 controls, run first

Run before any measurement, because a census whose instrument is unproven
measures the instrument.

| control | result |
|---|---|
| 4a in-process census + its three controls | **pass** — 1 entry, 1 selector arrival, 0 pre-selector returns, 0 firing, authority `FunctionizedUnits` |
| 4a.1 CONTROL 1, the positive child entry | **fires** — a real child `ken native-build` recorded 1 row (`thread=main`, admitted, reached selector) |
| 4a.1 CONTROL 2, observation absent vs present | **pass** — identical exit, stdout, stderr; unobserved child left no artifact |
| 4a.1 CONTROL 3, two concurrent children | **pass** — two sessions, two envelopes, union without collision or loss |
| 4a.1 CONTROL 4, feature-on with no session | **pass** — inert, no artifact |
| 4a.1 CONTROL 5, a non-entry command | **pass** — `ken check` wrote an envelope with **zero rows**, classified rather than omitted |
| 4a.1 feature gate at the artifact | **pass** — each of the four transport strings 0 with the feature off and exactly 1 with it on, read from two physically distinct binaries |

**CONTROL 1 is the load-bearing one and it is why every zero below is
readable.** Frame 4a.1 states that whether the launched `ken` binary carries
`px8-ds-test-support` is a property of Cargo's unit graph that no frame
asserts, so a zero-row child is otherwise ambiguous between *"correctly a
non-entry"* and *"the instrument never existed in the child."* It recorded a
real row, so that ambiguity is resolved by measurement rather than argument.

## 7.2 The instrument, and the one defect it has

Section 6.7 named what would close this domain: *"re-gate the census to
`#[cfg(any(test, feature = "px8-ds-test-support"))]` and run those suites with
the feature."* That is what was built — **disposable, and reverted**.

The committed recorder is a thread-local scope, so it observes only
compilations a harness explicitly wraps. The disposable variant changes exactly
one thing: **the sink**. `KEN_MRC_CENSUS_DIR` selects a per-process append
file, and when it is set the recorder installs itself on first use. It is the
**same production walk** (`enumerate_recursive_descent_residuals`), the **same
row schema**, and the same two terminal points — the pre-selector return and
the selector arrival. No second enumerator and no sampling rule. Child
processes inherit the variable, so child native-build entries land in the same
directory as in-process ones.

Rows are one `write_all` of a pre-formatted buffer, which is section 1.1's own
fix for `writeln!` interleaving concurrent threads mid-record. The sink is
keyed by pid **and a process-start stamp**, because pids are reused inside a
long run and an append-mode collision would merge two processes' rows.

**The defect, stated rather than discovered later.** `(run, thread, ordinal)`
is **not** a unique key under this variant. `with_match_recursor_census`
replaces the thread-local row buffer, so ordinals restart at zero inside a
nested committed scope; `mrc_4a_cross_crate_census_and_its_controls` therefore
produced two rows both keyed `(196489, …, 0)`. Those are two genuinely distinct
compilations — the outside build and the inside build — and because the sink is
append-only they appear as **two rows rather than one merged row**, so the
population count is unaffected. The committed instrument's key uniqueness holds
within a scope; it is this disposable harness that does not extend it across a
nested one, and no claim here rests on that key.

## 7.3 The known-answer control — why the zero is a measurement

**A negative check passes for any reason.** Every cross-crate row carries an
empty residual set, which is equally consistent with a correct census and with
a residual column that can never be non-empty. So the instrument was run
against a population whose answer is already recorded: `ken-runtime --lib`,
measured independently in sections 1 and 6.

| quantity | section 6.2a, base `166641c8` | this instrument, base `1729ef34` |
|---|---:|---:|
| entries reaching function entry | 680 | **680** |
| reaching the selector | 665 | **665** |
| refused before the selector | 15 | **15** |
| residual set of those 15 | `{}` every one | **`{}` every one** |
| `MatchScrutineeRecursor` at the compile site | 15 | **15** |
| `LexicalCallArgumentRecursor` | 16 (section 1.3) | **16** |

Every cell agrees with a measurement this record did not produce. The
`MatchScrutineeRecursor` authority split is **11 `FunctionizedUnits` / 4
`RecursiveDescent`**, which is section 1.4's point restated: several A-firing
compilations select `FunctionizedUnits` because those tests arm the committed
one-variant hook themselves, and production's unhooked answer is
`RecursiveDescent`.

⇒ **The instrument sees this class where the class exists.** The cross-crate
zero is therefore a property of the programs, not of the instrument.

Suite outcome: **838 passed, 0 failed, 5 ignored** — the base's own figure.

## 7.4 The censused domain, by crate

Non-ignored population, each crate run targeted (`-p <crate> --tests`), never
`--workspace`:

| crate | test targets | passed / failed | entries | firing |
|---|---:|---|---:|---:|
| `ken-cli` | 33 | 124 / 0 | **25** | **0** |
| `ken-verify` | 2 | 14 / 0 | 0 | 0 |
| `ken-interp` | 24 | 182 / 0 | **4** | **0** |
| `ken-elaborator` | 137 | 1106 / 0 | **6** | **0** |
| total | **196** | **1426 / 0** | **35** | **0** |

**`ken-verify`'s zero here is NOT a result, and was not read as one.** Its only
integration target holds a single test and that test is `#[ignore]`d, so the 14
passing tests are lib unit tests that compile nothing. A zero from a suite
whose compiling tests never ran means *not measured*, which is the same
ambiguity CONTROL 1 exists to defeat. It is settled in 7.5.

## 7.5 The `#[ignore]`d population, censused rather than assumed

Section 1.6.3 hit this and closed it by running `--ignored`; that measurement
was at a different base, so it was re-taken here rather than carried.

| crate | ignored targets run | passed / failed | entries | firing |
|---|---:|---|---:|---:|
| `ken-verify` | 1 | 0 / 9 | **9** | **0** |
| `ken-interp` | 15 | 1 / 2 | 0 | 0 |
| `ken-cli` | 16 | 0 / 3 | **3** | **0** |
| total | | | **12** | **0** |

**Their failures are their own and are declared at the `#[ignore]` site**, each
naming a different campaign node — `RT-CARRIER-BYTESPAN-OBSERVE` `D5` for the
nine `ken-verify` scenario rows and the three `ken-cli` fs rows, L-classes and
div-op registration for the two `ken-interp` rows. The census is unaffected by
them: an entry row is recorded **before** the transport validator, so a
compilation that later fails still has a census, which is exactly why 4a.1
writes the envelope before the CLI converts its result to an exit.

**This is what settles `ken-verify`.** Its compiling population is nine
entries, every one empty-residual and `FunctionizedUnits` — a measured
non-member, not an unmeasured suite.

## 7.6 The combined classification

| quantity | value |
|---|---|
| cross-crate compilation entries | **47** |
| selector arrivals | **47** |
| pre-selector returns | **0** |
| distinct residual sets observed | **1** — the empty set, on every row |
| `MatchScrutineeRecursor` rows | **0** |
| `LexicalCallArgumentRecursor` rows | **0** |
| authority, production's own and unmodified | `FunctionizedUnits`, all 47 |

The equation `entry = selector-arrival ⊎ pre-selector-return` holds with an
empty second cell: nothing in this domain is refused by
`validate_oriented_subcontinuation_transport` before the selector, so the
enumeration is not short by construction.

**Activation denominators.** No row fired, so no refusal is credited to any
path — there is no first refusal to report, and none of the 47 is a retained
`RecursiveDescent` run misread as activation. Activation stayed crate-local
through the existing `#[cfg(test)]` one-variant hook, which was neither
widened nor reached from any of these entries.

## 7.7 Candidate selectors, and what each missed

`AC-1`'s control requires stating which selectors were candidates. A grep was
used to *generate* the domain, and — as in section 1.5 — **none of them closed
it**; the enumeration did.

| candidate axis | hits | relation to the measured domain |
|---|---:|---|
| cross-crate callers of the runtime compile API | 5 files | finds `px4b_native_production`, three `ken-interp` tests, and `ken-elaborator/src/erasure.rs` — **misses every entry reached through `build_native_program`**, which is most of them |
| `build_native_program` / `compile_native_program_sources` | 20 files | over-generates: names files, not compilations, and says nothing about what any one enumerates |
| `CARGO_BIN_EXE_ken`, the child launchers | 12 files | over-generates badly — `ken run`, `ken check` and `ken fmt` launch the same binary and are ruled non-entries |
| crates declaring a `ken-runtime` dependency | 4 | correct as a **crate** boundary, and that is why the runs are whole-crate rather than per-file |

**The grep axes were used only to bound which crates to run.** Every entry in
7.4 and 7.5 is a measurement of a compilation that happened, with its complete
residual set recorded at the seam the selector consumes.

## 7.8 What this closes, and what it does not

**Closes:** `AC-1`'s remaining domain. The in-crate partial (sections 1 and 6,
merged at `28edeb00` and after) plus this cross-crate census together range
over every compilation entry the repository's harnesses reach, in-process and
child-process, ignored and not. **The `MatchScrutineeRecursor` population
contains no member outside `ken-runtime`'s own unit tests.**

**Does not close, and is not claimed:**

- **A Ken program outside the repository's test corpus is not measured.** This
  is campaign Trap 1's standing caveat and it is unchanged: every member of the
  in-crate population is a hand-built `RuntimeExpr`, and the cross-crate result
  says those shapes do not arise from real elaborated programs in this corpus —
  not that no such program could exist.
- **`D8` is unbuilt.** Frame section 4a.2 added it 2026-08-09 and only the
  frame text is on `main`; no pin asserts the three manifest facts that keep the
  transport out of the shipped binary. The artifact-level gate control in 7.1
  measures the premise **today**; it is not the committed pin `D8` asks for.
- **`AC-2` through `AC-8` are untouched by this record.** `AC-2` and `AC-3`
  presuppose a repaired root, and this census produced none.

## 7.9 Bans held

`RecursiveBackedge` untouched and still protocol-only. No generalized
activation hook and no simultaneous exclusion — the disposable recorder cannot
remove a residual, set an exclusion, choose an authority, or alter a
planner/ABI value, and the 4a parity control is what holds that to account. No
`RecursiveDescent` fallback, no `BoundaryUse`, no `PlannedEffectSeat` widening,
no lowering-minted token, no invocation-local state in ABI data. **Zero new
`#[ignore]`**, and no existing one retired or reinterpreted. No tracker
`status:` change. No resume of `10369776`. Both residual variants, both
classifier insertions, both collector insertions and the per-variant hook
present and unchanged — all discharged by the **empty `crates/` diff** rather
than by inspection.

**No control here keys on the pair `("Match", "scrutinee is not a constructor
value")`**, which the `#[cfg(test)]` mutation hook at `lowering/core.rs` can
fabricate by construction. Nothing in this record keys on a refusal string at
all; the rows key on the owner, the residual set and the authority.

**`AC-8` is a CI claim** and no local `--workspace` run was performed, per
`COORDINATION §12`.
