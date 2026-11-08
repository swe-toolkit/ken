---
id: SPEC-EFFECTS-DECLARED-TREE-RESULT-DENOTATION
title: "Spec 36 §2.5 elaborates `proc f (x : A) : B visits ρ` to a result `ITree ⟦ρ⟧ ⟦B⟧` with no exception for a declared B that is already a tree or a function, so read literally every runtime program's `main : HostIO … visits [Console]` denotes a tree nested in a row tree, which no elaborator, runner or handler builds; and an [FS] proc has no stated Auth index for its tree. State what a declared tree result and a declared function result denote, and which Auth index a non-tree [FS] proc's tree carries"
status: merged
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: [LANG-PROC-NON-TREE-RESULT-TREE-FS-ROW-AND-ARROW]
github: null
origin: "Architect AC-0 ruling and re-rule on LANG-PROC-NON-TREE-RESULT-TREE (evt_2f6y8k5n184e4 option (a), evt_46t5492bw0fsm), Steward scope decision evt_4c6nvqtavtgg8, and Spec leader reading evt_50mkm6tk3xft9: §2.5's B is the declared result as written, and the clause gives no declared-tree exception. Architect sizing and recommendation evt_6cwmzctkn0ht5. Steward-filed per COORDINATION section 2."
---

# What a declared tree result denotes

## Objective

Spec 36 §2.5 states what a `visits` proc denotes when its declared
result B, after whnf, is an ITree, and when it is a function type. It
also states which Auth index the tree of a non-tree-result proc with an
`FS` label carries, and what `ensures` on a tree-result proc denotes.
The elaborator's treatment of each is then either the spec's or a named
gap.

## Settled inputs (main `8bfa0b4e1`)

- **The clause.** §2.5 (`36-effects.md:651` to `:652`) gives
  `f↓ : caps → ⟦A⟧ → ITree ⟦ρ⟧ ⟦B⟧`. §2.4 (`:621` to `:625`) collapses a
  pure `ITree 𝟘 ⟦B⟧` to `⟦B⟧`; it is the only stated exception.
- **The Spec reading** (`evt_50mkm6tk3xft9`). The literal formula nests
  the row tree around a declared tree B. The text does not say a
  declared tree result is its own denotation.
- **As built.** The elaborator takes a declared tree result as the
  denotation itself. LANG-PROC-NON-TREE-RESULT-TREE Inc 1 keeps that and
  excludes arrow-valued results (`evt_46t5492bw0fsm`).
- **The population** (Architect regex scan, approximate): about 690
  proc declarations with a tree head and a `visits` row, 649 of them in
  125 crates files, plus examples, conformance, library and catalog.
  Every runtime program's `main` is one. The arrow-valued population
  measured by the LANG-PROC sweep is 3 declarations in 2 files
  (`selected_body`, `selected_result`).
- **The [FS] gap** (Architect `evt_2f6y8k5n184e4`). `FS a r` needs an
  Auth index the row does not carry, so a non-tree [FS] proc has no
  tree to name. The candidates named are an implicit Auth parameter or
  the caller's. The LANG-PROC sweep measured 12 such procs.
- **`ensures` on a tree-result proc** (Architect `evt_1vmhvzqhrv6ak`,
  research advisory `evt_5tetw4wh8cvrd`). It denotes a relational post on
  the tree's Ret leaves, with a bind (cut) rule. The result type and every
  consumer stay at B. LANG-PROC-NON-TREE-RESULT-TREE Inc 1 refuses it by
  name, with the sentinel
  `ensured_changed_proc_refuses_until_tree_post_is_specified`, renamed
  `ensured_changed_proc_checks_with_all_ret_post` when FSRA Inc 3
  flipped it.
- **A recommendation, not a ruling** (Architect `evt_6cwmzctkn0ht5`): a
  declared `ITree S R` is its own denotation, with ρ kept as an escape
  bound checked against S (each label injects into S, as `host_console`
  does). The Spec enclave decides.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

Normative text in spec 36 §2.5 for the three cases, with a conformance seed
of one accepted and one refused row for each rule it states.

## Acceptance

- **AC-0 (Architect).** Confirm that the chosen rule needs no kernel
  change and adds no trust, and name each existing elaboration it
  changes. A rule that changes the 690 declared-tree programs' core
  types is a stop to the Steward.
- **AC-1.** The text, reviewed by the Spec leader and CV with a
  Decision. It states the declared-tree case, the arrow-valued case,
  the [FS] Auth index, how ρ relates to a declared tree's signature, and
  the `ensures` post with its cut rule.
- **AC-2.** The seed's refused row for the declared-tree case is a
  label in ρ that does not inject into the declared tree's signature,
  if the rule makes that a refusal; its accepted same-form control
  differs only in that label.

## Stop conditions

- Any kernel or `trusted_base()` requirement: an operator question.
- A rule that requires running a tree twice.

LANG-PROC-NON-TREE-RESULT-TREE-FS-ROW-AND-ARROW builds the arrow-valued
and [FS] parts.
