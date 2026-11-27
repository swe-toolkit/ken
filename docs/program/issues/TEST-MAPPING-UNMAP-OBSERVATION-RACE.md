---
id: TEST-MAPPING-UNMAP-OBSERVATION-RACE
title: "consumed_mapping_is_actually_unmapped unmaps a page and then asserts that mincore on the freed address fails with ENOMEM, but libtest runs tests on parallel threads and any concurrent mmap can be handed the freed page first, so the row reddens with no product change. Make the observation race-free, so it proves the range was released rather than that nobody reused it"
status: ready
owner: language
size: S
tier: T2
gate: language-qa
depends_on: []
blocks: []
github: null
origin: "Architect finding evt_1h5cwex37895r, observed red in PX11 Inc2's R0 (evt_vmkjgh5hfpcn; disposition evt_17sefkhctvz3). Pre-existing on main; can red CI for any candidate. ABI-S6 mapping test in the Linux ABI program, placed on the L1b ring, which is idle and serves that program alongside L1 (operator 2026-11-22). Steward-filed per COORDINATION section 2."
---

# The unmap check observes the release, not the absence of a reuser

## Objective

`crates/ken-host/src/mapping_v1.rs` `tests::consumed_mapping_is_actually_unmapped`
passes if and only if `unmap_v1` released the range, whatever other threads
map concurrently.

## Settled inputs (Architect `evt_1h5cwex37895r`; anchors re-checked at release)

- **The row.** `:174`. It maps one page with `map_anonymous_v1`, unmaps it
  with `unmap_v1` (`:108`), whose `expect` at `:179` returned Ok in the
  failing run, and then asserts that `libc::mincore` on the address
  returns `-1` with `ENOMEM` (`:182-187`). The observed red was `left 0,
  right -1` at `:183`.
- **The cause.** `mincore` observes a process-global fact. Between munmap
  and mincore another libtest thread, the allocator or a new thread stack
  can map the freed page, and mincore then succeeds.
- **The file.** Blob `54d481cf1` on main `8653a01bd`; last changed by
  `816882102` (ABI-S6-D5B-COW-DISCHARGE). Inc2 does not touch it.
- **Two candidate forms, the owner rules.**
  - After `unmap_v1`, `mmap` the same range with `MAP_FIXED_NOREPLACE`.
    Success proves the range was free; unmap it again. `EEXIST` is
    inconclusive, not a pass.
  - Run the observation in a single-threaded child process.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

The one test row and any private test helper it needs. Out of scope:
`unmap_v1`, `map_anonymous_v1` and every other product function.

## Acceptance

- **AC-1.** The repaired row passes on the exact candidate, and its
  observation cannot be satisfied by a concurrent mapper. The handoff
  states which outcome the row treats as inconclusive and how it handles
  it, without counting it as a pass.
- **AC-2, mutation.** Make `unmap_v1` return Ok without calling munmap.
  The repaired row reddens. Restore byte-identically.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The race-free form needs a change to `unmap_v1` or another product
  function.
- The only available form retries until green or loosens the assertion.

Checks: 4 fired. I read the row and `unmap_v1` on `8653a01bd`; the failing
assertion is the mincore status after a successful munmap. 8 fired. AC-2
keeps the mapping alive, so a row that still passes cannot be observing the
release. 6 fired. One row is in scope; the Architect found no added tests,
threads or mappings between main and Inc2's ken-host population.
