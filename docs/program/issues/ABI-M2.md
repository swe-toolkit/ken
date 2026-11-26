---
id: ABI-M2
title: "Program 10 §4 requires host-facility availability to be probed per operation at run time and unavailability to be a stable named result, but only the pidfd trio is probed: a kernel or seccomp policy that lacks statx, /proc/self/fd or SOCK_CLOEXEC reaches the program as PermissionDenied, CapabilityDenied or Other(errno), and Unsupported names no operation. Probe each kernel-dependent facility once per process and refuse its operations with one named unavailability result before dispatch"
status: ready
owner: language
size: M
tier: T1
gate: architect
depends_on: [ABI-M1]
blocks: []
github: null
origin: "docs/program/10-linux-abi-completion.md §4 Track M (operator directive 2026-07-25 to frame the program). L1b order (operator 2026-11-22: \"Approved. Use the language ring.\"). Steward survey on e3e71f5a4. Steward-filed per COORDINATION section 2."
---

# Host facilities are probed, and their absence is named

## Objective

For each host facility that a supported Linux kernel can lack or a policy can
deny, the host probes it once per process. While the probe reports it absent,
each operation that needs it returns one stable named unavailability result
before dispatch. The result says which operation is unavailable, and it is
never a silent fallback. An operation with its facility present behaves
exactly as today.

## Settled inputs (main `e3e71f5a4`; anchors re-checked at release)

- **The one probe that exists.** `probe_pidfd_availability`
  (`ken-host/src/process_v1.rs:352-385`) runs before every spawn (`:77`). It
  maps ENOSYS, EINVAL, EPERM and EACCES to `Unsupported`, which reaches the
  wire as `IoErrorIdentityV1::Unsupported` (`effect_v1.rs:4489`). It refuses
  the spawn with no fallback. Its test seam is `PIDFD_FAULT`
  (`process_v1.rs:418-460`; test `tests.rs:270-288`).
- **Unprobed kernel-dependent facilities.**
  - `statx`, for metadata and for directory entries of unknown type
    (`lib.rs:401-408`, `:512`).
  - `SOCK_CLOEXEC` on `socket` and `accept4` (`net_v1.rs:151`, `:309-313`).
  - `/proc/self/fd`, for tree removal and Unix socket paths (`lib.rs:565`;
    `net_v1.rs:187`, `:256`, `:287`).
  - `close_range`, through glibc `posix_spawn_file_actions_addclosefrom_np`
    (`process_v1.rs:485`). Its rc passes through raw (`:518-524`).
- **The misclassification.** Errors go through `io_error_identity_v1`
  (`effect_v1.rs:4478-4491`).
  - A seccomp EPERM on `statx` reads as `PermissionDenied`.
  - A missing `/proc` reads as `CapabilityDenied` (`net_v1.rs:187-188`).
  - EOPNOTSUPP is `Other(errno)`.
  - `Unsupported` is also used by stubs (`effect_v1.rs:2609-2714`) and by
    statx mask failures (`lib.rs:232-236`), and it names no operation.
- **What ABI-M1 left.** The manifest's `facility_version` is a build-time
  constant (`build.rs:412-440`), and nothing in it is per operation. The
  `SyscallNumber` facts are the natural join key. `RepresentedUnavailable`
  is a build-time class, not a runtime result.
- **Contention.** `RT-CLOCK-OP-NATIVE-PROMOTION` is building in
  `effect_v1.rs` and owns entropy, which is out of scope here.
  `RT-FS-SET-LENGTH-HOST-WIDTH-ADMISSION` is held, with WIP in
  `effect_v1.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

A per-process facility probe table in `ken-host`, the pre-dispatch refusal
for the facilities above (pidfd folded into the same table), one named
result, its wire code, a spec 38 sentence, and fault-seam tests. Out of
scope: entropy (CLOCK), new operations, fallbacks, kernel version checks.

## Acceptance

- **AC-0 (Architect, before code).** Rule:
  - the result: a new `FacilityUnavailable`-shaped identity naming the
    operation, or `Unsupported` narrowed to mean only this, and its wire code;
  - the facility set and the probe for each, including whether
    `close_range` is observable through glibc at all;
  - the operation-to-facility map, stated once;
  - whether children wait for CLOCK to land, given the shared
    `effect_v1.rs` paths.
- **AC-1.** For each facility, a fault seam on the `PIDFD_FAULT` pattern
  makes its probe report absent. Every operation that needs it then
  returns the named result, with no backend leaf reached. With the seam
  off, the same row succeeds. The paired rows differ only in the seam.
- **AC-2.** The misclassification rows: seccomp-style EPERM on the `statx`
  probe, and a missing `/proc`, give the named result and not
  `PermissionDenied` or `CapabilityDenied`.
- **AC-3.** One mutation per facility: a probe that always reports present
  reddens that facility's AC-1 row.
- **AC-4.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- A facility cannot be probed without performing the operation's effect.
- The named result needs a change to the trust boundary or the kernel.
- A probe would need a fallback to report success.

Checks: 4 fired. I read the pidfd probe and its mapping, the four unprobed
facility call sites, and `io_error_identity_v1` on `e3e71f5a4`. 6 fired.
The facility population is every kernel-dependent call the host makes, and
the survey found eight, of which entropy belongs to CLOCK. 8 fired. Each AC-1
pair differs only in the probe seam, so a host that ignores the probe fails
the absent row.
