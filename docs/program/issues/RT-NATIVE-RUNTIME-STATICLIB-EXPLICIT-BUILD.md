---
id: RT-NATIVE-RUNTIME-STATICLIB-EXPLICIT-BUILD
title: "A native build finds ken-runtime-support's static archive only as a side effect of Cargo building a dependency's staticlib crate type under the test binary's target directory, and on the GitHub runner image carrying Cargo 1.100.0 that archive is not left where the finder looks, so every ken-cli native row on that image fails Toolchain/ken_runtime_staticlib and CI on main is red with no source change. Make the archive's production explicit, so the native build no longer depends on Cargo's side-effect output"
status: merged
owner: runtime
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Steward base-blocker evt_306smntr31x4v; Architect attribution and repair families evt_1h7zbhf3w15m7; Steward D-a evt_4acwvf6pz4h0z (runner image split). Steward-filed per COORDINATION section 2."
---

# The runtime support archive is built explicitly

## Objective

A native build in CI and locally locates a `ken-runtime-support` static
archive that the build produced on purpose. The finder no longer relies
on Cargo leaving a dependency's staticlib beside the test binary. CI is
green on runner images carrying Cargo 1.99.1 and 1.100.0 alike.

## Settled inputs (main `e5177f7c1`)

- **The finder.** `ken_runtime_staticlib`
  (`crates/ken-runtime/src/object_linker_packaging.rs:1556`-`:1593`)
  searches `current_exe().ancestors().take(4)` and each one's `deps` for
  `libken_runtime_support.a` or `libken_runtime_support-*.a`. If it finds
  none, it refuses with "Cargo did not materialize the required
  ken-runtime-support static archive".
- **The producer.** `crates/ken-runtime-support/Cargo.toml:13`-`:14`
  declares `crate-type = ["rlib", "staticlib"]`. The shard job builds with
  `cargo build --workspace --tests --profile test --locked`
  (`.github/workflows/ci.yml:232`). No step builds the archive by name.
- **The environment split.** The job runs on `ubuntu-latest` with no
  container, no toolchain step and no `rust-toolchain.toml`. Red runs
  44306662782 and 44318808979 each failed one shard, and both failing
  shards ran on image 20261116.202.1, which carries Cargo and Rust
  1.100.0. The other shards built the same merge tree on 20261111.x, with
  1.99.1, and passed their ken-cli native rows (Steward
  `evt_4acwvf6pz4h0z`).
- **The base is not the cause.** `e5177f7c1^{tree}` is the tree that green
  run 44295811327 tested. No Inc2 path is a manifest, lockfile, build
  script or workflow (Architect `evt_1h7zbhf3w15m7`).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Amendment: AC-0 ruling (2026-11-18)

Architect D0 `evt_7af482gtgwkfy` rules the explicit build, with no
toolchain pin.

- **The producer.** A new `scripts/build-runtime-support-archive` prints
  the one `.a` path that Cargo's compiler-artifact JSON reports for
  `ken_runtime_support`. If Cargo reports any other number of archives,
  it exits non-zero.
- **The channel.** CI and `scripts/ken-cargo` set
  `KEN_RUNTIME_SUPPORT_STATICLIB` from the producer's output.
- **The finder.** When the variable is set, the finder uses the file it
  names or refuses; there is no fallback. When the variable is unset, the
  side-effect search remains as the fallback.
- **CI.** The test-shard, native-write-partition and native-rt-parity jobs
  each gain one step after their build. It prints
  `CI_RUNTIME_SUPPORT_ARCHIVE` with the archive path and the Cargo
  version.

The D0's exact code, AC-1, AC-3 and outcomes S1 to S3 govern. AC-2 below is
replaced by the D0's toolchain-independent form:

- With the side-effect archive present, run the row with the variable
  naming an absent file. It must refuse with the "names ... which is not
  a file" reason.
- The mutant deletes the variable branch. The same command must then pass
  through the fallback. Restore the branch byte-identically.

## Scope

The archive's production and the finder's lookup. Either an explicit
build of the support archive or a toolchain pin, as AC-0 rules, together
with the `ci.yml` steps that change. No change to the archive's contents,
the linked symbols, the packaging stages, or any refusal other than this
one.

## Deliverable

One runtime candidate.

## Acceptance

- **AC-0 (D0, Architect, before code).** Rule the family. The options are
  an explicit build of `ken-runtime-support`'s staticlib, by CI or by a
  build-time producer the finder names, or pinning the shard toolchain.
  The Architect prefers the explicit build, because it removes the
  dependency on Cargo's behaviour rather than this instance of it. Rule
  where the archive is produced, how the finder names it, and whether the
  side-effect search is kept as a fallback.
- **AC-1.** On Cargo 1.100.0 with a fresh target directory,
  `k3_literal_char_native` passes, and the archive the finder returns is
  the one the ruled producer wrote. Report `cargo --version`.
- **AC-2.** With the ruled producer disabled, the same row refuses
  Toolchain/ken_runtime_staticlib on 1.100.0. Restore it byte-identically.
  This shows the producer, not a leftover side-effect archive, is what the
  finder relied on.
- **AC-3.** Full-workspace CI is green on the exact candidate. Each test
  shard's "Set up job" image version is read and reported, and at least
  one shard ran on a Cargo 1.100.0 image.

## Stop conditions

- The repair needs a change to the archive's contents, the link line, or
  `§4`'s one-archive rule.
- Cargo 1.100.0 also stops producing the staticlib when it is requested
  by name.

Checks: 1 fired. The environment attribution rests on two independent
measurements: the per-shard image versions from the job logs, and the
tree identity that the Architect measured from the object DB. 2 fired.
The finder has one producer, Cargo's side-effect staticlib, and no step
names it. That is why AC-2 disables the ruled producer to show the
finder now depends on it. 9 fired. The toolchain was an uncontrolled
axis, and AC-3 reads it per shard instead of assuming it.
