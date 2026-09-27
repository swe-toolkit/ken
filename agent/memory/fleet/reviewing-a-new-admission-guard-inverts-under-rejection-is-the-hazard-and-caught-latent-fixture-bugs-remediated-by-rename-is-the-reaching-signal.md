---
name: reviewing-a-new-admission-guard-inverts-under-rejection-is-the-hazard-and-caught-latent-fixture-bugs-remediated-by-rename-is-the-reaching-signal
description: >-
  LANG-CONSTRUCTOR-NAMESPACE-SHADOWING-GUARD (8c6136fa3), a declaration-time
  guard rejecting a constructor spelling already bound to a constructor of a
  different sum family (flat ctor namespace). Verdict NO DEFECT. Method for
  reviewing a NEW admission / negative guard (one that only ADDS rejections):
  the soundness review INVERTS relative to an over-accept gate. (1) An
  admission-tightening guard cannot introduce a soundness gap by construction --
  it admits nothing new -- so the review reduces to three questions, not
  over-acceptance: (a) CATCH COVERAGE / under-rejection (does it catch every case
  in its stated scope? -- keys on the durable property, runs on every insert
  path, registry threaded persistently; a freshly-allocated-id exclusion for the
  "self" case cannot collide with a foreign id, so it cannot wrongly exclude a
  real collision); (b) OVER-REJECTION / completeness (does it reject a legitimate
  program? -- distinguish an Architect-ruled design rejection from a regression;
  confirm the common legitimate cases via negative controls); (c) WEAKENED-TO-PASS
  (was the guard relaxed to make fixtures pass, or did the fixtures have real
  latent bugs?). (2) A guard that CATCHES LATENT BUGS in pre-existing fixtures is
  strong REACHING evidence -- IF the remediation renamed/reframed the fixtures
  rather than weakening the guard. Here four fixtures had genuine cross-family
  collisions (one was unknowingly redeclaring the prelude's DecimalPair, a real
  silent-shadow) and were renamed. (3) A degraded diagnostic input (a missing
  span -> Span::zero) must not gate the rejection itself. Dual of the
  over-accept-gate fail-closed-arm lesson.
metadata:
  type: feedback
---

# Reviewing a new admission guard inverts: under-rejection is the hazard, and caught latent fixture bugs (remediated by rename) is the reaching signal

**Measured 2026-09-11 on `8c6136fa3` (LANG-CONSTRUCTOR-NAMESPACE-SHADOWING-GUARD),
a fresh review. Verdict NO DEFECT.** The change adds a declaration-time guard:
`guard_constructor_spelling` rejects a constructor spelling already bound in
`globals` to a constructor (`env.constructor(id).is_some()`) of a DIFFERENT
family (`!own_ctor_ids.contains(id)`), naming both sites, before the insert. It
closes a real hazard: a cross-family collision previously overwrote
`globals[name]` silently and surfaced downstream as an unrelated `TypeMismatch`.

## The review inverts for an admission / negative guard

For an over-accept gate the hazard is admitting too much. For a guard that only
ADDS rejections it is the opposite, and the framing must flip:

1. **It cannot introduce a soundness gap by construction.** An
   admission-tightening guard admits nothing new; it can only reject more. So do
   not hunt "what does it now wrongly accept" -- there is nothing. The review
   reduces to three other questions.
2. **(a) Catch coverage / under-rejection** -- the closest thing to a soundness
   concern. Does it catch every case in its stated scope? Check it keys on the
   DURABLE property (`env.constructor(id)`, true however the binding was
   inserted), runs on EVERY insert path (both `elab_data_decl` and
   `elab_explicit_data_decl`), and threads its registry persistently across
   declarations (`ElabEnv::ctor_decl_spans` through the module path). Check the
   "self" exclusion cannot swallow a real collision: `own_ctor_ids` are
   freshly-allocated unique `GlobalId`s, so a FOREIGN family's ctor id can never
   land in this family's set -- the exclusion only ever skips genuine same-family.
3. **(b) Over-rejection / completeness.** Does it reject a legitimate program?
   Separate an ARCHITECT-RULED design rejection (here: a flat ctor namespace
   rejects two distinct families sharing a spelling -- intended, coexistence not
   offered) from a regression, and confirm the common legitimate cases with
   negative controls (distinct-spelling families compile; single family with
   distinct ctors compiles). A whole-family redeclaration was already caught
   upstream (`DuplicateDefinition` on the type name), so the guard adds no new
   over-rejection there.
4. **(c) Weakened-to-pass.** See below.

## Caught latent fixture bugs, remediated by rename, is the reaching signal

The strongest evidence a new guard REACHES is that it catches LATENT bugs in
PRE-EXISTING fixtures -- but only if the remediation renamed/reframed the
fixtures rather than relaxing the guard. Here four fixtures
(MkProd/FokDerivInit/MkUnit/MkDecimalPair) had genuine cross-family collisions;
`surface_def_refinement` was unknowingly redeclaring the prelude's `data
DecimalPair = MkDecimalPair Int Int` -- a real silent-shadow the guard exposed
-- and was renamed to `DecimalPairT`, the guard untouched. Always check which
side moved: guard-weakened-to-pass is the failure mode; fixtures-fixed is the
success mode.
`[[two-arm-producer-needs-a-case-per-arm]]`
is the dual (a fail-closed over-accept arm needs its own negative test);
`[[green-vs-green-does-not-confirm-a-fix]]` is the family root.

## A degraded diagnostic must not gate the rejection

The guard reports `first_span` from `ctor_decl_spans.get(name)` with a
`Span::zero` fallback. Confirm the fallback only degrades the DIAGNOSTIC, never
whether it rejects: the rejection is gated on `globals.get(name)` +
`env.constructor`, not on the span registry being complete. A missing span yields
a worse message, still a correct reject.

## Scope boundaries to state honestly (not gaps this merge introduces)

Name what the guard deliberately does NOT cover, and why each is not a soundness
hole introduced here: within-family duplicate spellings (`data Foo = MkFoo |
MkFoo`) pass the self-exclusion and are left to pre-existing kernel /
`DuplicateDefinition` handling (both share one type, so no cross-family type
confusion); ctor-vs-def/type and cross-MODULE collisions are out of this
flat-namespace guard's scope. Behavior at those boundaries is UNCHANGED by the
merge, so they are not gaps this merge introduces -- say so plainly rather than
inflating them.
