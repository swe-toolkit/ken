---
id: RT-CONSTRUCTOR-AUTHORITY-DISCHARGE
title: "Widen GeneratedConstructorAuthority from generated-context Result publication to every hand-off carrying a constructor word, and make the discharge the authority's CONSUMPTION at the consumer -- so a hand-off with no authority is a missing VALUE and a planner error, never a missing list entry and never a silent pass"
status: draft
owner: runtime
size: L
gate: none
depends_on: []
blocks: []
github: null
tier: T1
origin: "Architect rulings evt_5kzahxdv9w8d1 (the three parts, the forbidden alternative) and evt_76sfwdh03d69f (the two-halved bounded measurement, and the correction of the Steward's 'no object exists' claim). Recut of RT-CHECKED-IH-RESULT-OBLIGATION-REKEY's D1, which could not be written while the representation fork was held. Fixed inputs measured by the Steward on 5d977ac79 and origin/main; re-measure at D0."
---

> # DRAFT — pending Architect read. NOT released to the runtime ring.
>
> The Architect's closing instruction: *"Release nothing until the frame reaches
> me; I will read it before the ring."*
>
> **THIS FRAME DOES NOT CUT THE THREE PARTS THE WAY THE RULING DID, AND THE
> DIVERGENCE IS DELIBERATE.** The ruling described Part 1 — dropping
> `Clone, Copy` from `GeneratedConstructorAuthority` — as *"one line, failures
> are compile errors, independently landable today, and it must land first."*
> **Measurement contradicts all three clauses.** The evidence is in "The
> separability finding" below, and it is the first thing to read. If the
> Architect reads it and still wants the split, that is their call and the
> Steward will cut it — but it should be made against the measurement rather
> than against the Steward's description of it.

## What this is

`RT-CHECKED-IH-RESULT-OBLIGATION-REKEY`'s `D1` says the `(actual, demanded)`
pair must be discharged TOTALLY. Totality is a quantifier, a quantifier needs a
population, and the population was the held representation fork — so `D1` was
unwritable without deciding the fork. That is now resolved, and not by picking
an arm.

**The ruled shape:** the identity travels beside the word in a genuinely
move-only authority, and the discharge is that authority's **consumption** at
the consumer. **The carrier stays empty.**

**Why this is not the forbidden enumeration.** An authority that must be
PRODUCED to hand off and CONSUMED to receive makes the missing case a **missing
VALUE, not a missing list entry**. The failure is local — "you have no authority
to hand off here" — rather than a census that a new site can silently fall
outside of. This is the proven-closure form surviving contact with an emitter
that has no single producer gate.

## The base, and it is not `origin/main`

**`GeneratedConstructorAuthority` DOES NOT EXIST ON `origin/main`.** Measured:
`git grep -c GeneratedConstructorAuthority origin/main -- crates/` returns zero
hits. It exists only on the runtime ring's **held, unreleased** branch
`5d977ac79` (6 commits ahead of `origin/main`), introduced across `01d2ccb11`
and `5d977ac79` — "ABI-S6 HS18: certify generated Result path cuts" / "audit
generated Result protocols".

**Three consequences, and none of them is a blocker:**

1. **This node's base is `5d977ac79`, not `origin/main`**, and every coordinate
   below is measured there unless it says otherwise.
2. **Nothing here is "landable today" against `origin/main`.** The held branch
   must land first, or this work is authored on top of it and lands with it.
   **Which of those two, and when, is a sequencing call and it is the
   Steward's** — it is taken in "Sequencing" below rather than left to the ring.
3. The ruling's premise that Part 1 could go in ahead of everything came from
   prose describing the tree, not from the tree. Recorded here because this arc
   has now produced that shape more than once, on more than one seat, and the
   cheap correction is to name the base in the frame.

## The separability finding: Part 1 is not separable from Part 3

**The authority is a REGISTRY, not an authority, and removing `Clone, Copy` does
not change that.** Measured at `5d977ac79`:

- **Declared** `mod.rs:3852` with `#[derive(Clone, Copy)]`, directly under a doc
  comment reading *"Move-only compiler authority for publishing one
  generated-context Result."*
- **Stored** in `function_local.generated_constructor_authorities`, typed
  `BTreeMap<ir::Value, GeneratedConstructorAuthority>` (`mod.rs:1319`), written
  by `register_generated_constructor_authority` (`mod.rs:4053`).
- **Produced** at exactly two sites: `calls.rs:1866`, `core.rs:13994`.
- **Read at SIX sites, all in `units.rs`, and every one of them is a shared
  borrow over the map as a COLLECTION** — not one of them consumes anything:

  | site | shape | what it does |
  |---|---|---|
  | `3122-3124` | `.get(&value).is_some_and(..)` | the discharge check — **and `None` falls through** to `call_seeds` and then to a general forwarding search |
  | `3262` | `.values().any(..)` | `cfg(px8-ds-test-support)` mutation trigger |
  | `3307` | `.values().find(\|a\| a.identity != identity)` | `cfg(px8-ds-test-support)` — deliberately selects a **foreign** authority's word to substitute |
  | `3343` | `.iter().filter(..).map(..)` | grounds verification over **every** matching authority, feeding `verify_constructor_authority_ground` |
  | `4312` | `.get(&publication.returned_word)` | diagnostic |
  | `4314-4318` | `.len()`, `.values().filter(..).take(8)` | diagnostic |

- **Consumed** at `units.rs:5415`, `verify_constructor_authority_ground(body,
  authority: &GeneratedConstructorAuthority, helpers)` — by shared reference.

**The documented property is false in four independent ways**, and the derive is
only the most visible one:

| way | what makes it false |
|---|---|
| `Copy` | a `Copy` value can be duplicated and dropped, so nothing can require its consumption |
| stored in a `BTreeMap` | the entry survives every read; there is no move to observe |
| read via `get(..).is_some_and(..)` | a shared borrow, and **`None` falls through to the next check rather than erroring** |
| queried as an aggregate | `values()`, `iter()`, `len()`, `find()` — the usage pattern is collection query, which presupposes the entries persist |

**⇒ Dropping `Clone, Copy` produces NO compile error at ANY of the six read
sites.** `BTreeMap::get` yields `Option<&T>`; `values()`, `iter()` and `len()`
are borrows; `is_some_and` over `&T` compiles for a non-`Copy` `T`; the
`&`-taking function at `5415` is unchanged; and the producer already moves the
value into `insert`. **Landed exactly as specified, Part 1 is a green node that
leaves the property it is named for still false.**

**And the restructure is larger than the derive suggests in the other
direction.** Two of the six reads are `cfg(px8-ds-test-support)` mutation
harnesses, and the one at `3307` works by finding a **foreign** authority still
live in the map — it depends on multiple authorities coexisting. Consumption
semantics changes that harness too, and `D0` must count it rather than discover
it mid-build.

That is this arc's own predicate arriving inside a proposed repair for it: a
mechanism asserted in a comment, and a fix that also does not enforce it.

**What would actually make it move-only is `remove()` at the consumer plus a
hard error on `None` — which IS Part 3.** Part 1 and Part 3 are the same change
seen from two ends, so Part 1 cannot stand in front of Part 3 as an independent
increment. They are `D2` here, together.

**This analysis is not a build, and `D0` tests it rather than trusting it.** The
Steward did not compile it: that needs the held branch checked out, and
`ken-cargo` takes the machine-wide build mutex for the whole invocation. The one
command that settles it is the first deliverable.

## Fixed inputs

**MEASURED BY THE STEWARD at `5d977ac79`** unless a line says otherwise.
Re-measure every coordinate at `D0` — the `686ffa8ac` measurements of these same
facts were 2 to 280 lines off, which is why this list is dated and not trusted.

- **The carrier, and the invariant that forbids the obvious fix.**
  `lowering/mod.rs:3843`, one field `word: ir::Value`, under a declaration that
  reads verbatim:

      ⛔ It holds the word and NOTHING ELSE, and the emptiness is the point.
      ... Every question about this value -- which constructor, how many
      fields, which child -- is answered by calling an emitted helper at
      runtime, never by reading a field of this struct. ⇒ The struct having
      room for a compile-time answer is exactly how the wall would grow back.

- **Carrier construction is unsealed**: built by struct literal at **35 sites
  across 8 files**, no factory (aggregates 6, units 7, core 6, mod 5, joins 4,
  effects 4, calls 3, source 1). A 36th site is an ordinary edit and produces no
  compile error. **This is why the discharge cannot live at carrier
  construction**, independently of the invariant above.

- **The failing shape is a raw block edge**: `builder.ins().jump(block, &[word])`
  at **86 sites**, an open Cranelift API with no interposition point between
  deciding to jump and jumping.

- **The consumer's demanded identity is read at three non-definition sites**:
  `core.rs:9210`, `core.rs:9220`, `responses.rs:2247` (definitions at
  `responses.rs:358`, `:373`, `:1975`). **The failing `D6a` intra-function edge
  is none of them.** This is the same disjointness the REKEY node records from
  the other end, and it is the reason the consumer half of the measurement below
  is not free.

- **`LoweringOperand`** (`mod.rs`, sealed and wildcard-free, its doc refusing a
  fallback arm as *"a wildcard with better manners"*) is the near-miss to not
  confuse with the target: it discriminates `Specialized` vs `Carried`, i.e.
  whether a value is a runtime word **at all**. Right shape, wrong population.

## Deliverables

**`D0` — settle the separability question by running it, not by arguing it.**
Remove `#[derive(Clone, Copy)]` from `GeneratedConstructorAuthority`, build
`ken-runtime` alone (`scripts/ken-cargo ... -p ken-runtime`, never
`--workspace`), and **report the exact set of resulting compile errors.**

- **Zero or near-zero errors CONFIRMS the finding above**: the derive is not
  load-bearing, and the move-only property has to come from the consumer side.
  Proceed with `D1`/`D2` as one change.
- **A broad error set REFUTES it**, which is a real outcome and not a failure —
  report it and come back, because the cut then splits the way the ruling
  originally described.

**Report the number and the sites either way.** This is a measurement whose
purpose is to be able to come out the other way.

**`D0` also counts the restructure surface**, which the derive probe does not
reveal: the six read sites above, including the two `cfg(px8-ds-test-support)`
mutation harnesses and specifically the one at `units.rs:3307` that depends on
a foreign authority remaining live in the map. **How that harness survives
consumption semantics is a `D0` answer, not a `D2` surprise.**

**`D1` — the two-halved bounded measurement. BOTH HALVES ARE REPORTS, AND
NEITHER MAY BE SATISFIED BY INVENTION.**

  - **`D1a` PRODUCER SIDE.** At each hand-off that carries a constructor word,
    **is the word's ACTUAL identity in scope**, so an authority can be produced?
    At the two existing production sites it comes from the context plan.
  - **`D1b` CONSUMER SIDE.** At each consumer, **is the DEMANDED identity
    available**, so the authority can be discharged there? **This half is not
    free** — the demand is read at three sites and the failing `D6a` edge is
    none of them.

**If either is unavailable in principle at some site, THAT IS THE FINDING.**
Architect, verbatim: *"Do not synthesize an identity or a demand to satisfy the
type. I would rather learn that the demand cannot reach the D6a edge than have
it manufactured there."* A site that cannot name its identity or its demand is
reported to the Steward and the Architect; it is not defaulted, not inferred
from traversal order, and not filled with the nearest plausible value. **A
synthesized identity is indistinguishable from a real one at the type level and
converts an unsound pass into an unsound pass with a proof.**

**`D2` — widen the population and make the discharge a consumption.** One
change, per the separability finding:

  - **Widen** the authority from generated-context `Result` publication (2
    production sites today) to **every hand-off that carries a constructor
    word, including the `D6a` block edge that failed.**
  - **Discharge by consuming the authority at the consumer**, against the
    consumer's demanded identity:
      - identities **agree** ⇒ the word passes;
      - identities **differ** ⇒ the derived chain applies;
      - **neither** ⇒ **planner error. Never a silent pass.**
  - **Consumption must be a move, and the absence of an authority must be an
    error rather than a fallthrough.** The current
    `get(..).is_some_and(..)`-then-continue shape is the silent pass in
    structural form.

**`D3` — report what the widening cost**: how many hand-off sites acquired an
authority, how many consumers acquired a discharge, and every site reported
under `D1`.

## Acceptance criteria

**`AC-1` — the missing case is a MISSING VALUE.** A hand-off that carries a
constructor word without producing an authority does not compile, or fails at
plan time with a located error. **The control: it must be possible to write that
hand-off.** An AC discharged by "we checked every site" is the enumeration form
this node exists to avoid — the criterion is about what happens to a site nobody
checked.

**`AC-2` — the `D6a` edge discharges, or its failure is reported.** The
intra-function block edge that failed in HS18 either carries an authority
consumed against a real demanded identity, or `D1b` reports that the demand
cannot reach it. **Both outcomes satisfy this AC and only silence fails it.**

**`AC-3` — the neither-case is an error, and the test proves it.** A hand-off
whose actual identity neither agrees with the consumer's demand nor resolves
through the derived chain produces a planner error. **The control is a negative
one: the same test must show the pre-repair path passing it silently**, or the
AC cannot distinguish the repair from the defect.

**`AC-4` — the carrier is unchanged.** `CarriedBoundaryWord` still holds exactly
one field. A diff that adds a field to it fails this node regardless of what
else it achieves. See "Cut items".

**Not an acceptance criterion, deliberately:** "no hand-off lacks an authority."
That is the enumeration the ruling forbids, wearing an AC's clothes — it is
checkable only by census, and a census is blind to exactly the site that was
never added to it.

## Cut items — these are decided, and they are not the ring's to reopen

**SEALING `CarriedBoundaryWord` BEHIND A CONSTRUCTOR IS FORBIDDEN.** It is the
obvious move from the 35-site census and it is ruled out. A discharge at carrier
construction needs the identity at that point, which is a compile-time answer
about the value, which is the wall `RT-FNSPLIT-C1` `D3` removed. **It is a
soundness regression in the shape of a cheap win.** The carrier stays empty;
`AC-4` is what makes that failable rather than hortatory.

**The gate is not missing — it is NARROW.** `GeneratedConstructorAuthority`
already pairs `identity` beside `word` and is explicitly not added to the
carrier, which is how it coexists with the emptiness invariant instead of
fighting it. The act is **widen an existing narrow thing and make its central
property true**, not create a new mechanism.

**Do not fall back to enumerating hand-off sites** if the widening proves hard.
Report the difficulty instead.

## Sequencing

**The held branch goes first, and this node is authored on top of it.**
`5d977ac79` is 6 commits ahead of `origin/main` and carries the type this node
widens. The Steward's call, taken here rather than left open:

1. The runtime ring's held HS18 work routes on its own merits — the
   representation fork that held it is now ruled, so the reason for the hold is
   spent. It is not held on this node.
2. This node is then cut against whatever SHA that lands as, and `D0` re-measures
   every coordinate there.

**If the ring would rather fold `D0`'s one-line change into the held branch
before it routes, bring that to the Steward** — it is cheaper than a follow-on
node and it is a sequencing question, which is the Steward's to decide. Do not
take it unilaterally: the held branch is under review at an exact SHA and
amending it silently invalidates that review.

## Why this is FENCED, and the one place that assessment is thin

**Compiler-internal.** It grows no TCB, adds no capability reachable from Ken
source, relocates no emission ownership, and changes no ABI, schema, or
serialized artifact. It is confined to `ken-runtime`'s Cranelift backend.

**The thin part, stated rather than waved through:** `D2` converts a silent
fallthrough into a planner error, which is a **refusal-behaviour change**. This
fleet's standing lesson is that refusal-behaviour changes gate on whole consumer
crates, because a program that compiled yesterday can stop compiling today. The
assessment that this is still FENCED rests on the new error being reachable only
where the old path would have emitted a word the consumer did not demand — i.e.
only on inputs that were already unsound. **If `D0` or `D1` finds a reachable
case where the new error fires on a well-formed program, the FENCED assessment
does not hold and this comes back to the Steward before the build continues.**

## Sizing / tier

**Size L, tier T1.** The diff is not large but almost none of it is mechanical:
the population question, the two-halved measurement, and the neither-case are
each an argument, and the review turns on whether the discharge is genuinely a
consumption rather than a lookup that resembles one. Architect required on the
mechanism and on any `D1` finding.

## Contention

Runtime ring, `crates/ken-runtime` only. **No cross-lane contention** — L2
language is in `ken-elaborator`, L3 foundation is in `catalog/`. Note that
`RT-TRAP-IDENTITY-EMISSION-COORDINATE` **does** touch `ken-elaborator` and
therefore does contend with L2; this node does not, and the two are independent.

The candidate is a CODE merge: full CI, M8/M8a Adversary.

## Not this node

- The trap identity work — `RT-TRAP-IDENTITY-EMISSION-COORDINATE`. Disjoint
  code and disjoint obligation.
- `RT-CHECKED-IH-RESULT-OBLIGATION-REKEY`'s `D2` through `D6`, which are
  retained and not reopened. This node is the recut of its `D1` alone.
- Any change to what the runtime tag means, or to the carrier's contents.
