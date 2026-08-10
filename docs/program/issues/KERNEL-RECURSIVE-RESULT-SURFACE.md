---
id: KERNEL-RECURSIVE-RESULT-SURFACE
title: "A source term that denotes the kernel-supplied recursive method result for a lifted recursive field -- the missing surface capability that makes an unbounded residual-All fold expressible"
status: ready
owner: spec-enclave
size: M
gate: none
depends_on: [KERNEL-NESTED-IND]
blocks: [DS-9]
github: null
origin: Architect ruling evt_2s6gmzqvaj5mr (2026-08-10), issued after the conformance-validator rejected three KERNEL-NESTED-IND D6 candidates (dec_7d46tfm6pp3mq, dec_1r4sxfr3j2gs7, dec_8pyjkfs3qv7m) and kernel-implementer grounded the exact source-level obstruction at evt_7bx469t75cd2y. The ruling directs that D6 be recut to seven cases with nested-size-uses-lift gated, and that this capability become a separate node. Steward-filed (agents cannot create tracked work per COORDINATION §2).
---

> # THIS NODE IS A SPEC CONTRACT FIRST. IT IS NOT AN IMPLEMENTATION TICKET.
>
> The Architect has approved a **semantic shape** and explicitly refused to fix
> its **surface spelling**: *"do not freeze the metaname `recursive-result` or
> implement it as an unresolved magic identifier inside Kernel's node — the
> surface spelling, scoping, diagnostics, and interaction with ordinary
> direct/W-style matches require a Spec-owned contract and Language/elaborator
> implementation frame."*
>
> ⇒ **`recursive-result` below is metanotation, not a proposed keyword.**
> Choosing the actual spelling is the first deliverable, not a decision already
> made. ⛔ Do not implement against the metaname.
>
> **Status is `draft` and this node is NOT released.** It is framed so the
> capability is tracked rather than living in a thread, and so
> `nested-size-uses-lift` has a named blocker. Releasing it is a priority call
> against the current lanes and is the operator's.
>
> ## 2026-08-10 — THE PRIORITY CASE CHANGED. It now blocks a THIRD thing.
>
> **Architect `evt_6ysrp62e4zayg` ruled that this obstruction extends to
> `List`-carried recursion**, so **DS-9's `D3`+ unbounded Json fold over arrays
> and objects is blocked on this node.** That is Foundation's next work, and
> DS-9 `D1`/`D2` have both merged, so this is no longer a conformance-only
> blocker sitting behind an idle row. What it blocks now:
>
> 1. `nested-size-uses-lift` (`seed-nested.md`)
> 2. `nested-dependent-motive-uses-lift` (`seed-nested.md`)
> 3. **DS-9 `D3`+**, at slice granularity — `JsonArray : List Json` and
>    `JsonObject : List (Pair String Json)`. `D2`'s standalone `List Char`
>    recursion does **not** share the blocker and is not reopened.
>
> **`blocks:` now names DS-9, and that edge is documentation.** DS-9's
> frontmatter deliberately does **not** carry the reciprocal `depends_on`,
> because the Architect scoped the dependency to the first `D3`+ slice
> promising an unbounded fold, not to the node. `gen-progress.sh` reads
> `depends_on`, so this pairing will not render as a graph edge — that is
> intended, not an omission.
>
> **Size set to `M`** (was `TBD`). It covers `D0` and `D1` only; implementation
> remains an uncreated successor owned by Language/elaborator.
>
> ## `D0` does not depend on what is holding this node's parent
>
> `depends_on: [KERNEL-NESTED-IND]` is whole-node, and `KERNEL-NESTED-IND` is
> `active` **solely on `AC-K12`, which is Runtime-blocked**. `D0` is a Spec
> contract about surface spelling, scoping, diagnostics, and interaction with
> direct/W-style matches. **None of that is a function of `AC-K12`.**
>
> ⇒ **The operator's call is not gated on Kernel finishing.** If the answer at
> 11:30Z is yes, `D0` can start immediately, and it consumes a spec-enclave
> seat rather than one of the build lanes the cap applies to. Recorded so the
> priority call is a priority call and not an inherited sequencing assumption —
> a constraint can be practically binding for a reason that is false.

## Why this exists

`conformance/kernel/inductive/seed-nested.md`'s `nested-size-uses-lift` requires
`size (node (join (one leaf) (one (node empty))))` to reduce to `3` — a fold over
**all** `Bag`-indexed Nat leaves, at arbitrary depth. **No function expressible in
today's surface satisfies it**, and three cleanly-built `KERNEL-NESTED-IND` `D6`
candidates were rejected proving exactly that, each moving the counterexample one
level deeper.

**The values the fold needs already exist.** In an `All_Bag.Join` method,
`method_type` supplies one recursive method result for each recursive support
field, positioned after the support constructor's fields and evidence. The gap is
purely that **nothing in the source can name one.**

## The exact obstruction, measured

Grounded by kernel-implementer at `evt_7bx469t75cd2y`, against
`d7681153a0f6e155b7abf9bb3d386cc2f2b13c77`. Four facts, and each closes one
candidate route:

| # | fact | route it closes |
|---|---|---|
| 1 | `check_match_with_lift` puts every binder past `host_ctor.args.len()` in `hidden_positions`, and `surface_var` skips them | no source variable or pattern can name the recursive support IH |
| 2 | surface pattern arity is fixed to the host constructor's real fields | widening the pattern is not available |
| 3 | the aligned `LiftBinding` for a recursive field carries its generated evidence position plus `support: Some(All_Bag)` — and **no** surface-accessible fold result | the binding itself is not the answer |
| 4 | the self-call rewrite fires only for a direct guest leaf with `support: None`; a recursive `Bag` binding is `support: Some`, and `liftSize xs` is ill-typed at `Bag LiftRose` versus `LiftRose` | neither the rewrite nor a user helper reaches it — a helper reconstructs Rose self-calls instead of consuming the supplied result |

⇒ **The only legal source consumer of a `support: Some` binding is another
`match` on the residual source `Bag`.** That is a fresh finite elimination, so
repeating it is necessarily a finite unroll with a deeper fallback. **Finite
unrolling is the maximum expressible shape today, and it is not a fold.**

## Two repairs are PROHIBITED, and they are the tempting ones

Architect, `evt_2s6gmzqvaj5mr`:

1. ⛔ **Do not make hidden binders visible** through `surface_var`, and do not
   add them as ordinary constructor-pattern fields. That changes the source
   constructor's arity and exposes kernel-internal support topology.
2. ⛔ **Do not coerce an ordinary field reference or an owner self-call into the
   IH.** The former changes the field's source type contextually; the latter
   admits a call that is neither a direct guest-motive instance nor SCT-justified
   recursion.

## The approved semantic shape

**This is the only bounded shape the Architect approves.** It is a component
design, not an implementation plan, and the spelling is deliberately open.

- **Every generated support name and binder stays hidden.** No exposure.
- **Extend the elaborator's association** for a recursive enclosing source field
  with an optional **recursive-result position**, alongside its current evidence
  position and support provenance.
- **Derive that association only from the kernel method telescope.** For each
  `support_shapes[shape_ordinal]`: correlate `shape.position` with the aligned
  support-evidence argument `host_ctor.args.len() + evidence_ordinal`; map that
  evidence ordinal back through `all_support_evidence_positions` to the exact
  source field; the recursive result is the method binder at
  `support_ctor.args.len() + shape_ordinal`. ⛔ **Require a one-to-one, in-range,
  same-support correlation and fail closed** on missing, duplicate, swapped, or
  foreign associations.
- **Add an explicit structural-result surface form** — metanotationally
  `recursive-result xs`. Valid **only** for a surface variable carrying that
  exact recursive-result association; it elaborates directly to the hidden IH
  term and **rejects everywhere else**. ⛔ It is not a function, not a generated
  identifier, and not general recursion.

Then `Join xs ys` combines `recursive-result xs` and `recursive-result ys`. The
kernel eliminator supplies those results recursively at every depth, so the
source contains **no finite unroll and no unrestricted call**. Direct `One x`
leaves keep using the existing exact motive-instance self-call rewrite.

**The kernel and the generated `All` representation need no change.** This is a
surface and elaborator capability.

## Deliverables

- **`D0` — the Spec contract.** The surface spelling; its scoping rules; its
  diagnostics when used outside a lifted recursive field; and its interaction
  with ordinary direct and W-style matches. ⛔ This is the deliverable that must
  land before any implementation frame exists — do not skip it because the
  semantic shape is already ruled.
- **`D1` — the conformance contract.** What `seed-nested.md` must say once the
  capability exists, and the restoration of `nested-size-uses-lift`'s executing
  binding. Route with the conformance-validator, which rejected three candidates
  on exactly this row and holds the fidelity standard it must meet.

⚠ **Implementation is a SUCCESSOR NODE and is deliberately not created yet**
(`steward.md §4c` — a node in front of a held node compounds). The Steward
creates it when `D0` lands and the contract is concrete enough to frame against.
Its owner is Language/elaborator, not this seat.

## Acceptance — the discriminators must prove MECHANISM, not depth

⛔ **Architect, verbatim in force: "depth-three alone is not the closure
argument."** A control that only goes deeper than the last counterexample repeats
the exact failure that produced three rejections.

| AC | criterion |
|---|---|
| `AC-1` | emitted method bodies reference **the exact trailing recursive-result binder** for each correlated residual field |
| `AC-2` | **deleting that association reds an arbitrarily deeper value control** — the association is load-bearing, not incidentally correct at the depths tested |
| `AC-3` | swapped and foreign associations **reject**, and so does use outside a lifted recursive field |
| `AC-4` | generated support identifiers **remain unresolvable** from source |
| `AC-5` | direct and W-style matching stay **byte-for-behaviour unchanged** |

## What this unblocks

**TWO** `KERNEL-NESTED-IND` seed rows, both gated pending this node and both
blocked by the same obstruction (Architect `evt_5dg9ypms4d4ed`, ruling section
below):

- `nested-size-uses-lift` (`seed-nested.md:137`)
- `nested-dependent-motive-uses-lift` (`seed-nested.md:158`)

⛔ Landing this node does **not** by itself close either — the bindings come from
a `D6` successor with fresh QA, Architect, and frontier-class
conformance-validator review. No verdict from the four spent `D6` candidates
transfers to anything.

**Census effect when that successor binds both: `14 → 12`.** ⚠ **Population:
`^### ` heading markers in `conformance/kernel/inductive/seed-nested.md`**, and
**`14` is measured on `main`, not carried forward.** Both rows carry the
`[KERNEL-NESTED-IND]` marker today, and binding a row **removes** its marker —
that is exactly how `D6` went `19 → 14` by binding five. Corpus-wide the same
delta reads `15 → 13`, because `conformance/kernel/judgments/seed-judgments.md`
carries one unchanged marker. Both are right for their own population, so any
criterion citing a count must name which.

> ⚠ **This paragraph said `14 → 13` until the Architect ruling landed**, on the
> assumption that only the size row was gated. That was one row short, not
> wrong in direction. ⇒ **A census prediction is a function of how many rows are
> gated, so it moves whenever the gating does** — re-measure it at the point of
> use rather than treating it as a fixed property of this node.

> ⚠ **An earlier version of this paragraph said the marker is *restored*, census
> `14 → 15`.** That was the withdrawn arithmetic from the `D6` recut, inherited
> here and **wrong in direction as well as magnitude** — nothing is restored, and
> the count goes down when this node's successor lands, not up. Found by the
> Adversary at `evt_2zzy9q33cetm1` after the source was corrected at PR #1752.
> **This node is the one that owns unblocking the row, so it is exactly where a
> wrong prediction would have been used.** Recorded rather than silently fixed
> because the same bad number has now had to be corrected twice, one artifact
> apart: correcting a number at its source does not sweep the artifacts that
> already copied it.

## RULED: a SECOND seed row shares this blocker. This node un-gates BOTH.

**Architect `evt_5dg9ypms4d4ed`, 2026-08-10**, answering the question raised by
Adversary `evt_2zzy9q33cetm1` item 4. **`nested-dependent-motive-uses-lift`
(`seed-nested.md:158`) shares `nested-size-uses-lift`'s exact obstruction.**

The proposed distinction — that lockstep matching might be supplied by
`check_match_with_lift` implicitly and so need no source name — **holds only at
a direct guest leaf, not through the seed's recursive carrier.**

| | flat NC14 control | the seed row |
|---|---|---|
| carrier | `ProofJoin : a -> a -> ProofBag a` | `join : Bag A -> Bag A -> Bag A` |
| child binding | `support: None` | `support: Some(All^Omega_Bag)` |
| consumption | owner self-call takes the exact motive instance | telescope supplies a recursive result **no source term can denote** |

⇒ `nested_dependent_motive_consumes_correlated_child_proofs` is a **direct-leaf
association discriminator, not an executing binding** for this row. Implicit
lockstep preserves the correlation; it does not synthesize the branch's
recursive result. Another source `match` only finite-unrolls the residual
`Bag` — the Nat-size case again.

**The obstruction is sort-independent.** `All^Type` versus `All^Omega` changes
the leaf motive, not the need to consume the support eliminator's recursive
result at `Bag.join`; the topology-carrying `All^Omega` application is itself in
`Type`, as the row says. ⛔ The `D5` erasure note does **not** refute this:
provenance-gated erasure admitting the generated support `Elim` while rejecting
arbitrary dependent motives is a downstream artifact boundary, silent on what
source can name.

⇒ **One capability, two rows. No second surface form is needed** — the same
explicit structural-result form consumes the `Omega`-motive recursive result.

⚠ **Nothing that merged is over-claimed.** The row carries no binding and never
did, so `D6` did not assert an executing binding for it. The disposition is
recorded on the row itself (`seed-nested.md:158`).

## Sequencing and contention

**Not released, and not sequenced against a lane yet.** `depends_on` names
[[KERNEL-NESTED-IND]] because the capability is defined against `D5`'s landed
lifted-elimination surface, which is on `main`.

⚠ **The `depends_on` edge is to the node, and the node is `active` with `AC-K12`
open on a Runtime blocker.** Do not read that as "wait for `KERNEL-NESTED-IND` to
close" — the input this node needs is `D5`'s merged surface, which is already
there. Reading a node-level edge as whole-node is the error that stranded
[[DS-9]] behind a Runtime dependency it did not have.
