---
id: RT-EFFECT-CATALOG-GLOBAL-ARTIFACT-COUPLING
title: "Any edit to effect_abi_v1.catalog silently breaks six landed object_linker_packaging tests -- including a pure demotion on a green tree with no promotion anywhere. Packaging succeeds, every hash and binding check passes, and the linked executable then exits 1 with empty stdout AND empty stderr. main is green only because nothing has edited that file in a long time."
status: ready
owner: runtime
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Measured by runtime-implementer at evt_1d76d3fz7c4v6 (2026-09-15) while diagnosing the ABI-S6 D5b arc's red. Surfaced as an arc finding, established as a main finding by a four-experiment differential. Steward-filed per COORDINATION section 2 and routed out of the D5b repair candidate deliberately -- see 'Why this is not part of the D5b candidate'."
---

> # FILED 2026-09-15 BY THE STEWARD. STARTABLE NOW, STANDALONE REPRODUCER.
>
> **This is a `main` defect, not an arc defect.** It reproduces on the green
> base tree with none of the ABI-S6 D5b arc present, so the frame below never
> asks anyone to build 24 commits first. One file edit on a clean checkout
> reproduces it.
>
> **Lane 1.** It is on the D5b arc's critical path — D5b's deliverable IS a
> catalog edit, and every catalog edit trips this — but the defect predates the
> arc and the repair does not belong in the arc's candidate.

## The finding in one sentence

`crates/ken-host/effect_abi_v1.catalog` is globally load-bearing on compiled
artifacts: editing **any** operation's availability field changes the runtime
behaviour of compiled programs that never name that operation.

## The reproducer — one edit, on a green tree, no arc commits

Fixed inputs, measured at `4bf1ad362b5a5cfa512636087df5229ee57db706` (the D5b
branch base; `ken-runtime` lib tests there are **1011 passed, 0 failed**):

```
git checkout 4bf1ad362b5a5cfa512636087df5229ee57db706
# in crates/ken-host/effect_abi_v1.catalog, demote MappingWriteView only.
# No promotion anywhere. No other change.
scripts/ken-cargo test -p <the crate owning object_linker_packaging> \
    --test object_linker_packaging
```

    before the edit    27 passed, 0 failed
    after the edit     21 passed, 6 FAILED

## The differential that makes the attribution airtight

Four experiments, each changing exactly one thing, all run on one box minutes
apart. Numbers are the implementer's.

| # | tree | change | result |
|---|---|---|---|
| 1 | D5b tip | demote `MappingAcquireFile` (= base catalog content exactly) | 27 passed, 0 failed |
| 2 | D5b tip | demote `MappingAcquireFile` AND promote `FsSync` (base content, a **different** op promoted, count 26) | 21 passed, **6 failed** |
| 3 | D5b tip | as 2, plus demote `MappingWriteView` (count back to 25) | 21 passed, **6 failed** |
| 4 | **base `4bf1ad362b`** | demote `MappingWriteView` only — **no promotion at all, none of the arc present** | 21 passed, **6 failed** |

**Experiment 2 kills operation-specificity.** **Experiment 4 kills promotion,
kills the native operation count, and kills the arc as the cause, all at once.**

⇒ The variable is not the native path's readiness to carry an operation, and it
is not `MappingAcquireFile`. It is that the catalog file has a global path onto
compiled artifacts that nothing had exercised, because nothing had edited that
file in a long time.

## The symptom, stated precisely — this is not a packaging refusal

Packaging **succeeds**. Every hash check and every binding check passes. The
linked executable is then produced, runs, and **exits 1 with empty stdout and
empty stderr** — both streams captured explicitly and confirmed genuinely empty,
so this is a silent failure inside a real compiled process, not an assertion
mismatch and not a diagnostic that went unread.

The six rows:

    linked_process_executes_exact_big_int_support_without_host_dispatch
    same_process_artifact_observes_fresh_byte_exact_os_input
    process_artifact_maps_exitcode_and_reports_terminal_traps
    (+3 more in object_linker_packaging -- enumerate them in D0)

`linked_process_executes_exact_big_int_support_without_host_dispatch` is the
sharpest statement of the defect available: **a program whose own name says it
performs no host dispatch should not be able to notice which operations are
available.**

## Hypothesis already refuted — do not re-derive it

`af2c377b3` adds a new `PrivateMappingAcquireFile` constructor to the prelude's
`FSOp` datatype, which is a change every compiled program sees. **That cannot
explain experiment 4**, where the prelude is untouched and the defect still
reproduces. The catalog therefore has its own global path, independent of the
prelude datatype, and that path is the live question.

## Why this is not part of the D5b candidate

Folding a `main`-side soundness defect of unknown mechanism into a red-CI repair
would put two unrelated attributions in one diff and make the candidate
unreviewable. The D5b arc must clear this defect to land its deliverable, but it
did not author it and its candidate should not carry the repair.

## Deliverables

- **D0 — name the mechanism.** The path by which a catalog availability field
  reaches a compiled artifact's runtime behaviour, stated at the site. Enumerate
  all six failing rows. D0 is a diagnosis, not a fix, and it closes on a written
  mechanism even if the repair is then re-scoped.
- **D1 — the repair**, shaped by D0.
- **D2 — the control** (see below).

## Acceptance criteria

- **AC-1.** The mechanism is named in prose at the code site responsible, not
  only in the node. A reader who edits the catalog next year meets the
  explanation where they are working.
- **AC-2 (the control, and it needs a positive arm).** A test that applies a
  catalog edit which changes no operation any test program names, and asserts a
  compiled program's observable behaviour is unchanged. **It must be
  demonstrated to RED when the coupling is present** — run it against the
  pre-repair tree and show it failing. A control that has only ever been seen
  green is not yet known to test anything.
- **AC-3.** The reproducer above passes: `object_linker_packaging` is 27 passed,
  0 failed with the `MappingWriteView` demotion applied.
- **AC-4.** No regression, green in CI (never a local `--workspace` run;
  COORDINATION §12).

## Contention

Touches `crates/ken-host/effect_abi_v1.catalog` and whatever artifact/linking
path D0 identifies. **Contends with `wp/ABI-S6-d5b-file-backed`**, which edits
the same catalog file as its deliverable. Sequence this ahead of the arc's
landing; the arc rebases onto the repair.

## Fleet-wide consequence

`main` is green today only because nothing edits this file. **Any seat doing
ordinary ABI work trips this, and gets six red tests with no diagnostic output
to explain them.** That is the cost of leaving it open, and it is not confined
to this arc.
