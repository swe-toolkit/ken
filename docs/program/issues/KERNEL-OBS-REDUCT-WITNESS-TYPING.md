---
id: KERNEL-OBS-REDUCT-WITNESS-TYPING
title: "The observational reducer synthesizes Refl as the proof witness in four reducts where the stated Eq relates two different types, so each reduct fails to typecheck (subject reduction fails) and a J over such a reduct goes stuck. Derive each witness from the reduction's own evidence, as spec 16 §3.2 and §4.1 specify"
status: ready
owner: kernel
size: M
tier: T1
gate: architect
depends_on: [KERNEL-OBS-EQ-AT-TYPE-RIGID-BOTTOM]
blocks: [KERNEL-OBS-NESTED-CAST-LINEAR, KERNEL-OBS-TYPE-EQ-STRUCTURAL]
github: null
origin: "Architect stop ruling evt_449gyxrrejte1 on LANG-SIBLING-GOAL-REFINEMENT Class A: a kernel reducer defect, not an elaborator one. Trust-root work. Steward-filed per COORDINATION section 2."
---

# Well-typed witnesses in observational reducts

## Objective

Every reduct the observational reducer produces typechecks at the type of
its redex, with each synthesized proof witness derived from the reduction's
own evidence.

## Settled inputs (Architect `evt_449gyxrrejte1`, read at `3cdf24654`)

- **The defect.** `cast_at_inductive` Phase 3 (`obs.rs:604-615`) builds
  each dependent sub-cast as `Cast(a_ty_j, b_ty_j, Refl(a_ty_j), v)` with
  `a_ty_j ≢ b_ty_j`, and discards `e` (`let _ = e;`, `:450`). `check.rs:331`
  types a `Cast` by checking `e : Eq Type A B`, so the reduct fails
  `infer`.
- **The spec requires the well-typed form.** `16-observational.md §3.2`
  gives the sub-cast evidence as `cong (Vec A) eq'`, where `eq'` decomposes
  `e`'s index equality injectively. §4.1 says the kernel synthesizes
  `pair-eq` by the singleton schema.
- **Four sites, one predicate:** a reducer-synthesized witness is ill-typed
  at the `Eq` it stands for.
  1. `eq_at_sigma` `:153`: `Refl(b1_p1)` for `Eq Type (B1 p.1) (B1 q.1)`.
  2. The `Eq`-at-inductive dependent-telescope conjunct `:299`:
     `Refl(b_ty_j)` for `Eq Type a_ty_j b_ty_j`.
  3. `cast_at_inductive` Phase 3 `:614`.
  4. `j_reduce` `:697`: `pair_eq = Refl(P a refl)` for
     `Eq Type (P a refl) (P b e)`.
- **Reach.**
  - Conversion never inspects Ω proofs, so this is a completeness defect,
    not a soundness hole.
  - Every consumer that re-types a reduct rejects it. That includes
    `j_reduce` itself, which runs `check::infer` on its `eq` (`:686`,
    `.ok()?`), so such a `J` goes stuck instead of computing.
  - The five `LANG-SIBLING-GOAL-REFINEMENT` Class A rows (f7 at contexts 12
    and 13, f4 at context 19) are rejected this way.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

Recut by the Architect in `evt_229qe9tgfetw1`, after AC-0 landed at
`aa51bf9d7`. **Sites 1, 2 and 4 only.** Production edits wait until
`KERNEL-OBS-EQ-AT-TYPE-RIGID-BOTTOM` merges. Until then, a witness at a Π
or Σ type is still ill-typed, because its base type is `Bottom`.

Each witness is a `J` along evidence the reduction already has, with an
ascribed motive, as in the elaborator's `build_index_type_cong`:

- **Site 1.** Built inside the Σ codomain along `Var(0) : Eq A1 p.1 q.1`.
- **Site 2.** Built along the Σ-telescope pair `E_j = (h_1, (h_2, …))` of
  the earlier conjunct proofs.
- **Site 4.** Built directly along `e`.

The base is the canonical proof of `Eq Type X X`. It is `Refl x'` if whnf
gives `Eq _ x' _`, and `tt` if it gives `Top`. Otherwise the reducer
returns `None` and fabricates no witness.

**J reads its recorded endpoints** (Architect `evt_143zpap46cmfr`, on
research `evt_4vn1evrd2cn38`; it replaces the `eq_formation` ruling
`evt_3qfzytddvnjkn`). A J's endpoints are an input recorded once in the
term, and no consumer re-derives them from `whnf`.
- **Kernel.** One reader, `j_endpoints(env, ctx, eq)`, over the eq
  *term*. If `eq` is `Ascript(_, T)` and `T`'s head is `Eq` after β, δ and
  let only (never `eq_reduce`), it returns `T`'s `(A, a, b)`. Otherwise it
  falls back to today's `whnf(infer(eq))` and `Eq` shape demand. `infer_j`
  (`check.rs:742-750`) and `obs.rs::j_nonrefl` both use it, and nothing
  else reads J endpoints.
- **Elaborator.** Surface `infer_j` (`elab.rs` ~9435-9446) derives
  `(A, a, b)` exactly as today and emits `eq` as `Ascript(eq, Eq A a b)`.
  Nothing else in the elaborator changes.
- **Kernel witnesses record their evidence.** Site 1:
  `Ascript(Var(0), Eq A1 p.1 q.1)`, weakened into the codomain. Site 2:
  `E_j`, already ascribed. Site 4: `Ascript(e, Eq A a b)` with the
  `(A, a, b)` that `j_endpoints` gave `j_nonrefl`.
- This widens J admission to recorded formations whose `Eq` reduces. The
  merge Decision reviews that widening explicitly.

**Site 3 (`cast_at_inductive` Phase 3) moves to
`KERNEL-OBS-TYPE-EQ-STRUCTURAL`.** Its `e` has no projections, because
`eq_at_type` leaves same-former compound pairs neutral. The five Class A
fixtures move with it, and so do the `e.1`/`e.2` witnesses of `cast_at_pi`,
`cast_at_sigma` and `cast_at_quot` (Architect `evt_4sj0kg0kd2qbz`).

## Acceptance

- **AC-0: done** (`evt_1r571tvqwhywe`, `evt_1xa46f5v6661`).
  - Four subject-reduction rows reach the `Refl` guard at `aa51bf9d7`.
  - The Class A consumer is `check_dependent_branch_body`'s
    `kernel_check_current`.
  - The nested-Cast baseline cost is measured.
- **AC-1.**
  - At sites 1, 2 and 4, `infer(reduct)` is convertible to
    `infer(redex)`, on the AC-0 rows.
  - Each of those sites also has a row where X is a Π or Σ type.
- **AC-2 (controls).**
  - Reverting any one site reddens its row.
  - A base that does not whnf to `Eq` or `Top` leaves the reduct neutral.
  - The 57-package census shows no conversion verdict change.
  - `trusted_base()` is unchanged.
  - The nested-Cast cost is re-measured against the AC-0 baseline.
- **AC-2b (legacy raw fixtures, Architect `evt_7k85x8en4fekz`).** Scope adds
  `crates/ken-kernel/tests/acceptance.rs` and
  `crates/ken-kernel/tests/k2c_series2.rs` (`j_dependent_motive_fires` and
  `j_constant_motive_still_reduces`, found by the every-target sweep).
  - `k2_j_nonrefl_reduces_not_stuck` moves to a typed redex with an
    ascribed motive, and J-cast still fires.
  - `k2_seam1b_eq_inductive_dependent_stuck` instantiates the `Vec` level
    and asserts the §2.2 reduct with a typed witness. If the spec makes it
    neutral, it asserts neutral and is renamed to match.
  - Each legacy ill-typed redex gets a control that asserts it stays
    neutral, with no fabricated witness and no panic.
  - `j_dependent_motive_fires` moves to `P(y,h) = Vec Nat y` over an opaque
    `base` (Architect `evt_4sj0kg0kd2qbz`). J-cast fires,
    `infer(reduct) ≡ infer(redex)`, and the full whnf ends at a neutral,
    typed `Cast` that checks against `infer(J)`. The Π-motive row moves to
    `KERNEL-OBS-TYPE-EQ-STRUCTURAL` as its motivating red.
  - Every `ken-kernel` test target and the kernel conformance suites are
    run scoped before QA, and every changed raw-fixture observation is
    listed. A change on a typed input is a stop to the Architect.
  - The nested-Cast series at depths 8/16/32/64 is reported beside the AC-0
    baseline. A super-linear jump is a stop to the Architect.
- **AC-2c (recorded endpoints, `evt_143zpap46cmfr`).** Scope adds
  `crates/ken-elaborator/src/elab.rs` (surface `infer_j` only).
  - The 57-package census returns to 57/57 accept, with no verdict change
    against `65af5c7cd`.
  - A J recorded at `Eq Nat (suc a) (suc b)` and a J recorded at
    `Eq Nat a b`, over the same `e`, each infer their own recorded result
    type.
  - An unrecorded raw J keeps today's type.
  - The span-55965 probe is posted with the candidate.
- **AC-3 (J admission fences).**
  - The committed third-field row (`Nat; Vec Nat x1; Vec Nat x1`) computes,
    and `infer(reduct) ≡ infer(redex)`.
  - `J` over a variable `h : Eq (Σ x:Nat. Vec Nat x) p q` with a dependent
    motive typechecks.
  - `J` over `h : Top`, or over any proof whose type has no `Eq` head, is
    still rejected with `BadEliminator`.
  - `J` over an `Eq` reached only by δ-unfolding a def keeps today's
    verdict.
  - The 57-package census shows no accept→reject flip. New accepts appear
    only where the old verdict was this exact `BadEliminator`.

## Stop conditions

- A kernel-internal "irrelevant witness" term former typed by its stated
  `Eq`. It would assert an equality without evidence, which is an axiom in
  the TCB. An operator question.
- Any change to what the kernel accepts, beyond reducts that now compute:
  stop to the Architect.
- A spec change: an operator question.

## Hard-stop inventory (§1b)

§1a count: 3 (Architect `evt_1m0z85kjtyzbe`; research `evt_4vn1evrd2cn38`).

1. J cannot eliminate a proof whose Eq formation reduces. `infer_j` reads
   `whnf(e_ty)` and demands the `Eq` shape, but `Eq` at Σ reduces to a Σ
   (keyed on reading an Eq formation after its own reduction). The same
   shape sits in `check`'s `Refl` rule (`check.rs:483`), which is left to
   `KERNEL-OBS-TYPE-EQ-STRUCTURAL`.
2. `cast_at_pi` projects `e.1` from a Π/Π type equality that has no Σ
   reduct (keyed on the absent structural decomposition of `Eq Type` at a
   same-former compound pair). Recut to `KERNEL-OBS-TYPE-EQ-STRUCTURAL`.
3. J's reading of a proof's `Eq` changed for proofs whose `Eq` reduces to
   another `Eq`, and 22 catalog packages rejected (keyed on which `Eq`
   formation, written or reduct, the eliminator reads). Shared predicate
   with entries 1 and 2: each `Eq` consumer re-derives the proof's `Eq`
   from `whnf`. Ruled: endpoints recorded once, `evt_143zpap46cmfr`.
