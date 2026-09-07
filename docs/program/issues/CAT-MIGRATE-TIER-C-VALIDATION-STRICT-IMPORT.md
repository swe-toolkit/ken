---
id: CAT-MIGRATE-TIER-C-VALIDATION-STRICT-IMPORT
title: "Strict-transparent Validation: make Validation a strict, selectively-importable module -- export Valid, Invalid, the type, and the existing smart constructors as public importable symbols (NOT abstract; the raw constructors stay visible), retire ambient reach, and add the selective import edges into its clients. Flip-only and logic-free: no new definition, no eliminator, no denotation change. Validation carries no representation invariant, so abstraction is not warranted; the strict-roots goal here is importability, not opacity."
status: merged
owner: foundation
size: S
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-09-07, split out of [[CAT-MIGRATE-TIER-C-NONEMPTY-VALIDATION]] on the Architect ruling evt_1wgz67t0nj1ne + Spec enclave confirmation evt_5m18jwangsdf8 (spec-leader). The parent node's P3 bundled two roots under one 'flip to strict abstract' treatment. That is correct for NonEmpty (hiding NonEmptyCons protects the non-emptiness invariant and keeps nonempty_head total by construction) but REFUTED for Validation: Schema both CONSTRUCTS and PATTERN-MATCHES the raw Valid/Invalid constructors (Schema.ken.md:113-142), and Validation carries NO representation invariant (Validation.ken.md is a plain error-or-value sum; Valid/Invalid are just the two cases, freely and legitimately constructed and case-split by consumers). Abstracting Validation would therefore break its clients on TWO axes (construction: UnresolvedCon Valid/Invalid; elimination: cannot match a hidden raw ctor) and would buy no invariant protection -- pure cost against the intrinsic-merits and small-auditable-surface principles. Spec confirmed no published spec/conformance/API intent requires Validation constructor abstraction and that strict-transparent selective import is contract-consistent (no Decision needed). So Validation's correct strict-roots treatment is transparent/selectively-importable: keep the raw constructors public and importable, retire ambient reach. NonEmpty's abstract flip stays on the parent node. depends_on is empty: the symbols this node imports (NonEmpty's nonempty_cons/nonempty_append/Semigroup_instance_NonEmpty smart-ctor+accessor surface) are already public from P1 (CAT-MIGRATE-TIER-C-NONEMPTY-SMART-CTORS, merged) and stay public whether or not NonEmpty goes abstract, so this node does NOT depend on NonEmpty's abstract flip landing."
---

> # RECUT 2026-09-07 (Steward), split out of CAT-MIGRATE-TIER-C-NONEMPTY-VALIDATION.
> # The Validation ABSTRACT flip is refuted; this node does the TRANSPARENT flip
> # instead. Architect design ruling evt_1wgz67t0nj1ne (component-design authority:
> # Validation should be strict-transparent, not abstract -- it carries no
> # representation invariant to justify hiding its constructors) + Spec enclave
> # confirmation evt_5m18jwangsdf8 (no spec/conformance/API intent requires the
> # abstraction; strict-transparent selective import is contract-consistent; no
> # Decision needed). This is a genuine flip-only change: imports added, nothing
> # hidden that lacks an API. Architect is the REQUIRED reviewer for the loading
> # semantics (the strict-roots loader / standalone-green work is what hard-stopped
> # twice on the parent's monolithic scope), but there is NO abstraction-boundary
> # soundness pass here -- Validation stays transparent, so no opaque view exists.

## What this is

The strict-roots migration of `Data/Sums/Validation.ken.md`, done as a
TRANSPARENT flip. Today Validation's constructors are reached AMBIENTLY by its
clients; the strict-roots goal is that every client names what it uses through a
selective import, with no ambient reach. This node makes Validation a strict,
selectively-importable module: `Valid`, `Invalid`, the `Validation` type, and the
existing smart constructors (`validation_pure` and any sibling) become public
importable symbols, and every client gains the selective import edge it needs.

It is NOT the abstract flip. The raw constructors `Valid`/`Invalid` stay VISIBLE
to clients -- a consumer keeps constructing `Valid (...)`/`Invalid (...)` and
pattern-matching `Valid rest -> ...; Invalid issues -> ...` exactly as it does
today. The only change is that those names now arrive through an explicit import
rather than ambient reach. No raw constructor is hidden, so no eliminator/fold or
Invalid smart constructor is invented, and no consumer's construct-or-match logic
is rewritten.

## Fixed inputs (RE-MEASURE at your cut -- lines drift; the parent node's D0
## census MISSED Validation's construct+match sites, so re-run it from scratch)

- **Validation defining module**: `catalog/packages/Data/Sums/Validation.ken.md`.
  Measure its current export surface (§7 Public API): the `Validation` type, the
  raw `Valid`/`Invalid` constructors, `validation_pure` (= `Valid` smart ctor) and
  any sibling smart ctor, and any law witnesses. This node makes the constructors
  and type PUBLIC IMPORTABLE; it hides nothing.
- **CORRECTED consumer census (the parent D0 miss).** The parent node's Schema
  client line recorded only "NonEmpty type + nonempty_cons" and MISSED that Schema
  directly CONSTRUCTS and ELIMINATES Validation's `Valid`/`Invalid`. Re-run the
  Validation-consumer census across ALL clients for BOTH raw construction AND
  pattern-match of `Valid`/`Invalid`, plus type use:
  - `Application/Input/Schema.ken.md` -- constructs AND matches `Valid`/`Invalid`
    (around :113-142, e.g. `Valid rest -> Valid (...) (Cons ...); Invalid issues
    -> Invalid (...) issues`; re-measure exact lines). Needs selective imports of
    `Valid`, `Invalid`, and the `Validation` type.
  - `Application/CommandLine/ArgParse.ken.md` -- re-measure its Validation use
    (type and/or constructors).
  - `Application/Configuration/Decoder.ken.md` -- names the Validation type (and
    possibly constructors); re-measure.
  - `catalog/examples/CommandLine/Forge.ken.md` -- names the Validation type (and
    possibly constructors); re-measure.
  Any client that names a `Valid`/`Invalid`/`Validation` symbol through ambient
  reach today needs a selective import the moment the producer goes strict.
- **Harness legs**: whichever cross-checks (cc-legs) load Validation move to the
  roots loader; extend the loader inventory to cover the newly-strict Validation
  module. Re-measure which legs are affected at your cut.

## Deliverables

- **D0 -- re-measure at the release SHA.** Re-run the CORRECTED consumer census
  above (construct AND match, every client), the Validation export surface, and
  the affected cc-legs. This node's whole premise -- that Validation-transparent
  is a logic-free import-wiring change -- rests on this census being complete
  where the parent's was not. Any drift is D0's to correct before authoring.
- **D1 -- export Valid/Invalid/type + smart ctors as public importable.** In the
  Validation defining module, make `Valid`, `Invalid`, the `Validation` type, and
  the existing smart constructor(s) public importable symbols (§7 Public API). Do
  NOT hide the raw constructors; do NOT add any new definition, Invalid smart
  constructor, eliminator, or fold. Pub-widening an existing symbol is a
  byte-unchanged body.
- **D2 -- add the selective imports; retire ambient reach.** Into each client the
  census names, add the selective import of exactly the Validation symbols it uses
  (`Valid`, `Invalid`, the type, and any smart ctor), keeping its natural
  construct-and-match unchanged. Retire the ambient reach so the client resolves
  Validation only through its explicit imports. Extend the roots-loader inventory
  to cover the newly-strict Validation module.
- **D3 -- honest reach.** The affected cc-legs run through the roots loader over
  the newly-strict Validation module; standalone exit 0 for each affected module.

## Acceptance criteria

- **AC-TRANSPARENT** (the inverse of the parent's AC-ABSTRACT): a CLIENT module
  CAN both CONSTRUCT (`Valid (...)`/`Invalid (...)`) and PATTERN-MATCH
  (`Valid rest -> ...; Invalid issues -> ...`) Validation's raw constructors
  through its selective import -- both resolve. Control: with the selective import
  REMOVED, the exact same construct and match sites fail to resolve (ambient reach
  is gone), proving the import is load-bearing and the reach is now explicit.
- **AC-IMPORT-EXPORTED** (per published symbol): each symbol this flip publishes is
  loader-resolved from a client, with a still-private-sibling control (a sibling
  NOT exported is NOT resolvable), proving the export surface is exactly what is
  named.
- **AC-EXACT-INVENTORY**: a per-symbol reddening mutation -- removing any one
  exported Validation symbol reddens exactly the clients that use it; the loader
  inventory covers exactly the newly-strict Validation module.
- **AC-STANDALONE-GREEN**: removing an added import line restores the exact prior
  standalone failure for that client (the import is load-bearing, not decorative).
- **AC-VISIBILITY-ONLY**: no new definition and no denotation change. Pub-widening
  of existing symbols is a byte-unchanged body; nothing is hidden. NO abstraction,
  NO Invalid smart constructor, NO eliminator/fold, NO consumer construct-or-match
  rewrite.
- **AC-NO-REGRESSION**: full `-p ken-elaborator` green in CI over the complete
  affected-target closure (every target that loads any module whose loading this
  flip changes, diff-touched or not); the affected cc-legs green. Targeted via
  `scripts/ken-cargo`, never `--workspace` -- green in CI is the workspace verdict.

## Parent's abstract-boundary notes -- re-evaluate, expected MOOT here

The parent node carried two Architect notes (z3570/z3660) that were scoped to the
Validation ABSTRACT flip. Under a TRANSPARENT flip they are expected to not arise;
confirm at D0 and drop if so, rather than carrying them as work:

- **(a) Validation law witnesses that are `pub proof`/`pub theorem`.** The concern
  was a public proof "dangling across the abstraction boundary." With Validation
  transparent there is NO abstraction boundary, so a public law witness does not
  cross one. Confirm the witnesses remain valid public symbols and drop the note.
- **(b) The removed no-Monad-registry guard behavioral replacement.** That guard
  removal was part of the abstract-flip scope. A transparent flip removes no such
  guard. Confirm the guard is untouched by this node and drop the note; if the
  transparent flip does touch it, assert the protected property as a positive
  behavioral check (do not merely delete it).

## Gate, reviewer, sequencing

`gate: none`. On the candidate: **Architect** (REQUIRED -- soundness pass on the
loading semantics / roots-loader inventory / standalone-green; there is NO
abstraction-boundary pass here because Validation stays transparent) + **Foundation
QA** + **CV** on the exact SHA, then Steward M1-M4 -> lieutenant M5-M9. Independent
of NonEmpty's abstract flip (no depends_on edge); the two may land in either order.
