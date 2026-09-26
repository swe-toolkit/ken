---
name: a-byte-inert-plane-still-regresses-if-its-unconditional-build-can-err
description: >-
  Proving a planner-only plane or validation seam byte-inert certifies its
  OUTPUT, not its computation. A build or validate that runs unconditionally in
  production is a live Ok->Err surface through every Err arm a valid program
  can reach. Census each new Err arm and classify it (by construction, already
  taken on base, dead under the default cfg, pre-source venue, upstream
  enforced, or live invariant); only a live invariant is a finding, and the
  safe design for a discarded plane is uniform Ok(None). Also covers the
  byte-inertness layering defect (codegen reading a suppressible stored field
  instead of the canonical derivation), certifying inertness without a build,
  and why an inert plane has no behavioral oracle.
metadata:
  type: feedback
---

# A byte-inert plane still regresses if its unconditional build can Err

A recurring Runtime/codegen shape: a change adds a **planner-only,
diagnostic, or validation plane** (or the first increment of a multi-step
"forward edge" feature, a **seam** that installs and validates a compiler-only
handle but consumes it in no edge) and proves it **inert**: no lowering
consumer reads it, and a test compiles the same fixture with the plane
populated and suppressed and asserts byte-identical executables.

That proof is real, and it is about the **output**. It says nothing about the
plane's **build**. If the build or its validator runs unconditionally on the
production compile path (not `#[cfg(test)]`, not feature-gated), every `Err`
it can return is a compile it can now refuse. Because the output is
byte-proven discarded, such an `Err` aborts a valid compile for **zero
benefit**: a live `Ok->Err` regression surface behind a correct "inert" claim.

## The rule

1. **Certify output inertness first**, then do not stop there (section below).
2. **Find where the build and validate run.** Unconditional production path
   means its `Err` arms are live.
3. **Census every production `Err` arm the change adds or re-fires, and
   classify each one.** Only the last class is a finding:

   | class | why it cannot refuse a valid program | how to confirm |
   |---|---|---|
   | by construction | the arm checks a field against what the same derivation set a few statements earlier, or re-runs the same pure function on the same inputs | substitute the derivation into the check |
   | self-consistent round trip | a register-then-lookup on self-identical coordinates, into a slot whose uniqueness a pre-existing guard enforces | trace the coordinates and the uniqueness guard |
   | already taken on base | the new call is `&self`-pure and the base path already computed the identical value with identical arguments | find the base call site; confirm purity |
   | dead in production | the compared values are bound equal under `cfg(not(feature = ...))`; the branch fires only under a test mutation | read the default-cfg binding, not just the `if` |
   | pre-source venue | a single call site runs over fixed bootstrap input before any user source loads; it either always passes or reds every test | locate the call site relative to user input |
   | established lookup | a defensive `.find(...).ok_or_else(unsupported)` matching the codebase's existing pattern, whose target formation guarantees | grep for the same lookup elsewhere; find the assert that names the target |
   | upstream enforced | an invariant enforced (not merely expected) upstream | find the enforcing check on this path |
   | **live invariant** | a valid program could violate it | this is the latent `Ok->Err` |

4. **The safe design for a discarded plane is uniform `Ok(None)` on any
   shape surprise.** Skipping loses nothing. A build that is mostly
   `Ok(None)` with a few `Err` arms names its own deviation: attack those
   arms. The fix is `Err` -> `Ok(None)`.
5. **Direction check.** A change that turns former `Err` arms into
   `Ok(None)` inert gates **shrinks** the refusal surface (the safe
   direction). A gate placed at **consumption** rather than formation keeps
   `planned == formed == base`.
6. **A carried prior finding is not new.** If an earlier hunt filed a latent
   `Ok->Err` on this plane, check whether the change altered that arm's
   **firing condition**, not merely touched the enclosing function, before
   re-raising it.
7. **Grounding boundary.** If the link from "the arm fires on shape X" to "a
   compilable program produces shape X here" cannot be closed by reading, and a
   repro is barred on the shared box, file **PLAUSIBLE, not CONFIRMED**: the
   exact arms and trigger shapes, what was grounded, the one unclosed link, and
   the two dispositions that settle it (name the upstream invariant that bounds
   it, or change the arm to `Ok(None)`). Green CI proves only that the current
   corpus does not trip the arm
   ([[a-green-census-proves-its-classifier-total-over-the-current-population-not-total]]).
   A finder can enumerate and classify the arms; the seat that files owns
   re-grounding the load-bearing one.

## Certifying output inertness without a local build

- `git grep` the new type or field **at the review SHA, not the worktree**
  ([[publish-a-coordinate-from-the-git-object-and-name-the-sha-you-read]]),
  and confirm zero references under `lowering/`. If a real lowering input
  is touched, confirm its value is unchanged.
- The gold standard is a shipped **Exact vs `SuppressForInertness`**
  control that asserts identical `plan_transport_hash`,
  `core_semantic_hash`, `artifact_hash`, `executable_hash` **and raw
  executable bytes**, and ties `suppressed_applications ==
  exact_applications` so the reach counter fires identically in both arms.
- A **whole-diff emission grep** is a cheap corroborator for a planner-only
  increment: added or removed lines calling `ins()`, `create_block`,
  `append_block_param`, `switch_to_block`, `seal_block`, `def_var` should be
  zero.
- The non-consumption tell is a struct with **all underscore-prefixed
  fields** (`ComposedReturnForwardRetAuthority { _plan, _return_body }`)
  received by an `_plan`-named parameter: formed, validated, stored, never
  read.
- Confirm the test harness (thread-local recorders, the `...Mutation` enum,
  the `with_..._mutation` guard) is behind a feature that is **not** in the
  crate's `default = []`, so the shipped compiler takes the clean path.
- If the change refactors a shared element type (tuple -> struct with a new
  optional field), `git grep` the field for any remaining destructure rather
  than leaning on CI.
- Do not assume the increment after an inert seam is the consumer. A
  multi-step feature can land two byte-inert increments (D1 seam, D2 planner
  proof) before the consuming one; read the scope line and the emission grep,
  not the increment number.

## The layering defect: codegen that reads a suppressible stored field

The inverse failure: an "inert" plane whose read **does** change emitted
bytes. It shows only at the final-codegen hash. The mechanism: a codegen
decision reads the **stored, mutable** plane field
(`plan.checked_ih_continuation_inheritances`), which a `SuppressForInertness`
pass empties, so a formed authority flips to `None` and the emitted bytes
change while `core_semantic_hash` does not.

The ruled fix is the **canonical re-source**: every codegen-gating consumer
reads the canonical derivation built from the inert plan
(`build_checked_ih_continuation_inheritances(plan)?`), and the stored field
is demoted to a validation-only well-formedness assertion (`stored ==
canonical`) that gates neither codegen nor any artifact. Validation runs
before construction, so ill-formedness mutations still reject. When reviewing,
recognise the canonical re-source as the fix, not a smell.

**A prior lesson that predicts a defect is a hypothesis to test against the
diff, not a verdict to file.** Read whether the change already contains the
fix before reporting the shape as live; filing a defect the diff already
fixed is a false alarm laundered from memory.

## An inert plane has no behavioral oracle

Nothing downstream emits the plane's values, so no behavioral test can be
sensitive to them. Absolute correctness rests on golden constants
(change-detectors, not oracles) and on a validator that **shares the
derivation** (a self-consistency check that cannot see a systematic error
the deriver and re-deriver share). The one genuine discriminator is a
**relational** control: clone a validated plan, perturb it (insert an
intervening binder, shift an index), and assert the derived value moves as
the claimed semantics require. Require one when the plane's claim is about
value correctness. This is a coverage boundary to state, not a defect, when
the plane is inert, the arms are defensive, and the relational control is
present.

## Instances

- **2026-08-26, RT-ITREE-CHECKED-IH-RESULT-SUCCESSOR**, landed squash
  `7a3f6935e` (content byte-identical to reviewed `bd68bd3d2`; dispatch base
  `1d94e4bcf` stale, real parent `b722f6927`). Plane
  `checked_ih_continuation_inheritances`; build and validate unconditional
  (`construction.rs:1231-1240`, `closure.rs:2091`). Two live-invariant
  multiplicity arms: `aggregates.rs:3422` ("more than one ordinary
  fresh-result capture destination", a base-case closure capturing two read
  constructor-children, `case C x y => \a -> g x y a`) and
  `aggregates.rs:3245` ("same zero-argument self-resumption more than once",
  a recursive result used twice). Verdict PLAUSIBLE: the arms fire on those
  shapes and `predeclared_boundary_closure_environment` admits every lexical
  closure with a function owner, but routing through a cross-specialization
  checked-IH transport was not grounded. Secondary: the inert build ran three
  times per compile and its final `build == build` equality is vacuous in
  production. evt_2ndjbyjdfwy1t (thread thr_3wd9rz5wqpdz7). Contrast
  [[a-partial-landings-regression-risk-is-decided-at-the-consumer-not-in-the-diff]],
  a CONFIRMED production `Ok->Err` with a valid-program repro; that repro is
  the whole severity difference.
- **2026-08-27, RT-CHECKED-IH-K-AVAILABILITY-LOCATOR**, landed squash
  `b76943684`, byte-identical to reviewed `5c9f894ac` (QA evt_3v9mha55f3r8r
  era; dispatch base `1451fad16` stale, real parent `af8b258ab`). Extended
  the same plane with `CheckedIhImmediateKBindingLocator`; all seven new
  arms by construction (the derivation always pushes exactly one locator
  with domain and origins copied from the same step; the re-derivation arm
  re-runs `exact_zero_argument_self_resumption` on identical inputs; the
  uniqueness arm keys on the full locator so steps sharing
  `environment_index = 0` do not collide). The prior multiplicity arms were
  untouched, so carried, not new. The relational discriminator was
  `InsertInterveningBinder` (index + 1, preceding provenance `Ordinary`,
  semantic K identity preserved); golden origins `305/303/318/316` are
  change-detectors only. CLEAN. evt_sbqx4692m06t (thread thr_6c72gkaaf7fbt).
- **2026-08-30, RT-COMPOSED-RETURN-FORWARD-RET-EDGE D1**, landed squash
  `f193b074e731b9aa436996c134a17d2aa37cf94e` (candidate
  `dd0b824272c3ccf4f96b525ab3e618b6a5e2ddd2`, range `11ce6f3aa..dd0b8242`,
  parent `67189a46c`). Seam `ActiveCarriedComputationalRetSink` installed
  into the per-frame stack and validated by register+lookup; the
  `return_body` block is created at the same site as base and its consumer is
  untouched. Round trip self-consistent (uniqueness from the pre-existing
  re-entry guard that short-circuits to `RecursiveBackedge` before the push;
  slot `None` at install; `>1 frame` fail-closes to `Unsupported`). The one
  new production call `case_body_occurrence(...)` is `&self`-pure and was
  already computed on base for the same case. No objection.
  evt_5865012wcz8qa (thread thr_1ec03w0vvdmdr).
- **2026-08-30, same feature D2**, seven-mode respin landed
  `e7caf60be0e87cf559508a26c4ecef84609a2c36` (range `d9beb69e7...41e46ccf`). A
  second byte-inert increment: it names the forward-Ret route but consumes it in
  no edge. Emission grep zero; `finish_composed_return_forward_ret_authority`'s
  field-mismatch branch is always-false under `cfg(not(feature =
  "px8-ds-test-support"))` and fires only under the test-only `WrongSink`
  mutation; Direct and non-governed transports return `Ok(NonApplicable)`, not
  an error. No objection. evt_2xv1cetn325r2 (thread thr_62g7qqh46e11f).
- **2026-09-04, same feature D3 (b2 increment 1)**, landed `46433f03d`: the
  layering defect above was found and fixed in the diff by the Architect-C
  ruling (evt_70qj45jjt8sqm / evt_40dme966hce0a); three former formation
  `planner_error` arms became `Ok(None)` gates; a production
  `.find(...).ok_or_else(unsupported)` in `tail_worker_body_is_ret_kmatch`
  matched the existing lookup at `aggregates.rs:11525` with its target named by
  the assert at `7856`. Details in
  [[a-carrier-promoted-to-a-live-ssa-edge-clears-when-the-ordinal-is-derived-and-boundary-trapped-and-the-diff-already-fixed-the-byte-inertness-your-prior-lesson-predicts]].
  The same canonical re-source was applied to a new collapsibility guard in
  [[a-preventive-fail-safe-guard-that-and-narrows-an-existing-collapse-condition-flips-no-arm-today-so-verify-monotone-no-new-risk-and-pin-the-guards-own-determination-with-a-non-degenerate-pair-carrying-a-forward-gate]].
- **2026-08-27, LANG-MOD-CANONICAL-PAIR-PACKAGE** (`40e7f1199`): the pre-source
  venue class. `capture_strict_builtin_names` went infallible -> fallible, but
  its single call site (`lib.rs:249`, `ElabEnv::empty()`) runs over fixed
  bootstrap `globals` before any user source, so every new `Err` arm is a
  bootstrap self-check CI catches wholesale; the `filter_map` -> `map` +
  `ok_or_else` change turns a silent drop into a loud error. See
  [[in-a-multi-bucket-partition-census-a-growing-per-item-vector-is-not-a-regression-until-you-locate-the-items-bucket-transition]].

Kin:
[[certify-a-merge-block-unification-refactor-by-enumerating-the-environment-divergence-axes-and-proving-each-vacuous-in-tree-or-the-intended-fix]]
(certify a lowering change by a structural invariant over the exact diff; here
the "vacuous" axis is proved by byte-inertness and the already-computed-on-base
argument, and there is no intended-fix axis).
