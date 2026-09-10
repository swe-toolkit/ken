---
id: CAT-CC-ORACLE-BEHAVIORALIZE
title: "Behavioralize the pre-existing prohibited repository-text oracles in the cc3/cc4/cc5 catalog acceptance tests: replace the Axiom source scans with trusted-base delta checks, and the catalog data-declaration / fn text scans with loader-inventory / selective-resolution / elaboration controls. A test-only cleanup surfaced by CV during the CAT-MIGRATE-TIER-D-CURSOR review; pre-existing, non-blocking, queued behind the active lanes."
status: merged
owner: foundation
size: S
gate: none
tier: T2
depends_on: []
blocks: []
github: null
origin: "Steward-filed 2026-09-08 at foundation-leader's request (evt_54b2snsnh72f9, answer (b): unframed, file a small Foundation-owned draft; must NOT be folded into CAT-MIGRATE-TIER-D-CURSOR). Surfaced by CV (evt_7hy0ax01j3s08) during the CAT-MIGRATE-TIER-D-CURSOR review as the same prohibited-subject class the fleet blocked NonEmpty on: repository-text oracles that assert catalog TEXT rather than behavior. Pre-existing (NOT introduced by that candidate — its cc3/cc4/cc5 hunks only swapped the dependency-env to the roots loader, leaving these assertion lines untouched), so it did not block the migration; CV flagged it explicitly so it does not fall through the two-reviewer gap."
---

> # LANDED 2026-09-10 at 9d614db31 (squash of 701afd7bb) — oracles behavioralized.
> #
> # Merged via the publisher, blob-verified byte-identical on all 4 paths
> # (cc3/cc4/cc5 + support/catalog_or.rs). All 17 in-scope repository-text oracle
> # groups in cc3/cc4/cc5 replaced by mutation-proven behavioral controls
> # (trusted-base delta, loader-visible inventory, identity closure, selective-
> # client elaboration). Scope stayed frozen to the three CV-named suites; the
> # catalog_or reference-collection helpers are the trust-provider isolation, not
> # a widening. Test-only, no src/catalog/carrier/proof change, trusted_base()
> # unaffected (the suites MEASURE it). Foundation QA evt_12wmg3cm1gdma + CV
> # evt_56fyfdr1kawk7 (closes CV's prohibited-oracle finding across the Tier-D/E
> # slices); no Architect (standard control designs). Decision dec_6q4vmtqw3v1rc.

## The prohibited oracles (CV census, evt_7hy0ax01j3s08 — exact sites)

`crates/ken-elaborator/tests/` acceptance suites cc3 (parsing/cursor/decoder), cc4
(diagnostic core), cc5 (pretty/doc) carry assertions that scan catalog SOURCE TEXT
rather than checking behavior — the exact class the operator test policy prohibits
(no CI checker asserting facts about catalog/spec lines) and the shape the fleet
blocked NonEmpty on.

Grounded at `origin/main` `06c102a38` (the three suites exist unchanged from the
`39ebe2c8f` Decoder landing foundation-leader named; the intervening `06c102a38`
landing was docs/program-only, so the test files are byte-identical — D0
re-measures at pickup regardless). Line numbers below are the CV census
(evt_7hy0ax01j3s08); D0 re-measures the exact sites:

- **Axiom source scans:** cc3:410, cc4:267, cc5:298 — `!extracted.source.contains("Axiom")`.
- **Catalog declaration text scans:** cc4:208
  `!PARSING_KEN_MD.contains("data SourceId =")`; cc4:212
  `!DECODER_KEN_MD.contains("Diagnostic")/("Origin")`; cc4:371/372
  `checked.contains("data Diagnostic = ...")/("data Origin =")`.
- **Function / signature text scans:** cc5:299-305 — `contains(...)` over
  `"string_length"`, `"Diagnostic"`, `"Text : List Char -> Doc"`,
  `"fn text_string"`, `"fn render_string"`, `"Equal String"`.

## Deliverables (D0-first)

**D0 — re-measure the census and fix the mapping.** At the pickup SHA, enumerate
the exact source-text-scan sites in cc3/cc4/cc5 (the CV list below is the expected
set; report any drift), and for each one name the behavioral control that measures
its intended property and the test-support surface it uses (the roots loader,
`trusted_base()`, the `catalog_publication` helper). Hard-stop to the Steward if
any site's intended property has no behavioral equivalent — i.e. it asserts a
property of the source text that is not a loader/trusted-base/elaboration fact
(none is expected; every listed site maps below).

**D1 — execute the D0 mapping.** Replace each prohibited text-scan with the
behavioral control that measures the same intended property (CV's mapping):

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
RELEASED 2026-09-10 (foundation-leader evt_6hqdgayjbgtjp: the active migration
lanes are complete, so it is ready to frame + release now). NOT folded into any
CAT-MIGRATE-TIER-D node.

**Scope is exactly the CV-named cc3/cc4/cc5 sites.** The identical prohibited
text-scan class also exists in other suites (px8f, cc6a, cc7, ds7 all carry
`source.contains("Axiom")` / `_KEN_MD.contains(...)` scans). Those are OUT of
scope: this node behavioralizes the three suites the CV census named, no more. If
D0 finds the class in a cc3/cc4/cc5 site the census missed, fold it in (same
suite, same finding); a sibling suite is a separate observation to surface to the
Steward, NOT a scope expansion — do not widen the node into a corpus-wide sweep.

**Contention.** Test-only in `crates/ken-elaborator/tests/`; the three suites are
not touched by any in-flight WP (foundation-implementer is free post-Decoder; the
runtime lane's ABI-S6 D5a works native-promotion codegen + `rt_parity_native`, not
the cc-suites; the language lane is idle). Re-measure contention at pickup.
