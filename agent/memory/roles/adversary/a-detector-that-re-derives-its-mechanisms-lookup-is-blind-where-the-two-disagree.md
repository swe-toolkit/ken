---
name: a-detector-that-re-derives-its-mechanisms-lookup-is-blind-where-the-two-disagree
description: A sentinel standing in for a production filter re-implemented its callee join with `find` where production used a `BTreeMap`, and `continue`d on the case production deliberately retains — diff the two lookups as operations, and note that a non-vacuity guard bounds the scan's emptiness, never its coverage
---

# A detector that re-derives its mechanism's lookup is blind where the two disagree

**Measured 2026-08-10 on `35265ca5` (`RT-CALL-EDGE-EXECUTABILITY-AXIS` `D0`), a
recut built from my own earlier finding.**

A boundary sentinel exists to red when a production filter's population starts
to exist. Its fidelity to that filter *is* its whole value. The two joins were
not the same operation:

| | production | sentinel |
|---|---|---|
| lookup | `BTreeMap::collect` then `.get()` | `units.iter().find(...)` |
| duplicate key | **last** write wins | **first** match wins |
| no match | `None => true` — **retains** the edge | `continue` — **drops it from the scan** |

⇒ **The set production treats specially is the one set the detector cannot
see**, and the mismatch is invisible because both spellings look like "look the
callee up."

**Procedure: write the two lookups side by side as operations, not as
intentions.** Ask what each does on (i) a duplicate key and (ii) a miss. A
detector should *reuse the production expression*, not paraphrase it — that
makes the question moot instead of answering it, which is the cheaper repair to
ask for.

## A non-vacuity guard bounds EMPTINESS, never COVERAGE

The sentinel asserted `joined > 0` — the scan was not empty. **That says nothing
about how many edges it skipped.** A future fixture where nine of ten edges fail
to join passes identically.

⇒ **`> 0` and `== total` are different guarantees, and the first is what gets
written** because it is the one that makes the test pass today. When a scan can
skip elements, assert the count or report the skipped ones in the failure
message, so an exclusion is **visible rather than silent**.

**Why it survives review** (the Steward's sharpening on triage, and it is the
part that makes this findable): *a coverage claim resting on a non-emptiness
measurement is the failure mode where **every control over it passes.*** There
is no red anywhere to prompt the question — the guard is green, the scan is
green, the suite is green, and the gap is between two sentences nobody compared.

⇒ **Hunt this by reading the guard's predicate against the claim in its own
failure string**, not by looking for a red.

Distinct from the vacuity guard I verified and could not break: `superseded > 0`
was sound because it and the detector are computed **in the same loop iteration
over the same element**, so it really does put the detector one clause from red.
**Check that sameness explicitly** — a non-vacuity counter computed over a
different pass is the classic
[[differential-oracle-is-blind-to-a-shared-premise]] failure wearing a guard's
clothes.

## Bound a fidelity finding to fidelity; do not inflate it to reachability

I could not show duplicate function ids are reachable — `emittable_units()` maps
1:1 over the ABI descriptors and nothing nearby establishes uniqueness, but I
constructed no duplicate. **Say that as part of the claim**, and let the
severity be *the detector does not mirror the mechanism*, which needs no
reachability argument at all. Inflating it to "and therefore the filter is
wrong" invites a refutation that then buries the real, cheap point.

**Prefer the repair that dissolves the question over the one that answers it**,
and say why: reusing production's expression **survives the invariant later
becoming false**, whereas proving `descriptor.function` unique today is a fact
with an expiry date and no alarm on it. A finding whose fix removes the
dependency is worth more than the same finding with a proof attached — and it is
the cheaper ask, which is what
[[preventive-findings-are-unfalsifiable-so-keep-them-cheap]] says gets acted on.

## The over-claim recurred INSIDE the recut that was fixing an over-claim

The same doc carried the precise sentence (*"the defect's own failure
direction"*) and a loose one (*"the exact disagreement between the two
filters"*) — and the loose form was in the **assertion message**, which is the
only text a future engineer reads when it fires. The predicate is one-sided by
design and correctly so.

**This happened in a recut authored specifically to replace an over-claiming
proxy, by a ring that had just been given that finding.** ⇒ Not carelessness:
**the precise sentence and the summary are written in different passes, and only
the precise one is proofread as a claim.** Same shape as
[[a-type-narrowing-discharges-one-clause-and-a-producer-enumeration-carries-the-rest]].

⇒ **On any detector, read the FAILURE MESSAGE against the predicate separately
from the doc.** The message is the artifact's actual interface to its future
reader, it is written last, and it is where the loose summary lands.
