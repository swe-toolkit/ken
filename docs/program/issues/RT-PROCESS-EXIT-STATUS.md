---
id: RT-PROCESS-EXIT-STATUS
title: "ProcessExitStatus refusal in the escape lane (rt_escape r2_cross_buffer_freeze_fails_closed_with_invalid_bounds)"
status: draft
owner: runtime
size: TBD
gate: none
depends_on: [RT-SRCBODY-BIND-ORDER]
blocks: []
github: null
origin: Measured by the RT-SRCBODY-BIND-ORDER D12 complete no-fail-fast enumeration (evt_2n9wq8xyj0aa1). Fails at frozen base 21fd46dc as well as at the candidate, so it is pre-existing base debt and not a regression. Steward-filed (agents cannot create tracked work per COORDINATION §2).
---

> ## THE FRAME IS OWED. `draft`, NOT startable.
>
> It exists so a skipped CI row has an owner. **A skipped row measures nothing;
> this node owns un-skipping it.** Size is `TBD` deliberately.

## Exact signature — MEASURED, and it is NOT what this node was named for

> **The `id` and `title` of this node name a `ProcessExitStatus` refusal. The
> row does not produce one.** This block used to carry that name as the
> signature, with the honest caveat *"the signature above is the class, not yet
> the verbatim text."* **The verbatim text has since been measured and it is a
> different class.** Read this section, not the title.

**Measured at `04d4dd38a9cfb40da9b5d68bfeb42aaddbb76661`** by
`RT-IGNORED-FAILING-ROWS-INVENTORY` (row 13 of its ledger,
`docs/program/evidence/rt-ignored-failing-rows-ledger.md`):

```text
StaticResponseDeferred: a deferred host response is compiler control and
can only enter its exact response owner
```

The ledger records `label agrees? NO` for this row, noting the label predicts a
**Persistent-child / NoReferent ownership failure**. That prediction is not what
the row does.

**Superseded, kept so nobody re-derives it as new:**

```text
ProcessExitStatus refusal, rt_escape r2_cross_buffer_freeze_fails_closed_with_invalid_bounds
```

> **DO NOT RENAME THE NODE TO MATCH.** The id is cited from other artifacts and
> a rename breaks those citations for a cosmetic gain. **Fix the aim, not the
> label** — this section is the aim. If a rename is ever wanted, it is a
> separate deliberate act with the citations swept first.

**One more thing the ledger establishes about this row, and it is easy to
miss:** row 13 is **differential** — it calls `assert_native_matches_interpreter`
— even though its name advertises none of the four differential name patterns.
The ledger caught it **by body** after `§3`'s correction had excluded it **by
name**. So this row asserts native/interpreted agreement, and any framing that
treats it as a single-engine row is wrong at the first step.

## Why it is not one of the released owners

**Fits none of the five released owners** — the ring said so explicitly rather
than forcing it into the nearest one, which was the right call. **That judgment
was made against the old signature and has not been re-made against the
measured one**; re-check it before framing rather than inheriting it.

## Provenance

**Fails at frozen base `21fd46dc`.** The complete surface is 40 candidate
failures, all of which fail at the base — **zero regressions**, and the
candidate additionally **fixes six** base failures. Enumerated with
`--no-fail-fast`, which is a closed enumeration because fail-fast is per
**binary**, not per test.
