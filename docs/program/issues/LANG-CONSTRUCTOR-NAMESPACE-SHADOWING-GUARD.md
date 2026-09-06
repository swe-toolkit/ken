---
id: LANG-CONSTRUCTOR-NAMESPACE-SHADOWING-GUARD
title: "Elaborator constructor namespace: a later constructor with a spelling already bound silently replaces the earlier binding (flat global map, unqualified pattern resolution). Diagnose it instead of shadowing silently. QUEUED language debt — not on any active lane."
status: draft
owner: language
size: S
gate: none
tier: T2
depends_on: []
blocks: []
github: null
origin: "Filed by the Steward 2026-09-06. A recurring elaborator limitation hit by TWO lanes in one session: foundation TIER-C (pub data NonEmpty, evt_5h8nbwcweq87m) and runtime ABI-REVOKE-D2 (a second nullary `Revoked` across IOError + ResourceError, evt_1294jtct6rmfa). The runtime-leader named it explicitly as 'separate language debt, not D2 scope' (evt_5qns09rmgar2a). Captured so it is durable; QUEUED behind the three active lanes, NOT released. Flesh the frame at release time."
---

> # QUEUED — NOT RELEASED. Behind the three active lanes. This is the sanctioned
> # capture of a ring finding (received, queued, no lane blocked on it). Do not
> # release without an operator/Steward lane slot.

## The finding (measured, both instances at b13c3af1)

Data registration writes every constructor into one flat
`HashMap<String, GlobalId>` with an unconditional `globals.insert(c.name.clone(),
ctor_id)` (`crates/ken-elaborator/src/data.rs:111` / `:277`). A later constructor
with a spelling already bound SILENTLY REPLACES the earlier binding. Pattern
admission then resolves only `cx.globals[name]` and checks that one id belongs to
the scrutinee family (`elab.rs:12417` / `:12456`) — there is no family-indexed
alternative set and no qualified-constructor source form.

Consequence: two closed sums cannot share a constructor spelling; the second
declaration makes the first family's arms resolve to a foreign constructor
(`TypeMismatch`). This is a silent footgun — the shadowing is not diagnosed.

## Why this is the bounded fix, not the foreclosed one

Type-directed / qualified / overloaded constructor resolution (letting both
spellings coexist and disambiguating by family) is FORECLOSED — the Architect
ruled it out on the settled flat-namespace spec decision (§1), and both lanes
above took a distinct spelling instead (abstract + smart constructors for
NonEmpty; `ResourceRevoked` for the resource arm). So this node is NOT that. It is
the narrow hardening the runtime-leader named: make the silent duplicate-spelling
shadowing a DIAGNOSED error at declaration time (a clear elaborator diagnostic
naming both declarations), so the next collision fails loudly at its source
instead of surfacing as a downstream `TypeMismatch` in an unrelated family.

## Deliverables / ACs — TO BE FLESHED AT RELEASE

Sketch: detect an insert into `globals` whose spelling is already bound to a
constructor of a different family; emit a duplicate-constructor-spelling
diagnostic naming both sites; AC = a two-sum same-spelling fixture reddens with
that exact diagnostic (control: distinct spellings compile). Re-measure the
`data.rs` / `elab.rs` anchors at release — they drift.
