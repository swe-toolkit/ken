---
id: LANG-NESTED-SPLIT-FIELD-DEPENDENCE
title: "A nested split whose constructor's other fields depend on the split column or the constructor's index fails in the kernel: a sibling field typed by the index gives TypeMismatch on the constant-motive path, and a variable row binding a split column a later field depends on leaks a pattern-alias sentinel as VarOutOfScope. Both checks or gives a surface diagnostic"
status: active
owner: language
size: M
tier: T1
gate: architect
depends_on: [LANG-INFER-MATCH-INDEXED-COMPLETE]
blocks: [LANG-NESTED-MATRIX-DERIVED-TELESCOPE]
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

## Narrowed acceptance (Steward `evt_7dwx0stmecwd8`, Architect `evt_5eahknef2gfbm`)

After six advancing stops, this WP keeps what is proven at WIP `9a5b617c5`.
It is F1, F2, F3, `weaken_woven` and the unconditional `method_type` close.
The structural closure goes to `LANG-NESTED-MATRIX-DERIVED-TELESCOPE`. The
Architect's Part 1 list in `evt_5eahknef2gfbm` is the pin set. Every value
pin asserts the normalized constructor, not only that elaboration succeeds.

- **AC-N1 (F1 and the close).**
  - The exact repro checks, and both calls normalize to the expected `Out`.
  - The tail-rebase pins hold: Zero, a two-field constructor and a Suc
    guard.
  - Restoring `if needs_reverting` reddens the exact repro.
  - There is a unit test for `assert_nested_method_alignment`.
  - The two-`Nat` collision control returns the second binder's value,
    under Zero and under a two-field constructor.
- **AC-N2 (F2 and `weaken_woven`).**
  - The Dep row's value, which reddens when the in-matrix finalize is
    removed.
  - The unit test: own-frame gives `Ready`, enclosing-frame gives
    `Deferred`.
  - A deferred method with a type error is still rejected at `declare_def`.
  - An unregistered sentinel at the helper and at the outermost pop is each
    `Internal`.
  - M1, M2, M3, two-deep and middle-frame assert values. Reverting to
    `weaken` returns M2 to `Zero`.
  - The fan-in list and the consumer sweep go in the handoff.
- **AC-N3 (F3).** The diagnostic text ruled in `evt_hepczebkww5c`, or a
  statement that F2 made the program check.
- **AC-N4 (M-deep carve-out).** Measure the Zero fixture and its two-field
  sibling on `5d5e7bf02` and on the candidate.
  - If both are rejected on base, each gets a transition-sentinel pin on the
    candidate. The pin asserts `KernelRejected`, never a value, and names
    the successor that flips it.
  - If either passes on base, or gives a wrong value on either side, that is
    an advancing stop.
- **AC-N5 (check gate, `evt_4c4tgkc2gvypk`).**
  - `lang_match_record_pattern` is 9/9 with its value pins.
  - Dropping `&& needs_reverting` reddens the record row with
    `VarOutOfScope {4,3}`.
  - Gating the whole block, close included, still reddens the F1 repro.
  - Every verdict, value or diagnostic moved by the gate is reported.
  - A reverting nested split under a woven Var column (P5) is measured on
    base and on the candidate. The same rejection on both gives a
    transition-sentinel pin, and green on both gives a value pin.

## Symptom inventory (§1b, Architect)

1. A split column's pending tail types reach each constructor bucket without
   being rebased from the split binder onto that constructor's fields
   (`build_ctor_buckets`), so method domains are off by `n_args0 - 1`
   (`evt_24geh629pgfz1`, §1a 1). Repair: rebase the tail per constructor at
   the nested split; a two-field constructor pins the +1 direction.
2. Zero `ih'` `VarOutOfScope`; the Architect misattributed the cause to the
   tail-rebase skip (`evt_378pchbn859mr`, §1a 2; corrected
   `evt_4yewspasn0fps`).
3. A woven IH domain in a nested bucket is produced from the root method's
   cached domain by de Bruijn arithmetic (`indexed_root_ih_domain`: shift by
   ordinal, weaken by `real_depth_so_far − field_count`). A nested split that
   reverts the IH's field (`t` becomes `t'`) and replaces x' invalidates the
   arithmetic. Keyed on the domain's producing frame (`evt_4yewspasn0fps`,
   §1a 3; Research advisory `evt_4p5eqpfdywk97`, ruled `evt_4jz9mx20gpqv`).
4. Alias sentinels reach the in-matrix per-method kernel check, which is now
   unconditional, on the formerly constant path before finalization. Keyed on
   check-site order (`evt_434y2eqz37h5k`, §1a 4). F2 and `weaken_woven` pair
   with the unconditional close; F2's call site widens to both paths.
5. An enclosing frame's alias sentinel was finalized at an inner in-matrix
   check against `cx.ctx`. That context lacks the enclosing frame's
   not-yet-woven binders, so it resolved to a sibling parameter. Keyed on the
   frame coordinates of the occurrence (`evt_7ve4bw9145c1x`, §1a 5). Repair:
   finalize only the current frame's sentinels and defer the in-matrix check
   for a method carrying an enclosing frame's sentinel; `declare_def` stays
   the authority.
6. A second split inside a Zero bucket, with the root IH still in its tail.
   The interior motive's IH domain comes from `indexed_root_ih_domain`
   arithmetic, which gives `VarOutOfScope {6, 6}`. Keyed on the IH domain's
   producing frame (`evt_6xyk9hsbe6h7m`, §1a 6, M-deep at `9a5b617c5`;
   research hold).
7. The in-matrix per-method kernel check, made unconditional, runs in
   `nested_ctx` = `cx.ctx` + split column. That context omits the matrix's
   woven binders: the Var-column λ (:18414), the IH and enclosing split
   lambdas (:18218), and `enter_woven_real_binder` (:16226). The method's
   variables index through those binders, so a record/tuple leaf that
   returns a woven Var column gets `VarOutOfScope {4,3}`. Keyed on the check
   context being built from `cx.ctx` (`evt_4c4tgkc2gvypk`, §1a 7). Repair:
   finalize and close stay unconditional; the in-matrix check runs only where
   base ran it (`ready && needs_reverting`).

Shared predicate (`evt_4yewspasn0fps`, restated `evt_7ve4bw9145c1x`): a term
in one frame's coordinates is used in another frame's context through depth
arithmetic. Entries 1, 2, 3, 5, 6 and 7 are this predicate; entry 4 is a
consequence of the closure. The closure is the successor WP, nested matrix
construction in the derived telescope with woven binders as real context
pushes, framed on the stop-6 advisory and the M-deep base measurement.

## Stop conditions

- The repair needs the kernel or a spec change.
- A finding is the same defect as `LANG-NESTED-SPLIT-CONSTANT-MOTIVE-INDEX`
  scope: stop to the Architect to merge the frames.
- Each of these is an advancing stop (the 8th after `evt_4c4tgkc2gvypk`):
  the AC-N4 base-pass or wrong-value condition; a hardened value pin
  failing; a regression in `lang_infer_match_indexed_complete` (20/20), the
  record-pattern suite, or the as-pattern, nested-split and tuple-pattern
  suites; the record row red with the check gated; P5 green on base and red
  on the candidate, or a wrong value anywhere.
