# Stable diagnostic codes of the `ken` command

> **Status: DRAFT v0 (SPEC-DIAGNOSTIC-CODE-CONTRACT).** Section 1 is
> normative for the controlled, batch-mode failures returned by the `ken`
> command. It pins code identity, ownership, presentation, and process
> status; it does not assert that every producer or seed has landed. This
> tooling-facing contract is indexed with the standard packages, but is
> **not itself an ordinary Ken package** or an extension of CAT-5's
> source-located diagnostic type (`59 §6`).

## 1. Controlled CLI failures and their codes

### 1.1 Presence and ownership

Every controlled failure returned by a batch invocation of `ken` carries
**exactly one stable code for each diagnostic**. The command emits at
least one coded diagnostic when it returns such a failure. A diagnostic
is one whole stderr line beginning `error[<code>]: `, followed by a
free-text message. It has **one code slot**, not a sequence of codes;
continuation lines, such as `Usage:`, may follow. Neither the position
of the coded line among other stderr output nor the message text is
stable. One invocation may emit multiple diagnostic lines, **each**
with its own single code: `ken fmt` can report separate failed paths,
continue to the next path, and then exit. This contract does not
promise a single failure or prescribe the ordering or maximum number
of several elaboration diagnostics.

Choose the domain from the **failure's carrier**, never from the
subcommand or the phase in which the error is printed. When a
front-end wrapper receives an error already carried by another domain,
it presents that carrier's code rather than inventing a CLI code. In
particular, a `ken run` admission or execution error can be `E`, `X`,
`K`, or `H`, as its carrier dictates; a `native-build` error can be
`E` or `N`. A `ken fmt` formatting error returned as `ElabError` is
`E`, not `C`. Projecting a kernel or host error into a code in its
wrapping crate does not require the kernel or host to import the
code ledger.

The `C` domain means **a failure of the `ken` command's own front end
(argument and option parsing, subcommand dispatch, and reading or
writing the files and paths named on its command line, or reading the
working directory) whose cause no carrier in another domain covers**.
It includes front-end format-path decisions such as an unsupported
input path or a non-canonical `fmt --check` result. A carrier's
existence, not the string of the subcommand or its rendered message,
is the discriminator. For example, an unreadable `fmt` path is a CLI
failure, while a format operation returning `ElabError` retains its
elaboration code. `C` is not a fallback for every error that reaches
`main.rs`.

This presence rule excludes **program outcomes** of an admitted
program under `ken run` (§1.3), implementation panics (including
status 101), separately executed native artifacts produced by
`native-build`, and diagnostics printed by a continuing REPL process.
Those exclusions do not turn a pre-program host refusal into a
program outcome: root-execution denial or home-root resolution failure
is host-carried and requires a coded `H` line even if an existing
implementation represents it internally by an exit-status value.
The malformed host exit-code path also requires its `H` code if
reached; checked source cannot construct an out-of-range `Failure`
argument, but unreachable-from-checked-source is not permission for
an uncoded live fallback.

### 1.2 Code grammar, domains, and stability

A code is **exactly five ASCII characters** matching
`^[CEHKNX][0-9]{4}$`: one uppercase domain letter followed by four
ASCII digits. For example, `E0001` has the shape; `E001`, `e0001`,
`Q0001`, and `E00001` do not. Valid shape alone does not assign a code:
only a row in `crates/ken-diagnostic/codes.tsv` does. Assigned numbers
in **each domain** are dense from `0001` in ledger order; `0000` is
never assigned. Retired rows still occupy their numbers, so they do
not create reusable holes. New assignments append to the one ledger,
not to a copied spec table.

| Letter | Failure domain | Ledger label owners |
|---|---|---|
| `C` | CLI front end, only if no other domain carrier covers it | `cli` |
| `E` | Elaboration | `elab`, `compiler`, `native_build` |
| `K` | Kernel, coded by projection | `kernel` |
| `X` | Execution under `ken run` | `run`, `run_io` |
| `N` | Native lowering and backend | `native` |
| `H` | Host, coded by projection | `host` |

The ledger is **incorporated by reference** as the sole registry of
assigned codes, their labels, and active or retired status. A code
never gets a second meaning or a new owner; a retired code remains
reserved. Its meaning label is stable, while its accompanying human
message is free text and may change. Adding a cause assigns the next
code in its carrier's domain rather than renumbering or repurposing an
existing row. Retirement may change an active row's status, not its
code or meaning. A check of one ledger snapshot cannot establish that
an older code was never repurposed; that needs historical comparison.
Neither this chapter nor a conformance seed duplicates ledger rows
to form a competing source of identities. A closed case may cite the
relevant ledger **label** and check the code it owns at the
implementation under test; a prospective `C` case does not guess a
number before the `C` rows are assigned.

The native domain's first seven cause families have these stable
meanings. The family follows the underlying cause, not the wording of
a rendering or the command that happened to discover it:

| Code | Native cause family |
|---|---|
| `N0001` | Unsupported program fragment |
| `N0002` | Capacity |
| `N0003` | Planner invariant (a compiler defect) |
| `N0004` | Finished-code check |
| `N0005` | Admission and trust |
| `N0006` | Backend |
| `N0007` | Resource enforcement |

### 1.3 Exit status, non-failures, and the `ken run` carve-out

Every **coded failure** returned by the `ken` process exits with status
**1**, after emitting at least one coded line. Exit status by itself
is **not** a code and cannot distinguish a tool failure from a
program's own failure. In particular, an unavailable entrypoint or
console ABI under `ken run` is a coded `X` failure and exits 1, not
2. An invocation that only reports open verification obligations is
not made into a coded failure: `ken check` may print those reports and
exit 0. More generally, a non-failure report never begins with
`error[` and has no code under this section. A future warning format
is not fixed here.

The sole status carve-out applies **after admission, when `ken run`
is executing a program**. An ordinary program `Success` exits 0;
`Failure n` exits `n` for `1 ≤ n ≤ 255`, while `Failure 0` exits 1.
That is the program's outcome, not a coded CLI failure: the tool emits
no diagnostic code **for that outcome**. The source argument of
`Failure` is `UInt8`, so `Failure 300` is refused during source
checking rather than generating a new program-outcome status. The
program can itself write to stderr; its bytes are not a tool
diagnostic. A controlled admission, runtime, or host failure before
or during execution remains coded and exits 1 under the usual rule.
The code-bearing line, rather than status 1, is the tool's
human-facing discriminator for a coded failure. Under `ken run` even
that line is **not authenticated**: a running program may print
`error[<code>]: ` itself. No consumer may treat arbitrary program
stderr as a trustworthy tool diagnosis solely by matching the
prefix.

### 1.4 Machine channels and the resolution report

For controlled batch CLI **failures**, the human stderr line remains the
sole specified code carrier in this tranche. There is no JSON failure
format, `--format` flag or secondary code identifier. A future machine
failure format must carry the **same code string**, never a second code
space, and reconcile with the distinct `ken.verify/v1` verdict documents
(`../20-verification/25 §2–§3`). The opt-in elaboration-identity report
below is **neither** that failure format nor a verdict document. It
contains no diagnostic code or verification verdict, and does not change
the stderr code requirement on any controlled failure.

The three routes `ken check`, `ken run` and `ken native-build` accept one
leading token `--resolution-report=<path>` immediately after the
subcommand. The CLI removes only that first token; each existing
positional parser receives exactly the arguments it otherwise would.
A bare `--resolution-report` without `=` or a token in another position
is not this option and retains the route's existing positional behavior.
Only one leading token is recognized; a repeated token remains a
positional argument. The accepted positions are:

```text
ken check --resolution-report=<path> <file>
ken run --resolution-report=<path> <file> [-- <args>...]
ken native-build --resolution-report=<path> <file> <output-dir> <resource-profile.json>
```

Without the option, each route's stdout, stderr,
exit status and file effects remain unchanged. With a successfully
written report, stdout, stderr and exit status remain byte-identical to
the same invocation without the option; report-file creation or write
failures are the explicit exceptions. The report is written **only** to
the named file, never to stdout or stderr: `ken run` gives those streams
to the program, `ken check` uses stderr for obligations, and
`native-build` uses stdout for its artifact path.

The CLI exclusively creates the file before reading the source. An
empty, existing or unwritable path refuses creation without changing
any pre-existing file; the controlled CLI failure uses ledger label
`cli.resolution_report_create` in the `C` domain (§1.2). A failed
report write or flush uses `cli.resolution_report_write`, also a `C`
cause. An elaboration or classifier failure retains its own carrier's
code, not one of these creation/write labels. On every exit the CLI
process controls, it unlinks a file it created unless the **entire**
report was written and flushed. Thus on a controlled exit a newly
created report path exists iff a complete report was written; a
complete report persists even if execution or native completion later
fails. An abnormal worker termination, including a signal or native
stack exhaustion during elaboration, can instead leave an empty or
partial file. Every complete report is nonempty and newline-terminated;
readers must not treat mere file existence as completeness. The parent
process does not unlink the path: it cannot tell a worker-created file
from a pre-existing one. Under `ken run`, a program with filesystem
authority can rewrite a completed report; its later contents are not
authenticated as tool output (§1.3).

Each report comes from the **same elaboration environment** that its
route subsequently checks, executes or lowers, never from a second
resolution pass. `check` writes after successful root elaboration and
before displaying open obligations. For a catalog entry, its checked
document fences have also been elaborated, but `declarations` lists
only the entry unit's own root declarations, not those fences or its
import closure. `run` writes after the checked-main admission and
root-execution admission succeed, immediately before driving the host
tree. `native-build` computes the report from its preparation's
elaboration environment, then writes it after successful source
preparation and before native completion or object emission. These
are route-local **successful-elaboration** boundaries, not promises of
subsequent execution or native-build success; a later failure does not
retract a complete report.

The file is one compact UTF-8 JSON document on one line, followed by
one newline. Its ordered top-level keys are `schema`, `route`, `module`
and `declarations`. `schema` is `ken.resolution/v1`; `route` is
`check`, `run` or `native-build`; `module` is the catalog entry's
dotted module path for a catalog `check`, otherwise `null`.
`declarations` is an array in the root route's declaration-result
order. Each item has ordered keys `declaration` (the root's source
name without an internal native-driver package prefix) and
`references`. The latter is a deduplicated array sorted first by
class rank `pre-source`, `imported`, `source-declared`, then bytewise
by the name fields. Serialization is deterministic and byte-stable
for the same checked root and route; for a standalone root admitted
by all three routes, its documents differ only in `route`.

For each root result, the reported references are the checked
identities found recursively in the kernel-checked types and bodies
of that declaration and its auxiliaries. The collector is a pure read:
it does not elaborate, re-resolve names, mutate globals or change a
checked identity. Constant, inductive-former, constructor and
elimination-family references all count. A reference
inserted by elaboration counts just like one originating in source;
this is a report of **referenced identities**, not a record of written
free-name spellings or their provenance. Source-minted literal
identities are omitted. Classification uses the checked identity,
not a mutable lookup by spelling, in this order:

- A pre-source identity records `{"class":"pre-source","name":…}`,
  with its canonical source-visible name, or `null` if the identity
  has no captured name. A report-only display name for a renamed
  pre-source family (such as `ConsoleStream.Stdin`) does not alter
  its checked identity or native stable symbol.
- A root-owned checked declaration records
  `{"class":"source-declared","declaration":…}`. A constructor
  of a root-owned family also has a `constructor` field; its
  `declaration` is the family name. A checked **opaque postulate**
  admitted while elaborating a root result belongs to that result by
  checked identity, even when it is neither the result's own identity
  nor one of its auxiliaries. References to such identities record
  `source-declared` with that root's `declaration` name. This covers
  an `= Axiom` body, an `axiom` declaration and an open `ensures`
  obligation hole without guessing from an owner-label spelling. A
  side-effect postulate adds no extra root row; an explicit `axiom`
  declaration that is itself a root result retains its own row. This
  adds no general opaque fallback: non-literal identities without a
  captured pre-source, root or imported owner still fail closed.
- An identity owned by another loaded unit records
  `{"class":"imported","module":…,"declaration":…}`. That
  declaration field names the imported declaration in its module;
  this does not list the imported unit among root declarations.

If a referenced identity fits none of those classes and is not a
source-minted literal, generation fails closed with an elaboration
`Internal` error and its existing `E`-domain code. The incomplete
file is removed on controlled failure; no class is guessed, and no
raw `GlobalId` number occurs in a complete report. The record's versioned,
deterministic and total-on-success shape follows the machine-document
conventions of `25 §2`, but it is not `ken.verify/v1`; a breaking
schema change requires a new version, while readers ignore unknown
additive optional fields.

Architect AC-0 measured the accepted and source-shadowed roots on all
three CLI routes but did not walk the native driver's report environment
or the literate `.ken.md` arm. Those are distinct obligations, not
facts established by AC-0. The implementation child reports a
candidate-local named target passing 6/6, including native-report
parity and literate coverage, with two classifier mutations red and
restored; those reports do not establish independent QA, official
conformance execution or CI.

General elaborator error recovery and the ordering and cap on
multiple elaboration diagnostics are deferred. Warning severity and
its formatting are deferred, subject to §1.3's rule that a
non-failure report never starts with `error[`. The ordering of
several `ken fmt` path errors is not frozen. Machine-authenticated
separation between tool diagnostics and arbitrary `ken run` program
stderr is also deferred; the prefix alone cannot supply it.

### 1.5 Derivation and evidence boundary

These are contracts for the untrusted CLI, diagnostic-ledger,
elaborator, and runtime layers and for their kernel- and host-error
projections as delivered. They specify no Ken `data` declaration,
catalog module, new primitive, kernel rule, or check-rule change. The
single append-only ledger is the source of assigned identities; this
section supplies the behavioral rule that clients and black-box tests
may rely on **once its rendering tranche is delivered**. The D0
measured a corrected increment-3 scope of **21 C sites and 3 H sites**
on base `74dd59d20`, not the earlier provisional 19 CLI exits. The
prior count included an `E`-carried native-build error and the
aggregate `fmt` exit (not itself a diagnostic); `fmt` formatter
errors carried by `ElabError` also remain `E`. Three host sites in
the `ken run` library path were missed by the provisional `main.rs`
sweep. This population is design evidence for implementation scope,
**not** a frozen list of error cases or a claim the ledger already
contains C, H, or N rows.

The D0 observed program outcomes of `Failure 1` and `Failure 2`, and
read the three host sites without executing root-denial or home-root
failure. This author slice edits only `/spec`: it has not executed
its conformance seed, landed new code assignments, tested CLI
rendering, run CI, or measured a `trusted_base()` delta. The
conformance validator supplies the closed seed separately. Its CLI
cases must distinguish a coded failure at status 1 from a silent
program outcome also at status 1 and exercise multiple independently
coded `fmt` path errors. Code-shape and reused-ledger-row refusals
require a ledger-validation fixture or controlled mutation; they are
not evidence that an ordinary CLI invocation can edit the registry.
