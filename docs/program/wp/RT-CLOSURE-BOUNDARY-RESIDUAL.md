# WP frame — RT-CLOSURE-BOUNDARY-RESIDUAL (Track-1 consumer / M4)

- Node: `docs/program/issues/RT-CLOSURE-BOUNDARY-RESIDUAL.md`
- Program: `docs/program/issues/RT-NATIVE-CARRIED-VALUE.md`
- Owner: Runtime. Size: M (provisional — the entry census may revise it;
  see deliverable 1).
- Capability tier: T1, at its lighter end. The DESIGN is ruled and carries no
  new invention (apply an already-merged defunctionalization discipline at a new
  seam), but the review turns on a soundness argument — is the checked
  continuation crossing the effect-seat boundary honestly, or is a boundary
  crossing dressed up — so it is not a mechanical differential-diff review.
- Inputs pinned @ origin/main `011bf2a95` (the M6 terminal close).
- Design authority: the Architect's native-program frame `evt_9kat78d438cb`,
  the M6 reach ruling `evt_4sp2xftkmc1mz`, and the post-M6-landing shape
  assessment `evt_1vcwzkd3g0s1r` (verdict: M4 is a DISTINCT BUILD, not a
  collapse-to-re-point — the thinnest class, a proven discipline at a new site).
  Deps satisfied: `RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION` (M6, merged
  `79d64a967` / closed `011bf2a95`), `RT-CLOSURE-CROSSING-ELIMINATE`
  (merged, PR #2327), `RT-CLOSURE-BOUNDARY-LANE` (merged, the origin seam).

## The ruled design (front-loaded — this is not an open fork)

The checked continuation (`lambda response. rec (k response)`) crosses the
effect-seat boundary with no first-class native representation and is refused.
The resolution is RULED and carries no new invention: apply the same
defunctionalization discipline the merged `RT-CLOSURE-CROSSING-ELIMINATE`
(PR #2327) proved for the source-authored closure population, now using the
landed M6 representation (code identity + admitted env `Record` + finite static
apply dispatcher) for the checked-IH population. Give the checked continuation a
first-class native value so it crosses the boundary honestly, rather than being
refused as a runtime-local closure with no durable lane.

This is a consumer of M6, not a re-opening of it. M6 realized the escaping
functional-IH representation at the lowering/construction layer; M4 wires that
representation into the boundary-transfer seam for the rows that still refuse
there after M6.

## Architect disposition (evt_2dnst700ynbeh) — crossing discharged

The M4 crossing mechanism was BUILT and VALIDATED at WIP `422310a32`: the two
direct-result `px8l_recursive_decl_native` rows cross honestly (the `boundary.rs`
Closure refusal is gone, native construction succeeds, both production closure
arms byte-untouched, captures project faithfully). Both then red only at
EXECUTION on a newly-exposed trap (`malformed borrowed process input`,
`object_linker_packaging.rs:2221` value -1). The Architect ruled that trap a
DISTINCT execution-parity successor, NOT unfinished M4 — M4's closure-crossing
contract is discharged. The successor is
[[RT-BORROWED-INPUT-CARRIER-DURABILITY]] (Steward-framed).

This SUPERSEDES deliverable 3 / AC-GREEN below with the permitted continuation:

- **M4 must NOT retain the borrowed-input mechanism in scope.** M4's contract is
  the crossing DECISION; `emit_carrier_tag` / borrowed-input durability is the
  successor's, on the shared carried-value path.
- **Widen to the full censused checked-continuation population** (deliverable 2
  applies to all four census rows, not only the two started), and for each
  measure whether it crosses AND greens end-to-end — a row's capture shape may
  or may not hit the borrowed-input seam.
- **Re-point, do not un-ignore, any row that crosses then reds at a KNOWN,
  already-classified seam.** After M4's crossing, a row may advance to a deeper
  seam that another node already owns; re-point its `#[ignore]` string to that
  owner (an honest advancing refusal, exactly as M6 re-pointed
  `rt_write_writable_stage` to M3), do not un-ignore it, and do not fold that
  node's work into M4. The classification is by EXACT refusal string:
  - `Effect: seat Argument(1) of FsOpen needs ConstructorTag, which it cannot
    observe in CarriedWord` ⇒ M3 [[RT-CARRIED-IH-DISPATCH-SITEOP]].
  - `malformed borrowed process input` (native value -1) ⇒
    [[RT-BORROWED-INPUT-CARRIER-DURABILITY]].
- **Un-ignore ONLY rows that cross AND green end-to-end.** Those land green.
- **Stop-and-report only on a genuinely UNCLASSIFIED seam** — a refusal string
  matching none of the known owners above. A row advancing to a known seam is a
  re-point (proceed autonomously); a row advancing to an unclassified seam is a
  stop-and-report to Architect + Steward, so the Steward can place it.
- **M4 is landable as accepted work per COORDINATION section 8a** even if rows
  carry forward: the crossing mechanism plus the honest re-points is a complete
  M4 deliverable. Measured @ WIP `9de8397a`: `px8f_buffer_native` crosses then
  advances to M3's exact seam (re-point to M3); the two `px8l_recursive_decl`
  rows re-point to the borrowed-input successor; `px8ta_oriented_subcontinuation`
  is a test-structure fix (see the third-stop disposition below).

## Third-stop disposition (Architect evt_4tdex7kqzk2w9) — px8ta is a test fix

The `px8ta` row
`px8ds_real_same_depth_path_rejects_flat_order_and_runs_exact_edges` bundles two
independent assertions, and its retired-flat NEGATIVE control panics before M4's
real subject runs. The Architect ruled M4 must NOT represent the retired-flat
closure — wiring the crossing into a control that asserts a rejection would
invert it. This is a test-structure fix on M4's own surface, not a mechanism
representation and not a successor; the classifier's "unclassified → stop" fired
correctly, surfacing a genuinely new case (a correct negative-control refusal).

Permitted continuation (M4's surface — no successor, no widening):

- **HALF A, the retired-flat negative control** (`with_px8ds_retired_flat_order`,
  `mod.rs:8807`): stays a REJECTION assertion and receives NO M4 representation.
  Since M4 legitimately leaves the retired-flat path un-wired, update HALF A's
  `expect_err` content to the refusal it now truly reaches, OR reconfigure it to
  still exercise the "…do not compose" splice rejection it was built to prove.
  The runtime ring authors the exact assertion; the invariant is that it still
  asserts a rejection and folds no retired-path representation into M4.
- **HALF B, M4's real subject** (the ordinary oriented plan): split so it runs
  independently of HALF A, then classify it by the standing exact-string rule
  above. Its stale ignore reason is `RT-SITEOP-CARRIED-WITNESS` D2 (a
  carried-recursive-hypothesis refusal), so it most likely RE-POINTS to M3
  rather than greening — measure, do not assume green.
- Does not fold M3 or borrowed-input work into M4 and does not gate M4 on the
  retired-flat path. M4 lands per COORDINATION section 8a; the Architect reviews
  the candidate at soundness.

## Fourth-stop disposition (Architect hold evt_7vxzx3b82k7kk; Steward partial-land)

After the third-stop split, `px8ta` HALF B
(`px8ds_real_same_depth_path_runs_exact_edges`) runs independently and STILL
reaches the generic Closure refusal — the ordinary plan's own remaining
crossing, not control contamination. It fails M4's crossing by the PREDICATE'S
DESIGN, grounded by the Architect: HALF B's checked continuation is over a
self-recursive producer (`countdown` calls itself), instantiated as TWO
same-depth sibling IH instances, and applied as a bind continuation — so it is
not structurally contained in the emitted owner's result value, and M4's
capture-only single-static-descriptor model has no single descriptor for a
multi-instance same-depth recursive continuation. Not a mechanism bug.

The Architect HOLDS the same-contract-vs-successor ruling under its §1a trigger
(fourth hard-stop, a representation-scope fork), pending a research prior-art
advisory (Research picked it up, evt_64rxg73jq6651); its leaning is
distinct-successor. HALF B stays ignored + re-pointed to this node (M4) as
holding owner; no M4 extension may be attempted while the advisory is out.

Steward framing call: LAND THE PROVEN PARTIAL NOW (COORDINATION section 8a). The
proven mechanism — the px8f/px8l crossing + honest re-points (px8f→M3,
2×px8l→borrowed-input) + the px8ta split (HALF A independent rejection control) +
any genuinely cross-and-green un-ignores — assembles into an accepted-partial
candidate and lands, with the recursive-continuation population (HALF B + any
sibling rows sharing its shape) carried forward. On landing, M4's node goes
`active` (authorized partial, holding owner), NOT `merged`. When the Architect's
post-research ruling lands: distinct-successor ⇒ HALF B re-points to the new node
and M4 closes `merged`; same-contract ⇒ M4 takes a follow-up increment. HALF B
does not gate the proven mechanism.

## Why the population must be CENSUSED, not inherited (measured @ `011bf2a95`)

The node's 2026-08-22 population citations are STALE and must not be frozen into
this frame:

- `px8f_buffer_native.rs:203` carries the `RT-CLOSURE-BOUNDARY-RESIDUAL`
  `#[ignore]` label naming `boundary.rs:1044` — but the label was MEASURED
  2026-08-22, before M6 landed. M6 changed how the escaping functional IH is
  represented, so this row's CURRENT (post-M6) refusal disposition is unmeasured.
- `rt_parity_native.rs:825` (cited in the node title) is a `BufferFreeze`
  narrowing comment at `011bf2a95`, not a boundary refusal. The only
  boundary-labeled ignore in that file is at `:766`, and it carries the ORIGIN
  `RT-CLOSURE-BOUNDARY-LANE` label ("fails at base 21fd46dc"), not the residual.
- `px8f_write_partition.rs:354` (cited in the node title) has no `#[ignore]` row
  at `011bf2a95` at all.
- Six rows carry the origin `RT-CLOSURE-BOUNDARY-LANE` label at a stale base
  (`px8l_recursive_decl_native.rs:196,215`; `px8ta_oriented_subcontinuation.rs:282`;
  `rt_escape_second_resource_native.rs:631,691`; `rt_parity_native.rs:766`).
  `RT-CLOSURE-BOUNDARY-LANE` is merged/resolved, so each of these is either a
  stale ignore on a now-green row or a row that MOVED to the residual seam. Which
  it is per row is unmeasured and is exactly what the census settles.

Every `#[ignore]` label in scope predates M6. A census of them is a reading of
when they were written, not of what the tree does now — so the population is a
measurement the build performs first, not a roster this frame pins.

## Fixed inputs (verified @ `011bf2a95`)

- The seam is the boundary-transfer admissibility walk at
  `crates/ken-runtime/src/cranelift_backend/lowering/boundary.rs:1044`: the
  `Lowered::Closure { .. } | Lowered::DeclarationClosure { .. }` arm returns
  `Err(unsupported("Closure", "a closure cannot cross the boundary: it is
  runtime-local and live-domain only, and it has no durable lane"))`. The
  neighbouring `Lowered::ComputationalRecursorClosure { .. }` arm returns a
  DISTINCT refusal ("a computational recursor closure names an in-flight
  activation, not a transferable value"). The census must record WHICH arm each
  residual row hits post-M6 — the two are different dispositions and only one is
  M4's population.
- M6's landed delta (`79d64a967`) does NOT touch `boundary.rs` — the arm is
  untouched by M6, which is why the rows that reach it still refuse. This is the
  Architect's decisive distinct-build fact.
- The discipline to apply is merged and readable in-tree:
  `RT-CLOSURE-CROSSING-ELIMINATE` (PR #2327), which defunctionalized the
  source-authored closure population; and the M6 representation landed at
  `79d64a967`.

## Deliverables (ordered — the first is a GATE)

1. **Entry census (do this first, before wiring anything).** At `011bf2a95`,
   run the checked-write / PX8 native-witness rows and record the CURRENT
   boundary-refusal population: every cli-test row that refuses at
   `boundary.rs:1044`, and for each, WHICH arm (`Lowered::Closure` /
   `DeclarationClosure` vs `ComputationalRecursorClosure`) and the exact refusal
   string. Classify the six `RT-CLOSURE-BOUNDARY-LANE`-labeled rows: green-and-
   mislabeled, or moved-to-residual. The result is the bounded M4 population.
   Two return-fork conditions: (a) if a row refuses at
   `ComputationalRecursorClosure` rather than the `Closure` arm, it is a
   different disposition — record it, do not silently fold it in; (b) if the
   census population is materially larger than the "thin, ~3-row" estimate (e.g.
   the six LANE rows are all live residual), STOP and report to the Steward so
   the size and sequencing can be revised before the wiring lands.
2. **Apply the representation at the boundary seam.** For the censused
   `Lowered::Closure`-arm population, give the checked continuation the landed M6
   first-class representation so it crosses the boundary honestly, following the
   `RT-CLOSURE-CROSSING-ELIMINATE` discipline. No arm of `boundary.rs` is
   weakened or relabeled; the refusal is removed by making the value
   transferable, not by admitting a non-transferable one.
3. **Green the censused rows.** Remove the `#[ignore]` labels for the rows the
   census proved are this population and land them green (their first-order PX8
   witnesses — Wrote/ReadSome and the checked-write full programs — run on the
   native backend). Any row that does NOT green after the wiring is a genuine
   further refusal ⇒ STOP and report to Architect + Steward.

## Deliverable 1 result — post-M6 entry census

Measured on current release `main` `cf86ed061` (whose only delta from the M6
close `011bf2a95` is this node's release documentation). The Runtime staticlib
was materialized first. Every row ran individually under
`env -u RUST_MIN_STACK` and host `umask 0022` through:

```text
scripts/ken-cargo build -p ken-runtime --lib
scripts/ken-cargo test -p ken-cli --test <target> <row> \
  -- --exact --ignored --nocapture
```

The bounded input inventory was all seven ignored rows in `px8*.rs`, all six
ignored rows in `rt_escape_second_resource_native.rs`, and all six ignored rows
in `rt_parity_native.rs`: 19 rows total. This deliberately exceeds the stale
seven-row label roster. `px8f_write_partition` is a `ken-verify` target, not a
`ken-cli` native-witness target, and is outside this CLI census.

Exactly four rows reach the M4 boundary seam. Each emits the exact same
object-emission refusal and therefore reaches the combined
`Lowered::Closure { .. } | Lowered::DeclarationClosure { .. }` arm:

| CLI target | row | exact refusal | exact arm |
|---|---|---|---|
| `px8f_buffer_native` | `linked_checked_write_all_observes_short_progress_and_matches_interpreter` | `unsupported runtime-IR lowering: Closure: a closure cannot cross the boundary: it is runtime-local and live-domain only, and it has no durable lane` | `Lowered::Closure { .. } | Lowered::DeclarationClosure { .. }` |
| `px8l_recursive_decl_native` | `dynamic_zero_seed_takes_the_base_case` | `unsupported runtime-IR lowering: Closure: a closure cannot cross the boundary: it is runtime-local and live-domain only, and it has no durable lane` | `Lowered::Closure { .. } | Lowered::DeclarationClosure { .. }` |
| `px8l_recursive_decl_native` | `dynamic_multistep_seed_preserves_updated_parameter_order` | `unsupported runtime-IR lowering: Closure: a closure cannot cross the boundary: it is runtime-local and live-domain only, and it has no durable lane` | `Lowered::Closure { .. } | Lowered::DeclarationClosure { .. }` |
| `px8ta_oriented_subcontinuation` | `px8ds_real_same_depth_path_rejects_flat_order_and_runs_exact_edges` | `unsupported runtime-IR lowering: Closure: a closure cannot cross the boundary: it is runtime-local and live-domain only, and it has no durable lane` | `Lowered::Closure { .. } | Lowered::DeclarationClosure { .. }` |

The fourth row was absent from the stale roster because its ignore label still
names the earlier `RT-SITEOP-CARRIED-WITNESS` refusal. Its test first runs a
test-only retired-flat-order negative control, so the production path was
checked separately by temporarily removing only that wrapper: the ordinary
plan emitted the same exact closure refusal. The test file was then restored
byte-identically. This row is a real production residual, not an artifact of
its negative control.

No censused row emits the neighbouring exact refusal
`unsupported runtime-IR lowering: ComputationalMatch: a computational recursor
closure names an in-flight activation, not a transferable value`; the
`Lowered::ComputationalRecursorClosure { .. }` return fork is empty.

The six stale `RT-CLOSURE-BOUNDARY-LANE` rows classify as follows. None is now
green. Two moved to M4; four moved past the old closure refusal to distinct,
non-M4 refusals, so the frame's former green-or-M4 binary was incomplete:

| row | measured disposition |
|---|---|
| `px8l_recursive_decl_native::dynamic_zero_seed_takes_the_base_case` | M4 residual: exact `Closure` refusal above |
| `px8l_recursive_decl_native::dynamic_multistep_seed_preserves_updated_parameter_order` | M4 residual: exact `Closure` refusal above |
| `px8ta_oriented_subcontinuation::public_two_three_level_brackets_finish_and_release_lifo` | not M4: `unsupported runtime-IR lowering: Effect: seat Argument(1) of FsOpen needs ConstructorTag, which it cannot observe in CarriedWord` |
| `rt_escape_second_resource_native::escaped_resource_used_by_fanning_host_op_matches_interpreter` | not M4: `unsupported runtime-IR lowering: ComputationalMatch: tree-producing match scrutinee is not Bool or a constructor` |
| `rt_escape_second_resource_native::nat_fanout_escaped_resource_matches_interpreter` | not M4: `unsupported runtime-IR lowering: ComputationalMatch: tree-producing match scrutinee is not Bool or a constructor` |
| `rt_parity_native::fs_write_at_malformed_offset_narrows_to_invalid_offset` | not M4: `unsupported runtime-IR lowering: Effect: seat Argument(1) of FsOpen needs ConstructorTag, which it cannot observe in CarriedWord` |

The bounded M4 population is therefore four rows: one more than the thin
approximately-three estimate, not a material expansion. The size return fork
does not fire. The Runtime leader accepted this census and authorized the
representation wiring at `evt_7383v69fb6e5q`.

## Deliverables 2–3 attempt — stop disposition

The first direct-result subpopulation was wired without touching either
`boundary.rs` refusal arm. The planner issues a positional environment record
for the exact lexical-closure occurrence and emission owner. The producer emits
only its capture words in that aggregate, with no runtime code tag. The caller
retains only the planner-issued environment-record identity as compiler
metadata, resolves the static body from the same descriptor, projects the
capture words, and dispatches directly to the declared worker body. `body`
remains `Lowered::Closure`'s sole code identity.

Both `px8l_recursive_decl_native` rows then pass native object construction:
the exact `Closure: a closure cannot cross the boundary` refusal is retired.
Both remain red at execution, however, with the newly exposed exact observation
`ken native trap: malformed borrowed process input`, an empty effect trace,
`RuntimeTrap(1)`, and exit status 1. The zero-seed row expected normal exit 0;
the multistep row expected returned error 7.

A temporary, byte-restored diagnostic assigned distinct sentinels to the
emitted carrier guards and localized the new runtime sentinel to
`emit_carrier_tag`, on the generic carried-match path rather than either closure
boundary arm. A separate capture-class probe at the static dispatcher observed
the three transported captures in source order as `Int`, `Constructor`, and
`BorrowedOpaque`; the environment projection itself did not collapse or reorder
the capture run.

This is deliverable 3's named stop: a censused row does not green after honest
representation wiring. No ignore was removed, the remaining two population
rows were not widened into this newly exposed runtime debt, and no
`ComputationalRecursorClosure` disposition changed. At that point the checkpoint
remained a non-candidate pending the disposition now recorded above.

## Authorized continuation attempt — second stop disposition

The branch was rebased onto the landed disposition at `b745c2c93`, then the
representation was widened to the next censused row without changing either
production closure-refusal arm. Planner ownership records remain available for
validation, while crossing authority is narrower: a lexical closure must carry
a non-empty positional environment and be structurally contained in the exact
emitted owner's result value. The exact generated-unit result and mixed carried-
constructor paths replace only such an authorized capsule with its positional
environment. The ordinary one-way producer and the generic closure refusal stay
unchanged.

Two existing controls remain green: a capture-free source closure stored as
constructor data still reaches the exact closure-boundary refusal, and an
ordinary specialized constructor child still crosses through the direct
one-way producer. These distinguish the new route from blanket closure
admission and from blanket replacement of constructor-child transfer.

The next census row — target `px8f_buffer_native`, row
`linked_checked_write_all_observes_short_progress_and_matches_interpreter` —
then crosses the M4 seam: the exact `Closure: a closure cannot cross the
boundary` refusal is gone. It does not green end-to-end and does not reach the
borrowed-input successor. It advances at object emission to the distinct exact
refusal:

```text
unsupported runtime-IR lowering: Effect: seat Argument(1) of FsOpen needs ConstructorTag, which it cannot observe in CarriedWord
```

That refusal is owned by `RT-CARRIED-IH-DISPATCH-SITEOP` (M3), not
`RT-BORROWED-INPUT-CARRIER-DURABILITY`. It is therefore a new seam under the
amended continuation rule and fires the second stop before the fourth census
row is attempted. No ignore was removed at that checkpoint.

## Authorized continuation attempt — third stop disposition

The Steward's next disposition authorized the PX8F row to be re-pointed to
`RT-CARRIED-IH-DISPATCH-SITEOP` and both PX8L rows to be re-pointed to
`RT-BORROWED-INPUT-CARRIER-DURABILITY`, without un-ignoring any of them and
without folding either successor's work into M4. The test bodies and
expectations remain unchanged.

The fourth census row, target `px8ta_oriented_subcontinuation`, row
`px8ds_real_same_depth_path_rejects_flat_order_and_runs_exact_edges`, was then
run in full under the default ambient stack classification. Its test-only
retired-flat-order control reaches the helper thread and stops at the exact M4
refusal before the ordinary-plan half can run:

```text
unsupported runtime-IR lowering: Closure: a closure cannot cross the boundary: it is runtime-local and live-domain only, and it has no durable lane
```

This is neither native end-to-end green nor either authorized successor string.
It therefore fired the explicit third stop condition. The Architect then ruled
that stop to be test structure, not another closure to represent: the retired-
flat negative control and ordinary production plan must be independent halves.

## Split-half continuation — fourth stop disposition

The combined PX8TA row was split without changing either program. Half A,
`px8ds_retired_flat_order_does_not_gain_m4_representation`, keeps the retired-
flat mutation and positively asserts its one exact generic closure refusal. It
runs green as a negative control and receives no M4 representation.

Half B, `px8ds_real_same_depth_path_runs_exact_edges`, independently builds the
ordinary plan from the same source under the default ambient stack. It still
stops at object emission with the exact M4 refusal:

```text
unsupported runtime-IR lowering: Closure: a closure cannot cross the boundary: it is runtime-local and live-domain only, and it has no durable lane
```

The refusal is therefore the production plan's own remaining M4 crossing, not
an artifact of the retired-flat half. It is neither end-to-end green nor the
known M3 or borrowed-input successor. This fires the explicit unclassified-
refusal stop. The ordinary half remains ignored under M4; no censused row was
un-ignored, and this checkpoint is not an accepted M4 candidate.

## Acceptance criteria and controls

- **AC-CENSUS (this frame).** The post-M6 boundary-refusal population at
  `011bf2a95` is recorded with the per-row arm and refusal string, and the six
  `RT-CLOSURE-BOUNDARY-LANE` rows are each classified. The stale 2026-08-22
  citations are superseded by this measurement. If a return-fork condition
  fires, the WP stops there.
- **AC-CROSSING.** The censused checked-continuation population crosses the
  boundary via the M6 first-class representation, applying the merged
  `RT-CLOSURE-CROSSING-ELIMINATE` discipline. No `boundary.rs` arm is weakened,
  relabeled, or made to admit a runtime-local closure as transferable; the
  refusal is retired by honest representation, not by loosening the gate.
  Control (Adversary): a boundary crossing dressed as the representation, a
  `Closure`/`DeclarationClosure` arm relaxed to admit a non-transferable value,
  or a row silenced (ignore removed) without actually running green, must be
  detectable and is a reject.
- **AC-GREEN.** Every row the census assigned to this population runs green on
  the native backend with its `#[ignore]` removed; no row is left silently
  ignored, and no row outside the censused population is touched.
- **AC-NO-REGRESSION.** Whole-suite green in CI (COORDINATION section 12).
  Local targeted `-p` / `--test` only, never `--workspace`.
- **Required reviewers.** Architect — soundness review that the boundary
  crossing is honest (the value is genuinely transferable, not a refusal
  loosened). Adversary — over-accept hunt (a relaxed boundary arm; a row
  un-ignored without a real green run; a non-transferable value admitted).

## Contention check

Touches `ken-runtime` lowering (`boundary.rs`, and whatever construction/apply
sites the M6 representation is threaded through) plus the cli test rows. Within
lane 1 this is the critical path after M6 — M3 (`RT-CARRIED-IH-DISPATCH-SITEOP`)
is the sibling consumer and is sequenced AFTER this by the Steward, so no
concurrent runtime work runs on these files. Cross-lane: the language lane's
in-flight work does not touch `boundary.rs`, so it is contention-free now.
Workspace-green means green in CI, not a local `--workspace` run.

## No-regression

The censused rows advance from a documented boundary refusal to green; no
previously-green row may red. `--locked` and conformance run in CI.
