---
name: a-type-narrowing-discharges-one-clause-and-a-producer-enumeration-carries-the-rest
description: A non-Option parameter proves only that whoever calls that function supplies a value — the safety property is a CALL-GRAPH fact carried by a prose producer enumeration, and that enumeration is where the defect lives
---

# A type narrowing discharges one clause; a producer enumeration carries the rest

**Measured 2026-08-10 on `7bfc8ae5` (`RT-DYNAMIC-ARM-SCALAR-MERGE` `c1`).**

The merge's headline was *"implicit legacy fallback is structurally
unreachable"*, delivered by changing one parameter from
`Option<&NativeProcessSymbols>` to `&NativeProcessSymbols`. The doc comment
discharging it:

> ⛔ The authority is a **required** parameter, not an `Option`. Production
> callers obtain it from `program_authority`, which fails closed; the only other
> producer is the `#[cfg(test)]` synthetic entrypoint, which does not exist in a
> production build. **There is no third way to reach lowering.**

**The type supplies exactly one clause — *required parameter*.** It proves that
whoever calls *that* function supplies a value. It proves **nothing** about
which function a package-backed compile calls, and that is the whole property.

⇒ **Everything after the semicolon is a producer enumeration in prose**, and it
was wrong in both directions:

- **`program_authority` exists in no tree.** One hit at the merge — the comment
  itself. Zero at the declared base, so not a stale operand. `git log -S` pinned
  it: introduced as `fn program_authority`, renamed to `program_admission`
  **inside the same branch**, and the sibling's citation did not move.
- **A third production producer was omitted** — `seed_only_legacy_authority()`,
  ungated, wrapping a `pub(crate)` `legacy_prelude()`, reaching lowering through
  the `Option`-taking seed entry. The true claim is narrower and the *adjacent*
  comment already states it correctly: no third way for a **package-backed**
  program.

## Why the citation defect protects the enumeration defect

A reader auditing *"is there really no third way?"* greps the cited name, gets
zero hits, and **silently repairs it** to the obvious neighbour. The citation
defect then vanishes — and **the enumeration is never re-audited**, because the
repair felt like the finding. Same mechanism as
[[an-unresolvable-citation-gets-silently-repaired-by-the-reader-and-the-defect-vanishes]],
here protecting a *sibling* defect rather than itself.

⇒ When a citation fails to resolve, **do not stop at naming the right symbol.
Audit the claim the citation was there to support** — the broken name is a
marker that this sentence was not re-read after the code moved.

## Report the property and the argument SEPARATELY

I tested the impossibility on four axes (parameter, wider-typed sibling, module
scope, call graph — [[attack-an-impossibility-claim-at-module-scope-not-only-the-signature]])
and **all four held.** The property is true. Only the reason given for it is
false.

**Say both, in that order.** *"The mechanism is correct; the argument for
trusting it is not"* is actionable and cheap. Leading with the defect on a merge
whose safety property survives every attack reads as an alarm and invites a
re-audit nobody needs.

## Follow the call-graph axis to its END, not to the signature

The last axis is the one that takes real work: a caller taking non-`Option`
authority only moves the question up a level. I followed
`&entrypoint.process_symbols` from the packaging call to the **single production
writer of that field** and confirmed it is a struct literal, not the legacy
constructor. **Stopping at "this caller takes a required parameter" would have
cleared a path I had not actually traced** — and bound it: I checked the writer
is not the legacy constructor, not how its fields are derived.

⇒ **A non-`Option` parameter is a redirection, not a closure.** Chase the value
to a producer you can see, or say which hop you stopped at.

## The UNQUALIFIED statement is the outlier, and that is why it reads as law

**Added after the Steward's triage confirmed both claims and sharpened this
one.** The tree carried the *correct, qualified* sentence **twice** — at
`core.rs:1738-1742` and again in `seed_only_legacy_authority`'s own doc
(*"Nothing package-backed may call this: the program entries below take a
resolved authority and have no `None` to fill"*) — and the over-claiming form
**once**, eleven lines from the function that was renamed out from under it.

**Majority-correct is the dangerous configuration, not the safe one.** There is
no contradiction to trip over: the qualified sentences do not *refute* the
unqualified one, they read as **it plus detail**. So a reader comparing them
takes the shortest, strongest form as the summary and the others as caveats —
**confidence gets read as authority, and the outlier wins.**

⇒ When you find one artifact stating a property, **go count how many others
state it and compare their qualifiers.** A lone unqualified restatement among
qualified siblings is a finding; a lone *qualified* one among unqualified
siblings usually means the qualifier is the discovery. Either way the
**variance across restatements is the signal**, and it is invisible if you read
only the site you started at — sibling of
[[a-corrections-sweep-population-is-its-own-diff-scope]].

## The durable half can be the framing you stated in passing

Four ACs came out of this. Three were the concrete repairs I named. The fourth —
*"the argument rests on a call-graph fact rather than the parameter's type"* —
was a sentence I wrote to explain **why** the enumeration mattered, not as a
deliverable, and it is the one the Steward called the durable half.

⇒ **The mechanism-level sentence outlives the site-level ones**, because the
repairs close on one file and the framing binds the next author. Write it
explicitly rather than leaving it as the connective tissue between findings.
