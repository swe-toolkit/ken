---
id: SPEC-39-LOADER-FLOOR-FIFTEEN
title: "Reconcile spec 39 §2.0's source loader with the closed fifteen-member prelude floor of 30 §4 and 33 §3.3: replace its stale ten-name type set and 'reuses all ten' with the fifteen-name roster and the loader's reuse of all fifteen exact checked identities; spec-only, and it gates Language QA on LANG-PRELUDE-FLOOR-FIFTEEN"
status: ready
owner: spec
size: S
gate: architect
tier: T1
depends_on: []
blocks: [LANG-PRELUDE-FLOOR-FIFTEEN]
github: null
origin: "Spec-leader ruling evt_31mren189yexz on Language QA question qst_3r4pwa5jr5k2j (LANG-PRELUDE-FLOOR-FIFTEEN candidate 371a4680f): the settled fifteen-name floor controls; 39 §2.0's ten-name set is stale conflicting normative text. Serves the L2 objective. Steward-filed per COORDINATION section 2."
---

# Spec 39 §2.0 names the fifteen-member floor

## Objective

`spec/30-surface/39-elaboration.md §2.0` states the same closed prelude floor
as `30 §4` and `33 §3.3`, so a conforming loader built from `§39` admits
exactly the fifteen floor types.

## Settled inputs -- Spec leader `evt_31mren189yexz`, at `6bdd75394`

- **Controlling text.** `30-taxonomy.md §4` (`:198-202`) and
  `33-declarations.md §3.3` (`:258-259`) fix the closed fifteen-member floor
  `{Auth, Bool, Bottom, Char, Equal, List, Nat, Option, Pair, Prop, Proved,
  ResourceKind, Result, Top, Utf8Error}`.
- **Stale text.** `39 §2.0` step 4 (`:94-95`) gives the ten-name set
  `{Auth, Bool, Char, List, Nat, Option, Pair, ResourceKind, Result,
  Utf8Error}`. The reuse paragraph (`:119`) says the loader "reuses all ten
  compiler-installed, kernel-checked type `GlobalId`s".
- **The five added members are kernel-machinery-keyed** (`30 §4`
  `:180-197`). `Top` and `Bottom` are kernel `Ω₀` constants, `Proved : Top`
  is their canonical proof, `Equal`'s body is the kernel's `Eq`, and `Prop`
  aliases `Ω₀`. Each is a kernel constant or a re-checked definition outside
  `trusted_base()`.
- **Not changed:** the constructor and Pair-companion inventories as already
  specified, the floor itself, and any implementation.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

Amend `39 §2.0` step 4 to the fifteen-name roster, and the reuse paragraph to
say the loader reuses all fifteen exact checked identities. The Spec author
owns the wording. The wording must be true of all five kernel-keyed members,
including `Proved`, a proof term rather than a type former.

## Acceptance

- **AC-1.** No normative statement in `spec/` presents a floor type set
  other than the fifteen as current. Sweep `spec/` for `Utf8Error}`, the
  word `ten` near "floor" or "prelude", and each roster spelling. Post the
  hit list with each hit classified as current-fifteen, historical (for
  example `30 §4`'s "the former ten"), or stale. A stale hit outside `39` is
  a STOP.
- **AC-2.** The diff touches only `spec/30-surface/39-elaboration.md`, with
  no constructor or companion inventory change. Any other needed edit is a
  STOP.

## Stop conditions

- Reconciling needs a floor change, a new inventory entry, or an
  implementation change.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the child-2 checkpoint.

## Gate

Spec-domain vote (conformance validator) and the Architect, on the exact SHA.
Once it lands, Language QA re-evaluates `LANG-PRELUDE-FLOOR-FIFTEEN`.
