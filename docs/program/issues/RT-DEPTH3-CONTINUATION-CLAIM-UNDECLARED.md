---
id: RT-DEPTH3-CONTINUATION-CLAIM-UNDECLARED
title: "At depth 3 of the oriented-subcontinuation bracket row, object emission refuses with ContinuationSpecialization `the claimed continuation target was not declared into this function` (core.rs:11268) -- the claim's identity is absent from `function_local.continuation_calls`. This is the SECOND blocker on ledger row 9, invisible until now because depth 2 panics first, so no measurement of that row had ever executed depth 3."
status: draft
owner: runtime
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Measured by runtime-implementer under RT-SUBCONTINUATION-LIFO-RELEASE-ORDER's D0 (evt_707t1acbaxp62): the row body is `for depth in 2..=3`, depth 2 panics on the release-order assertion, and running both depths under catch_unwind exposed a depth-3 refusal nobody had seen -- order-independent, reproduced with depth 3 run first, separate temp dir and separate build_native_program per depth. Architect ruled it OUTSIDE RT-DISCHARGE-ARM-SUBSTITUTES-PLAN-FOR-OBSERVATION and therefore outside the standing do-not-repair-toward-green prohibition (evt_ptqeeexkz2xa), and directed the Steward to file an owner. Steward VERIFIED that ruling at source rather than inheriting it, and it is stronger than stated: `independent_contract` and `realized_call_words` have ZERO occurrences in ALL of crates/ at origin/main 5899268451d42e7c1337929a996921d6483b6541, not merely in units.rs, so no refusal observed on this tree can be an exit of that rule. Steward-filed per COORDINATION section 2 (agents cannot create tracked work)."
---

> ## THE ROW THIS BLOCKS IS ALREADY COUNTED, AND THIS NODE IS WHY IT CANNOT CLOSE
>
> The `px8ta_oriented_subcontinuation.rs` row
> `public_two_three_level_brackets_finish_and_release_lifo` is one of the
> **fifteen selected** ignored rows, the operator's top-priority population.
> **Repairing the release-order defect does not clear it.** The row is two
> blockers under one `#[ignore]`, and this node owns the second.
>
> **Do not treat this as a reason to defer the ordering repair.** Both are
> owed; neither subsumes the other. The ordering defect is
> `RT-BRACKET-RELEASE-ORDER-PARITY`'s.

## What was measured, and at what base

**Base: `origin/main` at `5899268451d42e7c1337929a996921d6483b6541`.** Every
coordinate below is perishable and was read there.

    crates/ken-cli/tests/px8ta_oriented_subcontinuation.rs
      fn public_two_three_level_brackets_finish_and_release_lifo()
      body: for depth in 2..=3

    depth 2   EXECUTES, releases in acquisition order   -> RT-BRACKET-RELEASE-ORDER-PARITY
    depth 3   REFUSES AT OBJECT EMISSION                -> THIS NODE

    Packaging(ObjectLinkerPackagingError { stage: ObjectEmission,
      field: "checked_process_object",
      reason: "unsupported runtime-IR lowering: ContinuationSpecialization:
               the claimed continuation target was not declared into this
               function" })

**The refusal site** is
`crates/ken-runtime/src/cranelift_backend/lowering/core.rs:11268`. It is the
`ok_or_else` on an exact lookup, reached after two earlier guards on the same
construct have already passed:

    :11241  defining_unit              must be Some   -- passed
    :11252  defining_emission_owner    must be Some   -- passed
    :11260  function_local.continuation_calls.get(&identity)   -- EMPTY for this identity

⇒ **A unit is being defined and an emission owner is bound; what is missing is
the claim itself.** That narrows the question considerably: this is not "no
context", it is **a claim looked up in a function that did not declare it.**

## The a-priori best guess, stated as a guess so it can be attacked

**The claim was issued under one emission owner and is being resolved in a
function that declared a different owner's tokens**, so
`declare_owned_in_func` (`units.rs:6383`) populated
`function_local.continuation_calls` from a set that does not contain this
identity.

**The tree already documents one way this happens**, which is why the guess is
worth building rather than measuring first. `units.rs:5121-5131`, verbatim:

> *"An earlier cut minted `ContinuationEmissionOwner::Fusion(fusion.id)` here
> and described it as the ambient authority. It was never bound to anything:
> the sole `AmbientBodyAuthority::bind` below takes `causal_owner`, and the
> planner issues no `Fusion`-owned tokens, so asking for them returns the empty
> set and the producer's first causal call refuses with **"the claimed
> continuation target was not declared into this function"**."*

**That comment describes this exact refusal string arising from an owner
mismatch, and says the condition is not live today.** So either a second owner
mismatch of the same shape exists on the depth-3 path, or the fusion case is
live after all and that comment is stale. **Both are cheap to separate and one
of them is the answer.**

> ### WHAT WOULD REFUTE THE GUESS
>
> The identity being absent for a reason that is not an owner mismatch — the
> claim never issued at all, or issued and then dropped. **Check which before
> building**: print the owner the claim was issued under and the owner whose
> tokens the defining function declared, and compare. If they agree, the guess
> is dead and the question moves upstream to issuance.

## What must not happen

- **Do not repair the refusal by widening the lookup — and here is what that
  would look like in the diff, which is stronger than a prohibition.**
  Production resolves the callee exactly once and has no fallback of any kind:

        :11260-11271   the lookup. Unconditional, no cfg.
                       continuation_calls.get(&identity), refuse on absence.
        :11283         #[cfg(test)]   the D4 substitution + D7 sentinel, to :11368
        :11369         #[cfg(not(test))]
        :11370           let target = exact_target;        <- production, ONE LINE

  ⇒ **Any repair that makes depth 3 succeed by accepting a different callee has
  to ADD a production fallback where there is currently a one-line binding.**
  Look for a new branch under `cfg(not(test))`. **A comment can be read past; a
  one-line binding that has to grow a branch cannot.** The refusal is the only
  outcome production has for an undeclared target, so making it succeed means
  minting a resolution that does not exist today.

  > **This node first cited `core.rs:11280`** — *"No fall back to exact, and no
  > widening"* — **as the production design rule. It is not.** That sentence is
  > the doc block of the `#[cfg(test)]` arm three lines below it, and it is
  > about the **D4 control's substitute search**, not the repair. Caught by the
  > Architect (`evt_6hbdybc9xsk19`), verified here at source before the edit.
  >
  > **The citation failed in the direction that inverts the ban.** A reader sent
  > to `:11280` lands inside test machinery and concludes either that the ban is
  > test-only and does not bind the repair, or that there is no production ban
  > at all. Neither reader is careless — the coordinate genuinely lands there.
  > The same caution applies to `:11296-11300`'s *"not a licence to import a
  > FuncRef from another function"*: a fact about the mutation's search.
  >
  > **A cited coordinate is not a verified one, and reading the surrounding
  > lines is not the same as reading the `cfg` they sit under.** I read
  > `:11240-11280` and attributed a test-control comment to the production path
  > because nothing in those forty lines said `cfg` — the boundary was three
  > lines past where I stopped.
- **Do not reintroduce `ContinuationEmissionOwner::Fusion(fusion.id)` at
  `units.rs`.** A repair that reaches for it is the removed conflation coming
  back. **The ban is at `:5121-5122`** (*"There is deliberately no `Fusion(id)`
  owner in this function. An earlier cut minted
  `ContinuationEmissionOwner::Fusion(fusion.id)` here…"*) and closes at
  `:5130-5131` (*"…this variable must not be reintroduced to suggest
  otherwise."*). `:5137` is the assignment that establishes the RIGHT owner,
  `ContinuationEmissionOwner::Predeclared(fusion.producer_owner)`.

  > **This bullet said the ban was at `:5137` and that "the comment there bans
  > it by name". `:5137` is code.** Sixteen lines low, landing on the line that
  > establishes the correct owner rather than the comment forbidding the wrong
  > one. Caught by the Architect (`evt_45z65hrp52n5f`); re-measured here.
  >
  > **It does not invert** — nobody reads `Predeclared(...)` as licensing
  > `Fusion` — so it was a dangling pointer, strictly less dangerous than the
  > `cfg(test)` one above it. **What makes it worth recording is where it sat.**
  > It is the adjacent bullet in the same section, and the pass that fixed the
  > citation above it walked straight past it — inside a commit whose whole
  > purpose was to repair a miscited coordinate, in a node that says in its own
  > voice that a cited coordinate is not a verified one.
  >
  > ⇒ **A fix to one citation must re-verify every other citation in the same
  > section, not the one that was reported.** A correction inherits the
  > reporter's scope, and the reporter was answering a question rather than
  > auditing a list. Every coordinate in this node has now been resolved at
  > `origin/main 5899268451`: `core.rs` `:11241`, `:11252`, `:11260-11271`,
  > `:11268`, `:11280`, `:11283`, `:11296-11300`, `:11368`, `:11369-11370`;
  > `units.rs` `:5121-5131`, `:6383`.
  >
  > ### AND A THIRD KIND: A COORDINATE THAT IS CORRECT TODAY AND EXPIRES
  >
  > This node originally cited the px8ta row as `:285`/`:286`. **Those were
  > correct at `5899268451` and were scheduled to become wrong on a commit
  > already ahead of this one in the same queue** —
  > `RT-SUBCONTINUATION-LIFO-RELEASE-ORDER`'s `D1` (`fa4fc3647b`) rewrites that
  > region under hunk `@@ -261,28 +261,69 @@`, shifting the row by +41 and
  > replacing the `#[ignore]` reason string outright. Caught by the Architect
  > (`evt_6w1f0dhex2k3r`), who measured all seven queued candidates against
  > this node's three cited files and found exactly one exposure.
  >
  > **The previous two citation defects were wrong when written; this one was
  > right when written.** Naming the revision it was resolved against is honest
  > and does not help — the revision it names is the one about to stop being
  > `main`. **A contention check asks whether two candidates collide. It does
  > not ask whether one moves coordinates the other cites.**
  >
  > ⇒ **The row is now cited by symbol name**, which survives that rewrite
  > unchanged. This is the convention `RT-DISCHARGE-ARM`'s banner already
  > states: *"Lead with the symbol name. The line number never travels without
  > its anchor."* The `#[ignore]` reason text is being replaced too, so a
  > reason-string quote would not have served as the anchor either.
- **Do not un-ignore the row.** The ordering blocker is still live and owned
  elsewhere; un-ignoring buys a red row for no information.
- **Do not fold this into `RT-BRACKET-RELEASE-ORDER-PARITY`.** Different
  mechanism, different layer, different owner. One `#[ignore]` covering two
  defects is what hid this one for as long as it was hidden.

## The generalizable finding, which outlives this node

**A test body that loops over cases fails at the first one and reports nothing
about the rest.** This row looped `2..=3`, and every measurement ever taken of
it — the ledger's included — saw only depth 2. **The population of fifteen
counts `#[ignore]` attributes, not defects**, and the instrument that separates
them (run past the first panic under `catch_unwind`) has not been applied to the
other fourteen rows.

That measurement is not this node's work. It is recorded here because this node
is the existence proof that it is worth doing.
