---
scope: fleet
audience: (see scope README) — every seat that diagnoses a stack overflow in a
  test, every seat that reaches for `--exact` to isolate one, and every seat
  that reads a suite-versus-isolated difference as a headroom difference
source: 2026-09-15 research advisory 3 on the D2 instrument question
  (evt_1x623f2dwats0), measured from `library/test/src/` at rustc 1.97.1;
  independently re-verified at source by the architect (evt_5h3z5ac3wyhb9),
  who also closed the one condition the advisory left open. Assumed in the
  runtime lane the same night, against a live class-1 SIGABRT.
metadata:
  type: feedback
---

# `--exact` does not give a test more stack — libtest always spawns

A test overflows its stack. You isolate it with `--exact` (or
`--test-threads=1`) to look at it on its own, and the behaviour changes. The
reading that arrives unbidden is **"isolating it changed how much stack it
had"** — the main thread has 8 MiB, a spawned test thread has 2 MiB, so surely
running one test alone moved it onto the big stack.

**It did not. Isolation does not change a test's stack at all.**

## The measurement

From `library/test/src/lib.rs` at `rustc 1.97.1`:

- `run_tests` branches on `concurrency == 1` (line 429), and **both branches
  call the same `run_test`**.
- `run_test` always spawns when the platform supports threads:

      let cfg = thread::Builder::new().name(name.as_slice().to_owned());
      ...
      match cfg.spawn(move || runtest2.lock().unwrap().take().unwrap()())

  The spawn is guarded by `supports_threads`, which is false only for
  emscripten, wasm and zkvm. On Linux the test body **always** runs on a
  spawned thread.
- `grep -rn 'stack_size|RUST_MIN_STACK' library/test/src/` returns **zero
  hits**. libtest never sets a stack size, on any path.

So the body runs on a spawned thread taking std's default — `RUST_MIN_STACK`
if set, else the platform default (2 MiB on Linux) — in the full-suite
configuration **and** under `--exact`. Neither `--exact` nor
`--test-threads=1` moves a test onto the main thread's 8 MiB.

## Why it is unconditional in this tree

The equal-stack conclusion holds only if both launch paths pass the same
`RUST_MIN_STACK`. Measured at `origin/main` across `crates/`, `scripts/` and
`.github/`: **19 occurrences in `crates/`, 13 of them `env_remove(...)`
strippings, the rest comments, and zero SETTINGS anywhere.** Nothing in Ken's
tree sets it — not the local wrapper, not CI, not a test. Both configurations
inherit an ambient that is normally unset, so both get the same 2 MiB.

Those 13 strippings are also the mechanism behind the erratum in `1031f3c21`
("RUST_MIN_STACK does not reach a child that strips it"), so that erratum and
this measurement agree rather than compete.

## What follows, and it is the useful half

**Equal stack means a suite-versus-isolated difference is a change in DEPTH or
in FRAME SIZE, never in headroom.** That kills the explanation everyone
reaches for first and redirects the hunt to what actually varies with suite
membership: **process-wide state that an earlier test warms.** A `OnceLock` or
`lazy_static` interning arena, a memo table, an on-disk artifact cache,
allocator arena state, test-order-dependent input. A path that recurses less
deeply because a previous test already populated something is the standard
shape of "the suite passes and the isolated test does not".

**Frame cost is about liveness, not volume.** A Rust frame is sized by the
function's maximum live set, so anything live *across* a recursive call is
paid at every depth: a `String`/`format!` temporary costs `O(depth)`, while an
`#[inline(never)] #[cold]` recorder taking a small `Copy` and completing
*before* the recursive call is paid once. Where a probe's data is live matters
more than how much of it there is. Frame layout is also optimization-
dependent, so an instrument that fits at one `opt-level` can overflow at
another.

**Do not repair this class by raising the stack.** Growing the stack in
process — `stacker` / rustc's `ensure_sufficient_stack` — masks excessive
stack usage by rustc's own documentation of it, and when the property under
measurement *is* the consumption, statedness does not rescue it. See
[[stated-stacks]] for the governing standard; the repair for a depth
regression is a per-level frame reduction, not a provision.

## The detector

The belief is invisible because both readings predict the same observation:
"the isolated run has more stack" and "the isolated run recurses less deeply"
both explain an isolated run behaving differently, and nothing in the output
separates them. **Print the thread's actual stack and the ambient
`RUST_MIN_STACK` on both sides before attributing anything to headroom.** One
cheap read, and it either closes the question or turns up a genuine divergence
between two launch paths, which is itself the finding.

Related: [[a-negative-check-passes-for-any-reason-so-it-needs-a-positive-control]]
(two causes, one observation), [[verify-the-report-is-real-before-explaining-it]]
(a confident mechanism for a misread measurement launders it into a finding).
