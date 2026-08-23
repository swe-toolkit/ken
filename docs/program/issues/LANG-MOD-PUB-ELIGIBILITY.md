---
id: LANG-MOD-PUB-ELIGIBILITY
title: "WP-3 — semantic gate rejecting pub on ineligible placements with a surface diagnostic; pub proof retains the subject-must-be-public rule"
status: draft
owner: language
size: M
gate: none
depends_on: []
blocks: [LANG-MOD-CATALOG-REALIZATION]
github: null
origin: "Architect component framing evt_hpnhqy1ex286 (WP-3), under [[LANG-MODULE-IMPORT-SYSTEM]]. Steward-filed per COORDINATION section 2, 2026-08-23. FRAMED; release HELD (see campaign root release gate)."
---

> # FRAMED — HELD FOR RELEASE
>
> Independent, can lead the campaign (parallel with WP-1). Release held until the
> language ring finishes [[V3-FO-EMBEDDING-ADEQUACY]] (finish-then-switch). The
> eligible/ineligible placement list is fixed by spec-author's grammar repair
> (SPEC REPAIR 1) in candidate 860c605; author the wp/ frame against the MERGED
> spec so the AC roster matches CV's landed conformance fold.

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
