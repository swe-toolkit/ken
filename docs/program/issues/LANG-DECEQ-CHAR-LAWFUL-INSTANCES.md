---
id: LANG-DECEQ-CHAR-LAWFUL-INSTANCES
title: "`37 §2.5` defers the proof-carrying `DecEq String` / `Ord String` instances as a `tracked follow-on` because the transport needs a lawful `DecEq Char` that is not landed -- and the follow-on was never filed, so the second unowned obligation in this chapter sits in spec prose with no tracker row"
status: closed
owner: language
size: unsized
gate: none
depends_on: []
blocks: []
github: null
origin: "Steward sweep 2026-08-13 at c1b9a1e8, taken while framing LANG-PRELUDE-ELABORATION-DEPTH. This is the second `tracked follow-on` in spec 37 found with no tracker row -- the first was `filter`, which produced LANG-PRELUDE-COLLECTIONS. Found by grepping the chapter for deferral language rather than by grepping the tracker for gaps."
---

# CLOSED 2026-09-12 — ALREADY SATISFIED (surface predates this node). READ FIRST.

> Architect L2 decomposition ruling evt_127n516pkvtnb (thr_4w935jjdyw5tk),
> re-derived from main c1ef0b004. This node's premise is stale: the entire
> requested product surface was already landed and checked BEFORE this node was
> filed (2026-08-13). It is a stale-corpus duplicate born from stale
> `37 §2.5` wording, not a newly-unblocked implementation obligation.
>
> - `DecEq Char` landed in DS-6a at 23e754e2d (2026-07-10). Current source:
>   `catalog/packages/Core/Classes/LawfulClasses.ken.md:625` (projects all three
>   `DecEq Int` fields; Char = `{ c : Int | isScalar c }`, `eqChar = eq_int`).
> - Lawful `DecEq String` / `Ord String` landed in the CC2 family (2026-07-13),
>   canonically re-homed by 82f5de01 (2026-09-03). Current source:
>   `LawfulClasses.ken.md:2359` (`DecEq String`) and `:2409` (`Ord String`), with
>   real `sound`/`complete` and order-law proofs, transported through the
>   canonical `String -> List Char` view.
>
> OPERATOR RULING (Pat, 2026-09-12, rec A): widen decidable equality, accepting
> +2 irreducible postulates per registrant. STANDS as standing TCB policy for any
> FUTURE opaque-primitive registrant, but requires NO new postulate for Char or
> String. The kernel mechanism `declare_deceq_certificate`
> (`crates/ken-kernel/src/check.rs:1234-1321`) mints exactly two `Decl::Opaque`
> per registrant; the ONLY production call site is `numbers.rs:403` (Int against
> `eq_int`), so the registrant population is one and the certificate delta is
> exactly the two Int postulates. Char is not a second registrant (transparent
> projection, empty local delta); String/Ord transport structurally (zero-NEW-
> delta, inheriting the Int certificate + the pre-existing String retraction
> postulate). The Char-pass/Decimal-reject pair remains the canonicity
> discriminator.
>
> DISPOSITION (Steward): closed as already-satisfied. A spec `37 §2.5` currency
> correction (the deferral promises a follow-on that already landed) is routed to
> the spec enclave. The literals pattern slice is INDEPENDENT of this node
> (spec 34 §…:393-417 makes literal comparison value-level; lawful DecEq optional)
> and is framed separately as the L2 head — [[LANG-MATCH-PATTERN-FORMS-ABSENT]].
> The history below is retained for context.

## What this is

`spec/30-surface/37-strings-collections.md §2.5` (`:205-213`) states the
deferral and its reason:

> *"these are soundly transportable to lawful `DecEq String` / `Ord String`
> **instances** — the canonicity precondition holds here, unlike `Decimal`. But
> that transport additionally needs a lawful **`DecEq Char`**, which is **not
> yet landed** (only the `eqChar` view + `Ord Char`-by-transport are on `main`);
> so the proof-carrying `DecEq String` / `Ord String` instances are a **tracked
> follow-on**, not delivered here. This WP delivers the *functions*; it does
> **not** ship the lawful instances — filing the functions as proof-carrying
> instances would over-claim the trust level."*

**The deferral is correct and well-reasoned. It was never filed.** A grep of
`docs/program/issues/` finds no node for `DecEq Char` or for the lawful
instances.

## Why it is filed as `draft` with `gate: operator`, not as ready work

**It is blocked on a live operator question, not on framing.** Raised
2026-08-12 and unanswered: *is widening decidable equality worth two irreducible
postulates per registrant?* A lawful `DecEq Char` is precisely a registrant on
that mechanism, so this node cannot be scoped — let alone sized — until that
answer exists. Framing deliverables now would produce a frame whose shape the
ruling may invert.

**Filing it anyway is the point.** The chapter has now produced two obligations
recorded in prose and owned by nobody, and the first one (`filter`) survived
undetected long enough that its stated blocking reason had gone false without
anyone noticing. A `draft` node with a written reason is discoverable; a
sentence in a spec section is not.

## The pattern, worth more than this instance

Both misses were found by **reading the chapter for deferral language**, not by
auditing the tracker for gaps. A tracker audit cannot see an obligation that was
never entered into it — it can only compare rows against rows. **The deferral
phrasing is the searchable artifact**: "tracked follow-on", "a separate change",
"deferred", "not delivered here".

⇒ When a chapter is next swept, grep it for that phrasing and check each hit for
a tracker row. This is cheap and it has now paid twice in one chapter.

## Flip condition

Flip to `ready` and frame it when the operator answers the decidable-equality
TCB question. If the answer forecloses lawful instances at this trust level,
close this node as resolved-without-landing and **amend `37 §2.5` so the spec
stops promising a follow-on that will not come** — a stale promise in the spec
is what created this node.
