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

## Fourth-stop disposition (Architect ruling evt_35sppt0bv08qx — bounded extension)

After the third-stop split, `px8ta` HALF B
(`px8ds_real_same_depth_path_runs_exact_edges`) runs independently and STILL
reaches the generic Closure refusal — the ordinary plan's own remaining crossing,
not control contamination. HALF B's checked continuation is over a self-recursive
producer (`countdown` calls itself by static name), instantiated as TWO same-depth
sibling IH instances, applied as a bind continuation.

Architect ruling (evt_35sppt0bv08qx), with research's prior-art advisory in hand
(evt_72x4sqz19bebz): this is an IN-SCOPE BOUNDED EXTENSION of M4 — a SECOND exact
authorization arm — NOT a distinct successor. It REVERSES the earlier
distinct-successor leaning. Recursion and same-depth-sibling multiplicity are
ORTHOGONAL to representation (Danvy-Nielsen: one source abstraction gives one
constructor used n times via captures; Lean IR `pap`/`fap` share one `FunId`; GHC
`StgRec` == `StgNonRec`). The deciding cardinality is STATIC CODE SHAPES at the
apply site, not dynamic instances — and PX8DS has ONE continuation source (the
producer recurses; the closure knot does not; the two IH instances are two control
edges of ONE code shape). The Architect verified the crux at the object DB: the
refusal is a SEPARABLE AUTHORIZATION predicate, not a representation limit —
`boundary_closure_crossing_environment` (`aggregates.rs:1480`) refuses HALF B only
on result-value containment (captures are non-empty), while the representation and
apply (`call_boundary_closure_environment`, `calls.rs:1568`) are already
capture-only singleton-per-body static dispatch and already multi-instance capable.

The ruled extension (stays M4 — the frame already names all four Closure-arm rows,
HALF B among them, as its population; this finishes M4's contract, no recut):

- **Second authorization arm.** Add a second exact arm to
  `boundary_closure_crossing_environment` for the bind-continuation edge, keeping
  the result-value-containment arm unchanged. Authorize the capture-only crossing
  there IFF the planner proves both (i) a SINGLETON target — `Targets(resume-site)
  = {body_origin}`, one declared `worker_calls[body]` — and (ii) per-response
  instance pairing: each response carries the exact environment word for its own
  dynamic instance.
- **Fail-closed, load-bearing.** Any bind-resume-site that may receive environments
  from two or more body origins, or whose pairing cannot be proven, MUST fail closed
  (stop-and-report). The arm must NOT degrade into generic closure admission and
  carries no runtime code/body tag (Ahmed-Blume: an unknown/non-singleton target
  must carry code identity; M4 is sound EXACTLY while the target is statically
  singleton).
- **Exactness pin (required for the Architect's APPROVE).** A control must RED if a
  non-singleton OR non-paired bind continuation is admitted by the new arm — prove
  the multi-target case is still REFUSED. Without it the extension is an unpinned
  generic-closure hole, the exact soundness regression the boundary refusal exists
  to prevent.
- **Re-measure HALF B under the extension:** singleton + pairing proven ⇒ it
  crosses; then end-to-end green ⇒ un-ignore, or a downstream exact M3 /
  borrowed-input string ⇒ re-point per the classifier. If the proof FAILS for HALF
  B, the multi-target case is a GENUINE successor (a code-discriminator dispatch) —
  stop and report to the Architect; do NOT pre-split.
- **HALF A** stays an independent passing rejection control, untouched by M4
  representation.

Steward sequencing (supersedes the partial-land route recorded above at
`4486e109a`): M4 lands COMPLETE via the extension candidate — the
result-value-containment crossing + the bind-continuation arm + honest re-points
(px8f→M3, 2×px8l→borrowed-input) + only genuinely-green un-ignores — freshly gated
(fresh QA; Architect soundness on the second arm's fail-closed proof + the exactness
pin; Adversary over-accept). The proven-partial candidate `0b7cab211` is superseded
(no vote). M4 closes `merged` on that landing. Only if HALF B's singleton/pairing
proof fails does HALF B become a successor node and M4 land as the partial — the
partial-land is now the FALLBACK, not the plan.

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
- **AC-EXTENSION (bind-continuation arm, Architect `evt_35sppt0bv08qx`).** The
  second `boundary_closure_crossing_environment` authorization arm authorizes a
  capture-only crossing ONLY on a proven SINGLETON target (`Targets(resume-site)
  = {body_origin}`, one `worker_calls[body]`) AND proven per-response instance
  pairing; the result-value-containment arm is unchanged and no runtime code/body
  tag is carried. Load-bearing exactness pin (required for the Architect's
  APPROVE): a test REDS if a non-singleton OR non-paired bind continuation is
  admitted — the multi-target case must still be REFUSED (fail-closed). Control
  (Adversary): the second arm degraded into generic closure admission, a
  non-singleton or unpaired admission, or HALF B un-ignored without genuine
  end-to-end green, must be detectable and is a reject.
- **AC-GREEN.** Every row the census assigned to this population runs green on
  the native backend with its `#[ignore]` removed; no row is left silently
  ignored, and no row outside the censused population is touched.
- **AC-NO-REGRESSION.** Whole-suite green in CI (COORDINATION section 12).
  Local targeted `-p` / `--test` only, never `--workspace`.
- **Gate reviewers (the merge Decision resolves on these).** Fresh QA (runtime)
  — APPROVE on the exact candidate SHA. Architect — soundness that the boundary
  crossing is honest (genuinely transferable, not a refusal loosened), with
  specific attention to the second arm's fail-closed singleton+pairing proof and
  the exactness pin (AC-EXTENSION). CV — conformance validation per the runtime
  gate. The over-accept guards are the IN-CANDIDATE controls of AC-CROSSING and
  AC-EXTENSION (a relaxed boundary arm; the second arm degraded to generic
  admission; a non-singleton/unpaired admission; a row un-ignored without a real
  green run; a non-transferable value admitted), authored in the candidate and
  verified at the gate. The Adversary is NOT a per-candidate gate (COORDINATION
  section 10⁻a): its independent over-accept hunt is the post-merge channel.

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
