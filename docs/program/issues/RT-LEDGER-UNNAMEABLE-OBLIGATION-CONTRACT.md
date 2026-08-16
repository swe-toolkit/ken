---
id: RT-LEDGER-UNNAMEABLE-OBLIGATION-CONTRACT
title: "Decide how the static-worker ledger should treat an obligation the emitter can provably never name, given that rebind is the transition and every recognition must have transitioned"
status: closed
owner: runtime
size: M
gate: none
depends_on: [RT-OVERCONSTRUCTED-OUTER-RECOGNITION]
blocks: []
github: null
origin: "Steward, 2026-08-16, on RT-OVERCONSTRUCTED-OUTER-RECOGNITION D1 reaching its pre-authorized CONTRACT CHANGES hard stop at evt_41wvqft0m091r, measured at b1b30c1c7. Carries the Architect's at-or-before-construction ruling from evt_6aarzqdm18vnh. TCB-adjacent: filed as its own node rather than absorbed into a deliverable sized as bookkeeping. Steward-filed per COORDINATION section 2."
---

## CLOSED. ALL FOUR SHAPES ARE DEAD. Read this before anything below it.

**This node produced no candidate and can never reach `merged`.** Its result is
a **complete negative** one: **the classified option space is exhausted.** Do
not gate anything on its landing.

**`D1` hard-stopped at `AC-7` with no candidate** (`evt_6gs1h1x6r3xbr`,
`evt_4hp0d5r8r22a7`), and the Architect **REFUTED shape (iv) on two independent
grounds** (`evt_29ar2vfvxf414`):

**GROUND A — the route.** Leg 1 is false, measured. The same nested composed
recursive-case occurrences reach construction by **two** entries: the composed
routes through `lower_source_machine`, which installs `Terminal::ResumeOuter`
with the exact pending suffix, and **the direct `lower_expr` `Construct` arm
(`core.rs:17609-17639`), which has no `SourceControl` and calls the template
immediately.** A not-construct keyed to `ResumeOuter` removes only the
source-machine copies while the direct dispatch constructs the same fields.

**GROUND B — the mint site, and it needed no probe.**
`static_worker_constructor_template` has **exactly two call sites**
(`core.rs:7534`, `core.rs:17631`), and `mod.rs:3201` calls it **"the sole
builder of the worker arm."** Both entries ask the same classifier before
either lowers a field.

⇒ **The construction is a property of ONE SHARED CALLEE's population, not of
either caller's terminal.** So the mint-site reading of (iv) needs a mint-time
discriminator, **and `RT-MINT-SITE-STATIC-DISCRIMINATOR` `D0` already ruled
none exists.** That closes it with no forcing measurement at all.

> ### THE KEYING ERROR, RECORDED SO THE SUCCESSOR DOES NOT INHERIT IT
>
> **`AC-7` asked whether every route installs `Terminal::ResumeOuter`. That
> keys totality on a property of the CALLER, while the construction happens in
> a CALLEE SHARED BY TWO CALLERS.** A disposition keyed to one caller's
> terminal was **structurally incapable of being total.** The Architect owns
> the phrasing (`evt_29ar2vfvxf414`); the ring did not get it wrong.
>
> **And the cheap version was in the tree before this node opened.**
> `mod.rs:3130-3140` already said the arm is *"reached from both the
> direct-descent and source-machine `RuntimeExpr::Construct` arms."*
> ⇒ **When a disposition is about whether something gets BUILT, read the
> BUILDER's doc before enumerating the callers' routes.**

**The one surviving direction — re-route so the machine's terminal is the only
entry — is NOT this node's and was NOT authorized here.** It is a dispatch
change whose population is the general `RuntimeExpr::Construct` arm, far wider
than the defect. It is filed as
[[RT-NESTED-CONSTRUCT-ROUTE-EXCLUSIVITY]] with its own `D0`.

**`D2` was never authored, correctly.** No candidate, no QA route, no retained
branch.

## `D0` DELIVERED. Shape (iv) was SELECTED here, and is now REFUTED above.

**`D0` came back at `1a4a1f723` with no candidate** (`evt_37p25sg8v56nx`):
**(i), (ii) and (iii) BARRED, (iv) LAWFUL.** The Architect concurred with the
three bars without qualification and **released (iv) conditioned on one thing
`D1` must establish** (`evt_1njg9qsfa3kak`).

| shape | disposition | the reason, in one line |
|---|---|---|
| (i) conditional transition | **BARRED** | loses `dom(transitioned) = dom(recognized)`; the omitted recognition is the forbidden fourth state reached **by subtraction rather than by erasure** |
| (ii) payload + `consumed` write point | **BARRED as repair-capable** | moving the write inward preserves the meaning and **still cannot disposition a binding no call reaches**; every repair-capable form collapses into a second meaning or into (i) |
| (iii) do not recognize the outer field | **BARRED** | EMITTER PROPERTY adds **no mint-time fact**; emission is downstream of the mint, so which transport a call NAMES supplies no rule for which recognition to SKIP |
| **(iv) do not CONSTRUCT the outer binding** | **REFUTED (was LAWFUL at `D0`)** | it disposes of nothing, so *"positive authority at or before construction"* is **never engaged** and `AC-3` cannot be violated — **there is no write to make** |

> ### THE PROPERTY THAT MADE (iv) ATTRACTIVE SURVIVES ITS REFUTATION. CARRY IT.
>
> **(iv) was the only disposition considered here under which the red goes away
> because the POPULATION changes rather than the CHECK.** Today's law — every
> transitioned obligation consumed, no exemption — keeps its present strength,
> **and it is the law that caught this.** The Architect refused to weaken it
> three times (`evt_1njg9qsfa3kak`).
>
> **That property is not what failed.** (iv) failed on WHERE it was keyed — a
> shared mint site with no context, and one caller's terminal.
> **[[RT-NESTED-CONSTRUCT-ROUTE-EXCLUSIVITY]] keeps the property and changes
> the keying**, which is the whole reason it is worth a node.

**(iv) was the shape this frame called cheapest if it holds and likeliest to be
wrong. It was the latter.** The `45/35/25` distinctness objection never reached
it — but Ground B did, and Ground B was available in `mod.rs:3130-3140` from the
start.

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

## `D1` — WAS a build of (iv). RAN, HARD-STOPPED AT `AC-7`, NO CANDIDATE.

**Kept as authored because `AC-7` is the reason this node ended in a stop
rather than a miscompile. Nothing below is live work.**

**`D1` was to build (iv): consume the immediate constructor/eliminator pair
directly,
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

## `D2` — NEVER AUTHORED, correctly. `D1` stopped before it was reachable.

**Kept because its two directions carry forward to
[[RT-NESTED-CONSTRUCT-ROUTE-EXCLUSIVITY]] `AC-5` in re-keyed form.** Nothing
here is live work.

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

> ### `AC-7`-`AC-9` ARE SPENT. `AC-7` FIRED AND IS WHY THIS NODE HAS NO CANDIDATE.
>
> **`AC-7` was measured FALSE** (`evt_6gs1h1x6r3xbr`) and the ring stopped
> rather than extending the repair to the direct arm. **`AC-8` and `AC-9` were
> never reached.** The keying error in `AC-7`'s own phrasing is recorded at the
> top of this file.

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

**CLOSED 2026-08-16. Nothing here is releasable and nothing is owed.** `D0`
delivered the classification, `D1` ran and hard-stopped at `AC-7` with no
candidate, `D2` was correctly never authored.

**The successor is [[RT-NESTED-CONSTRUCT-ROUTE-EXCLUSIVITY]]**, `ready`, with
its own `D0`. **It is not gated on this node's landing, which cannot happen.**

> ### WHAT THIS NODE BOUGHT, SO THE NEGATIVE RESULT IS NOT MISREAD AS WASTE
>
> **It forecloses a shape rather than deferring it.** Four dispositions are now
> dead with warrants — three by ruling, one by measurement and by a prior
> ruling reached independently. **A successor cannot re-propose any of them
> without meeting a stated ground**, and the one surviving direction is named
> with its blast radius identified before anyone builds it.
>
> **`AC-7` is why this cost one reverted probe instead of a miscompile.** The
> ring took the expensive branch twice in consecutive turns and was right both
> times.
