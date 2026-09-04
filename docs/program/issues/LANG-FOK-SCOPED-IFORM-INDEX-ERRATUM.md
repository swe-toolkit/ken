---
id: LANG-FOK-SCOPED-IFORM-INDEX-ERRATUM
title: "D2b HS20+HS21 (WIDENED 2026-09-04, RULED elaborator-only evt_5mg8n0bdz41m4): D1 = carrier migration `FokScopedIForm` depth n param->Nat index (catalog/); D2 = elaborator PLAIN-ELIMINATOR path for a declared-index recursive field whose motive returns a co-indexed family (crates/ken-elaborator). HS20 diagnosis: FokScopedIForm declares depth `n` as a datatype PARAMETER (FoKripke.ken:122, `data FokScopedIForm (sigma) (n) : Type`, params=2 indices=0) but FokScopedForall recurses `n -> Suc n` — non-uniform, which only an INDEX permits. The Elim of a parameterized type fixes `n` in the motive, so the correspondence-motive IH stays at outer `n` while the constructor body is at `Suc n` (generic D0 `KernelRejected TypeMismatch { expected (Dg581 (cg69 @3)), found (Dg581 (cg69 (cg69 @3))) }` = MiniForm(Suc m) vs MiniForm(Suc(Suc m))). Fix = move `n` from the parameter list to a `Nat ->` INDEX (`data FokScopedIForm (sigma : FokSignature) : Nat -> Type`), constructor result/field types carrying `n` as index exactly as the sibling FokObjectEnv (:88, `data FokObjectEnv (sigma) (c) : Nat -> Type 1`) already does and eliminates fine. Restores FokScopedIForm to its own stated intrinsic-index intent. Architect layer ruling evt_6ntratrsb8qtj."
status: active
owner: language
size: M
gate: none
tier: T1
depends_on: []
blocks: [V3-FO-EMBEDDING-ADEQUACY]
github: null
origin: "Steward, 2026-09-04. D2b hard-stop HS20 for V3-FO-EMBEDDING-ADEQUACY, split out as a distinct D1-erratum WP on the Steward's WP-boundary call. The Architect ruled the layer (evt_6ntratrsb8qtj, grounded by reading FoKripke.ken at build SHA 20f96de08): a D1 CARRIER defect — a semantic index (depth `n`, which the declaration comment itself says 'counts enclosing object binders; forall checks its body at one additional object slot') mis-declared as a non-uniform datatype parameter — NOT an elaborator gap and NOT a kernel gap. FokObjectEnv three declarations up proves the kernel ALREADY supports length-varying recursion via indices; Option 2 (touch kernel/Elim for parameter-changing recursion) is REJECTED (it either re-invents indices or breaks parameter-uniformity, which is load-bearing for soundness). The Architect ruled the fix an ERRATUM, not a re-author (changes no design, restores D1 to its own approved intrinsic-index intent, mirrors the correct sibling declaration already in the file, minimal data-head change), and left the WP boundary (re-open D1 in-release vs a separate D1-erratum WP that D2b rebases onto) to the Steward. Steward WP-boundary call: DISTINCT erratum WP — same shape as predecessors #4/#5, the carrier migration is a different object from the embedding_adequacy proof, carries its own no-regression gate and catalog/ (Architect) review domain, and landing it as an explicit reviewed erratum RESPECTS the 'do not re-author D1' boundary rather than smuggling a D1 change into the D2b proof candidate. NOT a kernel/TCB change (zero trusted-base delta), NOT proof authoring, NOT source-compensable. §1a: HS20 is NOT a research trigger; next mandatory = HS21 (Steward-authoritative count, Architect-confirmed). §1b: distinct predicate (upstream carrier mis-declaration — a semantic index declared as a non-uniform parameter), separate from the HS16-19 elaborator-convoy predicate; recorded as its own D2b inventory entry. Coordinates re-measure at your build SHA; held D2b WIP rebased to 20f96de08; generic grids /tmp/d2b-hs20-generic-d0.rs, /tmp/d2b-hs20-generic-final.log, /tmp/d2b-parameter-index-shape.log."
---

> # OPERATIVE (Steward, 2026-09-04) — HS21 RULED **ELABORATOR-ONLY** (Architect
> # evt_5mg8n0bdz41m4; §1a hold DISCHARGED, research advisory evt_1fcg4n5gwrfx8).
> # **This node is WIDENED to TWO deliverables** and is the single D2b-blocking
> # candidate:
> #
> # - **D1 (carrier migration, `catalog/`/FoKripke.ken, `n` param -> `Nat` index).**
> #   DONE/valid (WIP `af24b6a219`; AC-INDEX-CARRIER, AC-NO-REGRESSION,
> #   AC-ZERO-TRUST hold; green through the data-head, constant motive, Atom branch).
> # - **D2 (NEW — elaborator plain-eliminator path, `crates/ken-elaborator`).** The
> #   HS21 fix: an EXPLICIT plain-vs-coupled index-provenance classification, the
> #   plain-eliminator IH-at-declared-field-index path (IH at the field's OWN
> #   declared index `Suc n`; branch goal at the constructor's result index `n`; the
> #   bind body bridges `n -> Suc n`), the convoy/forced-index path (HS3-HS19) left
> #   UNTOUCHED, plus the §7b soundness accept/reject pair and the no-regression
> #   suite. This is the OPPOSITE of HS19's convoy extension — it needs LESS, not
> #   more (one scrutinee; `Env` in the motive's RETURN type, nothing to peel).
> #
> # The old AC-MOTIVE-ENABLED is SUBSUMED by D2's §7b accept arm (it was predicated
> # on the false "predecessor #5 covers it" expectation, which HS21 falsified). Build
> # BOTH on `af24b6a2` as ONE candidate (the carrier is not independently useful; it
> # enables nothing until the elaborator path lands) -> route review (Architect
> # design+soundness for `catalog/` + elaborator: classification explicitness, the
> # accept/reject pair, zero-new-trust, convoy-untouched; Language QA; CV IF any
> # conformance surface) -> Steward M1-M4 -> lieutenant. Zero kernel/TCB, NO operator
> # gate. On landing, Steward RE-RELEASES [[V3-FO-EMBEDDING-ADEQUACY]] D2b (rebases,
> # re-attempts the correspondence motive on the plain path). This banner supersedes
> # the RELEASED banner below for CURRENT state.
>
> # RELEASED (Steward, 2026-09-04) to the language ring as the D2b HS20 carrier
> # erratum. D2b ([[V3-FO-EMBEDDING-ADEQUACY]]) is HELD behind this node; it
> # RE-RELEASES only on the Steward's explicit re-release after this erratum
> # lands. MERGES on Architect (catalog/ erratum domain + migration soundness) +
> # Language QA + CV (only if the change touches a conformance surface; FoKripke.ken
> # is verification tooling, likely none) via Steward M1-M4 -> lieutenant.

## What HS20 is (Architect layer ruling `evt_6ntratrsb8qtj`)

D2b's correspondence motive needs a dependent elimination of `FokScopedIForm`
whose motive VARIES over the depth `n`. But `FokScopedIForm` (`FoKripke.ken:122`)
is declared `data FokScopedIForm (sigma : FokSignature) (n : Nat) : Type`, so `n`
is a datatype PARAMETER (elaborates params=2, indices=0). A parameter must be
UNIFORM across all constructors and recursive occurrences — yet

```text
FokScopedForall : FokScopedIForm sigma (Suc n) -> FokScopedIForm sigma n
```

recurses from `n` to `Suc n`. A recursive field at a different value is exactly
what an INDEX is for, not a parameter. The Elim of a parameterized type fixes `n`
in the motive, so a constant (n-invariant) motive is green but the correspondence
motive's coupled `n -> Suc n` makes the generated recursive IH stay at outer `n`
while the constructor body is at `Suc n`:

```text
KernelRejected { TypeMismatch { expected (Dg581 (cg69 @3)), found (Dg581 (cg69 (cg69 @3))) } }
```

= `MiniForm (Suc m)` expected vs the valid recursive `MiniForm (Suc (Suc m))`
body. This is NOT the HS19 return-telescope rebasing — there is no declared index
for predecessor #5's mechanism to carry.

### The fix pattern is already in the same file (do not re-derive)

`FokObjectEnv` (`:88`) is `data FokObjectEnv (sigma) (c) : Nat -> Type 1` — its
length IS an index, its constructors return `... Zero` / `... (Suc n)`, and it
eliminates fine. So the kernel ALREADY supports length-varying recursion via
indices; nothing kernel-facing is missing. Option 2 (touch kernel/Elim for
parameter-changing recursion) is REJECTED.

## Deliverable

**D1 — the carrier migration.** Move `n` from `FokScopedIForm`'s parameter list to
a `Nat ->` index:

```text
data FokScopedIForm (sigma : FokSignature) : Nat -> Type
```

with the constructor result/field types carrying `n` as an index exactly as
`FokObjectEnv` does. `FokScopedForall : FokScopedIForm sigma (Suc n) ->
FokScopedIForm sigma n` stays as written, but `n` is now the index the Elim
abstracts over. (The original HS20 expectation — that the landed
forced-index-telescope mechanism [[LANG-DEPELIM-FORCED-INDEX-TELESCOPE-CLOSURE]]
would suffice with no new elaborator increment — was FALSIFIED at HS21: D1 alone
keeps every existing proof green but the varying co-indexed motive needs the D2
elaborator plain-eliminator path below.) NO kernel change.

## Acceptance criteria — D1 (carrier)

- **AC-INDEX-CARRIER.** `FokScopedIForm` elaborates with `n` as an INDEX (`n` no
  longer in the parameter list), mirroring `FokObjectEnv`'s index treatment —
  verified by the params/indices measurement (`/tmp/d2b-parameter-index-shape.log`
  shape: `FokScopedIForm` moves from params=2 indices=0 to `n`-as-index). The
  arity `FokScopedIForm : FokSignature -> Nat -> Type` is UNCHANGED.
- **AC-MOTIVE-ENABLED — SUBSUMED by D2's §7b accept arm (HS21).** Originally: the
  generic coupled correspondence motive elaborates on the landed
  forced-index-telescope mechanism with no new elaborator change. HS21 falsified
  the "no new increment" premise; the motive-enabling now lives in D2
  (AC-D2-COUPLED-ELABORATES). Kept here only as the record of the falsified
  expectation — do not gate D1 on it.
- **AC-NO-REGRESSION.** Every existing green proof that relied on constant-motive
  elimination (a special case that still holds after param->index, which strictly
  ADDS eliminator power) stays green: `v3_fo_embedding_adequacy_d1` 6/6, the
  predecessor coherent-frame grid 9/9, target soundness through
  `fok_target_soundness` (`:4017`), the predecessor-#5 object-environment
  extension (`fok_embedding_env_object_index` + wrapper) and the `K(Sigma)`
  canonical-model proof. Green in CI (targeted `scripts/ken-cargo`).
- **AC-ZERO-TRUST.** `env.trusted_base()` unchanged; no kernel change; no axiom /
  postulate / primitive / trusted token added; `ken-kernel` BYTE-UNCHANGED. Every
  existing `FokScopedIForm sigma n` mention still type-checks (arity unchanged).

## Deliverable D2 — the elaborator plain-eliminator path (HS21)

**Added 2026-09-04, Architect ruling `evt_5mg8n0bdz41m4`, RULED ELABORATOR-ONLY.**
In `crates/ken-elaborator`, at the index-refinement / branch-goal-classification
step, add a PLAIN-eliminator motive-instantiation path for an upward-shifting
DECLARED-index recursive field, and route such fields to it instead of the
coupled-scrutinee convoy path:

- IH = the motive at the recursive field's OWN DECLARED index (`Suc n`, read from
  the constructor's recursive-argument type) — NOT index-refined/rebased from the
  scrutinee index.
- Branch goal = the motive at the CONSTRUCTOR's RESULT index (`n`).
- The bind method bridges them by the ordinary body lambda: `xs : Env n |- ih
  (EnvCons ... xs)`, extending `n -> Suc n` and applying the IH. No transport, no
  convoy, no auxiliary index equation.

Make the recursive-field index-provenance an EXPLICIT, deliberate branch —
declared-by-constructor (=> plain IH at the field's own index) vs
coupled-to-a-sibling-scrutinee-at-a-shared-index (=> the existing convoy/
refinement) — not an implicit fallthrough (the HS21 defect was a declared-index
field silently falling through into the coupled path). Explicit and covering the
shapes the elaborator handles is the bar; do NOT over-build a heavy sealed enum (a
mis-route fails SAFE — a wrong motive is kernel-rejected, incompleteness not
unsoundness). The convoy/forced-index path (HS3-HS19) is left UNTOUCHED.

## Acceptance criteria — D2 (predicate form)

- **AC-D2-CLASSIFICATION.** Recursive-field index-provenance is a readable
  plain-vs-coupled branch, not an implicit fallthrough; a declared-index recursive
  field routes to the plain-eliminator path.
- **AC-D2-COUPLED-ELABORATES (§7b accept arm).** The coupled recursive definition
  elaborates — `fok_denote_at`'s `FokScopedForall` branch and the correspondence
  motive, plus the `MiniForm` `mini_coupled` twin — with `env.trusted_base()`
  UNCHANGED.
- **AC-D2-BIND-FALSE-REJECTS (§7b reject arm, non-degenerate pair).** A
  Bind-branch-FALSE variant (a body whose result lands at the WRONG index) is
  REJECTED at `KernelError::TypeMismatch` THROUGH the plain-eliminator path — not
  incidentally by something else.
- **AC-D2-CONVOY-UNTOUCHED.** Every prior coupled-scrutinee D2b test stays green,
  proving the convoy path is unaltered for the shapes it correctly serves (the fix
  ADDS a path; it must not change the coupled one).
- **AC-D2-NO-REGRESSION.** The full existing green suite stays green (shared with
  D1's AC-NO-REGRESSION: D1 6/6, coherent-frame grid 9/9, target soundness `:4017`,
  `K(Sigma)`). Green in CI (targeted `scripts/ken-cargo`).
- **AC-D2-ZERO-TRUST.** No kernel/Elim change; no axiom / postulate / primitive /
  trusted token added; `ken-kernel` BYTE-UNCHANGED; `env.trusted_base()` unchanged.

## Reviewers

Architect (BOTH domains: catalog/ for D1 — his layer ruling `evt_6ntratrsb8qtj`;
and `crates/ken-elaborator` design+soundness for D2 — his fix-surface ruling
`evt_5mg8n0bdz41m4`, reviewing classification explicitness, the §7b accept/reject
pair, zero-new-trust, and convoy-untouched), Language QA, conformance-validator IF
the change touches a conformance surface (FoKripke.ken is verification tooling —
likely none; the elaborator increment adds a code path, no conformance surface).
Adversary runs the post-merge M8b hunt. Merge via Steward M1-M4 -> lieutenant.

## Capability tier

T1. D1 is small in lines (a data-head declaration move) but its acceptance turns
on a soundness judgment — that param->index preserves every existing
constant-motive proof — and on mirroring the load-bearing parameter-uniformity
discipline. D2 is a soundness-bearing elaborator increment: the §7b accept/reject
pair and the convoy-untouched invariant are argument-review, not
byte-faithfulness. Both are T1.

## Sequencing

Releasable now: fully diagnosed, no `depends_on` (the downstream forced-index
mechanism is already landed for D2b's re-attempt, but this erratum has no dep). On
landing, the Steward explicitly RE-RELEASES [[V3-FO-EMBEDDING-ADEQUACY]] D2b (held
WIP rebased to `20f96de08` reusable, statement unchanged, re-measure coordinates;
D2b rebases onto the erratum and re-attempts the correspondence motive on the
landed forced-index-telescope mechanism — no new elaborator increment).
