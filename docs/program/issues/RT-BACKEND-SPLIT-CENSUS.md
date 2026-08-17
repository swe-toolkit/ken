---
id: RT-BACKEND-SPLIT-CENSUS
title: "Stage A of the backend module split — five inventories over the post-retirement tree, before any code moves"
status: merged
owner: runtime
size: M
gate: none
depends_on: [RT-DESCENT-RETIRE, RT-CENSUS-CAVEAT-GUARD, RT-CALL-EDGE-EXECUTABILITY-AXIS, RT-SRCMACHINE-DISPATCH-REACHABILITY-CONTROL]
blocks: [RT-BACKEND-PRIMITIVE-LOWERING-SPLIT]
github: null
origin: Architect ruling evt_54zvaqbrm752x (2026-08-10) decomposing RT-BACKEND-MODULE-SPLIT into independently mergeable slices, cut item 1. Enclave pass anchored at evt_104nz8cedzyat on operator instruction 2026-08-10. Stage A is research/compiler-refactoring-program.md §5.1. Steward-filed per COORDINATION §2.
---

> # MERGED 2026-08-17. `D0`-`D6` at squash `8ebc2467d`, PR #2541. STAGE A IS DONE.
>
> **Reviewed candidate exact `bdf426ce3a48ac90e5fcbdb5172e632b8ea6eeaa`, range
> `4de48651434dd6340f81ec9b1b7a5ac2ec8c0199...bdf426ce3a48ac90e5fcbdb5172e632b8ea6eeaa`.**
> One commit, six `docs/program/` paths, `+1454/-0`, **zero changed paths under
> `crates/`** — `AC-5`. QA at `evt_54y0ak9ezsr25`; Architect resolved
> `dec_5xxct147r6ycr`, domain `docs/program/`, one non-blocking should-fix, no
> respin.
>
> **The five inventories are the durable deliverable**, all at one pinned
> measurement SHA which occurs exactly once in each document (`AC-1`):
>
> | inventory | headline |
> |---|---|
> | [type ownership](../backend-split-census-type-ownership.md) | 278 non-private type declarations, 24 / 8 / 199 / 47 by visibility |
> | [lifecycles](../backend-split-census-lifecycles.md) | authority and ledger mint / transition / close / terminal sites |
> | [re-exports](../backend-split-census-reexports.md) | 57 statements, 29 / 4 / 2 / 22 by build profile |
> | [tests](../backend-split-census-tests.md) | 716 `#[test]` functions, 127 mutation surfaces, 70 fixtures |
> | [co-change](../backend-split-census-cochange.md) | 156 distinct commits; 64 / 61 / 79 / 107 per file |
>
> ## `D6` RETURNED THE POSITIVE VERDICT, NOT THE STOP
>
> **The next slice's ownership proof survived revalidation.**
> `lower_primitive_call` has one definition and one caller; every call site of
> the twelve selected helpers lies inside that dispatcher (the three with extra
> hits are multi-arm dispatch, not foreign callers); `lowered_char_list` is
> definition plus one self-recursive call plus the dispatcher; `expect_two_args`
> has five calls, all from selected methods — **an acyclic shared arity seam,
> not a second lowering owner.** No new shared owner, no cycle.
>
> ⇒ **[[RT-BACKEND-PRIMITIVE-LOWERING-SPLIT]] is `ready`.** Its `draft` bar was
> exactly this verdict.
>
> ## `AC-2` AND `AC-4` ARE WHY THIS NODE WAS WORTH ITS SLOT
>
> **Every count carries its selector and states what the selector cannot see.**
> The type-ownership record names its regex and lists its blind spots outright:
> private types, macro-generated declarations, declarations whose visibility and
> type keyword are split across lines, traits, constants, functions, fields.
>
> **`AC-4` was amended the same morning, and the amendment changed the
> artifact.** It had required recording the caveat guard *"as FIXED"*. Inventory
> 4 now records it as **fixed but partial** — 322 bare `#[cfg(test)]` regions
> against a rationale domain of **340**, because 18 `any(test, …)` regions
> remain outside the guard. **A bare 322 would have frozen a false completeness
> onto seventeen later slices**, which is the failure the criterion exists to
> prevent, one level down.
>
> **The Architect reproduced every headline count from each record's own
> declared selector** rather than from QA's reproduction, and all matched.
>
> ## THE SHOULD-FIX IS APPLIED, AND IT NEEDED TWO SITES RATHER THAN ONE
>
> **Architect note, in the resolution:** `backend-split-census.md` item 1 read
> *"records 278 non-private declarations"*, dropping the word **type** that the
> measured domain requires. The same domain also holds 694 `pub fn`, 25
> `pub const`, 7 `pub static`, 5 `pub mod`. **The error direction understates
> the non-private surface, which for a split census makes a move look cheaper
> than it is.**
>
> **The note called this an index-sentence fix on the ground that the
> sub-record is exact. I swept for the phrase and it occurs twice.**
> `backend-split-census-type-ownership.md:20` also read *"It selects 278
> non-private declarations"*. **The note's judgment about where the harm lives
> is right** — that sentence sits between the selector, which literally shows
> `(struct|enum|type)`, and an explicit exclusion list naming traits,
> constants, functions and fields, so context defuses it there and does not in
> the index. **But a remedy scoped to the index leaves the same phrase in the
> tree**, available to the next reader who lifts the sentence rather than the
> paragraph. Both are corrected, and the index now carries the excluded
> categories and the direction.

> # SUPERSEDED: the release banner below. Kept — it records the bars coming down.
>
> **Every bar is down**, each verified at its landed squash rather than by
> ancestry:
>
> | dependency | landed |
> |---|---|
> | [[RT-DESCENT-RETIRE]] | route deleted at `1aec3e3e1`, closeout PR #2527 |
> | [[RT-CENSUS-CAVEAT-GUARD]] | squash `be25ea6a2`, PR #2531 |
> | [[RT-CALL-EDGE-EXECUTABILITY-AXIS]] | squash `e5286ea06`, PR #2533 |
> | [[RT-SRCMACHINE-DISPATCH-REACHABILITY-CONTROL]] | squash `6ed648762`, PR #2536 |
>
> **The tree this node measures now exists**, and the three clean-ups that
> sharpened the instruments it will use are in it. Deliverables and acceptance
> criteria below were written 2026-08-17 and are shovel-ready.
>
> **Per the operator's run order (2026-08-17), this is step 2 of 3**: the three
> clean-ups, then this census, then the [[RT-BACKEND-MODULE-SPLIT]] nodes.
>
> **It stays `draft` for a different and still-live reason: three of its four
> `depends_on` are unmerged**, and all three edit files inside this node's own
> scope. Flipping it `ready` now would put a node on the frontier that cannot
> lawfully start. It flips `ready` when the last of the three lands, and at
> that point it is shovel-ready with no Steward pass in between.
>
> **This node DOES reach `merged`.** It moves no code, but it commits five
> inventories as documents, so it is not a measurement-only node whose
> successors would gate on a landing that never happens.
>
> This node **is** the post-retirement remeasure. It is the reason every other
> #8 child stays `draft` too: **the census supplies their binding paths, counts
> and sizes.**

> # OPERATOR RULING 2026-08-16 — THREE CONTROL FIXES LAND BEFORE THIS CENSUS
>
> **`depends_on` now names three campaign spinouts**, and the edge is on
> `depends_on` rather than their `blocks` because `scripts/gen-progress.sh`
> reads only `depends_on`; a `blocks` edge alone is invisible to every generated
> view.
>
> | node | size | region |
> |---|---|---|
> | [[RT-CENSUS-CAVEAT-GUARD]] | S | `lowering/core/tests/control.rs` |
> | [[RT-CALL-EDGE-EXECUTABILITY-AXIS]] | S | `planning/static_transition.rs` |
> | [[RT-SRCMACHINE-DISPATCH-REACHABILITY-CONTROL]] | S | `control.rs`, `core.rs`, `mod.rs` |
>
> **The discriminator was file contention, not tidiness.** All three are
> semantic edits inside this node's own scope
> (`crates/ken-runtime/src/cranelift_backend/` plus `boundary_value_clif.rs`),
> and a split cannot run concurrently with semantic work on the files it
> partitions — campaign §4 ground 3. Landing them first means one rebase rather
> than a re-home followed by a fix.
>
> **`RT-CENSUS-CAVEAT-GUARD` has the stronger reason, and it is specific to this
> node.** Inventory 4 is a **test-property ledger that becomes binding on all
> seventeen later slices.** That node's defect is a staleness guard which cannot
> detect the drift it was written to catch. **Census it as-is and the ledger
> records a broken guard as the expected property**, after which every slice is
> checked against a wrong expected value. A faithful census can freeze a design
> defect; that is what this edge prevents.
>
> **Two other campaign spinouts were considered and deliberately NOT sequenced
> here** — [[RT-GROUNDVALUE-RECURSIVE-DROP]] and
> [[RT-FRONTEND-REACHABILITY-TRIPWIRE]]. Neither lives in this scope: the first
> is `RuntimeGroundValue` decode/drop, the second is a grammar and elaborator
> instrument by construction. **Gating the phase on them would hold
> [[RT-BACKEND-PRIMITIVE-LOWERING-SPLIT]] and the nineteen transitive ABI
> dependents behind it for work that does not contend.** Do not add them.
>
> **Sequencing within lane 1**, Steward's call under the ruling: the capstone
> first (it makes the largest change to these same files), then the three fixes,
> then this census. Putting them ahead of the capstone would have the deletion
> re-open work already done.

## What it is

The first work package of campaign node #8, and the only one that moves no
code. It produces five inventories over the post-`RT-DESCENT-RETIRE` tree:

1. **Type ownership** — every public-in-backend type, its minting module, and
   its consumers.
2. **Lifecycle, evidence and closeout** — each authority and the exact
   lifecycle it governs.
3. **Re-export surface** — every existing path and visibility class, in **both**
   the library and test builds.
4. **Test property** — each test, fixture, mutation, counter, denominator and
   source oracle, with its property class and production injection point.
5. **Co-change baseline** — the post-retirement version of the four-file churn
   matrix.

**The census is the refactor's plan.** A directory sketch without these
inventories is insufficient, because it cannot show which semantic edges a move
must preserve.

## Why it is a node and not a preamble

Every later slice's acceptance rests on a ledger this produces — the exact
old/new symbol ledger and the test-property ledger are binding on every
structural frame (Architect `evt_54zvaqbrm752x` §3). A slice that carries its
own census would be asserting the map it is being checked against.

It also holds one **fail-closed verification gate**: it revalidates the
primitive-lowering call graph that [[RT-BACKEND-PRIMITIVE-LOWERING-SPLIT]] was
chosen on. If #7 created a new shared owner or a cycle there, this node **stops
with the exact contradiction** rather than widening the slice.

## Deliverables

**`D0` — fix the measurement SHA, first, and put it in the document.** Every
inventory is measured at one named commit. Five inventories taken at five
different bases are not a census; they are five readings that cannot be
cross-checked, and a later slice checked against them cannot tell which one
moved.

**`D1`-`D5` — the five inventories**, one section each, committed under
`docs/program/` as durable documents. Not a thread, not a handback: a later
slice's acceptance is checked against these, so they must be readable by
someone who was not in the ring.

**`D6` — the fail-closed verification gate.** Revalidate the primitive-lowering
call graph that [[RT-BACKEND-PRIMITIVE-LOWERING-SPLIT]] was chosen on. **If a
new shared owner or a cycle appeared, stop and report the exact contradiction.
Do not widen the slice to accommodate it** — that is a re-cut and it is the
Steward's, not this node's.

## Acceptance criteria

- **`AC-1` — one base.** Every inventory names the same measurement SHA, and
  the document states it once at the top. A reader can re-run any figure.

- **`AC-2` — every lexical count declares its domain.** This is the lesson
  [[RT-DESCENT-RETIRE]] paid for and it lands hardest on a census node.
  **A grep-entered population is complete over its selector, never over the
  mechanism** — a symbol phrased outside the selector never enters the count
  and is never classified. Wherever an inventory figure came from a pattern,
  give the pattern and say what it cannot see. **A zero-hit result is evidence
  about a name, not about a mechanism.**

- **`AC-3` — inventory 3 covers BOTH builds.** The re-export surface is
  enumerated for the library build **and** the test build. A path visible only
  under `cfg(test)` is a real edge a move can break, and reading one profile
  and reporting one number is how it goes missing.

- **`AC-4` — inventory 4 records the caveat guard as FIXED, not as found —
  and records that its fix is PARTIAL.** It is a test-property ledger binding
  on all seventeen later slices, and [[RT-CENSUS-CAVEAT-GUARD]] is sequenced
  ahead of this node precisely so the ledger does not freeze a broken guard as
  the expected property. It has landed; census the post-fix state.

  > **"Fixed" here does not mean "correct", and writing 322 down unqualified
  > would repeat this criterion's own failure one level down.** The landed
  > guard pins `#[cfg(test)]` — 322 regions, exact for the sentence's selector
  > clause. **Its rationale clause covers 340**: 18 further regions under
  > `any(test, …)` gate code active under `cargo test` and are uncounted, and
  > the guard cannot see more arrive. That residual is
  > [[RT-CAVEAT-GUARD-SPELLING-DOMAIN]], filed, `ready`, and **not a blocker
  > for this node** — `AC-2` already obliges every lexical count here to name
  > its domain and say what its pattern cannot see.
  >
  > ⇒ **The ledger entry must carry the domain, not just the number.** A bare
  > 322 is what freezes a false completeness onto seventeen slices.

- **`AC-5` — no code moved.** `git diff --stat` against the base shows zero
  changed paths under `crates/`. An inventory that edits its own subject has
  invalidated itself.

- **`AC-6` — `D6` reports a verdict either way**, and a contradiction is a
  **stop**, not a widened scope.

## Scope

`crates/ken-runtime/src/cranelift_backend/` and `boundary_value_clif.rs`.

⛔ **No code movement, no renames, no visibility changes, no test relocation.**
An inventory only. Source text is a census aid, not the only semantic oracle.

## Sequencing

Campaign node #8, cut item 1 — first of the phase, gating every other child.
The phase record and the full 18-item cut are in
[[RT-BACKEND-MODULE-SPLIT]].
