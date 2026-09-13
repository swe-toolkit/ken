---
id: CAT-PROOF-COMPLETENESS-SURVEY
title: "Survey every catalog package for incomplete proofs and dependence on computational tests: classify each package as fully-proven, tested-only-with-deferred-proofs, or no-proof-obligation, citing the exact intrinsics vs tests per package; produce the ledger from which the Steward frames the missing *-LAWS follow-ons. Grounded in PRINCIPLES #16 (a package is finished only when proven)."
status: ready
owner: foundation
size: L
gate: none
depends_on: [CAT-PRIORITY-QUEUE-LAWS]
blocks: []
github: null
tier: T1
origin: "Operator directive (Pat, this session): 'Once that is done, we need a catalog survey to determine if other packages suffer from incomplete proofs and a dependence on computational tests. Note that computational tests are not part of the package, so a reader cannot trust them as intrinsics, but must believe that the implementation of both the package and its tests was done correctly. This is inherently inferior and weakens the promise ken strives to make.' Recorded as docs/PRINCIPLES.md #16. Steward-framed this session; sequenced by Pat AFTER the CAT-PRIORITY-QUEUE proof follow-on (CAT-PRIORITY-QUEUE-LAWS) so the completed exemplar establishes the pattern the survey applies. Released (flip ready) once CAT-PRIORITY-QUEUE-LAWS lands, or on operator re-sequencing."
---

> # RELEASED 2026-09-13 (Steward) — CAT-PRIORITY-QUEUE-LAWS has landed.
>
> Pat directed this survey to run AFTER the priority-queue proof follow-on:
> "Once that is done, we need a catalog survey ...". CAT-PRIORITY-QUEUE-LAWS is
> the exemplar — completing it establishes what "complete proofs" looks like for
> a tested-only catalog package, which is the classification standard this survey
> then applies across the corpus. That precondition is now met:
> CAT-PRIORITY-QUEUE-LAWS merged (main 7663ad9b9, closed 4335e95c4), so this node
> is released to the foundation ring. Re-measure every input at the cut SHA.

# Objective

Determine which catalog packages ship a computational implementation whose
semantic requirements are covered only by computational tests, with their
general kernel proofs deferred or absent — the exact defect Pat identified in
`CAT-PRIORITY-QUEUE`. Produce a per-package classification ledger. The Steward
then frames a `*-LAWS` proof follow-on for every tested-only package, the way
`CAT-PRIORITY-QUEUE-LAWS` and `CAT-REL-CLOSURE-LAWS` were framed.

# Why this constraint is grounded

`docs/PRINCIPLES.md` #16 (operator ruling, this session): a proof is an
intrinsic the kernel re-checks, so a reader verifies it; a computational test is
external, so a reader must instead TRUST that both the package and its tests were
implemented correctly. Tested-only computation is therefore an increment, never
a completion, and "a catalog package is not finished until its proofs are
complete." This survey measures the gap between that standard and the corpus as
built. The constraint is an operator commitment recorded in the reasoning
charter, not a frame-local or aesthetic preference.

# The classification criterion (apply per package)

For each package, enumerate its semantic requirements (the behavioral contract
its spec/chapter states), then classify:

- **fully-proven** — every semantic requirement has a general kernel proof
  (proof-bearing data in `Type`, Boolean decidable predicates reflected into
  `Omega`; no `Omega` path carrier, postulate, new primitive, `Axiom`, or TCB
  entry). Computational tests, if present, are redundant confidence, not the
  basis of the guarantee.
- **tested-only-with-deferred-proofs** — one or more semantic requirements are
  load-bearing and covered by computational tests, but their general proof is
  deferred (to a named follow-on) or absent. This is the defect class. Record
  exactly which requirements are tested-only and where the tests live.
- **no-proof-obligation** — the package is a pure re-export/consumer, a
  registration/inventory unit, or otherwise carries no semantic requirement of
  its own to prove. State why.

Classify by the criterion (what is proven vs what is only tested), not by
whether the package "looks done" or its tests are green. A green computational
test is evidence the code ran on fixtures, not that the requirement holds in
general — that distinction is the whole point.

# Scope

Walk `catalog/packages/` recursively — the package trees under `Algorithm/`,
`Application/`, `Capability/`, `Core/`, `Data/`, and `Tooling/`. For each
package (`.ken.md` unit), read its declarations, its attached proofs, and its
acceptance/computational tests. Cross-reference the tracker: a package with a
`CAT-*` build node and no `*-LAWS` follow-on, whose build shipped "tested
computation", is a prime candidate. Known follow-ons already filed:
`CAT-PRIORITY-QUEUE-LAWS`, `CAT-REL-CLOSURE-LAWS` — only two exist today, which
is itself the hypothesis under test (the corpus likely undercounts).

# Deliverables

1. A classification report, `docs/program/CATALOG-PROOF-COMPLETENESS-SURVEY.md`:
   one row per catalog package with its classification, the exact semantic
   requirements, which are proven (cite the proof) vs tested-only (cite the
   test), and the file locations. Measure every input at a named `origin/main`
   SHA and record it.
2. For every `tested-only-with-deferred-proofs` package, a one-line follow-on
   recommendation: the proof obligations a `*-LAWS` node would discharge, over
   the SAME representation the build uses (no second implementation), under the
   same trust posture as `CAT-PRIORITY-QUEUE-LAWS` (no `Omega` path
   carrier/postulate/new primitive/`Axiom`/TCB; kernel-untouched). The Steward
   frames the actual `*-LAWS` nodes from these recommendations.
3. A short summary: count of packages in each class, and the list of missing
   `*-LAWS` follow-ons.

# Acceptance criteria

- Every package under `catalog/packages/` appears in the ledger exactly once
  with a classification and its citation. A census that skips a subtree fails;
  state the walk and its count.
- Each `tested-only` classification cites the exact semantic requirement, the
  test that covers it, and the absence (or deferral target) of its proof — a
  bare "tested only" without the citation is not a finding.
- No package is classed `fully-proven` on the strength of a green test; the
  classification cites the proof term, or it is not `fully-proven`.
- The report distinguishes a genuinely deferred proof (named follow-on, e.g.
  complexity bounds explicitly out of scope) from an absent one.
- Inputs measured at a named SHA; re-measure at the cut.

# Trust posture and constraints

Read-only survey over the catalog and tracker; no catalog code change, no kernel
change, no new node minting inside this WP (the Steward frames follow-ons from
the report). This is analysis that scopes the L3 "complete catalog proofs"
objective, not the proof authoring itself.

# Not this node

- Authoring any `*-LAWS` proof — each is its own follow-on node.
- Machine-checked complexity bounds (a separate deferral class, per
  `CAT-PRIORITY-QUEUE-LAWS`).
- Non-catalog crates or the spec corpus.

# Sizing / tier

Size L, tier T1. Classifying proof completeness requires reading each package's
proofs and contract and judging generality vs fixture coverage — reasoning, not
a mechanical grep. The breadth (whole catalog) is the size; the per-package
judgment is the tier.
