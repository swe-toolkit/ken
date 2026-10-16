---
id: DIAG-STDERR-LINE-PRESENCE
title: "Spec 70 section 1.1 makes a diagnostic one whole stderr line, but a coded line that follows program stderr output without a trailing newline starts mid-line, so a line-based consumer sees exit 1 and no code. A non-UTF-8 or empty subcommand prints help and exits 0 instead of failing with C0003. Repair both front-end presence defects"
status: ready
owner: runtime
size: S
tier: T2
gate: architect
depends_on: [DIAG-STABLE-ERROR-CODES]
blocks: []
github: null
origin: "Adversary M8 hunt of e17020310 (evt_4m548by7d3qp9), F1 and F2. Architect scoping evt_24r7336y78bmb: one follow-up WP, separate from DIAG-N, crates only. Steward-filed per COORDINATION section 2."
---

# Every coded line is present and whole

## Objective

Every tool diagnostic that `ken` emits is a whole stderr line beginning
`error[<code>]: `, including after unterminated program output. An
unrecognised subcommand of any spelling fails with C0003.

## Settled inputs (Architect `evt_24r7336y78bmb`, on `374fc72ee`)

- **F1: post-execution emitters.** Only two steps in `run_program_inner`
  (ken-cli `lib.rs`) can follow program bytes. `RunError::Io` covers X0001,
  X0002 and X0003, which `run_file` prints. `exit_status_with_reporter`
  covers H0003. Every other `RunError` is raised before `run_io`. The
  program cannot reach fd 2 through FS: `reject_symlink` is at
  ken-host `abi_v1.rs:403`.
- **F1: reproduced.** A program writes `"partial"` to stderr, then exits
  `Failure (add_uint8 255 1)`. The output is rc 1 with stderr
  `partialerror[H0003]: ...`. Writing `"partial\n"` gives a whole line,
  and the `Success` control gives rc 0 with stderr exactly `partial`.
- **F1: the ruled repair.** It adds a default no-op method
  `HostHandler::console_finish_line(stream)` in ken-interp. `PosixHost`
  tracks `stderr_mid_line` after each non-empty Stderr write, and its
  override writes one LF only when that flag is set. Both
  post-execution exits in `run_program_inner` call the method before the
  line is reported. The ruling gives the code.
- **H0003 is reachable from checked source.** A stuck `add_uint8` leaves
  the `Failure` payload non-`Int`. The test comment at ken-cli
  `lib.rs:938`, "Checked source cannot construct these results", is false
  for this case. Correct it.
- **F2.** `main.rs:29`
  (`args.get(1).and_then(|s| s.to_str()).unwrap_or(""))` folds an absent,
  a non-UTF-8 and an empty argv[1] into the help arm. Only an absent
  argument requests help. The ruled repair keys on absence and emits the
  existing `CliError::UnknownSubcommand` (C0003) for the other two. The
  ledger is unchanged.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The F1 and F2 repairs as ruled, the `lib.rs:938` comment corrected, and
CLI tests for the rows below.

## Acceptance

- **AC-1 (mid-line).** The F1 program gives rc 1, and stderr has a whole
  line beginning `error[H0003]: `, preceded by the line `partial`. The
  Adversary's X0003 fixture (an unterminated stderr write, then a stuck
  tree) gives a whole `error[X0003]: ` line.
- **AC-2 (precision).** The `"partial\n"` variant gives
  `partial\nerror[H0003]: ...` with no blank line. The `Success` control
  gives rc 0 with stderr exactly `partial`.
- **AC-3 (subcommand).** `ken $'\xff'` and `ken ''` each give rc 1 with a
  whole `error[C0003]: ` line. `ken` with no argument gives rc 0 and help.
- **AC-4 (mutation, QA).**
  - A no-op `console_finish_line` reddens AC-1.
  - An unconditional LF reddens AC-2.
  - Restoring `unwrap_or("")` reddens AC-3.

## Stop conditions

- A `codes.tsv` change.
- A change outside ken-cli and ken-interp's host seam.
- Program-outcome stderr bytes change for any program whose last stderr
  byte is LF, or that exits without a tool diagnostic.
