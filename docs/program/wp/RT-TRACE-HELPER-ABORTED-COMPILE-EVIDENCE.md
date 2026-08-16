# RT-TRACE-HELPER-ABORTED-COMPILE-EVIDENCE

**Two trace helpers assert over aborted compiles. Live on `main`.**

Frame. Steward-authored 2026-08-16 on Architect ruling `evt_3bkkjpps1bcpe`,
from the measurement runtime-leader delivered as `D4` of
[[RT-DESCENT-LANE-COMPLETENESS]] (`evt_2fmjv69z5bg2g`, at `3c9b8bbd5`).

**Treat every anchor here as perishable. If a fixed input turns out false
against the landed code, say so and escalate — do not quietly build around
it.** Find each site by the quoted phrase, never by a line number.

## 1. Objective

Make the two helpers unable to report a green assertion from a compile that
never finished, and establish what — if anything — their callers' assertions
still support once that is true.

**This is a live-on-`main` evidence defect, not a retirement question.** It is
cut separately for exactly that reason: folded into the gated retirement it
would stay unfixed while being green.

## 2. Fixed inputs

| input | value |
|---|---|
| **file** | `crates/ken-runtime/src/cranelift_backend/lowering/core/tests/control.rs` |
| **measured at** | `3c9b8bbd5fae09859d6e330f8ac0a17b40fe1f68` |
| **the two helpers** | `owner(...)` and `multiplicity(...)` |
| **the discard, in both** | `let (_result, _trace) = px8j_capture_source_trace(expression, false, symbol);` followed by `d2k_owner_trace_take()` |
| **`multiplicity` also** | builds a `BTreeMap<String, usize>` of descents |
| **expressions per helper** | five |
| **functionized compiles that abort** | **five of five, in both** |
| **abort constructs** | row 1 `PlannerInvariant`; rows 4 and 5 `StaticWorkerBinding` |
| **completed functionized runs** | **zero** |

**The third discarding site is the sentinel** —
`recursive_descent_recursors_compile_without_a_boundary_crossing`, `let
(_excluded_result, _trace) = px8j_capture_source_trace(` — and it is **out of
scope**, owned by [[RT-DESCENT-RETIRE]]'s `D6`.

**Eighteen `set_selector_variant_exclusion(Some(...))` sites exist at base;
exactly three discard the result.** That census is the Architect's and was run
so nobody has to assume either way. **Do not widen the sweep to the other
fifteen.**

## 3. The best guess, stated so a reviewer can attack it

**Assert the compile result in both helpers, then follow the reds.**

The expected outcome is that both helpers red on all five expressions, and that
the reds propagate to every caller whose assertion depends on trace events that
only ever came from an aborted prefix.

**What happens next is the real content, and it is a judgment per caller:**

| what the caller's assertion turns out to establish | disposition |
|---|---|
| a property of the **retiring** lane's trace, incidentally routed through a functionized compile | the helper is measuring the wrong lane — re-home or retire the assertion |
| a property genuinely about **functionized** owner or recognition structure | **it is not established today.** Either drive it through a compile that completes, or record the claim as unsupported with a named owner |
| nothing that survives the abort | delete the assertion and say so |

**Do not repair by making the abort tolerated.** Widening the helper to accept a
failed compile reproduces the defect with a comment on it.

## 4. Deliverables

**`D1`.** Make both helpers assert their compile result. Report which callers
red and on which expressions.

**`D2`.** For each redding caller, classify what its assertion actually
establishes using the three-row table in section 3, and carry out that row's
disposition. **One classification per caller, named individually** — not a
single verdict for the group.

**`D3`.** State explicitly, in the landed artifact, whether any claim of the
form *"the functionized lane produces owner structure X"* survives — and if
none does, say that rather than leaving the absence to be inferred.

## 5. Acceptance criteria

**AC-1.** Neither helper can return trace events from a compile that did not
complete. A test proving this: a deliberately-refusing expression makes the
helper red, and it reds before any trace assertion is evaluated.

**AC-2.** Every caller of both helpers is enumerated, with its red/green state
under `AC-1`'s change recorded per expression. **An enumeration by grep at the
stated SHA, not a list of the ones that reded** — a caller that stays green is
part of the evidence and cannot be discovered by the run alone.

**AC-3.** Each redding caller carries one of the three dispositions in section
3, named individually, with the reason.

**AC-4.** No assertion is preserved by widening a helper to tolerate a failed
compile. The frame's guess is that this is never the right repair; a candidate
that takes it must argue the case explicitly.

**AC-5.** `D3`'s statement is present and answers in the negative where the
negative is true. **An unsupported claim silently dropped does not discharge
it** — the point is that a future reader learns the functionized owner-structure
evidence was never established.

**AC-6.** The sentinel and the other fifteen exclusion sites are untouched.

**AC-7.** No change to the functionized lane's behaviour. This node repairs
**evidence**, not the compiler.

## 6. Why the population is small and the finding is not

**Three of eighteen is narrow, not systemic.** But this exact shape — run a
compile for its side effects, discard the result, assert on the trace —
**concealed the `RecursiveDescent` campaign's blocking finding for nineteen
days.** The sentinel held the answer in `_excluded_result` and threw it away,
while `control.rs` asserts precisely that result at two other places in the same
file.

⇒ **The value here is not the two helpers. It is that a green assertion over an
aborted compile is invisible to every instrument the fleet runs**, and this node
is the one place that fact gets written down where it will be met again.
