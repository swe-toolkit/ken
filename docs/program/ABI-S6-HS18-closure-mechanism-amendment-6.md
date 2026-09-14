# ABI-S6 HS18 — closure mechanism, amendment 6

> **THIS DOCUMENT'S RULE IS SUPERSEDED by
> [amendment 7](ABI-S6-HS18-closure-mechanism-amendment-7.md).** The pre-build
> control mandated below returned EMPTY: both retained selected bodies are
> closed, so the producer environment was adequate and **the environment gap is
> not the cause of this witness.** Transfer-as-the-repair is withdrawn. The
> analysis here — the advisory's identity-does-not-entail-environment finding,
> the premise correction, the `D3b` reading, and the emission-ownership
> disposition — stands, and amendment 7 carries it forward. **Read amendment 7
> for the operative rule.** This document is retained as the published record of
> a ruling whose own control refuted it, which is what the control was for.

> **AMENDMENT 5'S HS9 CONDITIONAL FIRED. Its ruling was HELD, not live, from
> `evt_72kntabh04nns` until this document.** Amendment 5 wrote the branch out
> itself — *"If the full chain still fails with no cut attached and the
> population recomputed, that is a genuine hard stop and it is the 9th — at
> which point I hold the ruling and call the research advisory before ruling
> again"* — and that is exactly what happened. Amendment 5's identity rulings
> stand as built; what it predicted would follow from them does not. Read this
> document as the current authority.

Architect, 2026-09-14. Rules HS9 with the third mandatory research prior-art
advisory in hand. Grounded against the implementer's exact WIP
`b601e2ec78989d32222b34b9ec68e01844c6d70b`, which is **not** a candidate.
Runtime stays held at `5d977ac7968dff3763d330690a9b4df530925d79`.

## The count, so that ruling-with-an-advisory is auditable

HS9 is the ninth consecutive hard stop on one design question. The mandatory
holds fired at 3, 6 and 9. I held my ruling (`evt_72kntabh04nns`), framed the
question myself as the trigger requires, and research returned it in two parts
(`evt_1zh94t7vp86k7`, `evt_47ve9ns0xceht`). **Next re-trigger is HS12.**

## Prior art had something new, and this is what it was

The advisory's answer, which I adopt: **a static consumer chain is not
intrinsically replayable at its producer.** Replay is lawful only under a
closure-equivalence condition — after accounting for the scrutinee and case
binders, either the consumer is closed, or every free consumer binding is
explicitly captured or substituted by a value proved to denote that binding **in
the consumer's defining environment**, with effects, order and multiplicity
preserved. Absent that, the standard operation is transfer.

**The sentence that matters to this node:** *exact occurrence, call, value and
edge identities do not entail that environment condition.* Nine amendments
bound identities. None of them could have established this, because it is not
an identity fact.

## I corrected one of the advisory's premises before using it

Research wrote that the ordinary path *"has distinct `producer_env` and
`eliminator_env` parameters and installs `eliminator_env`."* That is true of the
**signature** and misleading as evidence. At the **only** production call site —
`lowering/core.rs:16795`, the sole caller in the crate — both arguments are the
same value:

```rust
self.lower_computational_match_expr(
    builder, scrutinee, cases, default, static_origin,
    env,
    env,
)
```

So the ordinary path does **not** operationally distinguish the two
environments. **This does not rescue the detached path. It makes the finding
stronger and moves it to the right place.**

In the ordinary path the two environments coincide **because the match is one
lexical expression**: the scrutinee is child 0 of the match occurrence, so
producer and eliminator genuinely stand in one scope. The identity
`producer_env == eliminator_env` is correct there **by co-location**, and the
two parameters exist so that a bridge can supply something else.

`checked_ih_post_call_eliminators` reconstructs its frame from
`self.retained_body_occurrence(occurrence.eliminator_origin())` — a body that
lives in the **continuation** — and then applies the ordinary path's identity to
it. **It inherits a justification whose premise is false for a detached
consumer.**

And it is not a wrong choice between two available environments. **There was
never an `eliminator_env` to choose.** The function takes one environment
parameter, and no caller could supply another, because the consumer's
environment is not in scope at the producer at all.

## What I measured at `b601e2ec7`

The reconstruction, verbatim:

```rust
fn checked_ih_post_call_eliminators<'frame>(
    &mut self,
    steps: &[CheckedIhPostCallConsumerStep],
    producer_env: &'frame [LoweringEnvironmentBinding],
) -> Result<Vec<EliminatorFrame<'frame>>, CraneliftBackendError>
...
            eliminators.push(EliminatorFrame::Computational(
                ComputationalEliminatorFrame {
                    cases,
                    default,
                    env: producer_env,
                    static_origin: occurrence.eliminator_origin(),
```

Every reconstructed consumer frame gets `env: producer_env`, while its
`static_origin` is the **eliminator's** origin. The frame's identity comes from
the consumer; its environment comes from the producer.

## Ken already ruled this, and the ruling is sitting in the tree

This is the part that decides the amendment. `ContinuationEnvironmentClaim`
(`planning/static_transition/continuations.rs`) carries its own doc comment:

> `D1` gave every continuation input a root coordinate
> ([`ContinuationSourceCoordinate`]): *which value is this, and in whose terms*.
> That answers identity and is never rewritten. This answers a different
> question — *where does the consumer about to read it find it* — and
> **neither determines the other.**

And its sibling `ContinuationEmitterFrame`:

> Deliberately an enum over the two emitting-frame classes rather than an
> `Option` with a defaulting arm: a generated context that declares no member
> for a value must **reject**, and a default would silently emit a call reading
> whatever sat at the root position.

**That is this failure, named in Ken's own law before HS9 happened.**
`RT-CONTSRC-PRODUCER-LOCAL` `D3b`/`D3c` already measured that root provenance
does not constrain availability, retired the `RootIsImmediate` premise by name,
and built a closed sum over environments with fail-closed resolvers — with an
explicit warning that production consumers must never *"assert their way past
the 'which environment' question."*

**The detached reconstruction asserts its way past it by handing over
`producer_env`.** So the capability this closure needs already exists and this
site does not use it — the same sentence the original consuming-occurrence
ruling wrote about `RequiredConsumerProjection`, one axis over.

## The §1b answer — inventory entry 21. The predicate did not die; it relocated

Amendment 5 named the predicate *"minted independently and then certified"* over
the relation's inputs, and read HS9 as exhausting it. **That reading was one
level too low.**

The predicate is **unchanged in form and moved in object**. What is minted
independently is no longer the edge, the destination or the before-value. It is
the **reconstructed consumer frame**, minted against the producer's environment
and then certified by identity checks that cannot see environments at all.

**This is why the failure did not move.** Every one of the nine corrections
operates on the identity axis. `D3b` says availability is a *separate* axis and
neither determines the other. Binding every identity input therefore leaves
availability free to be supplied from the producer — and **a defect on an axis
you are not correcting does not move when you correct the other one.** The
unchanged trap is not evidence that the predicate was wrong; it is the exact
signature of a predicate operating one axis away from every repair.

⇒ Entries 12 through 21 are **one defect**, not two and not ten.

## The rule

**A detached consuming occurrence must be executed by transferring the result
into the continuation's own emitted body, in the environment that binds its free
variables. The planner and lowering may not reconstruct a consumer frame at the
producer and supply the producer's environment to it.**

The ordered consumer chain remains fully load-bearing — as a **translation and
verification obligation**, not as an executable program. Adopting the advisory's
five steps:

1. identify the exact continuation body that must receive the result;
2. prove the before-value reaches it on the named incoming edge;
3. verify the emitted body realizes the ordered consumers, or that an explicit
   inline rewrite legally removed them;
4. relate the body's outgoing definition and edge to the demanded final
   identity; and
5. reject a producer-to-sink path that bypasses the continuation.

Steps 1, 2, 4 and 5 are what amendments 1 through 5 actually built. **They are
retained in full.** Step 3 is the one that was never built, and replay was the
attempt to discharge it by re-materialization.

## Why transfer, and not replay with a complete substitution

Prior art licenses both. Three grounds, and the first is decisive.

**1. The node's own predicate rules it out.** HS18 opens with a consumer that is
*emitted but never applied* — the four-arm match **is** emitted at
`funcid53`/`Context1`/`origin560` and its tag query never executes. Replay does
not apply that occurrence. It **manufactures a second one** at the producer and
leaves the first standing, unreached. A repair whose mechanism is to emit the
consumer again somewhere else does not answer the question of why the emitted
one is not reached, and it re-creates the two-occurrences-one-meaning shape that
amendment 4 already had to rule out at the value level.

**2. The asymmetry in how the two fail.** A complete-but-wrong substitution stays
well typed. Ken's environments are positional and `Var(index)` reads whatever
`Value` occupies the slot, so a wrong slot is invisible at the point of use and
surfaces as an ordinary default arm — the symptom we already cannot attribute.
Transfer has no substitution to get wrong: the body runs in its own environment
or it does not run.

**3. It is a reuse, not an invention.** Transfer into the already-emitted body
needs no new runtime carrier, no phase tag and no second planner relation. That
preserves the HS6 advisory's result, which I fenced out of this pull and am not
reopening.

## Emission ownership — the disposition, stated explicitly because it is owed

**Transfer does NOT relocate emission ownership, and this ruling reinstates the
prohibition on relocating it.**

- Amendment 1 ruled that the consuming occurrence executes in the generated
  context that **owns** the detached projection, reached by the existing
  context-invocation path, and that ownership must not be relocated.
- Amendment 3 **suspended** that ruling, so it was genuinely open.
- **I reinstate amendment 1's seat ruling and lift amendment 3's suspension.**

Transfer is precisely the mechanism that makes relocation unnecessary: the
consumer runs where it was already emitted, in the environment that binds its
free variables. **Replay is the arm that would have relocated execution in
substance** while leaving ownership formally elsewhere — one emitted definition
carrying two identities, which is amendment 4's failure reappearing at frame
level.

This is the answer to the Steward's conditional in `evt_x1fmkgy3jrvs`: the
ruling does not relocate emission ownership; it restores the rule that forbids
it. On that axis the fence read is unchanged.

## The discriminating control — run it BEFORE building

The advisory is explicit and I hold to it rather than softening it: **the
unchanged default arm is consistent with the environment invariant and does not
establish it.** A complete-but-wrong substitution, a correct environment
carrying a wrong semantic value, and a genuinely absent constructor all present
as this same default. `target/hs18-ci-red/hs8-two-arm-write-all.log` records no
free-variable or environment comparison, so it cannot discriminate.

So, for eliminator origins **699** and **661**, before any build:

- report the **free-variable set** each retained body reads beyond its own case
  binders and its scrutinee; and
- for each such index, the `(environment_origin, environment_index)` it resolves
  to in the **consumer's** environment, against what occupies that positional
  slot in `producer_env`.

**Non-empty and differing** ⇒ the cause is confirmed and this ruling closes it.

**Empty** — the bodies read nothing free — ⇒ `producer_env` was adequate for
this witness, the failure has another cause, and **I want that back before the
build, not after.** Say so plainly; that is a stop, not a detail.

**This control can fail, and it is the one that would refute me.** Report the
result either way. A confident negative here is worth more than a build that
happens to go green.

## Not authorized

No new runtime carrier, phase tag or second planner relation — the HS6 result
stands. No ABI, schema, frame, owner key, tag, route-to-runtime, stack or bound
change. No weakening of the pre-object refusal. No revert of the entry-18
verifier repair, which is what makes the transfer certifiable. No relocation of
emission ownership. R1 and R2 remain should-fix on the candidate. `b601e2ec7` is
not a candidate and nothing here promotes it.

**And do not attempt a third representation of the identity chain.** Identity is
finished — that is the point of this ruling, and re-cutting it would be the
tenth correction on the axis that was never the defect.

## Symptom inventory — entry 21, for the Steward to fold

```text
21. HS9: every identity input bound and honest at b601e2ec7 (self-defining arm,
    exact edge equality, recomputed population, full chain [699,661]) and the
    witness fails IDENTICALLY — same trap, same planned identity 43. The failure
    did not move. checked_ih_post_call_eliminators assigns env: producer_env to
    every reconstructed consumer frame while taking static_origin from the
    eliminator — keyed on the producer's environment, for a consumer that is not
    lexically there.
    PREDICATE (entries 12-21, answering the 3rd-entry question): unchanged in
    form, relocated in object. "Minted independently and then certified" now
    names the reconstructed consumer FRAME rather than the edge or the
    before-value. Ken's own D3b law states the reason it survived nine repairs:
    identity and availability are separate axes and neither determines the
    other, so binding every identity leaves the environment free to be minted
    from the producer.
```
