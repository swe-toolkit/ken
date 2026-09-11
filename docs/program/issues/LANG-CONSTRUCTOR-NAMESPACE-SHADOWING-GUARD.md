---
id: LANG-CONSTRUCTOR-NAMESPACE-SHADOWING-GUARD
title: "Elaborator constructor namespace: a later constructor with a spelling already bound silently replaces the earlier binding (flat global map, unqualified pattern resolution). Diagnose it instead of shadowing silently. QUEUED language debt — not on any active lane."
status: merged
owner: language
size: S
gate: none
tier: T2
depends_on: []
blocks: []
github: null
origin: "Filed by the Steward 2026-09-06. A recurring elaborator limitation hit by TWO lanes in one session: foundation TIER-C (pub data NonEmpty, evt_5h8nbwcweq87m) and runtime ABI-REVOKE-D2 (a second nullary `Revoked` across IOError + ResourceError, evt_1294jtct6rmfa). The runtime-leader named it explicitly as 'separate language debt, not D2 scope' (evt_5qns09rmgar2a). Captured so it is durable; QUEUED behind the three active lanes, NOT released. Flesh the frame at release time."
---

> # MERGED 2026-09-11 (Steward) — landed at origin/main 8c6136fa3 as the
> # corpus-clean respin 33bc3a4f (rebased onto d8bbef963, was b7a9101a),
> # blob-verified 12/12 across all crates/ken-elaborator paths (data/elab/error/
> # lib/modules/prelude src + guard test + seal2 census + 4 repaired fixtures).
> # SUPERSEDED the CI-red 64462ab7; production guard byte-identical to it. The
> # deliverable landed as framed: a declaration-time DuplicateConstructorSpelling
> # diagnostic naming both sites, guarding cross-family spelling collisions in the
> # flat globals map (data.rs constructor inserts). Gates on exact b7a9101a
> # (production byte-identical, carries to the rebase): Language QA APPROVE
> # evt_7arnwck41499e (guard suite 3/3, all four repaired fixtures pass, rt_dasm
> # reframe sound), Architect reframe APPROVE evt_32yhxanfmhrzw (byte-identical
> # production, rt_dasm reframe faithful and strictly stronger, four repairs are
> # genuine collision renames preserving each test's subject). Decision
> # dec_12454ckyvy68m RESOLVED-APPROVED. Steward route evt_4ry9qdd7x9ghs. The
> # dispositioned CI-red is CLOSED: the four surfaced collisions were real latent
> # cross-family shadows (not over-fires), repaired in-WP by incidental renames
> # (lc MkUnit->MkMyUnit, v3_fo FokDerivInit->DummyFokDerivInit, surface_def
> # MkDecimalPair rename) plus the rt_dasm_d1b test reframed to assert the
> # DuplicateConstructorSpelling rejection directly (its shadowing premise is now
> # statically impossible), inventory relation preserved separately. No new node,
> # no merge-past. No TCB/spec/catalog/kernel change.
> #
> # RELEASED 2026-09-10 (Steward) — the L2 language head. LANG-CHECKED-IH-BODY-
> # VIEW-CAUSE closed (D0 dissolved, 9f91155d0) and the language ring went idle,
> # so this is the next item in the operator L2 queue (item 6). status
> # draft->active; kicking the language ring. Tier T2 (mechanical diagnostic
> # emission at declaration time), size S.
> #
> # RE-MEASURED at release (9f91155d0): the constructor insert sites are UNCHANGED
> # at crates/ken-elaborator/src/data.rs:111 and :277 (globals.insert(c.name.clone(),
> # ctor_ids[i])). The pattern-admission family-membership resolution the frame
> # cited at elab.rs:12417/:12456 has DRIFTED to elab.rs:7081 (the matches!
> # RPatKind::Ctor ... cx.globals.get(name) family check) and :15103
> # (cx.globals.get(name).expect). The ring re-measures precisely at pickup.
> #
> # DELIVERABLE (per the sketch below, now the released contract): detect an
> # insert into `globals` whose spelling is already bound to a constructor of a
> # DIFFERENT family and emit a duplicate-constructor-spelling diagnostic naming
> # BOTH declaration sites — at declaration time, so the collision fails loudly at
> # its source rather than as a downstream TypeMismatch in an unrelated family.
> # Type-directed / qualified / overloaded coexistence is FORECLOSED (Architect,
> # settled flat-namespace spec §1) — do NOT build that; this is the narrow
> # diagnose-don't-shadow hardening only. AC: a two-sum same-spelling fixture reds
> # with that exact diagnostic; control: distinct spellings compile. No TCB/spec
> # change. Reviewers: Language QA + Architect (namespace-semantics confirmation)
> # -> Steward M1-M4 -> lieutenant. Reachability of the diagnostic seam is
> # unconfirmed — hard-stop to the Steward if the insert site cannot carry the
> # duplicate check without a broader refactor.

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
