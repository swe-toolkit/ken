---
id: KERNEL-CONV-RECURSIVE-HEAD-TOTALITY
title: "Restore kernel conversion totality at the distinct recursive-head boundary — converting two separately declared, source-isomorphic recursive transparent globals under a stuck eliminator currently unfolds their distinct self GlobalIds without bound and stack-overflows, instead of returning false and halting as the landed spec §17 contract now requires. Implement the finite §3.5 cross-identity boundary (no clone-equality, bisimulation, certificate, self-id rewrite, or custom normalizer), preserving ordinary same-head recursion, finite δ, and distinct-nonrecursive common reducts, and wire the executable black-box matrix."
status: draft
owner: kernel
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-08-30, filed on the Architect directive evt_7spzy25qqdsqx (re-confirmed evt_2q6s5215q1cth) after the Spec §17/conformance erratum LANDED (squash c2c12b090, blob-verified). The erratum specified the contract — distinct separately declared recursive self GlobalId heads under a stuck eliminator are non-convertible, observable false plus halting — and surfaced a real kernel totality bug: the current conversion diverges (stack overflow) on that case. The Architect ruled the repair a SEPARATE non-blocking follow-on to be framed after the erratum lands, and preselected no implementation technique. Steward-filed per COORDINATION section 2."
---

> # DRAFT — operator-gated. Kernel is NOT one of the three active lanes
> # (runtime/verify/foundation), so this is NOT released. It is SURFACED to the
> # operator (offline until 2026-08-30 14:00 UTC) as a lane/dispatch decision:
> # whether to authorize a kernel-lane turn for this soundness-adjacent totality
> # repair. Non-blocking — nothing waits on it (CAT-PRELUDE removed the duplicate
> # local recursive heads and does not depend on cross-identity conversion).
> #
> # The spec prerequisite is LANDED: the §17/conformance erratum (squash
> # c2c12b090) fixed the CONTRACT; this node fixes the KERNEL to meet it. Framed
> # now, while the contract and the Architect's black-box matrix are fresh, per
> # the Architect's directive. Do not release without operator authorization of a
> # kernel lane.

## The contract (landed spec §17, erratum c2c12b090)

Separately declared transparent recursive globals whose source clauses/types are
isomorphic and whose only body difference is the self `GlobalId` remain DISTINCT
RIGID HEADS. Ken has no cyclic/bisimulation quotient and no equality certificate.
When an open Π-η comparison reaches a stuck eliminator whose methods expose two
such distinct recursive self-ids, the required observable is **`false` and
halts** — repeated unfolding would only recreate the comparison on a fresh
neutral tail. SCT (the size-change/termination argument) certifies recursive
re-entry WITHIN one admitted group; it does NOT license unbounded symbolic
cross-identity unfolding beneath a stuck eliminator. Finite ordinary reduction,
same-head congruence, and distinct-nonrecursive-head common reducts stay
available and return their existing verdicts.

## The bug (current kernel, measured at c2c12b090)

`crates/ken-kernel/src/conv.rs` implements conversion: `convert` (`:337`),
`convert_type` (`:397`), `unfold_const` (`:37`), and the whnf δ-step over a stuck
eliminator (`~:44-78`; the module doc at `:13` already notes "δ is cyclic
post-K2c"). On the distinct-recursive-head case above, the δ-step unfolds the two
distinct self-ids without a finite boundary and the conversion query DIVERGES —
observed as a stack overflow rather than a `false` verdict. That is a conversion
TOTALITY gap: a well-typed conversion query that neither returns nor halts.

**Why CI is currently green (D0 must confirm the exact reason).** The erratum
landed with the conformance seeds present but the workspace gate green, so the
distinct-recursive-head seed is either not yet wired to an executor against the
kernel, or is pending/expected-fail. D0 establishes which; D1 makes it executed
and green.

## Fixed inputs (measured at `origin/main` `c2c12b090`)

- `crates/ken-kernel/src/conv.rs`: `convert` `:337`, `convert_type` `:397`,
  `unfold_const` `:37`, whnf δ-over-stuck-eliminator `~:44-78`. Re-measure before
  editing — coordinates decay.
- Landed conformance contract:
  `conformance/kernel/conversion/seed-conversion.md` (the distinct-recursive-head
  case, `map_f`/`map_g`; the stack instrument `CONVERSION_STACK_BYTES =
  2 * 1024 * 1024`, child `env_remove("RUST_MIN_STACK")`, normal exit required),
  plus the coupled §17/§18/§13/§14/README statements landed in c2c12b090.

## Deliverables

- **D0 — locate and reproduce.** Name the exact `conv.rs` recursion site where the
  two distinct recursive heads under a stuck eliminator unfold without a finite
  boundary. Reproduce the divergence with two well-typed, separately admitted
  source-isomorphic recursive maps in the 2 MiB-stack child, and record the
  current behavior (overflow/non-termination). Establish why the workspace gate is
  currently green for the landed seed (unexecuted vs. pending), so D1 knows what it
  must turn on.
- **D1 — the finite boundary + the executable matrix.** Implement the §3.5 finite
  cross-identity boundary so the distinct-recursive-head case returns `false` and
  halts, and wire the black-box matrix below as durable conformance/kernel tests
  under the stated stack instrument. The technique is the implementer's, guided by
  the Spec erratum; this frame preselects none.

## Acceptance criteria

- **AC-MATRIX** — the four black-box cases (Architect `evt_7spzy25qqdsqx`), each an
  executed control:
  1. two well-typed, separately admitted source-isomorphic recursive maps:
     `convert_type == true`, value `convert == false`, and the call HALTS on the
     ordinary test stack.
  2. same recursive head with equal spines: `true` and halts, with no δ expansion.
  3. distinct NONrecursive transparent heads with a finite common reduct: existing
     `true` behavior preserved.
  4. a genuinely different recursive body: `false` and halts.
- **AC-STACK-INSTRUMENT** — termination is observed through the stated instrument,
  not a workaround: `CONVERSION_STACK_BYTES = 2 * 1024 * 1024`, child
  `env_remove("RUST_MIN_STACK")`, an explicit worker stack, normal child exit
  required. `RUST_MIN_STACK`, a timeout-as-success, a larger stack, or a
  signal/overflow treated as pass are NONE of them evidence — a control that
  relies on any of these is a HARD STOP, not a landing.
- **AC-NO-CLONE-EQUALITY** — the fix adds NO clone-equality relation, graph
  bisimulation, equality certificate, self-`GlobalId` rewrite, larger fixed stack,
  or second/custom normalizer. Each would either assert the refuted equality or
  create a second conversion relation. The ruling preselects no visited-pair or
  rigid-head shortcut; a blanket "distinct self-recursive heads ⇒ false" rule is
  WRONG (it fails matrix case 3 and the canonical-input positives) and is a HARD
  STOP.
- **AC-PRESERVE-ORDINARY** — same-head recursive comparison, finite ordinary
  β/ι/δ reduction, and distinct-nonrecursive-head common reducts keep their
  current verdicts; the existing kernel conversion suite stays green in CI.
- **AC-CONFORMANCE-EXECUTED** — the landed `seed-conversion.md` distinct-head
  contract is now EXECUTED against the kernel and passes (closing the D0 gap), and
  the workspace/conformance gate is green in CI.

## Reviewers

Architect — the mechanism is a genuine finite cross-identity boundary that
preserves ordinary reduction and same-head congruence, NOT a clone-equality,
bisimulation, certificate, self-id rewrite, or custom normalizer, and not a
blanket rigid-head shortcut; the soundness argument (distinct rigid heads have no
cyclic equality hypothesis) holds. kernel-QA — the four-case matrix reds/greens
exactly as specified, the stack instrument is causally honest (neutering the
boundary reddens case 1; the instrument rejects `RUST_MIN_STACK`/timeout/larger
stack), and no ordinary-reduction regression. conformance-validator — the
executable conformance controls match the landed §17 contract, and the stack
contract is pinned (fixed numeric bound, `RUST_MIN_STACK` removed).

## Capability tier

T1 — a soundness/totality-bearing kernel conversion change, reviewed on the
argument (a finite boundary that returns false without a clone-equality relation
and without regressing ordinary reduction), not on a differential diff. Size M.

## Sequencing

Kernel team. **NOT an active lane** — operator-gated; surfaced to the operator for
a lane/dispatch decision, not released. The spec §17 contract prerequisite is
LANDED (`c2c12b090`), so `depends_on` is empty. Non-blocking: no in-flight node
depends on cross-identity conversion, and CAT-PRELUDE removed the duplicate local
recursive heads rather than relying on their (non-)convertibility. This is the
"real fix" behind CAT-PRELUDE's explicitly review/census-enforced residual; when
this lands, that residual is discharged for the recursive case in the kernel.

## Symptom inventory (append one line per hard-stop; never rewrite history)

```text
(none yet)
```
