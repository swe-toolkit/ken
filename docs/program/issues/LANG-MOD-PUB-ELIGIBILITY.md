---
id: LANG-MOD-PUB-ELIGIBILITY
title: "WP-3 — semantic gate rejecting pub on ineligible placements with a surface diagnostic; pub proof retains the subject-must-be-public rule"
status: ready
owner: language
size: M
gate: none
depends_on: []
blocks: [LANG-MOD-CATALOG-REALIZATION]
github: null
origin: "Architect component framing evt_hpnhqy1ex286 (WP-3), under [[LANG-MODULE-IMPORT-SYSTEM]]. Steward-filed per COORDINATION section 2, 2026-08-23. RELEASED 2026-08-23 (WP-1 merged be321d40b; language ring free; next in ring order WP-1->WP-3)."
---

> # RELEASED 2026-08-23 — module/import WP-3
>
> Full frame: `docs/program/wp/LANG-MOD-PUB-ELIGIBILITY.md`, fixed inputs at
> `origin/main be321d40b`. WP-1 ([[LANG-MOD-LOADER-ENTRY]]) merged, so the
> single-threaded language ring takes WP-3 next. The eligible/ineligible `pub`
> placement rules are normative in the merged spec surface (`def16ecf4`,
> `spec/30-surface/32-grammar.md:92-99`); this WP implements the surface
> rejection they describe.

# Objective

Reject `pub` on unsupported placements with a surface diagnostic, implementing
the eligibility behavior the /spec grammar repair normatively describes.

# The measured seam (Architect evt_hpnhqy1ex286)

`parse_pub_decl` (`crates/ken-elaborator/src/parser.rs:1286-1290`) is today a
blanket `Decl::Pub(Box<Decl>)` wrapper with NO eligibility check. `expand_scope`
(`modules.rs`) publishes/qualifies/silently-inerts by kind + nesting — nothing
REJECTS `pub` on an ineligible placement. So `pub` on
import/export/module/instance/fixity/header forms is silently accepted today.

# Deliverable

- A semantic gate that rejects unsupported `pub` placements
  (import/export/module/instance/program/package, per spec-author SPEC REPAIR 1)
  with a surface diagnostic.
- The `pub` proof rule retains §8.2 (the subject must be public).

# Acceptance criteria

- AC-1. `pub` on an eligible interface decl is accepted.
- AC-2. `pub` on each enumerated unsupported placement rejects with a surface
  diagnostic (one eligible-`pub` positive plus the enumerated ineligible
  placements — CV group 1 discriminator, evt_2wejn8hekr4qw). Freeze the
  CATEGORY of eligible placement, not a hand-listed roster.
- AC-3 (cross-cutting invariant). Zero `trusted_base()` delta; flat-Σ pin stays
  green.
- AC-NO-REGRESSION. Whole-suite green in CI; local targeted `-p` only.

# Reviewers

Architect (component fit) + conformance-validator (the pub-eligibility
rejections are CV's discriminator-pair territory).

# Capability tier

T2 (a bounded semantic gate at a known parser/scope seam; review is a
discriminator pair). Size M.
