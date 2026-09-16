---
id: ABI-S6-HS18-CHECKED-IH-CONSUMER-PORT
title: "Port the checked-IH post-call consumer machinery that main never grew, so ABI-S6-HS18 increment A can compile: one type family (CheckedIhPostCallConsumer, CheckedIhPostCallConsumerStep) plus three accessors on existing types (StaticTransitionPlan::checked_ih_generated_context_result_contract, StaticTransitionPlan::static_response_forwarded_result_identity, Lowering::checked_post_call_consumer_frame). Increment A holds the call sites and none of the definitions, so it cannot build without this. FIRST DELIVERABLE IS THE TRANSITIVE CENSUS, NOT THE PORT -- whether the consumer machinery itself calls anything else that is absent from main is UNMEASURED, and finding a fourth population after the port is the failure this node exists to prevent."
status: ready
owner: runtime
size: M
gate: none
depends_on: []
blocks: [ABI-S6-HS18-MAIN-BASED-CLOSURE]
github: null
tier: T1
origin: "Steward cut 2026-09-16 on the Architect's category-B ruling (evt_6mptkrtvysd8s), relayed by runtime-leader (evt_66sfqncjzkmm): 'Cut it as its own node, sequenced ahead of A, named for the capability rather than for the errors that exposed it.' Discovered by runtime-implementer's D0-2 characterisation of increment A's 21 compile errors (evt_2gg7fd748n466), which split 11 Category A (incomplete re-derivation, no dependency) / 10 Category B (this node). The Architect re-measured the census with a live control before ruling and CORRECTED the implementer's WIP column 20/40 -> 2/4; the correction strengthened the finding, because 2 and 4 are dangling references with no definitions behind them, which is what E0425 means, whereas 20 and 40 would have said the tree already carried the machinery."
---

> # READY. Frame:
> `docs/program/wp/ABI-S6-HS18-CHECKED-IH-CONSUMER-PORT.md`.
>
> **First deliverable is the TRANSITIVE CENSUS, not the port. Do not begin
> porting before D0 is answered and posted.**
>
> **This node is sequenced BEFORE increment A of
> [[ABI-S6-HS18-MAIN-BASED-CLOSURE]], and the ordering is structural rather
> than a preference: increment A cannot compile without it.** That is why it is
> not increment C (C is sequenced after A — a bucket that lands after the thing
> depending on it is a deadlock, not a bucket) and why it is not folded into A
> (A is the re-derivation of the verifier substance; adding a 48-reference type
> family plus three accessors makes A the whole substance again, which is
> exactly what the increment split exists to prevent).

# The census, measured with a live positive control

The Architect's measurement (`evt_6mptkrtvysd8s`), re-verified by the Steward
before this node was cut:

    symbol                                        main   base   ckpt    WIP
    CheckedIhPostCallConsumerStep                    0     16     16      2
    CheckedIhPostCallConsumer                        0     32     32      4
    checked_ih_generated_context_result_contract     0      5      5      3
    static_response_forwarded_result_identity        0      2      2      1
    checked_post_call_consumer_frame                 0      2      2      1
    StaticTransitionPlan                           291    300    300    291

      base = 30d35f6256421a0b86cf772c739794365b3402be
      ckpt = 5d977ac7968dff3763d330690a9b4df530925d79
      WIP  = f43ded9a0422426db68e7a0e6b7952c8f96f2b22
      control  cranelift_backend  2239 / 2325 / 2326 / 2241

**Every zero above is a measurement rather than a broken instrument**, because
the control fires in the thousands at every one of the same refs.

# The shape is NARROWER than "planner machinery absent from main"

**`StaticTransitionPlan` is on `main`** at 291 references. It is not a missing
layer, and the node must not be cut or framed as one. What is absent:

    one type family, genuinely absent   CheckedIhPostCallConsumer{,Step}
                                        48 references at base
    two accessors on an EXISTING type   StaticTransitionPlan::
                                          checked_ih_generated_context_result_contract
                                          static_response_forwarded_result_identity
    one accessor on an existing type    Lowering::checked_post_call_consumer_frame

That is a bounded, nameable capability — the checked-IH post-call consumer
machinery plus the three accessors that surface it.

# It is a PORT from a known source tree, not authoring from scratch

Steward-verified 2026-09-16 at
`b601e2ec78989d32222b34b9ec68e01844c6d70b` (the preserved amendment-8 WIP
branch), which runtime-leader identified (`evt_2e0vh7qnbq1ea`) and which this
node's author re-measured rather than accepting on report:

    symbol                                        defs   refs   refs on main
    CheckedIhPostCallConsumerStep                    1     20              0
    CheckedIhPostCallConsumer                        1     40              0
    checked_ih_generated_context_result_contract     1      5              0
    static_response_forwarded_result_identity        1      2              0
    checked_post_call_consumer_frame                 1      2              0

      positive control  cranelift_backend on main = 2244

**Exactly one definition site per symbol, all under `planning/**`.** So
`f43ded9a0` (the increment-A WIP) holds the `lowering/`-side call sites and
none of the definitions, and `b601e2ec7` holds the definitions. *"Port these
definitions from a named tree"* is a materially different size from *"author
this machinery"*, and the frame must say which it is.

> **`b601e2ec7` IS EVIDENCE, NOT A BASE.** The same rule the parent node
> applies to its two `preserve/` refs applies here without modification: read
> it, port from it, and **never make it an ancestor of a candidate**. The
> candidate's base is `origin/main`, per the operator's 2026-09-16 direction
> ("do not build on an unmerged commit"). A port whose provenance is a tree is
> still a main-based candidate.

# A SIXTH ERROR, AND THE CENSUS ALREADY EARNED ITS PLACE

After Category A was cleared (`bd3e1ff676444f48771d261371982a8f289ce26a`, 21
errors -> 11, every remainder Category B), the error list carried a symbol the
five-symbol seed above does not name: **`constructor_identity` x1**. The
implementer reported all eleven as *"all 0 on main"*. Steward-measured, that is
**false as stated and true as meant**, and the difference is the whole reason
the census is deliverable one:

    constructor_identity, domain crates/**              main: 4 refs, 1 def
    constructor_identity, domain crates/ken-runtime/**  main: 0 refs

    positive control cranelift_backend, ken-runtime, main   2239

**Two unrelated symbols share the name.** `main` carries a free function
`constructor_identity` in `crates/ken-elaborator/src/erasure.rs:6872` — a
different crate, nothing to do with this. In `ken-runtime` the name is a
**local variable** at `b601e2ec7`'s `lowering/core.rs:14247`. So the Category B
claim holds once the domain is `crates/ken-runtime/`, and a census run over
`crates/**` would have reported a false hit.

⇒ **This is the coordinate lesson one layer down: a symbol census needs its
DOMAIN pinned as explicitly as its REF.** Two correct measurements that
disagree are a coordinate problem before they are an instrument problem.

**And the local variable is not the dependency — but the method that produces
it is NOT the gap either.** `b601e2ec7:lowering/core.rs:14247` reads
`self.static_transition_plan.constructor_symbol_identity(origin)?`, and that
method is **present on `main`**, on the same receiver, with a byte-identical
signature and body (`closure.rs:1202`, `impl<'src> StaticTransitionPlan<'src>`).
The apparent third `impl` at `b601e2ec7` is a **string literal** in an
allowed-inventory test (`lowering/core/tests/mod.rs:1249`), not a definition.

⇒ **The sixth error's cause is UNESTABLISHED.** It is not an absent symbol and
it is not an absent impl. **Do not carry a third repair shape into the port on
this example** — resolve it in the census, where it belongs. What the sixth
error proves is only that the seed list was short, and that a compile found what
five greps did not, which is the whole case for census-first.

> **The defect in the retracted claim, recorded because it is the third of its
> kind today.** A `fn +NAME` grep counted `"pub(in crate::cranelift_backend) fn
> constructor_symbol_identity("` — a quoted string in a source-text inventory
> test — as a definition, producing "3 defs" where there are 2. That is the same
> class as the `constructor_identity = 70` substring count above: **an
> instrument that passed a reach control and was wrong on SPECIFICITY, in the
> inflating direction, both times.**
>
> ⇒ **A GREP KEY THAT MATCHES TEXT CANNOT DISTINGUISH A DEFINITION FROM A
> MENTION OF ONE** — a doc comment, a test fixture, an inventory assertion, an
> error-message string. The fix is not a better regex; it is **resolving what
> encloses each hit before the count is allowed to mean anything.** All three of
> today's instances were caught by opening the matches, and none by a control.

**The retracted claim was:** *"The symbol is on `main`. One `impl` of it is
not."* That was a materially
different repair from porting an absent method, and it is a **different shape
from the other five entries** — those are absent symbols; this is an absent
impl of a present symbol. **Do not assume the census's remaining hits all have
the first shape.**

# THE SCOPE BOUNDARY IS A PREDICATE. THE LIST IS A SEED.

**This is the node's single most important framing constraint** (runtime-leader,
`evt_z7rw3qpqsz3n`; it is also what the Architect's "do not let it be cut as
'add these five things'" amounts to operationally):

> Category B's boundary goes in as a **predicate** — *every symbol whose fix
> requires planner machinery absent from `main`* — **not the eleven-item list.
> The enumeration was already short by one, found only by attempting a fix, so
> it cannot be trusted as exhaustive.**

**A list cannot report being incomplete.** This one demonstrated that
empirically before the node was even cut: the five-symbol seed missed
`constructor_identity`, and nothing found it except trying to compile. A second
attempt can surface a seventh, and **that is expected under the predicate
rather than a defect in it.**

Two consequences the frame must carry:

- **Increment A's exit criterion is ZERO ERRORS, not "the eleven are gone."**
  Those are different criteria and only the first is sound. An exit criterion
  naming a count is satisfied by a tree that still does not build.
- **This node's own exit criterion cannot be "the seed is ported" either.** It
  is the predicate's closure: every symbol reachable from the ported
  definitions either has a non-zero count on `main` **in the `ken-runtime`
  domain** or is itself in scope here.

# Classify by what the FIX requires, not by the compiler's symptom

Runtime-implementer's own correction (`evt_59738y91rxyhg`), and the frame must
carry it as a criterion rather than as advice:

> My A/B split was by error code and that was wrong. `E0609 no field` reads
> structural and its fix needed a planner method. **Classify by what the fix
> requires, not by what the compiler calls the symptom.**

One of eleven landed in the wrong bucket, and the ring was sequencing on that
split. An error-code partition is a partition of *symptoms*, and the A/B
boundary is a claim about *causes* — they are not the same partition and
nothing makes them coincide.

**Related non-fix worth preserving:** `OwnedContext.result_contract` could be
set `None` today and would **build**, silently disabling the identity check the
field carries. The implementer instead wrote the checkpoint's real expression,
which calls a Category B method, **so the file fails on a named dependency
rather than passing on an invention.** That is the correct disposition and the
frame should not let a later author "fix" it back.

# THE FIRST DELIVERABLE IS THE TRANSITIVE CENSUS, NOT THE PORT

**Architect, and this is the clause they said the node must not be cut
without:**

> One boundary I have NOT measured, and it is the first thing the new node must
> check. Whether the consumer machinery *itself* calls into anything else that
> is 0 on `main`. I measured the five symbols you named and nothing beyond
> them. If it has its own absent dependencies, we find a fourth population at
> exactly this point again — so the new node's first deliverable is the
> transitive census, not the port. **Do not let it be cut as "add these five
> things."**

⇒ The five symbols above are the census's **seed**, not its result. A frame for
this node states the closure condition — every symbol reachable from the ported
definitions has a non-zero count on `main`, or is itself in this node's scope —
and gives it a control, because a census with no positive control cannot
distinguish "nothing else is missing" from "my grep was wrong."

# The predicate all three findings share, and why it matters here

**Architect, `evt_6mptkrtvysd8s`** — this is the third `§1b` inventory entry
keyed on the same property (`register_generated_context_result_authority`; the
`register_generated_context_result_join` / `close_generated_context_result_forwarding`
pair; now this):

> The substance's line and `main`'s line diverged, and every "missing" thing
> found so far is something **`main` never had** rather than something `main`
> removed. That is not a defect to fix per-symbol — it is the reason the
> re-derivation is a **port and not a rebase**.

⇒ **Categories A and B are the two halves that predicate produces: A is where
the substance touches code `main` changed; B is where it touches code `main`
never grew.** Anyone tempted to treat this node as a list of five symbols should
read that sentence again — the transitive census is the closure over the
predicate, and the five symbols are just where it surfaced.

# Not this node

- Increment A's 11 Category-A errors — incomplete re-derivation, no dependency,
  already unblocked and in progress on [[ABI-S6-HS18-MAIN-BASED-CLOSURE]].
- The `MappingAcquireFile` capability grant the Architect refused. It is carried
  by the preserved checkpoint across five coordinated sites and is out of scope
  here exactly as it is in the parent node.
- Increment B (Q1's resume-exit repair) and increment C (amendment 8's consumer
  relocation) — both remain in the parent node.

# Related

- [[ABI-S6-HS18-MAIN-BASED-CLOSURE]] — the parent; this node blocks its
  increment A.

# §1a / §1b bookkeeping, recorded here because the Architect cannot write to `main`

**Architect, `evt_6mptkrtvysd8s`:** *"This is a hard-stop on the same WP: you
built the ruling, hit a structural wall, stopped with evidence, and I am ruling
again. **HS18's §1a count goes 12 -> 13.** Trigger stays the 15th. @steward,
your tracker is the count of record."*

    §1a hard-stop count   13   (was 12; trigger remains the 15th)

**`§1b` inventory entry 14, verbatim as the Architect dictated it** — they
recorded it in-thread explicitly because they cannot write to `main` from their
worktree, and asked whoever holds the frame to file it:

    14. checked-IH post-call consumer machinery absent from main, referenced by
        the verifier substance — keyed on: the substance's divergent line grew
        machinery main never had

**This is the THIRD entry keyed on that same property** — the others being
`register_generated_context_result_authority`, and the
`register_generated_context_result_join` /
`close_generated_context_result_forwarding` pair. The Architect named the shared
predicate rather than leaving it as three coincidences:

> The substance's line and `main`'s line diverged, and **every "missing" thing
> found so far is something `main` never had rather than something `main`
> removed.** That is not a defect to fix per-symbol — it is the reason the
> re-derivation is a **port and not a rebase**.

The `§1a`/`§1b` ledger itself lives in the
`docs/program/ABI-S6-HS18-closure-mechanism-amendment-N.md` series, currently at
amendment 8. **Whether entry 14 and the count of 13 are transcribed into that
series or into a new amendment is the Architect's call about their own
instrument, not the Steward's** — this section exists so that the values are on
`main` and auditable in the meantime, rather than surviving only inside a convo
thread.
