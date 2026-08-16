---
id: RT-FRONTEND-REACHABILITY-TRIPWIRE
title: "Nothing in the tree would notice if a frontend change made one of the source-unreachable refusal shapes constructible -- the emitter fixtures that look like they cover this bypass the parser and elaborator entirely"
status: draft
owner: runtime
size: M
gate: none
depends_on: []
blocks: []
github: null
origin: "Architect evt_5h7vzc27mc11j, correcting a Steward claim in the RT-RECURSOR-TRANSPORT Trap-2 fork. Filed draft and NOT released -- this is not lane 1 and the second lane is quiet. Filed at all because an acknowledged gap with no node is indistinguishable from a gap nobody noticed, and this is precisely the gap a future reader would assume the existing fixtures cover."
---

## The gap

`RT-REFUSAL-SOURCE-WITNESS-OR-INVARIANT` (PR #2440) established four refusals
as compiler asserts/invariants on the ground that **no admitted Ken source
program can reach them**. That claim is about the current grammar, the
elaborator's admission rules, and the kernel gates.

**Nothing re-checks it.** If a later frontend change made one of those shapes
constructible from source, the disposition would silently become false and the
refusal would become a real, reachable capability gap wearing an invariant's
label.

## Why the obvious candidate does not cover it

The natural assumption is that the emitter fixtures pinning those refusals
would catch it. **They cannot.**

Those fixtures — `host_result_closure_match(px8j_*)` at
`core/tests/control.rs:6820-6826`, through `d2k_wall_under_current_selector`
(`:5569`) to `emit_process_entrypoint_object_with_cranelift` — feed
**hand-authored `RuntimeExpr` straight to the emitter**. No parser and no
elaborator run in them.

⇒ **They measure the emitter's response given the shape.** A change in
`elab.rs` that made row 4 depth 1 reachable from source would leave every one
of them green.

> **Their honest charter is "internal-contract pin on the emitter's refusal,"
> and that is now written into
> `docs/program/wp/RT-RECURSOR-TRANSPORT.md`:** *"these five rows therefore
> remain current internal emitter-contract pins; they cannot observe frontend
> reachability."*
>
> **Labelling them a reachability tripwire would be worse than leaving them
> unlabelled**, because a control whose stated purpose is one layer off from
> what it measures reads as covered. That is the mislocalization recorded on
> `RT-ROOT-AUTHORITY-BLAME-DOMAIN`.

## Why this is hard, and why it is not just a renaming

A real tripwire must be a **source-level** instrument: it has to start from
`.ken` text and establish that the shape still does not elaborate.

**But `RT-REFUSAL-SOURCE-WITNESS-OR-INVARIANT` `D3`'s whole argument is that no
such source program exists.** So the instrument cannot be "a `.ken` file that
reaches the refusal" — there is none, by the very result being protected. It
has to assert a **negative** over the frontend, and the method gate that node
established applies here in full: **a negative existence claim is not
established by failed attempts.** It must rest on named grammar productions,
admission rules, or kernel gates, and state its population.

**That is the design problem this node exists to solve, and it is not obviously
solvable.** Do not open it by writing fixtures.

## Deliverables

**`D0` — decide whether a sound instrument is possible at all.** Can the
negative be pinned against the frontend's own structure — a grammar production
census, an admission-rule enumeration, a kernel-gate list — such that a change
which opens the shape necessarily breaks the pin? **A recorded "no, and here is
why" is a complete and acceptable result**, and it converts this from a silent
gap into a stated one.

**`D1` — only if `D0` is yes: build it**, over the population `D0` names, with
a mutation proving the pin fails when the frontend admits the shape.

## Acceptance criteria

**`AC-1`.** Any instrument built starts from `.ken` source or from a structural
property of the frontend. **An instrument taking `RuntimeExpr` does not
discharge this node**, whatever it is named.

**`AC-2`.** The claim travels with its population — measured corpus, superset,
or corpus-independent argument. `D3`'s walk-back on
`RT-SYNTHESIZED-ENV-RECORD-OCCURRENCE` is the precedent.

**`AC-3`.** A mutation control: make the frontend admit one of the shapes and
show the instrument goes red. **Without this the instrument is unmeasured**,
and an unmeasured tripwire is the defect this node was filed against.

## Do not re-measure

The five rows' provenance is settled and recorded in
`docs/program/wp/RT-RECURSOR-TRANSPORT.md` at PR #2443: all are test-only
hand-authored `RuntimeExpr` or planner structures, verified per row. The
Architect confirmed the R3 seed is **not** a member of production
`nc5_seed_examples()` (`ir.rs:1084`).
