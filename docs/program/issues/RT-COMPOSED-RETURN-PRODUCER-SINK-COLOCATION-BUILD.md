---
id: RT-COMPOSED-RETURN-PRODUCER-SINK-COLOCATION-BUILD
title: "Production co-location repair: at each TRUE call emitter, consume the already-selected Tail proof and the actual returned SSA result into an exhaustive compiler-only forward disposition keyed per physical emission — installing/scheduling the exact Ret body once, branching the one real result to its one exact sink, and terminating the old path with no later carried/fallback claim, on both read and write products, holding every genuine spec wall (actual response, exact continuation, one application, one effect/resumption, no observable closure identity, differential agreement)."
status: ready
owner: runtime
size: L
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Architect ruling evt_6vpyaje9a7n85 (both gates on exact base 0be25235b188bc67b3f9209d1ff0b6f8fa063258, tree 769c24708fb2052c3d6e719a8adc135423c28192), resolving the operator-keyed disposition (Pat) on the RT-COMPOSED-RETURN-PRODUCER-SINK-COLOCATION D0=NO. The D0 returned NO only under campaign-situational constraints; the Architect grounded reachability-from-source YES (rt_parity_native.rs RT_PARITY_SOURCE, real build_native_program(SourceFormat::Ken) path) and ruled every blocking axis situational, not normative. Steward-recut per COORDINATION section 2. Design authority for this production recut only; no candidate/merge authority before fresh Runtime QA, Architect review, and publisher CI."
---

> # READY — PRODUCTION recut. Released to the runtime ring (lane 1). Runtime was parked; this IS the release.
>
> The `PRODUCER-SINK-COLOCATION` D0 returned NO, but only under the funded
> campaign-situational constraints. The Architect (`evt_6vpyaje9a7n85`) resolved
> both operator gates on exact base `0be25235b`: the construction is REACHED FROM
> WELL-FORMED KEN SOURCE, and the constraints that blocked true-producer/
> predeclared-sink co-location are campaign search bounds, not spec-normative. Per
> the operator's keyed disposition, un-close the minimum compiler return/control
> axes and fund the direct co-location repair. This is a PRODUCTION node: it lands
> production, routes Runtime QA, opens a merge gate, and needs a resolved Decision.
>
> **Do NOT** send this to Research, and **do NOT** revive the spent
> `source.rs:4492` / `CarriedEnvironment` / `InlineNoCall` D3 seat — that seat
> owns no application result and forwarding its environment word as `resp`
> violates `42 §6.2`. A different mechanism that carries the ACTUAL result is not
> "D3 salvage". No reuse of `e65cef092`; no expectation change.

## Exact base and coordinates (Architect `evt_6vpyaje9a7n85`)

Binding base `0be25235b188bc67b3f9209d1ff0b6f8fa063258`, tree
`769c24708fb2052c3d6e719a8adc135423c28192`. **Re-measure the exact seams at this
base before touching anything** — prior campaign increments moved coordinates and
this node cites functions, not frozen line numbers.

- **The TRUE producer is the checked-IH environment-transport call emitter**
  (`call_checked_ih_environment_transport`), in `crates/ken-runtime/src/
  cranelift_backend/lowering/{source,core}.rs`. The D0 recorded 112 true emissions
  on the read product and 75 on the write product. The case-environment
  `StaticWorker` arm had ZERO arrivals on both products — it is not the emitter to
  instrument. Instrument the real emitter, keyed per physical emission, never by
  aggregate stable identity.
- **The already-selected Tail proof** is available at that true emitter (the D0's
  Tail proof populations were nonzero there). Consume it in place; do not borrow
  it from a later carried arrival, reconstruct it by a second lookup, or select by
  coordinate coincidence.
- **The two failure modes the scratch probe exposed, which this node must close:**
  - Read: `RecursiveBackedge` skipped installation/lowering of the predeclared
    body (D0 saw `block247` left unfilled) — the exact Ret body is not schedulable
    on that path. This node must make it schedulable.
  - Write: the scratch emitted seven true-result edges but only six marker
    propagations and retained later carried claims — a physical
    producer/result/edge/sink population that is not closed one-to-one. This node
    must close every physical occurrence one-to-one.

## Normative floor (Architect — the genuine spec walls; keep these)

`36-effects.md` §§2.1/2.2/5.2, `42-evaluation.md` §6.2, `45-native-backend.md`
§§2-4, `41-values.md` §2.1, `44-capacity.md` §§1-3. The backend is OUTSIDE the
type-soundness TCB: a bug here is a wrong value caught by the differential, never
a false proof. What is spec-REQUIRED and must hold:

- The program's eliminator receives the ACTUAL declared-call `Result`
  constructor/value — not the carried environment/seed word.
- The exact continuation `k` is applied EXACTLY ONCE, in tail position
  (`36 §5.2`, `42 §6.2`). No second application, no capture, no multi-shot.
- Exactly one semantic response/resumption; no duplicated effect; strict CBV and
  the observable effect sequence unchanged.
- No observable closure identity, persistence, forged callable, or Ken-visible
  tag; internal representation stays private (`41 §2.1`, `44`, `45 §3`).
- Native value/effect observation agrees with the interpreter (`45 §§2-4`) —
  differential parity on both products.

## Un-closed axes (Architect per-axis grounding — available implementation choices now)

None is spec-required-closed in blanket form. Pick the SMALLEST that makes the
obligations below structural:

- Internal block/function parameter (active-header / internal ABI): allowed —
  `45 §3` does not fix the calling convention — if observable behavior and effect
  order are unchanged.
- A private typed internal result carrier: allowed (`41`, `45 §3`) if it creates
  no observable closure identity, persistence, or wrong value.
- A second native `Ret` lowering / multiple native blocks: allowed — `Ret r -> r`
  is semantic, the spec fixes no CFG block count — if there is still exactly one
  semantic response/resumption and no duplicated effect.
- A private exact-lookup store / side table / compiler receipt / explicit
  Ret-body scheduling: allowed only as an EXHAUSTIVE, one-consume,
  live-domain-only, fail-loud disposition that proves one-to-one causality. An
  optional droppable receipt or a heuristic identity store is NOT adequate.
- Reordering the producer after static Tail selection: allowed (static selection
  is not a world effect) if strict CBV and the observable effect sequence are
  identical.

## Deliverables

**D1 — forward-disposition protocol + read product green.** Introduce, at the
true call emitter, an exhaustive compiler-only forward disposition keyed per
physical emission that consumes the already-selected Tail proof and the actual
returned SSA value and, for the read product (`rt_read_offset_stage` /
`fs-read-at-offset-single`), (a) installs/schedules the exact Ret body exactly
once even when constructor materialization is bypassed, (b) branches the one
actual result to its one exact sink, (c) terminates the old path with no later
carried/fallback claim. The previously unfilled predeclared body becomes
schedulable. Read differential parity passes. D1 is independently mergeable
(partial-merge policy, COORDINATION / merge-policy) if the write product needs a
further turn.

**D2 — write product one-to-one closure.** Consume the D1 protocol for the write
product (`rt_write_writable_stage` / `fs-write-at-offset-single`): close every
physical producer/result/edge/sink occurrence one-to-one — no seven-edges/
six-markers gap, no retained later carried claim. Write differential parity
passes. If both products fall out of the shared protocol in one turn, D1 and D2
may land together; the split exists so read can land first, not to force two
turns.

## Acceptance criteria (each carries its own control; controls are mutations that must red)

- **AC-1 — Actual result at the sink.** The eliminator receives the actual
  declared-call `Result` value on the governed Tail route, both products.
  Control: substituting the carried environment/seed word for the real result
  reproduces the pre-repair `Result`/`PatternMatchFailure` mismatch (the check
  stays discriminating; it may disappear only because the correct result reaches
  it, or be replaced by an equally discriminating exact-result check — never
  weakened to accept the wrong word).
- **AC-2 — Exactly-once tail application.** `k` is applied once in tail position.
  Control: a mutation that applies it twice, captures it, or multi-shots it
  refuses or reds; a mutation that drops the application reds.
- **AC-3 — One physical occurrence closed per emission.** Every physical
  producer/result/edge/sink is paired one-to-one, keyed per physical emission.
  Control: suppressing or duplicating one edge/marker trips its own exact census
  (the write seven-edges/six-markers gap is closed and a re-introduced gap reds);
  identity is per-physical-emission, NOT cursor number, count, proximity, or
  aggregate equality.
- **AC-4 — Exact Ret body schedulable (read).** The predeclared body is installed/
  scheduled on the `RecursiveBackedge` path. Control: a mutation that skips
  installation reproduces the D0 unfilled-body failure.
- **AC-5 — No observable closure identity / single effect.** No Ken-visible tag,
  durable closure identity, forged callable, or duplicated effect is introduced.
  Control: mutating member, transport, frame, body, binder, direction, or
  delivery refuses before native execution at the corresponding natural boundary;
  ordinary Ret and Direct remain independently green.
- **AC-6 — Differential agreement, both products.** Native value/effect
  observation agrees with the interpreter for read and write through the real
  `build_native_program(SourceFormat::Ken)` path in `rt_parity_native.rs`. A green
  product alone is NOT the causal proof — AC-1..AC-5 controls must accompany it.

## Returns a hard stop (to the Architect) if

- closing the obligations structurally requires a Ken-VISIBLE tag, durable
  closure identity, or a wrong-word coercion (any genuine spec wall above);
- the only available closure needs a heuristic identity store / droppable receipt
  / recovery by liveness, count, proximity, or aggregate stable identity (these
  cannot prove the exact `k`/`resp` pairing and are spec-incompatible);
- producer and sink prove to be in genuinely different emitted functions with no
  dominating path — which would be a wider return-protocol question for the
  Architect, not a repair.

On a hard stop, return the exact condition and evidence to the Architect; do not
attempt a remedy or reopen a closed axis unilaterally.

## Prohibitions

Do NOT revive the spent `source.rs:4492` / `CarriedEnvironment` / `InlineNoCall`
D3 seat or treat the carried environment word as a fresh result. Do NOT weaken the
downstream `Result` wall to accept/coerce the wrong word (it may only be replaced
by an equally discriminating exact-result check). Do NOT introduce observable
closure identity, a Ken-visible tag, a second semantic application, a duplicated
effect, or a heuristic/droppable disposition. Do NOT reuse `e65cef092`, change any
expectation, or close the objective via Direct-only greenness while Tail executes
with a wrong value.

## Reviewers, sequencing, contention

- **Reviewers:** independent Runtime QA (mutation controls proven, both products);
  Architect (design review of the production increment against this frame and the
  normative floor). Publisher CI is the code gate. A resolved merge Decision is
  required before routing. No Conformance Validator — this is native-backend
  value-correctness, not kernel/conformance. This grants no candidate/merge
  authority before those.
- **Sequencing:** runtime ring. D1 (protocol + read) then D2 (write closure); D1
  independently mergeable, or both together if one turn suffices. Size L; hit a
  releasable increment or a genuine hard stop per turn.
- **Contention:** touches `crates/ken-runtime/src/cranelift_backend/lowering/
  {source,core}.rs` and the `crates/ken-cli/tests/rt_parity_native.rs` evidence
  wrappers. No crate/catalog contention with the concurrent lanes (kernel
  `conv.rs`, verify CI scripts). Targeted builds ONLY via `scripts/ken-cargo`
  scoped to `ken-runtime` / the parity test, never `--workspace`; the
  full-workspace and conformance gates run in CI.
