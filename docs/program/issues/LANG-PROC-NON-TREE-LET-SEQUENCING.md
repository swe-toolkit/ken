---
id: LANG-PROC-NON-TREE-LET-SEQUENCING
title: "A direct-style proc whose declared result is not a tree but whose row is non-empty elaborates a let through the ordinary-let fallback, so a performing right-hand side whose binder is unused never runs. `proc read : Int visits [Console] = let r = write ... \"a\" in 1`, called from main, prints only main's own output, with exit 0. Spec 36 §2.4 says a let sequences its right-hand side. Rule the normative reading, then repair or refuse fail-closed"
status: ready
owner: verify
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Architect ruling evt_4rqbdqy47e0xm on LANG-LET-HOSTIO-ACTION: a separate pre-existing defect, found by the Verify implementer (evt_5bcfdjwx9bver) and filed by the Steward on request. The Architect named Language as owner. The Steward places it with the Verify ring, which holds the adjacent let-HostIO context while the Language ring runs LANG-STRICT-ISOLATED-ROUTES Increment 3. Steward-filed per COORDINATION section 2."
---

# A let in a direct-style proc sequences its action

## Objective

A performing right-hand side of a `let` inside a direct-style proc runs,
or the program is refused at check time, as the ruling decides. It is
never silently dropped. The case is a proc whose declared result is a
non-tree type such as `Int`, and whose row is non-empty.

## Settled inputs (main `70dc30f97`)

- **The denotation.** Spec 36 §2.4 (`36-effects.md:600` to `:612`)
  gives `⟦ let x = e1 in e2 ⟧ = bind ⟦e1⟧ (λ x. ⟦e2⟧)`, and the grouped
  form sequences every right-hand side.
- **As built** (Verify implementer evt_5bcfdjwx9bver, Architect
  evt_4rqbdqy47e0xm). `check_let_once` enters the ordinary-let fallback
  when the expected type is not a tree. The unused binder's action then
  never runs. The probe:

  ```ken
  program capabilities FS APartial
  proc read : Int visits [Console] =
    let r = write ConsoleStream.Stdout (bytes_encode "a") in 1
  proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
    : HostIO APartial ExitCode visits [Console] =
    let n : Int = read in host_program APartial (print_line "b")
  ```

  `ken run` exits 0 with stdout `b\n` and no `a`.
- **This WP does not touch LANG-LET-HOSTIO-ACTION's repair.** That WP
  fixes the tree-expected let. Its residual names this case, and its
  amended row `proc direct_value : Int visits [Console] = 1` is
  byte-identical in core to the base. Any guard here must keep that row.
- **The form is widespread.** The Architect grepped 20 or more
  non-tree-result `visits` procs at `70dc30f97`. They include the
  performing `conformance/surface/effects/seed-effects.md:119`
  `proc setup () : Config visits [FS, Clock] = read_config "/x"`, the
  class-field-purity seed and tests, `ds8b_pure_into_proc_widening.rs`,
  `purity_keywords.rs` and `surf_space_cells_p1.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

One candidate that carries the repair or refusal the rulings name, a
pin on the probe, and the consumer sweep's result.

## Acceptance

- **AC-0.** The Spec Leader rules the normative reading for a let in a
  non-tree-result proc body with a non-empty row. The candidates are a
  §2.4 bind, a check-time refusal of an unused performing binder, or a
  rule that such a proc's result must be a tree. The Architect then
  rules the repair site, its fan-in and the consumer sweep. A spec
  change beyond 36, or a change to proc result typing, is a stop to the
  Steward.
- **AC-1.** Measure the base first: the probe is predicted to print
  `b\n` with exit 0, and if the base prints `ab\n`, stop and report.
  The candidate pins the probe's ruled outcome: stdout `ab\n`, or the
  ruled refusal at check time with a nonzero exit.
- **AC-2.** The consumer sweep. Every non-tree-result `visits` proc
  that the Architect's grep finds checks as before, or changes as the
  ruling names, listed by file. `proc direct_value : Int visits
  [Console] = 1` stays byte-identical in core.

## Stop conditions

- A kernel or trust change.
- A repair that changes a sweep consumer's observable output in a way
  the ruling did not name.

Checks: 8 fired: the probe yields `b\n` on the base, which cannot
satisfy `ab\n` or a refusal, so AC-1 discriminates. 3 fired: the sweep
population is the Architect's grep across catalog, crates tests,
examples, conformance and ken-cli tests at `70dc30f97`, not the probe
alone.
