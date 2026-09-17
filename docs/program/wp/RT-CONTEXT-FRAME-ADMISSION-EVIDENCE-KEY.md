# RT-CONTEXT-FRAME-ADMISSION-EVIDENCE-KEY — work package

**Owner: Team Runtime. Size S. Tier T1. Gate: none.**
**Implementation base: `origin/main` as of whenever you cut your branch. NAME
IT in your first post — AC-0 requires you to re-measure at your own base.**
**Inputs below measured at `b0421afd0816c44347bb3281a5062cc0ada00c8e`.**

**RE-HOMED from `RT-CONTEXT-FRAME-SLOT-HOLDS-ONE-PER-FUNCTION` §3, which is
closed REFUTED.** The hazard §3 named did not depend on that node's premise and
would have been deleted with it.

## 1. Objective

**The two readers key differently, DELIBERATELY, and the in-tree argument for
why is sound. That question is CLOSED — do not re-open it.**

What is open is one step further in:

> **The permission seat is safe BECAUSE the gather seat re-checks. Is that
> delegation load-bearing, and is it recorded where its ENFORCER will read
> it?**

It is currently recorded **only at the seat that depends on it**, never at the
seat that provides it. **Two deliverables: a verdict, and about four lines of
comment.**

## 2. THE DISAMBIGUATOR — CARRY IT AS A PROPERTY, NOT AS TWO LINE NUMBERS

A reviewer greps `constructed_context_frame`, gets two seats, and **cannot tell
which one they are looking at from the grep.** Both are real, which is exactly
what makes the wrong one read as plausible. The Architect asserted a negative
from the wrong seat on 2026-09-16 and handed this over on that basis.

    THE PERMISSION SEAT   returns Result<bool>.   Answers WHETHER.
    THE DATA SEAT         returns the operands.   Answers WHAT.

**Use the return type, not the coordinate.** Coordinates go stale; `core.rs` is
the lane's hottest file.

## 3. Fixed inputs — measured at `b0421afd0`, re-verified independently

Contributed by the Architect (`evt_3pptybqw28dgx`) and **re-measured by the
Steward from the objects before being written here.** Both of us ran each one.

**1. The permission gate has exactly ONE caller, crate-wide.**

    git grep -n recursive_position_captures_all_planner_recoverable <sha> -- crates/
    core.rs:13532   the definition
    core.rs:13673   the one call site
    2 hits, nothing else in the tree

**2. The gate is materially stronger than "body origin plus two
cardinalities."** Before the frame shortcut is reachable, `core.rs:13538-13549`
resolves the owning context by body origin and **refuses outright when two
contexts own one body**, verbatim:

> *"Two contexts for one body is an ambiguity this gate refuses rather than
> resolving by taking the first."*

⇒ **`claims` is DETERMINED by `body_origin`, not selected from candidates.**
The permission key is two cardinalities over a **uniqueness-enforced** lookup.
That is a different object from a two-field key, and §3 of the closed node did
not account for it.

**3. The frame gates WHETHER and never WHAT.** At the call site the queried
origin comes from the closure structure (`core.rs:13672`,
`child_occurrence(argument.static_origin, 0, body)`) and the value the arm
returns is derived the same way (`core.rs:13680`). **The frame is nowhere in
the data path.**

⇒ **A wrong frame can produce a wrong yes/no. It cannot produce a wrong body.**
Verifiable in four lines, not argued.

**4. The delegation is real and it is fail-LOUD.** The gather's `.filter()`
compares no cardinalities — correct, and it is why that seat reads as having
none. The re-check is in the **body**, `calls.rs:1008-1052`: three
`Err(unsupported(...))` returns with named diagnostics, against **two
independent authorities**, `claims` (the ordered projection) and `header` (the
declared frame). Its own comment says so:

> *"Both authorities, not one. `header` is the declared frame and `claims` is
> the ordered projection; checking the constructed run against each separately
> is what makes a disagreement BETWEEN them visible here rather than absorbed."*

⇒ `core.rs:13572-76`'s *"the two cannot disagree silently"* is **literally true
and enforced, by more than it claims.**

> ### WHY THE READER SET IS COMPLETE AND NOT MERELY THOROUGH
>
> Two seats independently grepped `constructed_context_frame` and agreed.
> **Agreement between two instruments that share a method is not
> independence** — a reader reaching the field under some other name would be
> invisible to both, identically.
>
> **It cannot currently be reached under another name — but NOT because Rust
> forbids it.** There are three ways a field is read or written with its name
> nowhere in the diff, and **all three are verified ABSENT for
> `FunctionLocalRefs` at `b0421afd0`**, each checkable in one command:
>
>     1  A DERIVE          Clone/Debug/serde read EVERY field and name none.
>                          ABSENT: mod.rs:1089 is blank, :1090 is the struct
>                          line -- no attribute of any kind above it.
>     2  STRUCT-UPDATE     `FunctionLocalRefs { x, ..base }` copies the rest
>                          unnamed.  ABSENT: SEVEN literal construction sites
>                          (mod.rs:924 production, plus six test fixtures) and
>                          not one uses `..`.
>     3  AN ACCESSOR       ABSENT for this field: one impl block exists,
>                          joins.rs:2758, whose single method touches
>                          `trap_exit` and nothing else.
>
> ⇒ **The completeness claim is conditional on those three states, not on a
> fact about Rust.** Every CODE occurrence is accounted for — eleven, all under
> `crates/ken-runtime/src/cranelift_backend/lowering/`:
>
>     mod.rs:1249                   the declaration
>     mod.rs:964                    per-function init to None (PRODUCTION)
>     core.rs:10866                 the sole write
>     calls.rs:986                  the DATA seat
>     core.rs:13577                 the PERMISSION seat
>     core/tests/*.rs  x6           test-fixture inits to None
>
> ⇒ **While those three hold, the grep is a COMPLETE instrument for this
> population, not a thorough one.** Structural argument; it does not need a
> third seat.
>
> ### THE TRIPWIRE IS THREE STATES, AND THE DANGEROUS TWO ARE THE ORDINARY ONES
>
> **Re-check all three at your base, not the accessor alone.** A new accessor
> is a deliberate act. The other two are routine edits that would pass review:
>
> - **`#[derive(Debug)]` is the most ordinary future edit to this struct** —
>   added to print lowering state while chasing an unrelated bug. It creates a
>   read with the field name nowhere in the diff, and it is too routine to draw
>   a second look.
> - **Struct-update is worse, because it is a WRITE.**
>   `FunctionLocalRefs { trap_exit: .., ..base }` in a new constructor or test
>   helper **carries `constructed_context_frame` from one function-local state
>   into another's, unnamed.** The single-write claim at `core.rs:10866` is what
>   this entire node stands on, and **`..base` is exactly how a second write
>   appears without a second write site.** In review it reads as tidying a
>   constructor.
>
> **Name the states, not a count.** A count goes stale and fires on correct
> states; these three are each one command.
>
> **THE CRATE-WIDE COUNT IS 15, NOT 11, AND THE EXTRA FOUR ARE PROSE.** They
> are the `#[ignore]` label strings in `crates/ken-cli/tests/px7l_*.rs` and
> `px7m_*.rs`, which *discuss* this field. **A grep on the field name cannot
> tell a use from a mention of one** — the same instrument defect the runtime
> implementer hit on these exact files, where a search for refuted phrases
> matched the new labels that quote them in order to refute them.
>
> ⇒ **Re-establish the ELEVEN against `lowering/` only.** A crate-wide count
> of 15 is the expected state, not a tripwire. **What would break the argument
> is a new accessor**, and it would look like an ordinary refactor.

## 4. THE FINDING: RECORDED AT THE SEAT THAT DEPENDS, NOT THE SEAT THAT ENFORCES

**`calls.rs:1008-1052` is the sole enforcement of `core.rs:13578`'s
correctness. `calls.rs` says so nowhere.** Measured over all **2499** lines,
not a window:

    recursive_position_captures    0 hits
    planner_recoverable            0 hits
    "permission"                   1 hit   -- worker ARITY, unrelated
    "admission"                    7 hits  -- all same-SCC / constructor-depth,
                                              a different subject, none this seat

The comment block at `calls.rs:960-975` is long and careful and explains the
filter **entirely in terms of the gather's own correctness** — why the condition
names the two ways the gather falls short. **It never says this code is also
what makes another seat's permission sound.**

> ### THE RATIONALIZATION IS THE OBVIOUS ONE, NOT AN EXOTIC ONE
>
> An editor at `calls.rs` who relaxes those three checks sees no reason not to,
> and the sentence that licenses it is:
>
> *"The admission query already matched this frame on body origin and both
> cardinalities — re-checking here is redundant."*
>
> **That is TRUE about what the other seat does and FALSE about what it
> means.** Acting on it deletes the only enforcement of a second property
> nobody named at that site.
>
> **Worse, the coupling runs BOTH ways.** `core.rs:13572-76` can be read as
> licensing a relaxation at `calls.rs`, and a relaxed `calls.rs` can be read as
> licensing a narrower key at `core.rs`. **Neither is visible from the seat
> that would be edited.**

## 5. Deliverables

1. **A verdict on §6's mutation control** — is the delegation load-bearing.
2. **The comment at the ENFORCING seat**, about four lines, naming what these
   three checks are also holding up and that a caller depends on them. This is
   the deliverable that outlives the verdict.
3. **No key change, and no relaxation.** If something unexpected turns up, it
   is a finding and comes back to the Steward as its own node.

## 6. Acceptance criteria

**AC-0 — A MUTATION CONTROL AT THE ENFORCING SEAT, AND IT MUST BE ABLE TO
FAIL.** Ask the reader-side question of this very frame — *which AC fails if
there is nothing here?* — because "none" is what closed the predecessor.

Disable **one at a time** and report whether behaviour changes:

    (a) calls.rs:1008   claims.len() != context_captures.len()
    (b) calls.rs:1028   declared_arguments + worker_captures.len() != header.parameters
    (c) calls.rs:1042   context_captures.len() != header.captures
    (d) the :989-991 filter, with continuation_origin and recursive_position
        DROPPED -- i.e. the data seat keyed the way the permission seat is

    CHANGES    -> the delegation is load-bearing, measured. Deliverable 2 is
                  mandatory rather than nice.
    NO CHANGE  -> see AC-1 BEFORE reading this as a result.

**AC-1 — REACH FIRST. A NULL MUTATION RESULT IS NOT A RESULT UNTIL YOU SHOW THE
CODE RAN.** Establish, and paste, that your exercising program **reaches
`calls.rs:1008-1052` at all.**

> ### THE WITNESS HAS A SHAPE. AIM AT IT RATHER THAN STUMBLING INTO IT.
>
> **The two seats select their context by DIFFERENT KEYS**, measured at
> `b0421afd0`:
>
>     core.rs:13539   .filter(|context| context.worker_body_origin() == body_origin)
>                     plus the uniqueness refusal at :13546-13549
>     calls.rs:914    .find(|candidate| candidate.id() == context)
>                     and calls.rs:976   let claims = view.captures()?
>
> **One selects by worker body origin, the other by context id**, so the two
> `claims` are not the same object and `calls.rs:1008` is **not** a restatement
> of `core.rs:13580`.
>
> ⇒ **`:1008` is precisely the check that catches "the frame was admitted
> against the context owning the BODY, but this call is bound to a DIFFERENT
> context."** That is the delegation `core.rs:13572-76` names, made concrete.
>
> **So the witness to aim for is a program where the context bound at the call
> site differs from the context owning the body origin.** A diagnostic produced
> by an arbitrary corruption proves only that the `Err` path **compiles**; this
> one proves the guard **discriminates the case it exists for**.
>
> **A print is a liveness witness; the deliberate divergence is a positive
> control. Reaching a line is not the same as the line's failure path being
> reachable** — prefer the second, and if you can only get the first, say which
> one you have.
>
> **Whether those two selections CAN diverge in a reachable state is not
> established by anyone, and this AC does not assume it** — producing the
> witness is what would show it. If you cannot construct one, that is a
> reportable result, not a failed AC.

> **A mutation that stays green has two causes and they are opposite.** Either
> the check is genuinely not load-bearing, or **nothing you ran ever got
> there.** The four `RT-CARRIED-RESIDUAL-IH-ARITY` rows are the obvious
> exercising candidates and they are **refused early, upstream of this seat** —
> so they are the *least* likely programs to reach it. **State which cause your
> null result has, or it is not a finding.**

**AC-2 — the verdict says which, and does not say "the argument looks right."**
The in-tree argument has now been *measured* right (§3). **An unchecked
plausible argument is what this node converts; a re-affirmed one is not a
deliverable.**

**AC-3 — every claim in §3 is re-run at YOUR base, not inherited.** All of it is
anchored at `b0421afd0` and `core.rs` is the lane's hot file. **If the
uniqueness refusal at `core.rs:13538-13549` is gone, or the permission gate has
acquired a second caller, STOP and return to the Steward** — §3's whole
argument rests on those two.

**AC-4 — reach is stated on every measurement**, per the closed node's standing
condition. A clean result with no stated reach is not a result on this material.

## 7. What this node is NOT

- **Not "do the readers key differently?"** They do, correctly. That question
  is answered in §3 and re-asking it gets "yes, correctly" and closes.
- **Not a repair, and not a relaxation.** Every mutation is reverted.
- **Not a reachability study.** **Nobody has established that two distinct
  occurrences can share a body origin and both cardinalities in a reachable
  planner state, and this node does not need it** — it is about where a safety
  property is recorded, not about whether the hazard it guards fires today. **A
  guard whose sole enforcement is unlabelled is worth labelling before someone
  measures whether it fires.**
- **Not the depth census.** `[[RT-CONTEXT-FRAME-REFUSAL-DEPTH-CENSUS]]` is
  independent; neither blocks the other.

## 8. `RTPROBE-WRITE = 1` IS FOUR ROWS. BINDING ON EVERYONE HERE.

It is not a statement about the planner's reachable states. **The predecessor
node died of exactly that promotion.** Cite it with its reach attached or not
at all.

## 9. Contention

`calls.rs:960-975` and `core.rs:13570-13576` are the two comment regions this
node edits. **It changes no predicate.** Coordinate on those two blocks only.

## 10. Estimated tier: T1

**The failure mode is accepting a plausible argument because it now has
measurements attached to it.** §3 makes the argument stronger, not finished —
and telling those apart is the judgment this node is for.

## 11. Sizing note

**S.** Four mutations, one reach check, four lines of comment. **If it grows,
the growth is the finding — bring it back rather than absorbing it.**
