# RT-LEXICAL-RECURSOR-CONSUMERS D2d — grounding, and a hard stop on clause 2

Node: `docs/program/issues/RT-LEXICAL-RECURSOR-CONSUMERS.md`. Frame:
`RT-LEXICAL-RECURSOR-CONSUMERS-D2d.md`. Architect ruling `evt_2wwh9yamyhs7p`.

**This candidate delivers `AC-6` and nothing else.** `AC-1` through `AC-4` are
**not** attempted: the ruled mechanism's clause 2 requires an emission region
this compiler cannot form without a move the same ruling forbids. That is
measured below, not inferred, and the question it raises is the Architect's.

## 0. Provenance — one coordinate, derived at the moment of writing

| coordinate | exact | moves? |
|---|---|---|
| candidate merge-base | `3419531cbe97f554da3c3630e03a6c6d90dea1c5` | **yes.** Re-derive it; do not reuse this value |

The branch was reset to `origin/main` at that commit and has not been rebased.
Every figure below was taken on the candidate tree, whose only `crates/` change
is the `AC-6` rider — a test-file edit that cannot reach the compiler paths
these measurements run through.

**The `D2c` premise was NOT re-measured.** The retained lane's zero carrier
transfers versus the functionized lane's six is settled and was taken as given.
The functionized compile below was run **once**, to locate the refusing seat —
that is a different question from the differential, and it does not restate it.

## 1. The two authorities that independently name origin 23

The planner and the lowering refusal were instrumented separately, on the same
`R3` before-hole compile under B-only exclusion
(`RecursiveDescentResidual::LexicalCallArgumentRecursor`), fixture
`px8j_equal_payload_hole_placement(BeforeReturnHole)`.

**The planner's continuation discovery, `build_continuation_specialization_plan`:**

```text
consumer_origin=StaticOriginId(5)   consumer_owner=PredeclaredFunctionId(0)
producer_construct=StaticOriginId(23) producer_owner=PredeclaredFunctionId(2)
position=0                          candidate=Var(0)
```

**The lowering refusal, at the carried join:**

```text
elim_origin=StaticOriginId(18) body_origin=StaticOriginId(23)
case=ctor:fixture::PX8JHoleInput::Node rec=[0]
active_scope=true remaining=[] lowered=Specialized(Constructor)
  -> Err: "a computational recursor closure names an in-flight activation,
           not a transferable value"
```

⇒ **The identity clause 1 asks for already exists.** The planner pairs the exact
producer occurrence `23` with the exact consumer continuation `5` at recursive
position `0`, from planner-issued occurrence identities. Nothing here is keyed
on constructor spelling, type, row number, a runtime tag, or *"the only
continuation"* — and no suffix identity is **inferred**, because the planner
already holds it. Two independent instruments name origin `23`.

## 2. Why no specialization is interned — the exact predicate

`build_continuation_specialization_plan` walks each recursive position of a
matched producer `Construct` and **requires the argument at that position to be
a syntactic `Closure` or `LexicalClosure`**; anything else is skipped by a bare
`continue`.

**On this producer the argument is `Var(0)`** — a de Bruijn reference to the
compiler-minted induction hypothesis, which is exactly the
`ComputationalRecursorClosure`. That single predicate is the whole reason the
pair is not interned. It is not a gap in discovery.

## 3. The hard stop — clause 2 cannot be satisfied without a forbidden move

**`producer_owner` is `PredeclaredFunctionId(2)`; `consumer_owner` is
`PredeclaredFunctionId(0)`.** Producer and consumer suffix are in **different
functionized units**, and the carrier boundary between them *is* the unit
boundary. Clause 2's *"emit the producer and that suffix in one functionized
emission region"* therefore names **unit formation**, not routing.

**Clause 2's routing half has no local subject.** At the refusing seat
`remaining_eliminators` is **empty**, and the backtrace puts the seat directly
under `define_unit_bodies` -> `lower_computational_match_expr` ->
`lower_carried_computational_match`. It is the top of its own unit body, whose
continuation is *return through the ABI*. There is no already-owned composed
source continuation to route a `RoutedAnswer` through *"before
`carried_join_arm`"* — the thing to route through is in another unit.

**The landed vehicle for a fused region cannot carry this one.** The
continuation-specialization unit class is the compiler's existing
producer-plus-suffix region, and it is built on a **static worker at the
recursive position** at every layer:

- `ContinuationSpecializationKey` contains `worker: ContinuationWorkerProvenance`
  — part of the interning identity, so a worker-less unit is not expressible as
  a value of this key;
- `build_continuation_worker_provenance` needs the position's `closure_origin`,
  `body_origin`, `declared_arity` and `captures`;
- `define_continuation_bodies` emits from `worker_closure_origin`,
  `worker_body_origin`, `worker_declared_arity`, `worker_capture_count`, and
  resolves worker call targets for the body.

A `ComputationalRecursorClosure` has **none** of these. Supplying them means
giving the recursor closure a closure/worker and ABI representation — which the
ruling rejects outright (*"a live activation and cursor, not a value"*) and
which the frame lists as a **stop condition**. So reusing the landed class is
forbidden, and not reusing it means a **new planned unit class**: its own
identity, ABI envelope, validators and emission body.

⇒ **Stopped and returned to the Architect, per the frame's stop conditions.**

## 4. The question that is the Architect's, stated exactly

**Is a worker-less continuation-specialized unit class authorized** — one whose
recursive-position operand is the in-region compiler-minted recursor activation,
consumed by the fused suffix's own `Call`, rather than a static worker?

If yes, one sub-question follows and is also not mine: **which owner does the
fused unit take**, the producer's (`2`) or the consumer's (`0`)? The existing
class answers *consumer* (`define_continuation_bodies` binds
`unit.consumer_owner`), but it answers it for a population where the producer
contributes only an envelope, not an activation.

**No repair was attempted against either answer**, and no mechanism was chosen.

## 5. What this candidate does deliver — `AC-6`

The confirmed Adversary finding `evt_6enwsf0jrdezx`. `s_arrivals` is now built
as a `NonZeroUsize` whose `expect` carries the old assertion's message, and the
suppressed leg's equality diagnostic reads that value. Two lines. No predicate
change, no count pinned, no new control.

**The compared value stays `0`.** Deriving it from the count would either pin a
count or replace a clear assertion message with a subtract-with-overflow panic;
the count is read where it is genuinely needed, which is the diagnostic.

### The A/B, and the informative side is the one that greens

| leg | denominator deleted | result |
|---|---|---|
| repaired (this candidate) | yes | **compile error** — `E0425: cannot find value 'established_s_arrivals'` |
| **pre-repair form** | yes | **compiles, 1 passed** |

The pre-repair leg passing with its denominator deleted is what makes the first
row informative: the old `assert!(s_arrivals > 0, ...)` was genuinely removable
without breaking anything, exactly as reported. The repair is what makes the
deletion impossible, not a neighbouring edit.

## 6. Scope

No production change — the only `crates/` path is
`lowering/core/tests/control.rs`. No repair, no new control, no enum change, no
`R4`, no `D3`, no retirement or lane deletion, no tracker `status:` change. Row
5 after-hole stays reported-only and no row is claimed green. `D2b`'s controls
are untouched and still hold (`AC-5`). All three probes used above were
temporary and are removed; the two compiler files they touched are byte-identical
to the merge-base.

Provenance: Ken-owned frame, tracker, ruling and this repository's own source
and measurements only. No `local/refs/`, no permissive reference, no copyleft
reference, no excluded prototype contact.
