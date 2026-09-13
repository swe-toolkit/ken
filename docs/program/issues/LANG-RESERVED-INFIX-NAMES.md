---
id: LANG-RESERVED-INFIX-NAMES
title: "A0 of the reserved-infix-glyph objective: admit the six currently-reserved glyph tokens (Le ≤/<=, Ge ≥/>=, Ne ≠//=, And ∧//\\, Or ∨/\\/, Member ∈) as ordinary symbolic GLOBAL names and infix/fixity targets through ONE shared parser token-to-canonical-name view reused at every consumer site, entering the existing neutral-spine + GlobalId-keyed fixity reassociation as ordinary RApp; NO standard semantic binding, NO Membership class, NO `!=`/`in` alias -- syntax/naming only"
status: draft
owner: language
size: M
gate: none
depends_on: [SPEC-RESERVED-INFIX-NAMES]
blocks: [LANG-STANDARD-INFIX-CALL-COMPLETION]
github: null
tier: T1
origin: "Steward cut 2026-09-13 from the Architect final-A decomposition (evt_784ge2nq65dfy, grounding corrections evt_7z907rns84e6n), grounded at main 4fdd4f0ad; Architect probe SHA e0434d58 (temporary token-normalization probe of five representative definitions, all six names reached). Operator objective (Pat, 2026-09-12): full reserved infix glyphs. status draft: HELD until SPEC-RESERVED-INFIX-NAMES lands, then the Steward flips it ready and releases to the language ring. Architect is the required reviewer. IN-LANE: elaborator/parser surface, no new kernel mechanism/trust-root/TCB. Re-measure the seven consumer sites and lexer/formatter anchors at the cut."
---

> # HELD until [[SPEC-RESERVED-INFIX-NAMES]] lands. Do NOT begin source edits.
>
> Draft, `depends_on` the spec amendment. The Steward flips it `ready` and
> releases to the language ring once the spec node merges. The design below is the
> Architect's A0 ruling (evt_784ge2nq65dfy); full detail and the probe are in that
> event. Re-measure the parser/lexer/formatter anchors at the cut.

## What this is (Architect A0)

The first, syntax-only, releasable node of the reserved-infix-glyph objective.
It admits the six currently-reserved glyph tokens as ordinary symbolic GLOBAL
names and infix/fixity targets. It does NOT give them any standard meaning: a
concrete user-defined `≤` already has its chosen meaning once the token reaches
ordinary application; generic Ord dispatch and the standard bindings are A1
([[LANG-STANDARD-INFIX-CALL-COMPLETION]]). Keep those deliverables separate.

The six admitted alias pairs (per the spec amendment): `Le: ≤/<=`, `Ge: ≥/>=`,
`Ne: ≠//=`, `And: ∧//\`, `Or: ∨/\/`, `Member: ∈` (glyph-only). No `!=` alias,
no `in` alias, no standard Membership class or default membership meaning; `∈`
may name a client-defined ordinary function.

## Implementation (Architect A0)

- Preserve the dedicated lexer tokens and the formatter canonicalization. Give
  the parser ONE bounded token-to-canonical-operator-name view, and reuse it at
  EVERY consumer, not just the two infix/fixity loops:
  1. the `contains_user_operator` fast-path decision;
  2. `parse_mixed_infix_expr`;
  3. `parse_fixity_decl`;
  4. global declaration / name positions;
  5. prefix atom application;
  6. qualified suffixes;
  7. selective-import / rename / export name positions.
  Changing only the infix/fixity loops misses real consumers.
- Do NOT extend local binders, constructor identifiers, module aliases,
  record-field grammar, or the generic Unicode operator alphabet.
- The admitted heads enter the existing neutral spine and post-resolution
  GlobalId-keyed reassociation, lowering to ordinary `RApp`. Keep existing
  duplicate-fixity / collision / privacy behavior and the default `infixl 9`
  when the actual binding has no fixity declaration. Do NOT assign precedence
  from glyph spelling.
- The five built-in `BinOp`s, fixed arithmetic/`==` association, and the
  no-spine fast paths remain unchanged. Preserve `< > / %` as user operators;
  `/=` keeps its existing token/formatter identity and gains the newly specified
  name/application uses.

A0's reaching semantics are ordinary explicit definitions, NOT an implicit
prelude claim. The Architect probed (at e0434d58, temporary token normalization)
that definitions of this shape check and that `≤`/`<=` infix bodies equal the
prefix body's kernel term, `infix 4 ≤` rejects a chained use with
`NonAssociativeInfix`, and all six names were reached including a client-defined
`∈`. Production must use the shared parser view, NOT a lexer rewrite.

## Acceptance criteria (Architect A0)

- Each admitted alias pair yields byte-identical admitted terms (`≤` and `<=`).
- For every one of the six names: an actual declaration plus prefix, infix, and
  fixity use — including a source file with NO generic `Operator` token, to
  prove the shared view (not the generic path) carries them.
- Qualified / selective / renamed imported bindings work, and defining-GlobalId
  fixity carries through resolution.
- Left / right / non-associative fixity controls with distinguishable operands;
  exact rejected precedence / collision / privacy cases each paired with a
  positive client.
- Formatter round-trip / canonical glyph, with comments and literals protected.
- Existing arithmetic/`==`, `< > / %`, `let … in`, lambda/arrow, truncation
  bars, annotation, and reserved punctuation are unchanged.
- Mutating an alias identity, omitting the fast-path trigger or any one of the
  seven consumers, or bypassing the resolved fixity must redden its
  corresponding observation. No broad corpus rewrite and no default semantic
  binding is smuggled in.
- No-regression in CI (`COORDINATION §12`); targeted locally `-p ken-elaborator`.

## Not this node

- The standard operator meanings + use-site dictionary/argument completion —
  [[LANG-STANDARD-INFIX-CALL-COMPLETION]] (A1, draft).
- Membership's class/carrier design — the parallel B track.
- `!=`/`in` aliases; any new BinOp variant; any prelude/default binding.

## Sizing / tier

**Size M, tier T1.** Ordinary elaborator/parser work over a bounded token view,
but the review turns on the shared-view completeness argument (all seven
consumers) and the syntax-only / no-default-binding boundary. Architect required.

## Contention

Language ring, `crates/ken-elaborator/src` (lexer/parser/resolve) + acceptance
tests under `crates/ken-elaborator/tests/`. The tests are under `crates/`, so a
candidate touching them is a CODE merge -> full CI, M8/M8a Adversary. No
cross-lane contention (L1 runtime on `crates/ken-lowering`/`crates/ken-runtime`;
L3 foundation on `catalog/`). Re-measure the seven consumer sites at the cut.
