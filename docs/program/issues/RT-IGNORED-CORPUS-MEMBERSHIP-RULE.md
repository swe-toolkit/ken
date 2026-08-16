---
id: RT-IGNORED-CORPUS-MEMBERSHIP-RULE
title: "State the measurement corpus's exclusion by its membership rule -- all 33 ignored tests -- rather than by the six whose stated reason matched, which re-selects by reason at one scale down from the defect just repaired"
status: merged
owner: runtime
size: XS
gate: none
depends_on: [RT-BOUNDARY-IGNORED-CORPUS-MEASURE]
blocks: []
github: "https://github.com/swe-toolkit/ken/pull/2389"
origin: "Steward, 2026-08-16, on Adversary hunt evt_6d81evnk2nyfn against the merged range 2b4ad0faa..c88a5e423 (PR #2381). The 33/161 census and the per-node breakdown were re-counted against the tree by the Steward before filing. Steward-filed per COORDINATION section 2."
---

## The defect: the predecessor named a reason where the corpus applies a rule

**`RT-BOUNDARY-IGNORED-CORPUS-MEASURE` landed a comment saying the measurement
corpus excludes the six closure-at-boundary tests.** That is true and it is not
the membership rule.

**Censused by the Steward at the merge SHA, in `crates/ken-cli/tests`:**

| node id on the `#[ignore]` | count |
|---|---|
| `RT-CARRIER-BYTESPAN-OBSERVE` | 20 |
| **`RT-CLOSURE-BOUNDARY-LANE`** | **6 (the ones named)** |
| `RT-CARRIED-RESOURCE-SCALAR` | 3 |
| `RT-FRAME-MARKER-ONCE` | 2 |
| `RT-PROCESS-EXIT-STATUS` | 1 |
| `RT-COMPMATCH-TREE-SCRUTINEE` | 1 |
| **total `#[ignore]`** | **33**, against 161 `#[test]` |

**An ignored test contributes zero returns regardless of WHY it is ignored.**
The corpus's membership rule is `#[ignore]` — **not `#[ignore]` for closures.**

> ### THIS IS THE PREDECESSOR'S OWN DEFECT, ONE SCALE DOWN.
>
> The predecessor existed because a comment cited *"81 completed returns, all
> empty"* from a corpus that silently excluded the population which could have
> disagreed. **Its repair names six tests by their stated reason — which
> re-selects by reason exactly as the original did.** A reader now takes "the
> six closure-at-boundary tests" as the exclusion and 27 further exclusions
> remain invisible.

**The largest excluded block is not remote from the mechanism.** The 20
`RT-CARRIER-BYTESPAN-OBSERVE` ignores concern **synthesized-aggregate carrier
transport — the same subsystem being measured**, refused because *"the
synthesized `FileError` declares `SiteOperand(0)`, which demands a compile-time
`Lowered` template the carried word cannot supply."* **Whether any of the other
27 can populate `unit_boundary_environment_fields` is neither measured nor
argued**, and this node does not resolve that — it stops the comment from
implying it was considered.

## `D0` — one clause, and it is comment-only

**Say that the corpus excludes all 33 ignored tests, of which the six
closure-at-boundary ones were measured individually.** That states the
membership rule and **keeps the credit for the six.**

**This is a repair to a claim, not a re-measurement.** Do not re-run the 18
returns, do not drive the other 27, and do not remove any `#[ignore]`.

> **Prefer a rule the reader can check over a count that will drift.** A bare
> "33" goes stale the moment an `#[ignore]` is added or removed. **Stating the
> membership rule — the corpus is the non-ignored tests — is what survives**,
> with the count as an at-this-SHA illustration rather than as the claim.

## Acceptance criteria

**`AC-1`. The membership rule appears, not just a larger number.** A reader must
learn that **`#[ignore]` is the exclusion criterion**, so a future block of
ignores is covered by the sentence without anyone editing it.

**`AC-2`. The six keep their credit.** The comment must still record that the
closure-at-boundary six were measured individually with three completed returns
each, empty, then the expected `Closure` refusal. **Generalizing the exclusion
must not delete the specific result.**

**`AC-3`. Comment-only, no production logic change**, and no `#[ignore]`
added or removed. Same bar as the predecessor's `AC-4`.

**`AC-4`. Do not restore the `"fails at base 21fd46dc"` pin.** The Steward's
note on the predecessor calling that "the perishable part" was **inverted** —
dropping the SHA makes the paraphrase more durable, since the reason outlives
the SHA. **The perishable clause is `"each is marked `#[ignore]`"`**, a
present-tense claim that becomes false when `RT-CLOSURE-BOUNDARY-LANE` succeeds.

**`AC-5`.** No-regression, in CI (`COORDINATION §12`). Local validation
targeted only — `-p ken-runtime`, never `--workspace`.

## Banned scope

- **Re-measuring anything.** Not the 81, not the 18, not the other 27.
- **Removing or adding an `#[ignore]`.**
- **Claiming anything about whether the other 27 can populate the field.** They
  are unmeasured and the comment must not imply otherwise in either direction.
- **Repairing the predecessor's over-strong `D1` conclusion in the comment.**
  That conclusion lives in the node file, is corrected there, and was never in
  the landed comment.

## Sequencing

**Lane 1, `XS`, comment-only.** Publishes `--doc-only`. It does not compete with
any measurement node and nothing blocks on it.

## Provenance

Adversary hunt `evt_6d81evnk2nyfn`, read-only at `d472ed6eb`, on the merged
range `2b4ad0faa..c88a5e423` (PR #2381). **The Steward re-counted the 33
`#[ignore]` and 161 `#[test]` totals and the full per-node breakdown against
the tree before filing**, per the standing rule that a report's claims are
confirmed at the point of use.
