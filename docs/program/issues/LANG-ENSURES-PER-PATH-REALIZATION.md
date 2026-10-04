---
id: LANG-ENSURES-PER-PATH-REALIZATION
title: "An ensures clause or a literal refined return over a branchy body emits one obligation over the whole body, which spec 22 §2.2 forbids: a recursive postcondition then carries no induction hypothesis and cannot be discharged. Realize the postcondition at the body's leaves, under each branch's path hypotheses"
status: ready
owner: language
size: M
tier: T1
gate: architect
depends_on: [LANG-REFINEMENT-INTRODUCTION-OBLIGATION]
blocks: []
github: null
origin: "Architect ruling evt_62k15kkjw0818 on the LANG-REFINEMENT-INTRODUCTION-OBLIGATION stop evt_5b9ywyr5p5w32: named refined returns are realized per leaf in that WP; the literal and ensures sites are carried here. Normative source spec/20-verification/22-obligations.md §2.2. Steward-filed per COORDINATION section 2."
---

# Postconditions are realized per path

## Objective

A postcondition, whether an `ensures` clause or a literal refined result or
let annotation, is the body's expected type and is pushed through its
structure. A branchy body emits one obligation per leaf under that branch's
path hypotheses, and none over the whole body (spec 22 §2.2).

## Settled inputs (Architect `evt_62k15kkjw0818`)

- After `LANG-REFINEMENT-INTRODUCTION-OBLIGATION`, a def-named refined
  return is checked against the declared type, and the introduction
  machinery fires at each leaf under the path conditions that `check_if` and
  `check_dependent_branch_body` push.
- The `ensures` path is main's V1 path, one hole per clause over the whole
  body (`elab.rs:15518` on `45a15f913`).
- A literal let or ascription emits once over the whole right-hand side: the
  `literal_let_match` row in `lang_refinement_introduction.rs` is 1.
- A literal refinement has no source identity to carry, so per-leaf
  realization needs the literal predicate threaded through `check`.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch; do not build around it.

## Deliverable

`ensures` clauses and literal refinements at a return, let or ascription are
realized at the body's leaves through `check`, under the branch path
hypotheses, including the induction hypothesis of a recursive branch. A
straight-line body still emits exactly one obligation with the same goal.

## Acceptance

- **AC-0 (measure, then the Architect rules the design).** Obligation counts
  via `elaborate_decl_v1` for an `ensures` clause and a literal refined
  return over a straight-line body, a two-arm `match`, an `if`, and a
  structurally recursive function.
- **AC-1.** Each branchy row emits one obligation per leaf and none over the
  whole body. The `literal_let_match` row goes from 1 to 2, and its comment
  no longer says it measures the seed form. A structurally recursive
  function whose `ensures` needs its induction hypothesis discharges.
- **AC-2 (controls).** Straight-line counts and goals are unchanged. The
  elaborator suites the Architect measured on the predecessor stay green
  (`cc2_text_codec_numeric_acceptance`, `lang_refinement_introduction`,
  `decimal_char_acceptance`, `ds9_json_codec_acceptance`, `v1_acceptance`,
  `v2_acceptance`), and so do `rtp1_elim_reduce_ih_perf_acceptance` and
  `rosetta`. CI is green.
- **AC-3 (mutation, QA).** Restoring the whole-body emission for `ensures`
  reddens the recursive row and the per-leaf counts.

## Stop conditions

- Any kernel or `trusted_base()` change, or a spec change.
- A checked program gains an obligation it cannot discharge.
