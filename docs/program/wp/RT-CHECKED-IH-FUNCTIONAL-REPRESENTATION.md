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
  weakened or relabeled. The apply dispatcher is exhaustive over the enumerated
  code identities with NO wildcard/fallback arm; an unenumerated identity is a
  BUILD ERROR, not a runtime path — so deliverable 1's enumerability precondition
  becomes a durable compile-time invariant forcing any future apply site back
  through the gate (the ABI-R3 next_in_inventory discipline; freeze a predicate,
  not a roster). That converts the gate from a reading into an enforcement.
  Control (Adversary): a boundary crossing dressed as the new representation, a
  relabeled/weakened arity gate, or a dispatcher widened with a wildcard arm,
  must be detectable and is a reject. (Architect fold-in evt_3wb4cy7f7aj50.)
- **AC-REENUM.** Rerun report-2's checked-family runner end-to-end. Both
  checked-family programs green => the family is bounded; re-point both runner
  tables from the advancing-refusal pins to green. Any FURTHER refusal => STOP
  and report to Architect + Steward.
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
scope is the representation plus the two checked-family programs green
(AC-REENUM). The consumers `RT-CARRIED-IH-DISPATCH-SITEOP` (M3) and
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
