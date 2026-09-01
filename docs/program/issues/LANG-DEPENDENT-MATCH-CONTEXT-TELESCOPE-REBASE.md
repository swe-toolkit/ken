---
id: LANG-DEPENDENT-MATCH-CONTEXT-TELESCOPE-REBASE
title: "D2b elaborator-only predecessor (the context telescope): constructor index refinement must transform the transitive forward-dependency closure of the LOCAL CONTEXT as one typed telescope substitution, not just the motive, constructor expected goal, and direct IH. One branch-refinement plan owns the root substitution plus the ordered convoy; it REPLACES the sibling-outer-binding portion of install_index_refinements, not layers beside it. Elaborator-crate only — NO kernel/TCB change; if implementation reveals a kernel/TCB touch is required (as LANG-RECORD-INDEX-REFINEMENT did), STOP and route to the Steward -> operator, do NOT make a silent kernel change."
status: active
owner: language
size: M
gate: lang-qa+architect
tier: T1
depends_on: []
blocks: [V3-FO-EMBEDDING-ADEQUACY]
github: null
origin: "Steward, 2026-09-01. FOURTH D2b predecessor, framed+released from the Architect HARD-STOP-3 mechanism ruling evt_mpmnxxh42r0z (thread thr_ncn98kb1htt0), which classified the third consecutive dependent-index elimination hard stop as (a) an elaborator completeness closure over the dependent context telescope — NO kernel/TCB route (no well-typed core Elim/Cast redex required by 14/17 shown stuck) and NO permanent FoKripke source convoy. Mandatory hard-stop-3 Research advisory evt_240ycvyantgrb (artifact /tmp/d2b-r2-hs3-research-advisory.txt sha b5d13c58...) preceded the ruling and reached the same disposition, with Agda with-abstraction and Lean getFVarsToGeneralize as prior art. The prior three predecessors: LANG-RECORD-INDEX-REFINEMENT (kernel eq_at_inductive weaken, MERGED PR #3216) and LANG-DEPENDENT-MATCH-MOTIVE-REBASE (three-site motive/expected_here/direct-IH rebasing, MERGED PR #3224, 1189aa3a) cleared the record-index MATCH and the derivation inversion; D2b then hit the fok_coh_extend context-telescope stop (evt_3b9k92cmkn5zh): a captured xs : FokObjectEnv ... n does not follow the constructor refinement of a FokFin index under match j. Shared predicate across all three: constructor-produced index refinement is not propagated through every dependent consumer — first the record-equality representation, then the explicit motive/goal/IH, now a captured dependent context telescope. D2b (V3-FO-EMBEDDING-ADEQUACY) is FROZEN at held checkpoint bf8f37326 (tree 8ff4f3921, base 436ac00e9) and FO stays Unknown behind this predecessor; D2b resumes ONLY after this lands and the exact held FoKripke consumer re-passes WITHOUT the manual convoy."
---

## Objective (Architect-ruled mechanism, evt_mpmnxxh42r0z)

In the elaborator's dependent-match machinery, make constructor index
refinement transform the **transitive forward-dependency closure of the local
context** — the ordered dependent tail of ambient bindings — as **one
well-typed telescope substitution**, together with the motive, constructor
expected goal, and direct recursive IH. A context is a telescope: refining an
earlier index `n` requires transforming every later declaration whose type or
value transitively depends on `n`, in order (`n`, then `xs : Env n`, then
`h : P n xs`, then `z : Family n xs h`). Independent per-binding retyping (the
current `install_index_refinements` scan) is not a telescope substitution — it
can move `xs` while a later `h : P n xs` remains stated over the old `n`/`xs`,
or leave the captured `xs` outside the generalized motive. The missing unit is
the ordered telescope, not a fourth independent term replacement.

## Required mechanism (the ruling's 7 steps)

Create **one branch-refinement plan before motive construction**. The minimum
internal shape is one data-bearing producer equivalent to (exact Rust names may
differ; the **one-plan ownership may not**):

```rust
struct DependentMatchRebasePlan {
    motive_pairs: Vec<(Term, Term)>,
    convoy: Vec<ConvoyEntry>,
}
struct ConvoyEntry {
    source_position: usize, // stable bottom-relative ambient-context position
    source_type: Term,      // at the plan's ambient depth
}
```

It must **replace** the sibling-outer-binding portion of
`install_index_refinements`, not layer beside it.

1. Derive the actual-index/scrutinee -> local-index/local-scrutinee
   substitution **once** from the checked family and constructor targets.
   Motive, `expected_here`, direct IH, and convoy all consume this **same
   plan**.
2. Walk genuine ambient bindings in **telescope order**. Preserve
   `match_field_regions`: a field bound by an enclosing match is not an outer
   captured binder. Include an entry when rebasing its type by the root pairs or
   by already-included binder replacements **changes** that type. After
   inclusion, add original-variable -> new-convoy-binder to the mapping before
   visiting the next entry (the transitive closure).
3. Generalize that ordered closure into the **motive codomain telescope**. The
   generated normal form is
   `M local_indices local_scrutinee = Pi xs' : Env local_n. Pi h' : P local_n xs'. …`,
   not independent final casts of `xs` and `h`.
4. For each method, push those convoy binders at constructor-local types, remap
   source references to the new binders, check the branch body, then wrap the
   checked body in the same ordered lambdas. The direct IH must expose/consume
   the same convoy telescope.
5. After constructing `Term::Elim`, **apply** its returned telescope to the
   original ambient values in the same order — reconstructing the original
   expected type at the actual indices. Carry both type and term mapping; a
   changed type with an `RVar` still resolving to the old raw variable is
   invalid.
6. Use only existing kernel-checked equality evidence. Invent no equality, add
   no kernel rule, do not loosen conversion. If any entry cannot be transformed
   and kernel-classified in the transformed prefix, **fail closed** with the
   dependent-match diagnostic; never keep an easy prefix and silently drop the
   rest.
7. `RLet` currently pushes only `rhs_ty` into `cx.ctx` while the value stays in
   the enclosing core `Let`. A dependent let-bound variable must therefore be
   treated as a captured binder by its pushed type and remapped term; if the
   implementation cannot preserve that mapping it must **reject explicitly**, not
   ignore the entry.

## Hard constraints

- **No kernel/TCB change.** Scope is `crates/ken-elaborator` production plus
  generic elaborator tests only. The diff must contain **zero** `ken-kernel`,
  `/spec`, FoKripke, theorem, axiom, primitive, conformance, or trust-base
  paths. If implementation reveals a kernel/TCB touch is required (the way
  LANG-RECORD-INDEX-REFINEMENT's elaborator hypothesis was falsified into the
  `eq_at_inductive` weaken), that is a **HARD STOP**: reset clean, post the
  measurement, and route to the Steward — the Steward routes any kernel/TCB
  touch to the operator. No silent kernel change under an elaborator-only frame.
- **Not a conversion extension.** Do **not** require the bare direct tail `Refl`
  (`lookup (Cons x xs) (FinSuc j) = lookup xs j` stated directly) to become
  definitional. The reaching acceptance is the ordinary **captured-binder match**
  whose constructor branches close; the ruling is explicit that a kernel/whnf
  change has no witness here.
- **Generic, not Fok-keyed.** Use generic fresh `Fin`/`Env` families; no
  `FokFin`/`FokDerivation`/`FokObjectEnv`/FoKripke names in the mechanism or its
  tests.
- No re-index of any relation, no axiom, no `Option` workaround, no permanent
  source convoy as the fix (the manual convoy is the positive **oracle**, not the
  repair).
- Currency debt, do not action here: `34-data-match §3.2.1` still calls the
  full two-vector `zip` follow-on a gap, but that sentence predates the landed
  remedy (`ds5b_dependent_match_refinement_acceptance.rs:247+`, `elab.rs:337-365`
  and `:3618+`). Track it as spec currency debt; do **not** widen this
  predecessor into `/spec`.

## Fixed inputs (Architect-measured at bf8f37326; re-measure at release base)

- `dependent_rebase_subs` consumed only for motive, constructor `expected_here`,
  and direct IH (`elab.rs:1592-1622`, `:2547`, `:2760`, `:2911`).
- `install_index_refinements` scans bindings one at a time (`:3719-3850`), builds
  an isolated `J`/`Cast`; `RVar` substitutes that alias later (`:4162-4177`).
  This scan is what the one plan replaces.
- Red witnesses: direct tail `Refl` RED `8c89b211…5303`; environment-first RED
  `f6a7ee11…e8d1`; full-extension attempt RED at
  `expected (((Dg602 @12) @11) @8), found (((Dg602 @12) @11) (cg69 @10))`
  (`55760de1…f25`). Manual convoy (`xs` behind `match j`) GREEN `1049cac6…7dbf`
  — the positive oracle.

## Required pins (acceptance / discriminating gates)

- **Generic red shape + positive oracle.** The unconvoyed theorem with
  parameters `(n)(j : Fin n)(x)(xs : Env n)`, body
  `match j { ZeroCase … => Refl; SucCase … => Refl }`, proving
  `lookup (Cons x xs) (FinSuc j) = lookup xs j` — the original red shape — now
  elaborates; the manually convoyed `(x)->(xs)->…` theorem is its positive
  oracle.
- **Transitive positive.** Add `h : P n xs` and `z : Family n xs h`; consume all
  three in the branch. **Inspect the emitted `Term` AST** to prove the
  motive/method telescope and the final applications carry them **in order** —
  "source checks" alone is insufficient.
- **Independent one-site mutations must RED** when: `xs` is omitted; `h` is
  omitted after `xs`; the type moves but the branch `RVar` still maps to the raw
  ambient term; application order is reversed; equality orientation is reversed.
- **Preserve the existing uncoupled `Vec` map/zip negative** from
  `dependent_match_motive_rebase_acceptance.rs` — blanket coincidental-index
  rebasing must RED it. **Preserve the DS5b enclosing-field-provenance fixture**
  — deleting the `match_field_regions` exclusion must RED it.
- **Dependent let:** include a dependent-let positive, or an explicit
  fail-closed negative, per the supported boundary. No silent skip receives
  credit.
- Diff-scope AC: zero forbidden paths (kernel/spec/FoKripke/theorem/axiom/
  primitive/conformance/trust-base). Targeted `scripts/ken-cargo` only; CI owns
  workspace closure.

## Gate and sequencing

Candidate reviewed by **language QA + Architect** on the exact SHA (the Architect
ruled the mechanism; the review confirms the implementation against it, and the
motive/telescope machinery is soundness-adjacent). Steward routes (M1-M4),
lieutenant executes (M5-M9). **After this predecessor lands, rerun the exact
held FoKripke consumer at `bf8f37326` WITHOUT the manual convoy** — that green is
the consumer gate; only then does the Steward explicitly RE-RELEASE D2b
(V3-FO-EMBEDDING-ADEQUACY) from `bf8f37326`. No parallel source repair, no
kernel escalation, and no D2b movement before that gate. FO remains `Unknown`
until D2b itself lands.

## Contention

Language lane (lane 2). Touches the elaborator's dependent-match/motive and
context-refinement machinery (`install_index_refinements` and the
`dependent_rebase_subs` consumers). No `crates/ken-kernel` change (hard
constraint above); no `/spec` change; no overlap with the runtime lane's
composed-return SSA work or the doc track. Base is current `main` at release.
