---
id: RT-MAPPING-UNMAP-PROBE-SOUNDNESS
title: "consumed_mapping_is_actually_unmapped asserts mincore(addr,1) == -1/ENOMEM after munmap, but mincore observes PROCESS-GLOBAL address space -- so a sibling test thread's mmap can be handed the just-freed VA between the munmap and the probe. The test is UNSOUND, not flaky: unmap_v1 is correct. Fix by re-mmap PROT_NONE over the range to RESERVE the address before probing, which is the only one of the three candidate fixes that still fails if unmap_v1 breaks."
status: draft
owner: runtime
size: S
gate: none
depends_on: []
blocks: []
github: null
tier: T1
origin: "Adversary Finding 2 on the landed slice 4. Steward cut 2026-09-16, explicitly NOT filed as a flaky test -- see below. Observed as 121/1 by the lieutenant under the default parallel runner and 122/0 by the runtime-implementer single-threaded, on the same tree."
---

> # DRAFT. Not framed, not released. Do not start.

# Objective

Make `consumed_mapping_is_actually_unmapped` (`crates/ken-host/src/mapping_v1.rs`)
assert what it means to assert, by reserving the address range before probing it.

# The defect, and why the label matters

The test maps one byte, `munmap`s, then asserts `mincore(addr, 1)` returns
`-1`/`ENOMEM`. **`mincore` observes process-global address space.** Under the
default parallel test runner, a sibling thread's `mmap` between the `munmap` and
the `mincore` can be handed the just-freed virtual address, and the probe then
reports the page as mapped. That is the `121/1`.

**`unmap_v1` is correct. The test is unsound.** Those are different defects, and
the second is the one present.

**This node is not filed as a "flaky test", and the label is not cosmetic.**
"Flaky" invites a retry-until-green remedy, and an assertion that an unrelated
thread can falsify is not intermittently right — it is wrong, and it happens to
usually pass. The two readings have the same green and completely different
repair sets.

# The fix, and why the other two candidates are rejected

Three directions were proposed. They are not equivalent and the node says so
rather than listing them flat:

| direction | effect |
|---|---|
| **re-mmap `PROT_NONE` over the range before probing** | **RESERVES the address, making the probe sound. Still FAILS if `unmap_v1` breaks. This is the fix.** |
| accept ENOMEM-or-remapped | weakens the assertion to something the defect already satisfies |
| run the test isolated | hides it from the runner that exposed it |

The discriminator is *"does it still red if `unmap_v1` actually breaks?"* Only
the first survives it. The other two make the green more reliable and less
informative, which is the wrong direction for a test whose entire job is to
witness that the unmap happened.

# The reporting lesson, recorded because it generalises past this test

Two runs of the same commit disagreed — `122/0` and `121/1` — and the
disagreement was not reconciled at the time. Two measurements of one artifact
that disagree are a **finding**, not a scheduling annoyance; *"the other run was
flaky"* is the reading that stops the investigation.

The sharper form, and the reusable one: **a green from a check that usually
passes for reasons unrelated to its subject is not evidence, and it is
indistinguishable from evidence in a gate report.** The `122/0` in slice 4's
gate report carried no information about this test in either direction.

# Not this node

- `unmap_v1` itself. It is correct; changing it would be repairing the wrong
  object.
- Runner configuration, isolation attributes, or retry policy. Those are the two
  rejected directions wearing infrastructure clothes.
- Any other `mincore`-based probe. If the re-census finds siblings with the same
  shape, report them — a second instance makes this a pattern rather than a
  point fix, and that is the Steward's to re-cut.

# Sizing / tier

**Size S, tier T1.** A handful of lines. T1 because the review is of an argument
about what the assertion can and cannot observe, and because the obvious cheaper
fixes are the wrong ones for a reason that has to be understood rather than
looked up.

# Contention

`crates/ken-host/src/mapping_v1.rs`, test code only. Overlaps no live slice;
`crates/ken-host` is busy, so sequence against whatever is in flight there.
