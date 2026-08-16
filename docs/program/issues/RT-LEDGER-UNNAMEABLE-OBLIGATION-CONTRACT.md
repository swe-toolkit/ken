---
id: RT-LEDGER-UNNAMEABLE-OBLIGATION-CONTRACT
title: "Decide how the static-worker ledger should treat an obligation the emitter can provably never name, given that rebind is the transition and every recognition must have transitioned"
status: active
owner: runtime
size: M
gate: none
depends_on: [RT-OVERCONSTRUCTED-OUTER-RECOGNITION]
blocks: []
github: null
origin: "Steward, 2026-08-16, on RT-OVERCONSTRUCTED-OUTER-RECOGNITION D1 reaching its pre-authorized CONTRACT CHANGES hard stop at evt_41wvqft0m091r, measured at b1b30c1c7. Carries the Architect's at-or-before-construction ruling from evt_6aarzqdm18vnh. TCB-adjacent: filed as its own node rather than absorbed into a deliverable sized as bookkeeping. Steward-filed per COORDINATION section 2."
---

## `D0` DELIVERED. Shape (iv) SELECTED, and `D1`/`D2` ARE RELEASED.

**`D0` came back at `1a4a1f723` with no candidate** (`evt_37p25sg8v56nx`):
**(i), (ii) and (iii) BARRED, (iv) LAWFUL.** The Architect concurred with the
three bars without qualification and **released (iv) conditioned on one thing
`D1` must establish** (`evt_1njg9qsfa3kak`).

| shape | disposition | the reason, in one line |
|---|---|---|
| (i) conditional transition | **BARRED** | loses `dom(transitioned) = dom(recognized)`; the omitted recognition is the forbidden fourth state reached **by subtraction rather than by erasure** |
| (ii) payload + `consumed` write point | **BARRED as repair-capable** | moving the write inward preserves the meaning and **still cannot disposition a binding no call reaches**; every repair-capable form collapses into a second meaning or into (i) |
| (iii) do not recognize the outer field | **BARRED** | EMITTER PROPERTY adds **no mint-time fact**; emission is downstream of the mint, so which transport a call NAMES supplies no rule for which recognition to SKIP |
| **(iv) do not CONSTRUCT the outer binding** | **LAWFUL, selected** | it disposes of nothing, so *"positive authority at or before construction"* is **never engaged** and `AC-3` cannot be violated — **there is no write to make** |

> ### WHY (iv) IS THE RIGHT ANSWER AND NOT MERELY THE SURVIVING ONE
>
> **Of the five dispositions this campaign considered, (iv) is the only one
> under which the red goes away because the POPULATION changes rather than the
> CHECK.** Today's law — every transitioned obligation consumed, no exemption —
> keeps exactly its present strength, **and it is the law that caught this.**
> The Architect refused to weaken it three times; **(iv) is the disposition that
> never asks.** Architect, `evt_1njg9qsfa3kak`.

**(iv) was the shape the frame called cheapest if it holds and likeliest to be
wrong, and it was answered from the lowering rather than from the ledger, as
`D0` required.** The `45/35/25` distinctness objection does not reach it: **the
outer recognition never exists**, so nothing is transferred between distinct
recognitions.

## The defect is real and it is not cosmetic

**Under `(A) over-construction`, valid programs at depth 2 and 3 do not
compile.** `D1d` established this by suppressing the refusal **only inside a
disposable probe** and observing correct execution and exit `0`. **With the
refusal in place, `close` blocks the compile.**

⇒ **`close` is refusing correctly on a program that is correct.** That is the
constraint this node exists to remove, and it is grounded in the build
producing a wrong outcome rather than in a preference for a tidy ledger.

## What is already settled. Do not reopen any of it.

| result | node | disposition |
|---|---|---|
| `(A) over-construction` | `RT-SECOND-RECOGNITION-ERASURE` `D1d` | (B) and (C) both excluded, positive control discriminating |
| **EMITTER PROPERTY** | `RT-OVERCONSTRUCTED-OUTER-RECOGNITION` `D0` | an emitted call can name only the transport on the binding being lowered; **structural, not a two-row coincidence** |
| `transfer` | `D1c` | **REFUTED** — the rebinds are distinct recognitions over distinct source fields |
| **erasure after construction** | Architect `evt_6aarzqdm18vnh` | **BARRED.** Lawful only under positive authority **at or before** construction |
| no mint-time discriminator | `RT-MINT-SITE-STATIC-DISCRIMINATOR` `D0` | the static plan exports no total mint-to-reader relation |

## Why the obvious repair is unavailable, stated so it is not re-attempted

**`rebind` IS the transition.** It mints the transport **and** writes both
`minted` and `transitioned` when the field enters lexical binding authority
(`lowering/mod.rs:4546-4603`, performed at `:4936-4972`), and the transport it
carries is part of the binding contract (`:3731-3753`). Consumption is an
exact-`Var` call discharging that already-minted transport (`:4629-4671`).
**Every recognition must have transitioned**, and the agreeing
`transitioned`/`minted` bijection depends on it (`:4721-4744`).

⇒ **Deferring the mint to the emitter would let a recognized field enter
binding authority with neither state.** Supporting that changes link one and the
bijection; avoiding it changes what the binding carries and when `consumed` is
written. **Either way the ledger's states, invariants, or meanings move** —
which is why this is its own node and not an enlarged `D1`.

> ### THE EXEMPTION WILL ARRIVE AS A WRITE, NOT AS A RELAXED CHECK
>
> **`consumed` is `BTreeMap<StaticWorkerTransportId, StaticOriginId>` and its
> value is the CONSUMING ORIGIN.** At a supersession there is no consuming
> origin, **so there is no honest value to write.** Inventing one makes
> `consumed` mean two things and collides with the double-consumption refusal.
>
> **A guard you refused to relax can be defeated by writing a dishonest value
> into the structure it reads.** The check is untouched and still passes.
> Architect, `evt_6aarzqdm18vnh`.

## `D0` — DELIVERED. The classification it asked for, kept as the record.

**Four shapes are on the table. `D0` says which are lawful, and it is a read,
not a build.** Report each as LAWFUL / BARRED / UNKNOWN with the warrant.

**(i) The transition becomes conditional.** A recognition may be recognized
without transitioning, so link one and the bijection weaken. **State exactly
which invariant is lost and what still forbids the fourth state** —
constructed, neither consumed nor authoritatively erased, then forgotten.

**(ii) The binding's payload and `consumed`'s write point move.** State what
`consumed`'s value becomes and whether it still means one thing.

**(iii) Do not recognize the outer field at all.** **This needs the mint-time
discriminator `RT-MINT-SITE-STATIC-DISCRIMINATOR` `D0` said does not exist** —
so it is available only if the EMITTER PROPERTY supplies something that read
did not have. **Say whether it does. Do not assume it does because it is
newer.**

**(iv) Do not CONSTRUCT the outer binding.** The over-construction is in the
**lowering**, not the ledger: the nested composed lowering builds the outer
constructor field bindings and the inner lowering supersedes them. **If the
outer construction is unnecessary, nothing is minted and no ledger change is
needed at all** — and *"positive authority at or before construction"* is
satisfied trivially, because there is no construction.

> **(iv) is the shape nobody has read yet and it is the cheapest if it holds.**
> **It is also the one most likely to be wrong**, because `D1c` showed the
> nested rebinds map **distinct** recognitions over **distinct** source fields
> (45/35/25) — they are not redundant copies of one field. **Whether the outer
> construction is nonetheless unnecessary is an open question about the
> lowering, and `D0` must answer it from the lowering rather than from the
> ledger.** Do not report (iv) LAWFUL on the strength of "it would be nice."

**Hand `D0` back on its own.** The Steward releases the build on the answer,
and the Architect rules any shape reported UNKNOWN.

## `D1` — build shape (iv). THE WARRANT IS THE DELIVERABLE, NOT THE DIFF.

**`D1` builds (iv): consume the immediate constructor/eliminator pair directly,
preserving the field's source position and case-binder arity, and never
construct the intermediate `ConstructorField::StaticWorker`.** The innermost
layer — whose binding is the one actually lowered and called — is unchanged.

> ### THE CONDITION. `D0`'s OWN PARAGRAPH CARRIES TWO SCOPES AND THEY DISAGREE.
>
> *"a recursive case **is forced** through `lower_source_machine` with non-empty
> suffix"* is universally quantified and structural. **"LAWFUL on the MEASURED
> lowering sequence"** is scoped to depth 2 and 3. **If the second is the true
> scope, (iv) is row-fitting** — the exact defect
> `RT-MINT-SITE-STATIC-DISCRIMINATOR` `AC-1` banned and whose `D0` this campaign
> correctly refused to commit: *"origin constants would fit rows, not state
> law."*
>
> **A not-construct that holds only on the measured sequences changes behaviour
> on the first unmeasured shape reaching that arm, in the direction where a
> needed field is never constructed. That is a MISCOMPILE, not a ledger
> complaint** — introduced by a repair whose stated purpose is to remove a
> bookkeeping red. Architect, `evt_1njg9qsfa3kak`.

**Establish both legs from the DISPATCH, not from rows:**

**Leg 1 — the forcing is TOTAL.** Every route by which a nested composed
recursive case reaches the `Construct` arm installs `Terminal::ResumeOuter` with
the exact pending suffix. **If any route reaches that arm without it, the
intermediate construction is load-bearing on that route and (iv) is FALSE
there.**

> **`ResumeOuter` refusing on `active.cursor != expected` (`core.rs:7905-7924`)
> is evidence the invariant is CHECKED, not evidence that every route INSTALLS
> it. Those are different facts** and only the second is leg 1.

**Leg 2 — the ownership PRECEDES the construction on every such route.** *"The
immediate constructor/eliminator pair is already owned before the intermediate
field would be constructed"* is an **ordering claim**, and it must hold by
construction of the dispatch rather than by observation at two depths.

**Do NOT re-establish that the not-constructed fields are unread.** `D1d`
measured non-traversal on two rows, **but the EMITTER PROPERTY already makes it
general** — an emitted call has no source from which to name a transport other
than the one on the binding being lowered. **That leg is structural; spend `D1`
on legs 1 and 2, which are not.**

**If leg 1 or leg 2 is FALSE — if some route reaches that arm without the
forcing — STOP with no candidate and hand back.** That is the same call this
ring already made once tonight, and it was the right one.

## `D2` — the control. THE SECOND DIRECTION IS NOW THE PRIMARY ONE.

**Two directions, mutation-proven, not argued.**

1. **A mutation leaving an outer recognition CONSTRUCTED must red.** **Under
   (iv) the claim is *never constructed*, so this is the direction that tests
   the actual claim.** It was the droppable one on the predecessor; here it is
   **primary**.
2. **A mutation suppressing a construction whose transport IS named by an
   emitted call must red.** This is the miscompile direction — it is what
   catches leg 1 being false on an unmeasured route.

> **A green "the leak is gone" tests the direction (iv) makes trivially
> true.** It is not evidence for either of the above.

## Acceptance criteria

**`AC-1`.** **The fourth state stays impossible.** Constructed, neither
consumed nor authoritatively erased, then forgotten. **Whatever moves, this
does not** — it is what `StaticWorkerFieldLedger` exists to prevent.

**`AC-2`.** **No erasure on authority acquired after construction**, under any
name. **The test is the Architect's:** does the disposition come from authority
at or before construction, or is the ledger **told** after the fact? **If the
ledger learns it from something the lowering asserts later, it is barred.**

**`AC-3`.** **`consumed` means one thing.** If `D1` writes it, the value is a
real consuming origin. **A sentinel, a reserved id, or an `Option` widening to
represent "superseded" fails this** — that is the exemption arriving as a write.

**`AC-4`. DISCHARGED by `D0`.** It reported UNKNOWN nowhere and warranted each
of the four dispositions from source. Kept for the record, not live.

> ### `AC-1`-`AC-3` ARE VACUOUS UNDER (iv). DO NOT HUNT FOR A WRITE.
>
> **(iv) constructs nothing, so there is no transport to disposition, no
> erasure to authorize and no `consumed` value to write.** QA should confirm
> **that no such write was added**, not look for one that means the right
> thing. **If a candidate for (iv) touches `consumed` at all, that is the
> finding** — it means the build drifted into (ii).

**`AC-7`. Leg 1 is warranted from the DISPATCH.** The claim *"every route by
which a nested composed recursive case reaches the `Construct` arm installs
`Terminal::ResumeOuter` with the exact pending suffix"* is established by
enumerating the arm's routes, **not by observing depth 2 and depth 3.** **A
warrant that holds on the measured sequences and names no general property
fails this AC**, and the frame prefers a hard stop to a candidate that has it.

**`AC-8`. Leg 2's ordering holds by construction.** *"Ownership precedes
construction"* is warranted from the dispatch on every route reaching that arm.

**`AC-9`. `D2`'s PRIMARY direction is demonstrated.** A mutation leaving an
outer recognition constructed must red. **A control that only shows the leak
gone has not tested the claim (iv) actually makes.**

**`AC-5`.** **The `D2k` controls still pass**, and row4-depth-1 and
row5-after-hole are behaviourally unchanged.

**`AC-6`.** No-regression, in CI (`COORDINATION §12`).

## Banned scope

- **`transfer`.** Refuted by `D1c`; not a fallback.
- **Voiding at supersession** (shape 1 of the predecessor). Barred by
  *"positive authority at or before construction"*, not merely by an AC.
- **Relaxing `close` or adding a second writer of `consumed`.**
- **Extending the static plan's exports.** Still unscoped by anyone.
- **Changing a producer so the ledger balances.**
- **A row-fitted warrant for (iv).** A not-construct justified by the sequences
  measured at depth 2 and 3, with no general property of the dispatch behind
  it. **That is the mint-site node's banned defect arriving one level down, and
  its failure direction is a miscompile.**

## Sequencing

**Lane 1 (operator priority). `D0` is releasable immediately** — the
predecessor's `D0` and `D1` are complete, and this node needs nothing further
from it.

> **The predecessor closes without a candidate.** Its `D0` returned EMITTER
> PROPERTY and its `D1` reached a lawful pre-authorized hard stop, so it can
> never reach `merged`. **Do not gate this node on that landing.**

**`D1`/`D2` RELEASED 2026-08-16 on the Architect's `D0` ruling**
(`evt_1njg9qsfa3kak`). **The condition on (iv) is an acceptance criterion
(`AC-7`), not a hold** — it says what the deliverable must warrant, not that the
ring must wait.

**TCB-adjacent: the Architect reviews the candidate.** A ledger-contract change
absorbed into a deliverable sized as bookkeeping is how a small repair becomes
an unreviewed structural one. **Under (iv) the ledger's contract does not move
at all**, which is why the build was releasable on a ruling rather than on a
re-scope.
