---
id: LANG-REFINEMENT-PROOF-ERASURE
title: "The interpreter is strict in Unknown on pairs, the checked-core pair view rejects dependent Σ, and the native decoder has no refl and decodes a λ over an equation as relevant, so neither a subset-Σ proof nor a path-condition convoy can run. Erase every Ω-classified position (binder, argument, refinement pair forms), keyed on the classification, in lowering and in the interpreter"
status: active
owner: language
size: M
tier: T1
gate: architect
depends_on: [SPEC-REFINEMENT-SUBSET-SIGMA]
blocks: [LANG-PATH-CONDITION-EVIDENCE, LANG-REFINEMENT-SUBSET-SIGMA]
github: null
origin: "Language ring, so L1 keeps the runtime ring (operator 2026-09-17: ignored tests are top priority). Sequenced before LANG-PATH-CONDITION-EVIDENCE and widened to every Ω position (Architect evt_31p5m7pnr13fv: the convoy does not erase on the native path). Operator 2026-10-07: \"agreed, refinements should be real kernel types\" (OQ-refinement-representation DECIDED for the core subset Σ). Architect design and cut evt_30frdrmj45ehg. Steward-filed per COORDINATION section 2. W3. Plane ruled at AC-0 (Architect evt_7avananzmmmph): one kernel-classified erasure plan, carried in the package; waits for SPEC-REFINEMENT-SUBSET-SIGMA to carry the package section."
---

# A refinement value runs as its carrier

## Objective

Nothing classified at Ω is evaluated or represented at runtime. A pair
at a refinement type runs as its carrier value, an Ω-classified λ binder
or application argument is erased, and a relevant Σ is unchanged.

## Settled inputs (Architect, measured at `1153a9fc6`)

- `eval.rs` Pair arm: an Unknown component makes the pair Unknown, and a
  postulate evaluates to Unknown.
- `checked_core.rs:3578` `type_has_dependent_sigma` rejects dependent Σ in
  the checked-core pair view.
- Refinement-ness is the kernel classification `whnf(T) = Sigma(A,φ)`
  with `φ` classified at Ω, never a spelling.
- (Architect, measured at `c8e59b77c`, `evt_31p5m7pnr13fv`.)
  `decode_supported_body_term_after_tag` (checked_core.rs ~3086) accepts
  `var int_lit const constructor_ref elim lam app let absurd pair proj1`;
  `refl` falls to `UnsupportedTermShape`. A `λ (e : Eq ..)` decodes as a
  runtime `Lambda`. The decoder's `app` arm has no expected type, so an
  argument's classification must come from the function's Π domain.

- (AC-0, `evt_1d4kzrvzne0ma`, ruled `evt_7avananzmmmph`.) Neither
  evaluator carries the sort plane: `eval` is value-only and a closure
  drops its domain; the native decoder sees canonical bytes, and
  `delivered_sort_kind` is a partial reader with no whnf. The elaborator
  has a typed context and `kernel_infer_raw`/`whnf` at emission, and the
  interpreter reaches it (`ken-interp → ken-elaborator → ken-kernel`).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

One kernel-classified erasure plan per checked declaration body, computed
in `ken-elaborator` and consumed by both evaluators. The classifier is
`is_omega_classified(env, ctx, ty)`: the whnf of `kernel_infer_raw` of `ty`
is `Omega`. It is never recomputed from bytes or from a spelling.

- `OmegaErasurePlan { erased_subterms, erased_binders, collapsed_sigmas }`,
  node ids as preorder indices of the body's canonical encoding.
  `omega_erasure_plan(env, decl_body)` walks the closed body, extending
  `ctx` under binders:
  - a node whose inferred type is classified Ω goes in `erased_subterms`,
    and the walk does not descend;
  - a `Lam` or `Let` whose domain is classified Ω goes in
    `erased_binders`;
  - a `Pair` at `Sigma(A, B)` with `B` classified Ω under `ctx·A`, and a
    `Proj1` of such a pair, go in `collapsed_sigmas`.
- **Native.** `emit_package_from_env` writes the plan per admitted
  declaration as a package section (a hash input), in the form
  `SPEC-REFINEMENT-SUBSET-SIGMA` specifies. `validate_checked_core_package`
  checks it structurally and fails closed: ids in range, erased binders on
  `lam`/`let`, collapsed Σ on `pair`/`proj1`. `erasure.rs` lowers per the
  plan: an erased binder has no runtime parameter, an erased argument is
  not passed, a collapsed `pair` is its first component and a collapsed
  `proj1` is the identity. An `eq`/`refl`/`proj2` node outside an erased
  subterm keeps `UnsupportedTermShape`. A declaration with no plan
  refuses.
- **Interpreter.** The same function computes the plan from the
  `GlobalEnv`, and `apply_omega_erasure(term, &plan)` rewrites an erased
  subterm to `Const tt`, a collapsed `Pair(a, _)` to `a` and a collapsed
  `Proj1 p` to `p`. Erased binders are kept and receive `tt`.
- **One writer for body and plan** (Architect `evt_4ajjj60nhpqze`, on the
  `7843729eb` CI red). Native preparation replaced executable bodies with
  normalized ones but kept the plan of the original body. A single writer
  now stores a declaration's body bytes and its plan together. Both
  original emission and normalized native preparation call it. It
  derives the plan from that exact body in its producing environment and
  checks the canonical preorder count independently. A classification or
  count error refuses the declaration. Package slicing filters
  `omega_erasure_plans` by the same reachable declaration set.
- **Synthetic test packages.** Two test files are in scope:
  `crates/ken-elaborator/tests/nc16_primitive_value_lowering.rs` and
  `crates/ken-elaborator/tests/nc17_recursion_dictionaries_modules.rs`.
  Their hand-built, mock-typed builders may insert only
  `OmegaErasurePlan::default()`, each with a MEASURED/CLAIMED/THE GAP
  comment. A non-empty hand-written plan, a production helper and a
  `cfg(test)` export are all out. A row that needs Ω erasure is rebuilt
  as a kernel-admitted environment and gets its plan from
  `omega_erasure_plan`. The production missing-plan refusal is unchanged.

## Acceptance

- **AC-0.** Done (`evt_7avananzmmmph`).
- **AC-1.** A hand-built core row with a pair at `Σ Int φ` whose proof is
  an open hole lowers and evaluates to the carrier value on every path.
- **AC-2.** A hand-built relevant Σ (second component not at Ω) stays a
  pair on every path. `Σ Int Int` is the native evidence. A proof-first
  relevant Σ (`Σ(p : Eq Int 7 7).Int`) keeps its pair in the plan
  (`collapsed_sigmas` empty, `erased_subterms = {1}`) and on the
  interpreter (`Pair { fst: Neutral, snd: Int(8) }`). Its native leg pins
  the current fail-closed refusal by its
  `erased_omega_subterm_reached_computation` lane, not by prose, with a
  MEASURED/CLAIMED/THE GAP comment naming `LANG-NATIVE-SIGMA-ERASED-FIELD`
  (Architect `evt_55gdejjrzap0p`).
- **AC-2b.** A hand-built `λ (e : P). body` applied to a proof argument
  runs on both paths, where the argument is (i) a variable, (ii) an open
  hole, and (iii) at a `P` that is a transparent alias of an `Eq`. The
  hole is never evaluated, so the result is not Unknown, and the native
  runtime arity excludes the slot.
- **AC-2c.** A hand-built convoyed `if` and a convoyed Nat `match`
  (motive `λ y. Eq S s y → T`, applied to `refl s`) lower and run at
  parity with their unconvoyed forms, on both paths.
- **AC-3.** Runtime parity is unchanged.
- **AC-4 (mutation).** Keying erasure on the first component alone
  reddens AC-2 first at the proof-first plan assertion ("a computational
  codomain retains its Σ pair"). Report the native leg's result under the
  mutant separately; if it is the same refusal, it is a boundary pin, not
  AC-4 evidence. No production edit.
- **AC-4b (mutation).** Classifying by spelling (an `eq`/`refl` tag, or
  `delivered_sort_kind`) instead of `is_omega_classified` reddens
  AC-2b(iii).
- **AC-5.** A package with the plan section removed refuses at erasure; it
  does not lower relevantly.
- **AC-6.** On `Capability.Parsing.Parsing` and
  `Core.Classes.LawfulClasses` (three hot runs each), the summed time of
  `emit_package_from_env`, one `validate_checked_core_package` and
  `declaration_dependency_index` is no greater than base. The
  `emit_package_from_env` seam alone is reported, not gated (Steward frame
  decision on Architect `evt_1ha06r87d1ccp`: after R1-R5 the residual is
  the kernel classifier's own cost). The R1-R5 reductions leave the
  `omega_erasure_plans` bytes and the `core_semantic_hash` pins unchanged,
  and AC-4b still passes.
- **AC-7 (the `7843729eb` CI red).**
  - All six missing-plan rows from CI pass: four in nc16, plus nc17's
    `recursive_body_view_lowers_to_explicit_runtime_declaration_ref` and
    `checked_core_imported_value_crosses_an_accepted_var_capture`. So do
    the two nc17 siblings reproduced locally,
    `imported_declaration_ref_requires_exact_dependency_seed_identity`
    and `dictionary_construction_lowers_only_runtime_fields_to_record_values`.
  - `rt_c5_primitive_type_native_gate::indexed_int_convoy_native_refusal_waits_for_compared_native_rows`
    returns to `UnsupportedDependentMotive { family: Vec }`.
  - A compile-preserving mutation that skips plan recomputation on the
    normalized path reddens at count validation or at that row, and is
    restored. An in-range stale plan must also be caught.
  - The handoff enumerates every production writer of
    `semantic.declarations` or `omega_erasure_plans` under `crates/*/src`,
    with its disposition (including `checked_core.rs:1506` and `:2250`).
    It also lists every `crates/ken-cli/tests` suite that calls
    `build_native_program`, with its result.
- **AC-8 (normalized-body inference, Architect `evt_eq3kbx5fa9b5`).**
  `ken_kernel::normalize` drops the elaborator's motive ascriptions, so
  kernel inference fails on a normalized body's eliminators. The four
  `omega_erasure.rs` inference sites retry through `infer_node`. It
  re-ascribes bare-λ motives on a copy that is never stored, visited or
  counted.
  - Reverting site 1 alone to plain `infer` reddens the AC-7 row above and
    `px7n_nested_computational_eliminator` with the "Ω erasure eliminator
    motive" refusal, and is restored.
  - Local evidence is the targeted set, one suite per invocation (WIP
    audit `evt_2sqhv28v8nnqm`): `rt_c5_primitive_type_native_gate`,
    `px7n_nested_computational_eliminator`,
    `px7o_heterogeneous_eliminator_frames`, `nc16`, `nc17`,
    `lang_refinement_proof_erasure`, and the two mutations. The handoff
    lists the 38 `build_native_program` suites by name, and CI runs them.
    A suite green at `1b68dfd45` and red in the candidate's CI is a stop
    to the Architect, not a fix-forward.
  - No change to the kernel, `conv.rs` or the stored native body.

## Symptom inventory

Append one entry per Architect hard stop; never rewrite history.

1. The plan was keyed on preorder position and not bound to the body
   version it was computed on: the zipPrimitive plan from the 387-node body
   was applied to the 303-node normalized body -- keyed on symbol plus
   position (`evt_4ajjj60nhpqze`).
2. The normalized body lost the motive ascriptions the classifier needs:
   normalize strips `Ascript`, and the kernel infers motives -- keyed on the
   elaborated body version's annotations (`evt_eq3kbx5fa9b5`).
3. The normalized package inherits the original version's plans through
   `let mut normalized = package.clone()`, so a stale in-range plan
   survives a skipped write -- keyed on symbol (`evt_3brrb9b5a14hh`).

## Stop conditions

- `SPEC-REFINEMENT-SUBSET-SIGMA` lands without the plan's package
  section: stop to the Architect.
- Any kernel, `trusted_base()` or spec change.
