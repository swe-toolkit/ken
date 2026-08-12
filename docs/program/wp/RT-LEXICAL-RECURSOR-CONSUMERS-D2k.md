# `RT-LEXICAL-RECURSOR-CONSUMERS` `D2k` — the `StaticWorkerBinding` wall

The five expressions that `D2a` advanced and left standing at a wall it filed
as *"successor, unfiled here"*. Architect ruling `evt_5wvk3e8k1bjqn`
(2026-08-12) placed them **inside `#6d`** — this is `D2`'s next increment, not
a new node, and **[[RT-CONTSRC-CALLABLE-CONTRACT]] is not a prerequisite.**

Fixed inputs measured at `main` **`b2ee3377`**. Re-derive them at your base;
a merge-base goes stale without your branch moving.

> # `D2k-1b-i` IS AN ACCEPTED CONSERVATION PARTIAL — 2026-08-12, Steward
> # `evt_5605tqyn8qzv3`. `AC-1` IS OPEN AND UNQUALIFIED.
>
> **Read this before `AC-1`, and do not read "conservation holds" as "the five
> are green".** Those are different claims and only the second closes the node.
>
> **What the candidate `11e4eae1` establishes.** The silent accept is closed:
> rows 4 and 5 **refuse** rather than compiling after dropping a worker, the
> ledger records no constructed field as dropped, and the forbidden fourth
> state is unreachable on this base. `739cfde3`'s per-row transition sentinel
> landed as a deliverable rather than staying on a `preserved/` ref. `value_at`
> is byte-identical to `65dc74a9` (`AC-2`), zero new `#[ignore]` and no tracker
> file touched (`AC-6`). Five Runtime paths, `+968/-127`, merge-base
> `65dc74a9`.
>
> **What it does NOT establish.** It discharges **no part of `AC-1`**. All five
> remain **unconsumed**. `AC-1` is not narrowed, not qualified by row, not
> advanced, and the node stays `active` after this lands.
>
> **The refusal is the RULED disposition, not a failure.** Architect
> `evt_5etamwj8tp2fh` stated this outcome in advance: an unreachable pairing
> leaves the row red with the route gap reported, and it does not become a
> lawful drop. The ring paired by planner-owned origin/position, found no
> pairing, refused, and reported. **The increment worked.**
>
> ## The residual is a MEASURED ROUTE GAP, and it is an OPEN QUESTION
>
> | row | the elimination consumes | the worker-bearing occurrence |
> |---|---|---|
> | 4 | ordinary `PX8JScopeTree::Node` built at origin **31**, eliminated at origin 5 | direct-descent, origin **26**, **no elimination** |
> | 5 | analogous, origin **22** | origin **27**, **no elimination** |
>
> **On the excluded lane there are two constructor occurrences, and the one
> carrying the worker is not the one anything eliminates.**
>
> **This section states remaining work, in the future tense, on purpose.** An
> accepted partial's danger is that the node's prose silently becomes a claim
> about the past — a later reader takes "the ring measured the gap" for "the
> gap was closed". It was not. **Nothing below has been done.**
>
> Whether that unpaired occurrence is a **lowering-route defect** on the
> excluded lane or something the lane **legitimately produces** is a mechanism
> question routed to the Architect at `evt_45j489cd5m36w` and **not yet
> answered**. The two answers cut different successors — a route repair flips
> two rows from refuse to consume, whereas legitimate production means `AC-1`
> cannot be met for rows 4/5 by any route work and the criterion itself needs
> attention. **No successor is framed until that ruling lands**, and no seat
> may start one.
>
> **Excluded from this candidate and from any review of it:** changing the
> excluded lane's lowering route to close the pairing. That is the residual,
> not a repair to reach for in order to turn `AC-1` green before merge.

## 1. What this increment owns

| cell | expressions | wall |
|---|---|---|
| row 1 | 1 | `StaticWorkerBinding` |
| row 4 (`scope_segments` depth 1, 2, 3) | 3 | `StaticWorkerBinding` |
| row 5 **after**-hole | 1 | `StaticWorkerBinding` |

**Five of `#6d`'s six remaining expressions.** Row 3's singular-specialization
wall is **not** this increment. Row 2 (`RT-LEXICAL-ROW2-MISSING-MINT`) and row
5's **before**-hole (`RT-LEXICAL-R3-FUSION-EMITTER`) have left `#6d` entirely.

`#6d` closure gates [[RT-RECURSOR-TRANSPORT]] `D3`, which gates
[[RT-DESCENT-RETIRE]].

## 2. Fixed inputs — the wall is one chokepoint, and it is deliberate

`LoweringEnvironmentBinding::value_at` at
`crates/ken-runtime/src/cranelift_backend/lowering/mod.rs:3494`:

```rust
LoweringEnvironmentBinding::StaticWorker(_) => Err(unsupported(
    "StaticWorkerBinding",
    format!(
        "{edge} is a value-producing position and a static worker binding has no \
         value representation; its only admissible use is as the callee of a call \
         with an exact Var callee"
    ),
)),
```

- The match is **exhaustive with no wildcard**, and its doc says why: *"a future
  third arm is a compile error at every value-producing read rather than a
  silent escape."* **That property is an asset of this increment, not an
  obstacle to it.**
- **`edge` names the call site in the diagnostic**, so the refusal text
  identifies *which* value-producing read was taken. `D2a`'s recorded wall
  carries edge `"a Var in value position"`.
- **`value_at` has exactly four callers** at `b2ee3377`: `core.rs:6200`
  (*"a source-machine Var in value position"*), `core.rs:11140` (*"a
  continuation capture input"*), `core.rs:14593` (*"a Var in value position"*),
  and `mod.rs:3661`. **Count them again at your base** — a fifth caller
  appearing is a scope signal, not a detail.
- The representation itself is landed and closed:
  `LoweringEnvironmentBinding::StaticWorker(StaticWorkerBinding)` at
  `mod.rs:3198`.

## 3. The design judgment, front-loaded — do not re-derive it

> ### MEASURED FALSE FOR THESE FIVE — 2026-08-12. Do not act on this section.
>
> **The premise below is that the lawful consumer is already installed and the
> binding merely reached the wrong consumer *shape*. For these five there is no
> call at all.** The causal consumer owner is `RuntimeExpr::Construct` —
> `PX8JTree1::Node`, `PX8JScopeTree::Node` at three depths, `PX8JHoleOutput::Node`
> — so **the static worker is a constructor argument**, a value-producing
> position **by construction**, and the exact-`Var` callee path has nothing to
> key on. Measured by the Runtime implementer at `evt_6z8xjk3gkh821`; stop
> report `evt_134atze90gs1m`.
>
> **This is why "do not re-derive it" is dangerous here rather than merely
> stale.** That instruction exists to stop a ring re-litigating a settled call,
> and it works by discouraging exactly the check that would have caught this.
> **A front-loaded judgment carried into a frame is only as good as the
> population it was measured on.**
>
> **The section below stands only as the record of what was believed, not as an
> instruction.** `AC-1` is materially false for the same reason and is replaced
> in section 6.
>
> ### THE RULED LAWFUL CLASS — Architect `evt_4krvq67427n5z`
>
> **A compiler-only static constructor-template field** that preserves a
> `StaticWorkerBinding` until a later **static constructor elimination** rebinds
> that field and the **existing exact-`Var` call arm** consumes it.
>
> **Why the source shape is not the defect.** A source constructor argument is
> syntactically a value position, but a specialized `Lowered::Constructor` is a
> **compiler template, not necessarily a materialized runtime aggregate.**
> Today its `args: Vec<Lowered>` bakes in the assumption that every statically
> eliminated field is an ordinary value; **the five rows expose the missing
> compiler-only field distinction.** The planning side already describes this
> exact realization — one closure occurrence realized as a `StaticWorkerBinding`
> stored into the producer constructor, later becoming the worker call.
> **Rejecting that source shape would contradict the planned population and
> remove the recursive callable transport these fixtures measure.**
>
> **Three classes ruled WRONG — do not re-propose:**
>
> 1. **Not a deferred application.** There is no application at the constructor
>    occurrence; the constructor transports the recursive callable through a
>    case binder, and the later `Call { callee: Var(..) }` is the application.
>    Inventing a call earlier changes evaluation and source semantics **and
>    would require the new planner population the stop forbids.**
> 2. **Not [[RT-CONTSRC-CALLABLE-CONTRACT]].** No continuation-source
>    projection is involved. That node needs planner-owned callable identity on
>    the projection surface; `StaticWorkerBinding` deliberately carries none.
>    **Adjacent precedent, not that component.**
> 3. **Not a runtime materialization.** No carrier word, ABI slot, tag,
>    descriptor, environment pointer, callable identity, or planner-owned
>    storage.
>
> **The admissible representation is a CLOSED compiler-only constructor-field
> distinction, narrower than `LoweringOperand`:** an ordinary specialized field
> versus a static-worker field. **The Architect ruled the semantic boundary, not
> a type name or enum layout.** Required structural properties:
>
> - **`Construct` recognizes the static-worker binding BEFORE `value_at`** and
>   retains it as a compiler-only field; every other constructor argument
>   follows the existing path unchanged.
> - **Static `Match` elimination installs each field into the one lexical
>   binding authority without erasing its kind:** ordinary →
>   `Value(Specialized(..))`; worker → the same `StaticWorker`. **The later
>   exact-`Var` call remains the sole callable consumer.**
> - **Direct descent and the source machine implement the same distinction.**
>   One path may not preserve the worker while the other calls `specialized_at`
>   and refuses.
> - **Any constructor containing a worker field is NON-MATERIALIZABLE.** A path
>   that would carry, allocate, store, join, project, return or publish it must
>   refuse in **whole-graph preflight, before the first allocation or emitted
>   transfer.** It may not descend partway and then refuse.
> - **`value_at` remains byte-identical and exhaustive.** A worker used as an
>   ordinary field value, result, scrutinee, primitive/effect argument, or call
>   argument still refuses.
> - **No wildcard/default conversion and no broad
>   `LoweringEnvironmentBinding`-as-constructor-payload arm** — that would admit
>   `Carried` values into a template and widen the contract beyond the measured
>   need.
>
> **This is broader than "one before-guard owner repair."** It necessarily
> couples the `Construct` producer, the static `Match` binder, **both** lowering
> engines, and boundary preflight.

**Architect `evt_5wvk3e8k1bjqn`, and this is the whole reason the increment is
small.** At this wall the callable fact is **already expressible and already
installed**. Both the direct lowerer and the source machine already have the
exact lawful consumer: **a `Call` whose callee is an exact `Var` bound to
`StaticWorker`.** Every value-producing use routes through `value_at` and
deliberately refuses.

⇒ **The measured wall says the binding reached the wrong CONSUMER SHAPE — a
bare `Var` value read — not that this component lacks vocabulary for "static
callable, no value carrier."**

**The repair boundary, stated once:** consume the already-represented
static-worker binding **at its owning lexical-recursion consumer, before the
value guard**, while preserving the guard everywhere else. That is exactly
`#6d` `D2`'s standing *consume at the owner before downstream guards*
responsibility.

**Why `RT-CONTSRC-CALLABLE-CONTRACT` is a different repair.** It closes a
planner/projection expressibility gap: `ContinuationSourceSlotAuthority` can
describe only a value source and cannot state a callable source carrying
planner-owned callable identity. Its own frame warns that the adjacent lowering
sums are **precedent, not one component**, and that `StaticWorkerBinding` is not
the continuation-source contract. **The node stays real and `ready`; it is not
on the retirement path by virtue of these five walls.**

## 4. THE TRAP — the shared refusal string is not a shared root

**Architect limit 1, and it is deliberately deliverable-shaped.** All five
expressions report the same sentence. **That is not evidence they have one
causal root**, and the campaign has already paid once for generalizing from a
population read at one member.

The refusal is emitted by **one chokepoint that every value read funnels
through**, so a common string is exactly what five *unrelated* wrong-consumer
routes would also produce. The discriminator is the `edge` argument and the
causal consumer owner, not the message.

⇒ **`D2k-0` exists to settle this before any repair is designed**, and a repair
sized against an unmeasured "they are all the same" is the failure this frame is
written to prevent.

## 5. Deliverables

**`D2k-0` — re-derive the five, and prove their consumer owners and routes.**
For each of the five expressions: the exact refusal with its `edge`, the causal
consumer owner, and the route that reached the value read. **Commit the table.**
Then state, as a measured conclusion rather than an assumption, how many
distinct roots the five have. **No repair in this deliverable.** If the answer
is more than one root, post it and stop for a sizing call — that is a good
outcome, not a failure.

> ### `D2k-1` IS RE-SPECIFIED — 2026-08-12, Architect `evt_4krvq67427n5z`
>
> **The old `D2k-1` — "the repair, at the owning consumer, before the guard" —
> is withdrawn. Its premise is measured false** (section 3). Do not reconstruct
> it from a memory of this frame.

**`D2k-1a` — the message-independent evidence. AUTHORIZED NOW, evidence only.**
Commit the per-expression `RuntimeExpr::Construct` owner and the caller tags
excluding the forwarding caller, and **execute the claimed edge/refusal redness
control** so that property becomes executable rather than inherited prose.
`value_at` stays byte-identical. **This makes `D2k-0`'s "one root" durable; it
does NOT authorize the repair.**

> **BINDING ON THE EDIT — every component compared against a LITERAL, none
> against the population.** Adversary `evt_3xn73gyttdm5g`. The existing expected
> side is a literal anchor (`"StaticWorkerBinding"`, `"a Var in value
> position"`), and **that is why the redness property holds in all three
> directions** — one row moving, **all five moving uniformly**, and a row that
> starts lowering returning `None` against `Some(..)`.
>
> ⇒ **Add the owner as *"each owner equals `RuntimeExpr::Construct`"*, NEVER as
> *"all five owners agree."*** A sameness check across the population is **green
> under a uniform move**, which is the case that matters most and the one the
> current assertion deliberately avoids. Landing it that way makes the control
> **mixed** — two axes that red on a uniform change and one that does not, with
> nothing marking the difference.
>
> **Preserve the labelled-pair comparison too**: failures name *which row*
> moved rather than reporting a tuple mismatch.

**`D2k-1b` — the closed compiler-only constructor-field distinction**, per
section 3's ruled structural properties. The coupling is `Construct` producer,
static `Match` binder, both lowering engines, and boundary preflight — more
than one turn. **Cut into the three increments below**, and the cut is not free
to fall anywhere.

> ### The producer and the preflight are ONE increment, and that is forced
>
> **A producer without a preflight is the shape the ruling forbids.** Land
> `Construct`'s recognition first and a constructor carrying a worker field
> exists on `main` with no whole-graph refusal in front of it: it descends
> until something reads the field. That is *"descends partway and then
> refuses"*, verbatim.
>
> **A preflight without a producer is inert.** Nothing can build the field it
> refuses, so it compiles green, exercises nothing, and its control passes
> vacuously.
>
> ⇒ **Neither ordering is available. `1b-i` carries both.**

> ### SIZED AGAIN on measured blast radius — 2026-08-12. `1b-i` is now TWO.
>
> `1b-i` was handed back **unstarted on capacity**, with grounding
> (`evt_12smbvsmxxk9d`): `Lowered::Constructor` is mentioned at **87 sites**,
> **9 of which read `args`**. Changing that element type touches those plus
> every construction site, and section 3 forbids a wildcard or default arm —
> so **each reader is an explicit decision, not a mechanical edit.**
>
> **The nine was a lower bound, and the real figure is 22.** When the type
> actually changed, the compiler enumerated **22 reader decision points** — 9 in
> `core.rs`, 10 reached in `mod.rs` through accessors, and 3 more that handle
> the kind without refusing (`evt_5qejxwewrhz8`). A pattern-bind grep answers
> *who names the field*; the question is *who depends on its element type*, and
> those differ by everything routed through a signature. **The split was right
> and the number that justified it was low by a factor of two** — recorded
> because it is the input any future sizing of this shape will reach for.
>
> **The atomicity ruling is untouched. What moves is the type migration.** The
> bulk of the work is introducing the two-variant field type; the part that
> must be atomic is *arming* it. Those are separable, and separating them does
> not put the forbidden shape on `main` — because in `1b-i0` **no site
> constructs the worker variant at all**, so no constructor can carry one.
>
> **`1b-i0` claims nothing about worker fields and carries no control asserting
> one.** Its acceptance is *behaviour is unchanged*. That is what keeps it from
> being the inert-preflight half the note above rejects: it is not a boundary
> landed early, it is a type landed early.

**`D2k-1b-i0` — the two-variant field type, with the worker variant
UNCONSTRUCTIBLE.** Change `Lowered::Constructor`'s `args` element type from
`Lowered` to the closed compiler-only field kind — an ordinary specialized
field versus a static-worker field. **Every construction site produces the
ordinary variant; nothing produces the worker variant.** Each of the nine
`args` readers gets an explicit worker arm, because section 3 forbids a
wildcard or default.

- *Acceptance:* behaviour is unchanged and the targeted suite is green. **No
  new claim about static workers, and no control asserting one** — there is
  nothing yet to assert it against.
- **The nine arms are a type-completeness obligation, NOT the boundary.** They
  are local, per-reader refusals; the ruling requires a **whole-graph** refusal
  *before the first allocation or emitted transfer*. **Do not let their
  existence read as the preflight being done** — a per-reader refusal reached
  during descent is precisely *"descends partway and then refuses"*. `1b-i`
  still owes the boundary, ahead of them.
- *Hard stop:* the split forces some construction site to produce the worker
  variant. Then the two are not separable after all — say so and hand back.

**`D2k-1b-i` — RECUT 2026-08-12. Producer, preflight and consumer are ONE
atomic increment.** The node previously stopped at producer plus preflight.
**That cut was built and measured, and it is not publishable** — the block
below records why. `D2k-1b-ii` is folded in here and is no longer a separate
increment.

Have `Construct` recognize a `StaticWorker` binding **before** `value_at` and
produce the worker variant; land the whole-graph preflight; **and** land the
static `Match` elimination that installs each field into the one lexical
binding authority without erasing kind, so the **existing** exact-`Var` call
arm consumes it.

- **State the property as a TOTAL over the DISPOSITION SPACE, never as a list
  of forbidden verbs.** Architect ruling `evt_5etamwj8tp2fh`: **the invariant is
  conservation of every compiler-only static-worker occurrence**, and
  exact-`Var` is a *terminal disposition*, not the invariant. Every recognized
  worker at constructor field `(owner_origin, position)` receives **exactly
  one** disposition before any runtime-value boundary:

  > **each recognized worker is consumed exactly once, erased before
  > construction under positive unobservability authority, or refused before
  > emission; none is dropped.**

  1. **Consume** — a static elimination of that exact constructor field rebinds
     the same `StaticWorkerBinding`, and an existing exact-`Var` callee call
     consumes it exactly once.
  2. **Erase as proven unobservable** — lawful **only** under a whole-graph,
     origin-keyed proof that this exact field cannot be destructured,
     projected, carried, joined, returned or published, with the source operand
     already the effect-free lookup of the existing static binding. **Erasure
     happens at or before construction: no `ConstructorField::StaticWorker` may
     be built and then ignored.** It is not transport and earns **no
     "consumed" credit**.
  3. **Refuse** — neither proof exists, so compilation refuses before
     allocation or emission. An undestructured constructor that can escape
     still denotes a value containing the callable, and with runtime
     representation excluded, silently omitting the field is **unsound**.

  **The forbidden fourth state is what `739cfde3` produced: constructed,
  neither consumed nor authoritatively erased, then forgotten.**

  The previous wording enumerated carry, allocate, store, join, project, return
  and publish: **every way a worker could be USED, and not one of the ways it
  could be LOST.** A dropped field satisfies an enumeration of uses vacuously,
  which is exactly how four rows went green.

- **"Zero destructures observed" is NOT disposition 2.** Absence from a
  lowering trace proves only that *the current route* did not consume the
  field. Erasure requires **positive** non-observation authority **plus** a
  mutation that makes the field observable and flips the disposition to consume
  or refuse. **An absence is not an authority.**
- **A row that COMPILES is not a row that PASSES.** Per row, record the
  static-worker field, the bare-`Var` worker reads, **and the consumption**. A
  row that compiles with zero consumptions is a **failure**, and the handback
  must read it that way. This is the check that would have caught the recut,
  and the implementer already built it at `739cfde3` — land it rather than
  rebuild it.
- **Do not assert where the refusal originates** until measured; preflight or
  the field's first reader both satisfy the ruling.
- *Controls:* the `D2k-2` escape-mutation row (a constructor requiring runtime
  transfer refuses **before** allocation or emission), the `D2k-2` positive row
  for the rows this engine covers, the phase mutation (forcing the field
  through ordinary value conversion reproduces the `StaticWorkerBinding`
  refusal), and `AC-2`'s empty diff on `value_at`.
- **Measure which engine the five route through and name it in the handback.**
  Do not assume direct descent. The other engine stays fail-closed for one more
  increment — a refusal, not a partial descent.
- **The three predecessor controls that go red must be re-derived one by one.**
  A control red *because the ruled semantics changed* and a control red
  *because the repair is incomplete* look identical in a suite count. Say which
  each is, per control, in the handback.
- *Hard stop:* the field cannot be recognized ahead of `value_at` without a
  wildcard/default arm or a `LoweringEnvironmentBinding`-as-payload arm.
  Section 3 forbids both — stop, do not widen.
- *Hard stop:* the atomic unit does not fit one turn. **Report it; do not split
  it back.** Any split that leaves a worker constructed and unconsumed
  reproduces the defect below. If you find a split that provably does not —
  the consumer landing inert ahead of the producer is the candidate, on the
  `1b-i0` pattern — **propose it to the Steward rather than taking it.**

> #### The producer-alone cut was MEASURED and it is a SILENT ACCEPT
>
> `739cfde3` is evidence, not a candidate (`evt_5h71ks63e71ma`). At base
> `04730469` it does everything the old cut asked: recognizes the worker ahead
> of `value_at`, produces `StaticWorker`, keeps `value_at` byte-identical, uses
> no wildcard/default or broad payload, and holds every prohibited-axis total
> at zero.
>
> **It also changes outcomes.** Row 1 advances to a distinct `NativeJoinPlanV1`
> refusal; **rows 4x3 and 5 compile successfully.** Every row records its
> static-worker field and zero bare-`Var` worker reads — **and no field is
> consumed.** The four green rows drop the worker rather than lawfully
> succeeding.
>
> ⇒ **This is the same defect that forced `1b-i` to be atomic, one level out.**
> `1b-i` was made atomic because a producer without a refusal admits a worker
> into emission. A producer plus a refusal, without the **consumer**, admits a
> worker into oblivion. Both are silent accepts; only the direction differs.
>
> **The old cut FORBADE its own discharge.** `1b-ii`'s static `Match`
> elimination is the seam that installs the field for the exact-`Var` call, and
> the cut banned it — so `1b-i` could not satisfy *"no consumer becomes green
> here"* by any means available inside its own scope. That is a frame defect,
> not a ring failure: the hard stop fired exactly as `§4b` intends.
>
> **Four green rows would have read as progress.** The arm count already had
> this trap named — type completeness is not the boundary — and this is its
> twin: **a green row is not a consumed field.**

> #### `1b-i0`'s discharges are INDUCTIVE on the premise `1b-i` deletes
>
> `1b-i0` is accepted as *behaviour is unchanged*, and it earns that honestly —
> **because nothing constructs the worker variant.** Three of its decisions were
> disclosed as resting on exactly that (`evt_5qejxwewrhz8`), each in the
> implementer's own words:
>
> - `emit_carrier_transfer` now reads its fields **before**
>   `emit_checked_aggregate_alloc`. *"Emitted instruction order is unchanged
>   today because the read cannot fail."*
> - `same_recursive_field_shapes` answers `false`, which *"cannot be wrong while
>   nothing constructs a worker."*
> - `unwrap_terminal_ret` hands the constructor back **intact** rather than
>   unwrapping it, on the ground that the read is infallible.
>
> **`1b-i`'s whole purpose is to make the worker variant constructible.** The
> instant it lands, every one of those three premises is false: the read *can*
> fail, a worker *can* be constructed, and the infallibility is gone. **Each
> becomes load-bearing rather than inert on the same commit.**
>
> ⇒ **Re-verify all three inside `1b-i`; do not inherit them.** They are not
> defects — the ordering move is the ruling's own *"refuse before you allocate"*
> shape, pre-positioned. **The hazard is that they were justified once, in
> writing, under a premise `1b-i` removes, and nothing in the diff will say so.**
> A reviewer reading `1b-i0`'s handback will find each one argued and settled.
>
> Concretely, `1b-i` must state: whether `emit_carrier_transfer`'s emitted
> instruction order is *still* unchanged once the read can fail; what
> `same_recursive_field_shapes` returning `false` licenses when a worker really
> can be present; and whether handing back an intact constructor is still right
> when the thing inside it has no value representation.
>
> #### ONE premise statement self-retires. The other four go false SILENTLY.
>
> Adversary `evt_6g3z0xm53cfaz` established the **asymmetry** and the count of
> **five** premise statements in `1b-i0`'s added lines. **The coordinates below
> are not its** — they were derived by grep at `8ab2813d`, and all five sit in
> `crates/ken-runtime/src/cranelift_backend/lowering/mod.rs`. The report's own
> `:136 / :275 / :323 / :377` are **offsets into a diff stream, not file
> lines** (`evt_5191zntrffvv3`), and its *"the premises live in `core.rs` and
> `mod.rs`"* was inherited from the merge's declared path list rather than
> measured. **Open the anchors below; do not chase the report's.**
>
> | line | what it asserts |
> |---|---|
> | `2872` | *"NOTHING CONSTRUCTS THIS ARM AT `D2k-1b-i0`, and the resulting `never constructed` warning is the open checkpoint staying visible"* |
> | `4551` | `d9_collect` — *"This walk is an infallible observation"* |
> | `16505` | `unwrap_terminal_ret` — *"This function is infallible, so the conservative move is to NOT see through the wrapper"* |
> | `19826` | `same_recursive_field_shapes` — *"`false` is the answer that cannot be wrong"* |
>
> plus `emit_carrier_transfer`'s ordering note.
>
> **The asymmetry is the finding.** When `1b-i` constructs the variant, the
> `never constructed` warning **disappears** — a real compiler-generated event.
> **The other four go false in place, with no warning, no red, and nothing
> naming them.** Three are one-line justifications inside otherwise unrelated
> functions, which is exactly where a behaviour-focused pass does not look.
>
> ⇒ **AC for `1b-i`, and it is greppable rather than a reminder: after `1b-i`,
> no comment in the changed files may still assert that nothing constructs the
> worker variant.** Each of the four is corrected or removed, and the handback
> states what each now says. A reviewer can check that by reading the four
> anchors above; *"we re-verified"* is not checkable and this is.
>
> **If the ring wants a cross-reference, `mod.rs:2872` is the place** — it is
> the one artifact whose obsolescence the compiler announces, so whoever closes
> the checkpoint is already reading it. **Put the obligation where the trigger
> fires, not only where the work is scheduled.** That is the Adversary's point
> and it is right; the AC above is the part that fails loudly if it is missed.
>
> **Do not re-derive whether anything constructs the variant by grep.** The
> `never constructed` warning is the compiler's total enumeration of exactly
> that question; a grep answers *who names the type*. That distinction is what
> turned this increment's `9` into `22`.

> ### The owner relation has a PLANNER-OWNED KEY. Use it; `D2k-1a` did not.
>
> Adversary `evt_7v6megkzsbapk` asked whether `D2k-1a`'s causal pairing is a
> same-event join or a positional one. **Settled, and it lands on a different
> component than the report named.**
>
> - **The refusing caller is same-event and sound.** `site` comes straight out
>   of the `StaticWorkerRead` event. No join, nothing to check.
> - **The OWNER is the positional one.** `control.rs:3323` binds `at` from
>   `.enumerate()`, then takes the nearest preceding `ConstructEntered` in
>   emission order. **Both events carry `origin: StaticOriginId` and both
>   discard it with `..`.**
>
> **That was disclosed and ruled, so it is not a defect in `D2k-1a`.** The
> resolved Decision states it in terms: *"the only remaining prefix lookup is
> for the enclosing `Construct`, not caller attribution."* Evidence-only work
> may rest on emission order.
>
> ⇒ **`1b-i` may not.** Here the producer/owner relation **is** the mechanism:
> recognizing the binding at `Construct` means knowing which constructor owns
> which argument. And the key already exists, planner-owned:
>
> ```rust
> // core.rs:12517 -- the child's origin is DERIVED from (parent, position)
> self.static_transition_plan.child_static_origin(parent, position)?
> ```
>
> So a `Construct` at origin `P` lowers its argument `i` at
> `child_static_origin(P, i)`. **Owner and argument are related by a planner
> fact, not by how close together they were emitted.**
>
> **Why emission order is not merely weaker here.** A constructor argument may
> itself be a `Construct`. Nearest-preceding then names the **inner**
> constructor, and nothing in the assertion notices — it is right on the five
> measured fixtures and silently wrong on a nested one. That is `D2k`'s own
> §3 failure repeating: a property measured true on the population it was
> measured on, carried forward as though it were general.
>
> **Key on `child_static_origin`, and do not infer the relation** — it is the
> planner's to state, which is the same prohibition section 3 applies to the
> consumer-identity alias, one level down.

**`D2k-1b-ii` — FOLDED into `D2k-1b-i` on 2026-08-12. It is not a separate
increment.** Its content — static `Match` elimination preserving the kind
(ordinary → `Value(Specialized(..))`, worker → the same `StaticWorker`), on the
engine the five actually route through, consumed by the **existing** exact-`Var`
call arm — is now a required part of `1b-i` above, with its controls and its
engine measurement. It was folded because producer plus preflight **without**
this consumer is a silent accept, measured at `739cfde3`.

> #### RULED 2026-08-12 (`evt_5etamwj8tp2fh`): rows 4 and 5 STAY, and the
> #### "never destructured" reading is a ROUTE finding, not a classification
>
> The implementer measured that rows 4 and 5 constructors are **never
> destructured on the excluded lane**. **That does not remove them from the
> five and does not license dropping their field.**
>
> **Their source graphs contain the opposite candidate**, visible at current
> `origin/main`:
>
> | row | source shape | test |
> |---|---|---|
> | 4 | constructs `PX8JScopeTree::Node(Var(0))`; the outer `ComputationalMatch` selects that `Node` and its selected body calls `Var(0)` | `px8j_scope_chain_observation_result` |
> | 5 | constructs `PX8JHoleOutput::Node(Var(0))`; the outer `ComputationalMatch` selects that `Node` and calls `Var(0)` | `px8j_equal_payload_hole_placement` |
>
> ⇒ *"Never destructured on the excluded lane"* is a **lowering-route
> finding**, not a source-semantic classification. The elimination exists in the
> source; the current route does not reach it.
>
> **Pair the recognized field to the later elimination by planner-owned
> origin/position — never by constructor spelling and never by trace
> proximity.** If that pairing cannot be reached, **the row stays RED and the
> route gap is reported. It does not become a lawful drop.**
>
> **`AC-1` is NOT qualified by row and rows 4/5 are NOT removed.** For these
> five, exact-`Var` remains the required positive endpoint unless an
> exact-occurrence non-observation proof is produced, and the positive control
> still expects the **consumed** arm.
>
> This ruling authorizes **no new representation and no new cut**. It tells the
> measurement what distinguishes a route bug from a genuinely dead field.

**`D2k-1b-iii` — route parity.** The second engine implements the identical
distinction; neither path may preserve the worker while the other calls
`specialized_at` and refuses. *Control:* the `D2k-2` route-parity row.

> **Every `1b` handback states the RUNNING TOTAL, not that increment's delta**
> — new planner population, continuation-source setters, ABI/carrier/descriptor
> entries, `#[ignore]`s and `value_at` callers, summed across `D2k-0`, `D2k-1a`
> and every `1b` increment landed so far. Section 8's stop is about an
> accumulating quantity, and a per-increment reading of it can never fail: that
> is how the parent node's identical stop sat silent through eleven partials.

**`D2k-2` — the successor controls**, separate from `D2k-1b` and additional to
the existing guard controls:

- **positive** — each of the five transports the worker through its constructor
  and later consumes it at the exact-`Var` call;
- **phase mutation** — forcing that field through ordinary value conversion
  reproduces the `StaticWorkerBinding` refusal;
- **escape mutation** — making the constructor require runtime transfer refuses
  **before** allocation or emission;
- **route parity** — direct descent and the source machine preserve and consume
  the same field kind;
- **non-aliasing** — ordinary constructor fields and ordinary exact-`Var` calls
  remain behaviorally unchanged;
- **census** — no new planner population, continuation-source setter,
  ABI/carrier/descriptor, `#[ignore]`, or `value_at` caller;
- **nested-owner** — a constructor whose argument is itself a `Construct`
  attributes the worker to the **owning** constructor, not the innermost one.
  This is the row that fails under an emission-order bridge and passes under
  the `child_static_origin` key, so it is what makes the `1b-i` note above
  executable rather than advisory. **If it cannot be constructed on these
  fixtures, say so and say why** — an unbuildable discriminating row is a
  finding, not an omission.

## 6. Acceptance criteria

**`AC-1` — the five are green** on the pre-retirement tree under `B`-only
exclusion, and each is green **because the worker is transported through its
constructor as a compiler-only field and consumed at the later exact-`Var`
call**, not because a guard stopped firing.

> **`AC-1` IS UNMET AND UNQUALIFIED as of `11e4eae1`.** All five are
> **unconsumed**; the accepted conservation partial discharges no part of this
> criterion. See the accepted-partial block at the head of this file for the
> measured route gap and the open Architect question. Do not read the partial's
> merge as progress against `AC-1`.

> **`AC-1` WAS REPLACED — 2026-08-12.** It previously read *"because its
> consumer routes to the exact-`Var` callee path"*, which is **materially
> false**: at the measured wall there is no call, and the constructor occurrence
> is not an application. The exact-`Var` call is still the sole callable
> consumer — but it happens **after** a static constructor elimination rebinds
> the field, not at the refusing site.

*Control:* the `D2k-2` positive and phase-mutation rows, plus the committed
`D2k-1a` owner evidence.

**`AC-2` — `value_at` is unchanged.** No third arm, no permissive
`StaticWorker` arm, no wildcard. *Control:* `git diff` on
`mod.rs:3494`-`3506` is empty. **If your repair requires editing `value_at`,
you have the wrong repair** — the guard is the thing being preserved.

**`AC-3` — no new runtime value representation for `StaticWorkerBinding`**: no
ABI slot, no planner population, no descriptor, no carrier. *Control:* name the
representation at each new crossing and show it is compiler-only.

**`AC-4` — exact-callee-only use is preserved and every value use still fails
closed**, including the four `value_at` callers not repaired. *Control:* a
committed negative witness per surviving caller, **each with a positive control
proving its path is reached** — a negative that passes because nothing arrived
is the defect this campaign keeps re-finding.

**`AC-5` — the five parent guards are intact**, unchanged from `#6d` `AC-3`.

**`AC-6` — zero new `#[ignore]`**, and no tracker `status:` change in the
candidate. *Control:* `git diff`.

**`AC-7` — CI green** on the merge. Not a local `--workspace` run
(`COORDINATION §12`).

## 7. Excluded scope

- **`ContinuationSourceSlotAuthority`**, and any claim that the
  [[RT-CONTSRC-CALLABLE-CONTRACT]] edge is closed. Architect limit 3.
- **Row 3's singular-specialization wall.** Same node, different increment.
- **Retirement, lane deletion, and the `AC-2b` dispositions.** Those are
  [[RT-RECURSOR-TRANSPORT]] and [[RT-DESCENT-RETIRE]].
- **Unwinding any landed `D2f` partial**, or touching
  [[RT-LEXICAL-R3-FUSION-EMITTER]]'s fusion machinery.

## 8. Stop conditions — return to the Steward, do not decide

- **Architect hard stop, verbatim in effect:** if any of the five can be
  repaired **only** by identifying or transporting its callee through the
  **continuation-source projection surface**, **stop that row** and return the
  measured dependency for a graph amendment. **Nothing currently grounded shows
  that condition** — the landed refusal occurs *after* the lexical static-worker
  binding already exists — so firing this stop is a real finding and must carry
  its measurement.
- **`D2k-0` returns more than one root.** Post the table and stop for sizing.
- **The repair cannot be expressed before the guard** without a signature change
  rippling beyond the owning consumer.

  > **THIS STOP FIRED — 2026-08-12, and it is answered, not armed.** The measured
  > `Construct` owner showed the repair needs the `Construct` producer, the
  > static `Match` binder, **both** lowering engines and boundary preflight.
  > Architect `evt_4krvq67427n5z` recut it into `D2k-1b`. **Do not re-read this
  > bullet as open**, and do not treat the recut as having discharged it —
  > `D2k-1b`'s own stop is below.

- **`D2k-1b` cannot express the closed compiler-only field** without runtime
  representation, new planner population, continuation-source transport, or a
  ripple **beyond the recut's explicitly enumerated constructor / match /
  preflight consumers.** Stop again and return the measured dependency.
- **A fifth `value_at` caller exists at your base.** Scope signal.

## 9. Contention and sizing

`crates/ken-runtime/src/cranelift_backend/lowering/mod.rs`, `.../core.rs`, and
the lexical-recursion consumer paths. This is the same file set as
[[RT-LEXICAL-R3-FUSION-EMITTER]].

> **THE SEQUENCING BAR IS LIFTED — released 2026-08-12 at `evt_9tx4kt0k8epm`.**
> This frame previously said to sequence **after** `RT-LEXICAL-R3-FUSION-EMITTER`
> because that node was in flight. **It is stopped** (Architect
> `evt_1q7v9fcw5hd87` fired its cumulative planner/ABI/representation stop) and
> its held range is preserved as **evidence only** — not a merge candidate, not
> competing for these files. **`D2k` is the in-flight node; you are not
> sequenced behind anything.** Re-derive the intersection at candidate time as
> always.

`scripts/ken-cargo test -p ken-runtime --lib` plus your focused suite. **Never
`--workspace`** — that is CI's gate, and `AC-7` means green in CI.

**Sizing.** `#6d` closure was measured at **closer to a week** (runtime-leader
`evt_645tm43wf1cne`) and these five expressions are the bulk of it. `D2k-0` and
`D2k-1a` were each sized as their own turn and each landed inside one.
**`D2k-1b` no longer cuts per root** — there is one root, so it cuts along the
ruling's own seams, and section 5 fixes the three increments and why the first
two cannot be split further. Per `§4b`: a hard stop inside an hour is a good
outcome — say so and hand back.
