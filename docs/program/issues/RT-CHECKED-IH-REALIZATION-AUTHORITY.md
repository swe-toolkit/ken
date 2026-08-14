---
id: RT-CHECKED-IH-REALIZATION-AUTHORITY
title: "Mint the checked-IH realization authority -- pending marker, oriented plan, call template, slot and parent -- so the ComputationalRecursorClosure capsule is realizable IN PLACE at the source-machine Match seat, without widening the ordinary-Match selector and without any terminal-All licensing"
status: ready
owner: runtime
size: M
gate: none
depends_on: []
blocks: []
github: null
origin: "Architect mechanism ruling evt_7mgzv23cnjm0a (2026-08-14), answering the Steward's question at evt_1469rndt5745r. The ruling is that checked-IH REALIZATION authority and RT-TERMINAL-ALL-ELIM-AUTHORITY's terminal-All ELIMINATION relation are TWO mechanisms, so this successor carries no KERNEL-NESTED-IND dependency and is framable now. Steward-filed (agents cannot create tracked work per COORDINATION §2). Every structural fact below re-verified by the Steward against main fea9cd96 before filing."
---

> # HELD BEHIND THE OPERATOR'S PRIORITY RULING, 2026-08-14. `ready`, NOT next.
>
> **Do not release this until [[RT-LEXICAL-RECURSOR-CONSUMERS]] lands.** The
> operator ruled the `RecursiveDescent` retirement Runtime's priority — *"that
> is the priority for the runtime team. prioritize that work over other runtime
> work."* That node is the single unblocked node on the retirement chain.
>
> **This node is unchanged, unblocked, and correct** — nothing here is deferred
> on its merits, and no dependency was added. It is `ready` and waiting on a
> priority call, which is the operator's under `ken-steward` §3. **A reader
> finding it idle should not diagnose framing debt.**

## What this is

**The planner capability that `RT-NESTED-IH-NATIVE-REALIZATION` `D2` stopped
at.** `D2` merged as `f7ec9f59` and is terminal at a **measured** stop, not an
unfinished one: the realization route is unreachable because the authority it
consumes is never minted.

**This node mints that authority. It does not touch the selector.**

## Why this is a node and not a slice of the parent

**The constraint is an Architect ruling, cited: `evt_7mgzv23cnjm0a`.** It says in
terms: *"two mechanisms; frame the realization successor now, against the cut
below; leave `RT-TERMINAL-ALL-ELIM-AUTHORITY` held on Kernel exactly as it
is."*

**It is not a duplicate of the parent's remaining deliverables.** The parent's
`D3`-`D5` are **outcomes** — native execution, interpreter agreement at `Nat 3`,
the carried control no longer ignored. **This node is the capability those
outcomes require**, and it is new machinery on the erasure/planning side rather
than another observation at the seam. The parent stays `active`; its `D3`-`D5`
are gated on this node landing.

**It is not `RT-TERMINAL-ALL-ELIM-AUTHORITY`, and the fold was refuted on three
independent grounds** — see below. Do not re-open that question.

## Fixed inputs, measured at `main` `fea9cd96`

**The stop, exactly.** `mint_checked_computational_ih_instance`
(`lowering/mod.rs:16888-16893`):

```rust
let Some(pending) = self.pending_computational_ih_call.take() else {
    return Ok(None);
};
```

⇒ It `.take()`s `pending_computational_ih_call` **first** and returns `Ok(None)`
when absent. **Everything downstream is unreachable**, including the
`oriented_subcontinuation_plan` lookup two statements later.

**The consuming route**, `core.rs:10521`:

```rust
mut recursor @ Lowered::ComputationalRecursorClosure { .. } => {
    let checked_ih_invocation =
        self.mint_checked_computational_ih_instance(&mut recursor)?;
    if let Some(CheckedRecursiveInvocationInstance {
        source: InvocationTemplateRef::ComputationalIHCall(call_template_id), ..
    }) = checked_ih_invocation
```

**What is absent at this seam, measured by `D2`:** no pending marker, no
oriented plan, no call template, no slot, no parent. The activation itself is
live — `ContinuationActivationId(0)`, at its resume cursor.

**The producer region this node extends**, `ken-elaborator/src/erasure.rs:1435-1510`,
building from `pending_computational_ih_slots` / `pending_computational_ih_calls`.
Fields confirmed present in that region: `slot_template_id`,
`checked_match_ordinal`, `frame_template_id`, `recursive_position`,
`ih_interface`, `segment_site_id`.

## THE CUT, stated so this node cannot drift into being the other one

The two authorities are adjacent, and a successor that overreaches becomes
**unlicensed consumption**. The Architect stated the boundary; it is reproduced
here because this is the file the implementer reads.

- **This node mints authority for the checked-IH invocation route** —
  `core.rs:10521` → `mint_checked_computational_ih_instance` →
  `OrientedSubcontinuationPlanV1` → `computational_ih_call`. Its job is to make
  the capsule **realizable in place**, so that a **realized constructor value**
  reaches ordinary-`Match` selection.
- **It does NOT license the ordinary-`Match` selector to accept a capsule.** A
  `ComputationalRecursorClosure` is a **suspended activation, not a constructor
  value**, and no widening of that selector is lawful. **Realize before
  selection; never accept at selection.**
- **`RT-TERMINAL-ALL-ELIM-AUTHORITY` keeps the licensing question** — *may this
  seat consume this capsule* — answerable only from kernel provenance. It stays
  `draft`, held on `KERNEL-NESTED-IND`, untouched by this node.

**Held that way the scopes are disjoint, the selector wall is untouched by both,
and this node can be built and measured WHILE THE SEAT STILL REFUSES** — exactly
as `D2` was. That is why it does not need Kernel.

## Why the fold was refuted — recorded so it is not re-litigated

**1. Different producers, measured rather than compared.** The realization
authority is built in checked erasure from the pending-IH slots and calls. That
entire producer region was grepped for `all_support*`, `terminal_support`,
`support_family`: **nothing.** It never reads the terminal-support relation at
any point in its construction. Elimination's facts 1-2 are **kernel-issued**.
⇒ **Different relations over different subject matter**, neither recoverable
from the other: realization is a *call-graph and continuation* relation;
elimination is a *datatype-provenance* relation.

**2. The other node's own text excludes the fold.**
`RT-TERMINAL-ALL-ELIM-AUTHORITY` `D2` says it routes *"through the existing
checked invocation/decomposition machinery. Nothing here authorizes new
machinery."* **That presupposes the invocation machinery is populated at this
seam, and `D2` measured that it is not.** Supplying it is precisely *new
machinery*. ⇒ Folded, that node would be **required to build the thing its own
text forbids it from building**, and would be both its own successor and its own
blocker.

**3. The fold would manufacture a cycle the tracker does not have.** Realization
would then depend on `KERNEL-NESTED-IND`, which is blocked at `AC-K12` **on this
very arc** — the work that unblocks the arc would sit behind the arc.
`KERNEL-NESTED-IND`'s `D5` Runtime-consumability requirement is a later
**acceptance condition**, not a reverse implementation dependency, and it
forbids adding a reverse edge. **A fold that turns a well-founded dependency
into a cycle is refuted by that fact alone.**

## Deliverables

**`D1` — mint the pending marker and the oriented plan at this seam.** The
producer must populate `pending_computational_ih_call` and an
`OrientedSubcontinuationPlanV1` carrying the `computational_ih_call` for this
activation, so `mint_checked_computational_ih_instance` returns `Some` rather
than `Ok(None)`. **Report the minted fields**, not merely that the function
stopped returning `None`.

**`D2` — the call template, slot, and parent.** `slot_template_id`,
`call_template_id`, and `parent_frame_template_id` resolved for this seam.
`D2` of the parent measured all three absent; **report each as present and where
it came from.**

**`D3` — the refusal ADVANCES, and that is the deliverable.** This node does not
have to make the seat succeed. **The correct outcome is that the stop moves
past the authority chain to whatever refuses next**, and that the new refusal is
named. An advancing refusal is the evidence the increment worked — the parent's
`D1` sentinel arc is the precedent, and it is why the sentinel there had to
change class rather than be restated. **If it advances to a refusal nobody
predicted, stop and report; that is a finding, not a failure.**

**`D4` — the positional ABI pin, which needs no authority and must land here.**
A `#[cfg(test)]` observation that the first `args.len()` entries of `inputs` are
the matched fields, **in source order**. The two halves have *different* lengths
at this seam (1 and 2), so a **positional** assertion discriminates where a
length check would not. Same shape as the `d2f` / `dasm_c2` observers already in
this file.

**`D5` — the seat-discriminating assertion.** The stop control must name **which
refusal site fired**, not merely that some `Match` seat refused a non-constructor
scrutinee.

## The emitter census `D5` rests on — CORRECTED, and the correction matters

**The Architect's ruling names three production emitters of the exact pair
`("Match", "scrutinee is not a constructor value")`, at `core.rs:17719`,
`core.rs:8407` and `core.rs:8424`. Re-measured at `fea9cd96`, `core.rs:8407` is
inside `#[cfg(test)]`** — it is an armed-mutation harness
(`source_carried_control_refusal(SourceCarriedControlMutation::RefuseClassifiedCarried, …)`),
not a production site.

**The corrected census:**

| site | kind | arm |
|---|---|---|
| `core.rs:17719` | production | the ordinary-`Match` selector's `let-else` |
| `core.rs:8424` | production | source-machine `Match`-scrutinee, `LoweringOperand::Specialized(_)` |
| `runtime_ir_evaluator.rs:1292` | production | the interpreter |
| `core.rs:8407` | **`#[cfg(test)]`** | armed-mutation harness only |

**The finding is unweakened and arguably sharper.** The two *production* core
sites emit a byte-identical pair, and **`:8424`'s arm is
`LoweringOperand::Specialized(_)`, which a `ComputationalRecursorClosure`
matches.** So the ambiguity the Architect identified is real between exactly the
two sites where it does most damage. **Under `cfg(test)` — which is where the
stop control runs — a third site can emit the same pair when its mutation is
armed.**

⇒ **The `D2` stop control today pins *"some `Match` seat refused a
non-constructor scrutinee"*, not *"the ordinary-`Match` selector refused for
absent checked-IH authority."* It is not wrong — the refusal *moving*
`Closure` → `Match` is real evidence the arm fired — but it is **less precise
than its name**, and precision is exactly what it needs the moment this node
moves the stop. **A control that cannot distinguish the emitters cannot tell you
the stop moved.**

## Acceptance criteria

**`AC-1` — the advancing refusal is REPORTED VERBATIM**, with the emitting site
identified. `D3` is discharged by naming the new refusal and its site, not by
asserting the old one is gone.

**`AC-2` — `D4`'s pin is fired, not merely written.** Transpose the order,
confirm the pin **reds**, restore, confirm green. **Report the failing text.**
The Adversary measured that a transposition is currently free across all three
suites (`evt_54sb0z31q5qhn`); this AC is the proof that is no longer true. A pin
added without being fired is not known to be a pin.

**`AC-3` — `D5` discriminates between `core.rs:17719` and `core.rs:8424`
specifically.** A control that fires on both is the defect, not the fix.
**Demonstrate it distinguishes them**, do not argue that it does.

**`AC-4` — the ordinary-`Match` selector is UNCHANGED.** Diff it and say so.
This is the acceptance criterion most likely to be violated by a change that
looks like progress: widening the selector would make the seat succeed and would
be **unlicensed consumption**. Realize before selection; never accept at
selection.

**`AC-5` — no `KERNEL-NESTED-IND` dependency is introduced**, in the node graph
or in the code. If the work appears to need terminal-`All` provenance, **stop
and report** — that is the fold being refuted a fourth time, and it is a finding
for the Architect, not something to route around.

**`AC-6` — no-regression, in CI.** `COORDINATION §12` — the venue is CI, never a
local `--workspace` run. Build targeted, `-p ken-runtime` and
`-p ken-elaborator`.

## Sizing and the hard stop

**`M`.** `D4` and `D5` are `cfg(test)` observations and are small; `D1`/`D2` are
the planner capability and are the real content. **`D4` and `D5` need no
authority and can land even if `D1`/`D2` hard-stop** — if the capability turns
out larger than `M`, **land `D4`+`D5` as an accepted partial and report the
`D1`/`D2` stop**, which is the shape this arc has used twice and which
`merge-policy.md` licenses.

**Hard stop and report** rather than pressing on if minting the authority
requires an **unchecked** plan. Every step in that function is a `checked` gate,
and short-circuiting one converts a fail-closed refusal into an **unverified
realization**.

## Forbidden — carried forward unchanged from the parent and the ruling

- **No widening of the ordinary-`Match` selector** (`AC-4`).
- **No minting of an *unchecked* plan** to satisfy the chain.
- No `.residual` inspection or unwrapping (`mod.rs:3327-3331`).
- No ordinary-`Match` selector arm for the capsule, and no catch-all change.
- No seventh admitted scalar merge shape.
- No new `LoweringOperand` / `Lowered` variant; no `Carried → Lowered`.
- No durable closure lane; no body-carrying field.
- **No edit to `RT-TERMINAL-ALL-ELIM-AUTHORITY`'s scope**, which stays `draft`
  and held on Kernel.
