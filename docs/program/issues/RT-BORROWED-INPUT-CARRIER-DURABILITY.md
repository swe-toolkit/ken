---
id: RT-BORROWED-INPUT-CARRIER-DURABILITY
title: "CLOSED REFUTED 2026-09-19, nothing landed — the premise has zero live witnesses; its two measured px8l rows were readmitted under RT-IGNORED-PASSING-ROWS-DISPOSITION (merged, mutation-controlled) and return the computed value, not -1. WAS: Execution-parity successor — give a borrowed process input (BorrowedOpaque) a durable carrier representation on the generic carried-value path, so a capture that crosses the closure boundary does not trap at run with `malformed borrowed process input` (object_linker_packaging.rs:2221, native stub value -1)"
status: closed
owner: runtime
size: M
gate: none
depends_on: [RT-CLOSURE-BOUNDARY-RESIDUAL]
blocks: []
github: null
origin: "Steward, 2026-08-24, from the Architect's M4-stop ruling (evt_2dnst700ynbeh) on WIP checkpoint 422310a32. M4's closure-crossing contract is discharged; the post-crossing execution trap is a DISTINCT execution-parity seam, not unfinished M4. Steward framing call (Architect left own-node-vs-fold open): a DEDICATED node, because the nearest family member RT-ENTRY-TRAP-254 is closed and specific (value 254) and CI-SKIPPED-NATIVE-TESTS is CI-coverage tracking, not a mechanism owner. Steward-filed per COORDINATION section 2."
---

> # CLOSED REFUTED 2026-09-19 (Steward, `evt_5mdddpmj3yr8n`). Nothing landed.
>
> **The node has ZERO live witnesses.** Measured at `ba67e549f`, before framing
> it. Everything below this block is the 2026-08-24 filing, preserved as the
> record of what was believed; read it as history, not as a specification.
>
> ## The only measured trigger was readmitted, and the label was disposed
>
> The fixed inputs name exactly two rows — *"First observed on the two
> direct-result `px8l_recursive_decl_native` rows at WIP `422310a32`."* Both are
> now un-ignored and passing (`dynamic_zero_seed_takes_the_base_case`,
> `dynamic_multistep_seed_preserves_updated_parameter_order`). No `#[ignore]`
> attribute remains anywhere in that file.
>
> They were readmitted by `RT-IGNORED-PASSING-ROWS-DISPOSITION` (**merged**),
> whose `AC-1` required a mutation showing each readmitted row goes red when the
> behaviour it covers breaks. **So this is not a row that quietly stopped
> reaching its assertion** — the distinction that node exists to draw. Its
> readmission comment disposes this label in terms:
>
>     The prior label read: "RT-BORROWED-INPUT-CARRIER-DURABILITY: closure
>     crossing succeeds; BorrowedOpaque reaches emit_carrier_tag as native
>     value -1 and traps as ...". That does not reproduce. The native process
>     returns the computed value the row names, not -1, and the trap string is
>     absent from the captured output.
>
>     The label named a trap the row would HIT; the row passes. Both cannot be
>     true, and the disposition is the label's, not the row's.
>
> ## The remaining citations do not carry it either
>
> - **`effects.rs:1112` has decayed.** The filing pins *"Test assertion sits at
>   `crates/ken-runtime/src/cranelift_backend/lowering/core/tests/effects.rs:1112`"*.
>   That line now sits inside the carried-site-operand substitution test and has
>   nothing to do with borrowed ingress.
> - **The one live `-1` assertion is BY DESIGN and passing.**
>   `borrowed_ingress_malformed_metadata_fails_closed` asserts `-1` for three
>   malformed shapes — bad metadata, null data pointer, wrong arity — and pins
>   the stub's trap string. That is the fail-closed path working correctly. **An
>   assertion that the fail-closed value appears on malformed input is not
>   evidence of a durability defect on well-formed input.**
> - **The stub branch still exists**, keyed on `value == -1`, in
>   `object_linker_packaging.rs`. Cite it by that branch, never by line: the
>   filing says `:2221` and it is `:2303` at `ba67e549f`.
>
> ⇒ Nothing in the tree exhibits *"a `BorrowedOpaque` capture crosses the
> closure boundary and traps at `-1`."* The two rows that demonstrated it now
> demonstrate the opposite, under a mutation control.
>
> ## What refuted it, and why nobody noticed for 25 days
>
> `depends_on: [RT-CLOSURE-BOUNDARY-RESIDUAL]` — that dependency **merged on
> 2026-08-25** (`c5fbfb661`), the day after this node was filed. The node sat
> `draft` for 25 days afterwards, and a `depends_on` edge records state at
> filing and is never re-evaluated when the dependency lands. **An unblocked
> node is indistinguishable from a blocked one**, so nothing ever re-asked
> whether the premise survived the landing that was supposed to precede it.
>
> The refutation then arrived through a **third** node — the readmission sweep —
> which correctly disposed the row's label and had no reason to walk back to the
> node the label named. **Each of the three steps was right; the edge between
> them does not exist.** Filed as the generalizable half, not as an incident.
>
> ## What this does NOT establish — stated because it is the easy misread
>
> Both readmitted rows run through `assert_agreement`, whose program is
> `fn main (input : ProcessInput) (_caps : ProgramCaps APartial)` and
> **consumes** its input (`walk (seed input) True`). The unused `_input` in that
> file belongs to `nondecreasing_cycle_is_rejected_before_native_lowering`, a
> compile-rejection test that never runs native.
>
> **So this says nothing about an UNUSED borrowed ingress.** The open runtime
> question — whether ingress validation runs for a `proc main (_input :
> ProcessInput)` the program never consumes — is untouched by this closure and
> remains the live crux for the `px7f_resource_native` rows. Those rows were a
> **candidate** owner's candidates and never members of this node; they carry
> `RT-SITEOP-CARRIED-WITNESS D2` in their own `#[ignore]` strings, and that node
> is merged.
>
> ## Downstream pointers, for whoever picks these up
>
> - `RT-NATIVE-COMPILE-RUNS-AT-THE-STACK-WALL` (`draft`) uses the two px8l rows
>   as its floor-side evidence, correctly dated *"at `1dec48f33`"*. That text is
>   a record of a past measurement and is left as written — but **its ignored
>   population has since changed**, so re-measure before building on the
>   ignored/live split it reports.
> - `RT-CARRIED-IH-DISPATCH-SITEOP`, `RT-CLOSURE-BOUNDARY-RESIDUAL`,
>   `RT-EXITCODE-FAILURE-PAYLOAD-TRANSPORT` and `PX8` name this node as a
>   successor or sibling. All are `merged` or `closed` records of belief and are
>   **deliberately not edited** — correcting a landed record in place would
>   rewrite what was true when it was written.

> # Execution-parity successor — gated behind M4's crossing

## Objective

A `BorrowedOpaque` is a borrowed process input: runtime-local and live-domain
only, with no durable carrier representation. When such a value is captured and
routed across the closure boundary (now that M4's crossing works), it survives
lowering/construction but TRAPS at execution. Give the borrowed process input a
durable carrier representation on the generic carried-value path so the capture
runs correctly end-to-end.

This is the SAME "no durable lane" root cause as the original closure-boundary
refusal, but for a DIFFERENT object: M4 gave the closure its durable lane; the
borrowed process VALUE the closure captured still has none. Building that
representation is a new mechanism on the shared carried-value path, not a member
M4's world already has — the exact M6→M3 pattern (the crossing succeeds and
EXPOSES a deeper, structurally-distinct seam).

## Fixed inputs (measured @ `011bf2a95` / WIP `422310a32`, verified)

- The trap is EXECUTION-layer, owned by the native C stub at
  `crates/ken-runtime/src/object_linker_packaging.rs:2221`:
  `if (value == -1) fputs("ken native trap: malformed borrowed process input\n",
  stderr);`. It is a sibling of the RT-ENTRY-TRAP-254 malformed-payload family
  (that one value 254; the ExitCode family value -3), on the same execution
  layer the Architect named as a distinct residual in the post-M6-landing
  assessment (evt_1vcwzkd3g0s1r). Empty effect trace + RuntimeTrap(1) confirm it
  fails after object emission, at run.
- The surface is a GENERIC SHARED mechanism, not M4's code: `emit_carrier_tag`
  is defined at `crates/ken-runtime/src/cranelift_backend/lowering/mod.rs:7346`
  and called from effects/joins/source and the carried-match at
  `core.rs:12270`. M4's closure transfer merely ROUTES a capture into it — a fix
  here changes a path many non-M4 flows depend on, which is why it is not M4's
  same-contract work.
- Measured trigger (fixed input): a `BorrowedOpaque` capture → the generic
  carried-match `emit_carrier_tag` → the native wrapper value `-1`. First
  observed on the two direct-result `px8l_recursive_decl_native` rows at WIP
  `422310a32`, which cross the boundary honestly (M4 works) then red only here.
  The static-dispatch capture run projected faithfully (`Int, Constructor,
  BorrowedOpaque`, not collapsed/reordered), so the fault is the borrowed-input
  durability, not a capture-projection defect.
- Test assertion sits at
  `crates/ken-runtime/src/cranelift_backend/lowering/core/tests/effects.rs:1112`.

## Relationship to M4 and the re-pointed rows

M4 (`RT-CLOSURE-BOUNDARY-RESIDUAL`) re-points the ignore string of any censused
row that crosses but reds ONLY at this borrowed-input execution seam to THIS
node as owner (per the Architect disposition, exactly as M6 re-pointed
`rt_write_writable_stage` to M3). Those rows carry forward as honest advancing
refusals, not regressions; they green here once this seam lands. As of the M4
stop, at least the two direct-result `px8l_recursive_decl_native` rows re-point
here; M4's widening measurement (its Deliverable 2) determines the full set.

## Sequencing and caution

Draft, execution-parity family. Its resolution greens the re-pointed rows, so it
follows M4's crossing landing (depends_on). Distinct from M3
(`RT-CARRIED-IH-DISPATCH-SITEOP`, the CarriedWord/ConstructorTag lowering seam)
and from M4 (the closure-boundary crossing): this is the shared carried-value
EXECUTION layer. Do not collapse the three because they neighbour on the
carried-value path — they are different objects and different layers. Whether it
stays standalone or later joins the execution-parity family umbrella is a
post-framing call; the Architect reviews the WP when it is framed and released.
