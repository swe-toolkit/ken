---
id: RT-GROUNDVALUE-RECURSIVE-DROP
title: "`RuntimeGroundValue` is a recursive type, so a decoder that is carefully iterative still cannot honour \"deep valid data uses no recursive host stack\" end to end -- a deeply nested value overflows the stack in its own `drop`, reproducible without the decoder, and the depth at which that happens is UNMEASURED: the two numbers in the source report are an observed abort and a deliberately-safe control, not a bisected threshold"
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

## NEITHER NUMBER ABOVE IS A THRESHOLD

**Corrected by runtime-implementer at `evt_2etrw0xrbv8yq`, before this node
could inherit them as sized.**

- **50,000 is where the overflow was observed. It is not a measured limit** —
  it was the first number picked, and it happened to be past the edge.
- **64 is the successful depth in the committed control. It is not a measured
  ceiling either** — it was chosen to be obviously safe after the abort, with
  no approach to the edge.

**No bisect was run, so the depth at which a successful decode overflows in
`drop` is UNKNOWN.** Nothing in the evidence distinguishes a ceiling near 100
from one near 40,000.

## The first work is a bisect, and it decides whether this node is real

**That unknown is the whole question, and my earlier framing of it was wrong.**
This node previously said the issue was whether "a legitimate program reaches
the ceiling", which quietly assumed the ceiling sits somewhere near 50,000. **It
is not a question about how deep programs get — it is a question about where the
ceiling is.** If it turns out to be a few hundred, the node changes character
entirely.

**A bisect is one cheap run against the value type.** It needs no decoder
involvement, and it should be the first thing done here.

## Two facts recorded so they are not re-derived

- **The overflow is reproducible WITHOUT the decoder.** Construct a deeply
  nested `RuntimeGroundValue` directly and let it fall out of scope; the
  recursion is in its `drop`.
- **`decode_invocation_ground`'s own cost is worse than the type's, and that is
  the decoder's shape rather than the type's.** Each level clones its child's
  decoded value into the parent while the child stays in the postorder map for
  sharing, so a chain costs **O(n²) in data** as well as O(n) in drop depth.
  Fair to fix at that decoder **if** this node ever wants the depth raised —
  and not otherwise.

## Also not established

- **What the fix would be, or whether there should be one.** An iterative
  `Drop`, a depth cap with a refusal, or a non-recursive representation are all
  shapes someone could propose; none has been costed and this node prefers
  none.
- **Whether other recursive runtime types have the same ceiling.** Not swept.

**Do not size or schedule this from the prose above.** Run the bisect first;
the answer decides whether anything follows it.
