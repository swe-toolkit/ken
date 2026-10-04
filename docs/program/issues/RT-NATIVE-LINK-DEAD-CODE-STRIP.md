---
id: RT-NATIVE-LINK-DEAD-CODE-STRIP
title: "Every native executable is about 20.7 MB (11.3 MB stripped) whatever it does, because the link step keeps the whole runtime archive, Cranelift backend and JIT included, though no run-time path calls them. Link with section garbage collection and strip, so an executable carries only the code it reaches"
status: merged
owner: runtime
size: S
tier: T2
gate: architect
depends_on: []
blocks: [RT-NATIVE-SUPPORT-CRATE-SPLIT]
github: null
origin: "Operator 2026-10-03 on the Steward's executable-size measurement: 'frame both, trim first, split after RT-CARRIER'. Placed at the RT-CARRIER-RESIDUAL-TYPED-OPERAND I-1 boundary. Steward-filed per COORDINATION section 2."
---

# Native executables carry only the code they reach

## Objective

An executable produced by the native packaging path contains only the runtime
code its program reaches. The hello-world Rosetta example is under 1 MB on
disk and prints what it prints today.

## Fixed inputs (measured at `66d72ceb8`, sizes from a release `ken` of 10-01)

- **The size.** Every executable is about 20.7 MB, or 11.3 MB after `strip`.
  Per-program code is 17 to 274 KB. `ken_host` is about 104 KB. The rest is
  `libken_runtime.a`, which carries the Cranelift backend and JIT.
- **The cause.** `ken-runtime` is built as `rlib` and `staticlib`
  (`crates/ken-runtime/Cargo.toml:18`). Its one archive is the starter's only
  runtime-support library (`object_linker_packaging.rs:1470-1478`). Rust packs
  a crate into a few large archive members, so any reference pulls in almost
  all of it, and the link passes no `--gc-sections`.
- **The link site.** `link_starter_executable`
  (`object_linker_packaging.rs:1427-1442`) is the only link command. It is
  called from `:857`, `:963`, `:1137` and the test at `:3127`.
- **The probe.** Relinking the same objects by hand with
  `-Wl,--gc-sections -s` gives 0.66 to 0.95 MB per program (hello-world 0.76
  MB), and each one runs. No Cranelift code survives, only eight source-path
  strings.
- **A symbol control.** `assert_no_undefined_native_int_service` (`:2839`)
  runs `nm -u` on a linked executable. On a binary with no symbol table it
  prints nothing and passes vacuously (check 8).

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## Scope

`object_linker_packaging.rs` and its tests. No change to the archive's
contents, the C stub, the emitted object, or the one-archive rule.

## Deliverable

`link_starter_executable` links with section garbage collection and strips
the symbol table from the product executable. Any test that inspects the
executable's symbols still sees the symbols it checks: it reads a table the
strip keeps, or it runs on an unstripped link. Name the choice in the handoff.

## Acceptance

- **AC-1.** The hello-world Rosetta example, built through the CLI, is under
  1 MB, and its stdout and exit status are unchanged. Each other Rosetta
  example that builds on the base still builds and runs. Report every size in
  the handoff.
- **AC-2 (control).** A test packages a starter executable through the
  product path and asserts its size is under 2 MB. Removing the
  garbage-collection flag reddens it.
- **AC-3.** The `nm -u` control is not vacuous. Pointing it at a binary with
  no symbol table makes it fail, not pass. `rt_parity_native` (186) and the
  packaging suites are green in CI.

## Stop conditions

- A run-time path needs a symbol that garbage collection or stripping removes,
  for example a `dlsym` lookup or a trap report that names symbols.
- The packaging targets a linker that does not take `--gc-sections`.

## Closeout

Merged `39992d356` from exact `be2f8cf58` (PR run 37168894136, after a
close and reopen cleared a check frozen at `in_progress`). Runtime QA
`evt_3mq6zrkncd5b`, Architect `evt_5cd1yekj815ze`, Decision
`dec_257h947hmf4ys`. A release hello-world drops from 20,882,072 to 762,880
bytes with identical stdout and status; the link always garbage-collects
sections and strips symbols unless a test retains them.
