---
id: LANG-EFFECT-ROW-LOCAL-LATENT
title: "The effect-escape gate releases a local binder's latent row: infer_expr_row_type has no typing context, so a callback parameter `f : A ->[Console] B` applied as `f a` contributes the empty row. A declared-pure `fn` that calls a Console callback checks with rc 0 and prints the callback's output, though spec 36 §1.4 makes the escape check the single soundness-relevant gate of the row system. Give the row plane the binders' latent rows, so every consumer of the plane answers alike, and treat an unknown latent row as unknown, never as empty"
status: ready
owner: verify
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Architect ruling evt_5q34qa1bnd950 on LANG-LET-HOSTIO-ACTION (QA block evt_3r5eknaaejceb): a soundness defect in the shared row plane, filed by the Steward on request and placed ahead of LANG-PROC-NON-TREE-LET-SEQUENCING because the gate fails open. The Verify ring takes it after LET-HOSTIO-ACTION while the Language ring runs LANG-STRICT-ISOLATED-ROUTES Increment 3. Steward-filed per COORDINATION section 2."
---

# The escape gate sees a local binder's latent row

## Objective

A call through a local binder whose type carries a latent effect row
contributes that row to the expression's row, as spec 36 §1.2's
`latent(typeof(Γ, f))` requires. The escape gate then refuses a
declared-pure `fn` or an under-declared `proc` that performs through a
callback. Every consumer of the row plane reads the same answer.

## Settled inputs (main `dd2da30f0`)

- **The rule.** Spec 36 §1.2 takes a call's row from
  `latent(typeof(Γ, f))`. §1.4 (`36-effects.md:168`) calls the escape
  check "the single soundness-relevant gate" of the row system.
- **As built** (Architect evt_5q34qa1bnd950). `infer_expr_row_type`
  (`elab.rs:17292`) is keyed on the global name and id row maps and has
  no Γ. `RVar` is always the empty row, and `RApp` only joins its
  operands. The latent-row map is empty by construction; the comment at
  `elab.rs:16762` says "Until production latent-row extraction is wired,
  this map is empty".
- **The plane has five consumers**, and the defect reaches each of them:
  - the `let` check and the `let` infer body;
  - `check_surface_purity`, the escape gate;
  - instance field purity;
  - `elaborate_space_decl`.
- **Measured fail-open.** The probe below checks with rc 0, and
  `ken run` prints hex `61 62 0a`: the Console callback runs inside a
  `fn`.

  ```ken
  program capabilities FS APartial
  proc action (x : Unit) : HostIO APartial Unit visits [Console] =
    host_bind APartial (Result IOError Unit) Unit
      (host_console APartial (Result IOError Unit)
        (write ConsoleStream.Stdout (bytes_encode "a")))
      (\_. host_pure APartial Unit MkUnit)
  fn run_with (f : Unit ->[Console] HostIO APartial Unit)
    : HostIO APartial Unit =
    f MkUnit
  proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
    : HostIO APartial ExitCode visits [Console] =
    host_bind APartial Unit ExitCode (run_with action)
      (\_. host_program APartial (print_line "b"))
  ```

  A `proc run_with ... visits [FS] = f MkUnit` is also accepted with no
  Console escape. The control `fn run_global (u : Unit) = action MkUnit`,
  through the global Console proc, is refused with "EffectEscapes in
  'run_global': Console". The Architect's fixtures are in
  `/workspaces/ken/tmp/arch-lethoq-probe/`, and QA's higher-order and
  cross-signature fixtures are in `/workspaces/ken/tmp/`; these paths
  are perishable.
- **Precedent for an unknown row.** `projected_field_row_type` treats an
  unknown checked class owner as a singleton unknown, not as empty.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

One candidate in which the row plane carries the latent rows of local
binders, with every consumer reading it. It needs no change local to
`let`. It carries the pins below and the consumer sweep's result.

## Acceptance

- **AC-0 (Architect).** Rule the plane's shape: how Γ reaches it, how a
  latent row is read from a binder's type, and what an unknown latent
  row is. Rule the consumer sweep: the programs in catalog, crates
  tests, examples and conformance that the corrected gate newly refuses,
  and the disposition of each.
- **AC-1, the gate.**
  - The `fn run_with` probe is refused with an escape naming Console.
  - The under-declared `proc run_with ... visits [FS]` is refused for
    Console.
  - A pure-callback control, `f : Unit -> HostIO APartial Unit`, stays
    accepted and unexecuted.
- **AC-2, the shared plane.** QA's higher-order `let` fixture prints hex
  `61 62 0a` through LET-HOSTIO-ACTION's unchanged dispatch.
- **AC-3, unknown is not empty.** A binder whose latent row is unknown
  is treated as unknown. A mutation that maps unknown to empty must
  redden AC-1's first row.
- **AC-4, no regression.** Every program the sweep names checks as
  before or changes as AC-0 rules, listed by file. `trusted_base()` is
  unchanged.

## Stop conditions

- A kernel or trust change.
- A spec 36 change beyond stating the plane, or a change to a row typing
  rule.
- The corrected gate refuses a program that AC-0 did not name.

Checks: 2 fired: the plane's fan-in is five consumers, all named, and
the escape gate is measured failing open on the same input. 5 fired:
the plane carries only global name and id rows, with no Γ, so the
property the gate needs is not on it today. 8 fired: on the base the
probe exits 0 and prints `61 62 0a`, which cannot satisfy AC-1's
refusal, and AC-3's mutation names the axis that must redden.
