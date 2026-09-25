---
id: LANG-QUALIFIED-CONSTRUCTORS
title: "Resolve qualified constructors T.C in expressions and patterns for every data type, add the per-type scoped-constructors property (a scoped family's constructors are reachable only as T.C, so only T is reserved), fail closed on module-versus-type ambiguity, and scope ResourceKind with its spec and prelude respelling in the same change"
status: active
owner: language
size: M
gate: architect
tier: T1
depends_on: []
blocks: []
github: null
origin: "Operator 2026-09-25: 'go with qualified constructors', adopting Architect design evt_7tb0f28wndkv5 under the specificity criterion ('Internal names should be selected such that enclose their meanings very closely'). Spec S0 (Architect APPROVE evt_74mna3r2fnk86 at 543b3ed01) states the rule in 32 §4, 33 §3.3 and 34 §1.1; the Architect's should-fix on that approval is AC-3 here. Precedes the L2 floor additions. Steward-filed per COORDINATION section 2."
---

# Qualified constructors and scoped families

## Objective

For every data type `T`, `T.C` resolves to the constructor `C` whose
kernel-recorded parent is `T`, in expressions and in patterns. A type marked
with scoped constructors binds no bare constructor names, so only `T` takes
space in the reserved set.

## Settled inputs

- **Spec** (S0, `spec/30-surface/34-data-match.md §1.1`, `33 §3.3`,
  `32 §4`) is the contract; this frame does not restate it.
  - `T.C` resolves to the same canonical `GlobalId` as an eligible bare use.
  - The property is per type, never per constructor.
  - A path that could mean both a module export and a type constructor is
    `AmbiguousReference`, even when both name one identity.
  - A bare scoped constructor in a pattern is unresolved, never a binder.
  - Coverage is keyed by constructor identity.
  - `T.C` never exposes a constructor hidden by abstract export.
  - `ResourceKind` is the scoped floor member; the other floor families stay
    bare.
- **Base, measured by the Architect at `0a6aa5987`.**
  - The parser accepts dotted `T.C` in expressions.
  - The elaborator does not resolve it: `Colour.Red` fails `UnresolvedCon`,
    and `Bool.True` fails `UnboundName`.
  - Module qualification lives in `modules.rs:456-612` (`exports`,
    `qualified_ids`).
  - A bare unresolved capitalized pattern name is already `UnresolvedCon`,
    not a binder.
  - ADR 0014 MRES-7 names qualified constructors as the collision escape.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

Elaborator resolution of `T.C` in expressions and patterns, the pattern
grammar production from 32 §4, the type-argument production (AC-3a), the
scoped property, and `ResourceKind` scoped. No kernel change and no
`trusted_base()` change.

## Acceptance

- **AC-1 (resolution).** For a local type, a prelude type and an imported
  type, `T.C` in an expression and in a pattern resolves to the exact
  constructor `GlobalId` a bare use gets. A same-shaped type from another
  module cannot donate its constructor. A private constructor stays hidden
  through `T.C`.
- **AC-2 (scoped property).** For a scoped type:
  - `T.C` resolves;
  - bare `C` is unresolved in expressions and patterns, and never binds a
    variable;
  - a user may bind bare `C` for their own purposes.

  A match over a scoped type written with `T.C` arms is exhaustive-checked by
  identity. A module export `T.C` beside a type `T` with constructor `C` raises
  `AmbiguousReference`, whether or not both name one id.
- **AC-3 (ResourceKind, Architect should-fix on S0).** `ResourceKind` becomes
  scoped. On this branch:
  - the prelude's `Resource FsHandle`-style registrations are respelled
    `ResourceKind.C`;
  - so is the normative text of `spec/30-surface/38-ffi-io.md` §1.7-1.9
    (`readAt`, `writeAt`, `writeAll`, `BufferHandle`), `39-elaboration.md`
    and `spec/70-behavioral/71-assumption-boundary.md`.

  Spec and implementation flip together. Catalog prose hits are respelled.
  Bare `Buffer`, `Mapping` and `FsHandle` are no longer reserved.
- **AC-3a (type position, Spec ruling on the conformance-validator gap).**
  AC-3 writes `ResourceKind.C` as a type argument (`Resource
  ResourceKind.Buffer`), which 32 §2 does not admit. On this branch:
  - `32-grammar.md` §2 admits a qualified constructor reference `T.C` as an
    `atype` in type application;
  - its resolution contract joins the qualified-reference rules in §3: it
    resolves to the canonical constructor identity, and `T` must resolve to
    that constructor's exact parent type;
  - `34-data-match.md` §1.1 cross-references it.

  The extension admits no other expression or projection in type arguments,
  leaves `tproj` unchanged, and makes no scoped constructor available bare.
  Module-versus-type ambiguity stays `AmbiguousReference`. Tests: `Resource
  ResourceKind.Buffer` elaborates to the same term as the pre-scoping bare
  spelling. A same-named constructor under another parent type is refused.
  The ambiguity case fails closed in type position too.
- **AC-3b (conformance, conformance-validator block `evt_73xcxjheqx3ap`).**
  The conformance validator adds black-box cases to `/conformance` on this
  branch, as its own commit on top of the implementation:
  - a qualified constructor in a type argument (`Resource
    ResourceKind.Buffer`) resolves to the canonical constructor;
  - a same-named constructor under another parent type is refused;
  - module-versus-type ambiguity in type position is `AmbiguousReference`.

  The same commit brings `conformance/surface/modules/seed-modules.md` and
  `conformance/surface/taxonomy/minimality.md` up to the landed
  fifteen-name floor and the `ResourceKind.C` spelling. That replaces the
  separate currency correction (`evt_6e58ajsz20rty`). The Spec leader
  reviews the conformance delta as the Spec-domain vote.
- **AC-4 (controls).** Each AC-1/AC-2 row is red on base. Reverting the
  resolution change reddens them. Existing bare-constructor suites stay green.
  Targeted builds only, through `scripts/ken-cargo`; no-regression means
  green in CI.

## Stop conditions

- Any kernel, telescope, reduction or `trusted_base()` change.
- A bare scoped constructor can become a pattern binder on any path.
- The spec and implementation would land on different branches for AC-3.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the child-2 checkpoint.

## Gate

Spec text (AC-3, AC-3a) and conformance cases (AC-3b) ride this branch. It
needs the Spec-domain vote as well as the Architect's and language QA's, on
the exact tip, and lands atomically.
