---
name: an-elaborator-acceptance-widening-clears-by-construction-and-the-payoff-is-sweeping-every-consumer
description: >-
  How an elaborator completeness widening (dependent-match index refinement,
  generated index evidence, a new checking arm, an Omega arm inside a shared
  callee) clears, and where it still hides a defect. Clear it by showing each
  new output is canonical at a whnf- or convert-confirmed type (so a
  conditional re-check is not load-bearing), by a byte-equivalence case
  partition against the pre-change path, or by a kernel re-check that forces
  the result plus orientation inherited from a validated sibling. Then the
  payoff: enumerate every consumer and call site of the widened function or
  shared premise at the review SHA; the one left on the old path is the
  finding, usually a completeness-only bounded observation.
metadata:
  type: feedback
---

# An elaborator acceptance widening clears by construction, and the payoff is sweeping every consumer

A family of dependent-match index-refinement increments (the D2b
predecessors and the Omega-arm series, 2026-08-27 to 2026-09-03) each widened
what the elaborator accepts. None was a soundness defect, and each clearance
used one of four arguments. The reusable finding in every one came from the
same move: enumerating every consumer of the thing that was widened.

## Four ways to clear the widened arm

**1. Every new output is a canonical inhabitant of a type the kernel's own
whnf or `convert_type` established.** Then the change is over-accept-safe
without depending on any downstream re-check, and a *conditional* re-check
(e.g. a `kernel_infer` gated on `add_hidden_equation`) is defense-in-depth,
not load-bearing: the change clears even on the path where it is skipped.
Enumerate every shape the widened entry can emit:

- `Eq → Refl(t)` guarded by `convert`;
- `Top → tt` when the goal whnf's to `Const top_id`;
- `Sigma → pair(fst, snd)` with `snd` typed at `subst0(cod, fst)`. `subst0`
  collapses the binder, so there is no raw-construct-under-a-new-binder hazard
  (the opposite direction from the eq_at_omega bug in
  [[a-binder-weakening-fix-that-grounds-against-one-sibling-leaves-an-equal-class-sibling-unswept-and-the-reductions-own-open-operand-test-ratifies-the-bug]]).

It never fabricates an unconvertible equality, so `app(elim, proof)` is
well-typed because each proof inhabits its whnf-confirmed premise type.
Contrast [[untrusted-layer-backstop-hole-for-omissions]] and
[[a-validator-whose-expected-value-is-its-own-builder-re-run]],
where the backstop *was* load-bearing and had a hole.

**2. A byte-equivalence case partition against the pre-change path.** Do not
judge a new arm as a monolith. Find what the input took before (for an
`RVar` in checking mode, the default `check` arm: `infer(expr)` +
`unify_types(expected, inferred)` + `Ok(core)`, the `_ =>` fall-through of
`fn check`), then partition the new arm's cases and mark each byte-equivalent
or divergent. When the **one** divergent case returns a canonical term (the
raw kernel variable, or a constructor applied to canonical arguments) at a
`convert_type`-validated type, soundness holds directly, which is sharper and
cheaper than "the kernel rechecks the whole term".

**3. An independent re-check forces the result, and orientation is
inherited.** When the widening's output is kernel-checked
(`assert_transparent_body_kernel_checks` → `kernel_check(body, ty)`) and a
`J` motive forces the result type to `cur_ty[new/old]`, a wrong transport is a
fail-closed refusal, never a miscompile. Add the second condition: each
site's orientation must be inherited from an already-validated sibling arm
(the same `old_idx`/`new_idx`/`h` the pre-existing `Cast` arm used there).
Absent either condition the same shape is a latent relaxation — exactly
[[a-ruling-that-widens-a-shared-map-names-only-the-consumer-it-was-about]],
where the second reader's contract lived only in an error string with no
backstop.

**4. On the consumption side, the leaf type and the proof type come from one
whnf.** When the widening installs refinements rather than synthesizing
terms (the consumption-side mirror of the synthesis closure), each installed
refinement is a proof-carrying `Cast`/`J`-cong whose proof is a projection
chain of the real premise sentinel. Check that each leaf's
`(index_ty, target, scrutinee)` comes from the *same* `whnf` that types the
projection: then the recorded leaf type is convertible to the proof's kernel
type, and a wrong refinement is a completeness loss (a silent no-op, or a term
the kernel rejects), **never** an acceptance. If the two come from different
normalizations, that argument is gone and the arm must clear by 1, 2 or 3.

**Atomicity across consumers, when a walker feeds several.** Each consumer
must build the complete leaf list (propagating `?` on an unsupported child)
before mutating state, so an unsupported child rejects the whole plan and
never leaves earlier leaves installed. Pin it on a test asserting the exact
walker diagnostic string (`unsupported_pi_beneath_sigma`), a genuine
differential.

Verify zero trust delta structurally in every case: no
axiom/postulate/primitive/`Cast` added, and any new function reads context only
(grep for `push`/`pop`/mutation and find none). See
[[an-elaborator-change-clears-on-soundness-structurally-when-the-kernel-is-unchanged-and-trust-delta-is-zero]].

## The sweep that pays: every consumer, every call site, at the SHA

A widening names the consumer it was about. Enumerate the rest.

- **A widening inside a shared callee reaches every call site**, not only the
  one the diff edits. `git grep <callee> <review-sha>`, map each site to the
  tests that drive it **with a discriminator** (direction mutations,
  reachability sentinels), and classify the rest by condition 3 above.
- **A shared function also reached by a guarded sibling caller leaks only as
  far as that guard's reachable shapes**, not as far as the guard's name
  suggests. Enumerate the whnf shapes the guard actually admits and confirm a
  preserved-rejection fixture pins each.
- **When a WP decomposes N consumers of a shared premise, grep for the N+1th
  consumer of the same premise** and check whether it was left on the old
  path. Reachability of that consumer may not be gated on the shape you
  assume.
- **A check-mode fix has an infer-mode sibling.** Grep every
  variable-resolution consumer of the refined state (not install/remove/
  contains sites) and ask which mode each input actually arrives in.
- **A widened classifier whose siblings stay narrow is intended when a green
  negative pin says so.** A dedicated `..._remains_type_only` test asserting
  the exact rejection is positive evidence the asymmetry is deliberate; do not
  file an asymmetry finding against it.
- **Trace the reused primitive, not only the callee.** When decision N widens
  a classifier by routing to a primitive an earlier decision validated
  elsewhere, the orientation check is on the primitive's call wherever it
  already lives.

**Enumerate at the review SHA, never the working tree.** A bare `grep` of
`try_reindex_cast` callers on a stale worktree found 2 sites; `git grep
try_reindex_cast e13df606a` found the real 4, and one containing function did
not exist under that name in the worktree. Undercounting call sites is how a
shared-callee widening's untested site goes unseen.

## Disposition: usually a bounded observation

A left-behind consumer in these increments was completeness-only (the kernel
backstops the `Cast`), its reachability was unconfirmed, and no executed repro
was possible (§12). Report it as a bounded observation with the precise open
question for the owning ring, not a filed defect
([[preventive-findings-are-unfalsifiable-so-keep-them-cheap]]). An asymmetry
the authors documented ("inference remains outer-refined by default") is
honest, not wrong. State the residual closure boundary explicitly (e.g.
closure is exactly `{Eq, Sigma, Top}`; a propext `Sigma` with `Pi` children,
the `Eq Ω` reduct, still false-rejects, in the safe direction). Do not
manufacture a soundness finding when the emitted term is kernel-re-checked
and the direction is inherited.

## Non-vacuity of the positive fixtures

Positives must drive the real generated path (a real dependent match through
`elaborate_decl` on `.ken` source), not the user-sugar path. A
one-call-at-a-time mutation should red only the matching fixture
(independent causality). In a 2x2 grid, the cell that falsifies the original
causal predicate is the control, not green-vs-green. See
[[the-population-that-exercises-a-property-can-be-the-population-that-refuses]].

## Instances

- **2026-08-27, LANG-INDEX-REFINEMENT-OMEGA-ARM D1**, landed `e13df606a`
  (byte-identical to reviewed `67ca8cbd9`; `elab.rs` blob `1b5d8c839`, ds5b
  blob `711f944da`; dispatch base `08914de05` was a stale base-move artifact,
  real parent `61c2fefa0`), reported to thread `thr_3ktdqsw5rvhqg`. CLEAN.
  `try_reindex_cast` gained an Omega arm (`build_index_omega_transport`, a
  direct `J`) reaching four call sites (`:1670` recursive-group sibling,
  `:3451` hidden-result, `:3525` constructor-field injectivity, `:3585`
  outer-binder convoy with `build_sym`); discriminators covered only `:3451`
  and `:3525`. Boundary, not bug, by condition 3.
- **2026-08-27, Omega-arm D2**, landed `ef91b8225`. CLEAN. `refine_branch_goal`
  (single caller) lifted the old cast triple into a producer-classified
  `BranchGoalRestoration` enum (`TypeCast` | `OmegaJ`) and reused the
  identical `build_index_omega_transport` (pure de Bruijn, no env/ctx, so
  deferring the call to apply time is safe);
  decisions 4 and 5 held Type-only by dedicated negative pins asserting
  `found Ω0`. `d2_omega_goal` was non-vacuous: pre-D2 the arm hard-errored.
- **2026-09-02, LANG-GENERATED-INDEX-EVIDENCE-CLOSURE**, squash `569712a4d`
  (PR #3249, `elab.rs` +4/-2), verdict `evt_2j4xehpb8w160`. NO OBJECTION.
  `synth_refl_proof`'s `Sigma` arm repointed to the Top-aware
  `synth_generated_index_evidence` (condition 1). The guarded sibling was user
  `Refl` (`RCon(SUGAR_REFL)`, gated by `refl_goal_originates_in_equality`);
  an equality-origin `Sigma` is the propext reduct with `Pi` children, which
  the widened arm still rejects, so `Top` never reaches user `Refl` (pinned by
  bare `Top` and bare `Nat` fixtures keeping "Refl expects an Eq-shaped
  goal"). Positives `closes_top_second_child` / `closes_top_first_child`.
- **2026-09-03, LANG-RECORD-INDEX-SIGMA-CLOSURE**, squash `4af6e16f4` (PR
  #3255, `elab.rs` +388/-110, `record_index_sigma_closure.rs` +253), verdict
  `evt_625k42qvwjdqz`. NO OBJECTION + one bounded observation. The walker
  `project_generated_index_equality_leaves` fed `refine_branch_goal`,
  `install_index_refinements` and `install_hidden_result_variable_refinements`
  atomically; `finish_dependent_elim` synthesized the hidden equation's proof
  with the already-cleared synthesizer instead of a raw `Refl`. The
  hidden-result premise, with identical arguments, was also consumed by the
  `result_refinements.push` feeding `transport_recursive_group_call_result`
  (`try_reindex_cast` → `build_index_type_cong` → `J` over the raw sentinel),
  left on the whole-`Eq` path; `hidden_group_result_refinement` fires for any
  recursive-group dependent match, so a record scrutinee reaches it. The WP's
  test `hidden_result_refinement_handles_a_reducible_record_scrutinee` covered
  only a whole-record index; a component-indexed result would be a silent
  no-op.
- **2026-09-03, LANG-DEPELIM-REFINED-INDEX-FIELD-DUAL-VIEW**, true squash
  `b22e62530` (PR #3268, `elab.rs` +115,
  `dependent_match_eigen_guard_localization.rs` +213), verdict
  `evt_3gh2pkrw3ny8z`. NO OBJECTION + one bounded observation. The arm
  `check_variable_with_index_views` selects by `convert_type` between the
  refined-alias view (`weaken(raw_refined_term, ctx.len()-install_depth)`)
  and the raw local binding. Partition (condition 2): no refinement for the
  position → `binding_term(position)`, byte-equivalent to `infer`; refined
  view converts → identical to infer+unify; neither converts → the documented
  fallback, identical; refined fails but local converts → the single
  divergence, returning `Term::var(actual_index)` at a convert-confirmed type.
  The 2x2 grid over (dependent refined field) x (context extension) had the
  no-extension dependent-field cell as the fail-before/pass-after fix and the
  extension-only cell as the control falsifying the original causal
  predicate; every cell asserted `trusted_base()` unchanged.
  The infer-mode `RVar` sibling stayed refined-only, documented by the
  authors. Provenance: the cited `aeb7ceef2` was a tree; the real commit was
  found with `git log --oneline --all --grep=<WP-ID>`, parent == base
  `c2bd9f4e5`, blobs identical to reviewed `7c9c2459e`
  ([[never-complete-an-abbreviated-sha-cite-rev-parse]]).
