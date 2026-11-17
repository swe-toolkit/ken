---
id: SPEC-FFI-IO-PROCESS-NATIVE-STATUS
title: "Spec 38 §1.11's status says a native build of a Process program is refused, and the spawn-plan-wait seed pins that refusal as a transition sentinel, but every Process and pipe operation is now native (ResourceStreamWrite last, at e5177f7c1) and a crates row already gives the argv-env pipe program native parity with the interpreter. Bring the §1.11 status and the seed case to the landed availability, so the contract no longer requires a refusal the product does not make"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Architect evt_4qajb1s6bgwp7 on RT-NATIVE-RESOURCE-STREAM-WRITE-PROMOTION Increment 2: the spec 38 status note and the seed's native-refusal case lag the promotion. Spec Leader evt_6vdt91b4hyr7n asked for a fresh node. Steward-filed per COORDINATION section 2."
---

# Spec 38 §1.11 states native Process availability as landed

## Objective

Spec 38 §1.11's status note and the spawn-plan-wait seed agree with the
availability that main enforces for Process and pipe operations. No case
requires a native refusal that the landed build does not produce, and no
text claims native parity beyond what a row measures.

## Settled inputs (main `e5177f7c1`)

- **The status note.** `spec/30-surface/38-ffi-io.md:994`-`:1003` says
  "interpreter slice implemented; native refused", and that a native
  build of a program declaring `Process` is refused before object
  emission, naming the operations that remain RepresentedUnavailable.
- **The seed case.**
  `surface/ffi-io/process-native-build-refuses-before-object-emission`
  (`conformance/surface/ffi-io/seed-spawn-plan-wait.md:612`-`:629`)
  expects that refusal for the program of
  `surface/ffi-io/process-spawn-passes-argv-and-environment`. Its `why`
  says the transition ends when `PX10-NATIVE-SPAWN-PARITY` lands, and
  that the successor must replace the refusal with native observations.
- **The landed availability.** In `crates/ken-host/effect_abi_v1.catalog`
  all eight Process and stream operations are `native`:
  ResourceStreamRead, ResourceStreamWrite (`:104`), ProcessPlanOpen,
  ProcessPlanPush, ProcessSpawn, ProcessTakePipe, ProcessWait and
  ProcessSignal. The nine operations still `unavailable` are clock,
  entropy and six FS operations. None is a Process operation.
- **The measured parity.**
  `crates/ken-cli/tests/rt_native_settlement_placement_key.rs:201`,
  `f_argv_env_pipe_matches_fresh_interpreter`, builds
  `fixtures/px10_argv_env_pipe.ken` natively and matches a fresh
  interpreter run. That program declares `FS AFull, Process`, and its
  pipe client visits `[FS, Process]` and reads stdout with `read_pipe`.
  These are the properties the seed case names.
- **Contrary premise.** Spec Leader `evt_6vdt91b4hyr7n` reads the case as
  a refusal for the remaining unavailable operations, to be preserved.
  The catalog leaves no Process operation unavailable for it to name. AC-0
  settles whether the seed's program can still refuse.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

The §1.11 status note, the one seed case and the seed's opening status
paragraph, and the `spec/SPEC-PROGRESS.md` line if it repeats the
refusal. No change to §1.11's operations or contract text, to any other
case, or to `crates/`.

## Deliverable

One Spec/CV-only candidate.

## Acceptance

- **AC-0 (CV with the Spec Leader, before text).** Establish, on
  `e5177f7c1`, whether the seed case's program is the program of the
  crates F row or differs in an operation it performs. Name every
  operation it performs and its catalog availability. If any is
  `unavailable`, the case keeps a refusal that names exactly those
  operations. If none is, the case is retired as a transition sentinel
  or rewritten as a native observation that the F row measures.
- **AC-1.** The status note states the landed native availability of the
  Process and stream operations, cites what measures parity, and claims
  nothing about conformance-runner execution or trust.
- **AC-2.** The seed's native case and its opening status paragraph agree
  with AC-0. No case expects a refusal that names an operation the
  catalog marks `native`.
- **AC-3.** The Spec Leader and CV review, with a Decision.

## Stop conditions

- AC-0 finds the seed's program differs from every crates row in a way
  that makes its native outcome unmeasured. Stop to the Steward; a native
  observation without a measuring row is not written.
- The change needs contract text outside the status note, or a `crates/`
  change.

Checks: 1 fired. The two derivations that disagree are the Spec Leader's
reading and the catalog. I traced both: the catalog marks every Process
operation `native`, and the F row measures native parity for an
argv-env pipe program, so the refusal has nothing to name unless AC-0
finds an unavailable operation. 4 fired. I wrote the case's obligation
in the delivered vocabulary: a refusal names RepresentedUnavailable
operations, and the catalog lists none in Process. 8 fired. A seed that
keeps the refusal reads the same on a base where Process is still
unavailable, so AC-2 is keyed to the catalog labels, not to the case's
presence.
