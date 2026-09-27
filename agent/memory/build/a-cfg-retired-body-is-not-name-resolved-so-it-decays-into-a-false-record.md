---
name: a-cfg-retired-body-is-not-name-resolved-so-it-decays-into-a-false-record
description: Retiring a control with `#[cfg(any())]` keeps its body readable as a record, but cfg-stripping precedes name resolution, so the body is never checked again and stops describing the tree, and any pin or caveat written about it guards a computation in no build. Probe with `compile_error!` plus a positive control, flip the cfg back and run it, and order the retirement against the guard with `git merge-base --is-ancestor`.
metadata:
  type: feedback
---

# A cfg-retired body is not name-resolved, so it decays into a false record

This repo retires source-text oracles **by cfg, in place**
(`#[cfg(any())]`, operator ruling; see
[[a-source-text-oracle-defect-is-non-actionable-so-report-the-broken-promise]]),
so the retired body stays readable and keeps looking like a live test.
Retire-in-place is chosen *over deletion* so the body stays a record of the
property. That presumes it keeps describing the tree. It does not:
**cfg-stripping happens before name resolution and before macro expansion**, so
a retired body is neither type-checked nor name-resolved. The compiler that
would have kept it honest was removed by the same act that retired it.

Two consequences, each measured:

1. **The body itself decays** into a false record (instance 1).
2. **Guards written about the body go on being enforced** — reddening CI,
   costing review rounds — while the computation they protect is in no build
   (instance 2).

## Instances

- **2026-08-17, `ca639b5ef`** (`RT-BRANCHED-SCRUTINEE-UNIT-BODY-PORT` D2). A
  candidate reddened CI on a source-text pin, forcing a second review round: a
  recut commit plus four seats independently re-measuring the same number. The
  pin's stated purpose was to keep a caveat honest about a census's call
  population. That census carried `#[cfg(any())]` and was in no build.
  Retirement `6a451b456` (2026-07-29) put the cfg on the census; the guard node
  `be25ea6a2` (2026-08-17) landed the count-keyed pin 19 days later, with the
  retirement already an ancestor. Its own issue doc still argued the present
  tense, *"the census is still sound in the direction that matters"*, about a
  computation that could not run.
- **2026-08-18, `b430d73e0`**, the pin-disposal deliverable of the node the
  first instance opened. Its fork was revive / retire / re-key, and it rejected
  revive because the census is compiled out. Sound, and it understates the
  case: **revive was not available.** Flipping `#[cfg(any())]` to `#[test]` on
  every retired census in the file and running them: **3 of 3 fail** on their
  first assertion.

  | retired census | measured vs asserted |
  |---|---|
  | join-helper caller population | `fn merge_branch_value(` definitions **0** vs 1 |
  | planner exported-surface pin | pinned list 32 entries, current one many times that |
  | `lower_expr` call population | tokens **70** vs 65 |

  An unrelated node deleted `fn merge_branch_value` from the production module
  on 2026-08-17, 19 days after its census was retired, and nothing could
  notice. The identifier then existed nowhere in `crates/` except as two string
  literals inside the retired body.

## Probe reachability with `compile_error!`, and carry a positive control

Cheapest probe for *is this item in the build*: plant `compile_error!("...")`
in its body and run `cargo check --tests -p <crate>`. A stripped item stays
silent.

**A clean compile is a disjunction** — item stripped, or file not compiled, or
you edited the wrong tree, or the check did not run. So plant the identical
macro in a **known-live item in the same file** and require it to fail:

| probe site | result | reading |
|---|---|---|
| the suspect item | compiles clean | not in the build |
| a live `#[test]` in the same file | `error: ADVERSARY-POSITIVE-CONTROL-...` | file IS compiled, probe IS live |

Same discipline as
[[a-green-mutation-does-not-tell-you-which-blindness-let-it-through]]: green
needs discrimination, not just a green.

The runtime twin is an unconditional `panic!` at the head of the mechanism plus
a whole-suite run — 923 passed / 1 failed showed that the only thing reaching a
newly-landed agreement check was its own unit test, and the deliverable's own
witness passed unchanged.

## Flip the cfg back and run it, letting the test's own instrument report

To show a retired body is false, do not grep for it — flip the cfg and run it.
libtest stops at the first assertion, so for the richest body replace its
asserts with `eprintln!` and re-run: here **four of its seven assertions were
false.** It is one extra compile, and it removes the objection that your census
criterion and the test's criterion differ.

## The ordering turns "stale" into a claim about the mechanism

"The pin is stale" is a shrug. **`git merge-base --is-ancestor <retirement>
<guard-or-deletion>`** tells "went stale" from "was born dead":

- guard authored after the retirement was already an ancestor ⇒ the guard node
  was not overtaken by a retirement; **it was authored against a subject that
  had already been removed.** Sibling of
  [[a-node-that-closes-without-discharging-inverts-the-doc-that-named-it]].
- subject deleted after its census was retired ⇒ the deletion was
  undetectable by construction.

**State the property, not the staleness**: a reader consulting the retired body
gets a three-member family whose first member does not exist. The oracle's own
defect is not the finding; the broken promise is.

## Chase the scariest assertion, then refute it out loud

The retired body's load-bearing line was *"every helper must require the
unmintable typed plan token"*, count 3, now measuring 2. Chased as a token-gate
hole, it is not one: the two live wrappers each take the token and reject a
wrong representation before delegating to a shared **untokened** callee, and
that callee has exactly two call sites in the whole of `crates/`, both wrappers.
The count fell because a helper was *deleted*, not because one lost its token.

⇒ **Put the refutation in the report.** A count that moved for a boring reason,
inside a body nobody can run, is exactly the shape a reader will re-derive as a
soundness scare. What the assertion genuinely guarded was the family's
**closure** — a third caller could be added with no token and nothing would
object — and that is a missing guard, not a defect. A gate keyed on the
wrappers is never total over a shared untokened callee, but that only bites
once a third caller exists; see
[[a-ruling-that-widens-a-shared-map-names-only-the-consumer-it-was-about]].

## Where to look

Every caveat, guard, or sentinel written about a retired body is a candidate:
`grep -n 'cfg(any())'`, then grep the retired function's identifiers for
anything still `#[test]`.

**And check the gate's population against the hazard it names.** The pin in
instance 1 counted lines whose trimmed text was exactly `#[cfg(test)]` while the
file carried six test-gating cfg spellings; the same candidate moved
`#[cfg(any(test, feature = "..."))]` from 17 to 19 lines, invisibly, and the
repair node one merge later moved it 19 to 23, again invisibly. Those figures
count the ONE exact spelling; the wider `#[cfg(any(test` family over the same
three SHAs is 23 → 25 → 29. Quote the criterion with the number: a drift figure
is meaningless without the population it was taken over. Same family as
CHECKS.md check 3 and
[[anchor-a-claim-census-to-position-and-validate-it-against-a-reference-count]].

## The remedy is a fork, and it is the ring's

Either the retired bodies are a record — then something live keeps them honest
(every identifier named inside a `#[cfg(any())]` block still resolves somewhere
in `crates/`; that would have reddened on the deletion commit) — or they are
not, and deleting them is more honest than preserving text that reads as one.
Note that the first branch is itself a repository-text check, which the
prohibited-subject rule in `qa-test-design` bars as a test oracle, so it could
only live as a tool or tripwire outside the test suite. **Both branches are
cheaper than discovering the decay when someone reaches for the revive
option.** Kin of [[deleting-a-check-has-a-text-surface-and-it-outlives-the-check]].
