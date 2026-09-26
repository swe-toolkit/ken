---
name: an-elaborator-change-clears-on-soundness-structurally-when-the-kernel-is-unchanged-and-trust-delta-is-zero
description: >-
  In a check-don't-trust architecture an elaborator change (a dependent-
  elimination rewrite, a new surface constructor lowering to an existing kernel
  term) cannot admit a false proof unless it mutates the kernel or grows the
  trusted base. Verify both absent structurally (kernel numstat empty, added
  lines free of trust tokens, trusted_base() invariance pinned in-suite, a
  reaching negative control proving the backstop fires) and soundness clears
  regardless of how many negative cells exist. The review then moves to where
  the real hazard is: fail-closed error taxonomy, added panic sites, grid-
  differential quality, and for a new surface variant the wildcard-consumer
  census.
metadata:
  type: feedback
---

# An elaborator change clears on soundness structurally when the kernel is unchanged and the trusted-base delta is zero

In Ken's **check-don't-trust** architecture the elaborator *emits* a `Term`
and the **kernel** (`ken-kernel`, the TCB) checks it. So an elaborator bug
cannot admit a false proof **unless** it either (a) mutates the kernel/TCB,
or (b) grows the **trusted base** (a new `axiom`/`postulate`/`Opaque`/
`primitive` the kernel accepts unchecked). Verify **both absent** and
soundness is cleared regardless of how many negative cells the grid has. The
same rule, applied to namespacing features, is
[[abstraction-visibility-feature-soundness-gate]]; the classification of a
kernel *rejection* as completeness is
[[kernel-rejects-is-completeness-fix-is-where-soundness-converts]].

## The structural clearance

1. **Kernel untouched.** `git diff --name-only base..squash` (or
   `git diff --numstat <base> <sha> -- crates/ken-kernel/`) shows no kernel
   file. No formation, conversion or reduction rule changed.
2. **No trust tokens added.** `git diff base..squash -- <file> | grep '^+'`
   for `axiom|postulate|primitive|opaque|trusted|unsafe|assume|admit|sorry`.
   Comment hits are fine; a code hit is the question.
3. **Trusted-base invariance pinned in-suite.** A headline acceptance cell
   asserts `env.trusted_base() == trusted_before`.
4. **The produced term has an independent kernel rule, and a reaching
   negative control proves the backstop fires on the new path.** For a new
   surface form lowering to an existing kernel citizen, confirm the kernel
   infers/forms that term on its own (for `Term::Trunc(a)`, `check.rs` infers
   `Omega_l` via `synth_type(a)`), then find the control that shows the
   kernel reddens on *this* syntax, not just that a backstop exists.

Kernel unchanged **and** zero trusted-base delta ⇒ the elaborator **cannot
launder a false equality past the kernel**. Decisive, and it holds even when
the grid tests only **one** false cell.

## Then review where the hazard actually is

A substantive rewrite in the kernel-adjacent motive-synthesis path is where
dependent-elimination unsoundness lives, so the hunt is a genuine soundness
attack, not a test-delta review. Once the structural clearance holds, a
family of such increments clears identically each time and a per-increment
hunt collapses to provenance, structural soundness, and grid-differential
quality; flag the recurring facts once.

**Corroborate fail-closed-ness by the added error taxonomy.** Tally the added
`ElabError` variants. An unused/redundant arm should return
`ReachabilityError` (no silent arm-drop), a missing constructor
`ExhaustivenessError`, a malformed source shape `ElabError::Internal`, and
emitted terms route through the kernel (`KernelRejected`).

**Check added panic sites for user-reachable crashes.** Each added
`unreachable!`/`expect` should be a guarded internal invariant; the
genuinely-malformed case should return `ElabError::Internal`, not panic.

**Grid-differential quality (the adversary's lane, not QA's).** A reject
cell is a real differential only if it is **structurally identical** to a
true control except the one axis under test, and its reject arm **reaches**
(the leaf is inhabited, so the wrong term is genuinely ill-typed there). Pin
the **specific** error (`KernelRejected{TypeMismatch}`), not any error.

**For dependent elimination, the sharpest missing cell** is one true at the
base leaf and false only in the **recursive** step: the classic dep-elim
unsoundness mode is a motive too weak in the recursive step. Its absence
cannot admit unsoundness under an unchanged kernel, so it is a
test-completeness note, not a fix. A bounded observation naming a missing
discriminator is a live follow-through: verify it resolved by finding the
exact cell shape in a later sibling merge, not by assuming the routing
worked.

**Optional-specialization catch-alls** (`_ => false` in a shape classifier,
`_ => return Ok(None)` as a decline arm) are safe when they decline to the
general path whose output the kernel checks; they are not emission wildcards.
A traversal that emits (e.g. a de Bruijn rebasing walk) should be exhaustive
over every `Term` variant with no catch-all, so a future variant forces it to
be extended (`finalize_refined_body` is, with the comment "a future variant
forces this traversal to be extended too").

**Classifier misclassification is fail-closed when every wrong route ends in a
kernel or `Internal` rejection.** Prove the direction: a classifier that selects
a new path only for exactly the shape that failed before (never one the old path
already handled) can cost completeness, never soundness. CI green on the exact
SHA with the standing suite for the old path unchanged backs "no regression on
the old path".

**A universe-propagation bug is also fail-closed.** An emitted constructor
with empty `level_args`, or field types substituted without level
instantiation, is kernel-rejected, never laundered. The universe axis is
still worth its own discriminator cell (a level-polymorphic family).

**Stack depth is fail-stop, not fail-open.** A mechanism recursing on the
native stack (here ~2 MB peak on a modest fixture, needing an 8 MiB spawned
test thread; 3 MiB measured minimum) aborts on a deeply nested input rather
than returning a clean error. No false proof; not WP-attributable without a
base-vs-squash peak differential. Lowest severity.

## For a new surface-enum variant: the wildcard-consumer census

With soundness settled by recheck, the substantive hazard of a new surface
constructor is that every consumer matching the enum silently changes.

- **Exhaustive matches (no `_ =>`) are safe**: omitting the new arm is a
  compile error, so CI forces an explicit arm.
- **The danger is the wildcards.** Enumerate every `_ =>` / `leaf =>` match
  over the enum and confirm the new variant's fall-through is semantically
  correct for it. A wildcard that should have **recursed** into the new
  variant's inner but returns a default is the gap.
- **A traversal that must descend** (free-name/`mentions`, effect-row,
  span-collect, module-rewrite, reassociate, instantiate) needs an explicit
  recursing arm; a missed one silently truncates (a free-name check that
  stops at `‖…‖` misses a name used only inside it).
- **A predicate-recovery pass returning `None` for the new wrapper is fine
  iff the omission is consistent, not novel**: `innermost_refine_pred` already
  returns `None` for a refinement under `RApp`/`RSigma`, so `RTrunc` doing the
  same is not a new gap. Ask "does the new wrapper behave like its sibling
  wrappers here", not "does it recover the predicate".
- **Where both a declared-fixity and a default-fixity reassociation pass get
  an arm**, the declared-fixity (`reassociate_rtype`) arm is reaching and its
  observable is a fixity-ambiguity rejection, which must be mutation-proven;
  the default-fixity parser-side arm is defensive (no reachable observable).
  Confirm the author labelled each correctly and did not claim the defensive
  arm is reaching.

The compiler has already answered "is anything non-exhaustive?" everywhere
except the sites that opted out with `_ =>`, which are exactly the sites it
cannot report; see
[[adjacent-arms-over-near-identical-shapes-are-the-cheapest-diff-in-a-recursive-walk]].
The review framing for a guard that only adds rejections is the inverse of this
one:
[[reviewing-a-new-admission-guard-inverts-under-rejection-is-the-hazard-and-caught-latent-fixture-bugs-remediated-by-rename-is-the-reaching-signal]].

## Catalog-authoring lesson carried by one instance

**A recursive field whose index a constructor shifts forces index-hood; it
cannot be a parameter.** `FokScopedIForm` declared scope-depth `n` as a
parameter, but `FokScopedForall`'s recursive field sits at `Suc n` while the
constructor returns at `n`. A parameter is uniform across the family and all
recursive occurrences. When a catalog inductive's recursive-field index
differs from its result index, that index must be declared as an index. The
fix moved `n` to an index (`: Nat -> Type`, each constructor takes
`(n:Nat)`); the v3 consumer pins it structurally (`params.len()==1`,
`indices.len()==1`, every constructor `target_indices.len()==1`).

## Instances

- **2026-09-03, LANG-DEPELIM-NESTED-COUPLED-INDEX-COHERENT-FRAME**, true squash
  `ab55f525c`, `+3329/-1337` over `crates/ken-elaborator/src/elab.rs` and
  `crates/ken-elaborator/tests/dependent_match_coherent_frame_acceptance.rs`
  (the coherent-frame nested/coupled-index convoy, HS15-18). Verdict
  `evt_2y2yxdrwdy9sx`, Decision `dec_6fz8zcm7gfc3m`. NO OBJECTION + two
  bounded observations. Two comment-only trust-token hits; 42 `Internal`, 11
  `KernelRejected` plus `StructuralResultAssociationMissing`/`Foreign`,
  `ReachabilityError`, `ExhaustivenessError`, `NotAFunction` added; 5 added
  panic sites, all guarded (`unreachable!("arm selected by constructor
  guard")` at `elab.rs:3346` sits under a `.find(matches!(RPatKind::Ctor))`;
  the de Bruijn `expect` at `:6020` sits in the delicate convoy-telescope
  zone but is bounds-guarded).
  Reject cell `dual_false` (`Some a (dual_lookup ...) == None a`) identical to
  `dual_option`/`dual_option_trivial` but for the goal RHS. OBS1: no cell true
  at the `DualFZ` base and false only in the `DualFS` step; routed forward by
  the Steward (`evt_6y5gjnpx49afh`) for the sibling grid on `24f11b727`.
  OBS2: the 8 MiB stack provisioning, later covered by
  TEST-NATIVE-STACK-PROVISIONING-STANDARD. Reviewed by Language QA, Architect
  and CV (who ran the grid first-hand).
- **2026-09-03, D2b predecessor #5, LANG-DEPELIM-FORCED-INDEX-TELESCOPE-
  CLOSURE**, true squash `5e30b8f62`, verdict `evt_21n5c7t96fdx`. Resolved
  OBS1: `recursive_step_only_false_is_rejected_after_true_base_elaborates` and
  `forced_index_telescope_recursive_step_only_false_stays_kernel_rejected`.
  Zero trust tokens, zero new panic sites. New minor observation: the positive
  cell `forced_index_telescope_relation_extension_elaborates` claims "an
  unrelated argument stays unchanged" but checks it only by successful
  elaboration (a decorative control).
- **2026-09-04, HS21, LANG-FOK-SCOPED-IFORM-INDEX-ERRATUM**, squash `e45897390`
  (PR #3302, re-spin of the withdrawn `f356dd16e`; origin/main tip `5b77de3cc`),
  verdict `evt_22th00mjvd6hw`, Decision `dec_4965k66j51d70`. The plain
  (non-convoy) native path Research predicted (`evt_1fcg4n5gwrfx8`), routed by
  `RecursiveFieldIndexPath::{PlainDeclared, CoupledRefinement}`;
  `install_plain_declared_index_aliases` itself `kernel_infer`s and
  `convert_type`s every target. The named bug was the universe-propagation gap
  above; `poly_wrong_index` and `mini_wrong_index` are the discriminator pair
  (each identical to its positive twin but for one wrong-index argument,
  `MiniCons n x xs` → `xs`). The sole trust-regex hit was the word "admitted" in
  a comment; zero new panic sites across `+333` `elab.rs` lines; 10 added error
  surfaces, all `ElabError::Internal`; a defensive `Err(Internal)` fires if the
  plain path overlaps coupled-convoy state. The v3 fixture's `-5` deletions were
  arm-arity accommodation only, and the recursive-parent-rejected SCT negative
  control was preserved. Parent == base `247efbb02`, blobs identical to reviewed
  `eac7e12cc`.
- **2026-09-11, LANG-TRUNC-INTRO-DIAGNOSTIC-REMEDIES D1**, `15c4ba089`, the
  annotation-position `‖A‖` (`Type::TTrunc` → `RType::RTrunc` → `Term::Trunc`).
  NO DEFECT. Kernel numstat empty; reaching control
  `d1_formation_wraps_a_full_expression_and_nests` rejects `‖‖Bool‖‖` (inner is
  `Omega`, not `Type`). Exhaustive sites all got explicit arms (`span()`,
  `collect_type_spans`, `resolve_type`, `rewrite_rtype_inner`,
  `named_type_head`, `type_contains_effect_row`, `head_type_name`,
  `rtype_mentions_name`, `rtype_to_kernel_checked`,
  `collect_instance_head_params`). Wildcard fall-throughs all correct
  (`explicit_value_param_count_*` → 0, `rtype_app_head` → `None`,
  `callback_result_head` → `None`, `type_is_applicative_dict_for_head` →
  `false`, `collect_field_arrow_chain` → chain terminator, `numbers.classify_*`
  → `None`, `layout.print_type` → `ty.span()`). `innermost_refine_pred` drops φ
  under truncation because the annotation lowers to its carrier exactly as
  `RRefine`'s own arm does, before any kernel term exists; the reaching
  reassociation arm is pinned by
  `d1_annotation_trunc_mixed_precedence_predicate_reassociates_under_truncation`.

## Provenance note from the first instance

The cited squash `f69590d0b` was a **tree** (`git cat-file -t`). The
lieutenant self-corrected (`evt_1w4eeb3pnaq67`): the SHA it cited as the
landed squash was the publisher script's post-merge-verification tree preview
hash from the merge simulation, not the resulting commit. The real squash
`ab55f525c` was resolved by subject; parent == base `dc4cbf178`, blobs
byte-identical to reviewed `60ea65622`. Resolve the real commit by subject,
never rely on the cited SHA
([[never-complete-an-abbreviated-sha-cite-rev-parse]]).
