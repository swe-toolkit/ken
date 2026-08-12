# `RT-LEXICAL-RECURSOR-CONSUMERS` `D2k` — the `StaticWorkerBinding` wall

The five expressions that `D2a` advanced and left standing at a wall it filed
as *"successor, unfiled here"*. Architect ruling `evt_5wvk3e8k1bjqn`
(2026-08-12) placed them **inside `#6d`** — this is `D2`'s next increment, not
a new node, and **[[RT-CONTSRC-CALLABLE-CONTRACT]] is not a prerequisite.**

Fixed inputs measured at `main` **`b2ee3377`**. Re-derive them at your base;
a merge-base goes stale without your branch moving.

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
section 3's ruled structural properties. **Cut it so each turn lands or hard-
stops inside about an hour** — the coupling is `Construct` producer, static
`Match` binder, both lowering engines, and boundary preflight, and that is more
than one turn. **Preflight refusal is not optional garnish**: a constructor
carrying a worker field that descends partway and then refuses fails the ruling.

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
  ABI/carrier/descriptor, `#[ignore]`, or `value_at` caller.

## 6. Acceptance criteria

**`AC-1` — the five are green** on the pre-retirement tree under `B`-only
exclusion, and each is green **because the worker is transported through its
constructor as a compiler-only field and consumed at the later exact-`Var`
call**, not because a guard stopped firing.

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
`evt_645tm43wf1cne`) and these five expressions are the bulk of it. **`D2k-0` is
sized as its own turn** and should land well inside one; `D2k-1` cuts per root.
Per `§4b`: a hard stop inside an hour is a good outcome — say so and hand back.
