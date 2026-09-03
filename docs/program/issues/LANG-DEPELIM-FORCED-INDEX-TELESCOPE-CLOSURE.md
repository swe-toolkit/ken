---
id: LANG-DEPELIM-FORCED-INDEX-TELESCOPE-CLOSURE
title: "D2b predecessor #5 (elaborator dependent-elim completeness): under a FORCED (non-variable) scrutinee index, the generated motive frame omits the motive's return-Pi TELESCOPE arguments from the HS16-18 coherent-frame refinement — a structurally-exhaustive forced-index correspondence false-rejects at motive/method-type CONSTRUCTION (before any arm body), exact `TypeMismatch { expected ((Dg581 @8) @4), found ((Dg581 @8) (cg69 @7)) }`, Dg581 = DualEnv, cg69 = Nat successor. The B-green/C-red 2x2 D0 (evt_2h58340324ehh) RULED OUT motive-Pi-binding-per-se (B green: return-Pi-bound non-destructed DualEnv rebases under a BARE index) and inner-destructure rescue (C red with the ill-typed-body confound EXCLUDED), locating the reject at forced-index motive-frame construction. Fix = extend the merged coherent-frame convoy to carry return-telescope arguments as the THIRD sealed datum class under the SAME refinement substitution. Architect fix-surface ruling evt_7tmjw5hvpxrb9."
status: active
owner: language
size: M
gate: none
tier: T1
depends_on: [LANG-DEPELIM-NESTED-COUPLED-INDEX-COHERENT-FRAME]
blocks: [V3-FO-EMBEDDING-ADEQUACY]
github: null
origin: "Steward, 2026-09-03. D2b hard-stop HS19 for V3-FO-EMBEDDING-ADEQUACY, split out as a distinct predecessor WP on the Steward's scope call (language-leader requested + recommended, evt_7bj5axbwmey15; the Architect explicitly left distinct-predecessor-vs-in-place to Language/Steward, evt_7tmjw5hvpxrb9). Grounded: this is generic elaborator-completeness machinery in crates/ken-elaborator (elab.rs forced-index convoy / method-type construction), serves an arbitrary DualEnv repro (target-decoupled — the generic DualFin/DualEnv twin reds identically, evt_2d9tf2tezhktn), is soundness-bearing with its own true/false forced-index-telescope pair, and D2b's FoKripke proof is hard-blocked until it lands. It is the SAME shape as the just-landed predecessor #4 [[LANG-DEPELIM-NESTED-COUPLED-INDEX-COHERENT-FRAME]] (a completeness EXTENSION of that node's HS16-18 coherent-frame convoy), so it lands the same way: reviewed and landed as its own accepted unit rather than bundled into the D2b proof candidate (bundling generic elaborator completeness into a proof is the anti-pattern the predecessor split exists to prevent). NOT a kernel/TCB change (the kernel correctly rejects the ill-framed term; the elaborator must PRODUCE a well-typed term against the unmodified kernel = completeness/false-reject fix), NOT source-compensable (no source workaround permitted; the correspondence is a required architecture-B lemma), NOT proof authoring. The SEALED-TOTALITY over {leaves, index-equations, telescope-args} forecloses HS20 (a 4th coupled-datum class = compile error, not a runtime hard-stop), so this predecessor is engineered to TERMINATE the elaborator-completeness chain for the convoy. §1a: HS19 is NOT a research trigger; next mandatory = HS21 (Steward-authoritative count). §1b (Architect): HS19 shares ONE closure predicate with HS16-18 — the generated motive frame must carry EVERY scrutinee-coupled datum into the constructor-refined frame — recorded in the D2b issue inventory as a single entry, not four. Coordinates re-measure at your build SHA; held D2b WIP 268c630494; grid fixtures /tmp/d2b-hs19-2x2-final.rs, /tmp/d2b-hs19-generic-d0.rs."
---

> # RELEASED (Steward, 2026-09-03) to the language ring as D2b predecessor #5. D2b
> # ([[V3-FO-EMBEDDING-ADEQUACY]]) is HELD behind this node; it RE-RELEASES only on
> # the Steward's explicit re-release after this predecessor's soundness pair lands.
> # This node MERGES on Architect (design + soundness) + Language QA + CV (if the
> # change touches a conformance surface) via Steward M1-M4 -> lieutenant.

## What HS19 is (Architect ruling `evt_7tmjw5hvpxrb9`, D0 `evt_2h58340324ehh`)

D2b's `embedding_adequacy` proof needs a forced-index dependent match over a
coupled index whose motive RETURN type is a Pi telescope mentioning a
scrutinee-coupled carrier (`DualEnv a n` / in D2b, `FokObjectEnv sigma c n`). The
merged HS16-18 coherent-frame convoy ([[LANG-DEPELIM-NESTED-COUPLED-INDEX-COHERENT-FRAME]])
substitutes the arm's constructor refinement into the equality LEAVES and the
INDEX EQUATIONS, but NOT into the motive's return-Pi TELESCOPE ARGUMENT types.
Under a FORCED (non-variable) scrutinee index (`i : DualFin (Suc n)`), the forced
variable recovered per arm (`Suc n = Suc m => n = m`) is therefore never
substituted into the telescope argument, and the method type is rejected at
CONSTRUCTION — before any arm body — with:

```text
KernelRejected { TypeMismatch { expected ((Dg581 @8) @4), found ((Dg581 @8) (cg69 @7)) } }
```

`Dg581 = DualEnv` (live reverse lookup), `cg69` = Nat successor: the successor
method loses the predecessor-frame view of the Pi-bound `DualEnv a n`.

### D0 that fixes the cause (do not re-derive; verify sites at HEAD)

The B-green / C-red 2x2 (`evt_2h58340324ehh`) is decisive:

| scrutinee index | coupled carrier placement | result |
|---|---|---|
| forced `DualFin (Suc n)` | motive-return Pi-bound `xs`, no destruct | A: RED, exact `DualEnv` index mismatch |
| bare `DualFin n` | motive-return Pi-bound `xs`, no destruct | B: GREEN |
| forced `DualFin (Suc n)` | motive-return Pi-bound `xs`, inner `match xs` | C: RED, same mismatch |
| bare `DualFin n` | pre-match param `xs`, inner `match xs` | D: GREEN (landed `dual_option` cell) |

- **B green** rules out motive-Pi-binding-per-se: a return-Pi-bound, non-destructed
  `DualEnv` rebases correctly under a BARE index.
- **C red**, with the ill-typed-body confound EXCLUDED (both concrete `DualFZ` and
  `DualFS` methods elaborate green in isolation, so C's red is FRAMING not a false
  proposition), rules out inner-destructure rescue and locates the reject at
  motive-frame / method-type construction, before any arm body.
- The decisive axis is the FORCED scrutinee index. HS4 confound broken correctly:
  the fix is NOT a generic "extend the convoy to telescope args" funded from a
  positive-only pair; the grid separated forcing from motive-binding and
  destructuring, and forcing is it.

## Deliverables

**D1 — the sealed forced-index telescope-argument closure.** Extend the merged
coherent-frame convoy at the named surface so the forced variable(s) recovered per
arm are ALSO substituted into the motive's return-Pi telescope argument types,
occurrence-gated (only where the forced variable occurs), per-side, at CONSTRUCTION
time (before the method type is checked — an arm-body repair is structurally too
late, C proves the reject is pre-arm-body).

Fix surface (Architect-named from the merged frame `60ea65622` == `ab55f525c`;
RE-MEASURE the sites hold at your build SHA):

- `build_index_equation_convoy_body`
- `project_generated_index_equality_leaves`
- the single-goal-J motive builder `check_large_convoy_recursive_arm`

all in `crates/ken-elaborator/src/elab.rs` — the forced-index motive-frame /
method-type construction that already substitutes the constructor refinement into
leaves + index equations.

**SEALED-TOTALITY (the closure's teeth — required, not optional).** Encode the
carried-datum enumeration as a SEALED match with NO catch-all over
`{equality leaves, index equations, motive-return-telescope arguments}`, so a new
coupled-datum class is a COMPILE error, not a silent drop (COORDINATION §7
exhaustive-by-construction). Refactor the merged convoy so all THREE datum classes
dispatch through this ONE sealed enumeration, not three ad-hoc sites. This is what
makes it a structural closure and not a 3rd patch; it forecloses HS20 ("a 4th
coupled-datum class silently omitted").

**GUARDRAIL (HS4, already reaffirmed by the ring).** Dual-view transport:
substitute the forced variable to obtain the predecessor-frame VIEW of the
telescope argument — NEVER level transport. No equality relates an arbitrary `n` to
a `Suc`-forced frame; a level cast is the unimplementable direction HS4 ruled out.

## Acceptance criteria (predicate form; the fixtures ARE the acceptance)

- **AC-CLOSURE-SEALED.** The three carried-datum classes dispatch through one
  sealed enumeration with no catch-all; a dummy 4th member (or a removed member)
  reds the BUILD (demonstrated compile-error property), not a test.
- **AC-TELESCOPE-TRUE (positive, §7b).** The true forced-index correspondence
  elaborates: the HS19 theorem AND the generic `DualFin`/`DualEnv` twin (the
  target-decoupled repro), both routed THROUGH the forced-index telescope-argument
  arm, not only the leaves.
- **AC-TELESCOPE-FALSE (soundness control, §7b — non-degenerate, do NOT ship
  green-vs-green).** A forced-index theorem whose motive-TELESCOPE argument makes
  the recursive-step goal FALSE is REJECTED at kernel `TypeMismatch` — the
  telescope-arg analog of the z2135 `recursive_step_only_false` cell (which covers
  the leaves path, landed 7/7). The unchanged kernel rejects any false term
  regardless, so this sharpens the differential on the NEW machinery; it is not a
  gate that may be skipped.
- **AC-OCCURRENCE-GATED.** The telescope substitution fires only where the forced
  variable occurs, per-side — no over-rebase (the coherent-frame invariant the
  predecessor established, extended to the telescope class).
- **AC-ZERO-TRUST.** `env.trusted_base()` unchanged; zero new axiom / postulate /
  primitive / trusted token; kernel (`ken-kernel`) BYTE-UNCHANGED.
- **AC-NO-REGRESSION.** The landed `dual_option` grid stays green (cells B/D above,
  the 7/7 `recursive_step_only_false` leaves grid); FoKripke and prover FO verdict
  boundary unchanged. Green in CI (targeted `scripts/ken-cargo`; the workspace
  verdict is CI's).

## Reviewers

Architect (design + soundness — the fix-shape ruling is his; he re-reviews the
forced-index telescope pair on the exact SHA), Language QA, conformance-validator
IF the change touches a conformance surface. Adversary runs the post-merge M8b
hunt. Merge via Steward M1-M4 -> lieutenant.

## Capability tier

T1. The whole deliverable is a soundness-bearing elaborator-completeness extension
whose acceptance turns on a true-accepts / false-rejects argument through new
motive-frame machinery — the same tier as predecessor #4.

## Sequencing

Releasable now: its sole `depends_on`, predecessor #4
[[LANG-DEPELIM-NESTED-COUPLED-INDEX-COHERENT-FRAME]], is MERGED (`ab55f525c`). On
landing, the Steward explicitly RE-RELEASES [[V3-FO-EMBEDDING-ADEQUACY]] D2b (held
WIP `268c630494` reusable, statement unchanged, re-measure coordinates).
