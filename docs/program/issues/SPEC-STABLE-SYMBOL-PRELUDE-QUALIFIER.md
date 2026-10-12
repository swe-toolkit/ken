---
id: SPEC-STABLE-SYMBOL-PRELUDE-QUALIFIER
title: "Spec 46 section 3.2 says top-level declarations use a package and module-qualified stable symbol, but never says which package qualifies a compiler-declared prelude name. The producer uses the user's package, which is why a rebinding user declaration collides with the prelude. Name the prelude's qualifier"
status: draft
owner: spec
size: S
tier: T1
gate: architect
depends_on: [VERIFY-NATIVE-STABLE-SYMBOL-PRELUDE-IDENTITY]
blocks: []
github: null
origin: "Architect carry in D0 evt_x572bpfnsyj1 (VERIFY-NATIVE-STABLE-SYMBOL-PRELUDE-IDENTITY), for the spec-leader: spec 46 should name the prelude's qualifier. Steward-filed per COORDINATION section 2."
---

# Name the prelude's stable-symbol qualifier

## Objective

Spec 46 section 3.2 states which package qualifies the stable symbol of a
compiler-declared prelude name.

## Settled inputs (Architect D0 `evt_x572bpfnsyj1`)

- Section 3.2 requires a package and module-qualified symbol for
  top-level declarations. It does not name the qualifier for
  compiler-declared names.
- The producer qualifies them with the user's package. In the rebinding
  case that makes a prelude declaration and a user declaration coincide.
  `VERIFY-NATIVE-STABLE-SYMBOL-PRELUDE-IDENTITY` resolves this in favour
  of the prelude identity, giving the user declaration `owner#ordinal`.
- That departure is confined to programs the L2 strict-resolution flip
  will refuse. The qualifier question outlives the flip.

## Deliverable

One normative sentence or short paragraph in section 3.2 naming the
qualifier, consistent with the landed producer or stating the change it
requires.

## Acceptance

- **AC-1.** Section 3.2 names the qualifier, and the conformance validator
  confirms that the landed producer satisfies it, or names the delta.
- **AC-2.** No change to the hash inputs or to any other namespace.

## Stop conditions

- The wording requires a producer change or a hash change: return to
  the operator with the delta.
