# RT-LEXICAL-RECURSOR-CONSUMERS D2i — the productive checked twin

Owner: runtime. Size: M. Node: [[RT-LEXICAL-RECURSOR-CONSUMERS]] (`#6d`).
Architect ruling **`evt_1dgwdvxhnabg4`** — this frame is that ruling made
executable, and where the two differ the ruling wins.

**Seat tier: T1.** The `#8` suspension does not reach `#6d`.

**RELEASED 2026-08-11 on `main` `2a2e311e`.** Re-derive your merge-base from
`origin/main`; do not take a SHA from this frame.

## Why this WP exists, stated so nobody re-derives it

[[RT-LEXICAL-RECURSOR-CONSUMERS-D2h]] hard-stopped before any candidate. Its key
plane compiled and ran and **interned zero fusions**, because the landed `D2g`
twin has shape `Construct[LexicalClosure[inner match]]` and production discovery
treats `Construct` and `LexicalClosure` as **terminal**. No producer/consumer
pair is ever presented, so nothing positive can fire.

**`D2g`'s `AC-1` is true and was the wrong criterion.** It pinned the
producer-to-IH-consumer relation through `CheckedIhBinding` — binder resolution
— which still passes on the twin. **Result-flow membership is a different
relation**, and nothing in `D2g` consumed the pair, so nothing in `D2g` could
have caught it. **That is a Steward framing defect, not an implementation or
review miss.** It is recorded here so the correction is not mistaken for a
criticism of the ring that found it.

**`D2g` is not reopened and `D2h` is not re-scoped.** `D2h` is respun unchanged
on this WP's output and keeps its own thread.

## The mechanism question is RULED. Do not reopen it.

`continuation_result_origins` owns the exact result-position closure of one
planner-issued root, and **its current traversal is semantically right**:
checked wrappers are transparent; a `Let` contributes its body, an `If` its
branches, a `Match`/`ComputationalMatch` its reachable case bodies; and
`Construct`, `Closure`, and `LexicalClosure` are **values, therefore terminal**.

**A closure stored in a constructor field is not the result of that
constructor.** Descending through either terminal would convert contained values
and dormant closure bodies into results of the enclosing expression, and would
destroy the disjoint seed-versus-worker-body ownership that existing
continuation discovery and emission-owner reasoning rely on. **That is a
semantic widening, not a repair.**

> **Do not change `continuation_result_origins`, and do not add a
> `Construct`/`LexicalClosure` descent for fusion discovery.** If your positive
> requires either, that is a hard stop and it comes back to me.

## Deliverable

**Additive, and the additivity is the point.** Retain the landed terminal twin
**unchanged** as a discriminator, and **add a productive checked sibling** on
which the production planner issues a result root whose result-flow population
contains the exact producer construct consumed by the exact computational
continuation.

The sibling may carry both roots as the real `R3` witness does, or another
arrangement with the same planner-issued relation. **The shape is not the
authority — the six facts below are.**

Deliver the productive fixture, its complete oriented plan, and the causal
observations. Nothing else.

## AC-1 — the required authority, six facts on ONE productive checked artifact

All must hold simultaneously.

1. **The actual continuation-discovery path — not a test reimplementation —
   issues the consumer and the productive result root.**
2. `continuation_result_origins` for that exact root **contains the exact
   producer `Construct`**, and the **terminal sibling root does not**.
3. The producer construct matches the exact consumer alternative, and its
   recursive argument is the exact IH binding from `build_checked_ih_bindings`.
4. The selected case's exact consuming `Call` resolves to that same
   consumer-frame/recursive-position binding.
5. The frame, selected slot, and invocation at that call resolve against **one
   complete, independently authored oriented plan**. The existing per-slot
   outer/inner constructor pins and the runtime-only marker-relocation
   discriminator **remain intact**.
6. The producer invocation edge, emission owner, owner split, complete ordered
   input projection, and exact consuming suffix are **projectable** on this
   artifact.

**Re-derive every occurrence coordinate. No numerical coincidence with `D2d` or
with the landed twin is evidence** — a coordinate that happens to match across
two artifacts is right by accident.

> ### THE PRODUCTIVE ROOT MUST BE PLANNER-ISSUED
>
> A test that calls `continuation_result_origins` with an **arbitrarily selected
> inner origin**, or that **searches structurally** for an inner match and
> treats it as a root, is **manufactured authority and does not discharge this
> ruling**. The whole failure being repaired is a fixture that satisfied a real
> relation nobody needed; **satisfying this one by construction would repeat it
> exactly, one layer down.**
>
> Observe the facts through a **test-only capture at the production candidate
> boundary** if you need to. **Do not create a second discovery algorithm in the
> test.**

## AC-2 — three populations, established SEPARATELY

The distinction is only causal if each of these is its own observation.

| population | what must hold |
|---|---|
| **landed terminal checked twin** | transport validates **and** the `CheckedIhBinding` relation holds, **and** production discovery presents **zero** fusion producer/consumer pairs for that consumer |
| **productive unmarked sibling** | the exact result-flow pair and exact consuming suffix exist, **and no checked coordinate resolves** |
| **productive checked sibling** | pair, suffix, and plan-backed coordinate all exist together, and **exactly one** complete pre-interning fusion candidate reaches the `D2h` handoff boundary |

**The first row is the regression control and it is the one this whole WP
exists to install:** it proves binder and transport truth **do not imply**
result flow. Had it existed in `D2g`, `D2h` would not have stopped.

The second row keeps **transport absence as a distinct, later gate** — do not
let it collapse into the pair-absence case.

## AC-3 — the informative mutation, and its converse

**Forward.** Change **only result-root reachability**, keeping the binder
relation and checked transport valid: place the productive inner computation
behind the value boundary, or select the terminal sibling root. **The exact
candidate population must go from one to zero before interning.**

**Converse, and it is the one that catches a lazy refusal.** Stripping or
transplanting transport **while preserving the productive result-flow pair**
must fail **at the transport gate** — it must **not** be reported as absence of
the pair. Two different refusals, two different reasons; a mechanism that
reports both the same way passes both halves for the wrong reason.

## Excluded scope — the stop line

**This WP contains no fusion key, id, descriptor, interner, re-derivation
validator, ABI, emission, edge redirection, or `R3`-green claim.** `D2h` resumes
unchanged only after this lands.

**Stop and return to me if producing the positive requires any of:**

- descending through `Construct` or `LexicalClosure`;
- injecting an unissued result root;
- structural search for the producer, the consumer, or the checked markers;
- weakening the existing `D2g` transport plan or its constructor pins;
- beginning the `D2h` key plane here.

**The discarded `D2h` plane remains correctly discarded.** A plane with zero
positive candidates must not land, and rebuilding it early inside this WP is the
same error with a different frame around it.

## Contention

`crates/ken-runtime`. Language is on `crates/ken-elaborator` under
[[LANG-SPACE-PRESTATE-BIND]] — no intersection. **No `spec/` or `conformance/`
path, so no Spec vote** on the merge Decision.

## Validation

Targeted only. `-p ken-runtime`, or `--test <name>`, **never `--workspace`**.
"No regression" means green in CI.

## Sizing

One turn to a releasable increment or a genuine hard stop. **Both are good
outcomes, and on this node the stops have been worth more than the increments.**

`D2h`'s stop was the fourth on `#6d` and every one has been a framing defect of
mine. **If this frame is wrong in the same way — an acceptance criterion that
measures a real property the next deliverable does not consume — say so and
stop.** You have now done that twice, correctly, at real cost to your own
completed work, and it is the behaviour I want rather than a candidate that
passes.
