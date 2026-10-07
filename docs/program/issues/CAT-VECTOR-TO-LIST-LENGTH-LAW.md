---
id: CAT-VECTOR-TO-LIST-LENGTH-LAW
title: "Prove the length/to_list bridge spec 60 §5 defers, length (to_list xs) = n for xs : Vec a n, importing length from the trust-free base list module so Vector's trust delta stays zero"
status: merged
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [CAT-LIST-LENGTH-TRUST-FREE-BASE, CAT-VECTOR-TO-LIST-ZIP-LAWS]
blocks: []
github: null
origin: "Architect recut evt_17t0yjaee4atj, item (c). The law was already checked at 28d937604 against Derived's length, which raised Vector's cold trust by 5. Steward-filed per COORDINATION section 2."
---

# Vector's length/to_list bridge

## Objective

Vector proves `length a (to_list a n xs)` equals `n`, with `length` imported
from the trust-free base module, and its trust delta stays zero.

## Settled inputs

- The proof (`to_list_length`) checked at `28d937604` against Derived's
  `length` (Architect `evt_17t0yjaee4atj`). Only the import changes.
- `CAT-LIST-LENGTH-TRUST-FREE-BASE` puts `length` in a module with no
  imports. Spec 60 §7 requires zero `trusted_base()` delta for Vec.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

Import `length` from the base module, add `to_list_length` as a private
checked theorem, and make §5's bridge bullet name it.
Follow `docs/program/07-catalog-style-guide.md` and
`agent/playbooks/tools/write-ken.md`.

## Acceptance

- **AC-1.** `to_list_length` checks for every `xs : Vec a n`, and one
  generic client in Vector's `ken example` fence uses it at the literal
  goal `Equal Nat (length a (to_list a n xs)) n`. Control: a same-name
  reflexive weakening reddens the client at its own span, in `ken check`
  and in a CI-run target.
- **AC-2 (controls).** The cold Vector trust test passes with its trust
  assertions unchanged. Its `expected_owned_names()` gains exactly
  `to_list_length`, the rebaseline the test names for the next authorized
  Vector declaration extension.
- **AC-3 (mutation, QA).** Dropping the head element in `to_list` reddens
  AC-1.
- **AC-4 (readability).** The proof and its prose read under the style
  guide: local names state a proof endpoint or stage, and the formatter
  layout is unchanged. Reviewed independently by QA.

## Stop conditions

- Any trust delta, `Axiom`, kernel or spec change.

## Closeout

Merged `e1bbed2d0` from exact `3194e2f9b` (PR #4567), a respin of
`24a488fa6`, whose CI failed in a flat harness that preloaded Vector's
providers by hand without `Data.Collections.List`. Foundation QA
`evt_65j1w6xhxkm29`, Architect `evt_2myzjzamhachx`, Decision
`dec_7fhst9p05eqpd`. Vector proves `length a (to_list a n xs) = n` against
the trust-free `Data.Collections.List.length`, and cold Vector trust stays
109.
