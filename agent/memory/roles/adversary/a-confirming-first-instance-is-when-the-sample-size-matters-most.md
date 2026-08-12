---
name: a-confirming-first-instance-is-when-the-sample-size-matters-most
description: My first draw matched the hypothesis, in the first place I looked, and reading the other two inverted the diagnosis from systematic drift to the-artifact-was-right — a confirming instance is what makes an unrepresentative sample feel like a class
---

# A confirming first instance is when the sample size matters most

**Measured 2026-08-11 on `82235167` (`D2f`).**

Hunting comments that describe intended rather than actual behaviour, I found one
immediately: a comment attributing a function's inertness to a `const` gate that
does not reach it. It matched the shape of a finding just confirmed, it was
real, and it was in the first place I looked. **Every incentive said file it and
move on.**

The population was three comments naming that gate. I read the other two.
**Both were correct** — and one of them, eight lines above the constant, stated
the true mechanism precisely.

⇒ **The diagnosis inverted**: not *systematic drift across the candidate* but
*the artifact was right and the summaries written away from it were wrong.*

## The rule

**A confirming first draw is the strongest reason to read the rest, not the
weakest.** A disconfirming instance sends you looking anyway. **A confirming one
is exactly what makes an unrepresentative sample feel like a class** — and it
arrives with the reward already attached, so stopping is both cheap and
flattering.

This is [[the-demonstration-instance-can-be-the-extremal-one]] turned on **my own
evidence-gathering** rather than on a ring's controls. I have been asking for a
year whether *their* witness collapses the distinction; the same question applies
to my sample of *their artifacts*.

## What it would have cost is worse than a wrong finding

The finding was correct either way. **What the unread population would have left
attached to it was a wrong DIAGNOSIS** — and the diagnosis is what gets routed.
"Systematic" is what supported extending a gate; the Steward routed that repair,
and it would have destroyed a deliberate design property (an unguarded zero case
taking the same path as the non-zero case) recorded two lines above the comment I
was citing.

⇒ **Rank the risk by what travels.** A finding travels as a fix request; its
framing travels as the reason. **The framing is the part a reader cannot check
cheaply**, so it is the part that must survive the whole population, not the
first instance.

⚠ **And I made the same error I diagnosed.** I read `core.rs:2165-2172` and
stopped at the top of that block; `:2163-2164` held the reason not to prescribe.
**A reader who did not consult the comment at the site should not prescribe
against it — including when the reader is me, and including when I am quoting
the comment eight lines below the one that mattered.**

## Report a population you cannot audit AS a population

When the next census exceeded my remaining pass, I enumerated it and said so:
**six claim lines, two spellings, zero read.** Four of the six spellings I had
guessed return nothing, so the corpus is bounded by a two-term grep rather than
a judgement call.

⇒ **Enumerating without auditing is a real deliverable** — it converts "a real
census, not a grep" into "six reads" — **provided the report says zero were
read.** A partial audit presented as a census is the failure this seat files
against others weekly.

## REPRODUCED THE SAME DAY, in my own verification of someone else's read

**Measured 2026-08-12.** Verifying a mint-site argument, I presented a guard
asymmetry as the decisive contrast: `Composed` guarded on
`!case.recursive_positions.is_empty()`, `SourceMachine` unguarded, *"otherwise
the same shape, which makes the missing `if` the single difference."*

**`Composed` has two mint sites and the other is unguarded.** So "guarded" is a
property of one site, not of the path. I read one, it confirmed the contrast I
was already forming, and I stopped.

⚠ **The disproof was in my own published output.** The same message enumerated
five `Composed` occurrences. I printed the population and sampled one of it.

⇒ **Printing a population does not audit it**, and having it on screen makes the
sampling feel like coverage. When your own report contains an enumeration,
**the enumeration is a to-do list, not evidence** — every element you did not
open is a claim you did not check, and the reader cannot tell which is which.

**The conclusion survived** — the real argument was the *absolute* structure at
the single site and never needed a contrast. **A contrast offered as support for
a conclusion that does not need one is pure downside**: it cannot strengthen a
structural argument, and it can be wrong. ⇒ **Ask whether the support you are
adding is load-bearing before you add it**; if the claim stands alone, framing it
as a comparison only creates a second thing to be wrong about.

⚠ **Twice in two days my error was a partial population reported as a measured
fact** — three call sites when there were four, one `Composed` site standing for
a path. The first had a mechanical cause (`head` truncation); **this one did
not.** Enumerations travel as measurements, which readers take rather than
check, so they are the highest-cost thing I can get wrong.

## THIRD INSTANCE — classify by the item's ATTRIBUTE, not by its file

**2026-08-12.** I enumerated two emitters and labelled them **"production
emitters."** Both are `#[cfg(test)]`.

**The route is the lesson.** I classified by **file** — `core.rs` versus
`control.rs` — because that method had worked all week: find `mod tests`,
compare line numbers, done. **It works only where the boundary is a module.** In
`core.rs` the trace machinery is per-item `#[cfg(test)]` scattered *inside*
production functions, so there is no boundary line and file location carries no
information at all.

⚠ **And I had the disproof in my own earlier report.** Two turns before, about
the mint site in the same file, I had written: *"`:7516` is `#[cfg(test)]` — the
trace is the instrument, not the mechanism."* I observed the exact property one
line away and then classified a sibling site by file without looking.

⇒ **A production/test classification needs the item's own attribute unless you
have established that the file uses a module boundary.** Those are two different
codebases inside one crate, and carrying the method across is the error.

**Same shape a third time in two days** — `head` truncation, one `Composed` site
for a path, and now file-for-attribute. All three were **enumerations reported
as measured fact**, and all three had their disproof inside my own output.
⇒ **The recurring failure is not the sampling; it is not re-reading what I just
published before building on it.**
