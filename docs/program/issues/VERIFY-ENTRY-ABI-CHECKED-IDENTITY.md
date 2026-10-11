---
id: VERIFY-ENTRY-ABI-CHECKED-IDENTITY
title: "The runner, the interpreter's console ids and program admission read the entry ABI (Success, Failure, Ret, Vis, Write, Stdout, HostIO, host_exit and more) from elab.globals by spelling, so a top-level const reusing one of those names, in an example or a plain fence, changes what a valid program runs. Key every entry-ABI read on checked prelude identity"
status: ready
owner: verify
size: M
tier: T1
gate: architect
depends_on: [VERIFY-EXAMPLE-MAIN-NOT-ENTRYPOINT]
blocks: []
github: null
origin: "Adversary report evt_10139ky1vcvzp on 131897bc8, anchors confirmed by the Steward on main. VERIFY-EXAMPLE-MAIN-NOT-ENTRYPOINT restored only main. Same class as VERIFY-GLOBALS-IDENTITY-CHECKED-INSERT (a lookup by spelling in a flat mutable table). Steward-filed per COORDINATION section 2."
---

# The entry ABI is read by identity

## Objective

What a valid program runs, and the exit status it reports, does not depend
on any user declaration's spelling: every entry-ABI read resolves the
prelude's checked identity.

## Settled inputs (Adversary `evt_10139ky1vcvzp`, at `131897bc8`)

- By-spelling reads from `globals`:
  - `crates/ken-cli/src/lib.rs` ~:255-256 (`Success`, `Failure`, feeding
    `exit_status`);
  - `crates/ken-interp/src/eval.rs` ~:3026-3036 (`ConsoleIds::from_elab`:
    `ITree`, `Ret`, `Vis`, `Read`, `Write`, `Stdout` and others);
  - `crates/ken-elaborator/src/program_admission.rs` ~:103-114 (`Cap`,
    `HostIO`, `host_exit`, `ExitCode`, `Success`, `Failure`, `Ret`, `Nil`,
    `Cons`, `MkProd`); the native plan takes `success_constructor` from
    there.
- `PreludeEnv` (`prelude.rs` ~:313) already holds checked ids for some of
  these (`inl_id`, `inr_id`, `Nil`, `Cons`).
- Reported, not yet measured by the ring: beside `fn main = host_exit
  ANone Success`, an example `const Success : Nat = Zero` turns `Ok(0)`
  into `Ok(1)`. Beside a printing `proc main`, an example or plain-fence
  `const Stdout`, `Write`, `Vis` or `Ret` makes the run fail. A control
  `const zz_unrelated` changes nothing.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

AC-0 first. Then every entry-ABI read on the run, interpreter and
admission routes takes the prelude's checked identity; the Architect rules
the carrier (extend `PreludeEnv` or an equivalent) at D0.

## Acceptance

- **AC-0 (measure).** The rows above on 131897bc8, each against its
  control, and every by-spelling entry-ABI read across `crates/`, swept by
  mechanism (a `globals` lookup on the run, interpreter, admission or
  native-plan path), not only the three files named.
- **AC-1.** Each AC-0 row runs and exits as its control does, on `ken run`
  and native admission, from an example fence and a plain fence.
- **AC-2 (control).** Programs that rebind none of these names are
  unchanged.
- **AC-3 (mutation, QA).** Restoring one spelling read reddens its row.

## Stop conditions

- The repair needs a rebinding of a prelude name to become a refusal: that
  is the strict-resolution flip on L2, not this node. Stop and report.
- Any kernel, trust or spec change.
