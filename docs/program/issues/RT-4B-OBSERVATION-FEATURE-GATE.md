---
id: RT-4B-OBSERVATION-FEATURE-GATE
title: "Re-gate the existing D2f observation behind an off-by-default Runtime feature with a doc-hidden feature-scoped accessor, and prove the feature inert by comparing artifacts from TWO COMPILATIONS -- not a runtime toggle inside one, which is what the landed switch already proves and is a different claim; this is the increment the C2 answer made conditional, and it is what unblocks re-pointing the reach node at the real witness"
status: ready
owner: runtime
size: M
gate: none
depends_on: []
blocks: [RT-4B-UNIQUENESS-GATE-REACH]
github: null
origin: Architect ruling evt_4a1pf1jfmdemd, holding that a default-off feature is one of the two mechanisms his original 4b ruling named and is therefore the envelope used as written, with five conditions he will review against. Conditional on RT-4B-C2-REACHABILITY's row-two answer (evt_29jjd4rrytnex, read at 9f22d70c). Framed by the Steward 2026-08-13.
---

## What this is

**The increment the C2 answer made conditional.** `RT-4B-C2-REACHABILITY`
established that the reaching build differs from production: the D2f record,
storage, note/take and sole write are all `#[cfg(test)]`, and C2 is a
`ken-elaborator` integration test whose Runtime dependency carries default
features only, so the observation write does not exist in the Runtime unit C2
links.

**Re-pointing the reach node at C2 is conditional on proving the enabling
mechanism inert. That proof IS this node, not a footnote to it.**

## The mechanism is settled — do not re-open it

An **isolated, off-by-default Runtime observation feature**, extending the
existing record, storage, note/take and sole write **rather than beside them**,
exposing a feature-gated doc-hidden scoped accessor.

Rejected, with reasons recorded so they are not re-litigated:

- **`px8-ds-test-support`** — it carries unrelated mutation and census support,
  which the Architect named as *a second observer wearing a shared name*.
- **Neutralizing or splitting that feature** — largest blast radius.

## The condition most likely to be skipped

**The identity proof must be across two COMPILATIONS, not a runtime toggle
inside one.** The landed increment proves identity with the thread-local
`D2F_GATE_OBSERVATION_ENABLED` switch flipped within a single build. That proves
**the recording is inert**; it does not prove **the feature is inert**, because
a feature is a compile-time property.

Cargo also unifies features across a build graph, so a `ken-elaborator`
dev-dependency enabling `ken-runtime/<feature>` compiles `ken-runtime` with it
for everything in that build. **You cannot produce a feature-on and a
feature-off artifact in one compilation even if you wanted to.**

## Not this node

- Counting anything, attributing any enumerator exit, or re-pointing the reach
  node. Re-pointing is what this unblocks; it is not this.
- Any second observer, any always-compiled accessor, any change to what the
  plan contains.
- Enumeration, classifier, checker, marker, fusion-candidate, representation,
  ledger or closure-boundary repair. Gates 5 and 6 held; production unarmed.
