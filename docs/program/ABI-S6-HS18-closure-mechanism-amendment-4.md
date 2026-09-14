# ABI-S6 HS18 — closure mechanism, amendment 4

Architect, 2026-09-14. Extends the
[closure mechanism](ABI-S6-HS18-closure-mechanism.md) and
[amendment 3](ABI-S6-HS18-closure-mechanism-amendment-3.md) on the ring's sixth
hard stop.

**This amendment does NOT supersede amendment 3. Amendment 3's rule stands
exactly as written and remains current authority.** Amendment 3 closed what the
planner may CONSTRUCT. It said nothing about what lowering must REALIZE, and
that is the half HS6 escaped through. Read the two together: amendment 3 is the
construction half of one closure, this is the realization half.

Issued after the §1a hold at the sixth hard stop: the ruling was held, a scoped
research prior-art advisory was called and returned (`evt_2k4w4qfasxxfq`,
`evt_31jk5cggky8ar`), and the ruling was issued with it in hand
(`evt_2tcb055fpy7ce`). Grounded against `44d50af25`, which is **not a
candidate** — it is CI-red and its merge Decision is void. Every line number
below is at that SHA.

## What the advisory settled, and what it refused

The fork I posed — does one relation carry both proven identities, or is a
second reified transport object required — came back **false as posed**.

- **A distinct semantic/IR transition is MANDATORY.** A distinct runtime
  carrier or a second planner object is **not**.
- One relation may carry both identities **iff** it is path-valued **and
  lowering consumes it to emit the consumer**. The admissible shape is
  `raw call definition -> exact consuming occurrence + incoming edge ->
  distinct final definition + outgoing edge`. It may **never** say that one raw
  call-result definition holds both identities.
- The static discriminator **is** decidable, when four steps close: the exact
  finished callee and Result-load edge establish the raw identity; the exact
  source consumer accepts that raw definition on a named incoming edge;
  lowering the consumer establishes a distinct after-definition or a Trap; and
  every path from that definition to the final obligation preserves the result.
- **Demanded identity, declared contract, equal runtime shape, status and Trap
  checks, and the runtime tag do not establish steps two or three.** In the
  advisory's words, they are "expectations or observations, not provenance."

That last sentence is the whole of this amendment. The defect site does
precisely the four things it names, and nothing else.

## Both arms of my own fork are refused

**Arm (b), a second metadata object, builds what already exists.**
`CheckedIhPostCallConsumer` (`responses.rs:415`) already stores the actual
result identity, the demanded result identity, the ordered `consumers`, the
ordered `selected_case_exits`, and the detached return context.
`build_checked_ih_post_call_consumers` (`responses.rs:1971`) already derives
actual and demanded independently and pairs the row to the exact required
consumer call. `RequiredConsumerDestination` (`aggregates.rs:234`) is already
paired with the exact defining transport and source consumer occurrence — that
pairing is amendment 3, landed. A second object that does not change the
def-use fact **duplicates the gap** rather than closing it.

**Arm (a), splitting the publication before Q2's pending obligation, is
refused outright.** Q2's `calls.rs` change attaches `target.result_contract` to
the raw `result_word` as a *pending* obligation rather than minting false
authority. That is correct, and it is the honest refusal. Splitting it recovers
green by giving back the refusal, which is the loosening the Steward's M-gate
exists to catch and which I pre-committed to rule against.

## The defect, in the code's own terms

The relation reaches lowering and is consumed at three sites. **Two of them
emit. One validates.** The fork between them is a single predicate.

| site | guard | what it does |
|---|---|---|
| `core.rs:7712` | any consumer | strips `selected_case_exits()` as a prefix, composes the residual — **produces a value** |
| `core.rs:9172` | `detached_return_context().is_none()` | calls `realize_checked_ih_post_call_steps` on `consumer.consumers()` — **produces a value** |
| `core.rs:6863`, `core.rs:6963` | `detached_return_context().is_some()` | calls `validate_checked_ih_detached_result_shape` — **produces no value** |

**The emitter and the validator are adjacent function definitions in the same
file** — `validate_checked_ih_detached_result_shape` ends at `core.rs:9387` and
`realize_checked_ih_post_call_steps` begins at `core.rs:9389` — **and the call
sites fork between them on whether the return context is detached.**
`realize_checked_ih_post_call_steps` (`core.rs:9389`) takes the consumer steps,
resolves each occurrence to its source computational match, builds an
eliminator frame per step, and composes a value through
`lower_computational_match_value_composed`. That is exactly the emitter this
ruling requires, and it already exists, already takes `consumer.consumers()`,
and is already reached on the non-detached branch.

**Detachment is not a reason to stop lowering the consumer.** It is a statement
about where the return context lives, and amendment 3 already settled that the
seat is derived rather than declared.

### What the validator does instead

`validate_checked_ih_detached_result_shape` (`core.rs:9336`) takes the raw
carried word, emits a tag query on it, requires that tag to equal
`demanded_result_identity()`, then finds the pending obligation **for that same
raw word** and writes the demanded identity onto it:

```text
let word = <the raw carried call Result>;
let actual = emit_carrier_tag(word);
require_i64(actual, demanded_result_identity().tag_abi_word());
let obligation = pending_call_result_obligations
    .find(|o| o.result_word == word.word)?;
match obligation.identity {
    Some(identity) if identity != demanded => Err(...),
    Some(_) => {}
    None => obligation.identity = Some(demanded),
}
obligation.realization_required = true;
```

It returns `Ok(())`. It creates no value, and the caller returns the original
answer unchanged.

**One definition now carries two identities** — its own, and the demanded
after-identity written onto its obligation. That is precisely the shape
amendment 3 prohibited, one level down from where amendment 3 prohibited it.

### The site's own doc comment says it is not authority

Verbatim, in the tree, on the function that does the above:

> "Guard a detached call result before a generated function may publish it
> under its Result contract. This proves only the demanded outer shape; **it
> neither mints nor applies a source-cut receipt.**"

And inline, three lines above the assignment:

> "Runtime shape validation is not finished compiler authority. It records a
> pending obligation on the exact call/load already emitted; the staged
> callee/body proof must discharge that obligation later."

**The function that declares itself not to be authority is the function that
assigns the identity.** This is the second time in this arc that the defect
site carries a doc comment stating the exact limitation the code then exceeds —
amendment 3 found `CheckedIhGeneratedEntryCoordinate`'s comment saying
source-call identity "is a class member, not a discriminator," beside code that
used it as one. Two instances is a pattern worth naming: **in this subsystem, a
doc comment disclaiming a capability is a live lead, not a reassurance.** Grep
them.

### How far it propagates

The identity is written onto the raw word's obligation, and `realization_required`
is set on that same obligation. `close_and_define_staged_result_bodies`
(`units.rs:3995`) then drives a fixed point over the staged bodies keyed on
exactly those obligations, collecting `obligation.result_word` into
`realized_call_words` and handing them to `prove_realized_value`.

So the downstream proof machinery is proving the demanded identity **of the raw
definition**, because that is the word the identity was attached to. px8f's
refusal — actual against demanded — is that machinery working correctly and
refusing an unproven claim. **The refusal is honest and must stay honest.**

## A correction to my own ruling's wording

My in-thread ruling said the repair must "execute `selected_case_exits`." That
is wrong about the mechanism, and the code is the authority.

`selected_case_exits()` is a **matching and residual key**, not an execution
list: `validate_checked_ih_consumed_active` (`core.rs:9275`) builds it into an
`expected` vector and compares it against the active frame suffix, and
`core.rs:7712` strips it as a prefix to compute a residual. The steps that are
**executed** are `consumer.consumers()`, via `realize_checked_ih_post_call_steps`.

⇒ **The detached path must lower `consumer.consumers()` — the same input the
non-detached branch already supplies to the existing emitter.** Read the rule
below with that spelling, not mine.

## The rule

**A final or demanded result identity must be minted by the function that
lowers the consuming occurrence, taking the after-definition it produced as a
required argument. It may never be assigned to the before-definition by a
function that validates the before-definition's shape.**

The invariant is unchanged from amendment 3, and that is the point:

```text
(exact before-value, exact consumer occurrence, exact incoming edge)
    -> (exact after-value, exact outgoing edge)
```

Amendment 3 closed the left-hand side: the planner may no longer name a
before-value by a class coordinate. **Nothing realized the right-hand side.**
On the detached branch lowering produces neither an after-value nor an outgoing
edge, and the arrow is discharged by declaration. A closure that constrains what
may be constructed, and leaves what must be emitted unconstrained, is closed on
one side of its own arrow.

## Representation

**The detached branch lowers the consumer.** Route it through the existing
emitter rather than a new one: `realize_checked_ih_post_call_steps` already
takes the ordered consumers and produces a composed value. The outcome is a
distinct after-definition, or a Trap. There is no third outcome — in
particular, "the shape already matched, so no new definition is needed" is the
defect restated.

**The identity becomes unmintable from the raw word.** The demanded identity
is carried by a private-field newtype minted only by the function that lowered
the consumer, taking the after-definition as a required argument — the same
discipline `RequiredConsumerDestination` already carries after amendment 3. A
mismatch check inside `validate_checked_ih_detached_result_shape` would be a
refusal wearing the closure's clothes, exactly as at every prior step of this
arc.

**No second metadata object is authorized.** The relation already carries every
input the emitter needs.

## The tell

**`validate_checked_ih_detached_result_shape` must LOSE the ability to
discharge a post-call obligation from the raw word.** If, after the repair, any
function can set `obligation.identity` or `obligation.realization_required`
without an after-definition in hand, the closure has not been built — whatever
else changed. Self-check against this before releasing a candidate; it is a grep,
not a judgment.

## Controls

Each must be able to fail.

1. **The emitter runs on the detached branch.** Assert the consumer's tag query
   executes and the after-definition exists — not that the route changed shape.
   Route-shape rows are not execution evidence; this has been control 6 since
   amendment 2 and it is the one that would have caught amendment 1.
2. **Mutation: skip or empty the lowered consumer steps.** Must go red with the
   **exact full reason string**, not a stage or field envelope. `ObjectEmission` /
   `checked_process_object` is the universal packaging envelope and discriminates
   nothing — an assertion on it passes for any backend error.
3. **BOTH px8f fixtures, and this is not optional.** `px8f_write_partition` and
   `px8f_write_all_native` are **homologous but distinct artifacts**. The `38`
   and `37` in `3380/38` and `3318/37` are the `len` field of a `DenseRange`
   (`planning/static_transition/semantic_ir.rs:390`) — a canonical-name length
   in a **per-artifact arena**, paired with a `start` offset into that same
   arena. Neither component is a **cross-artifact identifier**, so no identity
   proof transfers between them. A repair proven on the CI-red witness is
   **not** proven on the held one. Both fail at `funcid55`/`v77`; their sources
   differ.
4. **ANTI-LOOSENING.** A raw publication whose consumer transition is genuinely
   unproven must **still refuse**, and the px8f pre-object refusal must stay
   honest. This is the Steward's M-gate made mechanical: it fails on a candidate
   that recovers green by weakening the refusal rather than by emitting the
   consumer.
5. **Retained green.** Every amendment-3 control; the Q1 `source.rs` site
   preservation with addition-only, no-`_ =>`-wildcard match discipline; the
   Mapping suite at 18/0.
6. **QA scope must include all three px8f suites** — `ken-cli
   px8f_buffer_native`, `ken-verify px8f_write_partition`, `ken-elaborator
   px8f_buffer_io_surface`.

## Review tells

1. **A repair at the validator that keeps the validator's authority** — adding a
   check, a stricter comparison, or a second identity field to the obligation —
   is the withheld point repair with extra machinery. What this amendment
   authorizes is lowering the consumer on the detached branch and moving the
   mint.
2. **An after-definition that is the raw word renamed** satisfies the letter and
   not the rule. The advisory is explicit: equal runtime shape and a matching
   tag are observations, not provenance.
3. **A second planner or runtime object** is the refused arm (b) under a new
   name.

## Not authorized

No ABI, schema, frame, owner key, tag, route-to-runtime, stack or bound change.
**No runtime carrier and no runtime tag** — the advisory is explicit that the
reification belongs in compiler IR and control, not in the ABI. No weakening of
the pre-object refusal. No revert of Q2's `calls.rs` pending-obligation change.
No relocation of emission ownership. R1 and R2 remain should-fix on the
candidate.

## Scope: no recut

The missing work is at `core.rs:9336` — native emission — and the planner
relation needs nothing added. **The arc's scope stands; this is not a recut.**
The third branch I named before the advisory returned (no repair available
inside the authorized native-emission delta, which would have gone back to the
Steward as a recut rather than to the implementer as a repair) was **not
taken**.

## What this says about the predicate

Entry 21 is a new **witness**, not a new predicate. It shares the predicate of
entries 14, 16, 17 and 19, and sits closest to 16 and 17; entry 19 differs from
it only in whether the skipped application had been emitted at all.

My px8f prediction criterion was that the closure would retire px8f if the
required-consumer relation were **present** on those routes. Research established
that it is present and populated — and px8f is not retired. **The criterion was
wrong, not the prediction.** I tested the planner's data when the defect lives
in lowering's use of it. The predicate holds; the closure was incomplete, not
inapplicable. That is what this amendment repairs.
