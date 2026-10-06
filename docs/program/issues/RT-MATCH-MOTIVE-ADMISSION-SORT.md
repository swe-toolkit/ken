---
id: RT-MATCH-MOTIVE-ADMISSION-SORT
title: "Checked-core body-view admission classifies every elaborated Match motive as Dependent, because inspect_non_dependent_motive expects an ascribed motive the elaborator never emits. Key admission on the motive's checked sort, so the ConstantType and ProofOnly arms see their real population"
status: active
owner: runtime
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Architect carry evt_37bbprs1gwajn in the RT-CHECKED-JOIN-SITE-MATCH-POPULATION AC-0 ruling: the second decoder with the same ascript-first assumption, explicitly not repaired by that node's shape fix, because admission needs the sort and a bare lambda does not carry it. Steward-filed per COORDINATION section 2. Measured at origin/main e771f7e7d."
---

# Match body-view admission keys on the checked sort

## Objective

`validate_supported_match_motive` admits or refuses a non-indexed Match by
what its motive is: a constant type, a proposition, or dependent. That holds
for motives the elaborator actually emits, and no metadata hatch stands in
for the classification.

## Settled inputs (Architect `evt_37bbprs1gwajn`, measured at `e771f7e7d`)

- Elaborated Match motives are bare `lam <dom> <body>`. The leading tags
  were `lam, ind_former` or `lam, app` in 44 of 44 probed exits.
- `inspect_non_dependent_motive` (`checked_core.rs:4380`) returns
  `Dependent` unless the first tag is `ascript`. The ascribed shape is
  minted only by the unit-test constructor `constant_type_motive` (`:4862`).
- `validate_supported_match_motive` (`:4333`) admits `Dependent` only when
  some metadata entry starts with `HostEffectSpineV1\0` (`:4354`). That
  header is written by `compiler_driver.rs:3895`.
  - So in a host program every non-indexed Match is admitted through the
    hatch.
  - Without the header, every one is refused as
    `UnsupportedDependentMotive`.
  - The `ConstantType` arm and the `UnsupportedProofOnlyMatch` refusal
    never see an elaborated motive.
- The join-site shape fix (`a7d46d6f2`, never landed; its WP closed without
  repair, `evt_1xj5yh86d75st`) accepted the bare `lam` only because
  `record_match`'s answer-symbol head gate supplies the sort. Do not copy
  it here: admission has no such gate.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. **AC-0, at the start of the repair.**
   - On the six default D1 targets, log each
     `validate_supported_match_motive` call: owner, family, the arm taken,
     and whether the hatch decided it. For every hatch-admitted motive, also
     log the sort of its result type as the checker knows it.
   - Report the distribution and the census of proof-sorted motives the
     hatch admits.
   - The Architect then rules what admission keys on: the sort carried on
     the motive (the elaborator emits an ascribed motive) or a sort read
     from checked information already delivered to checked core. The
     Architect also rules whether a proof-only Match in a host program is
     refused or erased.
2. **The ruled repair.** `inspect_non_dependent_motive` classifies
   elaborated motives by the ruled key, and the `HostEffectSpineV1` hatch no
   longer decides admission.

## Acceptance

- **AC-1 (pins on elaborated programs, not constructed bytes).**
  - A host program with a non-dependent Type-valued `match` takes the
    `ConstantType` arm.
  - A Prop-valued `match` takes the ruled `ProofOnly` behaviour.
  - A dependent one takes the ruled `Dependent` behaviour.
- **AC-2 (falsifier).** Restoring the `ascript`-first early return sends the
  first two pins back to `Dependent`.
- **AC-3.** All six default targets pass, and `rt_parity_native` is 186/186
  at 4 threads. Any new refusal is either one the ruling names or a stop.

## Stop conditions

- The ruled key needs a kernel, trust or spec change: the Architect routes
  it to the operator.
- Emitting a sort changes motive bytes that a fingerprint, cache or
  conformance row pins: stop with the consumers listed (CHECKS 3).
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`.
