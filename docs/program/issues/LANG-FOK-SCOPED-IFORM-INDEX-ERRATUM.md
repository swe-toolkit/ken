---
id: LANG-FOK-SCOPED-IFORM-INDEX-ERRATUM
title: "D2b HS20 carrier erratum: FokScopedIForm declares depth `n` as a datatype PARAMETER (FoKripke.ken:122, `data FokScopedIForm (sigma) (n) : Type`, params=2 indices=0) but FokScopedForall recurses `n -> Suc n` — non-uniform, which only an INDEX permits. The Elim of a parameterized type fixes `n` in the motive, so the correspondence-motive IH stays at outer `n` while the constructor body is at `Suc n` (generic D0 `KernelRejected TypeMismatch { expected (Dg581 (cg69 @3)), found (Dg581 (cg69 (cg69 @3))) }` = MiniForm(Suc m) vs MiniForm(Suc(Suc m))). Fix = move `n` from the parameter list to a `Nat ->` INDEX (`data FokScopedIForm (sigma : FokSignature) : Nat -> Type`), constructor result/field types carrying `n` as index exactly as the sibling FokObjectEnv (:88, `data FokObjectEnv (sigma) (c) : Nat -> Type 1`) already does and eliminates fine. Restores FokScopedIForm to its own stated intrinsic-index intent. Architect layer ruling evt_6ntratrsb8qtj."
status: active
owner: language
size: S
gate: none
tier: T1
depends_on: []
blocks: [V3-FO-EMBEDDING-ADEQUACY]
github: null
origin: "Steward, 2026-09-04. D2b hard-stop HS20 for V3-FO-EMBEDDING-ADEQUACY, split out as a distinct D1-erratum WP on the Steward's WP-boundary call. The Architect ruled the layer (evt_6ntratrsb8qtj, grounded by reading FoKripke.ken at build SHA 20f96de08): a D1 CARRIER defect — a semantic index (depth `n`, which the declaration comment itself says 'counts enclosing object binders; forall checks its body at one additional object slot') mis-declared as a non-uniform datatype parameter — NOT an elaborator gap and NOT a kernel gap. FokObjectEnv three declarations up proves the kernel ALREADY supports length-varying recursion via indices; Option 2 (touch kernel/Elim for parameter-changing recursion) is REJECTED (it either re-invents indices or breaks parameter-uniformity, which is load-bearing for soundness). The Architect ruled the fix an ERRATUM, not a re-author (changes no design, restores D1 to its own approved intrinsic-index intent, mirrors the correct sibling declaration already in the file, minimal data-head change), and left the WP boundary (re-open D1 in-release vs a separate D1-erratum WP that D2b rebases onto) to the Steward. Steward WP-boundary call: DISTINCT erratum WP — same shape as predecessors #4/#5, the carrier migration is a different object from the embedding_adequacy proof, carries its own no-regression gate and catalog/ (Architect) review domain, and landing it as an explicit reviewed erratum RESPECTS the 'do not re-author D1' boundary rather than smuggling a D1 change into the D2b proof candidate. NOT a kernel/TCB change (zero trusted-base delta), NOT proof authoring, NOT source-compensable. §1a: HS20 is NOT a research trigger; next mandatory = HS21 (Steward-authoritative count, Architect-confirmed). §1b: distinct predicate (upstream carrier mis-declaration — a semantic index declared as a non-uniform parameter), separate from the HS16-19 elaborator-convoy predicate; recorded as its own D2b inventory entry. Coordinates re-measure at your build SHA; held D2b WIP rebased to 20f96de08; generic grids /tmp/d2b-hs20-generic-d0.rs, /tmp/d2b-hs20-generic-final.log, /tmp/d2b-parameter-index-shape.log."
---

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
abstracts over. No new elaborator increment (the forced/varying-index machinery
just landed in [[LANG-DEPELIM-FORCED-INDEX-TELESCOPE-CLOSURE]] is exactly the
support the correspondence motive will use once `n` is an index) and NO kernel
change.

## Acceptance criteria (predicate form)

- **AC-INDEX-CARRIER.** `FokScopedIForm` elaborates with `n` as an INDEX (`n` no
  longer in the parameter list), mirroring `FokObjectEnv`'s index treatment —
  verified by the params/indices measurement (`/tmp/d2b-parameter-index-shape.log`
  shape: `FokScopedIForm` moves from params=2 indices=0 to `n`-as-index). The
  arity `FokScopedIForm : FokSignature -> Nat -> Type` is UNCHANGED.
- **AC-MOTIVE-ENABLED (forward, target-decoupled).** The generic coupled
  correspondence motive that D0-REJECTED (`/tmp/d2b-hs20-generic-d0.rs` coupled
  case, currently `KernelError::TypeMismatch`) now ELABORATES on the
  already-landed forced-index-telescope mechanism, no new elaborator change. This
  proves the migration achieves its purpose; the full `embedding_adequacy` proof
  stays in D2b.
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

## Reviewers

Architect (catalog/ erratum domain + migration soundness — the layer ruling is
his `evt_6ntratrsb8qtj`; he re-reviews the migration and the D2b resumption),
Language QA, conformance-validator IF the change touches a conformance surface
(FoKripke.ken is verification tooling — likely none). Adversary runs the
post-merge M8b hunt. Merge via Steward M1-M4 -> lieutenant.

## Capability tier

T1. The deliverable is small in lines (a data-head declaration move) but the
acceptance turns on a soundness judgment — that param->index preserves every
existing constant-motive proof while enabling the varying-motive one — and on the
migration mirroring the load-bearing parameter-uniformity discipline.

## Sequencing

Releasable now: fully diagnosed, no `depends_on` (the downstream forced-index
mechanism is already landed for D2b's re-attempt, but this erratum has no dep). On
landing, the Steward explicitly RE-RELEASES [[V3-FO-EMBEDDING-ADEQUACY]] D2b (held
WIP rebased to `20f96de08` reusable, statement unchanged, re-measure coordinates;
D2b rebases onto the erratum and re-attempts the correspondence motive on the
landed forced-index-telescope mechanism — no new elaborator increment).
