---
id: LANG-ELAB-NATIVE-STACK-DEPTH
title: "Elaborator term walkers recurse on the native stack with depth equal to term depth and no guard. Once LANG-BRANCH-GOAL-BUDGET-VISIT-CHARGE removes the false refusal that capped goal size, a dependent-match goal with a closed spine about 160 deep aborts ken check with an uncoded stack overflow on the default 8 MiB stack. Close the class so a deep goal yields Ok or a typed refusal"
status: merged
owner: verify
size: M
tier: T1
gate: architect
depends_on: [LANG-BRANCH-GOAL-BUDGET-VISIT-CHARGE]
blocks: []
github: null
origin: "Architect merge review of LANG-BRANCH-GOAL-BUDGET-VISIT-CHARGE 4c1c69369 (evt_304zd75enpcqr): pre-existing overflow exposed by the repair, not a merge blocker, follow-up WP recommended. Steward-filed per COORDINATION section 2."
---

# A deep goal is checked or refused, never aborted

## Objective

`ken check` on a dependent-match goal with a deep closed spine returns Ok
or a coded refusal. It never dies of a native stack overflow.

## Settled inputs (Architect `evt_304zd75enpcqr`, on `4c1c69369`)

- **Repro.** Theorem `r1` from
  `lang_branch_goal_budget_visit_charge.rs`, with `build K Nil`, on an
  8 MiB thread. Debug K=160 aborts with rc 134; at 16 MiB it is Ok in
  3.3s. Release is Ok through K=640 on 8 MiB. A literal list at K=160
  and K=320 also aborts in debug.
- **Site.** lldb on the debug K=160 abort: 318 of 357 frames are in
  `refresh_embedded_elim_evidence` (elab.rs:3571), called from
  `plan_coherent_frame_motive` (:4747). It is a structural recursion over
  the simplified goal, about 26 KiB per Cons pair in debug, with no depth
  guard.
- **Class, not site.** The Architect ruled against bounding that one
  walker alone. Any elaborator walker that recurses on the native stack
  with depth equal to term depth has the same defect.
- **Stack today.** `ken-cli` runs elaboration on the main thread (8 MiB)
  and spawns no sized thread. Sized threads exist in elaborator test
  helpers (compiler_driver.rs:9030 `on_native_stack`, checked_core.rs:5912)
  and in `ken-verify` scenario.rs:661.
- **Two candidate closures (not yet ruled).** (a) Run `ken check` and
  `ken run` elaboration on a sized worker thread. (b) A depth bound on the
  walkers, refusing with a typed error. A sized thread moves the cliff
  rather than removing it, so the D0 must say what happens past it.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

D0 first, on the landed base with LANG-BRANCH-GOAL-BUDGET-VISIT-CHARGE
merged. The Architect measures which walkers a deep closed spine reaches
(by backtrace, not by reading), rules (a), (b) or both, and names the
depth or stack size and the refusal code. Then implement the ruling.

## Acceptance

- **AC-1.** `r1` at K=160, debug, through `ken check` or an 8 MiB thread,
  is Ok or a coded refusal, with `trusted_base()` unchanged.
- **AC-1b (every elaborating subcommand).** Closure (a) merged at
  `01acdfab6` covers only `check` and `run` (main.rs:30-33).
  `native-build` and `repl` still elaborate on the main thread: at K=160
  on 8 MiB, `ken native-build` aborts with rc 134 and passes at 16 MiB
  (Adversary `evt_25m7w6ssgxhya`; Architect carry 1, `evt_3gvjaqtc5vjz8`).
  The worker is drawn at the dispatch level for every subcommand that
  elaborates. A `native-build` K=160 row in
  `lang_elab_native_stack_depth.rs` is Ok and its built program runs. A
  failed worker spawn is a CLI front-end failure: it exits with a new
  appended `C` code (spec 70 §1.2), with its row in
  `crates/ken-diagnostic/codes.tsv` and `CLI_ERROR_TABLE` (carry 2). A
  worker panic is an implementation panic, which spec 70 §1.1 excludes
  from coding; it is propagated once, without a second panic message.
  Removing the worker from `native-build` returns that row to rc 134.
  Merged at `15661b590`.
- **AC-1c (every parsing subcommand).** The parser recurses once per
  nesting level, and `fmt` runs it on the 8 MiB main thread: `ken fmt
  --check` on `const deep : Nat = Suc (Suc (... Zero))` at N=400 aborts
  with rc 134 in `parse_fixed_additive_expr`, while `ken check` returns
  rc 0 at N=4000 (Adversary `evt_454shftbmsn2w`). The worker predicate
  in `main.rs` covers every subcommand that parses Ken source, which
  adds `fmt` to check, run, native-build and repl. A `fmt --check` row at
  N=400 in `lang_elab_native_stack_depth.rs` returns rc 0 or the coded
  C0016, and removing `fmt` from the predicate returns that row to rc
  134. `fmt` running time grows faster than linear at large N; that is
  out of scope here. Merged at `4573c8222`.
- **AC-2 (operator 2026-10-17, option 1; mechanism ruled by the Architect
  `evt_2cwbjvme2q90v`).** On unix, the worker subcommands (`check`, `run`,
  `native-build`, `repl`, `fmt`) re-execute the binary as a child that
  runs the unchanged 256 MiB worker, with inherited stdio. The parent
  reports `C0023` exactly when the child died of SIGABRT after a recorded
  guard fault. It propagates every other exit, and a spawn or wait failure
  is `C0022`. `C0023` is appended to `codes.tsv` and `CLI_ERROR_TABLE`
  (spec 70 §1.2). The rows, in `lang_elab_native_stack_depth.rs`, are:
  - **A.** N=1,000,000 through all five subcommands, both profiles: rc 1
    with one last-line `C0023`. Only the parser reaches the production
    limit; the elaborator and kernel walkers exhaust memory first.
  - **B (debug).** With `KEN_CLI_DEBUG_WORKER_STACK_KIB` lowered,
    `refresh_embedded_elim_evidence`, kernel `Term::clone` and the parser
    each return `C0023`. The 1048576 KiB clamp keeps rc 0.
  - **C.** The rows that are Ok today stay Ok through the child, including
    release K=640, AC-1c N=400 `C0016` and the `C0022` spawn row.
  - **D (linux).** An external ABRT gives rc 134 without `C0023`, and KILL
    gives 137.
  - **E (child authentication, Architect `evt_79312na4nzy20`).** The child
    role requires a marker `<fd>:<parent pid>:<dev>:<ino>` that names this
    process's actual parent and an open FIFO with that identity. An
    inherited `invalid` marker and a foreign `1:1:0:0` marker each give
    `C0023`. A parent-claimed marker with the wrong identity gives `C0022`
    before any work, with stdout byte-clean. The test helper clears the
    variable, with `RUST_MIN_STACK`, for every other row.
- **AC-3 (mutation, QA).** The ruling's M1-M4 (no recorder install, no
  fault-record test, no `cfg(unix)` block, knob ignored) and M5-M7
  (presence-only detection, no identity comparison, no `getppid`
  comparison) each redden their rows. Removing the worker still returns
  debug K=160 on 8 MiB to rc 134.

## Stop conditions

- Any kernel, conversion or `trusted_base()` change.
- A goal that checks today is refused.
- The census finds walkers outside the elaborator, such as the kernel
  checker, on the deep-spine path. That scope returns to the Architect
  and the operator.

## Hard-stop inventory (§1b)

§1a: the AC-2 process-boundary chain is at 1 (Architect `evt_79312na4nzy20`).

1. Child role taken from the presence of `KEN_CLI_WORKER_CHILD_FAULT_FD`,
   keyed on an inherited environment value rather than on the parent that
   created the channel.
