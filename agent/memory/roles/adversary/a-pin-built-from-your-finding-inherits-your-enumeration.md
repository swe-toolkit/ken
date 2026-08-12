---
name: a-pin-built-from-your-finding-inherits-your-enumeration
description: When a ring turns your finding into a pin, the pin's population is YOUR population — so an under-enumerated finding becomes an under-scoped gate that is green, faithful, and blind in exactly the place you did not look
scope: roles/adversary
---

# A pin built from your finding inherits your enumeration

**Measured 2026-08-09 on `74c60c5d` (`RT-MATCH-RECURSOR-CONSUMERS` `D8`).**

On an earlier merge I reported, preventively, that putting an unconditional
`pub` transport in production `main.rs` was safe because of **three** manifest
facts — resolver 2, the feature not defaulted, and `ken-cli` taking the
dependency unfeatured in `[dependencies]` with the featured edge only in
`[dev-dependencies]`. I wrote it down because "the premise is invisible at the
call site and a future manifest edit could remove it silently."

The ring built a 760-line pin. **It pins exactly those three facts, against
exactly the three manifests I named.** Its header states the argument "rests on
exactly three facts about three manifests."

**It rests on a fourth.** No *other* workspace member may enable the feature on
a **normal** `[dependencies]` edge. Resolver 2 withholds unification for
**dev-dependency** edges — which is the whole of what fact 3 buys — but not
across normal edges of sibling members in one `cargo build --workspace`, which
is what CI runs. Three sibling members already held that edge, each one token
from live.

⇒ **The pin was faithful to my finding and incomplete about the property.** No
reviewer failed: the pin does what the finding said, and the finding was the
specification. **When your finding becomes a pin, its population is your
population** — so the enumeration duty at finding-time is not reporting-grade,
it is **gate-authoring-grade**, and you do not get told when the promotion
happens.

## "Rests on exactly N facts" is a closure claim with no proof

The phrase is the tell. I wrote *three facts*; the header wrote *exactly three
facts*; nobody proved the enumeration closed. Same shape as
[[an-enumeration-needs-a-proven-closure-not-a-better-grep]], arriving as a
count in prose rather than as a grep.

**The repair is to take the population from the artifact's own declared list.**
The workspace manifest declares its `members` — eight of them. A check that
iterates that list is closed **by construction** and survives a ninth member; a
check that names three paths is closed by my memory of what mattered. This is
[[close-a-class-partition-the-declared-population]] applied to the fix rather
than to the hunt.

## Both instruments were keyed on the same graph

The companion artifact test builds `--bin ken`, which selects only `ken-cli`'s
graph — an invocation in which cross-member unification **structurally cannot
appear**. So the two pins did not fail independently and then coincide; they
were **the same question asked twice**. Two green instruments read as
corroboration and were one measurement — the
[[differential-oracle-is-blind-to-a-shared-premise]] shape, where the shared
premise is *which dependency graph gets built*.

⇒ Before crediting two pins as independent coverage, ask **what each one
actually varies**. If both hold the same operand fixed, their agreement carries
no information about it.

## Measure the resolver, do not reason about it

`cargo tree -e features,no-dev --workspace -p <dep>` **resolves without
building**, so the whole A/B is affordable under the no-`--workspace`-build
rule: baseline `0`, one-token mutation on a sibling's normal edge `1`,
`-p ken-cli` graph `0` both ways, and the pin green throughout. The baseline
`0` is the positive control that makes the `1` attributable — without it, see
[[a-green-mutation-does-not-tell-you-which-blindness-let-it-through]].

⇒ **A claim about a build system's semantics is measurable, so never file it as
an assertion.** And run the pin *with the hazard live*: a pin that stays green
under a mutation the property forbids is the finding, stated in one line.
