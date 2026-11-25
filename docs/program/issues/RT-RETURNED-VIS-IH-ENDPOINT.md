---
id: RT-RETURNED-VIS-IH-ENDPOINT
title: "The static planner's returned-Vis walker resolves a Call's callee only through emitted call edges. A computational induction hypothesis is evaluated inside its owning unit, so no edge names it, and PX11's exchange refuses with N0002 'returned-Vis generated Call at StaticOriginId(33) has an unknown endpoint' at response owner 9 in drain_full. Resolve an IH callee by checked binder provenance and walk every case of the eliminator that bound it; keep the refusal for every other edgeless callee"
status: merged
owner: runtime
size: S
tier: T1
gate: architect
depends_on: []
blocks: [PX11-NATIVE-STREAM-SOCKET-PARITY]
github: null
origin: "Architect ruling evt_427vemb7ks8fd on PX11 Inc2's probe P-RV (runtime-implementer evt_4nhtc1bgvysn4); Inc2 inventory entry 9. The ruling scopes it to its own node, as for the parent-edge and positional nodes. L1 Linux ABI program, Steward order."
---

# Returned-Vis walks through an induction hypothesis

## Objective

Returned-Vis settlement derives the returned set of a response owner whose
walk reaches a complete computational IH application. PX11's exchange gets
past response owner 9.

## Settled inputs (main `fc7d16a59`; Architect `evt_427vemb7ks8fd`)

- **The witness.** P-RV's Call 33 is under
  `CheckedComputationalIHInvocation` 34. Its callee, origin 32, is `Var(5)`,
  the sole IH binder of the `ITree::Vis` case of ComputationalMatch 25 in
  `drain_full`'s normalized body. None of the 47 emitted edges keys on 32 or
  33. Of 12 settled owners, owner 9 is the only Underived one.
- **The walker.** `returned_vis_in_context` (`static_transition/
  returned_vis.rs:459`). It treats the IH invocation as a transparent wrapper
  (`:480`) and refuses an edgeless Call (`:530`). The production caller is
  at `:630`, the existing tests at `:866` and `:871`.
- **The resolver.** `build_checked_binder_provenance` (`continuations.rs:
  6002`) answers an IH binder with `CheckedBinderProvenance::
  InductionHypothesis` (`:5593`). The IH lowers in its owning unit
  (`lowering/mod.rs:12956`, `enter_checked_computational_ih_invocation`).
- **The code.** The ruling gives the new arm, the fourth parameter and the
  caller change. Build it as written and reconcile only the three spots it
  names.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

`returned_vis.rs` and its tests. No change to the resolver, to other edge
consumers, to `returning_units`, or to the exchange fixture or profile.

## Acceptance

The ruling's rows, built as written:
- **AC-1.** A unit row in which the base-case Vis is reachable only through
  the IH returns that Vis origin. m1 (no new arm) reddens it with "unknown
  endpoint", and m2 (cases filtered by the scrutinee) reddens it on the
  missing Vis.
- **AC-2.** The same fixture with an ordinary-binder callee still refuses
  with "has an unknown endpoint". m3 (every invocation callee treated as an
  IH) reddens it.
- **AC-3.** The existing `returned_vis` tests and the targeted
  `static_transition` aggregates rows pass unchanged.
- **AC-4 (report only).** Every consumer that resolves a callee only through
  `emittable_call_edges`, starting with `responses.rs` (:2806, :3025,
  :6688), `closure.rs:1683`, `construction.rs:2628` and `units.rs`, with
  whether an IH call can reach it on the exchange.
- **AC-5 (measurement, not committed).** The exchange on Inc2 replayed onto
  this node, the positional node and `fc7d16a59`, reported by the ruling's
  arms R-a, R-limb, R-same, R-sib and R-other.
- **AC-6.** Full-workspace CI is green on the exact candidate.

## Stop conditions

R-same, R-sib and R-other go to the Architect, and so does an AC-4 consumer
that an IH call can reach. Do not relax the unknown-endpoint refusal for
non-IH callees.

Checks: 4 fired. I read the walker, its wrapper arm, the refusal, the caller
and both test calls, and the resolver and its IH variant, on `fc7d16a59`.
The IH entry the ruling cites at `core.rs:4067` is at `lowering/mod.rs:12956`
on main; the frame cites the latter. 8 fired. AC-1's Vis is reachable only
through the IH, so both the old wrapper and a filtered walk fail it. 2 fired.
AC-2 keeps the refusal for every edgeless non-IH callee.
