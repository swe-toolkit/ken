---
id: LANG-DEPELIM-NESTED-COUPLED-INDEX-COHERENT-FRAME
title: "D2b predecessor #4 (elaborator dependent-elim capability): a nontrivial CROSS-STRUCTURE equality whose two sides are dependent results that refine DIFFERENTLY under the same indexed elimination false-rejects a structurally-exhaustive proof — the two sides land in DIFFERENT de Bruijn frames (`expected g0, found Eq .. @3`; single-elim `expected (Dg578 @2), found (Dg578 (cg69 @4))`). The D0 grid (evt_t9w0fnhq68p0) RULED OUT the derived index (Row B red with a direct index), the coupled-index motive (Row A green, trivial), and nesting (single-elim C red): the fix is the STRUCTURAL COHERENT-FRAME CLOSURE over the motive-rebase / context-telescope invariant (subsuming 68100a5cd + 128b6c000 per-site), NOT a 4th point-fix — every refined occurrence on BOTH sides of a nontrivial equality at a single indexed elimination rebased into ONE frame, gated on ACTUAL occurrence per-side (no over-rebase). Architect classification evt_6zjegefzv0am2, fix-direction ruling evt_45h2qab5nejg3."
status: merged
owner: language
size: M
gate: none
tier: T1
depends_on: []
blocks: [V3-FO-EMBEDDING-ADEQUACY]
github: null
origin: "Steward, 2026-09-03. D2b design hard-stop #5 for V3-FO-EMBEDDING-ADEQUACY. Classified by the Architect (evt_6zjegefzv0am2) a distinct, reachable elaborator-capability predecessor in crates/ken-elaborator — NOT a kernel change (the kernel correctly rejects; the elaborator must PRODUCE a well-typed term that typechecks against the unmodified kernel = a completeness/false-reject fix), NOT source-compensable (the correspondence is a REQUIRED architecture-B lemma so it cannot live in FoKripke), NOT an architecture-B refutation (the proposition is true by construction and the match i/match xs proof is structurally exhaustive). The Architect's provisional lean (the derived/coupled index through the nested elim) was FALSIFIED by the D0 grid (evt_t9w0fnhq68p0, fixture /tmp/d2b-hs5-grid-final.rs) and WITHDRAWN (§7a, evt_45h2qab5nejg3): Row B (RED, direct index, no dual_fin_to_nat) rules the derived index OUT; Row A (GREEN, trivial equality, same coupled index + nesting) rules the coupled-index motive OUT; the single-elim C substitute (RED at ONE match xs) rules nesting OUT. CONFIRMED CAUSE (from the flipping cells): the discriminator is a NONTRIVIAL equality whose two sides are DEPENDENT RESULTS refining DIFFERENTLY under the same indexed elimination (cross-structure dependent-result refinement), failing ALREADY at a SINGLE elimination depth — the two sides' refinements land in different de Bruijn frames (@2 vs @4), so the branch's equality type does not sit in the same frame as the refined goal. FIX DIRECTION RULED (evt_45h2qab5nejg3): the structural coherent-frame CLOSURE, not a point-fix (the grid falsifies any special extension of the derived-occurrence/nested-elim gate). §1a: HS5 self-ruled; next Research trigger = HS6; no research pull. §1b (Architect, entry 4): entries 1 (68100a5cd, direct coupled scrutinee occurrence), 2 (128b6c000, captured convoy binder), 4 (this) share ONE predicate — no single coherent de Bruijn frame across the refined goal; entry 3 (b22e62530 DUAL-VIEW) is the field-side sibling. FOLD-VS-DISTINCT: this is motive/goal-rebase-side (expected g0) and DISTINCT from the Adversary's queued infer-mode-RVar successor (evt_3gh2pkrw3ny8z, field-view/application-head side); confirmed no application-head field-retyping is involved. Coordinates re-measure at your build SHA; grid fixture /tmp/d2b-hs5-grid-final.rs on the held WIP 26dd33f25."
---

> # LANDED (Steward, 2026-09-03) — MERGED at origin/main `ab55f525c` (squash of
> # PR #3291). Blob-verified: both changed paths byte-identical to the reviewed
> # candidate `60ea656228` — `crates/ken-elaborator/src/elab.rs` blob
> # `c62506078`, `crates/ken-elaborator/tests/dependent_match_coherent_frame_
> # acceptance.rs` blob `7ce70319c`. Decision `dec_6fz8zcm7gfc3m` (QA 207/207 +
> # lib/doc + Architect design/soundness + CV grid 6/6, all exact-SHA). The
> # lieutenant reported squash `f69590d0b`, which is NOT in main's ancestry — a
> # pre-squash preview SHA; the actual landed squash is `ab55f525c` (confirmed by
> # BLOB, not SHA). `ken-ci` auto-close did NOT fire (`github: null`); Steward
> # flipped `status: active -> merged` by hand. This discharges the `depends_on`
> # for `V3-FO-EMBEDDING-ADEQUACY` (D2b), now RE-RELEASED.
>
> # OPERATIVE (Steward, 2026-09-03) — OPERATOR GATE RESOLVED: option (a) ACCEPTED,
> # the cast/J INDEX-REFINEMENT realization. This banner SUPERSEDES the HS13-FIRED
> # and HS12-RULED banners below for CURRENT state and is the AUTHORIZED mechanism.
> #
> # Chain resolution: HS12's "transport-free with two nested
> # existing homogeneous Elims" was FALSIFIED at build (HS13,
> # language-implementer evt_51nwxnxvbstt0) and the Architect
> # ESCALATED the reserved policy gate to the Steward (Architect
> # evt_3megf5mnmh90t, §7a reversal). PROVEN: sequential
> # single-scrutinee elimination of a coupled shared-index pair with
> # opaque coupling functions (dual_nth/dual_lookup) INTRINSICALLY
> # requires index-transport — whichever family is eliminated SECOND,
> # its homogeneous recursor must generalize the shared index and
> # strands the FIRST family's now-concrete constructor; no split-order
> # and no convoy removes it (the convoy only relocates the horn to the
> # next elimination, never terminates). Conflation corrected for the
> # record: "without-K-solvable" is NOT "transport-free" — without-K
> # bans axiom K but PERMITS J; Cockx/GMC coupled Fin/Vec-over-Nat
> # elimination is without-K precisely by SOLVING the shared-index
> # equation and TRANSPORTING along it (J on the homogeneous Eq Nat
> # equation).
> #
> # THE OPERATOR RULING (2026-09-03, direct to the Steward): ACCEPT option (a), the
> # index-refinement cast/J realization. Options (b) decouple dual_lookup's index to
> # plain Nat and (c) declare out-of-scope were NOT chosen.
> #
> # AUTHORIZED MECHANISM (Architect evt_3megf5mnmh90t point 3+5a; the sound
> # route): carry the homogeneous `Eq Nat` index equation (the Suc-injectivity
> # peel already done at elab.rs:2982-2996) and `cast`/`J` the stranded sibling
> # along it. This is the index-refinement route (install_index_refinements) the
> # chain RETIRED in pursuit of transport-freeness; completing it for the coupled
> # covering is a real ELABORATOR build. HOMOGENEOUS (Eq Nat, NOT
> # heterogeneous/John-Major); the equations are reflexive-after-instantiation so
> # the casts COMPUTE (a stuck cast is a build defect, not the design); SOUND with
> # ZERO trusted-base DELTA — `cast`/`J` are EXISTING K2 primitives
> # (ADR-0005 observational kernel; ken-kernel/src/lib.rs:15, term.rs:322-324
> # Cast(A,B,e,t)/J(m,d,e)). NO kernel change, NO new primitive. The gate the
> # operator resolved was the POLICY of leaning on observational transport for
> # catalog coherences — NOT a soundness or novelty question.
> #
> # §1b CLOSURE (Architect point 6, refining the HS12 §1b below): the defect across
> # HS10/11/12/13 is ONE — the chain assumed a transport-free realization was
> # achievable; for this coupled coherence with opaque coupling functions it is NOT,
> # and index-transport (cast/J on Eq Nat) is intrinsic.
> #
> # RELEASE: the language ring builds the cast/J index-refinement for the coupled
> # covering from 7b766fc89 (production clean except byte-preserved user Cargo).
> # Architect is the required soundness/design reviewer on the candidate (their
> # mechanism); CV if strict-resolution conformance applies -> Steward M1-M4 ->
> # lieutenant. A stuck (non-computing) cast or any ambiguity in re-enabling
> # install_index_refinements for the coupled covering HARD-STOPS to the Architect
> # elaborator-side, never as a new gate (the operator has ruled the policy). D2b
> # (V3-FO-EMBEDDING-ADEQUACY) stays HELD (WIP 26dd33f25 over 7b766fc89, statement
> # UNCHANGED) until this predecessor lands; the Steward RE-RELEASES D2b only after
> # it merges.
>
> # OPERATIVE (Steward, 2026-09-03) — HS13 FIRED: xs-first SWAPS the horn but does
> # NOT remove it. The HS12 xs-first mechanism (banner below) is NOT settled —
> # awaiting the Architect's HS13 mechanism ruling; TCB tripwire ARMED.
> #
> # language-implementer evt_51nwxnxvbstt0: the covering transposer WORKS (a real
> # rectangular two-scrutinee split-order selection, one xs-first tree, no
> # cross-order equality emitted), but the advised two nested single-scrutinee
> # Elims are NOT well-typed. i-first pins i before xs's neutral motive (the retired
> # HS11 error); xs-first pins xs before i's neutral motive (the new wall) — the
> # inner Elim's motive always abstracts the shared index AFTER the sibling is
> # constructor-pinned, so keeping dual_nth/dual_lookup OPAQUE is ill-typed
> # (DualNil : DualEnv a Zero vs the arbitrary-p inner motive) and SIMPLIFYING
> # reintroduces carrier eliminators (the retired common-frame error). Measured
> # /tmp/hs12-transpose3.log + transpose4.log; disposable
> # /tmp/hs13-xs-first-covering.diff. The only two resolutions the ring sees
> # BOTH cross the TCB boundary: (a) a kernel term rep other than sequential
> # single Elims (a simultaneous telescopic eliminator = new kernel primitive),
> # or (b) an equality/transport convoy (het-eq / J / Cast). HS13 ROUTED to the
> # Architect (Steward evt_46ew5qfjayc4h): if either is load-bearing => TCB
> # growth => Architect escalates to Steward => operator gate; if a THIRD
> # elaborator-only realization exists (simultaneous split lowering to existing
> # homogeneous Elims, no kernel-rep, no transport), the Architect rules
> # directly. §1a: HS13 is NOT a trigger; next mandatory = HS15. Ring restored
> # clean at 7b766fc89, idle pending the ruling. The HS12 banner below records
> # what was RULED and BUILT — do not read it as the final mechanism.
>
> # OPERATIVE (Steward, 2026-09-03) — HS12 RULED: coupled-covering xs-FIRST.
> # This banner SUPERSEDES the "D2 (RECUT) path-A/path-B" mechanism and the HS6+HS7
> # recut banner below: the HS8->HS12 coupled-inner-match sub-chain refined the
> # mechanism to coupled-covering SPLIT-ORDER SELECTION. Elaborator-only, no gate.
> # Durable capture of the Architect's convo ruling (in-thread is not durable).
> #
> # Architect MECHANISM RULING evt_2ry5x3wy5hq9p (mandatory 4th §1a advisory
> # evt_f290zks5qpjp in hand; Research confident POSITIVE). VERDICT: adopt
> # coupled-covering compilation, xs-FIRST split (research's original Rep-3,
> # verbatim). TCB tripwire does NOT fire — ZERO trusted-base delta, NO transport,
> # NO heterogeneous equality, existing homogeneous Elims. NO operator gate; the
> # Architect ruled directly. §1a HS12 DISCHARGED; next mandatory §1a = HS15 (the
> # "next MANDATORY = HS9" in the Sequencing section is STALE — HS8..HS12 have
> # since fired within this predecessor's build).
> #
> # FINALIZED MECHANISM (existing homogeneous Elims; build from the ruling):
> # OUTER DualEnv-Elim on xs, motive M(p, xs:DualEnv a p) := Pi (i:DualFin p).
> #   Equal (Option a) (dual_nth a p xs (dual_fin_to_nat p i))
> #                    (Some a (dual_lookup a p xs i))
> #   [the Pi-result motive Ken already PROVED it admits at HS12 — outer convoy
> #    is built and lawful; do not re-derive it]
> #  - DualNil (p=Zero): Pi (i:DualFin Zero)... DualFin Zero is uninhabited => the
> #    inner DualFin-Elim has NO cases, discharged (no no-confusion needed).
> #  - DualCons k x rest (p=Suc k), OUTER IH M(k,rest)=Pi(i:DualFin k).Coh(k,rest,i):
> #    lambda i. INNER DualFin-Elim on i:
> #      * DualFZ: both sides whnf to Some a x => Equal (Some a x)(Some a x) =>
> #        Proved (the HS6 leaf, unchanged).
> #      * DualFS k j2: both sides whnf to the recursive calls on (rest,j2); the
> #        goal reduces to Coh(k,rest,j2) = OUTER IH M(k,rest) applied to j2. Closed.
> # SOURCE-TO-TERM: the surface recursive call `dual_option a m rest j` compiles to
> # the OUTER eliminator's IH on the recursive field (re-anchor Fin -> Env:
> # structural descent rest < xs, sound because rest and j descend in LOCKSTEP on
> # the shared Nat), NOT a fresh Fin-anchored call. Two nested EXISTING Elims,
> # convoyed IH from xs's recursive field, per-branch whnf of the OPAQUE functions.
> # No transport/J/Cast/JMeq, no kernel primitive.
> #
> # TWO INVARIANTS (correct ORDER now): (i) the coupled sibling i lives in the
> # motive RESULT as a Pi over the FIRST-eliminated family (xs) — never concrete at
> # a generalized index (horn 1 = the retired i-first error) and never a generalized
> # motive PARAMETER (horn 2 = the retired HS10 Rep-2 error). (ii) dual_nth /
> # dual_lookup stay OPAQUE, whnf-reduced per-branch ONLY after BOTH scrutinees are
> # constructor-headed; never decomposed into carrier eliminators, never reduced
> # inside the motive (green slime). ONE sibling, SINGLE-family motive — NOT the
> # retired cross-structure common motive over the two functions' carriers.
> #
> # D0 (verify FIRST, elaborator-side, the analogue of the HS11 motive-result-Pi
> # check that PASSED): does Ken's TERMINATION checker accept the reordered
> # recursion (structural descent on xs, rest < xs)? Standard structural checkers
> # do. IFF Ken's checker specifically requires anchoring on the SOURCE-matched var
> # (Fin/i), that is an elaborator/checker-capability relaxation (still NOT
> # kernel/TCB, NOT transport) and the FALLBACK is the SIMULTANEOUS telescopic split
> # of the (i,xs) telescope, anchoring descent on the coupled (rest,j) the source
> # already exhibits (also Cockx-standard, homogeneous, no gate). Build xs-first
> # FIRST (reuses exactly the HS12-proven machinery); fall back to the simultaneous
> # split ONLY if the checker rejects xs-anchoring. Either way it returns to the
> # Architect elaborator-side, never as a gate.
> #
> # §1b STRUCTURAL CLOSURE (Architect, confirmed by the advisory): the defect
> # across HS10/HS11/HS12 is ONE — surface-order-bound covering compilation pinning
> # the shared index before the coupled sibling can be convoyed as a free-index
> # binder. The closure is coupled-covering compilation that CHOOSES the split order
> # (xs-first, or simultaneous telescopic) and convoys the coupled sibling,
> # retaining everything already built (outer convoy + Pi-result motive + convoyed
> # IH). NOT another per-direction convoy patch — this is the split-order-selection
> # capability. HS10 (sibling as motive param) and HS11 (i-first pins the index) are
> # the two ways to get the convoy's PLACEMENT and ORDER wrong; Rep-3 xs-first is
> # both right.
> #
> # RESUME: the language ring builds xs-first from 7b766fc89 (production clean
> # except byte-preserved user Cargo), with the termination-checker acceptance as
> # the D0 check first. Architect is the required soundness/design reviewer on the
> # candidate; CV if strict-resolution conformance applies -> Steward M1-M4 ->
> # lieutenant. D2b (V3-FO-EMBEDDING-ADEQUACY) stays HELD (WIP over 7b766fc89,
> # statement unchanged) until this predecessor lands; the Steward RE-RELEASES D2b
> # only after it merges.
>
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

> # RECUT — FINALIZED (Architect MECHANISM RULING evt_e5vhq4jzze7m, on research
> # advisory evt_5w7t0qxd5d7gh; §1b finding evt_614ka77qm02gz). VERDICT: lean (a),
> # ELABORATOR-ONLY, NO operator gate. The D2 fix direction below ("single neutral
> # motive frame closure") is SUPERSEDED for the RECURSIVE cross-structure case —
> # see "D2 (RECUT)" for the finalized mechanism. HS6's proved two-companion
> # leaf-discharge convoy is RETAINED. D2b stays HELD (26dd33f25) until this lands.
> #
> # THE §1b NAMED PREDICATE (HS6+HS7): a neutral-index single frame DECORRELATES
> # the two recursions. The coherent frame neutralizes the embedded eliminator's
> # index; abstracting the shared Nat n to a neutral j abstracts nth's Nat
> # position and lookup's Fin position INDEPENDENTLY, so every index-dependent
> # companion that needs the PEELED per-constructor equation is lost — HS6 the
> # impossibility discharge, HS7 the recursive IH (IH relates unrelated positions =
> # measured option 2; spine at neutral index = measured option 1). The two
> # recursions stay correlated ONLY while they share a REFINED index; neutralizing
> # it is exactly what breaks the correlation the frame was built to express.
> # Seam (elab.rs ~2200): path A (coherent-frame convoy) hands the raw whole-index
> # equality at the neutralized index (v0 = Eq Nat (Suc k) j); path B
> # (install_index_refinements, whnf-peel 2982-2996) peels via the kernel's
> # EXISTING same-constructor no-confusion but only when both endpoints are
> # constructor-headed. "Also convoy the recursive IH" would be a 3rd point-fix on
> # ONE predicate; the closure splits the two conflated jobs (D2 RECUT).
> #
> # TCB BOUNDARY — CONFIRMED ELABORATOR-ONLY, NO gate (Architect evt_e5vhq4jzze7m
> # resolving the hold evt_4nmfxvdnhzjep). No kernel/infer_j change, no new
> # primitive: the fix is architectural ROUTING of the recursive goal through the
> # EXISTING path-B peel. The flip-caveat (a kernel-provided injectivity primitive
> # for the recursive IH) did NOT materialize — the kernel's no-confusion already
> # exists and is already consumed elaborator-side. §1a: HS7 DISCHARGED (voluntary
> # advisory in hand); next mandatory Research trigger = HS9.

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

## D2 (RECUT) — the fix: split correlation (path A) from recursion (path B)

RULED by the Architect's mechanism ruling (evt_e5vhq4jzze7m) on the research
advisory (evt_5w7t0qxd5d7gh): lean (a), elaborator-only. The single neutral
motive frame conflated two jobs and DECORRELATED the two recursions; split them.

- JOB 1 — CORRELATION (relating the two eliminators): path A's HS6 convoy of the
  OUTER equation, parametric. KEEP it unchanged — it is proved and correct for the
  leaf / non-recursive correlation and the impossible discharge.
- JOB 2 — RECURSION: structural induction on the INDEX-REFINING structure (the
  `FokFin`), recursion index kept CONSTRUCTOR-HEADED per branch — path B
  (`install_index_refinements`, whnf-peel 2982-2996). Do NOT neutralize the
  recursion index. The alignment is what makes this work: `FokFin`'s constructors
  both produce `FokFin (Suc n)`, so matching the `FokFin` refines the shared `Nat`
  to `Suc _` in every branch. At `i = FokFinSuc j` the goal refines to
  `R (nth (Suc k)) (lookup (FokFinSuc j))`; both sides unfold in lockstep; path B
  peels `Eq Nat (Suc k) (Suc j) -> Eq Nat k j`, so the recursive IH
  `R (nth k) (lookup j)` applies DIRECTLY — the predecessor equation comes for
  free from structural induction on the refiner, no per-index point-fix, no
  representation change.
- ARCHITECTURAL CHANGE (elab.rs, ~2200 seam): ROUTE the recursive cross-structure
  goal through path B (refining-structure induction); KEEP path A's convoy for the
  non-recursive correlation / leaf discharge (HS6). Lift the neutralization OFF the
  recursion index and restrict it to the outer-equation correlation. This is the
  closure, not a 3rd point-fix on the shared §1b predicate.
- DO NOT reach for (b) motive-by-recursion — it is the fallback for NON-ALIGNING
  measures only and trades the peel for a CBV motive-reduction hazard (definitional
  equality does not imply executability under Ken's CBV whnf). `FokFin`-over-`Nat`
  ALIGNS, so (a) applies; (c) a simultaneous-induction/correspondence datatype is
  heaviest and unnecessary here. Recorded so a future reader does not re-derive.
- NON-REGRESSION GUARD (load-bearing — the trap the landed fixes hit): path A's
  convoy rebase stays gated on ACTUAL occurrence (`scrut_occurs` /
  goal-references-a-convoy-binder) — a blanket rebase corrupts a
  coincidentally-equal index (Vec `zip_with` result-length coincidence; Vec `map`
  goal that never mentions the scrutinee). Routing the recursive arm through path B
  must not weaken that gate on the retained path-A correlation.
- Completeness-side only; kernel UNMODIFIED (path B's peel already exists); produce
  a term the standing kernel recheck accepts. No FoKripke compensation. Zero
  trusted-base delta.

## Acceptance criteria

- AC-D0-GRID: SATISFIED. Rows A/B/C built on the held WIP as a disposable
  localization (no production edit); flip pattern A-green / B-red / C-red posted
  and ruled (evt_45h2qab5nejg3). The grid is the D2 acceptance instrument.
- AC-RECURSIVE-GRID (RECUT, mandatory D0 up front — proves the FULL closure, not a
  4th partial; Architect evt_e5vhq4jzze7m). Four cells: (1) the Row B RECURSIVE
  case (`FokFinSuc` / `DualCons`) turns GREEN — the IH applies via path B's peel;
  (2) single-elim C (HS6) STAYS GREEN — the leaf convoy is retained; (3) the
  cross-structure equality is genuinely ESTABLISHED — both sides unfold in lockstep
  at the refined index (the two sides CORRELATED, not option-2 unrelated
  hypotheses); (4) the genuinely-false reachable branch (`Some = None`) still
  REJECTS, and the direct / same-side controls stay green. The NON-DEGENERATE pair
  is (1) greening WHILE (4) rejects: the peel discharges by real structural
  refinement, not blanket admission. Build the grid first as a disposable
  localization (no production edit), then build D2 to green it.
- AC-GRID-GREEN-AFTER (prior HS5 grid, retained): after the fix the earlier
  A/B/C grid stays consistent — Row A GREEN, and the generic cross-structure
  Option-equality goal elaborates and the kernel accepts it.
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
- AC-CLOSURE-NOT-POINT-FIX (RECUT): the fix closes the §1b predicate by SPLITTING
  the two conflated jobs — path A convoys the outer-equation correlation (leaf /
  non-recursive, HS6 retained); the recursive cross-structure goal routes through
  path B's refining-structure induction (recursion index constructor-headed) — NOT
  by adding a per-index special case (which would be the 3rd point-fix on one
  predicate). A point-fix here, or neutralizing the recursion index, is a defect
  against the mechanism ruling evt_e5vhq4jzze7m.

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
  RE-RELEASES D2b only after this predecessor merges.
- §1a HISTORY: HS5 self-ruled; HS6 was the Research-advisory trigger (Arm 1
  ruled, elaborator-only, discharged); HS7 (recursive-IH neutral-index wall) is
  NOT a trigger and carries a VOLUNTARY Architect research pull (count
  unaffected). Next MANDATORY §1a Research trigger = HS9. See the RECUT-PENDING
  operative banner at the top for the HS6+HS7 named predicate and the recut.
