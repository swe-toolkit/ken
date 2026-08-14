---
id: SURF-gadt-coverage-diagnostics
title: "Tracker node for the landed dependent-constructor coverage/diagnostics slice (`34 §4.3`/`§8`): points at `docs/program/wp/SURF-gadt-coverage-diagnostics.md`, and carries the tracker-gap node's two D3 findings -- the AC5 reject half EXISTS and is cited, and the exhaustiveness diagnostic names the missing CONSTRUCTOR, not `§4.1`'s most-general PATTERN witness, for any constructor with arguments"
status: merged
owner: language
size: S
gate: none
depends_on: [SURF-gadt-elaboration]
blocks: []
github: null
origin: "[[LANG-GADT-SEQUENCE-TRACKER-GAP]] D1/D3 -- Steward release evt_6jb0p5w0zx69p, thread thr_11nhrpk58gkqt. Frame is `docs/program/wp/SURF-gadt-coverage-diagnostics.md`; landed at squash-merge `127066d5` (\"SURF-gadt-coverage: repair indexed premise evidence\")."
---

## What this is

The tracker node for `SURF-gadt-coverage-diagnostics`, the third
dependent-constructor build slice. Full design content lives in
`docs/program/wp/SURF-gadt-coverage-diagnostics.md` -- not restated here.
This node also discharges `[[LANG-GADT-SEQUENCE-TRACKER-GAP]]`'s `D3`, which
named this slice as the natural home for its two open questions.

## Status, measured at `origin/main` `f78b486d`

**Landed.** Squash-merge `127066d5` is an ancestor of `f78b486d`. Tests live
in the same shared file as the elaboration slice's,
`crates/ken-elaborator/tests/explicit_data_elaboration.rs`.

Per-AC discharge, obligation-keyed:

- **AC1 (indexed impossible omission accepts).** Search: a `head`-style
  function over `Vector A (Suc n)` omitting `EmptyVector`, emitting a total
  `elim_Vector` with a synthesized (absurd) method, not an under-applied
  eliminator. Found:
  `indexed_impossible_constructor_may_be_omitted_from_non_empty_vector_match`
  (`explicit_data_elaboration.rs:304`) -- asserts `methods.len() == 2` (both
  constructors have a method; nothing is under-applied), that method 0
  (`EmptyVector`) `contains_absurd`, and that the motive abstracts the index
  before the scrutinee.
- **AC2 (impossible application rejects -- the AC5-pair reject half).** This
  is `[[LANG-GADT-SEQUENCE-TRACKER-GAP]]` `D3` question 1. Search: applying
  the omitting function to `EmptyVector` at the domain index it excludes.
  **Found, present**: `indexed_head_rejects_empty_vector_application`
  (`explicit_data_elaboration.rs:390`) -- applies `vectorHead Nat Zero
  (EmptyVector Nat)` after `vectorHead`'s domain is `Vector A (Suc n)`, and
  asserts the application errors (`"type mismatch"` or `"kernel rejected"`).
  **D3 answer: the reject half exists and is tested independently of the
  accept half** (a separate function/test from AC1's, not a shared fixture),
  satisfying `§8` AC5's "non-degenerate pair" requirement.
- **AC3 (type-possible omissions still reject with a named witness).**
  Search: omitting a constructor that IS type-possible at the scrutinee index
  rejects as non-exhaustive, naming it. Found:
  `type_possible_indexed_constructor_is_still_required`
  (`explicit_data_elaboration.rs:412`) -- a match over `Vector A n` (unfixed
  index) omitting `EmptyVector` rejects with `"non-exhaustive match"` and
  `"EmptyVector"`.

  **This is also where `[[LANG-GADT-SEQUENCE-TRACKER-GAP]]` `D3` question 2
  is answered**, because it is the diagnostic under test: does `missing
  constructor` discharge `34 §4.1`'s "names the unmatched pattern witness"?

  Read against the diagnostic's own construction
  (`crates/ken-elaborator/src/error.rs:183`, `ElabError::ExhaustivenessError {
  missing: String, span }`) and its four production sites in `elab.rs`
  (`:1585`, `:2134`, `:3199`, `:8316`), **every one populates `missing` from
  `ctor_name`/an inverse-scan constructor NAME only** -- never a pattern with
  placeholder argument positions. `§4.1`'s own example is explicit that the
  witness for a constructor with arguments is the **applied** form (`VCons _
  _ _`, not `VCons`). Searched the crate for any other diagnostic surface
  that could be supplying the fuller form (`grep -rn witness
  crates/ken-elaborator/src/*.rs`): no hit outside unrelated Kripke-model and
  runtime-witness code.

  **D3 answer, stated plainly: `missing constructor` does NOT discharge
  `§4.1`'s witness obligation for any constructor with arguments** (`VCons`,
  `ConsVector`, etc.) -- it names which constructor is missing, not the
  most-general uncovered pattern. **This is general to the whole
  exhaustiveness mechanism, not indexed-specific**: the same `missing:
  String` shape is the sole payload at all four call sites, both indexed and
  non-indexed. For a zero-arity constructor (`EmptyVector`, `Blue`) the name
  and the witness coincide, which is why the landed tests -- all against
  zero-arity omissions -- read as satisfying `§4.1` without exposing the gap.
  **Per `[[LANG-GADT-SEQUENCE-TRACKER-GAP]]` `D4`, this is named here and not
  fixed by this node.** A fix (emitting placeholder-applied witnesses) is
  future work for whoever owns `crates/ken-elaborator/src/error.rs` next.
- **AC4 (dependent motive shape is structural).** Search: the indexed
  accepted case's motive genuinely depends on index/scrutinee, not a constant
  motive. Found: reuses AC1's test
  (`indexed_impossible_constructor_may_be_omitted_from_non_empty_vector_match`,
  `:304`) via `motive_has_index_and_scrutinee_lambdas` -- asserts the motive
  is `Lam(_, Lam(_, _))` (two nested binders: index then scrutinee), not a
  single constant-returning lambda.
- **AC5 (non-indexed behavior is unchanged).** Search: legacy exhaustiveness/
  reachability/simple-data tests still present and unrelated to this slice's
  diff. `legacy_simple_data_still_elaborates`
  (`explicit_data_elaboration.rs:280`) predates this slice (landed with
  `SURF-gadt-elaboration`) and remains green; the L2 acceptance suite
  (`l2_acceptance`, cited in the landing commit's own validation list) is the
  broader non-indexed regression net and is out of this test file's scope.
- **AC6 (scope is bounded).** `git show --stat 127066d5`: touched area is
  `crates/ken-elaborator/src/elab.rs` plus the test file and frame doc -- no
  `ken-kernel`, `Cargo.lock`, `packages`, `spec`, or `conformance` movement.
- **AC7 (durable naming hygiene).** Same sweep as the prior two slices:
  `grep -rniE '\bsurf\b|gadt'` over non-test `src/*.rs` shows no
  `SURF-gadt-*` leakage; the diagnostic/AST surface uses domain names
  (`Vector`, `EmptyVector`, `ConsVector`).

**Two tests beyond the frame's AC list**,
`concrete_non_empty_vector_index_omits_empty_constructor` (`:354`) and
`dependent_index_telescope_lifts_prior_index_in_motive_premise` (`:369`), are
both regressions this slice's own landing commit added (its stated "Why" cites
an "earlier-index binder" bug fix); they support AC1/AC4's mechanism but
don't map to a distinct numbered AC, so not separately cited as discharges.

## Not this node

No repair of the `D3`-question-2 gap (bare-name vs. applied-pattern witness).
Per the tracker-gap node's `D4`, that is a finding for the relevant slice's
future work, not a reopening of this landed candidate.
