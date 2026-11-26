# Monotonic clock and kernel entropy

Format: `../../README.md`. Spec: `spec/30-surface/38-ffi-io.md` §1.15;
Strict homes: `spec/30-surface/33-declarations.md` §3.3 and
`30-taxonomy.md` §4.

These are post-promotion contract cases. Run them against both the checked
interpreter and native lanes only after RT-CLOCK-OP-NATIVE-PROMOTION is
assembled; do not run them against pre-promotion main. Compare the observable
contract, not readings or random bytes from separate host runs. No entropy
quality or cross-process epoch claim is made.

## Strict imports

### surface/ffi-io/clock-entropy-homes-require-explicit-imports

- promise class: durable provider-edge invariant.
- spec: §1.15 and `33 §3.3`.
- given: in fresh Strict-root clients, pair each import-restored client with
  an otherwise identical client omitting only the listed selective import.
  Use `export name` as the source use in both clients; load dependency-closed
  roots for each listed home. The `ClockIO` home is also required by the clock
  action client below.

  | Name | Home |
  |---|---|
  | `monotonic_now` | `Capability.Time.Monotonic` |
  | `sleep_until` | `Capability.Time.Monotonic` |
  | `EntropyIO` | `Capability.System.Entropy` |
  | `random_bytes` | `Capability.System.Entropy` |
  | `host_entropy` | `Capability.Process.Exit` |

- measured: imported reference resolution and the accept/refuse result for
  each single-import pair.
- expect: every import-restored client accepts. Each omission refuses with
  `UnboundName` naming exactly its missing name. The home implementation
  global alone does not make the source name available.
- controls: the positive client differs from its omission only by the
  selected import; `seed-modules.md` covers `ClockIO`, and
  `seed-strict-package-homes.md` covers `host_clock`.
- why: §1.15 assigns operation names to explicit homes, not the ambient
  floor.

## Effect and authority boundary

### surface/ffi-io/clock-and-entropy-rows-do-not-grant-filesystem-authority

- promise class: durable type-and-authority invariant.
- spec: §1.15; `36 §1.4`.
- given: in separate Strict clients, import and use only
  `monotonic_now` through `host_clock`, or `random_bytes` through
  `host_entropy`. Measure the checked effect row of each client and run it
  with the corresponding host operation available but no filesystem
  authority. Pair each accepted client with the same body under a declared
  row containing only `FS`.
- measured: each checked effect row, the host result, and the paired
  source-refusal phase and effect name.
- expect: the clock client requires `Clock` and the entropy client requires
  `Entropy`; neither requires `FS`. Both execute without filesystem
  authority. Each `FS`-only declaration rejects at the effect-escape check
  because its body's `Clock` or `Entropy` effect is not admitted.
- controls: each correctly rowed client is the positive control for its
  `FS`-only refusal. No filesystem operation or file capability appears in
  either body.
- why: §1.15 gives each operation its own host-effect domain and explicitly
  grants neither effect filesystem authority.

## Clock behavior

### surface/ffi-io/monotonic-readings-are-nonnegative-and-nondecreasing

- promise class: durable host-behavior invariant.
- spec: §1.15; `Capability.Time.Monotonic`.
- given: in one fresh process, run a checked action lifted through
  `host_clock` that requests at least three `monotonic_now` readings in
  sequence. Use the same action and authority shape in interpreter and
  native lanes; observe each returned `MonotonicInstant` through its
  qualified `MonotonicInstant.MkMonotonicInstant` constructor.
- measured: each returned nanosecond `Int`, in request order, separately in
  each lane.
- expect: each reply selects the exact checked
  `MonotonicInstant.MkMonotonicInstant` identity. Every reading is
  nonnegative, and each reading is greater than or equal to its predecessor
  within that process. Do not compare absolute readings between lanes or
  separate processes.
- controls: three reads make a decreasing middle or final reading observable;
  the result constructor and integer field are observed, not inferred from
  the package name.
- why: §1.15 fixes process-local monotonicity and nonnegativity while leaving
  the epoch unspecified.

### surface/ffi-io/sleep-until-enforces-domain-and-same-clock-deadline

- promise class: durable host-behavior invariant.
- spec: §1.15.
- given: for each lane, use a single fresh process and a `host_clock` action
  that exercises these inputs separately:
  1. `Deadline.MkDeadline (MonotonicInstant.MkMonotonicInstant 0)`;
  2. a deadline derived in that process from a preceding monotonic reading,
     with an admitted nanosecond value strictly later than that reading;
  3. deadlines wrapping `-1` and `2^64`, in separate runs.
  After case 2 returns, request another monotonic reading in the same process.
- measured: operation result, terminal class, `ClockSleepUntil` event
  presence, and a host-runner observation of whether the Ken process blocked
  waiting for this deadline; also measure case 2's post-return reading. Treat
  the event as dispatch evidence, not proof of suspension.
- expect: the native lane reads the exact checked
  `Deadline.MkDeadline` and `MonotonicInstant.MkMonotonicInstant` identities
  from the source value. Case 1 returns `Unit` without a blocked wait; do not
  require the logical operation's trace event to be absent. Case 2 returns
  only after the same clock reads at or past its deadline; the subsequent
  reading is at least that deadline. For each out-of-domain input, both lanes
  refuse with the same terminal class and emit no `ClockSleepUntil` trace
  event. Do not pin a more
  specific refusal variant than §1.15 specifies.
- controls: case 1 is the already-reached control; case 2 uses a same-process
  reading to rule out comparing unrelated epochs. The two invalid inputs
  straddle the domain boundary.
- why: this separates successful sleep on the process clock, immediate return,
  and pre-sleep refusal. Sleep is uncancellable; no cancellation guarantee is
  tested.

## Entropy behavior

### surface/ffi-io/random-bytes-observes-count-domain-and-exact-length

- promise class: durable host-behavior invariant.
- spec: §1.15; `Capability.System.Entropy`.
- given: invoke `random_bytes` through `host_entropy` in a fresh process for
  counts `0`, `1`, and `1024`, and separately for `-1` and `1025`. Run the
  same inputs in the interpreter and native lanes. Observe `Result IOError
  Bytes` by constructor and, on success, measure the returned byte length.
- measured: result constructor, error case where present, and exact returned
  byte length; do not compare successful byte contents across runs.
- expect: counts `0`, `1`, and `1024` return `Ok` containing exactly the
  requested number of bytes. Counts `-1` and `1025` return
  `Err IOError.InvalidInput`; neither truncates the request nor supplies
  bytes. For the negative count, both lanes record the same event or neither
  records an event, as §1.15 specifies.
- controls: zero distinguishes a valid empty reply from an error; one
  distinguishes success from an always-empty stub; the maximum and first
  out-of-range count distinguish the inclusive bound from truncation.
- why: this pins the count domain and exact-length result without claiming
  that returned bytes are unpredictable.

### surface/ffi-io/random-bytes-reports-unavailable-kernel-source

- promise class: durable host-behavior invariant.
- spec: §1.15.
- given: run the same admitted positive-count `random_bytes` request in two
  otherwise identical Linux host profiles. The control profile exposes its
  normal kernel CSPRNG. The test profile makes all kernel entropy interfaces
  available to the Ken process fail or unavailable; the harness verifies
  that condition inside that process. Keep authority, source, input and
  filesystem contents fixed. This is a post-promotion host-profile case, not
  a Rust source-seam or mocked-response test.
- measured: the in-process source-availability condition, `Result` response,
  and returned bytes, if any.
- expect: the control returns `Ok` with exactly the requested length. The
  unavailable-source profile returns `Err IOError.Unsupported` with no
  bytes. No userspace fallback, substitute source, partial buffer or retry
  result satisfies the case.
- controls: the supported-profile twin proves the client and operation are
  reachable. Only kernel-source availability differs.
- why: the paired host profiles distinguish a real unavailable-source path
  from an unconditional `Unsupported` result while testing only the boundary
  §1.15 specifies.
- platform: a runner that cannot provide and verify both host profiles has
  not executed this case. No claim is made that the current development host
  lacks kernel entropy.
