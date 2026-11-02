---
id: LANG-EFFECT-ROW-CLOSURE-BODY
title: "The row plane gives every lambda the empty row and never walks its body, so a closure that performs an undeclared effect escapes once a higher-order function applies it. `fn sneaky ... = host_bind ... (\\_. action MkUnit)`, with `action` a Console proc and the program declaring only FS, checks with rc 0, and `ken run` prints the Console output. Pay a lambda's body row where the lambda is built, as globals already pay at naming"
status: active
owner: verify
size: M
tier: T1
gate: architect
depends_on: [LANG-EFFECT-ROW-LOCAL-LATENT]
blocks: []
github: null
origin: "Architect AC-0a evt_4e3f0nek594k6 on LANG-EFFECT-ROW-LOCAL-LATENT: a boundary escape on the same row plane that the local-binder rule does not close. Steward scope decision evt_66fsaedygqr6f: a separate soundness WP, sequenced immediately after LOCAL-LATENT and ahead of LANG-PROC-NON-TREE-LET-SEQUENCING. Steward-filed per COORDINATION section 2."
---

# A closure pays its body's row where it is built

## Objective

A lambda contributes its body's row to the expression that builds it, so
a closure that performs an effect cannot carry that effect past a
declaration or a program header that does not declare it. The escape
gate then refuses `p5`.

## Settled inputs (main `3aaea53e4`)

- **The rule.** Spec 36 §1.2 joins every subexpression's row, and §1.4
  makes the escape check the single soundness-relevant gate. Globals
  already pay their row where they are named (`RCon -> row(g)`).
- **As built.** In `infer_expr_row_type` (`elab.rs:17767`), the `RLam`
  arm (`:17795`) returns the empty row and does not walk the body. No
  consumer releases a closure's row when a higher-order function such as
  `host_bind` applies it.
- **Measured fail-open** (Architect, base binary; fixtures in
  `/workspaces/ken/local/architect-shared/effect-row-local-latent/`).
  `p5_lambda_cont.ken` checks with rc 0, and `ken run` prints hex `61`:

  ```ken
  program capabilities FS APartial
  proc action (x : Unit) : HostIO APartial Unit visits [Console] =
    host_bind APartial (Result IOError Unit) Unit
      (host_console APartial (Result IOError Unit)
        (write ConsoleStream.Stdout (bytes_encode "a")))
      (\_. host_pure APartial Unit MkUnit)
  fn sneaky (u : Unit) : HostIO APartial Unit =
    host_bind APartial Unit Unit (host_pure APartial Unit MkUnit)
      (\_. action MkUnit)
  proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
    : HostIO APartial ExitCode visits [FS] =
    host_bind APartial Unit ExitCode (sneaky MkUnit)
      (\_. host_pure APartial ExitCode ExitCode.Success)
  ```

  No definition on the path declares Console.
- **The ruled direction** (AC-0a). The body is walked with
  LOCAL-LATENT's `LatentScope` (`elab.rs:17700`), and the lambda's own
  binders are opaque.
  LOCAL-LATENT landed as `3aaea53e4`, so this WP builds on its plane.
- **The refusal population is new.** Every proc continuation that names
  an effect its enclosing declaration does not declare is newly refused.
  The sweep is this WP's own and is not LOCAL-LATENT's.
- **The census sentinel moves.** `lang_strict_isolated_routes_census.rs`
  pins a count over every `.ken` and `.ken.md` file under `examples/`,
  `conformance/` and `crates/*/tests`. A new fixture there re-pins it,
  classified per file, as LOCAL-LATENT's respin did (`adde8892a`).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

One candidate in which the `RLam` arm contributes its body's row, with
the consumer sweep's result and the pins below.

## Acceptance

- **AC-0 (Architect).** Rule the arm's shape on the landed LOCAL-LATENT
  plane, and the consumer sweep: the catalog, crates tests, examples and
  conformance programs that the corrected gate newly refuses, with a
  disposition for each. A D0 recorder that logs old and new rows per
  consumer is the expected measurement.
- **AC-1, the gate.** `p5`'s `sneaky` is refused with an escape naming
  Console. A same-form control whose continuation performs only a
  declared effect stays accepted.
- **AC-2, a mutation.** Restoring the `RLam` arm to the empty row makes
  AC-1's refusal accept (red).
- **AC-3, no regression.** Every program the sweep names checks as
  before or changes as AC-0 rules, listed by file. `trusted_base()` is
  unchanged.

## Stop conditions

- A kernel or trust change.
- A spec 36 change beyond stating the rule, or a change to a row typing
  rule.
- The corrected gate refuses a program that AC-0 did not name.

Checks: 2 fired: the choke point is the `RLam` arm of the one row-plane
function, and its fan-in is the LOCAL-LATENT consumers, all of which
take the same scope parameter. 5 fired: the plane carries no body row
for a lambda today, so the property the gate needs is not on it. 8
fired: on the base `p5` exits 0 and prints `61`, which cannot satisfy
AC-1's refusal, and AC-2's mutation names the axis that must redden.
