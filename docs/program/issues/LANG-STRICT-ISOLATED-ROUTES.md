---
id: LANG-STRICT-ISOLATED-ROUTES
title: "After the catalog route is strict, ken check on an isolated .ken or .ken.md file, ken run and ken native-build still elaborate in Legacy, and 50 of the 62 isolated files that check today refuse under Strict: isolated files have no import loader, and the program vocabulary (host_program, host_exit, bind, FSOp, the State family and others) has no import home. Add strict file entries and an isolated-file import loader, home the program vocabulary under the 2026-09-25 rule, migrate the refusing files, and move the isolated routes to Strict"
status: ready
owner: language
size: L
tier: T1
gate: architect
depends_on: [LANG-STRICT-CATALOG-FLIP]
blocks: []
github: null
origin: "Operator 2026-09-25: one resolution mode; an ever-present intrinsic's name is reserved; no intrinsic module. Part 2 of the Steward recut evt_7tgtgt0vc3q3c of LANG-STRICT-CATALOG-FLIP, on Architect AC-0 evt_6069bcfays9wq. Split into three increments on Architect AC-0 evt_4fskbnm5tvmnb (Steward evt_6kyrbas7p0fww). Steward-filed per COORDINATION section 2."
---

# Isolated files and ken run resolve in strict mode

## Objective

`ken check` on an isolated file, `ken run` and `ken native-build`
elaborate in `ResolutionMode::Strict`. An isolated file can import
catalog modules, and every file that checks today still checks.

## Settled inputs (AC-0 evt_4fskbnm5tvmnb, on `83fdec077`)

- **The routes.** `ken check`'s isolated arm (`ken-cli/src/main.rs:524`,
  `:526`), `run_program_inner` (`ken-cli/src/lib.rs:191`, `:192`) and
  `native_build_file`'s preparation (`compiler_driver.rs:1302`, `:1304`).
  Test harnesses call the same entries, so none is changed in place.
- **The run API fan-in.** `run_program_inner` also serves the public run
  API: 37 ken-cli test files, `ken-verify/src/scenario.rs:701` in
  production, and ken-verify test sites. Switching it moves all of them.
- **The population.** Of 91 isolated files under `examples/`,
  `conformance/` and `crates/*/tests`, Legacy checks 62 and Strict 12.
  The 50 that refuse need 70 distinct names, and all 62 check once those
  names are bound. 16 of the 70 have a home today. No refusing file can
  migrate on those 16 alone. Inline sources, measured in 23 of about 81
  ken-cli test binaries, are a lower bound: 75 sources, 55 refusing,
  five more names.
- **No printing `main` under Strict.** 33 §3.2.1 requires `main :
  HostIO a ExitCode`, no catalog unit produces a HostIO value, and the
  lifts (`host_program` and its family) have no owner or spec promise.
- **The report defect.** `= Axiom`, `axiom name : T` and an open
  `ensures` hole each mint a postulate that its root's result does not
  record, so `--resolution-report` fails with "unattributed checked
  identity". The three catalog Axiom users fail this way.
- **The home rule** (operator 2026-09-25, Steward evt_7tgtgt0vc3q3c).
  There is no new intrinsic or host module. Each name either joins an
  existing catalog unit that already owns its concept, or is a floor
  addition with 30 §4's two witnesses.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

Three increments, each its own candidate, in order.

1. **Increment 1, entries and loader** (Language, M). The AC-0's R1
   root discovery, R2 strict isolated entries with the loader, and R4
   postulate ownership. Spec co-turn: 33 §3.2 (the isolated root) and
   70 §1.4 (postulate ownership). No route switch and no migration.
2. **Increment 2, homes.** It needs Spec promise rulings first, for the
   AC-0's groups C to G. The group G fork is ruled (i): the
   program-entry surface is promised and homed in an existing owner,
   with Capability.Process.Exit as the recommended owner. Then come the
   Architect's keying D0 per family, the catalog exports, and the 36, 38
   and ledger rows. A group D family with no owner and no §4 witnesses
   returns to the operator.
3. **Increment 3, route switches.** check, run (with roots threaded
   through the API) and native-build through a strict preparation entry.
   It also migrates the files and inline sources. It opens with its own
   AC-0 on the full inline population, ken-verify included.

The Spec co-turn owns `spec/`, and CV votes on `spec/` and
`conformance/`.

## Acceptance

Increment 1:

- **AC-1a (loader).** Four rows: no root, a root, a missing unit, and an
  inline-module import.
- **AC-1b (entry point).** An imported unit's `main` lands qualified, an
  explicit `import Lib (main)` beside an own `main` is ambiguous, and no
  imported entry point arises without an own `main`.
- **AC-1c (count).** Through the strict entry, exactly 12 of the 62
  Legacy-clean files are accepted, and the other 50 refuse with
  UnboundName.
- **AC-1d (report).** On check, run and native-build, a root-owned
  `= Axiom`, `axiom` and `ensures` hole each record as
  `source-declared` under 70 §1.4, beside `= Proved` and `prove`
  controls that are byte-unchanged. The three catalog Axiom entries
  report on `check` with exit 0. The fail-closed arm stays.
- **AC-1e (mutation, QA).** Dropping the postulate ownership loop
  reddens the Axiom row while the Proved row stays green.

Increment 3:

- **AC-3a.** Every isolated file that checks under Legacy on the cut base
  checks through the strict route, and a test asserts the count.
- **AC-3b.** Through the CLI binary, on `check`, `run` and
  `native-build`, a non-floor name without its import is refused with
  the strict unbound-name error, and accepted with the import.
- **AC-3c (mutation, QA).** Pointing the check route back at
  `elaborate_file_v1` makes AC-3b's `check` refusal row accept.
- **AC-3d (carried from LANG-CLI-RESOLUTION-REPORT AC-2).** The six
  standalone shadow invocations in
  `conformance/surface/modules/seed-modules.md` become covered: the
  shadow `fn write` imports its provider on an isolated file, and
  `--resolution-report` records `write` as `source-declared` on all
  three routes.

Increment 2's acceptance is framed once the Spec promise rulings land.

## Stop conditions

- A name with no existing catalog owner, where its floor addition lacks
  §4's witnesses.
- A kernel or trust change.

Checks: 4 fired: fork (ii) cannot be written as a printing `main`, since
no catalog unit yields HostIO. 6 fired: the population is the AC-0's 91
files at the strict seam, with the inline count named as a lower bound
for Increment 3's AC-0. 8 fired: AC-1e's mutant removes the ownership
the Axiom row reads, while the Proved row reads only def_id.
