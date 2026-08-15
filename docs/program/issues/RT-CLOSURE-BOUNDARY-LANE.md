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
