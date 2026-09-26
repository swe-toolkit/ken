---
name: deleting-a-test-row-leaves-its-fixture-programs-in-the-tree
description: A recut deleted four test rows and the record said the measurement "survives only on a preserved branch" — but the four governed programs were still in the fixture's source string, so the census that decides reachability is Rust-level references, not the identifier's presence; and the coverage gap it left could only be seen by taking a cross product of two identifiers, because a window is degenerate only relative to its buffer's capacity
metadata:
  type: feedback
---

# Deleting a test row leaves its fixture programs in the tree

**Measured 2026-08-17 on `f9dd79f52` (NATIVE-HANDLE-CARRIER).** A recut deleted
four `#[test]` rows plus one exclusive helper. The merge record then said the
measurement *"survives only as node prose plus
`origin/preserved/…-route1-3d23f118`."*

**It did not.** The four governed *programs* were still in the tree, in the
fixture's Ken source string (`rt_parity_native.rs:349-387`), together with their
`_file` and `_stage` wrappers -- 126 lines that still type-check on every
compile and that no Rust code invokes. What the recut removed was the drivers.

⇒ ***Restoring the measurement was four test fns in that file, not a cherry-pick
from a preserved branch.*** The record overstated the cost by a whole
integration step, and the correction is worth more than the finding it came
with.

## THE CENSUS THAT DECIDES THIS

`grep cap41 crates/` returns plenty of hits and every one of them is **inside
the fixture's `RT_PARITY_SOURCE` literal**. The reachability question is
answered by *where the hits sit*, not by whether they exist:

```
grep -rn "cap41" --include=*.rs crates/     # hits, all lines 58-481
grep -n "RT_PARITY_SOURCE\|^\"#;" …          # the literal spans 58-481
```

⇒ **Bound the string literal first, then ask whether any hit is outside it.** A
bare identifier count cannot tell a live driver from dead fixture text, and the
name being present is exactly what makes it *look* covered.

## "REMAINS LIVE" HAS TWO SENSES AND A REVIEW CAN PAIR THEM

The review inventory said two helpers *"remain live"*. One was row-reachable
through a live stage; the other was referenced **only by the four dead procs**.
Both still compile. Only one still runs. **Name which sense you mean, because
compile-liveness is what a deletion audit naturally measures and row-liveness is
what coverage means.**

## THE POPULATION WAS A PAIR, SO A ONE-IDENTIFIER GREP COULD NOT DECIDE IT

The gap the deletion left was that two new admission branches (a tail cap, and a
degenerate window answering `ReadEof`) had no executing witness. **A window is
not degenerate on its own -- only relative to its buffer's capacity.** So the
population is the cross product `MkBufferWindow` x its `withBuffer` capacity,
resolved per call site:

```
live pairs: (0,1)@1  (0,2)@8  (0,6)@6  (0,6)@8  (0,8)@8  (2,4)@8  (-1,1)@1
```

Every one fits exactly or sits strictly inside ⇒ no live program reaches either
branch. **One capacity-2 handle looked like a live endpoint case and was not --
that buffer is never read.** ⇒ ***When membership is a relation between two
values, grep both and join them at the call site; a grep over either alone
returns a population that is not the one you meant*** -- same family as
[[anchor-a-claim-census-to-position-and-validate-it-against-a-reference-count]]
and [[a-type-check-cannot-distinguish-two-fields-that-share-a-type]].

## AND SAY WHAT THE COVERAGE IS *NOT* BLOCKED BY

The dropped rows were expensive because their harness compiles a native
artifact, and every sibling row in that fixture refuses at object emission under
a known node. **But both uncovered branches return without ever visiting the
host**, so an interpreter-only row exercises them today. **A neighbour's ceiling
is not automatically yours -- check whether the uncovered path even reaches the
wall before you report the coverage as blocked.**
