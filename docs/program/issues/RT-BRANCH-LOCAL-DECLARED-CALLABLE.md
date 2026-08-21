---
id: RT-BRANCH-LOCAL-DECLARED-CALLABLE
title: "recursive_position_unit_body returns one Option<StaticOriginId> for the whole source, so whole-source agreement is too coarse for a Match whose arms differ -- the cut is constructor-and-recursive-position-specific callable authority installed inside the already-selected constructor case, which eliminates the closure crossing rather than opening a durable closure lane"
status: active
owner: runtime
size: L
gate: none
depends_on: [RT-RECURSIVE-POSITION-ARM-ARITY]
blocks: [NATIVE-HANDLE-CARRIER, PX8-F-CAP-41]
github: null
origin: "Architect ruling evt_7aeb7hqrykgpz, Decision dec_7aajmm0eac45c, resolved 2026-08-18. Cut by the Steward on that ruling's explicit instruction to frame the branch-local design capability separately from the rejected D1 AC-3 recut. Surfaced by RT-RECURSIVE-POSITION-ARM-ARITY D1, whose repair moved the governed rows onto the BoundaryCarrier refusal. Steward-filed per COORDINATION section 2."
---

> # THE BINARY I ROUTED WAS FALSE. Read this before reasoning from the guard.
>
> I asked the Architect whether a function-valued recursive field was out of
> scope by design, because `reject_carried_residual_arguments` fires on CAP-41
> and its doc says the durable closure lane is withheld. **The ruling is that
> both halves are true and they do not conflict:**
>
> **The durable closure lane REMAINS EXCLUDED. A function-valued recursive
> field is NOT out of scope.** There is already a separate lawful route, and
> the gap is elsewhere.

# THE BOUNDARIES THAT STAY. None of these is what this node changes.

- A raw `LoweringOperand::Carried` is a **transferred value, never callable
  authority**.
- **`reject_carried_residual_arguments` remains the fail-closed guard** for
  non-empty invocation through that raw-value arm, before control installation.
- **Not authorized, and none of it is a fallback if this cut gets hard:** no
  `PersistentClosure` lane, no new carrier tag or class admission, no
  `FrozenClosure`, no implicit `StaticCallableRef` conversion, and no metadata
  recovered from the carried word.

# THE LAWFUL ROUTE THAT ALREADY EXISTS

In `lower_recursor_residual_call`, the `recursive_unit_body` /
`FunctionizedUnits` arm **runs before `reject_carried_residual_arguments`**. It
lowers explicit source arguments and calls
`call_declared_recursive_position_unit`, and `call_declared_context` can append
planner-authorized capture operands.

⇒ **Static code identity and capture authority stay compiler-owned; the carried
word contributes only the eliminated value. No `Closure` value crosses.**

# THE ACTUAL GAP

`recursive_position_unit_body` returns **one `Option<StaticOriginId>` for the
whole source**. [[RT-RECURSIVE-POSITION-ARM-ARITY]] `D1` was right to refuse to
select a surviving unit when an arm lacks the recursive position — **but
`Ret`/`Vis` proves whole-source agreement is too coarse.** `Ret` has no
recursive position; `Vis.k` does.

**The cut is constructor-and-recursive-position-specific callable authority,
installed only inside the already-selected constructor case.** It must name the
declared body plus its checked explicit-input/capture plan, from retained
source and planner authority.

- **`Ret` installs none.**
- **`Vis` may install one only when the body and captures are lawfully
  expressible as declared call inputs.** If the captures cannot be supplied
  through planner-owned operands or an already-admitted structural-value route,
  **that case still refuses. The guard is not weakened.**

> ### WHY THIS IS `ready` WHILE ITS DEPENDENCY IS STILL `active`
>
> `gen-progress.sh` warns on this deliberately, so here is the answer rather
> than an ambiguity. **`D0` is classification only and touches no production
> line**, so it does not need the arity node's `D1` recut to land first. The
> `depends_on` edge is real and governs the **implementation** deliverable,
> which contends on the same function and must sequence after that recut.
>
> **Releasing `D0` early is safe; releasing an implementation deliverable early
> is not.** Do not read this block as licence for the latter.

# `D0` — TWO NAMED SETS. Freeze the PREDICATE, never the roster.

**Architect ruling `evt_4cvagpx6enpp8` / `dec_15546q1w6pd8s`, which corrects
this node's first framing and the Steward's proposed repair of it.** I grouped
16 rows across 7 test files by their shared refusal text and called that the
population. **A shared refusal text proves only a shared terminal guard.** I
then offered three replacement framings and **all three were rejected**: crediting
[[RT-SITEOP-CARRIED-WITNESS]] `D2` is **provenance, not design membership**; the
text census plus a bounded non-claim is honest about its roster but **cannot let
`D0` say "the population decides the shape" when `D0` never defined the shape's
predicate**; and "more than one constructor reachable" is **too coarse**, since
arms may produce the **same** constructor with different bodies or capture plans,
and conversely all arms may already agree and need no new route.

⇒ `D0` carries **two named sets, and they are not interchangeable.**

## SET 1 — the SEMANTIC MECHANISM POPULATION. This is the frozen predicate.

An occurrence is a `D0` subject when **all four** hold:

1. a carried computational recursive position is **invoked with nonzero source
   arguments**;
2. its source has **multiple reachable producer outcomes**;
3. whole-source `recursive_position_unit_body` authority is **absent,
   ambiguous, or otherwise too coarse**; and
4. invocation authority must therefore be assessed **after partitioning those
   outcomes by the compiler-owned key `(selected constructor identity,
   recursive position)`**.

**For each selected bucket, inspect every reachable producer outcome.**
Agreement is over a **complete declared-call descriptor**, not merely a matching
`StaticOriginId`:

- body origin,
- checked ABI / signature,
- ordered capture/input plan,
- and the **invocation-coordinate / context authority required at the exact
  consumer**.

**The mechanism-owned subset** is a bucket with **one agreed complete
descriptor** and a **lawful non-`Closure` input route**. A missing or disagreeing
descriptor, an unavailable capture or coordinate, or a durable-export boundary is
**still classified by `D0`** and then dispositioned as a refusal or a hard stop.

> **A different terminal error string does NOT exclude an occurrence that
> satisfies this predicate.** That is the whole point of separating the two sets,
> and it is the hole that this amendment closes: the old `AC-1` defined the
> population by the error string, so a row reaching this mechanism and refusing
> at a **different** guard was invisible to the census while `D0` reported
> complete coverage.

**This predicate is the structural closure the later implementation must enforce
at the authority-minting seam.** Ken programs are unbounded, so **no finite
test-string census can prove exhaustive program membership** — which is why the
predicate, not the roster, is the durable artifact.

## SET 2 — the BOUNDED WITNESS SNAPSHOT. The 16 rows live here, and only here.

**The exact refusal text is a legitimate DISCOVERY SEED. It is not the population
definition.** Record:

- the **exact base SHA**,
- the **exact query**,
- and the resulting stable **`(path, test name)` identities**.

**Require set equality** between that identity set and `D0`'s table. **The count
is informational only** — one removed row plus one added row preserves 16 and
defeats a count-based control. A fresh witness is **a delta to classify under the
same predicate**, not a new population.

Files carrying the text at framing time: `px7f_resource_native`,
`px7l_checked_host_recursive_bind`, `px7m_hostresult_computational_match`,
`px8ta_oriented_subcontinuation`, `px8x_single_schema_observation`,
`rt_parity_native`, `rt_escape_second_resource_native`.

## THE `D0` TABLE — write it DURABLY into this file

The original four axes **omit two facts needed to decide whether the route is
buildable**. Per row, at least:

1. stable **row identity** and its **current terminal guard**;
2. **which of the four carried-residual consumers** it reaches;
3. **reachable producer outcomes**;
4. **selected constructor** and **recursive position**;
5. **whole-source resolution outcome**;
6. the **complete descriptor set within the selected bucket**, and whether it
   agrees;
7. **capture/input representation**;
8. **invocation-coordinate / context availability at that exact consumer**;
9. **measured boundary kind**;
10. **disposition and evidence**.

**Boundary kind uses a CLOSED taxonomy:** intra-artifact generated-unit call,
live-domain cross-artifact call, durable export/persistence, or **unresolved**.

> ### `unresolved` IS A HANDBACK, NOT AN "INTENDED REFUSAL"
>
> An **intended refusal must cite the deciding prohibition or the failed
> authority condition.** A row nobody could classify is an unresolved hard stop
> and comes back to the Steward. Do not let the two collapse into one bucket —
> that is how an unclassified row acquires the appearance of a decided one.

> ### THE COORDINATE COLUMN IS LOAD-BEARING. Do not drop it as redundant.
>
> `call_declared_context` can append planner-authorized captures **only after it
> resolves a context**. **At least one current consumer supplies no coordinates
> and deliberately refuses raw fallback when that body has a generated context.**
>
> ⇒ **"Capture representation is `Record`-shaped" alone does NOT establish that
> the call site can supply it.** A classification that reads capture shape and
> stops has not answered the question the implementation needs answered.

## `D0` classification at base `6f86c9449560c82c0b728d9c598cf3f2461a0ef7`

**This section REPLACES the former classification at base `26fcced259`.** On
every row of that table, **eight of the ten axes carried no measurement** — six
read literally `unmeasured`, and both the boundary and the disposition read
`unresolved` — which is what `AC-5` refuses to close on. Nothing below is
carried over from it: every fact here was measured at this base by the recipe
recorded under "Measurement", and where the old table was *wrong* rather than
merely empty, the correction is stated rather than silently applied.

### Set 1: independently derived authority surface (`AC-1`)

The sole authority-minting seam is
`Lowering::call_declared_recursive_position_unit`, now at
`crates/ken-runtime/src/cranelift_backend/lowering/calls.rs:681` — **moved out
of `core.rs` by [[RT-BACKEND-MODULE-SPLIT]]**, which is why it is re-derived by
name here and not by the old file/offset. Its private helper
`call_declared_context` is `calls.rs:800`.

Seam behaviour, read at this base:

- with `Some(coordinates)` it resolves a context through
  `static_transition_plan.carried_invocation_context(continuation_origin,
  recursive_position, body_origin)`; with `None` the context is `None` without
  a lookup;
- it then **fails closed over both routes into "no context"** — if no context
  resolved *and* some continuation context has
  `worker_body_origin() == body_origin`, it refuses
  (`ContinuationSpecialization`) rather than emitting the raw target;
- `Some(context)` goes to `call_declared_context`, which appends the context's
  captures **in declared order**, resolving each through
  `resolve_context_capture_claim` and indexing
  `function_local.defining_abi_operands`; `None` goes to `call_declared_unit`.

The whole-source authority is `recursive_position_unit_body` (`core.rs:10796`)
delegating to `resolve_recursive_unit_body` (`core.rs:10813`), which returns one
`Option<StaticOriginId>` for the whole source:

- a `Construct` source yields `Some(body)` only when the recursive-position
  argument is a `Closure`, or a `LexicalClosure` **whose captures are empty**;
  every other argument shape yields `None`;
- a `Match` source resolves each arm, returns `None` as soon as one arm's
  `Construct` lacks the position or one arm fails to resolve, and otherwise
  reduces the arms through `agreeing_recursive_body_unit` (`core.rs:1129`);
- any other source expression yields `None`.

**Two facts about that agreement matter for the implementation cut, and neither
is visible from the function's name.** First, `agreeing_recursive_body_unit`
compares **`StaticOriginId` equality only** — it is not the *complete declared
call descriptor* Set 1 requires, so two arms agreeing there have agreed on body
origin and on nothing else. Second, the resolution is keyed on
`(eliminator.static_origin, position)` at its one call site, `core.rs:11114`
inside `lower_carried_computational_match_inner` — and **the selected
constructor case is in hand at that site** (`case.constructor`, and the loop is
over that case's own `recursive_positions`) **but is not part of the key.**
That is Set 1 condition 4 stated in code rather than in prose.

Every `recursive_unit_body` consumer re-derived at this base, from a closed
census of the field's reads:

| Carried-residual consumer | Site | Authority route | Coordinate authority |
| --- | --- | --- | --- |
| `lower_recursor_residual_call` | `core.rs:3000` | `Some(body)` calls the seam before the raw-value guard | **`None`** — fails closed if that body has a generated context |
| `lower_computational_producer_expr` | `core.rs:3485` | `Some(body)` calls the seam before the raw-value guard | retained `CarriedInvocationCoordinates` |
| `lower_expr` | `core.rs:12660` | `Some(body)` calls the seam before the raw-value guard | retained `CarriedInvocationCoordinates` |
| `source_call_state` | `source.rs:3854` | `Some(body)` calls the seam before the raw-value guard | retained `CarriedInvocationCoordinates` |

**Three corrections to the former consumer table, each measured here.**

1. `lower_carried_computational_match_inner` is **not** a seam caller at this
   base. It is the *resolver* site (`core.rs:11114`) — it mints the
   `Option<StaticOriginId>` into the recursor. The fourth carried-residual seam
   consumer is `lower_expr`.
2. The old table listed four seam call sites; there are **six**. The two not
   listed are on the **specialized `Lowered::Closure`** route rather than the
   carried route — `core.rs:12741` and `source.rs:3947` — and both pass
   `args ++ captures` as declared inputs with `Some(coordinates)`. They are out
   of this node's population (no carried word crosses) but they are the standing
   proof that a capture suffix *can* be presented to this seam.
3. `lower_recursor_residual_call`'s `None` is **structural, not an omission**:
   the pending-`Let` route reaches it through `PendingLetContinuationFrame`
   (`mod.rs:8202`), whose field set is
   `residual / args / call_origin / env / recursive_unit_body` and carries **no
   coordinates**. Coordinates come from
   `CarriedInvocationCoordinates::of(&RecursorInvocationSegment)`
   (`mod.rs:8176` — `selection.static_origin` plus `sibling_position`), and the
   segment is composed away before that frame is built. Supplying coordinates
   at this consumer therefore requires a new field on that frame; it is not a
   call-site fix.

### Set 2: bounded discovery snapshot (`AC-2`)

- **Base SHA:** `6f86c9449560c82c0b728d9c598cf3f2461a0ef7`.
- **Query:** unchanged from the former base and re-run verbatim. It is
  restated here in full rather than cited, because the block that formerly
  carried it was inside the classification section this one replaces:

```sh
for f in crates/ken-cli/tests/{px7f_resource_native,px7l_checked_host_recursive_bind,px7m_hostresult_computational_match,px8ta_oriented_subcontinuation,px8x_single_schema_observation,rt_escape_second_resource_native,rt_parity_native}.rs; do
  awk -v f="$f" '/a carried recursive hypothesis is an eliminated value, not a callable/{hit=1; next} hit && /^[[:space:]]*fn /{sub(/^crates\/ken-cli\/tests\//, "", f); sub(/^[[:space:]]*fn /, ""); sub(/\(.*/, ""); print f " :: " $0; hit=0}' "$f"
done
```

- **Closure:** the resulting identity set is **set-equal** to the durable table
  below — 16 identities, **zero added, zero removed**. This is identity-set
  equality computed with `comm -23` / `comm -13` over the sorted sets, not count
  equality; the count is informational.

**One thing the query cannot see, recorded so it is not rediscovered.** The
query iterates a **frozen seven-file roster**, so a new file carrying the seed
text is invisible to it. A whole-repo `git grep -l` at this base finds an eighth
test file — `crates/ken-cli/tests/rt_branched_scrutinee_unit_body_port.rs` — and
it is **not** a missing witness: its occurrence is a **live `assert!` on the
refusal text** in a passing test, i.e. a positive control that the guard fires,
not an `#[ignore]` annotation naming it as a blocker. The roster is complete
**for the ignore-annotation witness shape**, which is the shape Set 2 is about.
Full accounting of the 19 occurrences: 16 witness annotations (this table), 1
live assertion (above), 1 producer (the guard's own message, `core.rs:2958`),
and 1 in this frame.

### Measurement — how every axis below was obtained

The former classification concluded that the annotation "has no compiler-owned
attribution" and stopped there. **That premise is false at this base**, and the
instrument is already in the tree: `crates/ken-cli` depends on `ken-runtime`
with `features = ["px8-ds-test-support"]`, which is exactly the gate on the
`BranchedScrutineeUnitBodyRoute1` observers (`core.rs:932-1080`). The rows below
were obtained by running each ignored witness and reading the resolver, the
selected constructor case, and the reaching consumer directly:

```sh
scripts/ken-cargo build -p ken-runtime --lib          # materialize libken_runtime.a first
KEN_D0_PROBE=1 scripts/ken-cargo test -p ken-cli --test <file> \
    -- --ignored --nocapture --test-threads=1 <test-name>
```

`KEN_D0_PROBE` gates a **temporary, uncommitted** `eprintln!` probe placed at
the resolver's every `None` return, at the per-arm walk, at the mint site, at
each of the four carried consumers' refusals, and at the seam. **It touched no
production line in the candidate**: the three files were restored by
`git checkout --` and verified **byte-identical by blob hash** to their base
objects (`core.rs a616117a`, `source.rs 7cf81a35`, `calls.rs 5e1c460e`) with a
clean `git status`. `D0` remains classification-only. The probe script is
reproducible from this description; re-deriving it is a few minutes' work and is
cheaper than trusting a stale copy of it.

**What the measurement can and cannot rule out.** Every positive statement below
(consumer, key, arm shapes, capture counts, whole-source result) is a direct
observation on the row. The one **negative** — that no witness reaches the other
three consumers — is bounded by the populations actually executed: the 16 rows,
the `rt_branched_scrutinee_unit_body_port` suite, and the full
`ken-runtime --lib` suite (930 tests), across which
`call_declared_recursive_position_unit` was entered **258 times** and the only
carried-residual refusal consumer ever reached was `source_call_state`. That
measures those routes' reach, not the world; consumers 1-3 are **unwitnessed**,
not proven unreachable.

### Ten-axis classification table

Uniform across all sixteen rows, each measured per row: the terminal guard is
`reject_carried_residual_arguments` refusing `BoundaryCarrier` at **`args = 1`**,
surfacing as `ObjectLinkerPackagingError { stage: ObjectEmission, field:
"checked_process_object" }`; the reaching consumer is **`source_call_state`**
(`source.rs:3854`) with **coordinates present**; the selected key is
**`(<program>::ITree::Vis, recursive position 1)`** with
`recursive_positions = [1]`, `argument_binders = 2`; and the whole-source result
is **`Ok(None)`**. **The failed authority condition is also uniform**, so a row
reading "same failed authority condition" below means exactly this: the
recursive position's argument is a `LexicalClosure` carrying a non-empty capture
list, and `resolve_recursive_unit_body` (`core.rs:10813`) admits a
`LexicalClosure` only when `captures.is_empty()` — documented at
`core.rs:10792-10795` as "its carried value does not expose those capture
operands to a generated call frame". The columns below carry what actually
differs.

| Identity | Consumer / coords | Reachable producer outcomes | Selected key | Whole source | Complete descriptor in the bucket | Captures | Boundary | Disposition / evidence |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `px7f_resource_native.rs :: linked_public_right_denial_preserves_exact_masks` | `source_call_state`, `Some((12, 1))` | `Match` on `Result`, 2 arms: `Err` (no position), `Ok` (position = `LexicalClosure`) | `right-denial::ITree::Vis`, pos 1 | `None` — `match_arm_lacks_position`, arm 0 of 2 | **absent**: the one position-carrying arm resolves `Ok(None)` | 5 | intra-artifact generated-unit call | **intended refusal** — capture-bearing `LexicalClosure` at the recursive position; `resolve_recursive_unit_body` admits `LexicalClosure` only when `captures.is_empty()` |
| `px7f_resource_native.rs :: linked_public_second_release_is_closed_and_the_handle_closes_once` | `source_call_state`, `Some((12, 1))` | `Match` on `Result`, 2 arms: `Err` (no position), `Ok` (position = `LexicalClosure`) | `double-release::ITree::Vis`, pos 1 | `None` — `match_arm_lacks_position`, arm 0 of 2 | **absent**: position-carrying arm resolves `Ok(None)` | 5 | intra-artifact generated-unit call | **intended refusal** — same failed authority condition |
| `px7l_checked_host_recursive_bind.rs :: delayed_capturing_generic_bind_agrees_across_real_executors` | `source_call_state`, `Some((11, 1))` | `Match` on `Bool`, 2 arms: `True` and `False`, **both** carrying the position as `LexicalClosure` | `px7l-recursive-bind::ITree::Vis`, pos 1 | `None` — `match_arm_body_unresolved`, arm 0 of 2 | **absent**: *both* arms resolve `Ok(None)` | 3 | intra-artifact generated-unit call | **intended refusal** — both producer outcomes carry captures; no arm yields a descriptor to agree on |
| `px7l_checked_host_recursive_bind.rs :: runtime_selected_non_unit_response_is_consumed_across_real_executors` | `source_call_state`, `Some((11, 1))` | `Match` on `Bool`, 2 arms `True`/`False`, both position = `LexicalClosure` | `px7l-consumed-runtime-response::ITree::Vis`, pos 1 | `None` — `match_arm_body_unresolved`, arm 0 of 2 | **absent**: both arms resolve `Ok(None)` | 3 | intra-artifact generated-unit call | **intended refusal** — same failed authority condition |
| `px7m_hostresult_computational_match.rs :: dynamic_ok_payload_selects_a_multistep_tree_across_real_executors` | `source_call_state`, `Some((11, 1))` | outer `Match` on `Result`, 2 arms: `Err` (position = `LexicalClosure`), `Ok` (**body is itself a `Match`**, 1 arm `Unit::MkUnit`, position = `LexicalClosure`) | `px7m-ok::ITree::Vis`, pos 1 | `None` — `match_arm_body_unresolved` at both nesting levels | **absent** at both levels | 4 | intra-artifact generated-unit call | **intended refusal** — the only row with a **nested** producer `Match`; the nesting changes the walk, not the outcome |
| `px7m_hostresult_computational_match.rs :: dynamic_err_payload_selects_a_multistep_tree_across_real_executors` | `source_call_state`, `Some((12, 1))` | `Match` on `Result`, 2 arms `Err`/`Ok`, both position = `LexicalClosure` | `px7m-err::ITree::Vis`, pos 1 | `None` — `match_arm_body_unresolved`, arm 0 of 2 | **absent**: both arms resolve `Ok(None)` | 5 | intra-artifact generated-unit call | **intended refusal** — same failed authority condition |
| `px8ta_oriented_subcontinuation.rs :: public_one_level_bracket_finishes_and_releases` | `source_call_state`, `Some((12, 1))` | `Match` on `Result`, 2 arms: `Err` (no position), `Ok` (position = `LexicalClosure`) | `px8ta-depth-1::ITree::Vis`, pos 1 | `None` — `match_arm_lacks_position`, arm 0 of 2 | **absent**: position-carrying arm resolves `Ok(None)` | 5 | intra-artifact generated-unit call | **intended refusal** — same failed authority condition |
| `px8ta_oriented_subcontinuation.rs :: px8ds_real_same_depth_path_rejects_flat_order_and_runs_exact_edges` | `source_call_state`, `Some((34, 1))` | `Match` on `Result`, 2 arms: `Err` (no position), `Ok` (position = `LexicalClosure`) | `px8ds-retired-flat::ITree::Vis`, pos 1 | `None` — `match_arm_lacks_position`, arm 0 of 2 | **absent**: position-carrying arm resolves `Ok(None)` | 4 | intra-artifact generated-unit call | **intended refusal** — same failed authority condition |
| `px8x_single_schema_observation.rs :: linked_route_exposes_real_ordered_bindings_and_filters_reserved_input` | `source_call_state`, `Some((12, 1))` | `Match` on `Result`, 2 arms: `Err` (no position), `Ok` (position = `LexicalClosure`) | `px8x-single-schema-observation::ITree::Vis`, pos 1 | `None` — `match_arm_lacks_position`, arm 0 of 2 | **absent**: position-carrying arm resolves `Ok(None)` | 5 | intra-artifact generated-unit call | **intended refusal** — same failed authority condition |
| `rt_escape_second_resource_native.rs :: escape_resource_plus_plain_matches_interpreter` | `source_call_state`, `Some((12, 1))` | `Match` on `Result`, 2 arms: `Err` (no position), `Ok` (position = `LexicalClosure`) | `rt_escape_escape_res_plus_plain::ITree::Vis`, pos 1 | `None` — `match_arm_lacks_position`, arm 0 of 2 | **absent**: position-carrying arm resolves `Ok(None)` | 5 | intra-artifact generated-unit call | **intended refusal** — same failed authority condition |
| `rt_escape_second_resource_native.rs :: escaped_buffer_used_by_fanning_host_op_matches_interpreter` | `source_call_state`, `Some((12, 1))` | `Match` on `Result`, 2 arms: `Err` (no position), `Ok` (position = `LexicalClosure`) | `rt_escape_escape_buffer_then_readat::ITree::Vis`, pos 1 | `None` — `match_arm_lacks_position`, arm 0 of 2 | **absent**: position-carrying arm resolves `Ok(None)` | 5 | intra-artifact generated-unit call | **intended refusal** — same failed authority condition |
| `rt_parity_native.rs :: buffer_allocate_malformed_capacity_narrows_to_invalid_bounds` | `source_call_state`, `Some((12, 1))` | `Match` on `Result`, 2 arms: `Err` (no position), `Ok` (position = `LexicalClosure`) | `rt_parity_buffer_allocate_single::ITree::Vis`, pos 1 | `None` — `match_arm_lacks_position`, arm 0 of 2 | **absent**: position-carrying arm resolves `Ok(None)` | 5 | intra-artifact generated-unit call | **intended refusal** — same failed authority condition |
| `rt_parity_native.rs :: fs_read_at_malformed_offset_narrows_to_invalid_offset` | `source_call_state`, `Some((12, 1))` | `Match` on `Result`, 2 arms: `Err` (no position), `Ok` (position = `LexicalClosure`) | `rt_parity_fs_read_at_offset_single::ITree::Vis`, pos 1 | `None` — `match_arm_lacks_position`, arm 0 of 2 | **absent**: position-carrying arm resolves `Ok(None)` | 5 | intra-artifact generated-unit call | **intended refusal** — same failed authority condition |
| `rt_parity_native.rs :: fs_read_at_malformed_window_narrows_to_invalid_bounds` | `source_call_state`, `Some((12, 1))` | `Match` on `Result`, 2 arms: `Err` (no position), `Ok` (position = `LexicalClosure`) | `rt_parity_fs_read_at_window_single::ITree::Vis`, pos 1 | `None` — `match_arm_lacks_position`, arm 0 of 2 | **absent**: position-carrying arm resolves `Ok(None)` | 5 | intra-artifact generated-unit call | **intended refusal** — same failed authority condition |
| `rt_parity_native.rs :: fs_read_at_malformed_offset_without_read_right_narrows_to_invalid_offset` | `source_call_state`, `Some((12, 1))` | `Match` on `Result`, 2 arms: `Err` (no position), `Ok` (position = `LexicalClosure`) | `rt_parity_fs_read_at_offset_overlap::ITree::Vis`, pos 1 | `None` — `match_arm_lacks_position`, arm 0 of 2 | **absent**: position-carrying arm resolves `Ok(None)` | 5 | intra-artifact generated-unit call | **intended refusal** — same failed authority condition |
| `rt_parity_native.rs :: fs_write_at_malformed_offset_without_write_right_narrows_to_invalid_offset` | `source_call_state`, `Some((25, 1))` | `Match` on `Result`, 2 arms: `Err` (no position), `Ok` (position = `LexicalClosure`) | `rt_parity_fs_write_at_offset_overlap::ITree::Vis`, pos 1 | `None` — `match_arm_lacks_position`, arm 0 of 2 | **absent**: position-carrying arm resolves `Ok(None)` | 5 | intra-artifact generated-unit call | **intended refusal** — same failed authority condition |

### THE RESULT THAT DECIDES THE POST-`D0` CUT

**Every one of the sixteen witnesses satisfies Set 1's predicate, and
partitioning by `(selected constructor, recursive position)` unblocks none of
them.**

Both halves are measured, and the second is the one that matters. Set 1 holds
on every row: a carried computational recursive position is invoked with one
source argument, the source has two reachable producer outcomes, whole-source
authority is absent, and the compiler-owned key is exactly
`(ITree::Vis, 1)`. So the predicate is sound and this node's diagnosis of the
gap is confirmed against real programs.

But the cut is **necessary and not sufficient**. In all sixteen rows the
surviving `Vis` bucket's descriptor is **still absent after the partition**, for
a reason independent of `Ret`: the recursive-position argument is a
`LexicalClosure` **with 3 to 5 captures**, and `resolve_recursive_unit_body`
admits a `LexicalClosure` only when `captures.is_empty()` — by design, because
"its carried value does not expose those capture operands to a generated call
frame" (`core.rs:10792-10795`).

Twelve rows return `None` at `match_arm_lacks_position` (the `Ret`/`Vis`
asymmetry this node was cut for) and four at `match_arm_body_unresolved`. **The
distinction is invisible in the outcome**: in all sixteen, had the `Ret` veto
been lifted, the answer would still have been `None`. The four
`match_arm_body_unresolved` rows are the proof — **no arm in them trips the
missing-position veto at all**, and they refuse anyway, on the capture condition
alone.

This is exactly the condition this frame already reserved: *"`Vis` may install
one only when the body and captures are lawfully expressible as declared call
inputs. If the captures cannot be supplied through planner-owned operands or an
already-admitted structural-value route, that case still refuses."* Measured, the
antecedent fails on the whole witness set. **The guard is not weakened, and no
row is promoted.**

**Three consequences for whoever scopes the implementation.**

1. **A `D1` that implements only the branch-local partition turns no witness
   green.** Its acceptance cannot be "these rows pass"; it must be a property of
   the authority-minting seam, verified by a fixture whose recursive position is
   a `Closure` or a capture-free `LexicalClosure`. **No such fixture exists in
   the current witness set** — one must be authored, and it is the only thing
   that can discriminate the partition working from the partition being inert.
2. **The residual is capture supply, and it is outside this node's authorized
   scope.** Nothing in the not-authorized list is a route to it, and this `D0`
   proposes none. Whether a capture-bearing `LexicalClosure` at a recursive
   position can present its captures as declared call inputs — plausibly through
   the same planner-owned append that `call_declared_context` already performs,
   and to which the two specialized-`Closure` seam sites are the standing
   precedent — is a **separate design question**, and it, not this cut, is what
   gates [[NATIVE-HANDLE-CARRIER]] and [[PX8-F-CAP-41]] for this population.
3. **`AC-5` is unchanged and this strengthens it.** `D0` does not close the node
   and does not unblock the dependants; the measurement above is what tells the
   post-`D0` cut that unblocking them needs the capture question answered first.

`D0` claims no authorization to weaken `reject_carried_residual_arguments`, to
add a closure lane, or to touch `RT-RECURSIVE-POSITION-ARM-ARITY`.

# ACCEPTANCE

- **`AC-1`** — the **semantic predicate** (Set 1) is recorded, and the **single
  authority-minting call plus every `recursive_unit_body` consumer** is
  **re-derived at the `D0` base**, not cited from this frame.
- **`AC-2`** — the **bounded witness query** is recorded, and closure is
  **identity-set equality** with the durable table. **Count equality does not
  discharge this** — a swapped row preserves the count.
- **`AC-3`** — every table row is classified through **complete per-key
  descriptor agreement**, **capture/coordinate availability**, and **measured
  boundary kind**. **Shared refusal text alone fails. Historical credit alone
  fails.**
- **`AC-4`** — every disposition is exactly one of: **mechanism-owned**,
  **already served by the existing whole-source route**, **intended refusal with
  the exact reason or spec clause**, or **unresolved hard stop**. **No generic
  "excluded".**
- **`AC-5`** — `D0`'s merged artifact **is the table and its evidence**.
  **`D0` alone does NOT close this node** and does **not** unblock
  [[NATIVE-HANDLE-CARRIER]] or [[PX8-F-CAP-41]]; it supplies the **post-`D0`
  implementation cut**.

# BANNED SCOPE

- **No implementation before `D0`.** The **predicate** decides the shape, and
  `D0` is what defines it — see `AC-5`: `D0` does not close this node.
- **Nothing from the not-authorized list above**, and it is not a fallback.
- **No weakening of `reject_carried_residual_arguments`.** It stays the
  backstop wherever branch-local authority is absent.
- **No `RT-RECURSIVE-POSITION-ARM-ARITY` work.** That node's `D1` recut is in
  flight and owns its own `AC-3` control; this node does not touch it.

# CONTENTION

**Both formerly-cited blockers have merged, so the contention this section
described no longer exists.** [[RT-RECURSIVE-POSITION-ARM-ARITY]]'s `D1` recut
is landed, and [[RT-BACKEND-MODULE-SPLIT]] has run: the authority-minting seam
now lives in `lowering/calls.rs`, and the carried-residual consumers are spread
across `lowering/core.rs` and `lowering/source.rs`. A later implementation
deliverable contends on **`calls.rs` and `core.rs`**, not on `core.rs` alone.

**`D0` is classification only and touched no production line** — see the
Measurement subsection for the blob-hash restoration proof. The standing
instruction is unchanged and is now load-bearing rather than cautionary:
**re-derive every symbol by name rather than by offset**, because this node has
already had its seam relocated once underneath a frozen table.
