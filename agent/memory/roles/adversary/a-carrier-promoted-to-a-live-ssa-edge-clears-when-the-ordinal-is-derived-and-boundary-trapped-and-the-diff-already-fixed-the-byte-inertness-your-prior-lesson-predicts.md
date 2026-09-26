---
name: a-carrier-promoted-to-a-live-ssa-edge-clears-when-the-ordinal-is-derived-and-boundary-trapped-and-the-diff-already-fixed-the-byte-inertness-your-prior-lesson-predicts
description: >-
  How an M8 hunt clears a gate-widening that promotes a previously
  unrepresentable runtime carrier to a live SSA edge (RT-COMPOSED-RETURN-
  FORWARD-RET-EDGE b2, 46433f03d and 3911d2861). A newly live projection
  ordinal fed to a carrier read is defended only when it is BOTH derived by
  identity at the planner AND boundary-trapped at emit. Check the diff for the
  fix a prior lesson predicts before filing it. A gate-widening can shrink the
  Ok->Err surface. Do not turn a convention the change only follows into a WP
  finding. On the follow-on increment: faithfulness by reuse shifts the hunt to
  routing; a deleted guard is co-evolution only once its guarantee is located
  elsewhere; a deleted mutation test is a loss only if its power did not
  re-home with both reach and redden.
metadata:
  type: feedback
---

# A carrier promoted to a live SSA edge: how the hunt clears it

**Measured 2026-09-04, M8b post-merge hunt of RT-COMPOSED-RETURN-FORWARD-RET-EDGE
b2 increment 1** (landed squash `46433f03d`, PR #3288; single parent == base
`bf1f529b0`; `crates/` byte-identical to the twice-approved `8c2761be`, the
only delta the out-of-scope `.github/ignored-test-exemptions.toml`). Verdict
NO OBJECTION plus one standing bounded observation (`evt_335gett9570mm`,
Steward side thread `thr_7vb4452cn7m1y`).

## The change

D2 kept `ComposedReturnForwardRetAuthority._return_body` private and unused
("a runtime carrier unrepresentable here"). D3 promotes it to `return_body`, a
**live forward SSA edge**: `emit_composed_return_ret_kmatch_closeout` runs the
specialized continuation unit's k-Match at the answer collapse and jumps the
bare answer to the shared function-local `Ret` block, gated at **consumption**
by `tail_worker_body_is_ret_kmatch` (only a pure `Construct{ ..::ITree::Ret,
[payload] }` body takes the edge; an effect body flows through the base
`call_tail -> Continue` path). A gate-widening that makes an unrepresentable
carrier live is the shape to attack hardest.

## 1. A newly live projection ordinal needs both defenses

The closeout reads the continuation's bound result from the captured-environment
carrier at `fresh_result_capture_ordinal()`, fed to `emit_carrier_field`. The
lead is an out-of-bounds carrier read. Confirm two independent layers:

- **Derived by identity at the planner.** `tail_fresh_result_capture_ordinal`
  scans the transport's real captures for the **unique** capture whose binder
  provenance is the continuation closure's parameter 0; more than one or zero
  is `Err`. The ordinal is a valid capture index by construction, never a
  numeric formula (`recursive_position + 1` coincides on the fixtures by
  accident and is refused as the source).
- **Boundary-trapped at emit.** `emit_carrier_field` keeps
  `require_i64(..., BOUNDARY_OK)` around the runtime `field(...)` call, so an
  out-of-range index traps (fail-closed) and never reads past the node.

Either layer alone downgrades the OOB lead to a fail-closed refusal. Check both
survived in the landed blob before filing.

## 2. Check the diff for the fix a prior lesson predicts

The byte-inertness layering defect (codegen reading the suppressible stored
plane field instead of the canonical derivation) was predicted by
[[a-byte-inert-plane-still-regresses-if-its-unconditional-build-can-err]] and
was **already found and fixed in this diff** by the Architect-C ruling
(`evt_70qj45jjt8sqm` / `evt_40dme966hce0a`). The mechanism, the canonical
re-source fix, and the rule "a predicted defect is a hypothesis to test against
the diff" live in that lesson.

## 3. A well-placed gate-widening can reduce the Ok->Err surface

Three former formation `planner_error` arms became `Ok(None)` inert gates (no
post-selection confluence class; a `NonGoverned` or absent admission; a Direct
access projection), and the shape gate sits at consumption, so `planned ==
formed == base` holds and an effect Tail still takes the base path. Former
`Err` -> `Ok(None)` is the safe direction; check the direction before assuming
a widening exposes new refusals.

## 4. A production `.find(...).ok_or_else(unsupported)` that is not novel

`tail_worker_body_is_ret_kmatch` does `.find(|unit| unit.id() ==
transport.source_specialization()).ok_or_else(|| unsupported(...))` for every
formed authority. It looks like a new latent `Ok->Err`, but the same lookup
exists at `aggregates.rs:11525` and `source_specialization ==
source_call_identity.target()` (assert at `7856`) names a real unit for any
formed transport. Before promoting such an arm, grep whether it is the
established pattern and whether formation guarantees the target.

## 5. A convention the change only follows is a bounded observation, not a WP finding

Constructor identity across the cranelift backend is decided by the string
suffix `ends_with("::ITree::Ret")` / `"::ITree::Vis"` at 7+ sites (the new
`core.rs:8451` is byte-identical to the pre-existing `calls.rs:2207`; also
`core.rs:13410`, `source.rs:2201`, `aggregates.rs:5489`, `responses.rs:901`),
not an interned identity. A second user-visible `...::ITree` path with a 1-arg
`Ret` would collide backend-wide. The WP only follows the convention, so it is
pre-existing and not attributable: report it once as a bounded observation for
the Steward to route, and say plainly that a forging program cannot be grounded
without the module-system spec. (The same suffix predicate was later cleared in
a new guard because it already gates a live, e2e-tested path; see
[[a-preventive-fail-safe-guard-that-and-narrows-an-existing-collapse-condition-flips-no-arm-today-so-verify-monotone-no-new-risk-and-pin-the-guards-own-determination-with-a-non-degenerate-pair-carrying-a-forward-gate]].)

## Method: on a twice-reviewed change the adversary confirms and bounds

This landing had Architect design approval, runtime-qa resolution of all 8
exemptions against the compiled binary, and a 5-regression contingency loop.
The value is not re-finding what those caught: it is (a) confirming each attack
surface is defended at the landed blob by reading the mechanism, (b) verifying
a predicted defect was actually fixed, and (c) surfacing the one standing smell
nobody owns, bounded. Reason-verified; no local build (COORDINATION section 12).

## Follow-on: b2 increment 2, RT-EFFECT-CONTINUATION-WRITE-NARROWING (2026-09-04)

Landed squash `3911d2861` (PR #3324, base `c720f87be`, candidate `12795bbca`
byte-identical across all 11 paths; Runtime QA and Architect approvals carried
by blob identity through the rebase `9376d724e` -> `12795bbca`). NO OBJECTION
(`evt_6ez28vxdw6vem`). It discharged increment 1's forward gate: a formerly
always-deferred transport-source response is promoted to a real synchronous
response-owner call, so the discriminated `ResourceBody{Ok/Err}` crosses the
inner-to-outer return through the existing owner-call Result-word return and
reaches the exit sink as exact `InvalidOffset`, not `PatternMatchFailure`.

1. **Faithfulness by reuse, not a new carrier, shifts the hunt.** The result
   rides the already-proven owner-call return; the feature only routes more
   responses onto it. The attack becomes "is the routing correct and does the
   existing return stay proven": the classification gate plus a
   native == interpreted exactly-once effect-parity pin (`assert_narrowed_alike`
   requires BufferAllocate and FsReadAt each performed `(1,1)`: zero is the
   old collapse, two is a replay). Kernel tree hash byte-identical and no new
   `RuntimeExpr::`/`Term::` variant, consistent with the ruling that a value
   crossing a function return via the existing owner-call ABI is build/sizing,
   not a new construct.
2. **A deleted planning guard whose guarantee migrated is co-evolution, not a
   weakening.** The phase-B split used to error when a Specialized owner had a
   transport-source caller; that state is now the feature. Before clearing a
   deleted guard, locate where its guarantee now lives: here the standing
   `validate_response_owner_call_coverage` (`lowering/units.rs`, byte-identical,
   still called in production). The deleted arm was only a test-error router for
   the removed force-specialize hook. See
   [[a-deleted-guard-can-be-the-only-enforcement-of-a-second-thing-nobody-named]].
3. **A fail-closed fixed-point guard on a derived-plane rebuild.** After
   phase-B owner assignment, `construction.rs` rebuilds `aggregate_ownership`
   and `checked_ih_environment_transports` and errors if the transport-source
   population changed (the selection would be circular). A wrong guard yields a
   planner error, never a miscompile, and the closed-derivation validator re-runs
   the whole split, so every axis is fail-closed.
4. **Prove mutation power transferred before scoring a deleted test as a loss.**
   The `force_specialize` AC-7 test retired with its hook, but the 8
   previously ignored capsule controls were un-ignored and re-keyed onto the
   owner edge: 7 owner-body mutations each assert `applications == 1` (reach)
   and `red.is_err()` (redden), and the 8th is a suppression control with a
   genuine green -> trap flip (exact `PatternMatchFailure` /
   `ResourceBodyResult`, BufferAllocate/FsReadAt/FsWriteAt unreached,
   interpreter still exit 0). Trap -> clean assertion flips that were true while
   the increment was unbuilt are false by design, not weakenings.
5. **Attack a "shared, effect-agnostic" carrier fresh at each new consumer.**
   A buffer-specific assumption baked into the shared carrier, join or exit
   routing (fixed discriminant layout, payload width, read/write frame shape)
   surfaces only when the next effect family (socket, network) lands on it.
