---
id: CONF-VERIFY-SPEC-SYNTAX-PHANTOM-CLAIMS
title: "Four v1_acceptance tests claim verify/spec-syntax conformance rows that were never authored -- invisible until the row-claim checker's namespace widening, and now a mechanical merge blocker for CI-ROW-CLAIM-NAMESPACE"
status: active
owner: spec-enclave
size: S
gate: none
depends_on: []
blocks: [CI-ROW-CLAIM-NAMESPACE]
github: null
origin: "Surfaced by verify-implementer at evt_5y3etqdhn2f4s while building CI-ROW-CLAIM-NAMESPACE's D3, on held checkpoint 8f8bad6d. The widened checker's real-tree census is 34 attached claims / 30 resolving. Steward independently confirmed all four ids have zero occurrences anywhere under conformance/ and are not near-misses of the 16 existing verify/spec-syntax headings, and confirmed the nonzero-exit mechanism. Steward-filed (agents cannot create tracked work per COORDINATION §2)."
---

> ## Frame: `docs/program/wp/CONF-VERIFY-SPEC-SYNTAX-PHANTOM-CLAIMS.md`.
> `ready`, shovel-ready. **Blocks `CI-ROW-CLAIM-NAMESPACE`** — that node's
> widening cannot merge until this lands.

## The defect

Four `#[test]` functions in `crates/ken-elaborator/tests/v1_acceptance.rs`
carry `/// verify/spec-syntax/<id>` claims whose rows **do not exist**:

| test site | claimed row |
|---|---|
| `v1_acceptance.rs:205` | `verify/spec-syntax/old-fails-closed-without-pre-state` |
| `v1_acceptance.rs:543` | `verify/spec-syntax/requires-on-first-param-of-two` |
| `v1_acceptance.rs:583` | `verify/spec-syntax/requires-on-middle-param-of-three` |
| `v1_acceptance.rs:624` | `verify/spec-syntax/requires-on-final-param-unaffected` |

**All four have zero occurrences anywhere under `conformance/`** — measured, not
inferred. `conformance/verify/spec-syntax/seed-spec-syntax.md` carries 16
headings and none of these.

**They are not typos or renames.** The nearest existing headings are
`old-resolves-in-space-op-ensures` / `old-out-of-scope-rejects` and
`requires-elaborates-to-pi-proof-arg`. The three `requires-on-*-param` ids are a
**positional-coverage family** the seed does not have at all, and
`old-fails-closed-without-pre-state` is a distinct fail-closed behaviour from
either `old` row. So the tests exercise real, distinct behaviour that was never
written down.

## Why it stayed invisible, which is the same sentence a third time

`verify-row-claims` hardcoded `surface/` in both its patterns, so every
`verify/` claim was structurally invisible. This is the third axis of that one
defect — after executing-cover and comment-form — and it repeats the finding:

> **A claim the extractor cannot see is indistinguishable from a claim that does
> not exist.**

**These four are pre-existing.** They predate `CI-ROW-CLAIM-NAMESPACE` and are
untouched by it; that node merely made them visible.

## Why this blocks, and it is mechanical rather than cautionary

`verify_row_claims` raises `SweepError` on any claim that does not resolve to
exactly one heading. `main()` catches it and returns **2**. The
`ignored-row-sweep` job runs it under `shell: bash` (`-e`), so a nonzero exit
fails the step and the job, and `ci.yml` states that instrument failures are
"enforced by build-test's needs/result gate below because no trustworthy
measurement was made."

⇒ **Merging the widening while these four are unresolved reds `main` and blocks
the publisher for every ring in the fleet.**

That is a mechanical blocker. It is **not** the "a currently-working path would
go red" intuition the operator's 2026-07-28 no-users ruling retired, and it must
not be argued against on those grounds. The job's own name says "findings
non-blocking", which is true of findings and **false of instrument failures** —
do not read the name as the contract.

## The disposition is a conformance call, and it is not the Steward's

Per claim, exactly one of:

- **author the row** in `conformance/verify/spec-syntax/seed-spec-syntax.md`,
  because the behaviour deserves conformance coverage; or
- **correct or remove the claim**, because it does not.

**The Steward does not rule which.** These four behaviours are real and tested,
which makes authoring the likely answer, but whether a behaviour earns a
conformance row is the enclave's judgment. Decide it per claim, on the merits,
and say which you chose and why.

**Do not narrow the checker** and do not quarantine these ids to reach green —
that is the exact trap `CI-ROW-CLAIM-NAMESPACE` was framed to avoid.
