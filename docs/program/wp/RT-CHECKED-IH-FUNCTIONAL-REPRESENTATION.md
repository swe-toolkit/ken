# WP frame — RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION (Track-1 D0 / M6)

- Node: `docs/program/issues/RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION.md`
- Program: `docs/program/issues/RT-NATIVE-CARRIED-VALUE.md`
- Owner: Runtime. Size: L. Capability tier: T1 (novel value representation,
  soundness-bearing lowering change; reviewed on the argument, not a
  differential diff).
- Inputs pinned @ origin/main `b2a083c71`.
- Design authority: the Architect's native-program frame
  `evt_9kat78d438cb`, the D0 ruling (defunctionalize), the reach broadening
  `evt_4sp2xftkmc1mz`, and the Track-1 entry criterion. Deps satisfied:
  `RT-IH-MARKER-PRODUCER-COMPLETE` (closed), `RT-NATIVE-TRACK0-REARM`
  (merged, `e415be765`).

## The ruled design (front-loaded — this is not an open fork)

The D0 the node once carried (materialized closure value vs
defunctionalized carried tag) is RULED: **DEFUNCTIONALIZE.** Represent the
escaping functional induction hypothesis by:

- a **code identity** (which worker/continuation),
- an **environment** captured as an admitted `Record`, and
- a **finite static apply dispatcher** over the enumerated code identities.

This is not a new invention. `RT-CLOSURE-CROSSING-ELIMINATE` (merged,
PR #2327) applied exactly this discipline to the source-authored closure
population, and `spec/40-runtime/41-values.md:76-118` sanctions live-domain
closure exchange (constraining only the durable lane). The checked continuation
(`lambda response. rec (k response)`) is the population here.

## Fixed inputs (measured seams @ origin/main)

- Failing marker: the single `template=0, arity=0` nullary_force per checked
  program at `core.rs:11699` — distinct from the 12 well-behaved
  `template=4, arity=1` markers at `source.rs:655`.
- Escape measurement (runtime-implementer `evt_79jd1nxamqd95`): the realized
  IH value's `parent_chain` immediate parent is `Expression(Construct)` on BOTH
  checked-family programs — it is stored straight into a constructor field, so
  it escapes; the non-escaping use-site-specialization remedy cannot apply.
- The refusal today: `core.rs:11699` lowers the marker body as an ordinary
  value-producing expression, so `Call(Var(0), args=[])` resolves `Var(0)` to
  the arity-1 StaticWorker and calls it with zero args; the seam passes (0==0)
  and the refusal is the worker gate at `calls.rs:220` (escaping IH held as
  `StaticWorkerBinding` at `calls.rs:222`).
- No existing Ken object carries the value: `StaticWorkerBinding` is compiler
  metadata (`lowering/mod.rs:3578-3603`, "the binding never becomes a value");
  `LoweringOperand = { Specialized(Lowered), Carried(CarriedBoundaryWord) }`
  has no closure/worker arm.
- Likely language spillover: the elaborator's nullary_force emission on the
  marker/planner side, `compiler_driver.rs:1718-1743`.

## Deliverables (ordered — the first is a GATE)

1. **Entry-criterion gate (do this first, before building the
   representation).** Confirm that ALL apply sites of the stored/escaped checked
   IH are statically enumerable from the checked plan — i.e. the escaped value
   is the checked continuation with a finite, statically-known set of apply
   sites, matching CROSSING-ELIMINATE's defunctionalization precondition.
   Enumerable => proceed with the defunctionalized tag (sound). A genuinely
   NON-enumerable apply site is the Architect's return-fork condition: **STOP
   and return to Architect + enclave** (a materialization fork), never a silent
   widen. The Architect's lean, corroborated by the merged source-authored
   case, is that enumerability holds — this gate confirms it on the
   checked-write population before any lowering lands.
2. **The defunctionalized representation.** Introduce the first-class object
   (code id + env `Record` + finite static apply dispatcher) and construct it
   at the nullary_force seam (`core.rs:11699`), so the arity-1 worker is never
   called with zero args.
3. **Application at the escaped use sites.** Apply the representation across
   the `Construct` boundary at the escaped uses, so the two checked-family
   programs' checked-IH invocations lower and run correctly on the native
   backend.
4. **Likely language spillover** on the marker/planner emission
   (`compiler_driver.rs:1718-1743`) — assigned by this node's owner at
   framing per COORDINATION section 9a if the change reaches the elaborator.

## Acceptance criteria and controls

The node (`docs/program/issues/RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION.md`)
states the authoritative list. In brief:

- **AC-ENTRY (this frame).** The enumerability confirmation of deliverable 1
  is recorded (which apply sites, why finite). If non-enumerable, the WP STOPS
  at a return-fork and does not proceed to deliverable 2.
- **AC-REPR.** The realized functional IH is represented by the
  defunctionalized object; the arity-1 worker is never called with zero args;
  the escaped value is constructed and applied honestly across the constructor
  boundary; no closure boundary is crossed illegitimately and no arity gate is
  weakened or relabeled. Off-roster-identity enforcement is TWO-TIER (Architect
  ruling evt_3k0bv5px3m097, refining the earlier single-match fold-in which was
  unbuildable — Rust exhaustiveness is over TYPES, not the u64 template ids): KIND
  — a no-`_` match over the SEALED checked-IH shape set, so a new shape is a
  ken-runtime build error; INSTANCE — the template id resolved through the
  existing fail-closed plan lookup (`computational_ih_call`/`_slot`), the same
  point erasure.rs's double bijection is asserted, no parallel roster to drift;
  VALUE — the crossing value is the admitted env `Record` only, no runtime
  code-identity tag. Every apply/slot site MUST resolve its id through the plan
  lookup. The node holds the authoritative full form and the built-repr review
  conditions. Control (Adversary): a boundary crossing dressed as the
  representation, a relabeled/weakened arity gate, a `_`/fallback arm on the kind
  match, or an apply site bypassing the plan lookup, must be detectable and is a
  reject.
- **AC-REENUM.** Close over EVERY consumer of the retired
  `ESCAPING_FUNCTIONAL_IH` refusal oracle, not only report-2's four-pin
  checked-family source. The Architect ruled (`evt_6eztb270x0067`, part 3)
  that the eight entry-substituted `rt_cold_lowering_path_enumeration` rows
  (`rt_cold_lowering_path_enumeration.rs:575-583`) are a DISTINCT checked-IH
  content population — distinct proc bodies, not byte-copies of the four-pin
  identity — that the original four-fixture reconciliation under-enumerated.
  The re-enumeration is therefore CENSUS-driven, not a fixed roster: grep the
  retired oracle constant across the whole test tree and, after the Part-1 /
  Part-2 repairs make them complete, reconcile EVERY consumer so that (a)
  report-2's single content-distinct checked-family source is green — its four
  fixture copies are byte-identical and collapse to one source identity, so its
  TWO runner tests (not two content-distinct programs) are re-pointed off the
  advancing-refusal pins, and the runner's population pin
  (`family_rows_are_unique_by_source_content`) rejects any label-inflated row,
  holding one row per byte-distinct source (`dbadfad74`); (b) all eight
  `rt_cold_lowering_path_enumeration` rows land as `Disposition::Completes`;
  and (c) no surviving consumer of that oracle anywhere in the test tree still
  expects an `ESCAPING_FUNCTIONAL_IH` / advancing-refusal disposition — the
  grep census is complete by construction, so a leftover refusal-expecting
  consumer FAILS this AC. The implementer reports the full consumer census
  with the respin. Any program that does NOT complete after the repair is a
  genuine FURTHER refusal => STOP and report to Architect + Steward.
- **AC-NO-REGRESSION.** Whole-suite green in CI (COORDINATION section 12).
  Local targeted `-p` only, never `--workspace`.
- **Required reviewers.** Architect — design D0 confirmation AND soundness
  review of the built representation (on the argument). Adversary — over-accept
  hunt (boundary crossing as representation, relabeled arity, an apply
  dispatcher widened to admit a non-enumerable site).

## Reach, and the M3/M4 consumers

Per the Architect's reach ruling (`evt_4sp2xftkmc1mz`), this representation
is the WHOLE remaining PX8-closure critical path: every native checked-IO full
program carries the checked continuation closure, so this D0 gates ALL FOUR
native witnesses (ReadEof/ReadSome/Wrote AND SemanticErrorV1). This WP's OWN
scope is the representation plus the FULL retired-oracle consumer census green
(AC-REENUM) — the two report-2 checked-family programs AND the eight
`rt_cold_lowering_path_enumeration` rows, per the Architect's part-3 census
ruling (`evt_6eztb270x0067`). That ruling also named THE PREDICATE the respin
is authored to: M6 lacked a single closed characterization of its checked-IH
transport POPULATION, so the transform over-applied (forwarding-reorder reached
the transport-free population) and the census under-enumerated (the eight rows).
The respin defines that population per-producer
(`checked_ih_environment_transport_at`), applies the transform EXACTLY to it,
keeps the old fast path for everything else, and censuses EXACTLY its consumers.
The consumers `RT-CARRIED-IH-DISPATCH-SITEOP` (M3) and
`RT-CLOSURE-BOUNDARY-RESIDUAL` (M4) stay gated on it; whether they collapse to
re-measure/re-point work (as Track-0's rows did) or remain distinct builds is
a post-landing assessment for the Architect, not prejudged here.

## Contention check

Touches `ken-runtime` lowering (`calls.rs`, `core.rs`, `lowering/mod.rs`) and
a likely `compiler_driver` (language) spillover. Within lane 1 it is the
critical path — M3/M4 are gated on it, so no concurrent runtime work runs on
these files. Cross-lane: the language lane's in-flight work (FO
checker-soundness D3) is in the proof/checker area, not `compiler_driver`, so
it is contention-free now; if the deliverable-4 spillover reaches
`compiler_driver.rs` while language is also editing that crate, coordinate a
merge window with the Steward. Workspace-green means green in CI, not a local
`--workspace` run.

## No-regression

The two checked-family programs advance from a documented advancing refusal
to green; no previously-green row may red. `--locked` and conformance run in
CI.
