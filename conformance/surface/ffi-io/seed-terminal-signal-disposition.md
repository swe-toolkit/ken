# Executable-edge signal and terminal conformance (ABI-S5)

Format: `../../README.md`. Spec: `spec/30-surface/38-ffi-io.md §1.10`.
These are live-process observables, not a source-code inventory or an official
conformance-runner verdict. The execution paths differ: `ken run` executes Ken
in the CLI's worker child; a native artifact is C-started. The named Rust
witness is `crates/ken-cli/tests/abi_s5_terminal_signal_disposition.rs`.

## surface/ffi-io/terminal-is-total-bool

- promise class: durable invariant
- given: a terminal and a nonterminal stream descriptor supplied to the
  existing `is_terminal : Stream → IO Bool visits [Console]` operation.
- expect: `True` exactly for a terminal descriptor and `False` otherwise,
  without an `IOError` arm. This is the existing operation; this case does not
  ask for a new host operation, ABI field, or effect.
- control: treating a nonterminal descriptor as a terminal changes the Bool;
  adding a fallible `IOError` result changes the stated type.

## surface/ffi-io/run-worker-signal-posture

- promise class: normative compatibility vector for the executable edge
- given: launch `ken run` from a clean baseline (all catchable signals
  1–64 except glibc-reserved 32 and 33 set to `SIG_DFL`, empty mask), hold
  stdin open, and inspect the actual worker once a Ken program is blocked in
  `read(0)`. Inspect the worker, not only the CLI parent.
- expect: over signals 1–31, ignored is exactly `{SIGPIPE}` and caught is
  exactly `{SIGSEGV, SIGBUS}`. Every worker thread has an empty `SigBlk`.
  Ignored signals from 32 upward are absent; caught signals from 32 upward
  exclude only glibc-reserved 32 and 33. Closing stdin reaches the successful
  read arm, exit 41. The parent also ignores SIGPIPE and catches SIGSEGV and
  SIGBUS; the worker adds a fault recorder over the std handlers.
- controls: an extra SIGINT ignore changes the ignored set; resetting SIGPIPE
  changes it and makes the existing closed-stdout BrokenPipe test terminate by
  SIGPIPE. The test names `ken_run_worker_signal_posture_is_exact` and
  `console_exec::closed_stdout_is_an_io_failure_not_sigpipe_termination`.

## surface/ffi-io/native-artifact-signal-posture

- promise class: normative compatibility vector for the executable edge
- given: build a checked program with `ken native-build`, start the C-linked
  artifact from the same clean baseline, hold stdin open, and inspect the
  artifact process blocked in `read(0)`.
- expect: over signals 1–31, ignored is exactly `{SIGPIPE}` and caught is
  empty; every artifact thread has an empty `SigBlk`. The higher-signal
  constraints are as above. Closing stdin selects exit 41.
- controls: an extra SIGINT ignore changes the ignored set; skipping the C
  SIGPIPE posture changes it and makes the existing artifact closed-stdout
  BrokenPipe test terminate by SIGPIPE. The test names
  `native_artifact_signal_posture_is_exact` and
  `px4b_native_production.rs` test
  `linked_console_broken_pipe_reaches_ken_instead_of_signal_termination`.

## surface/ffi-io/spawned-child-restores-ken-dispositions

- promise class: durable invariant
- given: either live executable edge, including a launcher that already ignores
  SIGHUP and SIGINT and blocks SIGUSR1.
- expect: a spawned child retains those launcher dispositions and the inherited
  mask unchanged, but starts with Ken's SIGPIPE action reset to `SIG_DFL`.
  Caught SIGSEGV/SIGBUS on the run path reset on `exec`; the C-started artifact
  has none. glibc-reserved signal 33 is not a Ken-established action and its
  caught action resets on `exec`. No mask-clear operation is required or
  permitted by §1.10. Fatal re-raise paths have no surviving child state.
- control: an implementation that resets every signal and empties the mask
  loses launcher state; one that leaves SIGPIPE ignored passes an inherited
  ignore to the child. This is the target for PX10's spawn plan, not a claim
  that PX10 is implemented by this seed.

The live-process tests use a clean `pre_exec` launcher so an inherited SIGINT
ignore cannot make the extra-ignore mutant pass. The per-path observations are
not interchangeable: skipping the C posture does not change the run worker.
