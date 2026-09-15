---
id: RT-EFFECT-CATALOG-GLOBAL-ARTIFACT-COUPLING
title: "WITHDRAWN -- FILED ON A FALSE PREMISE. There is no global catalog coupling. The six failures were a stale libken_runtime.a linked by the test harness; the premise is retracted and the real defect is refiled as RT-TEST-HARNESS-STATICLIB-CURRENCY."
status: closed
owner: runtime
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Measured by runtime-implementer at evt_1d76d3fz7c4v6 (2026-09-15) while diagnosing the ABI-S6 D5b arc's red. Surfaced as an arc finding, established as a main finding by a four-experiment differential. Steward-filed per COORDINATION section 2 and routed out of the D5b repair candidate deliberately -- see 'Why this is not part of the D5b candidate'."
---

> # WITHDRAWN 2026-09-15, SAME DAY IT WAS FILED. THE PREMISE IS FALSE.
>
> **There is no global catalog coupling. `main` was never at risk.** Everything
> below the retraction is preserved as written so the reasoning can be audited;
> **do not act on any of it.** The real defect is refiled as
> `RT-TEST-HARNESS-STATICLIB-CURRENCY`.
>
> ## What was actually happening
>
> `object_linker_packaging`'s link step resolves its archive by scanning
> `target/` for the **newest `libken_runtime*.a` on disk**. It never asks cargo
> to build one, and `cargo test -p ken-runtime --lib` builds the rlib and the
> test binary — **not the staticlib**. So every run linked a stale archive:
>
>     target/debug/libken_runtime.a          Sep 14 21:05
>     crates/ken-host/effect_abi_v1.catalog  Sep 15 01:29
>
> The C starter bakes `ken_host::HOST_EFFECT_ABI_V1_HASH` at packaging time from
> the freshly compiled constant; the stale archive carries the hash from whenever
> it was last built. `ken_host_invocation_v1_init` compares them, mismatches, and
> the starter returns 1 with nothing on either stream. **That is the silent exit
> 1 — a build-freshness artifact, not a coupling.**
>
> Single-variable proof, same test binary, only the archive rebuilt:
>
>     before   libken_runtime.a Sep 14 21:05   1015 passed,  6 failed
>     after    libken_runtime.a Sep 15 01:45   1021 passed,  0 failed
>
> ## Why the four-experiment differential was convincing and wrong
>
> With the catalog unedited, the stale archive's baked hash still matched, so
> nothing fired. **Any** edit desynchronized them — producing a perfect
> correlation with catalog edits and none with anything else.
>
> **Experiment 4 was the strongest-looking evidence and the least sound.** It
> checked out the green base and applied a pure demotion, which **varies the tree
> while holding the true cause fixed** — a checkout does not rebuild
> `libken_runtime.a`, and the resolver simply takes the newest file present. A
> differential that holds the real variable constant yields a *perfect*
> correlation with whatever is being varied. The strength of the correlation was
> the evidence of the flaw, and it was read as the evidence of the finding.
>
> ## Claims withdrawn, explicitly
>
> - "`main` is green only because nothing has edited `effect_abi_v1.catalog` in a
>   long time." **False.** `main` is fine.
> - "Any seat that touches that file reds six landed tests with no diagnostic
>   output." **False.** Only a seat that then runs the `ken-runtime` lib suite
>   *without* materializing a current staticlib. CI never does — the shard and
>   px8f jobs run `cargo build --workspace --locked` first.
> - "A landmine for whoever next does ordinary ABI work." **Not in that form.**
>
> The last three were the **Steward's** widening of the implementer's measured
> claim, not the implementer's own. The measurement was "six rows fail when the
> catalog is edited"; the fleet-wide framing was added when this node was written
> and when the finding was briefed.
>
> Retracted by runtime-implementer at `evt_2jj57aq790r9e`, unprompted, while the
> node was already landed and already in an operator briefing.
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
