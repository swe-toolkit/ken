---
id: SURF-gadt-parser-ast
title: "Tracker node for the landed dependent-constructor parser/AST slice (`34 §8`): points at `docs/program/wp/SURF-gadt-parser-ast.md`, filed retroactively because `gen-progress.sh` generates the tracker from `docs/program/issues/` and this slice landed with a frame but no issue node"
status: merged
owner: language
size: S
gate: none
depends_on: []
blocks: []
github: null
origin: "[[LANG-GADT-SEQUENCE-TRACKER-GAP]] D1 -- Steward release evt_6jb0p5w0zx69p, thread thr_11nhrpk58gkqt. `spec/30-surface/34-data-match.md:900` names this WP; its frame is `docs/program/wp/SURF-gadt-parser-ast.md`; landed content is at squash-merge `8488af0f` (same tree as the frame's cited reviewed head `82fba107` -- a squash-merge duplicate, per the fleet's squash-merged-head-reads-unlanded lesson)."
---

## What this is

The tracker node for `SURF-gadt-parser-ast`, the first of the four
dependent-constructor build slices `34 §8` names. Full design content lives in
the frame, `docs/program/wp/SURF-gadt-parser-ast.md` -- this node does not
restate it, only tracks status with citations.

## Status, measured at `origin/main` `f78b486d`

**Landed.** Squash-merge `8488af0f` ("SURF-gadt-parser-ast reject empty data
block") is an ancestor of `f78b486d`. Test file:
`crates/ken-elaborator/tests/explicit_data_parser.rs`, 13 tests.

Per-AC discharge, obligation-keyed (`AC-3` of the tracker-gap node): for each
AC I searched for the test/code that discharges the frame's stated obligation,
not for the frame's own vocabulary.

- **AC1 (positive explicit family parse).** Search: a test constructing
  `data Vec (A : Type) : Nat -> Type where { ... }` and inspecting the parsed
  AST shape. Found:
  `explicit_family_vec_preserves_constructor_signature_shape`
  (`explicit_data_parser.rs:26`) -- asserts family name, parameter binder,
  result/index type, constructor names, and `VCons`'s telescope entries
  (named vs. anonymous) in order.
- **AC2 (proof-carrying constructor syntax parse).** Search: a test where a
  later argument type mentions an earlier binder name syntactically. Found:
  `proof_carrying_constructor_signature_parses_as_telescope`
  (`explicit_data_parser.rs:94`) -- `CheckedSource`'s telescope references
  `bs`/`len` from earlier binders; asserts shape only, not elaboration.
- **AC3 (legacy grammar boundary).** Search: tests pinning the three-way
  split (legacy accepts, explicit `where` accepts, legacy+explicit-signature
  rejects). Found:
  `legacy_data_form_stays_simple_and_rejects_explicit_signatures`
  (`explicit_data_parser.rs:250`) -- all three cases in one test, and the
  rejection asserts `msg.contains("found Colon")`, i.e. a syntax-boundary
  message, not an elaboration/kernel one.
- **AC4 (no elaboration scope creep).** Search: `explicit_data_parser.rs` for
  any assertion that a family elaborates or kernel-checks. None exists --
  every test in the file asserts on the parsed `Decl`/AST shape only. Negative
  claim verified by absence, not by a positive test.
- **AC5 (regression gate).** Search: whether legacy tests still exist and
  whether new tests are parser/AST-scoped. `legacy_data_form_stays_simple_...`
  and `explicit_family_rejects_bare_head_parameters`
  (`explicit_data_parser.rs:278`) are both grammar-boundary tests, not
  elaboration tests. `git show --stat 8488af0f` confirms the touched area was
  `crates/ken-elaborator/{ast,lib,modules,parser,resolve}.rs` plus the test
  file and the frame doc -- no `ken-kernel`, `Cargo.lock`, `packages`, `spec`,
  or `conformance` movement.
- **AC6 (durable naming hygiene).** Search: `grep -rniE '\bsurf\b|gadt'` over
  `crates/ken-elaborator/src/*.rs` outside test files. The only hits are
  unrelated WP-name comments (`SURF-def-refinement`, `SURF-named-proof-claims`,
  `SURF-IDENT-TR39-R1`) and one generic "GADT index" comment in `prelude.rs`
  describing a dependently-typed index, not a `SURF-gadt-*` identifier. AST
  variant/parser-helper names are domain-named (`ExplicitDataDecl`,
  `ExplicitDataCtor`, `ConstructorSignatureArg`).

**Two tests exist beyond the frame's stated AC list** --
`explicit_family_accepts_empty_constructor_block` and
`constructor_result_indices_use_expression_surface` -- both parser/AST-scoped,
consistent with AC5's boundary; not separately re-cited as they don't discharge
a distinct numbered AC.

## Not this node

No repair, no reframing of the linked frame. If a future slice needs to change
this area, that is a new node, per `[[LANG-GADT-SEQUENCE-TRACKER-GAP]]`.
