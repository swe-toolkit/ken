---
id: KERNEL-NORMALIZE-J-EVIDENCE-RECORDING
title: "Kernel normalization strips the Eq recording on J's equality argument, which j_endpoints treats as authoritative, so a normalized executable J over a reducing carrier cannot be re-inferred. Keep the recording through normalization and normalize only its components"
status: draft
owner: kernel
size: S
tier: T1
gate: architect
depends_on: []
blocks: [LANG-SOLVED-INDEX-EQUATION-UNIFIER]
github: null
origin: "Architect ruling evt_2xtv10vsa2fkd on the CI red of LANG-SOLVED-INDEX-EQUATION-UNIFIER 5344cf49c (PR #4924). Trusted-base change: release waits on the operator, fork evt_7a2475n6b13hh. Steward-filed per COORDINATION section 2."
---

# Normalization keeps J's Eq recording

## Objective

`normalize` keeps an Eq recording on J's equality argument, so the
normal form of a checked J is still a term `infer` accepts. The Language
unifier's computational J then reaches its named gate in executable
packaging, instead of failing in Ω-erasure for the wrong reason.

## Fixed inputs (main `cc072587c`, Architect `evt_2xtv10vsa2fkd`)

- **The defect.** `j_endpoints` (`check.rs:869`) reads endpoints from an
  ascription on J's evidence when one is present, after `infer` re-checks
  it. `normalize` (`conv.rs:501`) strips every ascription
  (`conv.rs:583`), including that one. A J whose recorded carrier
  reduces to a Π or Σ then normalizes to a body the checker cannot
  re-infer.
- **The counterexample.** With the unifier candidate 5344cf49c, ken-cli
  `rt_c5_primitive_type_native_gate::indexed_int_convoy_native_refusal_waits_for_compared_native_rows`
  fails with `OmegaErasurePlan ... bad eliminator: J's equality argument
  is not an Eq`. It passes on base d366662a6.
- **The patch, measured by the Architect.** It is in
  `local/architect-shared/lang-unifier-ci/k1-normalize-j-recording.patch`
  (sha256 `9a95b06e47e31ec4bf2ca5b4ebd1ef76756d1fa414d99cd6c26e6d53da9aba67`),
  touches `ken-kernel/src/conv.rs` only, +22/-1. The J arm of
  `normalize` calls a new `normalize_j_evidence`. When the evidence is an
  ascription whose recorded type has an Eq formation head
  (`eq_formation_head`, `check.rs:835`), that helper keeps the ascription
  with every component normalized; otherwise it falls back to
  `normalize`. Measured on 5344cf49c: the counterexample passes 1/1 and
  reaches `UnsupportedDependentMotive { family: Vec }`, and ken-kernel is
  green on every binary.
- **Why it can only refuse.** `j_endpoints` re-checks the kept ascription
  before reading it. The kept formation is the checked one with its
  components normalized. Conversion and whnf already look through
  ascriptions (`conv.rs:395`, `:1589-1590`).

Treat anchors as perishable. If a fixed input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

The patch above, applied to main, plus two kernel pins in a new
`crates/ken-kernel/tests/` file.

## Acceptance

- **AC-1 (positive).** Normalizing a J whose recorded evidence sits at a
  Σ- or Π-reducing carrier yields a term `infer` accepts. On main without
  the patch, the same pin is refused.
- **AC-2 (rejection).** A J whose recorded endpoint is wrong still
  carries it after normalization, and `j_endpoints` refuses it.
- **AC-3 (mutation, QA).** Reverting the J arm to plain `normalize`
  reddens AC-1.
- **AC-4.** The full CI run is the gate: every `normalize` caller is a
  consumer, and a normal form can now carry an ascription in
  J-evidence position.

## Stop conditions

- Any change outside `conv.rs` and the new test file.
- A changed verdict in any existing kernel test.

Checks: 2 fired: the recorded branch of `j_endpoints` is reached from
the elaborator's `build_sym` and from kernel transport reducts; both
pass through `normalize` at `conv.rs:583`, the one point the patch
changes. 3 fired: the consumers are every `normalize` caller, so AC-4
is full CI. The held, closed `wp/KERNEL-NORMALIZE-ORIGIN-TRACE`
(`7f1a04a40`) rewrites conv.rs's reducer and is not on main; this WP
does not touch it. 8 fired: the failure produces `OmegaErasurePlan`,
and the fix produces the named `UnsupportedDependentMotive`, which the
failure cannot produce.
