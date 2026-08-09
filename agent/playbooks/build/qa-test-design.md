---
name: qa-test-design
description: The QA test-design review pass — promise classes and the anti-fossilization gate, the prohibited-subject rule (never assert facts about repository text), and the ten hard gates applied before Approve. Load when reviewing what a test promises and what it is about.
archetype: build
scope: build
---

# QA: reviewing a test's design

Build-team QA task procedure. Read at the point of use. Governing playbook:
`qa.md`, which owns what you verify and how you cast a verdict. This file is
the pass over the tests themselves: what each one *promises*, what it is
*about*, and the gates it must clear before you Approve.


## Promise classes and the anti-fossilization gate

**Authoritative reference:** `research/qa-conformance-to-rust-test-guidelines.md`
(operator-commissioned, 2026-07-18). Read it once; it holds the full 10-step
conformance-to-test workflow, the oracle ranking, and the Rust assertion
patterns (evolving partitions, negative conformance, one-axis discriminators,
canonical bytes). This section is the **gate you apply every WP**; the reference
is the how.

The failure that motivated this: a `ken-verify` catalog test asserting *"exactly
nine native and thirteen unavailable"* went CI-red when a later WP legitimately
promoted 4 ops to native — a **milestone census frozen as a permanent
invariant**. The red was maintenance noise, not a real regression; only
full-workspace CI caught it because the count was mirrored across crates. QA's
job is to make full-workspace surprises **semantic, not incidental**.

**Every conformance-derived test declares one promise class — if you cannot
classify it, it is not ready (Block):**
- **Durable invariant** — survives *all* intended extensions that preserve the
  contract. Prefer relations, set equality, disjointness/exhaustiveness, typed
  variants, exhaustive matches. (e.g. "native ∪ unavailable == `HostOpV1::ALL`,
  disjoint, and native == the authoritative `NATIVE_TESTED_TARGETS_V1`" — *not*
  "there are 13 unavailable".)
- **Normative compatibility vector** — pins exact bytes/values *because those
  values are the contract* (ABI op identities, field order, canonical hashes,
  known-answer vectors, grammar arity). Changing one requires a contract
  decision, not a snapshot update.
- **Transition sentinel** — *intentionally* fails when a planned extension
  happens, to force review. Legal **only if labelled honestly**: named for the
  boundary (not the current count), states why extension stops here, names the
  event that retires it, sits beside the authoritative owner, and enumerates its
  blast-radius consumers.

## PROHIBITED SUBJECT — never assert facts about repository TEXT

**Operator rule, 2026-07-26:** *"Test oracles that assert facts about source
code, catalog, or documentation lines are an invitation for failure and delay.
Tests should focus on behavior."*

**BLOCK any test whose subject is the text of the repository** rather than the
behaviour of a program: line numbers, line contents, occurrence positions or
counts in prose, heading inventories, section presence, or a hardcoded census of
where words appear in `catalog/`, `docs/`, `library/`, `spec/`, or `agent/`.
This is **not** weighed against usefulness — a corpus-text assertion is
inadmissible even when it is accurate, even when it once caught something.

**WHY THIS IS A SUBJECT RULE AND NOT A FOURTH PROMISE CLASS — read this
before you argue an exception.** The three classes above govern *what kind of
promise* a test makes; **none of them asks what the test is ABOUT.** So a
corpus-text oracle self-classifies straight into **"normative compatibility
vector"** — *"pins exact values because those values are the contract"* — and
passes this gate cleanly. **That is exactly what happened.**
`crates/ken-elaborator/tests/kw_theorem_source_oracle.rs` pinned 64
`(path, line, count)` rows across 18 files in six top-level trees, froze their
line numbers repo-wide, and blocked an unrelated doc WP while reading, on its
face, as a compliant compatibility vector. And the failure recorded as this
section's *own* motivating example — *"a milestone census frozen as a permanent
invariant"* — **is the same shape**. The gate described the disease and still
admitted the patient.

⇒ **A subject prohibition is orthogonal to the promise classes, which is the
point: you cannot re-classify your way past it.** Ask *"what is this test
about?"* before *"what does it promise?"*.

## The tell, in one question

**The tell:** *"Does an edit that changes nothing about how
any program behaves make this test fail?"* If yes — inserting a paragraph,
renaming a heading, reflowing prose — **the test is measuring the repository,
not the software.** It will red for people who did nothing wrong, in files they
have never read, and the cost lands on whoever is unlucky rather than whoever
erred.

** What to do instead — the property is usually still testable, as behaviour:**
- a *policy* about identifiers → assert the **compiler/elaborator rejects** the
  construct, on a fixture you author. That is behaviour, it is local, and prose
  cannot break it.
- a *generated artifact* must match its source → assert the **generator is
  deterministic** and its output round-trips; do not pin the output's lines.
- a *structural* document invariant (a manifest covers every file) → assert the
  **relation** between two artifacts, keyed on identity, never on position.

## The boundary, stated so it is not over-applied

**Boundary.** a test that parses a
structured data file (a manifest, a lockfile) and checks a **relation** is
behaviour-adjacent and permitted — line-by-line *parsing* is an implementation
detail, not the subject. `crates/ken-cli/tests/library_documentation_gates.rs`
sits on the permitted side: it validates manifest↔library consistency with real
detector controls and hardcodes no coordinates. The prohibition is on
**assertions keyed to textual position or corpus-wide word census**, not on
reading a file.

## The ten hard gates

Apply at review; judgment, not syntax (§9 of the reference).
1. **Traceability** — every test names its spec/conformance source and promise
   class.
2. **Reachability** — ≥1 test reaches the *real* production mechanism ("if the
   mechanism were deleted, would this exact test still pass?" → if yes, it's a
   proxy/hand-fed; Block as sole evidence).
3. **Discrimination** — every boundary has an opposite-observable pair; every
   load-bearing guard/field exercised *independently* (one big malformed fixture
   can't show which checks are live).
4. **Oracle independence** — expected values aren't produced by the same logic
   under test, *unless* "consumer == authoritative producer" is the property.
   Round-trips prove self-consistency, not truth — pair with an exact structural
   or independent vector.
5. **Assertion stability** — typed structure/relations over literals; **classify
   every literal** (contract? fixed-fixture? derived? or merely today's repo
   state?). Contract/fixture literals OK; derived values computed or compared
   relationally; **repo-state literals belong only in labelled sentinels**.
6. **Completeness** — sealed enums exhaustive by construction; **cross-crate
   consumer closure explicitly reviewed** (search consumers of the *element and
   its obligation* — producer, serializer/parser, interp + native, verify/diff,
   docs/manifest, conformance + Rust tests — compare complete sets, never mirror
   counts).
7. **No phantom coverage** — ignored/empty/zero-count/placeholder/success-only
   tests do not count as conformance.
8. **Causality** — before Approve, demonstrate breaking the claimed mechanism at
   its seam makes the unchanged test fail with the expected opposite (scratch
   mutation / prior-commit run / test-only selector; don't keep the mutation)
   — **and when it does NOT fail, suspect a stale input or a broken build before
   concluding the property holds.**
   **The mutation-campaign discipline for this gate is `mutation-prove-a-pin`
   §12** — probe-state enumeration and the missing cell, mutating to the
   nearest legal neighbour, build-breaking mutations, stale inputs, and why
   re-running the implementer's mutation is agreement rather than
   corroboration.

9. **Maintenance** — the test states which intended extensions stay green and
   which incompatible changes go red. If both answers are "any change," it's a
   snapshot/sentinel, not an invariant — label it.
10. **Targeted execution** — affected tests/packages via `scripts/ken-cargo`,
    nonzero test count, inspect the message; **never the workspace locally** —
    full-workspace consumer surprises stay CI's job.
