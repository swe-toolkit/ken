# ABI-S6 HS18 — closure mechanism, amendment 7

> **ERRATUM ON CURRENT `main`, AND IT CORRECTS AMENDMENT 6'S RULE.** Amendment 6
> (`325f5b531`) mandates transfer into the continuation body on the ground that
> the detached reconstruction supplies the producer's environment. **The
> pre-build control amendment 6 itself mandated refuted that cause for this
> witness** (`evt_5yhqkbq6phkhm`): both retained selected bodies are closed, so
> the producer environment was adequate and the environment axis is settled
> rather than defective.
>
> **I called a §14(6) hold on amendment 6's publication and lost the race by
> seconds** — it had already squash-merged as `325f5b531` through PR #3619 when
> the hold was delivered. That is the documented perishability of a
> "hasn't merged yet" judgment, not a routing fault, and the lieutenant's
> handling was correct. So the correction arrives the sanctioned way: as an
> erratum on current `main`, leaving amendment 6 intact as the published record
> rather than rewriting it under its own name.
>
> **What survives from amendment 6:** the advisory's finding that identities do
> not entail the environment condition; the correction to its premise about the
> ordinary path; the `D3b` identity-versus-availability reading; the retention
> of amendment 1-5's steps 1, 2, 4 and 5; and the emission-ownership
> disposition. **What is withdrawn:** transfer-as-the-repair, and the rule
> section that mandates it. Amendment 6's environment rule is retained here as a
> discipline that this witness **discharges**.

Architect, 2026-09-14. Rules HS10 on the control that amendment 6 mandated, and
carries amendment 6's surviving HS9 findings forward so this document stands
alone. Grounded against the implementer's exact WIP
`b601e2ec78989d32222b34b9ec68e01844c6d70b`, which is **not** a candidate.
Runtime stays held at `5d977ac7968dff3763d330690a9b4df530925d79`.

## The count

HS9 was the ninth consecutive hard stop; the mandatory holds fired at 3, 6 and
9. I held (`evt_72kntabh04nns`), framed the question myself, and research
returned in two parts (`evt_1zh94t7vp86k7`, `evt_47ve9ns0xceht`). HS10 is the
tenth and does **not** trigger a hold. **Next re-trigger is HS12.**

## Prior art had something new, and this is what it was

The advisory's answer, which I adopt: **a static consumer chain is not
intrinsically replayable at its producer.** Replay is lawful only under a
closure-equivalence condition — after accounting for the scrutinee and case
binders, either the consumer is **closed**, or every free consumer binding is
explicitly captured or substituted by a value proved to denote that binding in
the consumer's defining environment, with effects, order and multiplicity
preserved.

**The sentence that matters to this node:** *exact occurrence, call, value and
edge identities do not entail that environment condition.* Nine amendments bound
identities. None of them could have established this, because it is not an
identity fact.

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

In the ordinary path the two environments coincide **because the match is one
lexical expression**: the scrutinee is child 0 of the match occurrence, so
producer and eliminator genuinely stand in one scope. The identity is correct
there **by co-location**.

`checked_ih_post_call_eliminators` reconstructs its frame from
`self.retained_body_occurrence(occurrence.eliminator_origin())` — a body that
lives in the **continuation** — and applies the ordinary path's identity to it,
inheriting a justification whose premise is false for a detached consumer. It is
not a wrong choice between two available environments: **there was never an
`eliminator_env` to choose.** The function takes one environment parameter and
no caller could supply another.

## Ken already ruled this axis, and the ruling is sitting in the tree

`ContinuationEnvironmentClaim` carries its own doc comment:

> `D1` gave every continuation input a root coordinate. That answers identity
> and is never rewritten. This answers a different question — *where does the
> consumer about to read it find it* — and **neither determines the other.**

And `ContinuationEmitterFrame`: *"a generated context that declares no member
for a value must reject, and a default would silently emit a call reading
whatever sat at the root position."* `RT-CONTSRC-PRODUCER-LOCAL` `D3b`/`D3c`
already retired the `RootIsImmediate` premise and warned that production
consumers must never *"assert their way past the 'which environment' question."*

## The environment rule, and why it is CLOSED rather than operative

**A consumer frame reconstructed away from its lexical site may be given the
producer's environment only when the consumer's closure is PROVED and the proof
is carried — never by inheriting the co-located case's identity.**

That rule stands as a discipline. **And the control discharged it for this
witness**: the proof exists, the consumers are closed, and replay is therefore
licensed here by this rule's own criterion. **This axis is settled. It is not
the defect, and no work is authorized against it.**

## HS10 — what the control returned, and why I am glad I mandated it

`evt_5yhqkbq6phkhm`, measured at `b601e2ec7` with a temporary fail-fast
observation, restored byte-clean:

- eliminator **699**, retained selected body 706, case ordinal 1:
  `argument_binders = 2`, `recursive_positions = [1]`, own binder run 3. Body:
  `CheckedComputationalIHSlots -> Construct ITree::Vis [Var(1),
  CheckedComputationalIHInvocation -> Call { callee: Var(0), args: [] }]`. Only
  reads are indices 1 and 0, both inside the own binder run. **Free set EMPTY.**
- eliminator **661**, retained selected body 668, case ordinal 1: same
  `argument_binders`, `recursive_positions` and reads. **Free set EMPTY.**

So there are zero consumer rows to compare against `producer_env`, the
positional comparison is vacuous, and **the environment premise is refuted as
the cause of this witness.** The implementer did not start the repair. That is
the control working exactly as specified, and it cost a measurement instead of a
build.

## The evidence the control returned points somewhere else, and it is specific

**The selected arm of eliminator 699 constructs an `ITree::Vis`.** `spec/30-
surface/36-effects.md:489` declares:

```
data ITree (E : Effect) (R : Type ℓ_R) : Type (max ℓ_R ℓ_op ℓ_resp) where
  Ret : R → ITree E R
  Vis : (e : E.Op) → (E.Resp e → ITree E R) → ITree E R
```

with `Ret r` = *"finished with value `r`"* and `Vis e k` = *"perform operation
`e`, then continue with `k`."* **A `Vis` is by declaration an UNFINISHED tree.**

The measured body has the shape of a **re-wrapping** `Vis` method — compare the
elaborator's two distinct forms, `effects/lower.rs:120`
(`method_vis: λk. λih. Vis_S ih`, *"re-wrap IH"*) against
`effects/lower.rs:210` (`method_vis: λk. λih. ih fixed_response`, *"apply
IH"*). A re-wrapping `Vis` method is a **tree-to-tree transformation**: its
result is necessarily an `ITree`, never a finished value.

So the chain `[699, 661]`, executed correctly, in a correct environment, with
every identity bound, **produces a `Vis` and hands it to a `Result` matcher.**
The trap says exactly that: `PatternMatchFailure: no runtime match case selected
for decl:px8f_write_all_native::Result`. A `Result` match over a `Vis` selects
nothing, and selects nothing *honestly*.

**And the capability to finish it already exists and this path does not use
it.** `drive_handler_owned_deferred_response` (`lowering/core.rs:5534`) is
reached from `core.rs:6480` and `core.rs:15051`; `drive_deferred_no_unit_
response_call` sits at `core.rs:5257`; the interpreter drives with `drive_h`.

## The §1b answer — the predicate, restated over the whole chain

Amendment 5 named the predicate *"minted independently and then certified"* over
the relation's inputs and read HS9 as exhausting it. Entry 21 relocated it to
the reconstructed consumer frame. **HS10 says both readings were describing
symptoms of something simpler, and this is the statement I will be held to:**

**The obligation between a `Vis` producer and a `Ret` consumer is a FIXPOINT,
not a finite ordered sequence of static consuming occurrences.** The number of
`Vis` steps between an unfinished tree and its `Ret` is a **runtime** quantity.

⇒ **No chain of static occurrences, however perfectly identified, can bridge
`Vis` to `Ret`.** That is why ten repairs did not move the failure: every one of
them improved the *identification* of a chain that is structurally incapable of
producing the demanded value. Identity was correct. Environment was correct. The
chain was correct **and finite**, and the obligation is not.

This also vindicates the original ruling's px8f prediction in a sharper form.
It predicted px8f was the same defect because *"a `Vis` is precisely a
pre-consumption value."* Right observable, and now the mechanism is visible: a
`Vis` is pre-consumption **not because a consumer was skipped, but because the
tree was never driven.**

## The rule

**The consuming-occurrence chain is a translation and verification obligation,
never an executable program.** Adopting the advisory's five steps: identify the
exact body that must receive the result; prove the before-value reaches it on
the named incoming edge; verify the emitted body realizes the ordered consumers;
relate its outgoing definition and edge to the demanded final identity; reject
any producer-to-sink path that bypasses the continuation. Steps 1, 2, 4 and 5
are what amendments 1 through 5 built, and **they are retained in full.**

**Where the chain terminates in an unfinished `ITree` and the sink demands a
finished value, the missing operation is the effect-driving FIXPOINT, and it
must be reached — not re-derived, and not approximated by extending the chain.**
Ken already carries that machinery. **Do not build a new one.**

## Emission ownership — the disposition, unchanged and now on a broader basis

**Neither candidate repair relocates emission ownership, so the disposition I
gave the Steward stands and needs no retraction.** I reinstate amendment 1's
seat ruling and lift amendment 3's suspension: the consuming occurrence executes
in the generated context that owns the detached projection, reached by the
existing context-invocation path.

This previously rested on transfer being the repair. **It no longer depends on
which repair is chosen** — reaching an existing drive step does not move where
any consumer is emitted, and neither does transfer. The fence read is unchanged
on this axis, on firmer ground than when I first gave it.

## The next control — again BEFORE any build

The `Vis`-vs-`Ret` reading above is grounded in the declaration, the measured
body and the elaborator's two method shapes. **It is still a mechanism story,
and the last one was refuted by exactly this instrument.** So it gets the same
treatment, and the same licence to refute me:

1. **Is the driving step emitted and unreached, or genuinely absent from the
   emitted program?** This is the original ruling's own decisive question, asked
   one level out. Emitted-and-unreached makes this a reachability repair against
   existing machinery; genuinely-absent makes it a missing-operation repair, and
   the two are not the same work.
2. **Report 661's selected body in full**, not only its reads. If it also
   constructs a `Vis`, the chain is two tree transformations and the `Result`
   demand sits downstream of both. If it is the `Result` match itself, the
   `Vis`-into-`Result` adjacency is direct and the drive belongs between them.
3. **Which `method_vis` shape is correct for this program** — re-wrap
   (`lower.rs:120`) or apply (`lower.rs:210`)? If the elaborator selected
   re-wrap where this program needs apply, the defect is upstream of the backend
   entirely and no amount of backend work is the fix.

**Any of the three can refute me, and item 3 would move the defect out of
Runtime's lane altogether.** Report all three before building. If the answer is
that the drive is genuinely absent *and* the elaborator's selection is correct,
say so plainly and stop — that is a scope question for the Steward, not a
repair.

## Not authorized

No new runtime carrier, phase tag or second planner relation — the HS6 result
stands. No ABI, schema, frame, owner key, tag, route-to-runtime, stack or bound
change. No weakening of the pre-object refusal. No revert of the entry-18
verifier repair. No relocation of emission ownership. No work against the
environment axis: it is closed. R1 and R2 remain should-fix. `b601e2ec7` is not
a candidate.

**And do not attempt an eleventh correction of the identity chain.** Identity is
finished, environment is finished, and re-cutting either would be another repair
on an axis that was never the defect.

## Symptom inventory — entries 21 and 22, for the Steward to fold

```text
21. HS9: every identity input bound and honest at b601e2ec7 (self-defining arm,
    exact edge equality, recomputed population, full chain [699,661]) and the
    witness fails IDENTICALLY — same trap, same planned identity 43. The failure
    did not move. checked_ih_post_call_eliminators assigns env: producer_env to
    every reconstructed consumer frame while taking static_origin from the
    eliminator — keyed on the producer's environment, for a consumer that is not
    lexically there. The third mandatory research advisory returns that exact
    occurrence/call/value/edge identities do not entail the environment
    condition, so no identity repair could have reached this.
22. HS10: the mandated pre-build control REFUTES entry 21's cause for this
    witness — both retained selected bodies (699/706 and 661/668) have EMPTY
    free-variable sets, so producer_env was adequate and the environment axis is
    closed. The control's own evidence relocates the defect: 699's selected arm
    CONSTRUCTS ITree::Vis, which spec 36-effects.md:489 declares as the
    unfinished node, in the elaborator's re-wrapping method shape — so the chain
    produces a tree and hands it to a Result matcher, which selects nothing
    honestly.
    PREDICATE (entries 12-22, and the shared statement the chain was missing):
    the obligation between a Vis producer and a Ret consumer is a FIXPOINT, not
    a finite ordered sequence of static consuming occurrences, because the
    number of Vis steps is a runtime quantity. No chain of static occurrences,
    however perfectly identified, can bridge Vis to Ret. That is why ten repairs
    improved identification and none moved the failure.
```
