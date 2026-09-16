---
id: RT-D5B-LIVE-WIRING
title: "Wire the immediate-bridge realization plane into the live planning path -- one import and one assignment before response phase B -- so the classifier and derivation stop being reachable only from tests. The deliverable is two lines; the work is AC-2a, a test that goes red when the call moves ABOVE the specialization installs at construction.rs:1382-1387 rather than one showing it runs -- red by EMPTINESS, since above them continuation_units() returns empty. Two earlier forms of AC-2a named construction.rs:1406 as the bound; both are refuted in frame section 6 and :1406 is not a partition point at all, so a context-bearing fixture would not pin it. The comment's upper bound (before phase B) is AC-2b and is DEFERRED: derive_'s read set and phase B's write set are disjoint and nothing live reads the field, so no test can go red for that move until the consumer exists"
status: merged
owner: runtime
size: S
gate: none
depends_on: [RT-D5B-BRIDGE-REALIZATION-PLANE]
blocks: []
github: null
tier: T1
origin: "Steward, 2026-09-16. Slice 3 and the last of the PR #3676 re-cut, after RT-D5B-IMMEDIATE-BRIDGE-CLASSIFIER (slice 1, landed 10321a158) and RT-D5B-BRIDGE-REALIZATION-PLANE (slice 2, landed 49e5ebfbe). Frame at docs/program/wp/RT-D5B-LIVE-WIRING.md, cut from main 49e5ebfbe. Absorbs the slice-3 carry-forward drafted as a section 5a block on the slice-2 frame while slice 2 was publishing (Architect evt_pyt8pmshvq9z, runtime-implementer evt_yht7xmkpxhsy, Architect evt_3k27pn82x36es, Steward evt_a6zk62tzb9mg) -- that block's rationale was that this frame did not exist, so it is superseded rather than landed. Steward-filed per COORDINATION section 2."
---

> ## MERGED 2026-09-16 at `67684fa5d9e960ebbdfba92f1a0d664115d73c25`
>
> **Verified by blob, not by ancestry** — the publisher squashes, so a routed
> commit is an ancestor of nothing. All three touched paths are byte-identical
> between the approved candidate `1bcc359dc33148d42900ed6ca5e8779a12936d02`
> and `main` (`static_transition.rs`, `static_transition/construction.rs`,
> `static_transition/immediate_bridge.rs`), landed via PR #3752 with full-mode
> CI green (all 8 test shards, `rt_parity_native` 1-6, z3, conformance).
>
> This closes the PR #3676 re-cut arc: slice 1
> (`RT-D5B-IMMEDIATE-BRIDGE-CLASSIFIER`, `10321a158`), slice 2
> (`RT-D5B-BRIDGE-REALIZATION-PLANE`, `49e5ebfbe`), and this slice 3 are all on
> `main`. Adversary verdict: NO DEFECT (evt_4mfzx89c4ncdj) — the wired plane is
> correctly inert in production (no consumer yet; that is
> `RT-D5B-POSTCALL-REFUSAL-MECHANISM`'s subject) and the new fallible call is
> fail-closed only on planner self-inconsistency, never on a valid program.

Read the frame: `docs/program/wp/RT-D5B-LIVE-WIRING.md`.

## What this slice ends

Slices 1 and 2 were each **defined by the plane having no production caller**.
That property is why their `AC-3` asserted the four functions **are** in
`ken-runtime`'s `never used` diagnostic, and it is what this slice deliberately
ends. **The same instrument therefore inverts here**, and `AC-3` is the
inverted reading.

## The two things most likely to be got wrong

**1. `AC-2a` is not "show the wired call works".** A candidate showing the
wired call produces the expected plan has tested **that it runs**, not **that
it runs there**. The AC asks for a test that goes red when the call is moved
**above the specialization installs at `construction.rs:1382-1387`**. Above
them both populations the `continuations.rs:6867` guard compares are empty, it
passes `0 == 0`, `continuation_units()` returns empty, and the derived plane is
empty — **red by emptiness, not by error.**

**`construction.rs:1406` is NOT the bound, and two separate rationales for
calling it one are refuted in frame §6.** `:1406` writes only `abi.context_*`
fields; `derive_`'s path reads only `abi.continuation_*` ones, and never calls
the `continuation_contexts()` function where the `:6943` context guard lives.
**A context-bearing fixture would be green above `:1406` too**, so do not
commission one to pin it — that decision is closed, not deferred.

**`AC-2b` — the "before phase B" half — is DEFERRED and is not yours.**
`derive_`'s read set and phase B's write set are disjoint, and nothing on the
live path reads `immediate_bridge_realizations`, so moving the call later
changes no observable and **no test can go red for it.** That is the scope
decision working, not a gap: the consumer that would make the upper bound
matter is `CheckedIhPostCallConsumer`, which this slice excludes. **Do not
build a test-only ordering probe to satisfy its words** — a hook recording that
the call happened before phase B cannot fail for the reason anyone cares about.

**One measurement IS yours:** `with_d5b_hs10_bridge_plan_mutation`
(`immediate_bridge.rs:559`). If that feature-gated hook can perturb the plan
between the wiring point and phase B, `AC-2b` may be constructible after all
and is due now. Report what it reaches either way.

**2. `AC-3` asserts on the diagnostic's NAMED ITEMS, never on the warning
count.** rustc groups several never-used items into one diagnostic, so an item
becoming live — the exact event the control exists to catch — changes the text
and leaves the count unmoved:

    without the substitution   methods `producer_owner` and `identity`
                               are never used
    with it                    method `producer_owner` is never used
                               106 warnings EITHER WAY

`AC-9`'s warning delta is a **disclosure** requirement and stays a count. The
count is the wrong shape only for `AC-3`. Do not strip `AC-9`'s figure.

## The expected `AC-3` answer, stated so a wrong one is visible

Read from the function bodies at `main`, not from a caller grep:

    publish_   ->  derive_, relation_from_rows
    build_     ->  derive_, relation_from_rows
    validate_  ->  build_

Wiring `publish_` makes **`publish_`, `derive_`, `relation_from_rows`**
reachable. **`build_` and `validate_` stay dead**, because `validate_` has only
test callers and `build_`'s only non-test caller is `validate_`.

**An earlier draft of this control proposed re-arming when the diagnostic stops
naming `build_` and `validate_` — the two functions this slice does NOT
move.** It is recorded because the same mistake is available twice: a still
earlier form keyed on *"has a call site outside `cfg(test)`"*, which was
already true of `build_` at slice 2's tip (`:678`, inside `validate_`, itself
unreachable). **A call site outside `cfg(test)` is a local fact; reachability
is a global one, and they differ precisely on an intra-cluster edge — the only
kind an unwired module has.**

⇒ Both failures came from naming functions instead of naming the property, so
`AC-3` asserts the named **set changes** and then reads which names left. If
the observed set differs from the expectation above, **that is a finding about
the call graph, reported rather than reconciled away.**

## The re-export decision is probably moot, and `AC-7` wants that said either way

`publish_immediate_bridge_realization_plan` is `pub(super)` and
`construction.rs` is a sibling module inside `static_transition`, so
`use super::immediate_bridge::...` resolves with **no re-export on
`static_transition`'s surface** — which is how the reference's own
`construction.rs` import is spelled.

`static_transition.rs:86-89` carries a comment slice 2 authored, whose expiry
condition is *"ahead of the slice that uses it"*. **This is that slice.** If no
re-export is needed, say so and delete the comment; if one is, state the
visibility chosen and why. **Do not author a re-export because the reference
has one** — the reference's serves hunks this slice excludes.

## Scope, and the mechanism that is NOT a prerequisite

Out of scope: the whole `CheckedIhPostCallConsumer` mechanism,
`InlineBridgeNoCall`, and the `owns_seat` rewrite.

`CheckedIhPostCallConsumer` has **zero hits on `main`** and six files at the
reference, and the reference's `construction.rs` wiring hunk is entangled with
it — so a verbatim lift of that hunk does not compile and is not repairable by
renaming. **That mechanism is `RT-D5B-POSTCALL-REFUSAL-MECHANISM`'s subject**
(`ready`, runtime, M, T1), an open investigation carrying three refuted
mechanisms and no candidate.

**It is not a dependency of this node.** This slice is cut specifically so that
it is not, which is the entire reason the scope is two lines.
