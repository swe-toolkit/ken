---
id: LANG-KENFMT-SELECTIVE-IMPORT-WRAP
title: "kenfmt: breakable selective-import item lists — teach the layout engine to wrap a selective-import item list at CANONICAL_WIDTH so a wide import renders width-conformant, with byte-idempotence and exact parse/token-shape preservation. Pure layout; zero parser/import-relation/kernel change. The reusable predecessor the Tier-C/D/E import migration needs (every multi-item dictionary import hits the 96-col wall)."
status: merged
owner: language
size: S
gate: none
tier: T1
depends_on: []
blocks: [CAT-MIGRATE-EC-FUNCTOR-IMPORT]
github: null
origin: "Steward, 2026-09-05, minting the language-lane predecessor the Architect ruled for the CAT-MIGRATE-EC-FUNCTOR-IMPORT formatter-vs-width hard-stop (Architect evt_3q0kf5cfq857e, grounded @c9b38b4a2; foundation hard-stop evt_6agemmt7th7az/evt_kn48hry8v102). EC's 14-item LawfulFunctors selective import renders as one 140-col line because kenfmt has no break-point at import commas; format_ken discards source whitespace and canonicalizes any hand-reflow back to that line, which `ken fmt --check` accepts but the whole-catalog width gate (kenfmt_b3_layout ac7, CANONICAL_WIDTH=96) rejects. The Architect REFUSED relaxing the width invariant and REFUSED splitting the declaration (the 14 items are wire-canonical/required; a split trips the approved duplicate-module uniqueness control and is not expressible). The ruled fix is this formatter layout capability. Operator authorized running it now on the idle language ring (2026-09-05). Steward-filed per COORDINATION §2; scope/sequencing is the Steward's call per the Architect ruling."
---

> # RELEASED 2026-09-05 to the language ring (idle off LANG-ROOTS). Lane-2
> # LANGUAGE ring, run now to unblock lane-3 (operator ruling 2026-09-05). It is
> # EC-FUNCTOR-IMPORT's predecessor (P-shape, like the roots-loader was) AND a
> # reusable capability the whole Tier-C/D/E dictionary-import migration needs.
> # Base = current main. Architect is a REQUIRED reviewer.

## Why this exists (the mechanism, Architect evt_3q0kf5cfq857e)

An import declaration has no layout production: `print_decl` falls to `_ =>
print_span` = `doc_from_tokens(Soft)` = `grouped_token_slice` (layout.rs:749).
That path segments a decl only at top-level parens and renders the
parenthesized item list with a hard `Doc::text(" ")` at every comma-to-item
boundary — `soft_break_between` emits a breakable `Doc::line()` only when
`atom_can_end(left) && atom_can_start(right)`, and `Comma` is in NEITHER
enumeration, so there is no break-point inside the list. `format_ken` discards
source whitespace and reconstructs from tokens, so any hand-reflow canonicalizes
back to the single 140-col line. `CANONICAL_WIDTH = 96` (layout.rs:12). The
constraint is genuinely new: the widest LANDED catalog import list is 86
columns; EC's LawfulFunctors list at 140 is the first to exceed 96, so this
wrapping gap was never exercised until now.

## Objective

Teach `grouped_token_slice`'s parenthesized-list rendering to break a
selective-import item list: emit `Doc::line()` between comma-separated import
items inside a nested `fit_group` so the list wraps at `CANONICAL_WIDTH`. The
canonical broken shape (one item per line, or fill-to-width) is THIS lane's
detailed design call — pick one, justify it, and make it idempotent. Pure
layout: zero change to the parser, the import relation, or the kernel.

## Acceptance criteria, each with its control

- **AC-IMPORT-WRAP.** A selective-import item list whose one-line render exceeds
  `CANONICAL_WIDTH` wraps at import-item boundaries so `format_ken`'s output is
  `<= 96` columns on every line. Control: a mutation removing the inserted
  break-point restores the >96-col line and REDS.
- **AC-IDEMPOTENT.** `format_ken` is byte-idempotent on the wrapped form
  (`ken fmt --check` accepts its own output; a second format pass is a no-op).
  Control: a second-pass diff over the wrapped output is non-empty under a
  mutation that makes the break-shape non-canonical, REDS.
- **AC-PARSE-PRESERVED.** The wrapped form parses to the identical token stream
  and import inventory (membership + order) as the unwrapped form. Control:
  parse both, assert identical parsed import inventory; a mutation that drops or
  reorders an item REDS.
- **AC-EC-AC7-GREEN.** The exact `kenfmt_b3_layout::ac7_whole_catalog_is_parse
  _preserved_idempotent_and_width_bounded` passes with EC's 14-item
  LawfulFunctors selective import present (the empirical unblock — reproduce it
  from the void EC candidate 1f439178's import block). Control: reverting the
  layout fix REDS ac7 on that import at 140 columns.
- **AC-NARROW-IMPORTS-UNCHANGED.** Import lists already within width (every
  landed catalog import, `<= 86` cols) render BYTE-IDENTICALLY — no gratuitous
  re-wrapping of lists that already fit. Control: a differential of `format_ken`
  over the landed catalog imports shows byte-identical output before/after.
- **AC-ZERO-PARSER-RELATION-KERNEL.** No parser change, no import-relation
  change, kernel tree stays `51d04bba…b58b15a8`. Control: `git diff` confines the
  change to the layout engine; kernel tree hash unchanged.
- **AC-NO-REGRESSION.** Re-run the affected ken-elaborator layout/kenfmt suites
  plus the whole-catalog fmt idempotency, scoped by changed PATHS via
  `scripts/ken-cargo`, never `--workspace`. Workspace-green is CI's verdict.

## Gate, reviewer, sequencing, TCB

`gate: none` — zero-TCB (pure layout engine; kenfmt is not the trusted kernel;
kernel tree stays `51d04bba…b58b15a8`; no parser/import-relation/ABI/spec/
conformance delta). Reviewer: **Architect** (REQUIRED — it grounded the
mechanism and is the required reviewer on both this WP and the EC respin) +
**language-qa** + CI on the exact SHA, then Steward M1-M4 -> lieutenant. Lane-2
LANGUAGE ring. This is the PREDECESSOR of `CAT-MIGRATE-EC-FUNCTOR-IMPORT`
(repointed onto this node) and a reusable capability for the whole Tier-C/D/E
dictionary-import migration. On landing, EC re-verifies: the import RELATION is
unchanged (the Architect's design/soundness approval evt_5g66nsdc9s024 carries in
substance) and now renders width-conformant; the EC respin SHA gets a FRESH QA +
Architect vote-of-record (no auto-carry).

## Relationship to LANG-MOD-KENFMT-DECL-LAYOUT (dedup, not duplicate)

[[LANG-MOD-KENFMT-DECL-LAYOUT]] (draft, Component-B campaign tail, scheduled by
the operator 2026-08-21 AFTER LANG-MOD-CATALOG-COMPLETENESS) records the SAME
formatter gap from the same diagnosis: `print_decl` has no `ImportDecl` /
`ExportDecl` arm and falls to the generic `_ => print_span` fallback, which has
no break-point at name-list commas. This node is the PARENTHESIZED
selective-import slice of that gap, pulled forward under the operator's
2026-09-05 "run it now" ruling to unblock the foundation lane-3 EC hard-stop —
not a competing duplicate. The two are complementary: DECL-LAYOUT additionally
covers the NON-parenthesized `export A, B, C` / re-export name lists and carries
the Component-B recombination trigger. Whether this node's `grouped_token_slice`
fix generically subsumes the ExportDecl half (both surfaces share the same
fallback path) or only the parenthesized selective-import list is an Architect
subsumption call to make ON LANDING; until then DECL-LAYOUT stays open for its
remaining (export/re-export + recombination) scope, and the Steward reconciles
it — narrow or close — once the subsumption is measured, not before.
