---
id: LANG-DEPELIM-NESTED-COUPLED-INDEX-COHERENT-FRAME
title: "D2b predecessor #4 (elaborator dependent-elim capability): a nontrivial CROSS-STRUCTURE equality whose two sides are dependent results that refine DIFFERENTLY under the same indexed elimination false-rejects a structurally-exhaustive proof — the two sides land in DIFFERENT de Bruijn frames (`expected g0, found Eq .. @3`; single-elim `expected (Dg578 @2), found (Dg578 (cg69 @4))`). The D0 grid (evt_t9w0fnhq68p0) RULED OUT the derived index (Row B red with a direct index), the coupled-index motive (Row A green, trivial), and nesting (single-elim C red): the fix is the STRUCTURAL COHERENT-FRAME CLOSURE over the motive-rebase / context-telescope invariant (subsuming 68100a5cd + 128b6c000 per-site), NOT a 4th point-fix — every refined occurrence on BOTH sides of a nontrivial equality at a single indexed elimination rebased into ONE frame, gated on ACTUAL occurrence per-side (no over-rebase). Architect classification evt_6zjegefzv0am2, fix-direction ruling evt_45h2qab5nejg3."
status: active
owner: language
size: M
gate: none
tier: T1
depends_on: []
blocks: [V3-FO-EMBEDDING-ADEQUACY]
github: null
origin: "Steward, 2026-09-03. D2b design hard-stop #5 for V3-FO-EMBEDDING-ADEQUACY. Classified by the Architect (evt_6zjegefzv0am2) a distinct, reachable elaborator-capability predecessor in crates/ken-elaborator — NOT a kernel change (the kernel correctly rejects; the elaborator must PRODUCE a well-typed term that typechecks against the unmodified kernel = a completeness/false-reject fix), NOT source-compensable (the correspondence is a REQUIRED architecture-B lemma so it cannot live in FoKripke), NOT an architecture-B refutation (the proposition is true by construction and the match i/match xs proof is structurally exhaustive). The Architect's provisional lean (the derived/coupled index through the nested elim) was FALSIFIED by the D0 grid (evt_t9w0fnhq68p0, fixture /tmp/d2b-hs5-grid-final.rs) and WITHDRAWN (§7a, evt_45h2qab5nejg3): Row B (RED, direct index, no dual_fin_to_nat) rules the derived index OUT; Row A (GREEN, trivial equality, same coupled index + nesting) rules the coupled-index motive OUT; the single-elim C substitute (RED at ONE match xs) rules nesting OUT. CONFIRMED CAUSE (from the flipping cells): the discriminator is a NONTRIVIAL equality whose two sides are DEPENDENT RESULTS refining DIFFERENTLY under the same indexed elimination (cross-structure dependent-result refinement), failing ALREADY at a SINGLE elimination depth — the two sides' refinements land in different de Bruijn frames (@2 vs @4), so the branch's equality type does not sit in the same frame as the refined goal. FIX DIRECTION RULED (evt_45h2qab5nejg3): the structural coherent-frame CLOSURE, not a point-fix (the grid falsifies any special extension of the derived-occurrence/nested-elim gate). §1a: HS5 self-ruled; next Research trigger = HS6; no research pull. §1b (Architect, entry 4): entries 1 (68100a5cd, direct coupled scrutinee occurrence), 2 (128b6c000, captured convoy binder), 4 (this) share ONE predicate — no single coherent de Bruijn frame across the refined goal; entry 3 (b22e62530 DUAL-VIEW) is the field-side sibling. FOLD-VS-DISTINCT: this is motive/goal-rebase-side (expected g0) and DISTINCT from the Adversary's queued infer-mode-RVar successor (evt_3gh2pkrw3ny8z, field-view/application-head side); confirmed no application-head field-retyping is involved. Coordinates re-measure at your build SHA; grid fixture /tmp/d2b-hs5-grid-final.rs on the held WIP 26dd33f25."
---

> # OPERATIVE (Steward, 2026-09-03, from Architect evt_6zjegefzv0am2 +
> # fix-direction ruling evt_45h2qab5nejg3). D2b predecessor #4. Elaborator
> # dependent-elimination capability, crates/ken-elaborator ONLY. A COMPLETENESS
> # fix (the kernel is unmodified and correctly rejects the current ill-typed
> # term); the elaborator must maintain a coherent de Bruijn frame across both
> # sides of a nontrivial equality so the structurally-exhaustive proof
> # typechecks.
> #
> # D0 GRID IS DONE and the fix DIRECTION IS RULED: the STRUCTURAL COHERENT-FRAME
> # CLOSURE, not a point-fix. Do NOT add a 4th special case (the grid falsifies
> # both the derived-index and the nesting extensions). Build the closure to the
> # invariant in D2; the grid + the landed regression suites are the acceptance.
>
> D2b (V3-FO-EMBEDDING-ADEQUACY) is HELD (WIP 26dd33f25 over main b5cad7322,
> byte-preserved Cargo.toml, statement unchanged) until this predecessor lands. On
> landing the language ring rebuilds architecture B's embedding-correspondence
> lemma against the fixed elaborator, produces the D2b candidate, and the
> Architect is the required soundness reviewer on it.

## One-line objective

Close the coherent-frame invariant in the dependent-match refinement machinery:
after an indexed `match` refines the scrutinee, the refined goal — including EACH
side of a nontrivial (cross-structure) `Equal`/`Eq` whose sides reduce through
different dependent structure — is expressed in ONE de Bruijn frame, so a branch
term typed by one side's refinement is convertible against the refined goal.
Gated on ACTUAL occurrence per-side (no blanket rebase). This subsumes the
per-site invariant of the two landed rebase fixes rather than adding a fourth.

## The false reject and the confirmed cause

Established generically on the held rebased WIP (26dd33f25 over b5cad7322),
Fok-free (`/tmp/d2b-hs5-grid-final.rs`):

- The goal
  `Equal (Option a) (dual_nth a n xs (dual_fin_to_nat n i)) (Some a (dual_lookup a n xs i))`
  false-rejects under the structurally exhaustive `match i / match xs` proof with
  `KernelRejected TypeMismatch: expected g0, found Eq Dg67 cg68 @3`; the single
  elimination substitute rejects with `expected (Dg578 @2), found (Dg578 (cg69 @4))`.
- CONFIRMED CAUSE (from the D0 grid, not a guess): the discriminator is a
  NONTRIVIAL equality whose two sides are DEPENDENT RESULTS that refine
  DIFFERENTLY under the same indexed elimination — cross-structure
  dependent-result refinement — and it fails ALREADY at a SINGLE elimination
  depth. The `@2` vs `@4` tell: the two sides' refinements land in different de
  Bruijn frames, so the branch's equality type does not sit in the same frame as
  the refined goal. The refinement machinery is not holding ONE coherent frame
  across both sides of the goal.

## D1 — the D0 discriminating grid (DONE; it ruled the direction)

Built by the language ring (evt_t9w0fnhq68p0, fixture
`/tmp/d2b-hs5-grid-final.rs`); the Architect ruled on it (evt_45h2qab5nejg3) and
WITHDREW his provisional lean (§7a). The cells:

- Row A (TRIVIAL vs NON-TRIVIAL): trivial equality, same coupled index + nesting —
  GREEN. Rules the coupled-index motive OUT (with the Proved->Refl baseline
  correction: after the constructor exposes `Some x = Some x` the live goal is
  `Equal a x x`, a neutral Refl terminal, not Top/Proved).
- Row B (DERIVED vs DIRECT index): nontrivial equality with a DIRECT scrutinee
  index and NO `dual_fin_to_nat` — RED. Rules the derived/coupled index OUT.
- Single-elim C substitute (one `match xs`): RED. Rules NESTING out.

The flip pattern (A green, B red, C red) is why a special extension of the
derived-occurrence / nested-elim gate is falsified and the closure is the
remaining shape. The grid is also the acceptance instrument for D2.

## D2 — the fix: the structural coherent-frame closure (direction RULED)

Build the closure to the invariant in the One-line objective. Not a 4th special
case; the grid rules that out.

- WHERE (Architect, provisional on the closure — you build it, this names the
  machinery not the patch): the same motive-construction / `dependent_rebase_subs`
  / `install_index_refinements` path the two landed fixes live in. The closure
  generalizes their per-site rebase to a whole-goal, both-sides-of-equality,
  single-frame rebase. EXTEND the shared helper the convoy fix (128b6c000)
  already introduced rather than adding a parallel path.
- NON-REGRESSION GUARD (load-bearing — the trap both landed fixes hit): the
  closure MUST NOT over-rebase. The `scrut_occurs` / goal-references-a-convoy-
  binder gating exists because a blanket index rebase corrupts a
  coincidentally-equal index (Vec `zip_with` result-length coincidence; Vec `map`
  goal that never mentions the scrutinee). The closure's rebase stays gated on
  ACTUAL occurrence, now applied per-side across the equality, not a blanket pass.
- Completeness-side only; kernel unmodified; produce a term the standing kernel
  recheck accepts. No FoKripke compensation.

## Acceptance criteria

- AC-D0-GRID: SATISFIED. Rows A/B/C built on the held WIP as a disposable
  localization (no production edit); flip pattern A-green / B-red / C-red posted
  and ruled (evt_45h2qab5nejg3). The grid is the D2 acceptance instrument.
- AC-GRID-GREEN-AFTER: after the fix — baseline GREEN, Row B GREEN, single-elim C
  GREEN, Row A stays GREEN. The generic cross-structure Option-equality goal
  elaborates and the kernel accepts it.
- AC-NON-REGRESSION (hard, load-bearing): a red-turning mutation at the NEW closure
  site demonstrates the control has power, AND the full landed regression set stays
  green — 68100a5cd motive-rebase, 128b6c000 convoy, record-index, and the Vec
  `zip_with` / `map` suites (the over-rebase guard). Prove both; do not assert the
  guard by construction.
- AC-CORRESPONDENCE-GREEN: the architecture-B embedding-correspondence lemma over
  the intrinsic source (`fok_qobject_value` fail-closed `Option a` resolver related
  to the total `FokObjectEnv`/`FokFin n` lookup) elaborates against the fixed
  elaborator, with no invented carrier/default and no narrowing of validity.
- AC-ZERO-TRUSTED-BASE-DELTA: `env.trusted_base()` byte-equal before/after; the
  whole diff adds no axiom/postulate/primitive/Cast (kernel untouched). Measured,
  not asserted.
- AC-COMPLETENESS-NOT-SOUNDNESS: a mutation that mis-selects the rebase frame yields
  a COMPLETENESS loss (kernel recheck rejects), never an acceptance — the standing
  top-level kernel recheck is unmodified, so over-acceptance is impossible.
- AC-CLOSURE-NOT-POINT-FIX: the fix closes the invariant (every refined occurrence,
  both sides of a nontrivial equality, one frame) rather than adding a special case
  for the derived-index/nested-elim shape; a point-fix here is a defect against the
  grid + evt_45h2qab5nejg3.

## §1b — the coherent-frame invariant (Architect entry 4, CONFIRMED by the grid)

The D2b dependent-elim predecessor inventory:

| # | node | keyed on |
|---|---|---|
| 1 | `LANG-DEPENDENT-MATCH-MOTIVE-REBASE` (68100a5cd) | direct coupled scrutinee occurrence |
| 2 | `LANG-DEPENDENT-MATCH-CONTEXT-TELESCOPE-REBASE` (128b6c000) | captured convoy binder in goal |
| 3 | `LANG-DEPELIM-REFINED-INDEX-FIELD-DUAL-VIEW` (b22e62530) | field retyped to outer refined index (field-side sibling) |
| 4 | this node | both sides of a nontrivial cross-structure equality, one indexed elim |

Entries 1, 2, 4 share ONE predicate: the dependent-match refinement machinery does
not keep a single coherent de Bruijn frame across the refined goal — each landed
fix patched one coupling SITE. The grid CONFIRMED (the point-fix is falsified; the
closure is the remaining shape): the fix closes the invariant across every refined
occurrence, both sides of a nontrivial equality, at a single indexed elimination,
rather than adding a 5th special case. Entry 3 is the field-side sibling of the
same coherence failure.

## Contention check

crates/ken-elaborator only, on the held D2b WIP. No contention with lane 1
(crates/ken-runtime, ken-cli) or lane 3 (catalog/). Same file/machinery as the
landed 68100a5cd / 128b6c000 / DUAL-VIEW fixes; no other lane-2 node is in flight
(D2b itself is HELD behind this).

## Capability tier: T1

The whole deliverable is de Bruijn coherence reasoning over dependent
eliminations — closing an invariant three landed point-fixes each established for
one site, while proving the over-rebase guard holds. Reasoning-dense, not
mechanical.

## Gate, reviewers, sequencing

- gate: none.
- Reviewers on the candidate: Architect (soundness) + conformance-validator
  (conformance). crates/ only, no Librarian; no kernel/spec/FoKripke.
- Sequencing: D1 (the D0 grid) is DONE and the direction is ruled. The
  language-implementer builds D2 (the closure) to the invariant, proves the grid
  greens + the over-rebase non-regression; the candidate goes Architect + CV ->
  Steward M1-M4 -> lieutenant M5-M9. D2b stays HELD until this lands; the Steward
  RE-RELEASES D2b only after this predecessor merges. Next §1a Research trigger =
  HS6.
