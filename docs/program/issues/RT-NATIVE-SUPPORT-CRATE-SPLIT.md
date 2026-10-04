---
id: RT-NATIVE-SUPPORT-CRATE-SPLIT
title: "The run-time support a native executable calls lives in the same crate as the Cranelift compiler backend, so the runtime archive contains the compiler and only link-time garbage collection keeps it out of an executable. Move the run-time support into its own crate with no compiler dependency, and link its archive instead"
status: active
owner: runtime
size: M
tier: T1
gate: architect
depends_on: [RT-CARRIER-RESIDUAL-TYPED-OPERAND, RT-NATIVE-LINK-DEAD-CODE-STRIP]
blocks: []
github: null
origin: "Operator 2026-10-03 on the Steward's executable-size measurement: 'frame both, trim first, split after RT-CARRIER'. Steward-filed per COORDINATION section 2."
---

# Run-time support is a crate without the compiler

## Objective

The archive a native executable links contains no compiler code by
construction. Nothing a program runs can depend on Cranelift, because the
crate that provides it has no Cranelift dependency.

## Fixed inputs (measured at `66d72ceb8`)

- **What a program calls.** The hello-world entrypoint object and its C stub
  reference only `ken_activation_v1_*`, `ken_boundary_store_v1_*` and
  `ken_selected_call_v1_*` (the 16 `extern "C"` functions in
  `activation_abi.rs`), the `ken_host_*` functions of `ken-host/src/abi_v1.rs`,
  and libc.
- **The closure.** Outside test modules, `activation_abi` reaches 12
  modules of `ken-runtime`: `activation_abi`, `activation_services`,
  `boundary_activation`, `boundary_resource_profile`, `boundary_value`,
  `canonical`, `hash`, `invocation_tickets`, `ir`, `native_int`, `store`,
  `values`. None of them names a `cranelift_*` crate or `cranelift_backend`
  except in doc comments (`activation_services.rs:9`, `boundary_value.rs:16`,
  `boundary_resource_profile.rs:248`).
  - On `5bad638bf` the closure also reaches `artifact_validation`:
    `BoundaryValueStore` stores `Option<RuntimeArtifactIdentity>`
    (`boundary_value.rs:2159`), defined at `artifact_validation.rs:50`. And
    `canonical` calls `unicode_normalization`'s `.nfc()` (`:406`, `:593`).
    Neither names a compiler item (runtime-implementer `evt_472a4ytd2c33w`).
- **The archive.** `ken-runtime` is `rlib` plus `staticlib`. The packaging
  finds `libken_runtime.a` through `ken_runtime_staticlib()`
  (`object_linker_packaging.rs:1478`). The one-archive rule (`:1470`) holds:
  the support archive owns the direction to `ken-host`, and `libken_host.a` is
  not linked beside it.
- **Consumers.** `ken-cli`, `ken-interp`, `ken-elaborator` and `ken-verify`
  depend on `ken-runtime`.

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it. RT-CARRIER edits some of
these files. Re-measure the closure on the base you start from.

## Scope

A new workspace crate, `ken-runtime-support`, which is `rlib` plus
`staticlib`. It depends on `ken-host` and on the non-compiler crates the
closure already uses, such as `unicode-normalization`, and never on a
`cranelift-*` crate or on `ken-runtime`. The measured closure, including
`artifact_validation`, moves into it.
`ken-runtime` depends on it, drops `staticlib`, and re-exports the moved
modules so consumers need not change their paths. The packaging links
`libken_runtime_support.a`. Do not change the `extern "C"` ABI, the emitted
object, or the C stub.

## Deliverable

1. Measure the closure on the base, then move it. A test module that needs
   compiler items stays in `ken-runtime`.
2. Turn the doc links to compiler items into plain prose.
3. Point `ken_runtime_staticlib()` and its search at the new archive, keeping
   the one-archive rule.

## Acceptance

- **AC-1.** `ken-runtime-support`'s `Cargo.toml` has no `cranelift-*`
  dependency, directly or through another crate, so support code that names
  the compiler fails to build. Reviewer check, not a test.
- **AC-2 (differential).** `nm` on `libken_runtime_support.a` finds no
  Cranelift symbol. On the base, an unstripped hello-world linked from
  `libken_runtime.a` has about 14,700. The
  product hello-world stays under 1 MB. Report its size with and without
  `--gc-sections` in the handoff. The size without the flag has no bound,
  because the archive also carries the whole Rust standard library.
- **AC-3.** `rt_parity_native` (186) and the packaging and activation suites
  are green in CI, along with every consumer crate.

## Stop conditions

- A closure module needs a compiler item at run time, or a support type has to
  embed a compiler type, which would give a dependency cycle.
- The split needs an ABI change.
