---
id: V3-FO-EMBEDDING-ADEQUACY
title: "Author the embedding and prove embedding_adequacy (classically_valid of the translated form implies the source form) — the SECOND of the two theorems 23 section 4.4 requires before route FO may return proved"
status: active
owner: language
size: L
gate: none
depends_on: [V3-FO-KEN-LEVEL-CHECKER-AUTHORING, V3-FO-CHECKER-SOUNDNESS, LANG-INDEX-REFINEMENT-OMEGA-ARM, V3-FO-SORTED-EIGENPARAMETER-DERIVATION, LANG-RECORD-INDEX-REFINEMENT, LANG-DEPENDENT-MATCH-MOTIVE-REBASE, LANG-DEPENDENT-MATCH-CONTEXT-TELESCOPE-REBASE, LANG-GENERATED-INDEX-EVIDENCE-CLOSURE, LANG-RECORD-INDEX-SIGMA-CLOSURE]
blocks: []
github: null
origin: "Steward, 2026-08-22, discharging the framing debt surfaced when the FO D0 fork was routed to the spec enclave. V3-FO-CHECKER-SOUNDNESS is the FIRST of the two 23 section 4.4 theorems (merged); this node is the SECOND. The enclave D0 ruling (spec-leader evt_2enqgkgqwd2g5, from spec-author evt_3kefqcayzajq9) directed that this node be cut AFTER D0 landed, on the structural assumption, so it does not race ahead and silently assume a (b)/(c) kernel premise. Framed to ready 2026-08-22 as the interim lane-2 WP after checker-soundness completed; all coordinates measured at origin/main 6842689b. Steward-filed per COORDINATION section 2. RECUT by the Steward 2026-08-27 at origin/main b76943684, before release, without an operator or Architect ruling because nothing about the objective changed: D1 had LANDED (771eec449, 87f26d0d2, 215b88071, 1308e9ea0, 5ef0f0983; Architect-approved, Decision dec_7f4k3whvy9n8 resolved) while this node still read status ready with D1 listed as work and its artifacts declared ABSENT with zero occurrences. Re-measured every fixed input; every Ken-side line number had moved and the absence claim was false. The releasable remainder is D2+D3 only. Rust-side coordinates (fo_kripke.rs Carriers:500 AtomEnv:508 denote:517; prover.rs attempt_with_cert:316 attempt_fo_with_signature:574 emit_unknown_hole_fo_withheld:800) all re-verified UNCHANGED."
---

## Symptom inventory

1. A derivation family over a constructor-headed record index can be built but
   not dependently eliminated: generated refinement treats observational record
   equality as a primitive `Eq`/`J` witness after it has reduced to a Sigma of
   field equalities — keyed on the index equality's representation rather than
   the derivation relation.
2. Generated-index reflexive-evidence synthesis stratifies its terminal
   vocabulary — `Top` is admitted only at the outer entry
   (`synth_generated_index_evidence`, `elab.rs:1361`) and not inside the `Sigma`
   recursion (`synth_refl_proof`, `:1331`, which rejects `Top` at `:1356`) —
   keyed on the nesting depth of the evidence goal, so a reflexive record index
   that reduces through a `Sigma` to a nested closed-equal field (WHNF `Top`) is
   false-rejected. Fifth D2b predecessor; owned by
   [[LANG-GENERATED-INDEX-EVIDENCE-CLOSURE]] (Architect design HS1 — evidence
   SYNTHESIS side). LANDED/landing.
3. The CONSUMPTION-side sibling of entry 2: elaborator consumers of a
   record-index equality handle the `Eq` head but not its observational
   Sigma-decomposition. `install_index_refinements` peels only `Term::Eq` and
   falls through on the `Sigma`-shaped record-index equality, installing no
   per-component refinement (`gamma -> g0`), so a dependent inversion needs
   `fok_nth_form @14` (outer) where the constructor type says `@10` (local);
   `14-10=4` outer binders, both well-weakened, the absent operation is component
   refinement through the Sigma. Section-1b predicate NAMED (Architect
   evt_2ptgr3f2ef7c4): same structural gap, two consumers. Sixth D2b predecessor
   / Architect design HS2; owned by [[LANG-RECORD-INDEX-SIGMA-CLOSURE]]
   (structural closure: fix + audit all such consumers + per-site fixtures).

> # D2b RE-FROZEN 2026-09-02 (HS-fifth) — behind a FIFTH elaborator predecessor
>
> **This banner supersedes the RE-RELEASED banner below.** The re-release
> below anticipated it verbatim: "if the fold hits a genuinely new elimination
> stop, that is a FIFTH predecessor hard stop — reset clean, measure, route to
> the Steward; do not source-convoy it permanently." It did, at `acef50612`.
>
> The fold reached the generated-index reflexive-evidence synthesizer and
> deterministically false-rejected: `synth_generated_index_evidence` admits a
> top-level `Top` at `elab.rs:1361` while its `Sigma` recursion re-enters
> `synth_refl_proof` (`:1331`), which admits only `{Eq, Sigma}` and rejects
> `Top` at `:1356`. The reflexive `FokSequent` index reduces through a `Sigma`
> to a nested `Equal (List FokForm) Nil Nil` (WHNF `Top`) — exactly this path.
>
> **This is NOT a source-convoy stop and NOT a kernel/TCB fix.** Architect ruling
> evt_27wm95g6gvtje: a COMPLETENESS gap (false-reject), not a soundness hole —
> the assembled elim term is still `kernel_infer`-re-derived at `elab.rs:2740`,
> the kernel gate is the untouched soundness authority. Sanctioned generic
> repair: route the Sigma recursion through the Top-aware entry (two call-target
> swaps), closing `{Eq, Sigma, Top}` uniformly at every nesting depth. A bare
> `Top` arm on `synth_refl_proof` is FORBIDDEN (its other caller is user-`Refl`
> sugar, whose equality-origin guard must stay). The `fok_invert_atomlike:
> theorem -> fn` correction is valid and stays on D2b.
>
> The predecessor is [[LANG-GENERATED-INDEX-EVIDENCE-CLOSURE]] (`ready`,
> language, S/T1, `gate: none` — elaborator-only, no operator authorization),
> released to the language ring 2026-09-02. D2b is HELD at `acef50612` (held
> evidence reusable, statement unchanged) until it lands; the Steward EXPLICITLY
> re-releases D2b only after, and the Architect re-reviews the resumed proof.
> This is D2b design hard-stop #1 by the Architect's per-design-question count
> (the OOM was resource, the `theorem -> fn` was mechanical); §1a Research
> advisory fires at the 3rd, not triggered.
>
> UPDATE 2026-09-02 (HS2 / SIXTH predecessor): the Top-closure fix let D2b
> advance past HS1 to `fok_invert_atomlike`, which false-rejected — the
> CONSUMPTION-side sibling gap (Architect design HS2, Section-1b predicate named
> evt_2ptgr3f2ef7c4). `install_index_refinements` peels only `Term::Eq` and
> does not decompose the `Sigma`-shaped record-index equality, installing no
> per-component refinement. Predecessor [[LANG-RECORD-INDEX-SIGMA-CLOSURE]]
> (`ready`, language, M/T1, `gate: none`) framed and released — a STRUCTURAL
> CLOSURE (fix + audit every such consumer + per-site fixtures), not a point
> fix, depends_on the Top-closure predecessor. D2b stays HELD at acef50612
> behind BOTH; re-released only after the chain lands. §1a fires at the 3rd
> design HS; not triggered.
>
> # D2b RE-RELEASED 2026-09-02 — fourth predecessor landed, consumer probe green
>
> **This banner supersedes the RE-FROZEN banner below.** The fourth predecessor
> [[LANG-DEPENDENT-MATCH-CONTEXT-TELESCOPE-REBASE]] MERGED: exact candidate
> `361ad044` landed as squash `0202018065ee460cd3ec3d38ee2b9d83e13e86bd` on
> `main`, full CI green, all three routed blobs byte-identical, node `merged`
> (PR #3228, M7 closure `e93210c4`). A route-integrity STOP from the lieutenant
> (a `7bc920a4` tree from the PRIOR predecessor `a3ded668d`, not this candidate)
> was adjudicated: candidate `361ad044` tree `94b6fcb5` landed verbatim.
>
> **Consumer probe GREEN on the merged tree.** The generic context-telescope
> acceptance test
> (`crates/ken-elaborator/tests/dependent_match_context_telescope_acceptance.rs`,
> blob `97b13750` on `main`) carries the mandated pins — the unconvoyed positive
> `captured_env_follows_fin_index_refinement_under_match` (the original red shape
> now elaborates), the manually-convoyed positive oracle, the one-site-mutation
> REDs, the dependent-let member, and the AST inspection proving the three-deep
> `xs'/h'/z'` convoy telescope is carried in the motive, every method, and the
> final application in dependency order. It passed in the merge's full CI. This is
> the SAME standard as the 2026-09-01 re-release (predecessor generic acceptance
> test green on the merged tree); the actual FoKripke un-convoying is the ring's
> fold work below, not this gate. Basis: test identity + required pins verified by
> the Steward against `origin/main`; green established by the merge's full CI
> (local rerun withheld — CI owns workspace closure and this box's RAM was at
> ~1.6Gi available; forcing a local elaborator build risked an OOM-SIGKILL).
>
> **RE-RELEASED to the language ring 2026-09-02 from `bf8f37326`.** The ring now
> rebases/folds the held spine `70a291a96` and continues the EXACT unchanged
> adequacy theorem, past the `fok_coh_extend` captured-context-telescope stop the
> fourth predecessor cleared. Constraints still bind: do NOT re-index
> `FokDerivation`, add a Nat sequent code, split the record into List indices, or
> touch `fok_derives`/`fok_classically_valid`. If the fold hits a genuinely new
> elimination stop, that is a FIFTH predecessor hard stop — reset clean, measure,
> route to the Steward; do not source-convoy it permanently.
>
> # D2b RE-FROZEN 2026-09-01 (HS3) — behind a FOURTH elaborator predecessor (history)
>
> **This banner supersedes the RE-RELEASED banner below.** After the two
> predecessors landed and D2b was re-released, the proof advanced (spine folded,
> coherence chain, atom/forcing projections) then hit its THIRD consecutive hard
> stop in the same dependent-index elimination class: `fok_coh_extend`, where a
> captured `xs : FokObjectEnv … n` does not follow the constructor refinement of a
> `FokFin` index under `match j` (implementer measurement `evt_3b9k92cmkn5zh`).
> Mandatory hard-stop-3 Research advisory (`evt_240ycvyantgrb`) and the Architect
> ruling (`evt_mpmnxxh42r0z`) both classify it **(a) an elaborator completeness
> closure over the dependent context telescope — NO kernel/TCB, NO permanent
> FoKripke source convoy.** The shared predicate across all three stops:
> constructor index refinement is not propagated through every dependent consumer
> (record-equality representation, then motive/goal/IH, now the captured context
> telescope).
>
> **D2b is FROZEN at held checkpoint `bf8f37326` (tree `8ff4f3921`, base
> `436ac00e9`); FO stays `Unknown`** behind the fourth predecessor
> [[LANG-DEPENDENT-MATCH-CONTEXT-TELESCOPE-REBASE]] (elaborator-only: one typed
> branch-refinement plan transforming the transitive forward-dependency closure of
> the local context; framed+released 2026-09-01 from the Architect mechanism
> ruling). **Consumer gate for re-release:** after that predecessor lands, rerun
> the exact held FoKripke consumer at `bf8f37326` WITHOUT the manual convoy; only
> then does the Steward explicitly RE-RELEASE D2b from `bf8f37326`. No parallel
> source repair, no kernel escalation, no D2b movement before that gate.
>
> # D2b RE-RELEASED 2026-09-01 — both predecessors landed, consumer probe green (history)
>
> **RELEASED to the language ring 2026-09-01.** Both D2b predecessors are now
> merged: (1) [[LANG-RECORD-INDEX-REFINEMENT]] (kernel `eq_at_inductive` weaken,
> PR #3216) cleared the record-index MATCH elaboration; (2)
> [[LANG-DEPENDENT-MATCH-MOTIVE-REBASE]] (elaborator-only simultaneous rebasing of
> motive + constructor goal + direct IH) landed as `1189aa3a74bc21ba4d6377f73ee4ccb4c7e14544`
> on `main` `6c4fbfa8c` (PR #3224, node `merged`; both elaborator blobs
> byte-identical to reviewed `a3ded668d`). The exact consumer probe is GREEN on the
> merged tree: `ken-elaborator --test dependent_match_motive_rebase_acceptance`
> = 4 passed, 0 failed. The ring now rebases/folds the held spine `70a291a96` and
> continues the EXACT unchanged adequacy theorem from `main` `6c4fbfa8c`. The
> constraints below still bind (do NOT re-index `FokDerivation`, add a Nat sequent
> code, split the record into List indices, or touch
> `fok_derives`/`fok_classically_valid`).
>
> # D2b UNBLOCKED 2026-09-01 — kernel predecessor MERGED (history)
>
> **This banner supersedes the D2a-outcome status: D2a is DONE and the statement
> survives verbatim, but D2b hard-stopped at the derivation inversion.**
>
> Proving adequacy requires dependently eliminating `FokDerivation` over its
> compound record index `FokMkSequent gamma delta`. The current match compiler
> cannot: generated branch refinement treats observational record equality as a
> primitive `Eq`/`J` after it has reduced to a Sigma of field equalities. Six
> encodings fail (compound index, equality convoy as arg/param, projection
> equalities, separate-field/List equalities). Held green evidence `70a291a96`
> (177-line strengthened ledger/inversion spine) is reusable D2b material, NOT a
> candidate.
>
> **UPDATE 2026-09-01: the ELABORATOR-only hypothesis (Architect
> `evt_68t4wwrs274nh`) was FALSIFIED by four-probe measurement.** The real fix
> was a one-line kernel/TCB binder-hygiene COMPLETENESS repair in
> `eq_at_inductive` (`obs.rs` `weaken(&acc, 1)`), Architect-confirmed layer (c)
> as completeness-not-soundness (`evt_3f61wtca219hw`), operator-APPROVED, and now
> MERGED via [[LANG-RECORD-INDEX-REFINEMENT]] (on main at `ab28450fa` /
> `5b7466ce2`, PR #3216). D2b is UNBLOCKED and re-released; the held evidence
> `70a291a96` (177-line strengthened ledger/inversion spine) folds onto the
> landed kernel fix and the ring continues the EXACT unchanged adequacy theorem. **Do NOT re-index
> `FokDerivation`, add a Nat code for sequents, split the record into List
> indices, or touch `fok_derives`/`fok_classically_valid`** — that would
> compensate in the relation for an elaborator defect. The D2b theorem statement
> is UNCHANGED. Only after the predecessor's exact consumer gate is green does
> the Steward explicitly RE-RELEASE D2b; the ring then rebases/folds the held
> spine and continues the exact unchanged theorem.

> # D2 RECUT 2026-08-27 — THE LANDED D1 STATEMENT IS REFUTED
>
> **This banner supersedes the two below.**
>
> **`fok_embedding_adequacy_statement` (`FoKripke.ken:881`), landed by D1, is
> REFUTED under the relation it is stated over.** The language-implementer built
> an accepted certificate that exploits eigenparameter capture and derived
> `Bottom` from the statement in a kernel-checked Ken theorem, zero new trust
> (`evt_2yh515wg0mczy`, exact base `ef91b8225`). **No D2 proof and no D2
> fragment can be authored against the current relation, because the proposition
> is false.**
>
> **Architect ruling `evt_6hx31xvw9tqs2` REJECTED the current FO
> checker/derivation/adequacy interface as a semantic soundness gate**, and ruled
> it **not repairable by finishing the current proof** (base `ef91b8225`, tree
> `19e0543a4ac006b24b256a038e25e83f29894162`). D2 is item 6 of that ruling's
> six-item repair envelope: **re-establish adequacy ONLY over the corrected
> sorted/scoped relation.**
>
> **PRODUCTION IS UNAFFECTED AND NEEDS NO EMERGENCY CHANGE.** The rejection
> invalidates the proposed **theorem gate**, not the production verdict
> boundary. `attempt_fo_with_signature` (`prover.rs:562-604`) still returns
> `emit_unknown_hole_fo_withheld` — an audited `Unknown`, **never `Proved`** —
> when `quote_fo` + `find_certificate` + `check_cert` all accept. **The defect is
> real but LATENT, and the gate that contained it is the very theorem now
> rejected.** Say this whenever quoting "REJECT".
>
> **This node is now THIRD in a three-node repair sequence**, and its predecessor
> did not exist when the sections below were written:
> [[CORE-FO-CHECK-TREE-SORT-VALIDATION]] (sorted/scoped validation, both
> boundaries) → [[V3-FO-SORTED-EIGENPARAMETER-DERIVATION]] (the atomic lockstep
> increment: parameter-only sorted `ForallRight`, typed instantiation,
> `FokDerivation` and its reflection proofs) → this node.
>
> **Reflection of checker acceptance is not evidence that the reflected rule is
> semantically lawful.** That is why adequacy is last and why it cannot be
> attempted early: it is the only node in the sequence that makes a semantic
> claim.
>
> **WHAT IS OPEN, AND DO NOT PRE-DECIDE IT.** `fok_classically_valid q` is
> `fok_derives (⊢ q)` (`FoKripke.ken:876`), so the statement's **text** quantifies
> over the derivation relation without naming its constructors. Correcting
> `FokDerivation` in the predecessor changes what the statement **means** without
> necessarily changing what it **says**. **Whether the landed statement text
> survives verbatim is a measurement for `D2a` below, not an assumption in either
> direction** — the old instruction "prove it as written, do NOT restate it" is
> withdrawn as an unconditional rule, and a licence to freely rewrite it is not
> granted in its place.
>
> **The Omega/index hold recorded in the next banner is NOT the live blocker and
> was never the cause.** The ring ruled that out before this was found. That
> predecessor is discharged; this one is not.

> # D2 HELD 2026-08-27 behind an elaborator predecessor
>
> Architect `evt_pw69nxgxn99j`. **SUPERSEDED by the banner above; retained as
> record.**
>
> D1 has landed. **D2 hard-stopped immediately on release** and the Language
> ring is held. Eliminating the proof-indexed `FokDerivation` with
> index-dependent Omega evidence fails elaboration:
> `index refinement: ... not classified by a Type universe, found Ω0`
> (implementer hard stop `evt_5fxgv9eeqm68f`, leader route `evt_74q124wnb3zaf`; the
> minimal `Probe` reproduces, ordinary indexed-family and unindexed `FokCert`
> controls pass). The Steward routed it without diagnosing it
> (`evt_nrvb2atg0xay`).
>
> **The Architect ruled the route VIABLE** across two CUMULATIVE rulings — the
> detailed mechanism ruling `evt_1wnk1ek4s8sgj` and the concise status
> clarification `evt_pw69nxgxn99j`. **Neither supersedes the other.** The defect
> is in the elaborator, not here:
>
> - It is **not** prohibited elimination out of Omega. D2 eliminates
>   `‖FokDerivation s‖` into the Omega-valued denotation, and its method
>   eliminates the Type-inductive `FokDerivation s` into an Omega motive. Both
>   permitted.
> - It is **not** a re-representation fork and **not** a TCB question. Kernel
>   `J` already transports index-dependent Omega evidence and kernel-checks the
>   result.
> - No additional discriminator is needed.
>
> **Nothing in this node is re-cut.** The premise, the validity statement,
> `fok_classically_valid`, `FokDerivation`, and D1 are all unchanged and
> explicitly not authorized to change. What changed is that D2 now has a
> `depends_on` predecessor: [[LANG-INDEX-REFINEMENT-OMEGA-ARM]].
>
> Evidence commit `3f687a460f4399bd1204a03ca8cbb57cad75eb92` (tree
> `15f2d977e`, `+126`, blob `c9eefe4e5`, 2/2 passing; independently reproduced
> by the Architect) is **held transition evidence, not a D2 candidate**. Do not
> advance it toward one.
>
> **The ring resumes only on an explicit Steward re-release** after **both**
> increments of the predecessor land through publisher CI. A predecessor
> landing does not by itself release D2.

> # FRAMED 2026-08-22 — the second route-FO `proved` theorem, on the structural arm
>
> Route FO may return `proved` only once BOTH `23 §4.4` theorems hold. The first,
> `checker_soundness`, is merged ([[V3-FO-CHECKER-SOUNDNESS]], origin/main
> `ba20b4810`). This node is the second, `embedding_adequacy`. **It authors the
> denotation side and proves adequacy; it does NOT wire `attempt_fo` or emit
> `proved` — that verdict-flip is a separate downstream node** (see AC-3 and the
> `attempt_fo` fixed input).

> # D1 BUILD HANDOFF — Architect-ruled 2026-08-23 (authoritative build spec)
>
> D0 landed as a genuine representability hard stop (probe `4d5fd8cee`,
> QA-approved evt_2ty00dbtgy1xv): the plain `FokIForm` atom (bare `Nat`, no scope
> proof) has no total denotation, and the two invention workarounds were correctly
> refused. The spec enclave concurred on the fix's semantic faithfulness
> (spec-author evt_2q0tkxtca67e2 / spec-leader evt_447d5qssxrftr: scope-indexed
> quotation/denotation commutation, zero new trust), and the Architect ruled the
> component design (evt_4s0p5m234544h + D1 handoff evt_pxmzdg6kq6as; `§4.4:531`
> places the representation with the Architect). This section is the authoritative
> D1 build; it supersedes the abstract D-sequence below where they differ.
>
> COMPONENT DESIGN (all NEW/additive; nothing landed is re-cut):
> 1. `IForm Sigma n` — intrinsically-scoped source form. Atom variable = bounded
>    index `i : Fin n` (n = object-binder depth), NOT a bare `Nat`. Five
>    constructors mirroring the landed `FokIForm` (Bottom, Atom, Or, Imp, Forall),
>    scope-indexed. This IS the spec's `IForm Sigma`.
> 2. `denote_at` (internal): interprets `IForm Sigma n` under a length-n carrier
>    env; atom arm = total `Fin` lookup (no Option, no default, no invented
>    Bottom); Forall extends the env (n -> n+1); proof-hypothesis Imp introduces NO
>    object slot (n unchanged). Bottom / truncated Or / Pi into Omega / carrier Pi
>    into Omega = the four green D0 axes. Public `denote C rho f` derives from it.
> 3. `erase_n : IForm Sigma n -> FokIForm` — constructor-homomorphic, maps
>    `i : Fin n` to the same de Bruijn `Nat`, changes neither binder order nor atom
>    identity. Used ONLY on the target-embedding leg.
> 4. `embed Sigma f := fok_embed (erase_n f)` — reuses the landed `fok_embed`
>    verbatim.
> 5. `embedding_adequacy` statement (`§4.4:498-503`): `(Sigma)(C)(rho)(f : IForm
>    Sigma) -> classically_valid (embed Sigma f) -> denote C rho f`.
>
> THE TWO LEGS (spec-author's refinement, load-bearing): the DENOTE leg consumes
> the intrinsic `f` directly (`denote_at`, no erase); the EMBED leg uses erase
> (`embed = fok_embed . erase_n`). Adequacy consumes intrinsic `f`; the landed
> `fok_embed` + checker operate only on its forgetful image. Quotation-then-
> intrinsic-denotation is the identity up to kernel conversion.
>
> REUSED UNTOUCHED (a fold must NOT touch these): `check_cert`,
> `checker_soundness`, `FokForm`, `FokCert`, `fok_embed`, `fok_w_forces`
> (`FoKripke.ken:165-199`). The discharge composition (`§4.4:515-521`) type-checks
> with `checker_soundness` reused verbatim on `embed Sigma f : Form`.
>
> D1 SLICE (Steward's cut, per the Architect's recommended boundary): D1 = the
> apparatus (`IForm Sigma` + `denote_at`/`denote` + `erase_n` + `embed`) + the
> `embedding_adequacy` STATEMENT + the quotation-preservation STATEMENT, all
> zero-trust, with the D0 probe axes carried in as durable controls ON THE
> INTRINSIC FORM (the four passing axes carry over directly; the atom axis is
> re-expressed on `IForm Sigma n` — total-by-construction, the D0-refused
> workarounds stay refused). The D0 probe `4d5fd8cee` is ABSORBED INTO the D1
> branch (rebased/carried), NOT merged standalone, so no plain-form atom assertion
> lands only to be reshaped. The two PROOFS (adequacy: `classically_valid embed ->
> denote`; quotation-preservation: `denote` = the Pi-closed obligation up to
> conversion) follow as D2/D3. `§4.5`'s first slice (one sort A, one unary P, five
> source forms) is exactly this grammar.
>
> D1 ACs (sourced-from-source + the enclave's zero-trust conditions):
> - `denote` TOTAL BY CONSTRUCTION — atom carrier value admitted from the `Fin`
>   index into the length-n env; NO default, NO Bottom-on-None, NO invented carrier.
> - ZERO trusted-base delta (before==after) on every new declaration.
> - Connectives per `§4.4:489` / `16 §1.3` (the four green D0 axes).
> - SCT green by direct-subterm descent (scope index n->n+1 at Forall rides a
>   strictly-smaller subform).
> - Discharge composition type-checks with `checker_soundness` reused verbatim.
> - GATE: this does NOT license an FO `proved` verdict — held behind the
>   kernel-checked `embedding_adequacy` AND quotation-preservation TERMS landing
>   (later deliverables; the verdict-flip stays a separate downstream node, AC-3).
>
> Reviewers: Architect (required soundness reviewer — checker reuse untouched,
> `denote` total-by-construction, zero-trust, discharge composition sound),
> language-QA, Adversary (over-accept hunt in parallel). Capability tier: T1.

## Objective

Author a Ken-level embedding-denotation apparatus (`Carriers`, `AtomEnv`,
`denote`) and prove `embedding_adequacy`: that classical validity of the embedded
form implies the source form's denotation. This is the validity-to-Ken direction
`23 §4.4` needs for the discharge; the unused converse is out of scope (spec
`:520-521`).

## The statement, taken from the spec rather than restated

`spec/20-verification/23-prover.md:508-513`:

```
embedding_adequacy :
  (Sigma : Signature) ->
  (C : Carriers Sigma) ->
  (rho : AtomEnv Sigma C) ->
  (f : IForm Sigma) ->
  classically_valid (embed Sigma f) -> denote C rho f
```

Supporting definitions: `denote : (C : Carriers Sigma) -> AtomEnv Sigma C ->
IForm Sigma -> Omega` (`:496`, "interpret the `IForm` constructors by Ken's
connectives"); `classically_valid q := Derives([] => [q])` (`:465-466`);
`Derives(s) : Omega := || Derivation(s) ||` (`:458`, a truncation);
`Carriers Sigma` / `AtomEnv Sigma C` (`:356`, `:491-493`). The discharge
composition is `sound ... := embedding_adequacy ... (checker_soundness ...)`
(`:526-531`); trust boundary at `:539-544`.

## Why it is the cleaner of the two, and forces no kernel arm

The spec enclave's D0 analysis (spec-author `evt_3kefqcayzajq9`) established that
adequacy is **structural induction on the source `IForm` over the translation
clauses** — no certificate, and no rotation-prone second argument like the one
that made `checker_soundness` the SCT-expressibility crux. It needs no SCT and
forces no kernel `size_rel` change. Under the arm-(a) ruling
([[V3-FO-SOUNDNESS-SCT-EXPRESSIBILITY]]) each theorem resolves the same way —
structural elaboration — so this node does NOT depend on the SCT rotation fix
and does not gate on a kernel arm.

## Fixed inputs, RE-MEASURED at origin/main `b76943684` (2026-08-27)

> **The 2026-08-22 coordinates were measured at `6842689b` and are SUPERSEDED.**
> Every Ken-side line number had moved, and the absence claim below had been
> falsified by this node's own D1 landing. Re-measured by the Steward before
> release. Do not read the `6842689b` figures out of the `origin:` line.

**Reuse (already merged — do not re-author):**

| artifact | location | role |
|---|---|---|
| `FokIForm` | `FoKripke.ken:28` | the source slice (Bottom/atom/or/imp/forall) |
| `fok_embed` | `FoKripke.ken:300` | the embedding `K(Sigma) => forall w. w \|= f` |
| `fok_w_forces` | `FoKripke.ken:267` | per-world classical forcing translation |
| `FokDerivation` `:825`, `fok_derives`, `fok_classically_valid` `:877` | `FoKripke.ken` | the reflected proof-tree family + `classically_valid` |
| `fok_checker_soundness` | `FoKripke.ken:2268` | the FIRST theorem — the discharge composes with it |
| `TruncBar` (`‖…‖` / `\|\|…\|\|`) | lexer `:112`, parser `:2636` | truncation surface token — usable; `classically_valid` uses it |

**ALSO ALREADY LANDED — this node's own `D1`, merged 2026-08-23. DO NOT
RE-AUTHOR ANY OF IT:**

| artifact | location |
|---|---|
| `FokCarriers` | `FoKripke.ken:44` |
| `fok_sort_a` | `FoKripke.ken:48` |
| `FokAtomEnv` | `FoKripke.ken:59` |
| `fok_denote_at` | `FoKripke.ken:306` |
| `fok_denote` | `FoKripke.ken:332` |
| `fok_embedding_adequacy_statement` | `FoKripke.ken:881` |
| the D1 differential | `crates/ken-elaborator/tests/v3_fo_embedding_adequacy_d1.rs` |

**Still ABSENT, and the only thing this node has left to author:** the
kernel-checked PROOF `fok_embedding_adequacy`. Zero occurrences outside
`fok_embedding_adequacy_statement`. `FoKripke.ken:879-880` says so in its own
words — "The D1 adequacy statement. Its kernel-checked proof is the following
increment; this proposition adds no assumption or trusted declaration."

**The Rust reference to mirror (NOT to invent a semantics):**
`crates/ken-elaborator/src/fo_kripke.rs:517`
`denote(env, sig, f) -> Term`, five arms (`:518-540`): `Bottom -> bottom`;
`Atom(IVar i) -> P (Var i)`; `Or(p,q) -> Term::Trunc(...)`; `Imp(p,q) ->
Term::pi(...)`; `Forall(p) -> Term::pi(sort_a, ...)`. `Carriers`/`AtomEnv` are
Rust structs at `:500`/`:508`. The module doc (`:16-19`, `:514-516`) names
`embedding_adequacy` as "the reserved boundary, not built here" — this node
crosses it. **A differential harness exists** for the executable mirrors
(`tests/v3_fo_ken_level_checker_authoring.rs`: Ken `fok_embed`/`fok_check_cert`
vs the Rust `embed`/`check_cert`). The `denote` differential that D1 owed now
EXISTS, at `tests/v3_fo_embedding_adequacy_d1.rs`; it is the landed `AC-2`
control and it is an input to D2, not work to redo.

**`attempt_fo` gating** (`crates/ken-elaborator/src/prover.rs`): the `proved`
verdict is withheld UNCONDITIONALLY (`attempt_fo_with_signature:574-604`) — even
when quotation, `find_certificate`, and `check_cert` all succeed it returns an
`Unknown` hole (`:584-597`, `emit_unknown_hole_fo_withheld:800`), citing the
reserved `§4.4` home. **This node does NOT touch `prover.rs`.** Flipping the
verdict (replace the withhold at `:597` with the composed discharge term through
`attempt_with_cert:316`) is a separate downstream node that depends on BOTH
theorems having an approved kernel-checked home.

## Deliverables

> **THE RELEASABLE REMAINDER IS `D2` + `D3`. `D0` AND `D1` HAVE BOTH LANDED.**
> Start at `D2`. `D0` landed as the representability hard stop (probe
> `4d5fd8cee`), already folded into the component design above. `D1` landed
> 2026-08-23 across `771eec449`, `87f26d0d2`, `215b88071`, `1308e9ea0`,
> `5ef0f0983` — Architect-approved, Decision `dec_7f4k3whvy9n8` resolved. Their
> text is retained below as the record of what was built and on what terms; it
> is NOT an assignment. Re-authoring any of it is a scope defect, and the
> "confirmed ABSENT" claim it was written against is now false — see the
> re-measured fixed inputs.

**`D0` (LANDED) — the buildability probe. A HARD STOP HERE IS A COMPLETE RESULT.**
Elaborate a minimal file establishing, independently and reported separately:

1. `Carriers Sigma` and `AtomEnv Sigma C` are writable as Ken data (the
   `Signature`-indexed families the spec names), kernel-checked;
2. a Ken `denote : Carriers -> AtomEnv -> FokIForm -> Omega` mirroring the Rust
   five arms elaborates — in particular the `Or` arm's truncation (`‖…‖`)
   and the `Imp`/`Forall` arms' `Pi` into `Omega`;
3. the recursion in (2) over `FokIForm` passes SCT with the guard **inlined at
   each recursive call site**, not factored through a non-recursive helper.

**Report each separately.** If an arm is not structurally expressible (e.g. a
`denote` clause whose termination is non-structural), that is the binding return
condition below — a complete result, returned to the enclave + Architect, never a
silent kernel measure change.

**`D1` (LANDED) — author `Carriers`, `AtomEnv`, `denote`, and the `denote`
differential.** Mirror the Rust `fo_kripke.rs:517` five arms exactly. Extend the
differential harness with a `denote` differential row (render Rust `denote`
results to Ken source, convert, compare) — this is the `AC-2` control for the
interpretation. Delivered as `FokCarriers`/`FokAtomEnv`/`fok_denote_at`/
`fok_denote` plus `fok_embedding_adequacy_statement`, with the differential at
`tests/v3_fo_embedding_adequacy_d1.rs`. Consume these; do not rebuild them.

> **`D2` WAS RECUT 2026-08-27 on Architect ruling `evt_6hx31xvw9tqs2`.** The
> version below is superseded by `D2a` + `D2b`; its text is retained because the
> proof shape (structural induction on `FokIForm`, `Imp` and `Forall` the
> non-trivial arms) still applies once the relation is corrected. **The
> instruction "prove it as written, do NOT restate it" is withdrawn** — that
> instruction is what the refutation falsifies.

**`D2a` (FIRST, AND IT MAY HARD-STOP) — re-measure the landed statement against
the corrected relation.** After [[V3-FO-SORTED-EIGENPARAMETER-DERIVATION]]
lands, determine whether `fok_embedding_adequacy_statement` (`FoKripke.ken:881`)
is **true as written** over the corrected `FokDerivation`, or whether its text
must change. **Report which, with the reason.** Re-run the refuting certificate
from `evt_2yh515wg0mczy` against the corrected relation and show it no longer
witnesses `fok_classically_valid` for that form — that is the concrete evidence
the refutation is closed, and a `D2a` that reports "the text survives" without
it is incomplete. **If the statement must be restated, that is a complete
result** returned to the Steward and Architect, not a licence to edit it inside
this deliverable: D1's approval bound that exact form.

**`D2b` (THE WORK) — prove adequacy over the corrected relation.** Structural
induction on `FokIForm`, on the statement `D2a` established. **Adequacy may be
proved ONLY over the corrected sorted/scoped relation** (repair-envelope item
6). A proof that composes with a `fok_checker_soundness` still reflecting the
old permissive rule is the rejected gate rebuilt, not the repair.

<details><summary>Superseded D2 text, retained as record</summary>

**`D2` (THE WORK) — prove `embedding_adequacy` by structural induction on
`FokIForm`.** Prove the ALREADY-LANDED proposition `fok_embedding_adequacy_statement`
(`FoKripke.ken:881`) as written — do NOT restate it. It reads

```
fok_classically_valid (fok_scoped_embed sigma f) -> fok_denote sigma c rho f
```

over `(sigma : FokSignature) (c : FokCarriers sigma) (rho : FokAtomEnv sigma c)
(f : FokScopedIForm sigma Zero)`. If the proof needs the proposition restated,
that is a hard stop to the Architect, not a licence to edit the landed
statement: D1's approval bound that exact form and `AC-1` still forbids adding
an assumption or trusted declaration to it. The `Imp` and `Forall` arms are the
non-trivial cases (as `ForallRight` was for checker-soundness), and the proof
composes with `fok_checker_soundness` (`:2268`).

</details>

> **D2 partition — Architect interface ruling requested at the D2 candidate.**
> If D2 splits into a propositional fragment (Bottom/Atom/Or/Imp) plus the
> quantifier arm (Forall), it MUST follow the checker-soundness D3/D4 shape the
> Architect ruled: a **structural full-tree `Bool` predicate over `FokIForm`**
> carried as a hypothesis, removed by the quantifier increment — NOT a parallel
> restricted `IForm` type. One `denote`, one adequacy relation, split by an
> ordinary hypothesis. Reviewed against that interface, not a restricted-type
> variant.

**`D3` — state the theorem's reach honestly.** Which `IForm` arms are covered,
whether route FO is now exactly one (downstream) wiring step from `proved`, and
that `attempt_fo` remains untouched. A partial fragment is a result; a fragment
reported as the whole is a defect.

## Acceptance criteria

**`AC-1`. Zero new entries in `trusted_base()`.** No `declare_primitive`, no
`declare_postulate`, no new kernel file, no trusted axiom. This is the entire
value of proving rather than postulating (`fo-route-theorem-home.md §3`); a
candidate that admits the theorem has destroyed the node's value.

**`AC-2`. The Ken `denote` and the Rust `denote` reference are the SAME
interpretation, established by a CONTROL, not a reading.** Mutate a `denote` arm
in one and show the differential reds. Per arm. If a mutation passes, that arm is
unprotected and saying why is part of this AC. (The checker-soundness campaign
twice had a safety argument that was wrong rather than merely unproven; a reading
is not a control.)

**`AC-3`. No FO `Proved` verdict, on any basis, and `prover.rs`/`attempt_fo` is
NOT edited by this node.** `23 §4.4` needs both theorems in an approved home AND
a downstream verdict-flip node; none of that is here. The reservation is
untouched.

**`AC-4`. The Rust reference (`fo_kripke.rs`) is NOT changed to make the proof go
through.** If adequacy genuinely cannot be proved against the reference `denote`,
that is a finding about the reference and it routes back — not a licence to edit
the subject of the theorem.

**`AC-5`. A hard stop is a complete result**, at `D0` or after. Report what was
established, what blocked, and what the next node would need.

**`AC-6`. No-regression, in CI (`COORDINATION §12`).** Targeted local validation
only — `-p ken-elaborator`, never `--workspace`.

**Required reviewers.** language-QA + Architect (soundness oracle — the Ken
`denote` faithfully mirrors the reference, the induction is structural, zero
`trusted_base` delta) + the Adversary (invention-in-costume: a `denote` arm that
silently diverges from the Rust reference, a fabricated `Carriers`/`AtomEnv`
shape, a restricted-`IForm` partition smuggled past the D2 interface).

## Banned scope

- **Emitting `proved` for FO / editing `attempt_fo` or `prover.rs`.** The
  verdict-flip is the downstream node, gated on both theorems' approved home.
- **Changing the Rust reference** (`fo_kripke.rs`) — it is the differential's
  reference, exactly as it was for the checker.
- **The converse direction** (`denote -> classically_valid`) — spec `:520-521`
  says only validity-to-Ken is needed; the converse is not in this contract.
- **Widening the `IForm` slice** beyond `23 §4.5`.
- **A kernel `size_rel` / SCT change.** If termination is non-structural, return
  to the enclave + Architect (the binding condition below), never a silent kernel
  measure change.

## Lessons the sibling node paid for — read before `D0`

- **SCT traces a decrease only through a DIRECT match feeding the recursive
  call**, not through a non-recursive helper's return (`FoKripke.ken:305-313`).
  A `denote` recursion over `FokIForm` is exactly that shape — inline the guard
  at each call site. Budget for it.
- **Discharge `Equal Bool _ True/False` with `Proved`, not `refl True`.** The
  spec sketch writes `refl True` (`:531`) but the kernel-checked idiom in this
  file is `Proved` (e.g. `:698`, `:727`); absurd hypotheses use `absurd h`.
- **Derive the operand fact you need from the CALL PATH, not the shape of the
  connective.** The checker-soundness campaign twice cut the wrong-polarity
  Bool-inversion lemma from the connective's shape; the correct one came from
  where the checker consumes it.

## Return condition (binding, from the enclave)

If authoring the embedding-denotation reaches a named sub-structure whose
termination is genuinely non-structural — not a subterm descent of any derived
inductive, which would contradict `23 §4` fixing the reflective apparatus as
structural — that returns to the spec enclave + Architect as a new fork, never a
silent fall into a kernel measure change.

## Sequencing

**Interim lane-2 WP after `checker_soundness`.** Released `ready` 2026-08-22 as
the low-contention FO completion while the operator's next language priority
([[LANG-MODULE-IMPORT-SYSTEM]]) is still being framed with the enclave. Per the
operator's finish-then-switch directive, the language ring finishes its
then-current WP and switches to module/import ahead of surface-syntax the moment
the enclave hands back the spec-surface.

**Contention: LOW.** Touches `catalog/packages/Tooling/Verification/FoKripke.ken`
plus `crates/ken-elaborator/tests/` — proof/catalog work, not the contended
lexer/parser/elaborator-core the surface-syntax backlog would move, and not the
lane-1 runtime crates. Coordinate a merge window only if a runtime WP is in
flight against `ken-elaborator` tests.

**Capability tier: T1.** A soundness-bearing theorem proved against a reference
— structural induction over the denotation, reviewed on the argument, not a
mechanical diff.

**Nothing consumes FO `proved` today** ([[V3-FO-CONVERSION-LOAD-MEASURED]] `D1`:
no Ken program in this repo produces an FO-quotable obligation), so this gates
nothing urgently. It completes the FO route's theorem obligations; the
verdict-flip remains a distinct downstream node.
