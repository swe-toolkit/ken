---
id: CONFORMANCE-DISCHARGE-HAS-NO-MECHANISM
title: "Discharging a conformance seed case has never once been performed in this repo, and three independent facts say so: the `conformance suite` CI job is a placeholder that echoes a string and is structurally incapable of failing, the two D5a seed rows that the seed document says lift on the D1 candidate still read BLOCKED-ON-ABI-S6-D5a four days after D5a landed, and nothing anywhere reads the 85-file corpus. ABI-S6 D5b carries DISCHARGE the MAP_PRIVATE COW seed case as half its stated objective, so D5b has an acceptance criterion that no seat can satisfy or falsify."
status: draft
owner: spec
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Filed by the Steward 2026-09-15 on operator instruction, after the operator asked whether the conformance-validator seat needs reseating for D5b to land. The answer was no -- the seed case is fully authored, including its discriminating control -- and establishing that surfaced this instead. All measurements below are the Steward's, taken at origin/main 19fe0b0ea."
---

## The question this came from, because it determines the scope

The operator asked whether landing ABI-S6 D5b needs authoring work from the
conformance-validator, or whether the conformance corpus is already-supplied
test material. **It is already-supplied.** `conformance/surface/ffi-io/seed-mapping.md:40-60`
carries case 1 complete: promise class (normative property), spec citation
(`38 §1.9`, `38 §1.3`), a specified `given`, a specified `expect` — *"the
post-write file read returns the **original** bytes, not the mapped write"* —
and the discriminating control, *"a MAP_SHARED/write-through implementation
reds… A lone 'the write is visible in the mapping' positive passes such an
implementation; the file-read arm is what refutes it."*

That control is the corpus's characteristic contribution and it is **done**.
The conformance-validator already made its pass — `71b60b93d` is titled *"D5b
frame + CV seed re-key"*.

⇒ **This node is therefore NOT about a missing author, and reseating the
conformance-validator would not close it.** What is missing is a mechanism, and
that is what follows.

## The three facts, each measured

### 1. The `conformance suite` CI job cannot fail

`.github/workflows/ci.yml:511-519`, verbatim:

    conformance:
      needs: classify-paths
      name: conformance suite
      runs-on: ubuntu-latest
      steps:
        - uses: actions/checkout@v4
        # TODO(F2/Team Spec): run the black-box conformance corpus once it exists.
        - name: Conformance (placeholder)
          run: echo "conformance suite not yet implemented (WP F2)"

It checks out the repo and echoes a string. **A green from this job is evidence
about nothing**, and it is reported under a name that reads as coverage.

Measured cost, 2026-09-15: it reported `pass` in 7 seconds on both PR #3669 and
PR #3676. **#3676 is the D5b candidate**, whose second stated objective is a
conformance discharge, and which touches no `conformance/` file at all — its
`seed-mapping.md` blob is byte-identical to `origin/main`
(`e1fe767b79cf9d7d46b43a00f492f5468d7164f3` on both). So the candidate
displayed a green conformance check while shipping zero of its conformance
deliverable. The Steward read that green before checking what was behind it.

### 2. The two D5a rows were to lift on D1. D1 landed. They did not lift.

`seed-mapping.md:8-15` states the staging: cases 2 and 3 (4 KiB granule;
opacity + bounds) are *"RED-UNTIL-BUILT / BLOCKED-ON-ABI-S6-D5a and **lift to
green on the D1 candidate**."*

| artifact | landed | on `origin/main` |
|---|---|---|
| `29f64ff6f` ABI-S6 D5a-core: native anonymous Mapping promotion | 2026-09-10 07:41 | yes |
| `d8bbef963` ABI-S6 D5a-surface Path-B | 2026-09-11 01:13 | yes |
| `d27bd8d13` last commit touching `seed-mapping.md` | 2026-09-10 23:29 | yes |

**Commits touching `seed-mapping.md` after `d27bd8d13`: zero.** Cases 2 and 3
read `expect: RED-UNTIL-BUILT` and `fixture: BLOCKED-ON-ABI-S6-D5a` today
(`:74`, `:77`, `:97`, `:104`), four days after the gate they were blocked on
landed.

**This is the load-bearing fact of the node.** Fact 1 alone would be ordinary
scaffolding debt. Fact 2 is a *performed* experiment: the discharge step was
specified, its precondition was met, and it did not happen — and nothing
detected that it did not happen, because of fact 1.

### 3. Nothing reads the corpus

85 files under `conformance/`. `grep -rn 'conformance' .github/workflows/*.yml`
returns exactly one non-comment hit: the job name in fact 1. There is no runner
in CI and none was found elsewhere.

## What this does to ABI-S6 D5b

D5b's objective is a conjunction (`docs/program/issues/ABI-S6.md:3771-3790`):
promote `MappingAcquireFile` to native, **and** *"DISCHARGE the deferred
MAP_PRIVATE COW seed case… a green-vs-green pass against its named
non-conforming implementation is vacuous and is a **HARD STOP, not a
discharge**."*

⇒ **D5b carries an acceptance criterion that no seat can satisfy or falsify
today.** The criterion is well-written — it even anticipates the vacuous-pass
failure and forbids it by name — but there is no artifact that executes the
named non-conforming implementation, so neither the discharge nor the hard stop
can be observed.

This is not a reason to weaken D5b's criterion. It is the reason this node
exists: **the criterion is right and its mechanism is absent**, and the two
un-lifted D5a rows are the evidence that an absent mechanism silently converts
"discharge it" into "nobody did."

## Deliverables

- **D0 — ESTABLISH WHAT "DISCHARGE" MEANS OPERATIONALLY, BEFORE BUILDING
  ANYTHING.** Name the artifact that would change, the observation that would
  show it changed, and who performs it. The corpus is prose with a `fixture:`
  field; whether a fixture is a Rust test, a `.ken` program plus expectation, or
  a corpus runner input is **not established anywhere** and must not be assumed
  from the field's name. This is the sizing question for everything below.
- **D1 — RESOLVE THE TWO STALE D5a ROWS.** Cases 2 and 3 are stale against
  landed capability. Either lift them (if D5a's landing does satisfy them, show
  the observation) or re-key them to what actually still blocks them. **Say
  which, and why.** Do not lift them on the ground that D5a landed; that is the
  inference that has already failed once here.
- **D2 — THE CI JOB STOPS REPORTING A PASS IT CANNOT EARN.** Either it runs the
  corpus, or it reports `skipped`/`neutral` rather than `pass`. **A job that
  cannot fail must not be green.** The cheap arm is available immediately and is
  not blocked on D0.
- **D3 — D5b'S AC GETS A SATISFIABLE FORM, OR AN EXPLICIT DEFERRAL.** Route to
  the Architect with D0's answer. **Do not silently drop the COW clause** — its
  hard-stop wording is the only thing standing between D5b and a vacuous
  green-vs-green close.

## Acceptance criteria

- **AC-1.** The `conformance suite` job's outcome is a function of the corpus.
  Demonstrate by a deliberate corpus mutation that turns the job non-green — a
  measurement, not an argument. **If D2 lands as the reporting-only arm, this AC
  is amended in the same change** to say the job reports non-pass and that
  corpus coverage is still absent, so a later reader never mistakes a
  `skipped` for a discharge.
- **AC-2.** Every `BLOCKED-ON-*` row in `conformance/` is checked against
  whether its named gate has landed, and each one is either lifted with its
  observation shown or re-keyed with its reason. **State the census method and
  the denominator** — there are 14 such rows across 4 distinct gates
  (`RT-COMPMATCH-TREE-SCRUTINEE` 5, `ABI-S6-D5` 4, `PX9-INC1` 3,
  `LANG-RESERVED-INFIX-NAMES` 2) — so a zero for any gate is readable as
  evidence about that gate rather than about the corpus.
- **AC-3.** D0's answer is recorded as a durable statement, not as a thread
  reply, and it names the artifact, the observation and the performer. A
  deliverable whose meaning lives in conversation is how this defect arose.
- **AC-4.** No regression, green in CI (never a local `--workspace` run;
  COORDINATION §12).

## NOT this node: building the conformance runner

WP F2 is the corpus runner and it is large. **This node must not be read as
funding it, and D2 explicitly offers an arm that does not.** The subject here is
narrower and is worth separating: a gate that reports a pass it is structurally
incapable of earning, and two rows that prove the discharge step does not happen
on its own. Both are closable without F2.

If D0 concludes that nothing short of F2 gives "discharge" a meaning, **that is
a finding to route, not a licence to start F2 under this id.**

## Release trigger and sequencing

`draft`, and deliberately not released. `owner: spec` because the CI job's own
TODO says `TODO(F2/Team Spec)` and the corpus is the Spec enclave's, guarded by
the conformance-validator.

**Every seat that could own this is on the Codex quota wall as of 2026-09-15**
(spec-leader, spec-author, conformance-validator, and the verify ring), with
panes reading ~6490 minutes to reset. **That is the honest reason this is
`draft` and not `ready`** — not a judgment that it is unimportant. It is on
D5b's closing path.

Release when a spec-enclave or verify seat is live, or on an operator reseat.
**D2's reporting-only arm is small enough that any live seat could take it
alone** if the lane wants the misleading green gone sooner.

## Contention

`conformance/`, `.github/workflows/ci.yml`, and possibly
`docs/program/issues/ABI-S6.md`. **Contention-free with the runtime lane's
current work**, which is entirely in `crates/` — the same basis as the doc-track
exception. Does not touch `crates/`.

## Related

- `ABI-S6` — the node whose D5b deliverable carries the undischargeable half.
- `CI-WRITE-PARTITION-JOB-COMMENT-STALE` — the adjacent instance one layer out:
  a CI comment telling readers a job's green is vacuous when it has carried
  signal since 2026-09-05. Same axis, opposite direction, and the pair is the
  point: **a reader cannot tell which of this repo's greens mean anything
  without opening the job.**
- `RT-OBSERVATION-EXIT-STATUS-LAUNDERS-SIGNAL-DEATH` — the same shape at the
  process boundary: an observation that substitutes a value for absent
  information and carries it onward unmarked.
- `RT-TRAP-MESSAGE-NAMES-FAMILY-NOT-POPULATION` — a verdict reported without
  the population that would make it readable.
