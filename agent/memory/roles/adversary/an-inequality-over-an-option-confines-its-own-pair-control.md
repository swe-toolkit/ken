---
name: an-inequality-over-an-option-confines-its-own-pair-control
description: A guard written `a != b` over an Option passes vacuously at None-vs-None, and a two-different-values pair control cannot be built out of the absent value — so the control is structurally confined to the inhabited side and can never show the guard is live
---

# An inequality over an `Option` confines its own pair control

**Measured 2026-08-12 on `5c7b40b6` (`D2k-1c-1`).**

Two provenance refusals compared an evidence scope: `recognized.scope != scope`
and `minted.scope != scope`, both `Option<FuncId>`. **`None != None` is false**,
so a compile in which the scope is absent throughout passes both guards without
the comparison carrying any content.

Every control row supplied `Some(FuncId::from_u32(0))` and
`Some(FuncId::from_u32(1))` — and **it could not have done otherwise.** A
discriminator pair needs two values that differ; the absent value is one value,
so **no pair control for this guard can be built on the vacuous side.**

⇒ **The shape of the guard forces its own control onto the inhabited side.**
This is not an author sampling badly
([[the-demonstration-instance-can-be-the-extremal-one]]); the extremal instance
is the only one the control *can* instantiate. So the usual repair — "add a row
where the distinction collapses" — does not exist, and a control set that looks
complete on the axis is complete on the only half it can reach.

## The question the pair control cannot ask, so ask it separately

⇒ **For any guard of the form `a != b` / `a == b` over an `Option`, a nullable
column, a sentinel, or a default-valued field: measure what value PRODUCTION
supplies.** Two reads:

1. **Where does the operand come from at every call site?** Here all three
   ledger calls passed one field, `self.defining_function_id`.
2. **What is that field's setter, and does the live path reach it?** One setter,
   whose callers were **entirely in a different module** governing one of the two
   emission arms — so on the other arm the field is unset for the whole descent
   and both guards are inert.

**That is a structural answer, available without running anything**, and it is
the half the control cannot supply.

⚠ **Failure direction is fail-open, and only where the guard was the answer.**
The doc called cross-scope carriage *"provenance failure, not a licence to
consume it again"* — true of the code and, on the arm where the operand is
absent, false of the compile.

## WHAT I MISSED: RETIRING AN ARM ARMS THE GUARD WITH NO EDIT

**Steward's triage, and it inverts the finding's urgency.** I reported the guard
as inert on one of two emission arms and framed the deciding read as *"once the
route repair delivers a rebound field."* **The node in flight retires that
arm.** If the live population lands on the surviving arm, the operand is `Some`
and **the guard becomes live without anyone touching it or deciding anything.**

⇒ **A vacuous guard is not a stable state when one of the two arms is being
retired.** The vacuity is a property of *which arm runs*, not of the code, so an
unrelated node changes the guard's status as a side effect. **The dangerous
window is while both arms exist, and it closes by accident rather than by
decision** — which is a reason to answer the question now, not to defer it
behind the repair that would make it matter.

⚠ **I had the direction backwards and it was the actionable half.** My bound
said the read needs the route repair first; the *timing* argument says the
opposite, and nothing in my report contained it because I reasoned about the
guard and not about what else was in flight around it. ⇒ **When you find a
mechanism vacuous on one branch of a fork, ask whether the fork itself is
scheduled to disappear** — a scope-of-vacuity argument has a lifetime, and the
node that ends it may not mention the guard at all.

## THE FACT WAS WRITTEN DOWN, in a comment justifying the test constructor

> *`from_u32` is a TEST-only way to name a body identity. Production always
> passes `defining_function_id`, **which is `None` outside the emission
> pass**.*

The author stated the entire finding, in a comment about the fixture, **and drew
no consequence from it** — nothing asks what `None` implies for the two
refusals that comment is about.

⇒ **Read a fixture's justification comment as a claim about production, not as
housekeeping.** *"Production actually passes X"* sitting beside a control built
on `not-X` is the finding, already measured, waiting for someone to compose the
two sentences
([[compose-your-own-measurements-against-the-artifacts-relational-claims]] —
here both measurements were the author's and only the composition was missing).

## A REUSABLE PROOF THAT N REFUSALS ARE INDEPENDENT

Asked whether one of three new refusals was redundant with another — the standard
worry when each claims its own test row — the answer is short and mechanical:

⇒ **For each refusal, exhibit a state the OTHER refusals admit and it catches.**
Three exhibits, three sentences, and redundancy is refuted without running
anything:

| refusal | violation the others admit |
|---|---|
| forward existence | `t[r] = T ∉ minted` — invisible to a loop over `minted`, and precedes the `get` the agreement check needs |
| forward agreement | `t[r] = t[s] = T`, `minted[T].recognition = s` — the converse checks `s` and passes; only this sees `r` |
| converse | `minted[T].recognition = r` but `t[r] = T' ≠ T` — forward laws only ever inspect `T'` |

⚠ **A mutation grid answers a different question.** Dropping refusal K and
watching row K red establishes that row K *depends* on K; **the exhibit
establishes that K catches something nothing else does**, which is what
"redundant" means. Both are worth having and only the second is cheap.

⇒ **Report a flagged risk that turns out ABSENT with the same weight as one that
lands, provided it carries the reason.** *"I checked and it's fine"* transfers
nothing; three exhibits transfer the whole argument and retire the question.

## A synthetic control can be NECESSARY and still owe a label

The rows wrote the ledger's maps directly instead of going through the one
production writer — correctly, because the finding was that the closeout cannot
fail when that writer's two inserts are non-adjacent, so a row built through it
would assume the property under test. **The identities stayed unforgeable
(issuer-only mint); only the relation was synthesized**, which is the right
split.

⇒ **The residue is that nothing said these states are production-unreachable
today.** Six rows constructing impossible ledgers read as reachable defects to
the next person. One sentence — *the laws exist for a future second writer* —
and it is the same disclosure the closeout's own doc already makes for its
redundant containment check, absent one function away.
