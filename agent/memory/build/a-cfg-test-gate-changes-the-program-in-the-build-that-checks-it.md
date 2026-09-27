---
name: a-cfg-test-gate-changes-the-program-in-the-build-that-checks-it
description: A `#[cfg(test)]` gate makes the test build a different program from production, and the difference lands exactly where the checks run. A cfg-split refusal can silently accept only in the test profile; a cfg(test) field on a PartialEq/Eq/Hash/Ord type changes equality only in test builds; and widening such a cfg to a feature turns a structural guarantee into a configuration fact.
metadata:
  type: feedback
---

# A `cfg(test)` gate changes the program in the build that checks it

A `#[cfg(test)]` (or `#[cfg(not(test))]`) item is usually described by its
production footprint: *"zero production footprint"*, *"production omits it"*.
That is a claim about **behaviour in the non-test build**. It says nothing about
what the gate does to the **test** build, which is a different program — and
the test build is the one where every control, comparison and refusal assertion
runs. Three shapes, each measured.

## 1. A cfg-split refusal loses the behaviour in the profile that checks it

**Measured 2026-08-14 on a pre-merge candidate** (read because an Architect
verdict mentioned the Adversary).

```rust
if rederive(...)? != Some(claimed) {
    #[cfg(test)]
    { let reason = if …{ "…eliminator_origin…" } else { "…body_origin…" };
      return Err(planner_error(reason)); }
    #[cfg(not(test))]
    return Err(planner_error("…the general sentence…"));
}
```

**The Architect's finding**: no test profile compiles the `cfg(not(test))` arm,
so the production string is asserted nowhere and an edit to it goes unnoticed.
Correct, and the mirror of the usual `cfg(test)` hazard.

⇒ **The sharper half is about behaviour, not text.** Under `cfg(test)` the other
arm is **absent, not dead**. Drop that inner `return` and the block evaluates to
`()`, the `if` body completes, the loop **continues to the next unit** — the
validator silently accepts. It compiles clean. In production the same edit is
harmless, so **the defect exists only in the test profile, and the tests that
would notice are the ones built with the broken arm.** The Architect's case
loses cover for a correct behaviour; this one loses the behaviour in the profile
that does the checking.

⇒ **When a refusal is split across `cfg` arms, check what the block evaluates to
if a `return` goes missing in each arm separately.** A split refusal has no
shared fallthrough, so each arm is individually load-bearing and neither's
failure is visible from the other.

**A shared const pins the text, not the use.** The proposed repair — hoist the
common tail into a `const` referenced by both arms so an existing assertion pins
the production wording transitively — works while both arms reference it, and
nothing asserts that the production arm still does. A transitive pin through a
shared definition establishes the definition's content, never that the other
consumer still consumes it (the idiom being present is not the property being
covered).

⇒ **Look for the repair with no residual before accepting the transitive one.**
Here the classifier's only input was **not `cfg`-gated**, so deleting the split
entirely gives one message path, direct assertions, and closes the behavioural
hazard too. Check that dependency yourself: the option exists only if the input
is ungated, and that is one grep.

## 2. A `cfg(test)` field on a `PartialEq` type changes identity in test builds only

**Measured 2026-08-13 on `2a1d87a2` (`RT-4B-ENUMERATION-INPUT-SIZE`).** A new
observation field landed on a production struct, `#[cfg(test)]`-gated,
described as having zero production footprint. True of behaviour. **The type
derives `Clone, Debug, Default, Eq, PartialEq`.**

⇒ In a test build, two values with identical real content but different
observation fields now compare **unequal**; in a non-test build they compare
equal. The equality relation differs between the builds, and **the test build
is the one where controls compare things.** "Zero production footprint" says
nothing about the type's **identity**; the gating is exactly what pulls the two
apart.

**The audit is three greps, and all three must be empty:**

1. **Does the type derive a structural trait?** `PartialEq`, `Eq`, `Hash`,
   `Ord`, `PartialOrd` — each is generated over all fields present in that build.
2. **Is the type a field of a container that derives one?** The divergence
   propagates upward silently. (Here it did not: the container derived only
   `Clone`.)
3. **Does anything compare two values of the type?**

Here (1) was non-empty and (2)/(3) empty, so the divergence was real and had no
witness: a cheap preventive finding, to be reported at that size
([[preventive-findings-are-unfalsifiable-so-keep-them-cheap]]). The repair is a
sentence, and the field itself is sound.

**The equality population is wider than `==`.** Equality is invoked by things
that never spell it: a container's derived `PartialEq`, `assert_eq!`/
`assert_ne!`, `contains`, `dedup`, `position`, `.eq()`, set/map membership. And
`grep ": TypeName"` returns function **parameters** alongside struct **fields**;
of two hits here one was a field (whose container's derives are the question)
and one a parameter (no derive, irrelevant). Check which, or you will audit a
signature and report a container.

### It falsifies every neighbouring "and nothing else" claim, silently

An A/B one screen away built its arms as `resolved` (production builder,
observation field set) and `mutated` (`Type::default()` plus one interned key,
observation field `0`), under the comment *"The operand moved, and nothing
else. The key is otherwise the one production derived, so a refusal here is
attributable to the carrier and not to a hand-built key."* **"And nothing else"
is now false at the type level.** Harmless while the test compares views and
slots rather than whole values, and it stops being harmless the moment anyone
observes the new field in that A/B — the natural next edit. The comment would
then attribute to the carrier a difference that belongs to the synthetic
construction.

⇒ A field addition falsifies every "nothing else differs" claim about values of
that type, and none of those claims mention the field, so no grep for the
field's name finds them. **Grep for the CLAIM** — *"and nothing else"*,
*"otherwise identical"*, *"differs only in"* — across the type's own test
population whenever a struct gains a member. It is also a stand-down clause
pointing at the wrong operand
([[hunt-the-stand-down-clause-it-lives-in-prose-no-gate-reads]]).

### A qualifier narrower than the truth invites a hunt for the divergence it implies

The same commit said an early return now yields `fusion` where it previously
yielded `default()`, *"which **in a non-test build** are the same value."* They
are the same value in **every** build: the early return fires on the same
`None` that makes the enumerator report `0`, so the field is `0` there too.
⇒ An over-narrow qualifier asserts that a divergence exists somewhere; a reader
who trusts it goes hunting for the test-build case, and there isn't one.
**Qualify to what you measured, not to what you were cautious about** — mirror
of [[an-error-in-the-safe-direction-is-a-claim-about-what-you-did-not-measure]].
This sentence lived only in the commit message, so "it will be swept when
someone next edits that comment" had no comment to edit; extracted to
[[a-precise-fact-can-live-in-an-artifact-its-reader-never-opens]].

### Count the READ side of a write/read asymmetry first

The opening hypothesis was that recorded-zero versus unrecorded-zero recurred
one layer down: eleven construction sites of the type, exactly one of which
sets the field. **Refuted by counting readers**: one production reader, taking
the value from the one setter. A field with eleven writers and one reader is
bounded by the reader; a field with one writer and eleven readers is not. The
writer census is the cheaper grep and the wrong one. Say the refutation.

## 3. Widening the cfg to a feature changes the guarantee's kind, not its wording

**`0902e62f`** widened the same field from `#[cfg(test)]` to
`#[cfg(any(test, feature = "r3-4b-observation"))]`. The disclosure was
unchanged — *no production code compares two fusion plans, and we grepped* —
and still true. **What enforces it is not.**

| before | after |
|---|---|
| the field **does not exist** in a non-test build | the field exists whenever a feature is on |
| the divergence is **structurally impossible** in production | the divergence is absent because **no `Cargo.toml` enables it** |
| type-system guarantee | **configuration fact** |

⇒ **When a `cfg` widens, do not re-ask "is the claim still true" — ask "what
enforces it now."** The first question returns *yes* in every case that
matters. Feature activation is mutable external state (COORDINATION §7a): it
changes without touching any file that describes the invariant, and nothing
goes red when it does. A guarantee that has migrated into the dependency graph
needs its condition written down and a command that tests it:
`cargo tree -e features,no-dev` resolves without building.

**A precedent in the same workspace beats an argument about what someone might
do.** *"Default-off, therefore inert"* invites a reply about hypotheticals; go
find whether the route has already been taken for something else. Here
`crates/ken-cli/Cargo.toml` already depends on `ken-runtime` with
`features = ["px8-ds-test-support"]`, turning on the sibling
`cfg(any(test, feature = ...))` family (13 sites) from another crate.
**Correction (verified 2026-09-26):** the original report called this a normal
`[dependencies]` edge compiled into the shipped CLI; at `0902e62f` and on
current `main` it is a `[dev-dependencies]` edge, so under resolver 2 it
reaches every workspace test build (all of CI) but not the shipped binary.
Read the manifest section before citing an edge's reach. The precedent is the
evidence and costs one grep. Say
*"the precedent is the evidence; I am not claiming intent"* out loud, or a
factual observation reads as an accusation. Cross-crate gate coupling is
[[a-feature-gate-pairs-hazard-is-the-set-of-edits-that-desynchronize-it]].

## Reviewing practice from instance 1

- **Read a verdict that mentions you, and route the result to your own edge.**
  A review verdict is not a merge notification, and the reviewer said nothing
  was owed; reading it was still right, because a code shape in an approved
  candidate is cheaper to raise before the merge. Filing to the Steward rather
  than replying in the review thread keeps an observer out of a ring's gate.
  State explicitly that you are not gating, because a finding arriving at an
  approved candidate reads as an objection unless it says otherwise.
- **Say when someone else's correction discharges your own open premise.** The
  premise *"at most one parent has a given occurrence at position zero"* sat on
  the read-but-not-fired list; the Architect grounded it structurally (every
  visit mints a fresh append-only identity, and origins and nodes share one
  identity space). Record that it is discharged and by whom, or the next pass
  re-raises a question that has an answer. The Architect's reframing is the
  durable part: the uniqueness error is not the source of the independence, it
  is the **runtime pin on the structural invariant** the check rests on — which
  is also what would notice if the plan ever became a DAG.
