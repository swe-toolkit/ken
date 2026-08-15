---
id: V3-FO-OBLIGATION-SIGNATURE-DISCOVERY
title: "Decide and build how an incoming obligation is matched to an FO slice signature, so route FO's public entry point can reach the embedding at all"
status: ready
owner: language
size: L
gate: none
depends_on: [V3-FO-KRIPKE-SLICE]
blocks: []
github: null
origin: "Steward, 2026-08-15. Filed as the named home for the public-route residual cut out of V3-FO-KRIPKE-SLICE D5/AC-5 (disposition in that node's leading banner). Architect finding evt_30mehjtecaazy; QA block on e0474679c upheld. The load-bearing mechanism was re-derived by the Steward from crates/ at e0474679c before filing. Steward-filed per COORDINATION section 2."
---

## The gap, stated as a fact about the tree

**Route FO is built and unreachable.** `attempt_fo` (`prover.rs:545` at
`e0474679c`) opens with:

```rust
let sig = crate::fo_kripke::declare_fo_slice_signature(env);
attempt_fo_with_signature(env, ctx, phi, phi_closed, &sig)
```

`declare_fo_slice_signature` calls `declare_postulate` and `declare_inductive`,
**which mint fresh `GlobalId`s on every call.** It is not idempotent, not even
against the same `env`: two calls yield two disjoint signatures.

⇒ **The signature `attempt_fo` quotes against is unreachable by any caller**, so
`quote_fo` refuses every externally-constructed term and the route falls through
to the unchanged IPC fallback. **Every real obligation, always.** The slice's
quotation, embedding, certificate search, and checker are correct and no program
can reach them.

**Two consequences worth separating.** The capability gap is that route FO does
nothing new in production. The measurement gap is that any probe driven through
`attempt_fo` is **inert** — it returns `Unknown` by quotation refusal rather than
by the theorem-home boundary, and would return exactly the same with the slice
deleted.

## The decision this node exists to make, and it is not the ring's

The missing step is **deciding which signature an incoming obligation belongs
to.** That is a design call with its own review, per `evt_30mehjtecaazy`, and it
must be made before anything is built.

> ### THE CHEAP REPAIR IS THE DANGEROUS ONE. Named so nobody reaches for it.
>
> The obvious fix is to have `attempt_fo` adopt the signature implied by the
> incoming term's own `GlobalId`s. **Do not build that, and in particular do not
> build it to make an acceptance criterion pass.**
>
> It makes the term dictate the signature it is quoted against, so **`embed` is
> computed over predicates the caller chose.** Signature selection sits upstream
> of everything the embedding's soundness argument assumes. A route where the
> obligation picks its own signature is a surface to be designed deliberately,
> not introduced as test-enablement.
>
> **The shape, which recurs:** the cheapest-looking repair is cheap precisely
> because it skips the part that needed deciding.

## THIS NODE REMOVES THE FIRST OF TWO GATES THAT ARE MASKING EACH OTHER

**Architect condition on the `D0` ruling, `evt_7r29t8139t4p2`. Read this before
`D0`, not at `D3`.**

Route FO cannot return `proved` today for **two independent reasons**:

1. **Quotation refuses every real obligation**, so nothing reaches the boundary.
2. **`23-prover.md §4.4` forbids `proved`** until `embedding_adequacy` and
   `checker_soundness` are kernel-checked in an approved home — an Architect and
   operator item, still unsettled.

**Gate 1 is currently doing all the visible work, and gate 2 has never been under
load.** This node removes gate 1. The moment quotation starts succeeding, **`§4.4`
becomes the only thing between an accepted certificate and `proved`** — and it
becomes load-bearing for the first time inside a node whose subject is signature
matching, reviewed by people reasoning about signatures rather than about theorem
homes.

⇒ **That is exactly the shape in which a fail-safe gets refactored past by
someone who does not know it is the last one.**

**Two conditions follow, and they bind:**

- **`D0` must state the `§4.4` interaction explicitly as part of the soundness
  obligation**, not only the signature-matching rule.
- **`AC-2`'s mutation must be aimed at the `§4.4` gate, not at quotation.** A
  discrimination test that only proves quotation started working leaves the
  second gate untested at the precise moment it becomes the only one.

## Deliverables

**`D0` — the design question, posed to the Architect before any code.** State
the candidate rule for matching an obligation to a signature **as an attackable
claim**, with the soundness obligation it must discharge: what prevents the
obligation from selecting the predicates its own embedding is computed over.
**And state the `§4.4` interaction explicitly**, per the two-gate section above —
a `D0` that covers only the signature-matching rule does not meet the Architect's
stated condition. **Hand this to the Architect and stop.** `D1` onward is gated
on that ruling.

**`D1` — the discovery mechanism**, built to whatever `D0` rules. Recognition
must be by a checked structural property, not by ambient state and not by
declaration order.

**`D2` — `attempt_fo` reaches the boundary on a real obligation.** The public
route quotes an externally-constructed positive control, finds its certificate,
and arrives at the `23 §4.4` boundary.

**`D3` — the inherited residual, discharged.** The `AC-5` fail-safe is
demonstrated **through the public `attempt_fo`**, not through
`attempt_fo_with_signature`: an obligation whose certificate `check_cert`
genuinely accepts yields `Unknown`, never `Proved`. **Assert the acceptance as a
precondition** — without it the test passes via the IPC fallback and measures
nothing.

**`D4` — a refusal that is still a refusal.** An obligation outside the slice
fragment is refused by `quote_fo` as before. Discovery must not widen what
quotes.

## Acceptance criteria

**`AC-1`.** No signature-selection rule is landed that `D0`'s ruling did not
authorize. **If the work seems to require deciding it, that is the handback.**

**`AC-2`.** `D3`'s public-route test fails if the boundary is removed.
**Demonstrate the discrimination by mutation** — an inert probe is what produced
this node, and a second one would not be caught by the same reasoning twice.

**Aim the mutation at the `§4.4` gate, not at quotation** (Architect condition,
`evt_7r29t8139t4p2`). The term under test must be one that **now quotes
successfully and carries an accepted certificate**, and it must still yield
`Unknown`. A mutation that only shows quotation started working proves gate 1 and
leaves gate 2 — by then the only gate — unmeasured.

**`AC-3`.** Route FO does not return `proved`. The `23 §4.4` reservation stands
until both theorems are kernel-checked in an approved home, and **this node does
not discharge it.**

**`AC-4`.** No new kernel primitive and no trusted axiom.

**`AC-5`.** The slice fragment is not widened. More sorts, predicates,
connectives, or `Cert` constructors remain a later node.

**`AC-6`.** No-regression, in CI (`COORDINATION §12`).

## Banned scope

- **Deciding the theorem home.** Still an Architect and operator call, still not
  this ring's, and it is not unblocked by this node.
- **Widening the slice.** `§4.5`'s bounds are unchanged.
- **Returning `proved` for FO.** `AC-3`.

## What this node does not settle

**It does not make route FO return `proved`**, and landing it changes no
verdict from `Unknown` to `Proved`. It makes the slice *reachable*, which is the
precondition for the theorem-home work to matter at all.

## Provenance

`V3-FO-KRIPKE-SLICE`'s disposition banner; Architect finding
`evt_30mehjtecaazy`; QA block on `e0474679c`. `attempt_fo` and
`declare_fo_slice_signature` read at `e0474679c` by the Steward, who re-derived
the non-idempotence rather than inheriting it.
