---
id: LANG-NESTED-SPLIT-FIELD-DEPENDENCE
title: "A nested split whose constructor's other fields depend on the split column or the constructor's index fails in the kernel: a sibling field typed by the index gives TypeMismatch on the constant-motive path, and a variable row binding a split column a later field depends on leaks a pattern-alias sentinel as VarOutOfScope. Both checks or gives a surface diagnostic"
status: active
owner: language
size: M
tier: T1
gate: architect
depends_on: [LANG-INFER-MATCH-INDEXED-COMPLETE]
blocks: []
github: null
origin: "Adversary M8 findings evt_1vhs6demxejpg on 5d139f422 (LANG-INFER-MATCH-INDEXED-COMPLETE): elaborator correctness, no soundness issue (the kernel rejects every bad term). Reachable from well-formed source. Steward-filed per COORDINATION section 2."
---

# A nested split respects its siblings' dependencies

## Objective

A nested split on a constructor field elaborates, or gives a surface
diagnostic, when other fields' types mention the split column or the
constructor's index. Well-formed source never reaches a kernel type error or
a kernel de Bruijn error on this path.

## Settled inputs (Adversary `evt_1vhs6demxejpg`, read at `5d139f422`)

With `data Vec (a : Type) : Nat → Type` (`VNil`, `VCons : (n : Nat) → a →
Vec a n → Vec a (Suc n)`) and `data PairOut : Type where { Out : Nat → Nat →
PairOut }`:

- **Sibling typed by the index.** `fn f (n : Nat) (xs : Vec Nat (Suc n)) :
  PairOut = match xs { VCons _ Zero _ ↦ Out Zero Zero; VCons _ _ _ ↦ Out
  Zero Zero }` is `KernelRejected TypeMismatch { expected: Nat, found: Vec Nat
  (Suc n) }`. Measured: `dependent_tail = []` and `result_mentions_split =
  false`, so the constant-motive branch (`elab.rs:18318`) is taken with
  codomain `Π (Vec Nat m). PairOut`. It reproduces with `let`, at `Vec Nat n`,
  with a Bool element, and on a non-recursive family. A split on the tail,
  and the same match with no split, check.
- **Alias sentinel leak.** With `data Dep : Type where { MkDep : (n : Nat) →
  Vec Nat n → Dep }`, `match d { MkDep Zero v ↦ Out Zero Zero; MkDep n v ↦
  Out n n }` is `KernelRejected VarOutOfScope { index: 2^54 + 2 }`, which is
  `PATTERN_ALIAS_SENTINEL_BASE` (`:15675`) plus 2. Here `dependent_tail =
  [0]`. `finalize_pattern_aliases` (`:15802`) passes an unregistered sentinel
  through instead of failing closed. With `n` unreferenced, it checks.
- **Two split columns, one variable row.** `MkTri Zero Zero c ↦ …; MkTri a b
  c ↦ Out a b` is refused by `infer_virtual_pattern_alias` (`:15726`) with
  advice to annotate, which cannot apply to the checked form.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## AC-0 (Architect, at kickoff)

The Architect rules the repair for each finding, and whether this WP and
`LANG-NESTED-SPLIT-CONSTANT-MOTIVE-INDEX`, which edits the same branch, land
together or in sequence.

## Deliverable

The first two repros check, with arm values verified by full normalization.
An unregistered sentinel fails closed. The third either checks or gives a
diagnostic whose advice applies.

**Folded in: the cross-frame alias miscompilation** (Architect
`evt_kzkfxd4aenqq`, measured `evt_358pbpsezs8`). On `5d5e7bf02`, an outer
as-pattern alias used inside an inner match that splits a nested field
reads the next variable in, which is a silent wrong value. The inner
split's woven binder is counted twice. The repair is one frame-aware
`weaken_woven` at every woven-binder site of the matrix, with the fan-in
listed in the handoff. F2 builds on it. Owed with the handoff: a consumer
sweep of `catalog/`, `examples/`, `conformance/` and `crates/*/tests` for an
alias used across a nested-splitting inner match, reporting any runtime
(non-proof) hit.

## Acceptance

- **AC-1.** Each repro, and each listed variant, checks with the expected
  value, or gives the ruled surface diagnostic.
- **AC-2 (controls).**
  - The tail split, the no-split match and the landed IndexedPair rows keep
    their verdicts and values.
  - The cross-frame rows assert constructor values: M1, M2 and M3, two
    levels deep, and a middle-frame alias. Reverting `weaken_woven` at the
    nested-split site returns M2 to `Zero`.
  - A sentinel registered in no active frame is an `Internal` error. The
    enclosing-frame passthrough stays, per the Architect's correction.
  - Nested-column omission is still an `ExhaustivenessError`.
- **AC-3.** The targeted match and pattern suites stay green, the catalog
  census is byte-identical, and `trusted_base()` is unchanged.

## Symptom inventory (§1b, Architect)

1. A split column's pending tail types reach each constructor bucket without
   being rebased from the split binder onto that constructor's fields
   (`build_ctor_buckets`), so method domains are off by `n_args0 - 1`
   (`evt_24geh629pgfz1`, §1a 1). Repair: rebase the tail per constructor at
   the nested split; a two-field constructor pins the +1 direction.

## Stop conditions

- The repair needs the kernel or a spec change.
- A finding is the same defect as `LANG-NESTED-SPLIT-CONSTANT-MOTIVE-INDEX`
  scope: stop to the Architect to merge the frames.
