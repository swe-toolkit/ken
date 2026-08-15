---
id: RT-CLOSURE-BOUNDARY-LANE
title: "A closure cannot cross the durable boundary -- runtime-local and live-domain only, with no durable lane"
status: draft
owner: runtime
size: TBD
gate: none
depends_on: [RT-SRCBODY-BIND-ORDER]
blocks: []
github: null
origin: Measured at frozen base 21fd46dc by the RT-SRCBODY-BIND-ORDER D10 differential (evt_2jc88hbzfskpm). All 16 CI failures at aa032cc2 fail at the base too -- ZERO bind-order flips -- so this is pre-existing base debt, not a regression. Steward-filed (agents cannot create tracked work per COORDINATION §2).
---

> ## THE FRAME IS OWED. This node is `draft` and NOT startable.
>
> It exists so that a **skipped CI row has an owner**. A skipped row measures
> nothing; the node that owns it owns **un-skipping** it. Size is `TBD`
> deliberately -- nothing measured bounds the repair, and a guessed size on this
> campaign has been wrong every time it was guessed.

> ## A SECOND POPULATION NOW REACHES THIS SIGNATURE. DO NOT ABSORB IT.
> ## Steward, 2026-08-15.
>
> [[RT-REQUIRED-CONSUMER-REACH-CENSUS]] `D1` measured **row 4 depths 2 and 3 of
> the recursor campaign** refusing at `lowering/mod.rs:11550-11552` with **this
> node's exact sentence** (`evt_6qc0vkzj43c0e`, base `a737d8c9b`).
>
> **That is not a reason to widen this node's rows**, and the reason is
> structural rather than procedural. The site is the
> `Lowered::Closure | Lowered::DeclarationClosure` arm of
> **`boundary_transfer_admissibility`** — a **total, wildcard-free walk over the
> whole value graph**. ⇒ **Every closure-carrying graph that attempts the
> crossing refuses here.** A shared sentence is evidence the gate is total, not
> evidence of a shared production root.
>
> **The upstream fork is with the Architect** (`evt_7rpkfc7awktmb`): for the
> recursor rows, is a closure in the crossing graph **correct** — in which case
> the durable lane this node is about is the shared repair and a subsumption is
> real — or **incorrect**, in which case that chain owns a lowering fix that
> never reaches this gate and the convergence is a coincidence.
>
> **Until that is ruled: do not add the recursor rows here, do not cite this
> node as their owner, and do not treat this node's size as covering them.**
> Whichever way it goes, the frame this node still owes is unchanged.

> ## RULED. THE RECURSOR ROWS ARE NOT THIS NODE'S. Steward, 2026-08-15.
>
> **The fork above is settled against subsumption, and this node's scope is
> unchanged by it.** [[RT-REQUIRED-CONSUMER-REACH-CENSUS]] `D5` measured the
> enabled rows at `(closure_present, crossing_reached) = (true, true)` and both
> suppressed legs at `(false, false)`, returning to `StaticWorkerBinding`
> (`dec_35e0tfng528d`, Architect verdict `evt_38p42gjq12br`).
>
> **The Architect's dispositions named exactly one branch under which the two
> populations could be one defect: closure PRESENT under suppression AND the
> crossing ALSO reached under suppression.** The crossing is **not** reached
> under suppression. ⇒ **That branch is excluded by measurement.**
>
> The two branches still live — the realization produces a value that should not
> be closure-shaped, or the value is legitimately closure-shaped and should never
> have been **routed** — **both leave this node untouched by their own terms.**
> The separator between them is [[RT-CROSSING-CALL-SITE-ATTRIBUTION]].
>
> ⇒ **Do not wait on the recursor chain to size this node**, and do not expect a
> shared repair from it. **The frame this node owes is its own**, over the row it
> already lists. The shared refusal sentence was totality, as the predecessor
> block said, and that reading is now measured rather than argued.

## Exact signature

```text
Closure: a closure cannot cross the boundary: it is runtime-local and live-domain only, and it has no durable lane
```

## Rows it owns

- \`rt_escape_second_resource_native\` \`escaped_resource_used_by_fanning_host_op_matches_interpreter\`

## Why this is NOT [[RT-CARRIER-BYTESPAN-OBSERVE]]

**Different mechanism.** A representation/lane gap for closures crossing the
durable boundary, refused at object emission. Not an effect-seat availability
question.

## Provenance

**Fails at frozen base `21fd46dc`, so it is not caused by the de Bruijn
binding repair.** Measured per row with `--no-fail-fast`; see the hazard note
in the D10 handback -- `cargo test` with several `--test` flags is fail-fast
**per binary**, and a partial run reads as a complete one.
