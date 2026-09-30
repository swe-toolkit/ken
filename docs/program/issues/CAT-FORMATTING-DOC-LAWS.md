---
id: CAT-FORMATTING-DOC-LAWS
title: "Prove the Formatting Doc fitting and layout laws with no new trust: a document that fits renders flat in exactly its flat width, Group and Alt choose flat exactly when their layout fits, and render_string is coherent with render under an explicit round-trip premise"
status: ready
owner: foundation
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "CATALOG-PROOF-COMPLETENESS-SURVEY row Capability/Formatting/Doc (docs/program/CATALOG-PROOF-COMPLETENESS-SURVEY.md:75): the content laws are proved, but no fitting or layout law and no render_string theorem exists. L3 proof backfill under the operator's 2026-09-13 rulings; successor to CAT-DERIVED-STRING-VIEW-LAWS. Steward-filed per COORDINATION section 2."
---

# Formatting Doc fitting and layout laws, at zero trust

## Objective

`Capability/Formatting/Doc.ken.md` §2 states the fitting rule in prose:
"`Group` uses its flat layout exactly when it fits." After this WP, checked
laws state it, with no `trusted_base()` entry.

## Settled inputs (read at `0699d6e90`)

- **`doc_flat_width`** counts each text character and each `Line` as one
  column. `Nest` does not change it, and `Alt` takes its first branch
  (`Doc.ken.md:89`). `doc_fits w d` is `leq_nat (doc_flat_width d) w`
  (`:99`).
- **`render_mode flat w i d`** (`:101`): a flat `Line` is one space, and a
  broken `Line` is a newline followed by `i` spaces. `Group` and `Alt`
  re-decide with `doc_fits` at every node, including inside flat mode.
- **Proved today** (`:167`-`:350`): `preserves_text_tokens`,
  `width_independent` and `fixed_point`. These are laws about content, not
  layout.
- **`render_string`** is `list_char_to_string (render w d)` (`:369`). It is
  the thin, proof-free String boundary. Spec 37 NFC-normalizes this
  conversion, so an unconditional String law is false (the
  `CAT-DERIVED-STRING-VIEW-LAWS` finding).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

Private theorems in `Doc.ken.md` beside the renderer. The expected set,
which AC-0 fixes, is:
- **Fitting soundness.** If `doc_fits w d = True`, then
  `length (render_mode True w i d) = doc_flat_width d` for every `i`.
- **Choice laws.** `Group` and `Alt` render flat when their layout fits
  and broken otherwise, each stated with its `doc_fits` premise.
- **String boundary.** `render_string` agrees with `render` under the
  premise `Equal (List Char) (s2l (l2s cs)) cs` for the rendered list,
  using route (b) of `CAT-DERIVED-STRING-VIEW-LAWS`.

## Acceptance

- **AC-0 (statement set; no build).**
  - Propose the exact laws, their premises, and any Nat order lemma each
    needs from the landed `Data/Numeric/Nat` packages.
  - The Architect rules the set before any proof.
- **AC-1.**
  - Every ruled law checks.
  - A typed consumer applies each law at its general proposition.
  - `trusted_base()` is unchanged.
- **AC-2 (controls).**
  - Fitting soundness with its premise deleted does not check. The
    counterexample is `Group (Concat (Text t) Line)`, with `t` a
    one-character list, at width 0 and indent 2. The group does not fit,
    so even `render_mode True` renders it broken: 4 characters against a
    flat width of 2.
  - Replacing a law with a reflexive filler reddens its consumer.

## Residual (carried, not in scope)

Nat order leq-add, leq-refl and leq-suc private duplicates in Map
(`:17119`, `:17131`), Gcd (`:328`) and Parsing (`:3239`) should import from
`Data.Numeric.Nat.Order` once this WP adds the shared bounds (Architect AC-0
ruling `evt_2w0bcx37ysnen`, read at `510e25a8d`). A name collision with
Map's private `leq_nat_add_right` brings Map's migration into this WP.

## Stop conditions

- A law needing a new postulate, axiom or `trusted_base()` entry: an
  operator question.
- Any change to `render_mode`, `doc_flat_width`, `doc_fits` or the
  existing content laws.
