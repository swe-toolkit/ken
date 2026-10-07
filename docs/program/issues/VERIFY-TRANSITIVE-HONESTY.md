---
id: VERIFY-TRANSITIVE-HONESTY
title: "A claim reads proved when its own certificate checks and its own hole is outside trusted_base(); nothing computes reachability. Under subset Σ a certificate using the proof of a constant whose obligation is open would read proved. Add a read-only kernel reachability query and require it to find no open hole or unaccepted postulate"
status: ready
owner: verify
size: M
tier: T1
gate: architect
depends_on: []
blocks: [LANG-REFINEMENT-SUBSET-SIGMA]
github: null
origin: "Operator 2026-10-07: \"agreed, refinements should be real kernel types\" (OQ-refinement-representation DECIDED for the core subset Σ). Architect design and cut evt_30frdrmj45ehg. Steward-filed per COORDINATION section 2. W4, kernel hunk K-b."
---

# `proved` means nothing open is reachable

## Objective

A claim reads `proved` only if its certificate checks and no open
obligation hole or unaccepted postulate is reachable from it (21 §5.4).

## Settled inputs (Architect, measured at `1153a9fc6`)

- `Proved` means the certificate checks (`prover.rs:344`). Status is the
  hole's own membership in `trusted_base()`. Nothing computes
  reachability, and no kernel query for it exists.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

- **Kernel hunk K-b**, authorized: a read-only
  `postulates_reachable(env, term) -> BTreeSet<GlobalId>` over the δ-closure
  through transparent bodies. It is off the check and conversion paths.
- The status computation for prover certificates and term proofs
  (`theorem`, `proof`) reads `proved` only when that set contains no
  open obligation hole and no unaccepted postulate.
- **Conformance flip** (Architect `evt_3t2jfn2bt8n3e`). In the same
  candidate, flip `v1_acceptance.rs:276`
  (`proved_status_cert_checks_not_in_trusted_base`) to claim
  `verify/spec-syntax/proved-status-cert-checks-no-reachable-open-hole`,
  and retire the current-behaviour row
  `verify/spec-syntax/proved-status-cert-checks-not-in-trusted-base`.

## Acceptance

- **AC-0 (D0, measure only).** Whether a `prove name` certificate whose
  closure reaches an open postulate already reads `proved` on main.
- **AC-1.** A claim proved through a transparent constant whose body
  uses an open hole reads not-proved. The same claim with the hole
  discharged reads `proved`.
- **AC-2.** Existing statuses across the catalog are unchanged, except
  rows AC-0 lists.
- **AC-3.** Kernel QA reviews the K-b lines; no other kernel line
  changes. A unit test pins the query on a two-level δ chain.
- **AC-4 (mutation).** Stopping the walk at the first δ level reddens
  AC-1.

## Stop conditions

- Any kernel line beyond K-b, any check or conversion change, or any
  `trusted_base()` or spec change.
