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
