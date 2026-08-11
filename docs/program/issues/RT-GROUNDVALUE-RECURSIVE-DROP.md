---
id: RT-GROUNDVALUE-RECURSIVE-DROP
title: "`RuntimeGroundValue` is a recursive type, so a decoder that is carefully iterative still cannot honour \"deep valid data uses no recursive host stack\" end to end -- a 50,000-deep aggregate decodes without a host frame and then overflows the stack in `drop`, which means the bound belongs to the value type and not to any decoder that returns it"
status: draft
owner: runtime
size: unknown
gate: none
depends_on: []
blocks: []
github: null
origin: Surfaced by runtime-implementer at evt_2tbfhha1tyerh, 2026-08-11, while discharging RT-FNUNIT-RESULT-TOKEN D3 under the Architect's ruling evt_78ynjwzj0gpa8. Filed by the Steward rather than left in the thread, because that node merges and its thread closes. Not measured further by the Steward; the numbers below are the implementer's.
---

## The finding

The Architect's `D3` ruling required that **deep valid data use no recursive
host stack**. The implementer satisfied that for the decoder: traversal of
invocation aggregates is iterative and postorder with grey/black state, and a
**50,000-deep chain descends the whole way and refuses at an unissued bottom,
consuming arena nodes and no host frames.**

**The property still cannot hold end to end, and the reason is not the
decoder.** `RuntimeGroundValue` is itself a recursive type. A 50,000-deep
aggregate that *succeeds* decodes to a 50,000-deep value, and that value's own
`drop` recurses. The implementer's first version of the control built one and
aborted:

```
fatal runtime error: stack overflow
```

**in `drop`, not in `decode_invocation_ground`.**

⇒ The successful-depth control in `RT-FNUNIT-RESULT-TOKEN` is deliberately
modest and states whose bound it respects. **That is an accurate control, not a
weak one** — but it means the ruling's property is satisfied by the traversal
and **unsatisfiable end-to-end while the result type is recursive.**

## Why this is filed separately

**It is a property of `RuntimeGroundValue`, not of the decoder**, and the
implementer explicitly did not touch it. Any future decoder, boundary path, or
observation route that returns a `RuntimeGroundValue` inherits the same ceiling,
so fixing it inside one consumer would be the wrong layer.

**And it was surfaced in a thread that closes.** `RT-FNUNIT-RESULT-TOKEN`
merges and its thread takes no further posts; a finding left there is
unreachable by anyone who does not already know it exists.

## What is NOT established

- **Whether this matters in practice.** Nobody has shown that a legitimate
  program produces a ground value deep enough to overflow on drop. The observed
  overflow was a control constructed to probe the traversal, not a workload.
- **What the fix would be, or whether there should be one.** An iterative
  `Drop`, a depth cap with a refusal, or a non-recursive representation are all
  shapes someone could propose; none has been costed and this node does not
  prefer one.
- **Whether other recursive runtime types have the same ceiling.** Not swept.

**Do not size or schedule this from the prose above.** The first work is
deciding whether the ceiling is reachable by a real program — if it is not,
recording that answer is the whole node.
