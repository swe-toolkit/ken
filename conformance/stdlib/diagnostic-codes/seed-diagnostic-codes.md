# Stable diagnostic codes: conformance seed

Format: `../../README.md`. These cases pin the tooling-facing contract in
`spec/50-stdlib/70-diagnostic-codes.md §1`; they do not specify a Ken package,
new CLI option, JSON shape, or implementation mechanism.

The code-bearing stderr line is the oracle, not message text or exit status
alone. Match a whole stderr line beginning `error[<code>]: `, wherever it
appears. The message and the line's position are not pinned. A program's own
stderr is not an authenticated tool diagnostic.

These cases are authored against the normative candidate, not executed. At
base `74dd59d20`, the ledger has E, K, and X rows but no C, H, or N rows; no
C or H number is guessed here. Cases needing new assignments become executable
when their owning ledger rows and rendering are delivered. N0001–N0007 are the
exact family identities already fixed by §1.2.

## Batch CLI observations

### stdlib/diagnostic-codes/coded-line-has-one-code-slot
- promise class: **normative compatibility vector**
- spec: `70 §1.1–§1.2`
- given: one controlled batch CLI failure with an assigned code.
- expect: stderr contains a whole line matching
  `^error\[([CEHKNX][0-9]{4})\]: .*$`. That line has one leading code
  slot and a free-text message. Other stderr lines may precede or follow it.
- control: a malformed code or a line without the complete
  `error[<code>]: ` prefix is not a coded diagnostic line. Code-like text
  inside the free-text message is not a second code slot.
- why: code grammar and line shape are stable; message text and line position
  are not.

### stdlib/diagnostic-codes/status-one-separates-program-and-cli-failure
- promise class: **durable invariant**
- spec: `70 §1.1`, `§1.3`
- given: (a) an admitted `ken run` program whose `main` returns `Failure 1`
  and writes nothing to stderr, and (b) `ken frobnicate`.
- expect: both process statuses are 1. The program outcome has no tool
  diagnostic; the unknown-subcommand failure has one C-coded diagnostic.
  Resolve that C code from the ledger row assigned to this cause; do not guess
  a numeric C code before assignment.
- control: using status 1 alone as the predicate classifies both alike and
  fails to distinguish a coded tool failure from the admitted program's own
  `Failure 1` outcome.
- why: status is not a code. The distinguishing observation is the tool's
  coded line, with the no-stderr program fixture keeping program-authored
  lookalike text out of this pair.

### stdlib/diagnostic-codes/run-program-outcomes-are-not-tool-codes
- promise class: **normative compatibility vector**
- spec: `70 §1.3`
- given: admitted `ken run` programs returning `Success`, `Failure 0`,
  `Failure 1`, and `Failure 255`; each writes nothing to stderr.
- expect: statuses are respectively 0, 1, 1, and 255. None of these program
  outcomes gets a tool diagnostic code.
- control: `Failure 300` is not a `UInt8` program outcome. It is refused by
  source checking before execution and returns the code belonging to that
  elaboration failure, with coded-failure status 1.
- why: this boundary prevents a program's exit status from being read as a
  CLI error identity.

### stdlib/diagnostic-codes/cli-option-and-working-directory-failures-use-c
- promise class: **durable invariant**
- spec: `70 §1.1–§1.2`
- given: (a) `ken fmt --unknown`, and (b) a `ken run` invocation with an
  absolute, readable source path where a controlled OS/test fixture makes
  the CLI's current-directory read fail.
- expect: each front-end failure exits 1 with a C code selected from the
  active `cli`-owned ledger row for that cause.
- control: with the same `ken run` source and a readable working directory,
  root-execution denial is host-carried and uses H, not C.
- why: option parsing and working-directory reads belong to the CLI carrier;
  a `ken run` subcommand does not turn them into X or H errors.

### stdlib/diagnostic-codes/run-missing-main-retains-x0005
- promise class: **normative compatibility vector**
- spec: `70 §1.1–§1.3`
- given: `ken run` on a checked source file with no admitted `main`.
- expect: status 1 and a coded diagnostic whose code is `X0005`, the ledger
  identity `run.missing_entrypoint`.
- control: pass the same file to `ken native-build` with a writable output
  directory and a valid resource profile. It reaches native-build admission,
  not the `ken run` carrier; it uses the E-domain ledger code for
  `native_build.admission`, not `X0005`.
- why: the command name does not select the domain; the failure carrier does.

### stdlib/diagnostic-codes/kernel-rejection-retains-k-domain
- promise class: **durable invariant**
- spec: `70 §1.1–§1.2`; `spec/10-kernel/14 §3`
- given: `ken check` on `data Negative = MkNegative (Negative -> Bool)`, whose
  negative recursive occurrence reaches `KernelError::PositivityViolation`.
- expect: status 1 and the K-domain code assigned by the ledger to
  `kernel.positivity_violation`, projected from the kernel carrier.
- control: `data Positive = MkPositive (Bool -> Positive)` is accepted and
  emits no error code.
- why: a kernel error wrapped by the elaborator retains the K carrier; it is
  not reclassified as E because of the wrapper.

### stdlib/diagnostic-codes/open-obligation-report-is-not-a-coded-failure
- promise class: **durable invariant**
- spec: `70 §1.1`, `§1.3`
- given: `ken check` on a source file that reports an open verification
  obligation and has no other failure.
- expect: status 0; the report does not begin with `error[` and carries no
  code under this section.
- control: a controlled error from the same command has a code-bearing line
  and status 1.
- why: reporting an open obligation is not converted into a failed CLI
  invocation.

## Carrier-selected domains

### stdlib/diagnostic-codes/fmt-path-causes-use-cli-domain
- promise class: **durable invariant**
- spec: `70 §1.1`, `§1.2`
- given: separate formatter invocations for (a) a missing `.ken` path,
  (b) an existing unsupported-suffix path, and (c) a readable, formattable
  path whose write is denied after formatting changes its contents.
- expect: each is a C-coded front-end path failure. Each code is the active
  `cli`-owned ledger identity for its cause; the seed does not invent a C
  number.
- control: the same changed source at a writable path is formatted normally.
  A valid `.ken` path whose formatting operation returns an
  `ElabError::ParseError` uses the ledger code for `elab.parse_error`, not C.
- why: the front-end owns path decisions only when no other carrier covers
  the failure; a formatter error retains its elaboration carrier.

### stdlib/diagnostic-codes/fmt-codes-each-reported-path-failure
- promise class: **durable invariant**
- spec: `70 §1.1`, `§1.4`
- given: one `ken fmt --check` invocation with a missing `.ken` path and an
  existing unsupported-suffix path. The formatter continues to the second
  path and reports both failures.
- expect: each reported failure has its own whole stderr diagnostic line and
  its own C code, selected by that path failure's cause. Either line may come
  first; do not pin messages or a maximum number of other diagnostics.
- control: a single code-bearing line for the whole invocation, or one
  un-coded path-error line, fails the per-diagnostic assertion.
- why: multiple diagnostics are permitted; each has one code. The two
  distinct path causes prevent one copied code from standing in for both.

### stdlib/diagnostic-codes/host-run-failures-remain-host-coded
- promise class: **durable invariant**
- spec: `70 §1.1`, `§1.3`
- given: each independently controlled pre-program host failure: root
  execution denial, effective-user home-root resolution failure, and a
  malformed host exit-code path if reached.
- expect: each failure is H-coded and exits 1. Resolve its code from the
  corresponding host-owned ledger row; do not infer X from the `ken run`
  subcommand or accept an uncoded status-only refusal.
- control: the admitted program's own `Failure 1` is not host-coded. The
  positive control distinguishes a pre-program host refusal from an ordinary
  program result.
- why: these failures belong to the host carrier even where the current
  implementation uses a status value or has a silent path. The malformed
  exit case is exercised through a controlled host-boundary fixture; checked
  source cannot construct an out-of-range `Failure` value.

### stdlib/diagnostic-codes/native-build-error-keeps-its-carrier
- promise class: **durable invariant**
- spec: `70 §1.1–§1.2`
- given: one native-build admission error and one native-backend refusal.
- expect: the admission error retains its E carrier; a backend refusal uses
  the N code for its underlying cause family. Neither is reclassified as C
  merely because it is returned by `ken native-build`.
- control: a missing command-line path or argument is a front-end C failure,
  not an E or N failure.
- why: projection follows the error carrier rather than the printing phase
  or subcommand.

## Native cause-family identities

### stdlib/diagnostic-codes/native-cause-families-have-fixed-meanings
- promise class: **normative compatibility vector**
- spec: `70 §1.2`
- given: one independently identified native refusal for each listed cause
  family. The witness must establish the cause independently of the code
  renderer; a renderer-selected label is not its own oracle.
- expect: each refusal uses the code in its corresponding row of the
  `70 §1.2` native-family table, and the ledger assigns that code the same
  stable meaning. The seed does not duplicate those code assignments.
- control: for each independently witnessed family, substituting another
  family's code fails that row. A family label read back from the same
  renderer does not count as an independent witness.
- why: each N number has a stable meaning; message wording may change.

## Registry and identity

### stdlib/diagnostic-codes/registry-accepts-only-the-six-domain-shape
- promise class: **durable invariant**
- spec: `70 §1.2`
- given: a registry-validation fixture with one valid synthetic row in each
  allowed domain (`C`, `E`, `H`, `K`, `N`, `X`), plus independent malformed
  row fixtures.
- expect: valid codes have exactly five ASCII characters: one allowed
  uppercase domain letter and four ASCII digits. `0000` is not assigned;
  per-domain assignments begin at `0001`.
- refuse each malformed shape independently: `E001`, `e0001`, `Q0001`,
  `E00001`, and a code containing a non-ASCII digit. Separately refuse the
  well-shaped but unassigned `E0000`. The valid fixture is the positive
  control; one malformed row must not mask another.
- why: grammar and allowed domains are exact. The fixture rows are synthetic
  validator inputs, not proposed production assignments.

### stdlib/diagnostic-codes/ledger-is-dense-unique-and-authoritative
- promise class: **durable invariant**
- spec: `70 §1.2`
- given: registry fixtures with unique, dense rows; then independent
  mutations for a duplicate code, duplicate label, per-domain gap, malformed
  row, unregistered table code, and a table entry for a retired row.
- expect: the valid fixture is accepted; each mutation is refused for its
  corresponding registry or owner-table inconsistency. New identities come
  from the single ledger, not a copied spec table.
- control: interleaving another domain's first row does not create a false
  gap in the first domain; a valid next code in each domain remains accepted.
- why: this checks the active registry/owner relation, not a repository-text
  census. Each bad fixture changes one property so its detector is visible.

### stdlib/diagnostic-codes/ledger-label-owners-match-domains
- promise class: **durable invariant**
- spec: `70 §1.2`
- given: synthetic valid ledger rows for each domain, plus rows whose label
  owner belongs to a different domain.
- expect: `C` is owned by `cli`; `E` by `elab`, `compiler`, or
  `native_build`; `K` by `kernel`; `X` by `run` or `run_io`; `N` by
  `native`; and `H` by `host`. Reject each mismatched domain/owner pair.
- control: each listed owner/domain pairing is accepted by its owner table;
  the fixture labels are not production code assignments.
- why: the domain is selected from the failure carrier and its owner, not
  inferred from a code-shaped string.

### stdlib/diagnostic-codes/retirement-preserves-identity-across-history
- promise class: **durable invariant**
- spec: `70 §1.2`
- given: a prior ledger snapshot and its proposed successor. The positive
  successor keeps every existing code and label, may change an active row to
  retired, and appends new rows. Negative successors separately remove a
  retired row, reuse its code, or change a prior code's label.
- expect: the positive transition preserves identity and order; each
  identity-changing or reuse transition is refused.
- control: changing only `active` to `retired` is allowed; changing the code
  or meaning is not.
- why: a single-snapshot uniqueness check cannot establish non-reuse or
  non-remeaning over time. The comparison consumes two registry values; it
  does not claim that a current snapshot proves history.

### stdlib/diagnostic-codes/future-machine-view-reuses-code-identity
- promise class: **normative compatibility vector**
- spec: `70 §1.4`
- given: when a future machine representation is delivered, one failure that
  has both its human stderr line and its machine representation.
- expect: the machine representation carries the same code string as the
  human line and ledger identity; it introduces no second code space.
- control: a different machine-only identifier fails the comparison.
- why: the machine channel is deferred, but its code identity is not open.
  This case fixes no JSON fields, CLI flag, or serialization shape.

## Boundaries not frozen here

- No case pins the position of a coded line, the wording of its message, the
  order or maximum number of elaboration diagnostics, or `ken fmt` path-error
  ordering.
- No JSON field, new CLI flag, or warning format is asserted. The future
  machine-view case fixes only the shared code string.
- Program-authored stderr may contain text matching `error[<code>]: `; the
  prefix alone does not authenticate a tool diagnostic. The status-one pair
  uses a program that writes no stderr and does not claim authentication.
- Implementation panics, separately executed native artifacts, and
  continuing-REPL diagnostics are outside this batch-failure contract.
- No Ken declaration, catalog package, kernel rule, primitive, or
  `trusted_base()` change is implied.

## Coverage map

- Presence, carrier ownership, and one-code line shape: the status-one pair,
  the CLI-option case, the `fmt` carrier cases, the missing-main case, and the
  host/native carrier cases.
- Grammar, allowed domains, density, ledger ownership, and retirement:
  the four registry fixtures.
- Native family meanings: the §1.2 N-code compatibility vector.
- Exit status, open obligations, and the `ken run` carve-out: the program
  outcome, missing-main, and open-obligation cases.
- Machine-code identity: the future-machine-view case; deferred formatting
  shape and other exclusions remain non-claims.
