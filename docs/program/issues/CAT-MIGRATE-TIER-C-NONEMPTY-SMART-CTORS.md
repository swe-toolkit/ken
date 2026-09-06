---
id: CAT-MIGRATE-TIER-C-NONEMPTY-SMART-CTORS
title: "Tier-C staging P1: add the two NonEmpty smart constructors (nonempty_singleton, nonempty_cons) as additive ambient pub fns. NonEmpty/Validation stay AMBIENT (no module boundary, no import lines); pub is inert under ambient loading, raw NonEmptyCons stays reachable, all harnesses stay green. The ctor-migration half of the abstract-export staging, decoupled from importability."
status: merged
owner: foundation
size: S
gate: none
tier: T2
depends_on: []
blocks: [CAT-MIGRATE-TIER-C-NONEMPTY-CALLSITE-SWAP]
github: null
origin: "Steward cut 2026-09-06, first step of the Architect's 3-step additive staging DAG (evt_5fxtzhk104q96) for the held Tier-C {NonEmpty, Validation} abstract-export migration. The Architect ruled the original monolithic flip (candidate 94b061de, superseded) reds cc7/cc8 because making NonEmpty a strict importable module hides its ctors from the ambient consumers (Schema/ArgParse) that still name raw NonEmptyCons -- spec 33-declarations §4.2/§4.3, module-level pub data is abstract-export only, and 'importable + public raw ctors' is not an expressible surface state. The insight: separate 'add smart ctors' (additive, ambient-safe) from 'become importable' (the flip). This node is the first: add the smart ctors while NonEmpty stays ambient. P2 (CAT-MIGRATE-TIER-C-NONEMPTY-CALLSITE-SWAP) swaps the raw call sites; P3 (the recut CAT-MIGRATE-TIER-C-NONEMPTY-VALIDATION) is the logic-free flip once no consumer names a raw ctor."
---

> # RELEASED 2026-09-06 to the foundation ring (lane-3). Step P1 of the
> # Architect's 3-step additive staging (evt_5fxtzhk104q96). PURELY ADDITIVE and
> # AMBIENT-SAFE: it adds two functions and changes nothing else, so every harness
> # (cc1/cc7/cc8 + full -p ken-elaborator) stays green. Base = current origin/main
> # (re-measure at cut). Seat foundation gpt-5.6-terra/medium (T2) is correct.

## What this is

The additive first step of retiring the {NonEmpty, Validation} fixture scaffold
onto the abstract-export end state, cut per the Architect's staging ruling. It
adds the public smart constructors NonEmpty will expose once it becomes an
abstract module -- but does it NOW, while NonEmpty is still ambient, so it breaks
nothing and requires no loading-model change. Importability and raw-ctor hiding
are deferred to P3 (the flip); the raw call-site swap is P2.

## The design, front-loaded (Architect evt_5fxtzhk104q96)

Under ambient loading there is no module boundary and the namespace is flat, so
`pub` is INERT (cf. the z3490 EC finding) and raw `NonEmptyCons` stays ambiently
reachable. Adding two `pub fn` wrappers is therefore pure addition: nothing that
currently resolves stops resolving. This is the exact property that makes the
staging sound -- all the code motion happens here and in P2, ahead of the flip.

Validation needs NO constructor add: its public construction is already via
`pub fn` (validation_map/pure/ap), and Invalid/Valid have no external raw
consumers (census evt_6s25z56pjzvcd). P1 touches NonEmpty only.

## Deliverables

- **D1 -- add exactly two smart constructors to NonEmpty** (the package's
  defining `.ken.md`), bodies verbatim per the Architect:
  ```
  pub fn nonempty_singleton (a:Type) (x:a) : NonEmpty a = NonEmptyCons a x (Nil a)
  pub fn nonempty_cons (a:Type) (x:a) (rest:List a) : NonEmpty a = NonEmptyCons a x rest
  ```
  Add them to the package Public API (§7) surface. Do NOT add a module boundary,
  do NOT add import lines, do NOT touch the raw `data NonEmpty`/`NonEmptyCons`
  declaration, do NOT touch Validation. Re-measure the exact insertion anchor at
  cut.

## Acceptance criteria

- **AC-ADDITIVE.** The diff adds only the two `pub fn` definitions (plus their §7
  Public API rows); no existing line's semantics change, no module boundary, no
  `import` line. NonEmpty/Validation remain ambient; raw `NonEmptyCons` remains
  ambiently reachable (control: an existing ambient consumer that names
  `NonEmptyCons` still elaborates unchanged).
- **AC-DENOTATION.** `nonempty_singleton`/`nonempty_cons` elaborate and reduce to
  the corresponding `NonEmptyCons` application, kernel-checked. Control: a checked
  example asserting `nonempty_cons a x rest` is convertible to
  `NonEmptyCons a x rest`.
- **AC-NO-REGRESSION.** Full `-p ken-elaborator` green in CI (COORDINATION §12);
  cc1/cc7/cc8 unchanged and green (this step touches no harness). Targeted locally
  via `scripts/ken-cargo`, never `--workspace`.

## Gate, reviewer, sequencing

`gate: none`. Reviewer per candidate: **Architect** (additive/mechanical pass) +
**Foundation QA** + CI -> Steward M1-M4 -> lieutenant. First of the 3-step DAG;
blocks [[CAT-MIGRATE-TIER-C-NONEMPTY-CALLSITE-SWAP]].
