---
id: V3-FO-SEARCH-FUEL-STACK-AGREEMENT
title: "Relate find_certificate's fuel budget to the depth the production stack actually survives, so the FO route's designed refusal cannot be pre-empted by an abort, and name the measured quantity in the printed report"
status: active
owner: language
size: S
gate: none
depends_on: []
blocks: []
github: null
origin: "Adversary hunt evt_6k9w72heevm9w on the merged range 7512b1e8b...40ed5d6e9 (V3-FO-CONVERSION-LOAD-MEASURED D0-D4). Every coordinate below was re-verified by the Steward against origin/main 0a19e3714 before filing, and one inference in the report is qualified rather than carried. Steward-filed per COORDINATION section 2."
---

## The property, and it is about two numbers that nothing relates

**`find_certificate` already has the right mechanism.** `fo_kripke.rs:949`
verified:

```rust
pub fn find_certificate(f: &IForm) -> Option<Cert> {
    ...
    search(&root, &mut next_param, 200)
}

fn search(sequent: &Sequent, next_param: &mut usize, fuel: usize) -> Option<Cert> {
    if fuel == 0 { return None; }
```

⇒ **Fuel exhaustion IS the designed refusal.** `None` falls through to IPC,
which is the route's *"refusal is always safe and always available"* posture
working exactly as intended. **Nothing here is missing a valve.**

**What is missing is any relation between the fuel budget and the stack.**
Verified at `origin/main` `0a19e3714`:

| fact | evidence |
|---|---|
| fuel is the constant `200` | `fo_kripke.rs:949` |
| `crates/ken-elaborator/src/` contains **no** `stack_size`, `stacker`, or `grow(` | censused, zero hits |
| `run_with_big_stack` is **test-only** | occurs in four files, all under `tests/` |

⇒ **Production runs on the default stack, and no code relates how deep `search`
may descend to how deep the stack survives.** Where the fuel budget permits a
descent the stack cannot take, the outcome is **an abort, not a refusal** — the
one outcome this route's design says is unavailable.

## The "3.5x" ratio is NOT established, and the frame must not carry it

**The report states the fuel is set 3.5x above the depth that survives**, from
`D4`'s depth-56 datapoint. **That compares numbers measured on different
recursions**, and this node must not inherit the equation:

| recursion | what its depth counts |
|---|---|
| `search` (`fo_kripke.rs:952`) | one level per proof-search step; **this is what `fuel` bounds** |
| `check_tree` (`:812`) | one level per **certificate node** |
| the elaborator on a depth-56 formula | **what `D4` actually measured**, and it is neither of the above |

**They are related — a deeper formula drives a deeper search — but no measured
function maps one to the other.** `D4`'s 56 is a bound on elaborating a formula
on an 8 MiB thread; it is **not** a measured bound on `search`'s recursion.

> ### THE FINDING'S DIRECTION IS RIGHT AND ITS ARITHMETIC IS UNMEASURED.
>
> **"Two numbers must agree and nothing relates them" stands on its own** — it
> needs no ratio. **`D0` establishes the real one.** Do not open by assuming
> the answer is near 56, and do not open by assuming it is near 200.

## Deliverables

**`D0` — measure what `search` actually survives on the default production
stack.** Drive `find_certificate` to increasing search depth and find the
boundary. **Not on `run_with_big_stack`** — the whole question is about the
stack production actually has. **Report the depth and the failure mode**
(abort vs refusal), and state the profile: a debug build's frames are larger
than a release build's, **so measure the one production ships and say which.**

**`D1` — relate the fuel constant to `D0`'s measurement.** If the surviving
depth is below `200`, **lower the fuel** so exhaustion is reached before the
stack is. **The repair is a smaller number, not a bigger stack** — the valve
exists and is correct in kind; adding stack machinery to production would be a
new mechanism where a constant suffices.

**State the relation in the code**, not only in the commit: a bare changed
constant is indistinguishable from a tuning tweak, and the next person to raise
it for reach will re-open this.

**`D2` — name the measured quantity in the printed report.** The module doc's
`AC-2` correction is thorough and the test name is now honest, **but the
artifact a reader pastes elsewhere is the `eprintln!` block**
(`v3_fo_conversion_load_measured.rs:295`), which reads:

```
=== V3-FO-CONVERSION-LOAD-MEASURED: D1-D4 report ===
label   formula_depth   cert_nodes   wall_clock_us   outcome
```

**Nothing printed says what was measured.** The heading carries the withdrawn
framing and the columns are neutral timings, so the report travels as
*"conversion load measured"* plus numbers **with the correction left behind in
the source file.** **One line under the heading naming the Rust reference
checker is enough** — it must travel with the numbers.

> **The node id in the heading is a legitimate identifier and is not the
> defect.** Do not rename the node.

## Acceptance criteria

**`AC-1`.** **`D0`'s number is measured on the production stack and the shipped
profile**, both stated. **A depth measured under `run_with_big_stack` does not
discharge it** — that helper is exactly what makes the test disagree with
production.

**`AC-2`.** **If `D1` changes the constant, a control demonstrates the new value
refuses where the old one would have aborted.** A changed constant with no
control is a claim, not a repair.

**`AC-3`.** **No new stack mechanism in `crates/ken-elaborator/src/`** — no
`stack_size`, no `stacker`, no manual growth. **`AC-3` is the scope boundary and
the reason this node is `S`.**

**`AC-4`.** **No FO `Proved` verdict and no change to the refusal-to-IPC
fallthrough.** Fuel exhaustion must still return `None` and still fall through.

**`AC-5`.** No-regression, in CI (`COORDINATION §12`).

## On reachability, stated honestly rather than argued

**`V3-FO-CONVERSION-LOAD-MEASURED` `D1` measured that no Ken program in this
repository produces an FO-quotable obligation**, so today's reachable population
is **empty**.

> **That is the same corpus-scope argument the Adversary raised on
> [[RT-SYNTHESIZED-ENV-RECORD-OCCURRENCE]]: *"no program does this today"* is
> not *"no program can."*** Here it is **cheaper to close than to argue** — the
> fix is one constant — which is why this is filed rather than deferred to
> whenever the population becomes non-empty.

**This is not urgent and it is not a soundness hole.** An abort is a crash, not
a wrong answer, and `AC-4` keeps the refusal path intact. **It is filed because
the route's stated posture is that refusal is always available, and there is a
depth band where it is not.**

## Banned scope

- **Raising the fuel to increase reach.** Opposite direction; a separate
  question, and it would need `D0` first anyway.
- **Adding stack machinery to production.** `AC-3`.
- **Touching `embed`, the slice, or the certificate format.**
- **Renaming `V3-FO-CONVERSION-LOAD-MEASURED`** or reopening its `AC-2`.

## Sequencing

**Queued behind the two `ready` lane-2 nodes**
([[V3-FO-GUARD-SHIFT-DIFFERENTIAL]], [[V3-FO-DISCOVERY-BOTTOM-OVERCOLLECT]]) and
behind `V3-Z3-EMISSION-CONTROL` `D2a`. **Nothing blocks on it.**

**`D2` is a one-line edit and may be folded into any lane-2 candidate that
touches that test file**, rather than waiting for this node to be dispatched.
