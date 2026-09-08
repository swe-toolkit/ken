---
id: CAT-CC-ORACLE-BEHAVIORALIZE
title: "Behavioralize the pre-existing prohibited repository-text oracles in the cc3/cc4/cc5 catalog acceptance tests: replace the Axiom source scans with trusted-base delta checks, and the catalog data-declaration / fn text scans with loader-inventory / selective-resolution / elaboration controls. A test-only cleanup surfaced by CV during the CAT-MIGRATE-TIER-D-CURSOR review; pre-existing, non-blocking, queued behind the active lanes."
status: draft
owner: foundation
size: S
gate: none
tier: T2
depends_on: []
blocks: []
github: null
origin: "Steward-filed 2026-09-08 at foundation-leader's request (evt_54b2snsnh72f9, answer (b): unframed, file a small Foundation-owned draft; must NOT be folded into CAT-MIGRATE-TIER-D-CURSOR). Surfaced by CV (evt_7hy0ax01j3s08) during the CAT-MIGRATE-TIER-D-CURSOR review as the same prohibited-subject class the fleet blocked NonEmpty on: repository-text oracles that assert catalog TEXT rather than behavior. Pre-existing (NOT introduced by that candidate — its cc3/cc4/cc5 hunks only swapped the dependency-env to the roots loader, leaving these assertion lines untouched), so it did not block the migration; CV flagged it explicitly so it does not fall through the two-reviewer gap."
---

> # Test-only cleanup. Behavioralize prohibited catalog-text oracles in cc3/cc4/cc5.
> # Queued behind the active lanes; pre-existing, non-blocking.

## The prohibited oracles (CV census, evt_7hy0ax01j3s08 — exact sites)

`crates/ken-elaborator/tests/` acceptance suites cc3 (parsing/cursor/decoder), cc4
(diagnostic core), cc5 (pretty/doc) carry assertions that scan catalog SOURCE TEXT
rather than checking behavior — the exact class the operator test policy prohibits
(no CI checker asserting facts about catalog/spec lines) and the shape the fleet
blocked NonEmpty on. Re-measure exact line numbers at pickup; as CV named them:

- **Axiom source scans:** cc3:410, cc4:267, cc5:298 — `!extracted.source.contains("Axiom")`.
- **Catalog declaration text scans:** cc4:208
  `!PARSING_KEN_MD.contains("data SourceId =")`; cc4:212
  `!DECODER_KEN_MD.contains("Diagnostic")/("Origin")`; cc4:371/372
  `checked.contains("data Diagnostic = ...")/("data Origin =")`.
- **Function / signature text scans:** cc5:299-305 — `contains(...)` over
  `"string_length"`, `"Diagnostic"`, `"Text : List Char -> Doc"`,
  `"fn text_string"`, `"fn render_string"`, `"Equal String"`.

## Deliverable

Replace each prohibited text-scan with the behavioral control that measures the
same intended property (CV's mapping):

- **Axiom scans -> trusted-base delta.** The intent ("no axiom snuck into this
  module") is a `trusted_base()` before/after equality across the module's
  roots-load — the same live control the Tier-D migration already uses. Assert the
  trusted-base population is unchanged, not that the string "Axiom" is absent from
  source text.
- **Catalog data-declaration / fn text scans -> loader-inventory / selective-
  resolution / elaboration controls.** "This module declares `data SourceId`",
  "exposes `fn text_string`", etc. are loader-visible-inventory facts: probe the
  name through the real roots loader (resolves to the owning module's `GlobalId`,
  or rejects `UnboundName`), or assert the elaboration behavior, rather than
  grepping the `.ken.md` source. Reuse the `catalog_publication` test-support
  helper the Tier-D-Cursor slice landed where it fits.

Each converted assertion must keep its discriminating power: a mutation that
violates the intended property still reds (a real axiom addition reds the
trusted-base delta; a removed/renamed declaration reds the loader probe). Do NOT
weaken a probe to pass.

## Acceptance

- Zero remaining `.contains(<catalog-source-substring>)` / source-text scans in
  cc3/cc4/cc5; each replaced by a behavioral control (trusted-base delta, loader
  inventory/resolution, or elaboration).
- Each converted control reds under a real injected violation (a mutation control,
  not a vacuous pass).
- The three suites stay green on the current catalog; affected-target closure
  green (targeted via `scripts/ken-cargo`, never `--workspace`; CI is the
  workspace verdict).

## Scope / sequencing

Test-only — `crates/ken-elaborator/tests/` (cc3/cc4/cc5 + possibly the shared
support helper); no `catalog/` source edit, no `crates/**/src`. `gate: none`,
TCB-neutral. Reviewer: Foundation QA + CV (the finding's origin) on the exact SHA,
Architect if any control's design is non-obvious, then Steward M1-M4 -> lieutenant.
Draft and NON-BLOCKING: queued behind the active lanes (foundation-leader's
instruction) — released when a foundation seat is free and no higher-priority Tier
D slice is in flight. NOT folded into any CAT-MIGRATE-TIER-D node.
