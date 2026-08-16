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

## `D0` — classify the option space before building anything

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

## `D1`/`D2` — not released by this frame

**`D1` is the selected shape, built.** **`D2` is its control, two-directional
and mutation-proven**, exactly as on the predecessor: a mutation that would
suppress an obligation whose transport IS named by an emitted call must red,
**and** a mutation leaving an unnameable obligation minted must red. **A green
"the leak is gone" is not evidence for the second direction.**

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

**`AC-4`.** **`D0` reports UNKNOWN rather than guessing.** A shape classified
LAWFUL without a warrant is worse than one left open, because the build is
released on it.

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

## Sequencing

**Lane 1 (operator priority). `D0` is releasable immediately** — the
predecessor's `D0` and `D1` are complete, and this node needs nothing further
from it.

> **The predecessor closes without a candidate.** Its `D0` returned EMITTER
> PROPERTY and its `D1` reached a lawful pre-authorized hard stop, so it can
> never reach `merged`. **Do not gate this node on that landing.**

**TCB-adjacent: expect the Architect on `D0`'s answer before `D1` is
released.** That is deliberate. A ledger-contract change absorbed into a
deliverable sized as bookkeeping is how a small repair becomes an unreviewed
structural one.
