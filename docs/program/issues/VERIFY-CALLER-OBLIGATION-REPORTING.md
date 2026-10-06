---
id: VERIFY-CALLER-OBLIGATION-REPORTING
title: "ken check, ken run, ken native-build, the compiler driver and the REPL elaborate through ID-only wrappers that drop ElabResult.obligations, so an open obligation gets no report and the package obligation map stays empty. Make every user-facing caller report each open obligation and still succeed"
status: merged
owner: verify
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Architect evt_2sb1ehveqk5m0 (from the REFINEMENT review carry evt_53r1rc38nxqgb): the same objective as VERIFY-REUSED-ENV-TRUST-RESIDUE, trust nobody reports, but a different mechanism and seam, so a separate WP on the verify ring. Placed ahead of LANG-ELAB-RECURSIVE-FRAME-BUDGET while REUSED-ENV waits for the operator. Steward-filed per COORDINATION section 2. Measured at origin/main 3ad9a5590."
---

# User-facing callers report open obligations

## Objective

When `ken check`, `ken run`, `ken native-build`, the compiler driver or the
REPL accepts a declaration whose `ElabResult.obligations` is non-empty, the
user sees each open obligation as `unknown`, and the package records it.
The command still succeeds (spec 21 §5.1-5.2, 24 §2): silence is the
defect, not success.

## AC-0 result (verify `evt_1kb41zsgdvjep`, ruled `evt_6jvxr0czd8cfr`)

On one open `requires` (`ac0_use`), at `3c9a283e2`:
- `ken check` exits 0 with no output;
- `ken run` exits 0 and prints only the program's output;
- both driver entries return `Ok` with an empty `report.obligations`, while
  the hole sits in `report.assumptions` and `trusted_base_delta`;
- the REPL prints `defined:` and nothing else.

No catalog, example, conformance or test consumer relies on the silence.
`v2_acceptance::open_call_requires_hole_exports_unknown` needs only `Ok`
from the denotation route, which flagged success keeps.

## Deliverable: the ruled repair (size M, T1)

- **One shared renderer** in `ken-elaborator`, `render_open_obligations`:
  one line per open obligation, `unknown <id> at <span>: <goal>`. The CLI and
  the REPL both call it. Wherever stdout carries a product, report to stderr.
- **`ken check`** (`main.rs:383-395`): print the report, then `ken check: N
  open obligation(s), status unknown`. Exit 0.
- **`ken run`**: print the report before execution. The exit status stays the
  program's own.
- **`ken native-build`** (`main.rs:218`, through
  `prepare_native_program_sources`): report to stderr; stdout stays the
  executable path. Exit 0. Measure its row first as its AC-0 baseline.
- **REPL `do_def`** (`repl.rs:137`): use `elaborate_decl_v1`, as `do_check`
  does (`repl.rs:158-163`), and print the report after `defined:`.
- **Compiler driver.** Thread the V2 triples into `emit_package_from_env`
  (`compiler_driver.rs:3279`). Per triple, fill `semantic.obligations` with
  the canonically encoded goal and `obligation_metadata` with status
  `Unknown`, origin the owning declaration's stable symbol, and
  `affects_runtime_meaning: true`. All four driver routes (denotation,
  package, native, the denotation's carried package) inherit it. The trust
  delta entry stays.
- **APIs.** Add `elaborate_ken_md_file_v1` (including `ken example` fence
  results; `ken reject` ranges are excluded) and
  `elaborate_module_from_roots_v1` (every declaration it elaborates,
  dependencies included). Each ID-only wrapper becomes a projection of its
  `_v1` form.

## Acceptance

- **AC-1.** Rows for check, run, REPL, native-build and the four driver
  routes, on the AC-0 file.
  - Each asserts its exit status first, then exactly one report line starting
    `unknown ac0_use.requires.`.
  - Driver rows assert exactly one `report.obligations` key with that prefix,
    with status `Unknown` and origin `ac0_use`'s symbol.
  - Assert the prefix and the count, never the numeric suffix.
- **AC-2 (controls).** A discharged-premise twin and a no-`requires` file
  print no report line, keep byte-identical output, and keep
  `core_semantic_hash` unchanged on the driver routes. Catalog and examples
  stay green. The denotation consumers rerun: `v2_acceptance`,
  `b1_acceptance`, `b1_exact_denotation_alphabet`, `b2_acceptance`,
  `px7f_resource_lifetime_export`, `px8p_checked_buffer_producer`,
  `px8x_static_export_projection`, and `ken-interp` `b3_acceptance`. A hash
  change is expected only where an open obligation exists (spec 46 §3.1).
- **AC-3 (falsifier).** Pointing any one caller back at its ID-only wrapper
  reddens its row and leaves the controls green.

## Stop conditions

- Any kernel, `trusted_base()` or spec change.
- A consumer relies on a clean exit or an empty obligation map with open
  obligations: stop to the Architect with the consumer (CHECKS 3).
- **Not this WP:** obligation identities that depend on allocation order
  (`VERIFY-OBLIGATION-STABLE-IDENTITY`), and a strict mode that fails on
  open obligations.

## Closeout

Merged `fd1bafb0e` from exact `51cad091c` (PR #4520). Verify QA
`evt_55h84zcg0165f`, Architect `evt_5ajsvmkq83fnz`, Decision
`dec_mgc3t6c655hv`. Supersedes `9919c8808` and `f601c761c`.

- `ken check`, `ken run`, `ken native-build`, the compiler driver and the
  REPL report each open obligation as `unknown` through one renderer,
  `render_open_obligations`, and keep their exit status.
- The driver's obligation map is populated through an exhaustive
  `ProvKind` mapping shared by the package, denotation and native routes.
- No kernel, `trusted_base()` or spec change.
