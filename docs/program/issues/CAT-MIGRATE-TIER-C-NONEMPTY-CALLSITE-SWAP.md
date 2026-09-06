---
id: CAT-MIGRATE-TIER-C-NONEMPTY-CALLSITE-SWAP
title: "Tier-C staging P2: swap the three raw NonEmptyCons call sites (Schema.ken.md:123,:130; ArgParse.ken.md:252) to nonempty_cons, still ambient. After this ZERO external consumer names a raw NonEmpty/Validation constructor, which is what makes the P3 abstract flip safe (no window where a live consumer names a hidden ctor)."
status: draft
owner: foundation
size: S
gate: none
tier: T2
depends_on: [CAT-MIGRATE-TIER-C-NONEMPTY-SMART-CTORS]
blocks: [CAT-MIGRATE-TIER-C-NONEMPTY-VALIDATION]
github: null
origin: "Steward cut 2026-09-06, second step of the Architect's 3-step additive staging DAG (evt_5fxtzhk104q96) for the held Tier-C {NonEmpty, Validation} abstract-export migration. Once P1 (CAT-MIGRATE-TIER-C-NONEMPTY-SMART-CTORS) has added the smart constructors, the ambient raw-ctor call sites can swap to nonempty_cons with identical denotation, still under ambient loading. This removes the last external raw-ctor consumer BEFORE the P3 flip hides the raw ctor, satisfying the only atomicity invariant the Architect identified: no window where a live consumer names a hidden ctor. Held until P1 lands; the Steward flips it ready then."
---

> # HELD 2026-09-06 pending P1. Step P2 of the Architect's 3-step additive staging
> # (evt_5fxtzhk104q96). ADDITIVE and AMBIENT-SAFE: nonempty_cons resolves in the
> # flat ambient namespace exactly like the raw ctor did, identical denotation, so
> # every harness stays green. The Steward flips this draft->ready and releases it
> # once CAT-MIGRATE-TIER-C-NONEMPTY-SMART-CTORS lands. Base = the P1-landed main
> # (re-measure the exact call-site anchors at cut -- lines drift). Seat foundation
> # gpt-5.6-terra/medium (T2) correct.

## What this is

The second additive step of the Tier-C scaffold retirement. With the smart
constructors in place (P1), swap the three remaining raw-`NonEmptyCons` call
sites in the ambient Application-tier consumers to the smart constructor. Still
ambient -- no module boundary, no imports. Its purpose is to make the P3 flip
safe: after P2, no source outside NonEmpty's defining region names a raw
constructor, so hiding the raw ctor at P3 breaks nothing.

## Fixed inputs (census evt_6s25z56pjzvcd + Architect evt_5fxtzhk104q96; re-measure at cut)

Exactly three raw `NonEmptyCons` call sites outside NonEmpty/Validation:
- `Application.Input.Schema` (Schema.ken.md:123 and :130)
- `Application.CommandLine.ArgParse` (ArgParse.ken.md:252)

No other catalog/examples source names raw `NonEmptyCons`; Invalid/Valid have no
external raw consumers. Forge and Configuration.Decoder name the Validation/
NonEmpty TYPES only (no raw ctor) -- they are P3's import concern, NOT this node.

## Deliverables

- **D1 -- swap the three call sites** `NonEmptyCons a x rest` -> `nonempty_cons a x
  rest` at Schema.ken.md:123/:130 and ArgParse.ken.md:252. Identical denotation.
  Still ambient: add NO import lines, NO module boundary. Re-measure the exact
  lines at cut.

## Acceptance criteria

- **AC-SWAP.** Exactly those three call sites change from the raw ctor to
  `nonempty_cons`; the elaborated/reduced result is unchanged (control: the
  Schema/ArgParse checked examples/harnesses produce the same values as before).
- **AC-NO-RAW-CONSUMER (the P3-enabling control).** After this node, a census
  over catalog/ + examples/ finds NO raw `NonEmptyCons` reference outside
  NonEmpty's own defining module. This is the invariant that makes P3 safe; assert
  it as a behavioral/grep control the node carries.
- **AC-NO-REGRESSION.** Full `-p ken-elaborator` green in CI; cc1/cc7/cc8 remain
  green (still ambient, no harness change). Targeted via `scripts/ken-cargo`,
  never `--workspace`.

## Gate, reviewer, sequencing

`gate: none`. Reviewer per candidate: **Architect** (additive/mechanical) +
**Foundation QA** + CI -> Steward M1-M4 -> lieutenant. Second of the 3-step DAG;
depends_on [[CAT-MIGRATE-TIER-C-NONEMPTY-SMART-CTORS]], blocks the P3 flip
[[CAT-MIGRATE-TIER-C-NONEMPTY-VALIDATION]].
