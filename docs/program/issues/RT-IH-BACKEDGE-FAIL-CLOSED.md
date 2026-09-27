---
id: RT-IH-BACKEDGE-FAIL-CLOSED
title: "Refuse at compile time, never lower to RecursiveBackedge, a functional-IH value in a non-tail position such as a Vis K constructor field, closing a latent miscompile that takes the recursive transfer without the response"
status: closed
owner: runtime
size: S
gate: architect
tier: T1
depends_on: []
blocks: [RT-SOURCE-IH-RELAY-K-VALUE]
github: null
origin: "Architect ruling 2026-09-27 (evt_2b0dwgwyb0tvc): RT-SOURCE-IH-RELAY-K-VALUE AC-0a soundness STOP confirmed; a fail-closed correctness repair on main ahead of any representation work. Steward-sequenced after RT-OWNER-VIS-RETURN-PROTOCOL increment 1. Steward-filed per COORDINATION section 2."
---

# Fail closed on a non-tail IH backedge

## Objective

No compiled program contains a translation that is wrong whenever it runs.
A functional-IH value in a non-tail position is refused at compile time
instead of becoming a recursive jump that drops the response `r`.

## Settled inputs (Architect `evt_2b0dwgwyb0tvc`, from Runtime QA's AC-0a)

- **The contract.** `42 §2` makes constructor arguments eager.
  `42 §6.1-6.2` gives a `Vis` K the response `r` only when it is resumed.
- **The miscompile.** The source machine turns a K-field IH marker
  (`source.rs:829-857`) into `RecursiveBackedge` through `ConstructArgument`
  (`source.rs:1643-1660`) and the finisher (`source.rs:5941-5943`). That
  takes the recursive transfer without `r` and abandons the constructor.
- **Reachability does not decide it.** A runtime `-1` is the same
  observation whether or not the edge runs (Check 8). The disposition rests
  on the translation being wrong whenever it executes.
- **The core route is already right.** It takes the defunctionalization seam
  (`calls.rs:268-285`); leave it unchanged.

## Deliverable

A compile-time refusal, not a new value representation, at both ends:

1. **Producer.** A `CheckedComputationalIHInvocation` whose planned call is a
   functional-IH value (`call.arity == 0`, functional worker,
   `worker.declared_arity == 1`) never lowers to `RecursiveBackedge`; on the
   source-machine route it refuses with an exact error. Grep every marker
   arm (at least `core.rs:3574`, `core.rs:15382`, `source.rs:829`).
2. **Consumer.** No non-tail consumer forwards a `RecursiveBackedge`; it
   refuses with an exact error. Grep every `Lowered::RecursiveBackedge`
   match and post a table classifying each site as tail (forwarding lawful)
   or non-tail (refuses). QA's list is the floor: `source.rs:1643`,
   `source.rs:5941`, `core.rs:15550-15625`.

## Acceptance

- **AC-1.** The r2 fixture's observation changes from the runtime
  `UnclassifiedRuntimeTrap(-1)` to the new exact compile-time refusal; r2
  stays ignored, relabelled to that refusal.
- **AC-2 (control).** Removing the refusal restores the old observation.
- **AC-3.** Targeted suites unchanged; Full CI is the breadth gate. Any
  other program that newly refuses compiled the unsound translation: report
  each to the Architect with its site as a soundness finding, never weaken
  the refusal.

## Stop conditions

- A site whose tail status the plan cannot determine: stop to the Architect
  with the site.
- Any kernel, `trusted_base()` or spec change (an operator question).
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the child-2 checkpoint.

## Closeout: premise falsified by measurement; no change

Architect `evt_6x4nk9x3pe0r`. The node closes with no code change, and its
§1a count stays at 0.

- **The measurement** (runtime-implementer `evt_519z9k1pqp47q`, the census
  `evt_87xae9s1b2hr`). A passive counter at the `ConstructArgument` backedge
  arm (`source.rs:1643-1676` at `551a651`), over completed runs, recorded:
  - `px8f_buffer_native`: 6/6 tests, 93 hits;
  - `rt_parity_native`: 185/185 tests, 220 hits;
  - r2, run explicitly: 13 hits, still `UnclassifiedRuntimeTrap(-1)`.
- **The five semantic tuples** over the 313 lawful hits differ only in
  `env.len()`.
- **r2 falls inside that lawful population.** Its Vis577/K matches 44 px8f
  arrivals. Its Vis361/K matches 244 arrivals, including 2 from the agreeing
  native/evaluator differential `fs_write_at_malformed_offset_narrows_to_invalid_offset`,
  with the same origin, child, template and env numbers.
- **What is withdrawn.** The AC-0a soundness stop (`evt_2b0dwgwyb0tvc`) was
  inferred from syntax position and is falsified. The site is the designed
  Vis-K resumption forward. No refusal is authorized anywhere in this node.
- **What remains of r2.** Its defect is only the missing relay K value, owned
  by `RT-SOURCE-IH-RELAY-K-VALUE`. r2 fails closed at runtime at the owner
  Ret-tag check. A future program with this tuple and disagreeing output
  would be a new soundness finding to the Architect.
- **Released.** This node's `blocks` edge on `RT-SOURCE-IH-RELAY-K-VALUE`.
  `RT-OWNER-VIS-RETURN-PROTOCOL` AC-3 pins r2 unchanged: ignored, runtime
  `-1`, and excluded by the relay.

Semantic tuple table (`/tmp/rt-ih-backedge-semantic-tuples-v2.tsv`, SHA-256
`19e1af4f56c6eb4c4b4af60abbbae75665acc6096e211ca96d5aede7d5e7b3b0`), copied
verbatim:

```text
constructor_name	field_index	remaining_count	is_vis_k	field_expr_form	marker_kind	call_arity	callee_index	callee_binding	env0_binding	pending_marker_kind	answer_route	answer_role	next_frame	environment_length	px8f_hits	parity_hits	r2_hits	write_offset_hits
Vis	1	0	true	DirectIHMarker	OrdinaryApplication	0	Some(0)	Value(Specialized(ComputationalRecursorClosure,recursive_unit_body=None,residual=Carried))	Value(Specialized(ComputationalRecursorClosure,recursive_unit_body=None,residual=Carried))	none	DirectScrutinee	Scrutinee	Terminal	13	28	216	4	2
Vis	1	0	true	DirectIHMarker	OrdinaryApplication	0	Some(0)	Value(Specialized(ComputationalRecursorClosure,recursive_unit_body=None,residual=Carried))	Value(Specialized(ComputationalRecursorClosure,recursive_unit_body=None,residual=Carried))	none	DirectScrutinee	Scrutinee	Terminal	14	0	4	0	0
Vis	1	0	true	DirectIHMarker	OrdinaryApplication	0	Some(0)	Value(Specialized(ComputationalRecursorClosure,recursive_unit_body=None,residual=Carried))	Value(Specialized(ComputationalRecursorClosure,recursive_unit_body=None,residual=Carried))	none	DirectScrutinee	Scrutinee	Terminal	17	44	0	9	0
Vis	1	0	true	DirectIHMarker	OrdinaryApplication	0	Some(0)	Value(Specialized(ComputationalRecursorClosure,recursive_unit_body=None,residual=Carried))	Value(Specialized(ComputationalRecursorClosure,recursive_unit_body=None,residual=Carried))	none	DirectScrutinee	Scrutinee	Terminal	18	14	0	0	0
Vis	1	0	true	DirectIHMarker	OrdinaryApplication	0	Some(0)	Value(Specialized(ComputationalRecursorClosure,recursive_unit_body=None,residual=Carried))	Value(Specialized(ComputationalRecursorClosure,recursive_unit_body=None,residual=Carried))	none	DirectScrutinee	Scrutinee	Terminal	22	7	0	0	0
```
