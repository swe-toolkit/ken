# RT-LEXICAL-RECURSOR-CONSUMERS — `D2a` accepted partial

Node: `docs/program/issues/RT-LEXICAL-RECURSOR-CONSUMERS.md`.
Frame: `docs/program/wp/RT-LEXICAL-RECURSOR-CONSUMERS.md`.
Authority: Architect ruling `evt_k64pjherb9x0`.
Predecessor record: `RT-LEXICAL-RECURSOR-CONSUMERS-D0-D1.md`, **historical at
the `D0`/`D1` record commit** — this file supersedes it wherever they disagree.

> # THIS IS AN ACCEPTED PARTIAL. THE NODE DOES NOT CLOSE.
>
> `D2a` repairs **`R1` only — rows 1 and 4**. It is a boundary, not a
> completion, and the rows it repairs **do not turn green**.

## 1. What `D2a` covers, exactly

| axis | disposition |
|---|---|
| **`R1`, rows 1 and 4** (five compiles) | **repaired** — the first refusal is gone |
| `R2` (row 3) | **untouched** |
| `R3` (row 5) | **untouched** |
| `R4` (row 2) | **untouched**, and out of this node entirely |
| rows 1 and 4 overall | **not green** — they advance to a successor wall, §3 |

`R2`/`R3` were held pending separate evidence whether they share a root. `R4`
left this node by ruling and is not referenced in code.

## 2. The mechanism, in one paragraph

All five `R1` compiles deliver `Specialized(RecursiveBackedge)` on the
`DirectScrutinee` route with no frame. That is a **protocol marker in a value
position**, not a non-constructor scrutinee — so the constructor guard was
right and the question was wrong. A new arm at source-machine
`SourceContinuation::ComputationalMatchScrutinee` recognises exactly that
marker, sets the continuation to `next`, and **forwards the same marker**,
carrying `incoming_route` rather than resetting it. It is taken **before**
`enter_source_occurrence_plan`, so it consumes no occurrence plan, mints no
authority, selects and dispositions no case, and constructs no value.

## 3. The successor wall — a WALL, not a completion

With `R1` gone, rows 1 and 4 advance and stop at:

```text
Unsupported(StaticWorkerBinding): "a Var in value position is a value-producing
position and a static worker binding has no value representation; its only
admissible use is as the callee of a call with an exact Var callee"
```

⛔ **This is the campaign's Trap 2 again and it belongs to a successor, not to
`D2a`.** It is recorded here so the next reader inherits the exact wall rather
than re-deriving it, and so that *"`R1` is repaired"* is never read as *"rows 1
and 4 pass"*.

**The advance is itself the evidence the repair was reached** — the refusal it
replaced is gone from all five compiles — but an advance is not a pass.

## 4. Debt this partial carries forward

| debt | owner |
|---|---|
| a **genuine non-constructor** witness refused at `ComputationalMatchScrutinee` | **`D3`** |
| `R2`/`R3` root evidence, then their repair | `D2b`, held |
| `R4` | outside this node |
| the `StaticWorkerBinding` wall | successor, unfiled here |

**On the `D3` debt, stated in the honest direction.** The committed control's
suppressed leg proves the constructor guard is still **present, reached and
refusing** on all five compiles, so `D2a` did not delete or bypass it. It does
**not** establish a genuine non-constructor refused at this seat: no existing
fixture delivers one there — the operands that reach it are constructors,
`Carried` words, and now this marker — and authoring one is new fixture work
outside a one-authority mandate. **Recorded as owed, not implied.**

## 5. What the committed control does and does not assert

**Does not key on the refusal's absence.** A repair that deleted the sentence
from production would make `!contains(…)` true for free, and this campaign has
already shipped one control with that defect.

⛔ **The assertions are not co-equal, and an earlier revision of this sentence
listed them as if they were.** `arrivals > 0` is the **denominator** and is
irreplaceable; `forwards == arrivals` is satisfied by `0 == 0`, so alone it is a
**prospective** guard about a future failure rather than evidence today; the
**suppression A/B** likewise compares refusals that only exist once something
arrived. The control now establishes the denominator **first and alone**, and
reads the equality off a value that exists only because of it. Relations
throughout; no count pinned.

## 6. Guards

`AC-3`'s five guards are intact. Neither the constructor guard nor the
boundary-transfer guard is weakened; `Trap`, `Carried`, constructors, `Nat`,
`Bool` and the ordinary specialized paths are unaltered; `carried_join_arm` and
its controls — `RT-MATCH-RECURSOR-CONSUMERS`'s — are untouched.

`AC-5` zero new `#[ignore]`. `AC-7` no tracker `status:` change.

## 7. Shared-root verdict

`R1` and `#6c`'s row 6 share a protocol **representation**, not a repair root:
`#6c`'s owner is `carried_join_arm`, which `R1` never traverses. Same marker,
distinct first missing owner and route. **No subsumption proposal is owed.**

The `D0`/`D1` record originally reached that verdict from the refusal **string**
and was false as written; it is corrected in that file's §4. The carried lesson:
a shared-root question is about the **value and its owner**, and a refusal string
renders neither.
