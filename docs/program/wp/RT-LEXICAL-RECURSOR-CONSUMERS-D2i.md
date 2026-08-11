# RT-LEXICAL-RECURSOR-CONSUMERS D2i — the discovery ledger and its enumerator

**The title said "the R3-derived artifact and the discovery ledger" until the
scope ruling below retired the R3-derived half.** Read that ruling before the
deliverables; where it and the body differ, the ruling wins.

Owner: runtime. Size: **L**. Node: [[RT-LEXICAL-RECURSOR-CONSUMERS]] (`#6d`).
Architect ruling **`evt_2x157jk8bmpxk`** — this frame is that ruling made
executable, and where the two differ the ruling wins.

**Seat tier: T1.** The `#8` suspension does not reach `#6d`.

**RE-CUT 2026-08-11 on `main` `8898c426`.** This supersedes the first `D2i`
frame entirely. Re-derive your merge-base from `origin/main`.

> ## THE FIRST CUT WAS WRONG AND `a334b9a0` IS EVIDENCE ONLY
>
> **Do not extend `a334b9a0` as a candidate and do not publish it.** Its
> "productive" sibling proved pair, suffix and transport, but **changed away the
> producer unit boundary** to get result flow. It is useful only as an
> edge-absence refusal if retained.
>
> **The measured absence was never authority to invent a third fixture**, and it
> is not a reason to widen `continuation_result_origins`. Two synthetic shapes
> each obtained one half of what `R3` has; a third would have been a guess.
> **Runtime stopped instead of guessing, twice. That was right both times.**

## The relation already exists in production — nobody had to invent it

`build_continuation_specialization_plan` seeds the outer computational match,
interns the ordinary closure specialization, and **only after that successful
insertion** enqueues:

```
ContinuationDiscovery {
    continuation_origin: <the same consumer>,
    result_root: worker.body_origin,
    enclosing_specialization: Some(target),
}
```

On the real `R3` before-hole witness **that second, planner-issued root is
origin 18, and its result population contains producer origin 23.** That is also
why `R3` carries the unique `StaticBody` edge into producer owner 2 while the
direct-root sibling does not.

⇒ **`R3` was never a luckier shape. It is the shape production actually
issues**, and both prior fixtures were missing the second discovery rather than
missing a structural trick.

## Deliverable 1 — the R3-derived checked artifact

**Replace the positive synthetic sibling with a checked/unmarked variant of the
actual `R3` before-hole builder.**

- **Preserve the `Construct[LexicalClosure[...]]` and function-unit boundary
  exactly.** That boundary is what produces the `StaticBody` edge; the previous
  cut destroyed it by making the inner frame a scrutinee.
- **Add only checked wrappers, through the same parameterized builder.**
- **Assert that erasing those wrappers yields the unmarked `R3` expression.**
  That assertion is what makes it *derived* rather than *resembling*.
- Author and **positively validate one complete oriented plan** at the **freshly
  derived** marker locations.

**Banned:** copying the `R3` shape into a second planning-only builder, and
post-processing an already-built tree by structural search. **Either would
reintroduce exactly the divergence this WP exists to remove.**

## Deliverable 2 — fusion enumeration consumes production-issued discoveries

**The current enumerator is not that, and this is the defect.** It independently
scans every computational match and uses only `child(consumer, 0)`, so **it can
never see the worker-body root that production issues after ordinary
specialization.**

- **Record each discovery only after the production `visited.insert` admits
  it.**
- **Return or pass that closed ledger from the same
  `build_continuation_specialization_plan` invocation.**
- Fusion enumeration **may** walk `continuation_result_origins` for those ledger
  roots.
- It **may not** reconstruct seeds, scan worker bodies, or run a parallel fixed
  point.

Ordinary specialization may already be interned. **No fusion id, key, or
descriptor may exist yet.**

> ## THE CLOSED HANDOFF CONTRACT — no further gate may be discovered piecemeal
>
> **This list is the point of the re-cut.** Five stops on this node, three of
> them a fixture lacking a property the *next* gate consumed. The Architect has
> now closed the set. **Before `D2h`, one candidate must carry or expose all of
> these exact facts:**
>
> 1. the admitted production `ContinuationDiscovery` — consumer origin, result
>    root, and enclosing specialization/context;
> 2. producer construct origin and owner, matching alternative, recursive
>    position, exact argument origin, and its producer-side `CheckedIhBinding`;
> 3. selected case-body origin, the exact consuming `Call`, its callee origin,
>    and the consumer-frame/position `CheckedIhBinding`;
> 4. the resolved frame/slot/invocation `CheckedTransportCoordinate`;
> 5. the unique exact `StaticBody` edge triple — emission caller, producer
>    callee owner, and callee entry;
> 6. the producer/consumer owner split and exact result-edge membership;
> 7. the **complete ordered** `ContinuationProducerEnvironment.inputs`
>    projection, every required slot closed, unique, and source-slot validated.
>
> **These are pre-interning facts, not key or interner work.**
>
> ### Two corrections that follow, and both are easy to get wrong
>
> - **`continuation_inputs: usize` is NOT the complete ordered projection.**
>   `D2h` keys and later validates the projection itself; **retaining only its
>   length is insufficient.**
> - **Retain the selected case-body origin NOW, alongside the exact call.** It
>   **must not** be recoverable later from "the case containing the call" —
>   that is a structural re-derivation of a fact you were holding.

## AC-1 — three populations, established separately

| population | what must hold |
|---|---|
| **landed `D2g` terminal checked twin** | binder relation and transport hold; **zero** result-flow pair |
| **`R3`-derived productive unmarked** | production-issued pair, suffix, and edge; **no checked coordinate** |
| **the same `R3`-derived productive checked artifact** | the same pair, suffix and edge, **plus** one plan-backed coordinate, reaching **exactly one** pre-interning candidate |

The second and third are **the same artifact** differing only in checked
wrappers — that is what the erasure assertion in Deliverable 1 buys.

## AC-2 — the causal root control

**Suppress only the production descent after the ordinary closure specialization
is inserted.** Then:

- the worker-body discovery **disappears**;
- the fusion candidate count goes **1 → 0**;
- **the initial terminal root remains unchanged.**

That third clause is what makes it causal rather than a blunt disable.

## AC-3 — the transport converse, and multiplicity

**Strip or transplant only the checked transport** on the productive artifact.
It must **refuse at transport** — **not at pair discovery and not at edge
discovery.** Three gates, three distinct refusals; a mechanism that reports them
alike passes for the wrong reason.

**Multiplicity refuses.** More than one otherwise-matching issued discovery, or
more than one matching producer edge, is a refusal. **Do not choose one.**

## Excluded scope and stop line

**No semantic traversal changes.** `Construct`, `Closure`, and `LexicalClosure`
**stay terminal**. `D2g` stays closed. `D2h` stays held.

**Contains no** fusion key, id, descriptor, re-derivation validator, ABI,
emitter, edge redirection, or `R3`-green claim.

**Stop and return to me if the positive requires** widening the traversal,
reconstructing seeds, scanning worker bodies, running a parallel fixed point,
a second planning-only builder, structural post-processing of a built tree,
choosing among multiple matches, or beginning the `D2h` key plane.

## Declared partial seam — land the ledger without waiting for the whole

**This is size L and I am not pretending otherwise.** Under the accepted-partial
policy you may land at one declared seam:

> **Deliverable 2 plus the `R3`-derived *unmarked* artifact**, with `AC-1`'s
> first two populations and `AC-2`'s causal root control.

That is a coherent, independently reviewable increment: the enumerator consumes
production-issued discoveries, and the pair/suffix/edge are shown on a real
derived artifact. **The checked variant, the plan-backed coordinate, and the
closed fact list then follow as the remainder.**

**Take the seam if the whole does not fit one turn.** Do not take it if the
ledger is not independently exercised — a ledger with nothing enumerating from
it is the same green-by-construction shape you correctly refused twice.

> ### SEAM SPLIT FURTHER — Steward disposition 2026-08-11, `evt_5sr5hdnqqtxbk`
>
> **This records a scope ruling, not a landed fact.** Runtime released exact
> `401c2c96` — the discovery ledger alone, below the seam above, and said so
> plainly rather than describing it as meeting it. **I ruled it an accepted
> partial and released the hold**; it does not wait for the R3-derived artifact.
>
> **The seam's one substantive condition was met.** The ledger is exercised in
> both directions on the landed `D2g` twin: **every seed is admitted**, and
> **strictly more pairs are admitted than the seeds can name.** Containment
> alone would pass on a ledger that merely echoes the seeds — the vacuous shape
> — so the strict extension is the half that carries the weight.
>
> **The rest of the seam was bundling, not gating.** I attached the unmarked R3
> artifact to it because I could not then name a way to exercise the ledger that
> did not go through R3. The implementer found one.
>
> **`Deliverable 2` has two halves and the review must say which landed:**
> recording the closed ledger, and **making fusion enumeration consume it**
> instead of `child(consumer, 0)`. Either answer merges. **A candidate described
> as "Delivery 2" while carrying one half is the exact shape that caused three
> of the five stops on this node** — a later gate consuming a property an
> earlier claim was read as having delivered.
>
> **Residual after `401c2c96`:** the R3-derived unmarked artifact; the
> R3-derived checked artifact; the three-population matrix; `AC-2`'s causal root
> control; `AC-3`'s transport converse and multiplicity refusals; the closed
> 7-fact handoff contract; and, if enumeration still keys on the seed, the
> consuming half of `Deliverable 2`.

> ### SCOPE RULING — Steward, 2026-08-11, `evt_57ypzyard9jjx`
>
> **The ledger partial `eaaaf141` merged at `b7142fe5`** (one path, +100/-0). It
> establishes the root source and claims nothing beyond it: no live enumerator,
> no production consumer, no removed seed path, and the ledger equality
> explicitly labelled an alias observation.
>
> Making the enumerator live then measured **the landed `D2g` terminal twin as
> productive** on a production-issued ledger descent root, carrying one full
> seven-fact candidate. **That inverts the premise `AC-1` was built on.**
>
> `AC-1`'s first population is the twin with **zero** result-flow pair. The
> `R3`-derived sibling exists *because* the twin was measured as having no pair
> to observe. The reconciliation is that both measurements are right and a
> qualifier is missing: **zero pair under seed enumeration, a pair under ledger
> enumeration** — which is the merged partial's own claim, that the ledger
> admits roots no seed scan can name.
>
> ⇒ **The `R3`-derived sibling and the three-population matrix are NOT owed if
> the twin carries the seven facts.**
>
> **But this removes the matrix's negative population rather than shortening
> it.** All three rows would be productive, and nothing would then discriminate
> *"finds a pair where production issues one"* from *"finds a pair anywhere it
> looks."* **`AC-2`'s causal root control replaces it and is not optional** —
> suppressing the production descent must take the count 1 to 0 with the initial
> terminal root unchanged. That is a stronger negative than a second fixture,
> because it is causal rather than comparative.
>
> **State the twin's pair count under both enumerators, naming each.** Restating
> the old number as simply wrong hides the finding: a reader seeing only the new
> count cannot tell whether the twin changed or the instrument did.
>
> **`AC-3` is untouched** — three gates, three distinct refusals, multiplicity
> refuses rather than choosing.
>
> **Also fold in `evt_htsm7kmzzv5g`:** the merged `static_transition.rs:14631`
> control asserts `len() > len()`, which is **cardinality where its own prose
> claims containment** — a ledger that lost a seed while gaining two unnameable
> roots passes it. Compare on the `(continuation_origin, result_root)`
> projection, not the full triple: `PartialEq` derives over all three fields and
> the seed reconstruction hardcodes `enclosing_specialization: None`, so a
> full-triple superset test fails for a reason unrelated to the defect.
>
> **The seven-fact confirmation is the Architect's**, not mine. A fact outside
> the seven is the closed-contract failure this frame told you to report
> immediately, and it comes to me.

## Contention

`crates/ken-runtime`. Language is on `crates/ken-elaborator` under
[[LANG-SPACE-PRESTATE-BIND]] — no intersection. **No `spec/` or `conformance/`
path, so no Spec vote.**

## Validation

Targeted only. `-p ken-runtime`, or `--test <name>`, **never `--workspace`**.
"No regression" means green in CI.

## Sizing, and what I owe you

**This is the fifth stop on `#6d` and the pattern in the first four was mine:**
each frame required the properties I could name, and each next gate consumed one
I had not. `D2g`'s `AC-1` pinned binder resolution; `D2h` needed result flow.
The first `D2i` pinned result flow and transport; the enumerator needed a
`StaticBody` edge.

**The closed handoff contract above is the fix for that**, and it is the
Architect's list rather than mine — which is the point. **If a gate beyond it
still consumes something unnamed, that is a defect in the contract and I want to
know immediately**, because it means the piecemeal discovery is not closed and
the next cut has to be structural rather than another slice.
