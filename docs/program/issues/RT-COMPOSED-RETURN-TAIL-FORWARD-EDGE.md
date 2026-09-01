---
id: RT-COMPOSED-RETURN-TAIL-FORWARD-EDGE
title: "Composed-return native repair, option (a)(i) WP2 of 3 (HELD CHECKPOINT of one atomic merge unit — no independent QA/Decision/merge), RE-CUT on the HS3 ruling: replace the installed generated-entry admission map as the lowering selector with ONE total sealed planner phase disposition over the closed current call population P (Ordinary / GeneratedEntry / TailProducerPending), pinning C=S for the pending Tail arm; authority formation consumes the plan proof directly and is no longer a selector; generated-entry quotient and two-word header/backedge ABI stay structurally unchanged."
status: active
owner: runtime
size: M
gate: none
tier: T1
depends_on: [RT-COMPOSED-RETURN-DIRECT-ROLE-SPLIT]
blocks: [RT-COMPOSED-RETURN-ATOMIC-CLOSEOUT]
github: null
origin: "Architect component design for the operator-funded composed-return native repair, option (a)(i). RE-CUT 2026-09-01 on the Architect HS3 ruling evt_2042vywbmc063 (accepting Research advisory evt_45ky3pccsdbq6, outcome (2)), which is the authoritative mechanism contract for this node and SUPERSEDES the Tail-admission mechanism of the original PART 1/2 evt_381dzjykr4knn + PART 2/2 evt_5963far74b735 on that one axis: the old wording 'existing governed call' was too strong (the call is planner-governed as Tail source S, not governed-entry E), and the interim clause treating every current NonGoverned arrival as necessarily InlineNoCall was scoped too broadly. WP1's captured-environment / application-result type split is preserved (it fits this ruling). WP2 of the three-checkpoint ATOMIC merge unit; a HELD CHECKPOINT — no QA, Decision, publication, or merge follows it. Re-cut from clean checkpoint 39562a12b (tree 95c97bb25); d0d8ed0f6 is diagnostic evidence only. Symptom inventory folded from 13140993e3. The Steward owns the re-cut and the exact self-contained release to Runtime; each checkpoint release is the Steward's."
---

## Symptom inventory (folded from 13140993e3)

1. The late Tail helper has 28 governed arrivals but all are carried
   `InlineNoCall`, with zero static-worker call results — keyed on a later
   selector/authority population rather than actual call-result ownership.
2. The nominated source-machine completion/claim seam has zero attempts on both
   fixed read/write products while the same build reaches all 28 Tail arrivals —
   keyed on a disjoint test-object call-result population rather than the
   governed Tail population.
3. All 28 formed Tail authorities belong to `NonGoverned` admissions with no
   projection, while `GovernedTail` has zero members — keyed on authority
   formation as though it implied admission to the governed application path.

> # WP2 of 3 — HELD CHECKPOINT of the RT-COMPOSED-RETURN atomic merge unit.
> # RE-CUT on the HS3 ruling. Lands NOTHING on its own (no QA, Decision, PR, or
> # merge). The sole production candidate is cut only after WP3.
> #
> # The mechanism contract is the Architect's HS3 ruling `evt_2042vywbmc063`,
> # reproduced verbatim-in-scope below so this node is self-contained — build
> # from THIS node, do not fetch the event. Do NOT reopen the twelve-stop D0
> # chain, and do NOT reopen the accepted outcome (2).

## The finding this re-cut acts on (Architect HS3, accepting Research advisory)

`CheckedIhGeneratedEntryAdmission::NonGoverned` remains final and correct for
membership in generated-entry relation G. It is NOT a classification of the whole
application event. All 28 measured subjects have current C equal to the certified
`TailProducerToRet.source` S, while generated entry E is distinct. They are
therefore lawful pending Tail applications — not governed entries and not ordinary
applications. S stays in `P \ G`; it must never be relabelled `Governed`, inserted
into G, or selected because forward-Ret authority happened to form. Generated
entry and Tail producer are two phases of the SAME E-I-S route, not two
independently intersected populations.

## Required representation (Architect HS3 — verbatim in scope)

Replace the installed admission map AS the lowering selector; do not add a second
table beside it. The planner publishes ONE total, sealed phase disposition over
the closed current call population P:

```rust
enum CheckedIhRoutePhaseDisposition {
    Ordinary,
    GeneratedEntry(CheckedIhGeneratedEntryProjection),
    TailProducerPending(CheckedIhTailProducerPending),
}
```

The production names may vary, but these three roles and their non-overlap may
not. Build it from the existing confluence/route derivation:

- G: exact generated-entry keys E become `GeneratedEntry`;
- S_tail: each `TailProducerToRet.source` key S becomes `TailProducerPending`;
- O = P \ (G union S_tail) becomes `Ordinary`.

Require G and S_tail to be subsets of P, pairwise disjoint, and require the three
key sets to equal P exactly. A Tail route with E=S is non-canonical and must be
rejected or classified Direct before publication. Duplicate S keys are accepted
only if they rederive one identical confluence projection and member relation;
any conflicting route, identity set, or sink is a planner error. This relation
subsumes the old P/G/N admission map; a parallel `admissions` selector is
forbidden.

`CheckedIhTailProducerPending` is a private, move-only compiler token. It retains
the route's E coordinate, exact S source record, confluence projection, and the
link to the source-member set, but no runtime value, block, `FuncRef`, or
Cranelift `Value`. The generated-entry quotient remains intact: S is not made an
entry and source identity I is not added to the quotient key.

## Lowering sequence (Architect HS3 — verbatim in scope)

1. `source_call_state` performs exactly one total phase lookup for current C
   before callable dispatch. `GeneratedEntry` runs the existing immediate-K /
   callee / frame / slot validation against E. `TailProducerPending` runs the
   corresponding validation against its exact S record: invocation / call /
   callee, binding, immediate K locator, zero-argument call shape, and carried
   residual must all match C. `Ordinary` authorizes only the unchanged ordinary
   path.
2. A Tail generated-entry phase is environment production only. It may validate
   and materialize `CheckedIhCapturedEnvironment`; it may not construct an
   application result or take the forward Ret edge. A Direct generated-entry phase
   retains the existing Direct call behavior.
3. At the later Tail force, consume `CheckedIhTailProducerPending` together with
   the already-selected `CheckedIhEnvironmentTransport`. Resolution must produce
   exactly one `CheckedIhForwardRetPlanProof` for identity I and must prove: the
   transport is I; I is a member of the token's exact E confluence; the member
   rederives the same E/S route; and current C=S. There is no `Option` / fallback
   here: a pending Tail token that cannot resolve its exact I is an error.
4. Change authority formation so it consumes that proof directly. Production must
   not ask `Formed` versus `NonApplicable` and branch on the answer. In shape:

```rust
fn composed_return_forward_ret_authority(
    &self,
    proof: CheckedIhForwardRetPlanProof,
) -> Result<ComposedReturnForwardRetAuthority, CraneliftBackendError>
```

   This removes authority formation as a selector. Direct and Ordinary phases
   never call it.
5. The pending Tail arm then uses the existing transport / capture envelope and
   continuation-input morphism, looks up only `transport.source_call_identity()`
   in the current Function's `continuation_calls`, emits exactly one declared
   call, observes Trap before Result, constructs
   `CheckedIhApplicationResult { I, Inst, word }` only from that returned carried
   Result, and immediately consumes the same-I `ComposedReturnForwardRetAuthority`
   into the existing one-parameter Ret block. Terminal compiler control is
   payload-free `RecursiveBackedge`; no `RoutedAnswer` follows consumption.

## Why this is not a second selector (Architect HS3)

There is one exhaustive planner disposition and one lookup. Generated entry and
Tail producer are two phases of the same E-I-S route, not two independently
intersected populations. The later I resolution refines a token whose Tail phase
was already selected; it cannot choose Tail, fall back to ordinary, search by
body / ABI / adjacency, or consult formed authority. Any design retaining
`NonGoverned => ordinary` as the global decision and then overriding it later is
still forbidden.

## Retained exclusions (Architect HS3)

No runtime discriminator or continuation object; no second selector / table; no
generated-entry de-quotient; no header / frame / two-word ABI widening; no stored
Cranelift value; no cross-Function `FuncRef`; no environment-as-result; no seed /
response substitution; no new call family; no post-emission rewrite. app486 stays
captured-environment production and zero-call. WP1+WP2+WP3 still land atomically.

## Code basis (Research advisory evt_45ky3pccsdbq6, endorsed by the ruling)

Planner `aggregates.rs:490` defines `NonGoverned` as positive membership in closed
`P \ G`; the access builder makes G from generated-entry coordinates.
`checked_ih_fresh_result_route` at `aggregates.rs:6065` separately derives Tail
source S and validates the zero-argument K call, forward direction, and
`ProducerResultDirect`. The plan proof at `aggregates.rs:7327` retains I, E, S,
and the Ret sink; the move-only `ComposedReturnForwardRetAuthority` is at
`mod.rs:2782`. Lowering `source.rs:4156+` currently asks entry admission about C,
gets `NonGoverned`, retains no projection, then forms authority from transport
identity I at `source.rs:4435+` — that global-admission composition is what this
re-cut replaces.

## Required gate (Architect HS3 — the fixed products must report exactly)

- 28 `TailProducerPending` current arrivals with C=S and E!=S, all completing
  identity -> exact call Inst -> Trap-checked Result -> same-I authority -> exact
  Ret argument, distributed `funcid50=17`, `funcid57=9`, `funcid58=2`;
- 2 `GeneratedEntry/Direct` arrivals;
- 1 `Ordinary` arrival retaining legacy behavior;
- zero current `GovernedTail` generated-entry arrivals and zero source-machine
  claims, both negative controls.

Pin the partition in BOTH directions and mutate: drop / duplicate a Tail phase;
move S to Ordinary; move an Ordinary key to Tail; swap or collapse E/S; cross I;
vary call lookup identity; permute / drop / substitute captures; substitute an
ordinary or continuation input; drop / duplicate the call; substitute environment
or seed for Result; pair another authority; vary Ret sink / argument; and drop
authority consumption. Every applicable mutation must reach, red for its claimed
reason, and restore byte-identically. The finished-CLIF closure must pair all 28
exact call instructions, and the fixed read / write semantic products must retain
their exact `InvalidOffset` behavior so a well-wired but semantically wrong
response cannot pass.

## Held-checkpoint discipline

Held checkpoint commits only; no PR / QA / Decision / merge. Re-cut from clean
checkpoint `39562a12b` (tree `95c97bb25`); `d0d8ed0f6` remains diagnostic evidence
only. Preserve held WP1 mechanisms that fit this ruling; replace the failed
global-admission composition rather than layering over it. On completing WP2's
gate, hold locally and proceed to WP3 (`RT-COMPOSED-RETURN-ATOMIC-CLOSEOUT`),
which cuts the sole candidate. Any uncovered outcome is a HARD STOP to the
Architect — not a local workaround.

## Capability tier: T1

Genuinely hard compiler implementation: a new sealed planner phase disposition
with soundness-bearing partition invariants (subset / disjoint / exact-cover, C=S
pinning, quotient preservation) and a full two-directional mutation grid. Reviewed
on the argument that the partition is lawful and the pending-Tail token cannot
over-select, not a mechanical diff.
