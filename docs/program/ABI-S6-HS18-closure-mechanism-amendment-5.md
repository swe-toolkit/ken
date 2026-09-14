# ABI-S6 HS18 — closure mechanism, amendment 5

Architect, 2026-09-14. Extends
[amendment 3](ABI-S6-HS18-closure-mechanism-amendment-3.md) and
[amendment 4](ABI-S6-HS18-closure-mechanism-amendment-4.md) on the ring's
**eighth** hard stop (`evt_35n0d5ehaf3ex`, WIP
`3e8cc3c3d`, explicitly not a candidate). Every line number below is at that
SHA, which is the tree the ring measured.

**This amendment supersedes nothing.** Amendment 3's construction rule and
amendment 4's realization rule both stand exactly as written. What follows is
not a fourth statement of the rule — it is the **first step of the rule's own
four-step closure**, which was stated in amendment 4, adopted from the
advisory, and never made enforceable.

## The §1a count, stated so it is auditable

This is hard stop **8**. The mandatory research hold fired at the 3rd
(`evt_76xhx9x5fpafq`) and again at the 6th (advisory `evt_2k4w4qfasxxfq`,
`evt_31jk5cggky8ar`). **The next re-trigger is the 9th.** It does not fire here,
so this ruling is issued unaided — deliberately, and on the record, so that if
HS9 arrives the hold is not argued about.

## The ring did the right thing and the report is correct

The stop names the mismatch precisely and localizes it to two lines. It also
declines the tempting move: *"I have not tried the tempting prefix-only `[699]`:
that contradicts the ruling and would guess past the missing owner handoff."*
That is the correct behaviour, and it is why this costs an amendment rather
than a wrong repair that passes.

## The question as posed has no answer, and that is the finding

The stop asks: *what existing lowering path must produce the response-owner
post-prefix Result before the selected suffix is applied at the terminal branch?*

**None, and none should be built.** The owner's post-prefix Result is produced
by the owner call and nowhere else — `core.rs:4331` refuses outright unless a
Result load follows *that exact selected-owner call*
(`"a static-response return receipt's Result load does not follow its exact
selected-owner call"`). If the terminal branch emits no such call, that value
does not exist in the emitted program. **Synthesizing it would be minting the
authority the whole closure exists to forbid.**

So the question is inverted. The terminal branch is not short of a value. It is
applying a consumer edge that belongs to a call it does not emit.

## Step one of the advisory's own closure is unenforced

Amendment 4 records the four steps the advisory said must close, in its words:

1. **the exact finished callee and Result-load edge establish the raw
   identity**;
2. the exact source consumer accepts that raw definition on a named incoming
   edge;
3. lowering the consumer establishes a distinct after-definition or a Trap;
4. every path from that definition to the final obligation preserves the result.

HS7 built steps 2 and 3 — the completed incoming-edge triple and the
edge-selected suffix. **Step 1 was never built.** Nothing relates the value
handed to the consumer to the callee the edge names.

`RequiredConsumerIncomingEdge` states the invariant in its own doc comment
(`responses.rs:292-296`):

> The exact before-value transport paired with the selected incoming consumer
> edge. The defining call is derived from the destination's one transport;
> **lowering has no second call identity it could supply independently.**

**Lowering supplies exactly that, at both of its two sites.** The before-value
is a bare parameter:

```text
core.rs:6864   let raw = answer.value;                       // terminal / no-active
               lower_checked_ih_detached_required_consumer_edge(builder, consumer, raw, ...)

core.rs:6982   let raw = claimed.answer.value;               // active
               lower_checked_ih_detached_required_consumer_edge(builder, consumer, raw.clone(), ...)
```

At both sites `raw` is the result of `call_checked_ih_environment_transport`
for *that site's* `transport`. The edge is fetched separately, from
`consumer.required_consumer_incoming_edge()` (`core.rs:9382`), and **the two
are never compared.** The function signature takes `raw` and `consumer` as
independent arguments and has no expression relating them.

### The one guard that exists does not close it

`apply_required_consumer_incoming_edge` (`core.rs:9254`) opens with what looks
like the missing check:

```text
let defining_call = edge.destination().defining_call_identity();
if !self.continuation_candidate_is_consumed(defining_call) { refuse }
```

`continuation_candidate_is_consumed` (`core.rs:8705`) is a **ledger query**: it
asks whether that identity is settled in `continuation_candidates`, or present
in `pending_composed_discharges`. It is a fact about the plan's state, not about
the operand. **It answers "has this call been discharged somewhere", never "is
the word I am about to consume the result of this call".** It passes on the
terminal branch for exactly that reason.

⇒ The pairing amendment 3 requires — *(exact before-value, exact consumer
occurrence, exact incoming edge)* — is enforced on **two** of its three
components and open on the third. The open one is the before-value.

## The predicate, and it has not changed since the original ruling

§1b requires me to say whether these stops share a predicate rather than ruling
on them one at a time. **They do, and it is the predicate the
[consuming-occurrence ruling](ABI-S6-HS18-consuming-occurrence.md) named
before any amendment was written:**

> A producer-to-sink transport edge must be derived from the
> consuming-occurrence chain, **never minted independently and then certified.**

Read the amendments against it:

| stop | what was minted independently, then certified |
|---|---|
| amendment 1 | the projection *shape* — keyed on the producer's own, when a detached one selects it |
| amendment 2 | the route *variant* — `DirectInvocationReturn` was a second, unguarded producer-to-sink route |
| amendment 3 | the *destination* — a class coordinate resolved by membership, applied to whichever transport arrived |
| **HS8** | **the before-VALUE — supplied by the call site, applied to whichever edge the consumer names** |

**Each amendment closed one input of the relation and left another open.** That
is not four defects. It is one defect, relocating each time the nearest input is
bound — which is precisely what "minted independently and then certified"
predicts will happen for as long as *any* input is still supplied from outside.

**I am not going to close the fourth input and wait for a fifth.** Amendment 2
pre-committed that at the third recurrence "my rule was too narrow" stops being
a better explanation than "the rule is the wrong shape". That pre-commitment is
due, and this is my answer to it: the rule's *content* has been right since
amendment 3. What is wrong is that it is stated over an **object** while being
enforced at the **sites that build the object**, leaving every input not yet
bound free to arrive from somewhere else.

## The rule

**No consumer application may be lowered from inputs that were not minted
together.** Concretely:

**`lower_checked_ih_detached_required_consumer_edge` must stop accepting the
before-value as an independent parameter.** The before-value must be obtained
**from the edge**, by the same object that already names the defining call — so
that a call site holding a word from a different call has no way to present it.

Where a call site already holds the word, the obligation is the check that is
absent today, and it is one comparison:

> the call identity of the transport **just emitted at this site** must equal
> `edge.destination().defining_call_identity()`.

A mismatch is a **refusal naming both identities**, at the point of the
mismatch, in the compiler. It is never a default match arm at runtime.

**That refusal is the deliverable, not the repair.** Today the mismatch is
discovered by executing the program and trapping in frame 2's default
(`PatternMatchFailure: no runtime match case selected for
decl:px8f_write_all_native::Result`). Under this rule it is discovered where it
is committed, and it names which call was expected and which arrived.

## What this does to the fork the stop raises

The stop's second question — *is the terminal branch required to emit or route
through the existing response owner?* — **is not mine to choose, and under this
rule it stops being a choice.**

Build the refusal first. Then the terminal branch tells you which it is:

- If `edge.destination().defining_call_identity()` **is** the response-owner
  call, the terminal branch is carrying an edge for a call it does not emit.
  The row was attached where it does not belong, and the question becomes where
  the attachment came from — not how to feed it.
- If it **is** the terminal transport call, then the edge's own caller cut is
  wrong for that call, and the cut is what to re-derive — the cut is a property
  of a call that completed a prefix (`responses.rs:318-322`: *"which
  already-selected source exit that call completed and which caller suffix
  remains live"*), and the terminal call completed nothing.

**Report which, before repairing either.** Both readings are consistent with
everything measured so far, and I am not going to pick between them from a
mechanism story — that is how amendments 1 and 2 were each wrong.

This also explains, without a new hypothesis, why whole-chain `[699, 661]` was
falsified alongside edge-only `[661]`: if the raw word is not the before-value
of *this* consumer at all, then **no prefix of the chain will fit it**, and the
two failures are one fact rather than two data points.

## One thing to state about the generated-entry population

The stop reports feeding the selected suffix into the Q2 generated-entry
population, *"without this, emission honestly refuses at pending call
`(665,664,663)` because detached `consumers()` is empty"*.

Amendment 4's erratum established that `consumers()` is empty by construction on
every detached row and the executable input is the sliced suffix, so a suffix
feed is very likely the ruled spelling. **But an honest refusal that stops
firing because we populated the thing it checks is the exact shape amendment 4
pre-committed to rule against.** So state it plainly in the next return: is the
population derived from the edge, or supplied to satisfy the refusal? If the
former, say from what. If you cannot tell, that is the answer and it is a stop.

## Controls

1. **The refusal fires** — a control at the terminal site where the emitted
   transport's call identity differs from `edge.destination()
   .defining_call_identity()`, asserting the **exact** error text and **both**
   identities. A refusal control that only asserts "it refused" passes for any
   reason and is not evidence.
2. **The paired positive** — the same construction where the identities match
   lowers exactly as today. Without this the design's plausible failure —
   over-tightening so the active site at `core.rs:6982` stops lowering — has
   nothing watching it.
3. **Non-regression at the active site.** `core.rs:6982` works today; it is the
   one site where the pairing is believed sound, and it is the control that says
   so rather than assuming it.
4. **Both fixtures.** `px8f_write_partition` and `px8f_write_all_native` are
   homologous but distinct artifacts; no identity proof transfers between them.
5. **Retained green** — every grid and mutation control standing at `01d2ccb11`
   and `5d977ac79`, and the px8f pre-object refusal still honest.

## Review tells

- **If the repair adds a case to the terminal branch at `core.rs:6864` rather
  than changing what `lower_checked_ih_detached_required_consumer_edge`
  accepts, it is the withheld point repair with extra machinery.** The whole
  content of this amendment is that the *function's signature* is the defect.
- **If the mismatch is resolved by recomputing the caller cut at the terminal so
  that `[661]` becomes `[699, 661]`, that is guessing past the missing owner
  handoff** — the ring already named and declined this, and I am recording the
  decline as correct so it is not revisited under pressure.
- **If step 1 is implemented as a check that returns an error and nothing else
  changes, the design is half-built.** The check is the floor. Obtaining the
  before-value from the edge is the closure.

## Not authorized

No ABI, schema, frame, owner key, tag, route-to-runtime, stack or bound change.
No new phase mechanism, runtime carrier or payload tag. No weakening of the
pre-object refusal. No revert of the entry-18 verifier repair. **Amendment 1's
application-seat ruling stays SUSPENDED — this amendment does not re-rule it**,
and nothing here authorizes relocating emission ownership. R1 and R2 remain
should-fix on the candidate. `3e8cc3c3d` is not a candidate and nothing here
promotes it. `source.rs` stays blob `38ec787dac261f02d46463ee1e9fca555c76d694`.

## Symptom inventory — continuation of entry 20

To be folded by the Steward without rewriting history:

> HS7's edge-selected suffix is built and emits, and HS8 (`evt_35n0d5ehaf3ex`,
> WIP `3e8cc3c3d`, not a candidate) traps at frame 2's default because the
> before-value handed to the consumer is the terminal environment-transport
> call's raw `ITree::Vis`, while the edge names a different defining call. The
> advisory's step 1 — the exact finished callee and Result-load edge establish
> the raw identity — was never made enforceable:
> `lower_checked_ih_detached_required_consumer_edge` takes the before-value as
> an independent parameter at both of its two call sites (`core.rs:6864`,
> `core.rs:6982`) and never compares it to
> `edge.destination().defining_call_identity()`, while the one guard present,
> `continuation_candidate_is_consumed`, is a ledger query about the plan rather
> than about the operand. Keyed on: **an input of the pairing supplied
> independently and then certified** — the same predicate as entries 12-17, 19
> and amendments 1-3, relocated to the last unbound input.

---

## Fork resolved — arm 2, and the caller cut must NOT be constructed

Added 2026-09-14, after the ring built the refusal and returned the
classification (`evt_2bj4w3e2kc6sp`, WIP `c35111dc2`, not a candidate). Folded
into this amendment rather than issued as a sixth, because it answers the fork
this amendment posed and splitting it would scatter one ruling across two
documents.

**This is not hard stop 9.** The refusal was built, it did not fire, and it
produced exactly the classification it was designed to produce. The ring then
asked the representation question this amendment instructed it to ask. **A stop
I asked for is not a stop against the ruling** — the §1a count stays at **8**
and the next re-trigger remains the **9th**. Recording that so the count is not
inflated by a report I requested, and not deflated later either.

The ring's closure is the ruled shape: `lower_checked_ih_detached_required_
consumer_result` takes the single `ClaimedContinuationResult`, derives the edge
internally, compares owned identity to defining identity with both in the
refusal, and only then extracts the word. No independent before-value parameter
remains. That is step 1, built.

### The measurement answers the fork: neither option as posed

The question was whether the no-completed-prefix case is **(a)** an empty owner
prefix inside the existing cut, or **(b)** absence of a caller cut with a
separate edge derivation.

**Option (a) is not available, and the reason is in the cut's constructor.**
`CheckedIhDetachedCallerCut` (`responses.rs:241-250`) is definitionally a
**two-call** object. It carries `selecting_call`, and **both**
`producer_transport` and `caller_transport` as separate transports. Its builder
(`responses.rs:2246-2277`) takes every one of its caller-side fields from
`boundary.caller_context()` — `caller_result_origin` from
`caller_context().result_origin()`, `consumed_caller_suffix` from
`caller_context().caller_suffix()` — and `consumed_continuation_origin` is *the
continuation the caller consumed*.

At the terminal defining call **there is no caller context.** So an "empty owner
prefix" is not a degenerate value of an existing shape; **it requires
fabricating the caller** — a caller context, a caller transport, and a selecting
call for a call that nothing selected. That collapses producer and caller into
one call, which is the "one definition now carries two identities" failure
amendment 4 named, reintroduced in the object built to prevent it. It is also
the degenerate-endpoint trap: a two-endpoint relation reviewed only at the point
where its endpoints coincide reads as consistent precisely because the
discriminator has been removed.

**Option (b) is right in substance and wrong as posed.** There is no caller cut
at this call, because there is no caller. But *"absence of a cut plus a separate
edge derivation"* puts the decision back at the site: something must notice the
`None` and choose a derivation, and a site that chooses is the open input this
whole amendment exists to close. An `Option::None` that downstream code defaults
on is the fourth input arriving from outside wearing a new spelling.

### The authorized representation

**The executable suffix is selected by an explicit two-armed relation, minted
with the edge, never by the presence or absence of a cut.**

- **Caller-completed arm** — a caller cut exists, carrying its two transports
  and its caller context. Suffix is the exits at and after the caller's first
  exit, exactly as `checked_ih_post_call_caller_exit_index` derives today.
- **Self-defining arm** — one call, no caller, no cut, and **the suffix is the
  FULL selected-case-exit chain**, because a call that completed nothing leaves
  the whole chain live.

Both arms carry their own evidence, and the arm is a property of the row, fixed
where the defining call is named. **No lowering site may select the arm**, and
neither arm is reachable by defaulting on the other's absence. The relation
should not be representable without one arm or the other being chosen by its
minter — the same discipline amendment 3 applied to the destination and this
amendment applies to the before-value.

### The falsification of the full chain is VOID — re-run it

**This is the part not to skip.** The self-defining arm predicts the suffix at
this call is `[699/frame3, 661/frame2]` — the whole chain. That is precisely
what the eighth stop reported as already falsified: *"whole-chain execution
`[699,661]` was already falsified: it reaches the same Result default after
replaying owner work."*

**That evidence does not transfer, and the ring's own reasoning is what voids
it.** The return states that the generated-entry population is *"a faithful
derivation from a false premise and must be recomputed after the cut is
corrected."* The `[699,661]` run was made on the same tree, with the same false
cut attached and the population derived from it. **The identical argument
applies: it is a faithful execution of a program built from a false premise.**

So the full chain has **not** been tested against the corrected representation.
It has been tested against a program that removed frame 699 by a cut now proven
false. Re-run it after the cut is gone, and report the result as new evidence.

**If the full chain still fails with no cut attached and the population
recomputed, that is a genuine hard stop and it is the 9th** — at which point I
hold the ruling and call the research advisory before ruling again, per the
standing trigger. Say so plainly if it happens; do not attempt a third
representation.

### On the population answer

The answer given is the right one and it was answered in the right direction:
the population is derived from the edge through
`required_consumer_executable_suffix()`, not inserted to silence the refusal,
and it is a faithful derivation from a premise now known false. **Recomputing it
rather than preserving it is correct and is not optional** — a population that
happens to be green under a corrected cut, but was derived under the false one,
is green by coincidence and nothing would report that.

### Not authorized, restated for this arm

Fabricating a caller context, caller transport, or selecting call for a call
that nothing selected is **not** authorized, in any spelling, including a
sentinel or an "empty" caller. If the self-defining arm cannot be built without
one, that is a stop and I want it back — not a synthesized caller.
