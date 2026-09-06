---
id: LANG-ABSTRACT-EXPORT-PARAM-ELAB
title: "abstract export of a parameterized module data type: mint the opaque view at the type's full kind (not nullary Type 0), and keep the defining module's constructors transparent (two-faced elaboration). Elaborator-only, no kernel edit. Unblocks the NonEmpty/Validation Tier-C migration."
status: merged
owner: language
size: M
gate: none
tier: T1
depends_on: []
blocks: [CAT-MIGRATE-TIER-C-NONEMPTY-VALIDATION]
github: null
origin: "Architect component-design ruling evt_4s9y6bpyzgaet (2026-09-06) on the foundation NonEmpty pub-data hard stop (Tier-C D1, foundation-implementer evt_5h8nbwcweq87m). Two separable elaborator defects in module `pub data` elaboration, both refuted by the published spec/kernel. Behavioral contract confirmed by the Spec enclave (spec-author evt_7vpq0673kjcyp, spec-leader evt_jp6ygzq8awr4): both clauses are DERIVATIONS of the current contract, not design forks; durable spec text authored in parallel as LANG-ABSTRACT-EXPORT-PARAM-spec (b3b4e5cee, §4.2 +21/-0, in CV+Architect review). Steward frame per COORDINATION §2."
---

> # RELEASED 2026-09-06 to the language ring (lane-2). Base = current main
> # b13c3af1. This is the NEXT lane-2 language item, AHEAD of the discretionary
> # language sequence (as-pattern / symbolic-operators / etc.), because it
> # unblocks lane-3 foundation (the language-unblocks-foundation pattern). The
> # Architect is a REQUIRED per-candidate reviewer (soundness-adjacent: the
> # elaborator currently makes the kernel reject a VALID proof). Re-measure every
> # anchor at the cut: the Architect located the divert at modules.rs:1700-1762 /
> # :1739 when it grounded from origin/main, but main has since moved and at
> # b13c3af1 the `declare_postulate` + `Term::ty(Level::Zero)` site reads ~:2526.
> # The anchor is the DIVERT BEHAVIOR, not the line number. ELABORATOR-ONLY: no
> # kernel edit, no new kernel feature (verify the kernel tree stays
> # byte-identical). Can proceed against the CONFIRMED spec ruling now; it does
> # not wait on the spec text landing (enclave: build proceeds in parallel).

## Objective

Fix the two elaborator defects that make a parameterized `pub data` inside a
`module { }` unusable. Today the elaborator diverts a module-level `pub data`
wholesale to a single opaque `declare_postulate` (the `Term::ty(Level::Zero)` +
`vec![]`-params site in `crates/ken-elaborator/src/modules.rs`), which (1) mints
the type former at the wrong kind — nullary `Type 0`, dropping the parameter
telescope — so `NonEmpty a` = `App(Const(NonEmpty), a)` has a sort at its head and
the kernel correctly raises `NotAFunction { head: Type 0 }`; and (2) never
elaborates the real inductive, so the DEFINING module loses its own constructors
and cannot prove its laws. The kernel is correct throughout — it is being handed a
malformed term. The fix is entirely in the elaborator.

## Deliverables

**`D-ARITY` — mint the opaque view at the type's full kind.** At the module
`pub data` divert, mint the abstract-export opaque constant at the data type's
FULL kind (its parameter telescope preserved), not a fixed nullary `Type 0`. For
`data NonEmpty (a : Type) = ...` the exported constant is `NonEmpty : (A : Type) →
Type` — a Π-kinded postulate. Spec-grounded and refuted by §4.2's own text: "the
kernel representation is byte-identical to a hand-written opaque constant" means
byte-identical AT THE SAME KIND, and the kernel already declares opaque constants
of any kind (spec-author: kernel `11 §4`) and has Π (`11 §1`, `13`). Collapsing to
nullary is a DIFFERENT constant. No new kernel feature.

**`D-TWOFACE` — two-faced elaboration: transparent inside, opaque to clients.**
The defining module retains transparent access to the real inductive and its
constructors, so the module's own operations (`nonempty_append`, `head`, `tail`)
and proofs (`nonempty_append::assoc`) elaborate and kernel-check; only the
EXPORTED view to clients is opaque. Spec-grounded: `33 §4.1` (module-private names
are in scope WITHIN their own module, invisible only outside) + `§3.3` (name
resolution is surface-only, never reaching Σ). Constructors are ordinary
module-private globals — one flat Σ, differing surface scopes. Replace the current
single-faced whole-unit `declare_postulate` substitution, which never elaborates
the real inductive at all.

## Acceptance criteria

**`AC-ARITY-PRESERVED`** — a parameterized module `pub data T (a : Type) = ...`
exports `T : (A : Type) → Type`; `T x` elaborates (the application head is a Π,
not a sort). CONTROL: a nullary `pub data U = MkU` still exports `U : Type` — the
previously-covered nullary case does not regress.

**`AC-INTERNAL-TRANSPARENT`** — within the defining module, a function or proof
that matches or constructs `T`'s constructors elaborates and kernel-checks green.
Concretely: the NonEmpty module's `nonempty_append::assoc` proof, which today
kernel-rejects `NotAFunction { head: Type 0 }` at the `cong` application under the
structural cases when `NonEmpty` is `pub data`, checks GREEN with the fix.
CONTROL: the SAME raw-constructor reference from a CLIENT module is rejected.

**`AC-CLIENT-OPAQUE`** — a client importing the module sees `T` as abstract: it
can name `T` and use the exported accessors/smart-constructors, and CANNOT match
or construct `T`'s raw constructors. The sanctioned abstract-export feature
(constructor hiding at the client) is preserved, not weakened.

**`AC-KERNEL-ACCEPTS-VALID-PROOF`** — the exact foundation scratch shape that
produced `NotAFunction { head: Type 0 }` (imports LF `Semigroup` + Transport
`cong` + Derived `list_append`/`list_append::assoc`, with `pub data NonEmpty`)
elaborates and kernel-checks green. This is the soundness-adjacent regression the
Architect flagged closing: the kernel was correctly rejecting an elaborator-made
malformed term; the fix makes the elaborator produce the well-formed term.

**`AC-NO-KERNEL-EDIT`** — the `crates/ken-kernel` tree is byte-identical
(`git rev-parse <cand>:crates/ken-kernel` unchanged): no new kernel feature, no
per-view kernel Σ divergence, no kernel visibility flag. Elaborator-only.

**`AC-NO-REGRESSION`** — green in CI; targeted `-p ken-elaborator` plus the
affected-closure consumers (every target loading a module whose pub-data
elaboration this changes), via `scripts/ken-cargo`, never `--workspace`. Green in
CI is the workspace verdict.

## Banned scope

- **No kernel edit / no new kernel feature.** The kernel already has
  opaque-constant-of-any-kind and Π; §4.2 explicitly rejects a per-view divergent
  kernel Σ and a kernel visibility flag (spec-author). If the fix appears to need
  a kernel change, that is a HARD STOP to the Steward + Architect, not a kernel
  edit — it would grow the TCB and require operator authorization.
- **Do not weaken client opacity.** Constructor hiding at the client is the
  sanctioned feature; keep it.
- **Do not touch the foundation package.** The NonEmpty/Validation API revision
  (abstract type + smart constructors) is the held foundation successor, not this
  WP.

## Reviewers, sequencing

`gate: none`. On each candidate: **Architect** (REQUIRED — soundness-adjacent, its
own flagged gap; confirm the two-faced elaboration is sound and the minted kinds
are correct) + **Language QA** on the exact SHA, then Steward M1-M4 → lieutenant.
Released as the next lane-2 language increment; the discretionary language
sequence queues behind it. On landing, the Steward reframes Tier-C D1 to the
abstract + smart-constructor shape and re-releases it to foundation (also gated on
the parallel `LANG-ABSTRACT-EXPORT-PARAM-spec` text landing).
