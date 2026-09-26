---
name: a-factor-out-into-a-shared-helper-can-strengthen-a-pre-existing-callers-precondition-so-diff-the-helpers-guard-set-against-each-old-inline-check
description: >-
  ABI-S6 D2 (7cbf0725, PR #3439) post-merge hunt, verdict NO DEFECT. D2 factors
  checked_bounded_range out of the pre-existing BufferRegionV1::initialized_slice
  AND uses it for the NEW MappingRegionV1 view; the extracted helper carries a
  `live_end > capacity` guard the buffer's OLD inline check did not have. A
  factor-out reads as behavior-neutral debt cleanup, but the shared helper's
  guard set need not equal the pre-existing caller's inline guard set -- the
  helper may carry an extra precondition to satisfy a DIFFERENT (new) caller, and
  that guard now also runs for the old caller. So a factor-out can silently
  STRENGTHEN a pre-existing caller's precondition and reject formerly-valid
  inputs; it is inert only if that caller's own producer maintains the invariant
  the new guard assumes (here install_window keeps initialized_start+len<=capacity
  via checked_buffer_range, so the guard never fires and a would-be slice panic
  becomes InvalidBounds -- strictly safer). Method: diff the extracted helper's
  guards against EACH pre-existing caller's old inline check, and for every extra
  guard prove the old caller's producer cannot violate the invariant it assumes,
  on the reachable domain -- do not stop at "the new caller works". Dual of the
  widening-inside-a-shared-callee lesson.
metadata:
  type: feedback
---

# A factor-out into a shared helper can strengthen a pre-existing caller's precondition

**Measured 2026-09-09 on `7cbf0725` (ABI-S6 D2, PR #3439), a post-merge hunt.
Verdict NO DEFECT; this is the method that settled it.**

D2 extracts `checked_bounded_range(capacity, live_start, live_len, start, len)`
and points three sites at it: the pre-existing
`BufferRegionV1::initialized_slice`, and two NEW `MappingRegionV1` view methods
(`bounded_slice` / `write_bounded_slice`). The commit message frames the helper
as "factored once and shared by Buffer/Mapping/FS." A factor-out is normally
read as behavior-neutral debt cleanup. **It is not automatically so.**

## The shared helper's guard set need not equal the old inline check's

The buffer's OLD inline check (before D2):

```
if start < initialized_start || end > live_end { InvalidBounds }
```

The extracted helper carries a guard the old check LACKED:

```
if live_end > capacity || start < live_start || end > live_end { InvalidBounds }
```

The `live_end > capacity` clause exists for the OTHER caller (the mapping view,
whose `live_len = capacity`, makes it a no-op there by construction). But once
`initialized_slice` routes through the helper, **that clause now also runs for
the buffer**. So **a factor-out can silently STRENGTHEN a pre-existing caller's
precondition** -- reject an input the caller previously accepted (here: any
buffer read whose live window's end exceeds capacity, e.g. bytes.len()=8,
initialized_start=2, initialized_len=10, read start=2 len=3: OLD returns
bytes[2..5], NEW returns InvalidBounds). The diff reads as a "no-op refactor";
the guard set at one caller quietly grew, and no test in the diff targets the
strengthened case at the old caller.

## It is inert only if that caller's producer maintains the assumed invariant

The new guard is safe for the buffer ONLY because
`initialized_start + initialized_len <= capacity` holds on every reachable
state. The sole production `install_window` caller (effect_v1.rs:2972) derives
`(start, read)` from `checked_buffer_range(capacity, ...)` with `read <=
effective` and `start + effective <= capacity`, so `start + read <= capacity`;
`clear_window` sets `(0,0)`. So `live_end <= capacity` always and the extra
guard never fires on a reachable buffer; on a (unreachable) corrupted state it
converts a would-be slice-index panic into `InvalidBounds` -- strictly safer.
**THAT is why NO DEFECT -- not "it's just a refactor."** The refactor is
behavior-preserving on the reachable domain because a *producer* upholds the
invariant the extracted guard assumes, which is a fact you have to go find, not
one the diff shows.

## The method

1. **Diff the extracted helper's guard set against EACH pre-existing caller's
   old inline check** -- not just confirm the NEW caller works. Any guard the
   helper carries that a pre-existing caller lacked inline is a candidate
   precondition-strengthening at that caller.
2. **For each extra guard, prove the pre-existing caller's producer cannot
   violate the invariant it assumes**, on the reachable domain. If the producer
   maintains it, the guard is inert (no regression). If it cannot be shown, the
   factor-out rejects formerly-valid inputs -- a real behavior change hiding in a
   "cleanup" diff. Ground it at the sole / every producer.
3. This is the DUAL of
   [[an-elaborator-acceptance-widening-clears-by-construction-and-the-payoff-is-sweeping-every-consumer]]:
   that one is a capability WIDENING inside a shared callee reaching every call
   site; this one is an EXTRACTION that strengthens one pre-existing caller's
   precondition to serve a new caller. Same root -- a shared callee's behavior
   is not the union of what each caller separately needed -- opposite direction.
   Sibling of
   [[a-control-over-an-or-route-needs-a-row-where-each-disjunct-decides]] (a
   share-the-logic tidy-up silently killing a per-caller property).
