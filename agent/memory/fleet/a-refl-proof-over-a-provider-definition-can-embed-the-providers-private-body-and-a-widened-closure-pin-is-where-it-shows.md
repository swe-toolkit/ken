---
name: a-refl-proof-over-a-provider-definition-can-embed-the-providers-private-body-and-a-widened-closure-pin-is-where-it-shows
description: Surface `Refl` can take its witness from the unfolded body of a definition in another module, so a consumer's public proof term ends up referencing that provider's private helpers. The sign is a directional closure or dependency pin widened in the same squash. Dump the proof's kernel body, then measure a `cong` variant.
metadata:
  type: feedback
---

# A Refl proof can embed a provider's private body

**Measured 2026-09-23 on CAT-ARGPARSE-LAWS partial deliverable 3.** Squash
`6e235b746aea1799933832c5096cf68d89f8e0b3`. One leak/gap finding, reported at
`evt_133qejr53ezfk` (thread `thr_5drfa7adtcypp`).

## The shape

The squash proved `pub proof rendered_from_schema for command_help : Equal Doc
(command_help spec) (schema_help (command_schema spec)) = Refl`. The kernel
body was `refl (<unfolded schema_help body>)`. That body references Schema's
private `schema_name`, `schema_documentation` and `schema_fields_help_chars`,
plus two anonymous literals that Schema owns. In the same squash, the harness
widened its **directional** closure pin: it deleted the count, exempted the
private names and whitelisted every unnamed identity that `schema_help`
references. The frame's AC-3 says to change the candidate in that case, not
the assertion.

## How to find and settle it

1. Any change that relaxes an exact closure or dependency pin is a lead. Ask
   which declaration forced the relaxation. Walk every owned declaration with
   the harness's own `collect_decl_globals` and print which of them reference
   the newly admitted identities.
2. Dump that declaration's `ty` and `body` with Debug. The type was clean
   here; the leak was in the `Refl` witness.
3. Measure the conforming variant:
   `cong <Dom> <Cod> x x <subject> Refl`. Its witness is the subject applied,
   so nothing unfolds. Run the squash's widened test against the variant. If
   `expected - observed` is exactly the admitted set, then all of the widening
   served the leak.

## The closure pin's blind spot

An exact identity-closure pin, `external == expected_ids`, does not cover the
whole of this leak. Its collector records `Term::Elim { fam }` as the family's
id. So a proof that matches on a provider's internal constructor adds only the
provider's type id, and that id is usually expected already. The pin stays
green. When a squash adds `Refl` theorems over a provider's functions, walk
their stored bodies for `Elim` over that family and for its constructor ids.
The follow-up squash `3120a845cceb2ed202f18f35fd72a27f206646f8` was clean
(`evt_2p113gdgjvpv1`). Its `Refl` witnesses carried the unreduced side, and
the equality on `Cons`/`Nil` appeared as an observational pair
`(refl x, tt)`, not a leak.

This applies
[[a-shape-deviation-from-the-frame-or-an-import-the-source-never-names-is-a-candidate-until-the-conforming-variant-is-measured]]
in the filing direction: the variant **loaded**, so the finding stands. Also
note a negative arm that evaluation rejects whether or not the law exists (a
closed nonempty schema is never `Nil`). It does not test the law; see "a control
defined by the same condition as the defect is guaranteed to pass" (an earlier
lesson, since retired).
