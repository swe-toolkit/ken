---
id: LANG-SEAL2-GATE-INCRATE-RELOCATION
title: "Move SEAL-2's producer WALK (`enumerate_producer_types`) into ken-elaborator so its completeness claim becomes enforceable for the FIRST time. The walk enforces namespace closure by destructuring `ElabEnv` with no `..`, which an integration test can do only while EVERY field is `pub` -- so the gate was structurally incapable of its own claim from the day it was written, and held only by the accident that no field was private yet. `ElabEnv` now holds a `pub(crate)` field permanently, so the walk moves in-crate or its completeness claim is abandoned. Carries a second, SMALLER deliverable: a twenty-line private non-test destructure in lib.rs that fires at `cargo check`. The two are deliberately in ONE node so closing the cheap one cannot be read as buying the expensive one."
status: ready
owner: language
size: M
gate: none
tier: T2
depends_on: []
blocks: []
github: null
origin: "language-leader, 2026-09-19 (evt_1ba06c3zvjby2, evt_44h9npanevdrh, evt_1se8wycskre80), surfaced during LANG-STANDARD-INFIX-CALL-COMPLETION's A1 build. Filed, WITHDRAWN when the work was folded into A1 under a scope amendment, and RE-FILED when that amendment's blocking premise dissolved. Design fully ruled by the Architect (evt_6b39fyc17xzm1, evt_x1b90s36dtc2). Steward-filed per COORDINATION section 2; constraint interrogated per steward.md section 4c."
---

# THE REFRAME THAT GOVERNS EVERY AC BELOW: THIS BUILDS THE PROPERTY, IT DOES NOT RESTORE IT

**Architect, `evt_1se8wycskre80`.** The gate was **structurally incapable of its
own claim from the day it was written.**

    An integration test can NEVER name a private field.
    So the exhaustive destructure held only by the accident that every
    ElabEnv field happened to be `pub`.
    fa37c6ca6 was the first field to TEST the gate, not the first to WEAKEN it.

⇒ **"Restore the gate" is unsatisfiable — there is nothing to restore.** Any
version in the crate is stronger than anything that ever existed; any version in
`tests/` can never reach full strength no matter how it is repaired.

> ### THE REFRAME IS NOT COSMETIC. IT DECIDES WHICH OPTIONS PASS.
>
> An AC reading *"restore the gate"* is **satisfied** by the cheap D1 below —
> something is gating again, from a stronger location than before. The AC in
> this node is **not**. Same options, same prices; the framing is what separates
> them, which is why it is the first section rather than a note.

# D2 (the node's actual purpose): move the WALK in-crate

**`enumerate_producer_types` moves into `ken-elaborator` and names every
`ElabEnv` field with no `..`, in a module where private fields are nameable.**

    AC-D2  A new ElabEnv field cannot reach the producer WALK unclassified.
           NOT "the gate breaks the build somewhere."
           NOT satisfiable by D1's existence. See the fence below.

**Measured at `origin/main` `7fadb164c`:**

    seal2_support/mod.rs              1198 lines, TWO consumers
                                      (adversary_seal2_repros,
                                       seal2_producer_closure)
    enumerate_producer_types          69 lines, :104-172 -- about 6%
    the WALK's consumers              seal2_producer_closure ONLY, at :181
                                      and :215, plus internal mod.rs:347

> **The module has two consumers; the WALK has one.** An earlier Steward
> revision said the gate's move required rewiring both. It does not, and the
> Steward's own `git grep` output already showed no hit in
> `adversary_seal2_repros` — which imports `closed_producers`,
> `conservative_deep_producers`, `head_only_producers` and not the walk. **A
> count carried onto a neighbouring subject**, corrected by the Architect at
> `evt_1se8wycskre80`.

**The 69-line figure supports the sizing rather than undercutting it: the walk
cannot travel alone, because its caller cannot follow it in-crate.** Sizing is
`M` for that reason, not for the line count.

## Why there is no cheaper route that keeps the claim

The Architect worked the obvious ones and they all close (`evt_x1b90s36dtc2`):

    export the field list for an outside walk to assert against
        needs a non-test `pub` item -- widening the production surface
        for a test, already ruled out
    #[doc(hidden)] pub
        the same thing wearing a hat
    classification in-crate, walk stays outside
        the two drift; the original defect with extra steps

⇒ **A walk's COMPLETENESS is not enforceable from outside the crate once
`ElabEnv` holds any non-`pub` field — and it now does, permanently.** The walk
moves in-crate or the completeness claim is abandoned. There is no third option.

# D1 (smaller, real, and NOT a substitute): a `cargo check`-time destructure

**A private non-test `fn` in `lib.rs`, `#[allow(dead_code)]`, exhaustively
destructuring `ElabEnv`.** About twenty lines; nothing moves.

**Not `#[cfg(test)]`** (Architect's amendment to the Steward's shape): as a
plain private fn it fires at `cargo check` — the cheapest and most frequent
command anyone runs — instead of only in test builds. One attribute is the whole
cost.

**What it buys, and it is a capability the tree has never had:** adding ANY
field to `ElabEnv` — `pub`, `pub(crate)`, or private — becomes a build break.

> ### WHY D1 CANNOT REPLACE D2, WORKED THROUGH RATHER THAN ASSERTED
>
> **The Steward proposed D1 as a possible substitute. The Architect refuted it,
> and the refutation is why both live in this node.**
>
> Today the forcing comes from the trigger and the walk being **ONE PATTERN**:
> you cannot add a field without deciding, in that same list, whether it is a
> producer source or a justified non-source. **Split them, and the author
> satisfies the compiler by typing one identifier into a gate that does nothing,
> while `enumerate_producer_types` silently stays incomplete.**
>
> **Follow the consequence, because it is not a weaker check — it is the exact
> failure the gate is named after:**
>
>     incomplete walk  ->  SMALLER producer set
>     smaller set      ->  EASIER to close
>     seal2_producer_closure computes its closure over a population missing
>     the new namespace and PASSES -- greener than before, for precisely the
>     reason that should have reddened it.
>
> ⇒ **The split converts "you cannot add a field without CLASSIFYING it" into
> "you cannot add a field without ACKNOWLEDGING it," and the acknowledgement is
> satisfiable by typing a name.**

## The two requirements that come with D1, and the structural one

**Architect's, and they are cheap:**

1. **D1's doc states what it does NOT establish** — that the seal2 walk is
   complete — and **names `enumerate_producer_types` as the obligation it does
   not discharge.**
2. **AC-D2 is not satisfiable by D1's existence.** It is satisfied by the walk
   naming every field with no `..`, in a module where private fields are
   nameable.

**Steward's, and it is why these are ONE node rather than two:** a green compile
gate named after the property **makes this node look done.** Someone adds a
field, hits D1, satisfies it, sees `seal2_producer_closure` green, and concludes
the closure holds. **The cheap thing standing in for the expensive thing is how
the expensive thing never gets funded** — and here it would also be *reported as
covered*.

> **Two nodes would let the cheap one CLOSE, and a closed node reads as the
> property bought.** One node with an AC only D2 can satisfy makes that
> impossible to record. **Do not split this node to land D1 sooner.** D1 may be
> released ahead of D2 inside this node if the Steward chooses; it may not
> close it.

# Current state: the interim is CORRECT, not merely tolerable

At `722a48cec` the walk carries `..` with the retraction recorded at the head of
the doc comment. **Nothing is silently wrong and nothing is blocked.** The
interim's notice must **govern** the whole comment rather than precede it — the
body carries three further clauses (*"with no `..`"*, *"can never be a silent
pass"*, *"naming every field with no `..` is the entire point"*) that are each
false at that head.

**The relocation retires all of it:** once the walk is in-crate the doc comment's
claim is true again and the notice comes back out with the `..`.

# The constraint, interrogated

**Grounded.** `docs/PRINCIPLES.md` §8 — prefer loud refusal over silent
degradation. A closure oracle computing over a population that silently omits a
namespace is silent degradation that reports as a pass. **Not grounded, and must
not be written in:** this is not a TCB argument and not a safety-of-`main`
argument. `main` is clean and nothing is red.

# Why this is `draft`, and what it is NOT waiting for

**QUEUED by priority.** L1 (clearing the ignored tests) is the operator's top
priority as of 2026-09-17; the language lane's objective is
`LANG-MODULE-IMPORT-SYSTEM`. Not released until the Steward releases it.

**It is NOT waiting on A1, and A1 is not waiting on it.** This node was folded
into `[[LANG-STANDARD-INFIX-CALL-COMPLETION]]` and then un-folded when that
amendment's blocking premise — *"AC-8 cannot be discharged without it"* —
**dissolved rather than being satisfied** at `6ab1639f1`. A deferral argument's
precondition is a state, not a label; nobody discharged it and nothing announced
that it had stopped being true.

# Contention

`[[LANG-R-LAYER-EXPORT-RETRACTION]]` also relocates integration tests in-crate
in this same crate on the same Architect ruling. **Still not folded, but the
reason has CHANGED and the old one should not be re-cited:** it was *"the
schedules differ"* while A1 was about to force this one. Both are queued now and
neither decays. What separates them today is **subject and contention** — that
node retracts a visibility surface and contends on `classes.rs` with
`[[LANG-INSTANCE-REGISTRY-IDENTITY-KEY]]`; this one has no contention. Whichever
runs second re-grounds its coordinates.
