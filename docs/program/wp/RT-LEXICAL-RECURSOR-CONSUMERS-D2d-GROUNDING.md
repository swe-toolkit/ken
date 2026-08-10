# RT-LEXICAL-RECURSOR-CONSUMERS D2d — grounding for the fusion class

Node: `docs/program/issues/RT-LEXICAL-RECURSOR-CONSUMERS.md`. Frame:
`RT-LEXICAL-RECURSOR-CONSUMERS-D2d.md`. Architect rulings `evt_2wwh9yamyhs7p`
(the mechanism) and `evt_6sk3czsbcr85r` (the `StaticContinuationFusion` class).

**This candidate delivers `AC-6` and the grounding below. `AC-1` through `AC-4`
are authorized and NOT built.** An earlier revision of this record stopped on
clause 2 and asked whether a worker-less unit class was permitted. **That
question has been answered — the answer is a distinct
`StaticContinuationFusion` class — and this record is rewritten accordingly.
Nothing here is still blocked on a ruling.**

## 0. Provenance — one coordinate, derived at the moment of writing

| coordinate | exact | moves? |
|---|---|---|
| candidate merge-base | `112a2ae450c087420ba21ced58ec0b42f5c3b5d3` | **yes.** Re-derive it; do not reuse this value |

Rebased onto that commit; the rebase replayed both commits with **zero delta**
(`git range-diff` reports `=` for each, and the two range patches are
byte-identical). The candidate's only `crates/` change is the `AC-6` rider — a
test-file edit that cannot reach the compiler paths these measurements run
through.

**The `D2c` premise was NOT re-measured.** The retained lane's zero carrier
transfers versus the functionized lane's six is settled and taken as given. The
functionized compile below was run to locate coordinates, which is a different
question from the differential and does not restate it.

## 1. The two authorities that independently name origin 23

Instrumented separately on the same `R3` before-hole compile under B-only
exclusion (`RecursiveDescentResidual::LexicalCallArgumentRecursor`), fixture
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

⇒ The producer/consumer pairing is **already a planner fact**, from
planner-issued occurrence identities. Nothing is keyed on constructor spelling,
type, row number, a runtime tag, or *"the only continuation"*, and no suffix
identity is inferred. Two independent instruments name origin `23`.

## 2. Why the existing class does not intern it

`build_continuation_specialization_plan` walks each recursive position of a
matched producer `Construct` and **requires the argument at that position to be
a syntactic `Closure` or `LexicalClosure`**; anything else is skipped.

**Here the argument is `Var(0)`** — the compiler-minted induction hypothesis,
which is the `ComputationalRecursorClosure` itself. That predicate is the whole
reason the pair is not interned.

**And the existing class must stay that way.** Its identity carries
`worker: ContinuationWorkerProvenance`, `build_continuation_worker_provenance`
needs a closure origin, body origin, declared arity and captures, and
`define_continuation_bodies` emits from all four. Every one of its validators
and emitters is correctly defined over a **real static worker**; making that
optional or synthesizing one would silently widen the class's authority. That
is why the ruling introduces a separate class rather than relaxing this one.

## 3. Why a routing change could not have worked

**`producer_owner` is `PredeclaredFunctionId(2)`; `consumer_owner` is
`PredeclaredFunctionId(0)`** — different functionized units, with the carrier
boundary between them *being* the unit boundary. And at the refusing seat
`remaining_eliminators` is **empty**, with the seat sitting directly under
`define_unit_bodies` -> `lower_computational_match_expr` ->
`lower_carried_computational_match`: it is the top of its own unit body, whose
continuation is *return through the ABI*.

⇒ There is no already-owned composed continuation at that seat to route a
`RoutedAnswer` through. The fused region has to be **formed**, which is what the
new unit class is for.

## 4. The two planner preconditions are measured CLEAR

The ruling requires planning to project the exact producer invocation edge and
its complete input run. Both planner preconditions are measured clear below. The
independent emitter/ABI stop condition cannot be discharged by this
non-implementation partial: §4.3 establishes only that no activation appears in
the projected inputs. It remains a live obligation on the emitter.

### 4.1 The exact producer invocation edge is projectable and unique

Every `StaticBody` call edge in the plan:

| caller | callee | callee entry origin |
|---|---|---|
| **`PredeclaredFunctionId(0)`** | **`PredeclaredFunctionId(2)`** | **`StaticOriginId(28)`** |
| `PredeclaredFunctionId(0)` | `PredeclaredFunctionId(3)` | `StaticOriginId(38)` |
| `PredeclaredFunctionId(2)` | `PredeclaredFunctionId(1)` | `StaticOriginId(26)` |

**Exactly one edge invokes the producer unit**, and its caller is
`PredeclaredFunctionId(0)` — **independently confirming the ruling's stated
coordinate** that the fusion call is emitted by the consumer-side owner. This is
a measured edge, not a search for a plausible one.

### 4.2 The complete input run is projectable

`exact_continuation_source_environment` returns `Some` for this pair — it is
**not** declined for an open or ambiguous source value environment:

```text
ContinuationProducerEnvironment {
  producer_owner: PredeclaredFunctionId(2),
  producer_result_origin: StaticOriginId(18),
  producer_construct_origin: StaticOriginId(23),
  consumer_owner: PredeclaredFunctionId(0),
  inputs: [
    { coordinate: EntryAbi { source_owner: PredeclaredFunctionId(0),
                             source_abi_position: 0, source: Parameter },
      carrier: ValueWord, ownership: OwnedByFrame,
      storage_owner: ActivationFrame,
      referent_affinity: [NoReferent, PersistentStore, InvocationArena] },
    { coordinate: EntryAbi { source_owner: PredeclaredFunctionId(0),
                             source_abi_position: 1, source: Parameter },
      carrier: ValueWord, ownership: OwnedByFrame,
      storage_owner: ActivationFrame,
      referent_affinity: [NoReferent, PersistentStore, InvocationArena] },
  ],
}
```

### 4.3 No activation reaches a carrier in that projection

Both projected inputs are ordinary **parameter-sourced value words**. No
`ComputationalRecursorClosure`, activation, cursor, selection or unwind state
appears anywhere in the input run. The third stop condition is therefore not
triggered *by the projection*; it remains a live obligation on the emitter,
which this candidate does not write.

## 5. The complete measured key material

Every member of the ruling's key except the checked frame/invocation-template
identity is now a measured coordinate on this witness:

| key member | measured value |
|---|---|
| producer invocation edge | the single `StaticBody` edge `0 -> 2`, callee entry `StaticOriginId(28)` |
| emission owner of that call | `PredeclaredFunctionId(0)` |
| producer owner | `PredeclaredFunctionId(2)` |
| producer result root | `StaticOriginId(18)` |
| producer construct origin | `StaticOriginId(23)` |
| selected alternative | `0` |
| recursive position | `0` |
| consumer owner | `PredeclaredFunctionId(0)` |
| continuation-frame origin | `StaticOriginId(5)` |
| selected case | `ctor:fixture::PX8JHoleOutput::Node` |
| **exact consuming `Call` occurrence** | **`StaticOriginId(12)`** — `Call { callee: Var(0), args: [Construct Unit::MkUnit] }` |
| ordered ABI input projection | the two-entry run in §4.2 |

⛔ **This tuple is grounding, not an identity.** It is what a key must *resolve
to* on this witness — the class must derive each member from planner facts, and
a fixture-shaped tuple is exactly the "identity that happens to be unique in the
measured population" the ruling forbids. It is recorded so the next slice can be
checked against measured coordinates instead of re-deriving them.

## 6. `AC-6` — delivered, and proved load-bearing

The confirmed Adversary finding `evt_6enwsf0jrdezx`. `s_arrivals` is built as a
`NonZeroUsize` whose `expect` carries the old assertion's message, and the
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
without breaking anything. The repair is what makes the deletion impossible, not
a neighbouring edit.

## 7. What is NOT in this candidate

**No part of the `StaticContinuationFusion` class is built** — no
`StaticContinuationFusionId`, no key, no interning, no
`AbiUnitDefinition::StaticContinuationFusion` arm, no descriptor population, no
bijection validators, no `ContinuationEmissionOwner::Fusion` arm, no scoped
source-body authorities, no generated-definition emitter, and no redirection of
the producer invocation edge. `AC-1` through `AC-4` and their controls are
therefore **not** discharged, and no `R3` row is claimed green.

This is a scope statement, not a stop: **nothing above is blocked.** The class
is authorized and its two preconditions are measured clear.

## 8. Scope

No production change — the only `crates/` path is
`lowering/core/tests/control.rs`. No repair, no new control, no enum change, no
`R4`, no `D3`, no retirement or lane deletion, no tracker `status:` change. Row
5 after-hole stays reported-only. `D2b`'s controls are untouched and still hold
(`AC-5`). The existing continuation-specialization class is **unchanged**: no
`Option` on `worker`, no synthesized provenance, no reuse of
`ContinuationSpecializationId`.

Every probe used above was temporary and is removed;
`planning/static_transition.rs`, `lowering/core.rs`, `lowering/mod.rs` and
`lowering/units.rs` are proven **byte-identical** to the merge-base by blob
hash.

Provenance: Ken-owned frame, tracker, rulings and this repository's own source
and measurements only. No `local/refs/`, no permissive reference, no copyleft
reference, no excluded prototype contact.
