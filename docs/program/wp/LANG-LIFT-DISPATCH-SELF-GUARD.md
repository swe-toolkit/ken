# LANG-LIFT-DISPATCH-SELF-GUARD — make `check_match_with_lift` self-guarding

**Owner:** Team Language (`language-leader` + `language-implementer` +
`language-qa`). **Branch:** `wp/LANG-LIFT-DISPATCH-SELF-GUARD`. **Size:** S.
**Tier:** T1. **Risk:** low — one guarded read on a currently-unreachable path;
adds a rejection, changes no existing accept.

**Authority + full design:** the issue node
[[LANG-LIFT-DISPATCH-SELF-GUARD]] (`docs/program/issues/`). It is shovel-ready
and is the authoritative spec: read it, not a paraphrase. It carries the
measurement (the three dispatch caller counts with line numbers), the exact
one-line vehicle (`ensure_arm_ctors_belong_to_family(cx, arms, host,
host.id)?;` at the top of `check_match_with_lift`, before the `binding.support`
unwrap), deliverables D1/D2, acceptance criteria AC-1..AC-5, not-in-scope, and
the recorded general lesson. This frame adds only the fixed-input base and the
contention check.

## Fixed inputs (measured at `origin/main = df4c56817`; RE-MEASURE at your cut)

- Base = current `origin/main` `df4c56817` (LANG-MATCH-GUARDS closed). The issue
  node's caller-count table (`check_match_with_lift:1491` single caller `:2042`;
  `infer_match:8460` two callers `:1222`/`:3580`; `check_match_dependent:1969`
  guard `:2013`) was derived on an earlier `main` — **D1 re-derives the three
  counts at your base and reports them; a moved count is a finding before it is a
  repair** (that is D1's own instruction).
- The vehicle needs no plumbing: the signature already carries `arms:
  &[RMatchArm]` and `host: &InductiveDecl`, and `host.id` is already used at
  `:1517`.

## Deliverables and acceptance

Exactly D1/D2 and AC-1..AC-5 as stated in the issue node. In particular AC-2's
control must exercise the LIFTED path specifically (the existing foreign-arm
tests reach the dispatch through `check_match_dependent` and pass with or
without the guard, so they do not cover this node), demonstrate the red by
mutation, and report the diagnostic. AC-3's direction: this ADDS a rejection on
a currently-unreachable path; if any landed test changes behaviour, STOP — the
premise was wrong and that is a bigger finding than the repair.

## Contention

None. Single file `crates/ken-elaborator/src/elab.rs` plus one focused test;
path-disjoint from the runtime lane (ABI-A2, `crates/ken-runtime`/`ken-host`)
and the foundation lane (`catalog/`). No shared-symbol or shared-fixture
overlap. Architect is a required reviewer alongside language-QA + CI (the node
is a soundness-adjacent reachability argument, per the Architect's request that
it not be discharged with a comment).

## Sizing / tier

Size S, one-hour turn: a one-line guard plus a lifted-path control. Tier T1 —
the diff is one line but the review turns on the reachability argument (what the
guarded code sees, and from how many entry points), not on the line.
