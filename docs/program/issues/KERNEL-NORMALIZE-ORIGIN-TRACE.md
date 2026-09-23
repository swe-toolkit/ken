---
id: KERNEL-NORMALIZE-ORIGIN-TRACE
title: "add an observational origin-tracking mode to ken-kernel's normalizer -- one reducer with a no-op observer for today's normalize and a collecting observer for a trace API -- so the bracket tree's child 1 can identify each compiler-authored bracket role's exact normalized occurrence without the shared dispatcher producing two host response routes; the traced Term must equal normalize's Term"
status: active
owner: kernel
size: L
gate: architect
tier: T1
depends_on: []
blocks: []
github: null
origin: "Operator authorized 2026-09-23 ('add an origin-tracking mode to the kernel's normalizer'), answering Steward fork evt_4xax1j2qjg62v. TCB growth, admitted by that ruling. Kernel ring seated to run it beside the L1 carrier (operator 2026-09-23 14:07Z: 'seat the spec and kernel rings as needed'). Design basis: Architect sketch and runtime-implementer feasibility sketch evt_5cwnemaw1zpkx (runtime-leader summary evt_1ndp4xwre8qx6). Steward-filed per COORDINATION section 2."
---

# The normalizer must say where each bracket role went

## Settled inputs

- The held bracket tree's child 1
  (`wp/RT-BRACKET-PRODUCER-AUTHENTICITY` `4b4c8565c`) wraps each bracket
  role in a compiler marker. The kernel's first `normalize` copies the
  shared dispatcher through them. The unignored native row
  `px7f_resource_native::linked_public_escape_is_exact_closed`, green on
  `main`, then goes red on the correct "two host response cases claim one
  operation constructor" guard. Without the wrappers there is one route.
- No-kernel routes are refuted (`evt_1ndp4xwre8qx6` (c)). The markers are
  already undeclared and opaque to delta, yet the duplicate still appears.
  An opaque `bind` gives no lowering of all five roles.
- Measured call graph: `conv.rs::normalize` -> `whnf` / `whnf_progress`.
  Beta goes through `subst0` -> `subst.rs::subst_var` / `shift`, delta
  through `unfold_const` / `subst_levels`, and iota through
  `inductive.rs::iota_reduct` (method choice and recursive IH generation and
  pruning). Transparent recursive `bind` unfolds through the same machinery.
- Estimate, **unmeasured**: 500-1,100 kernel lines plus 200-500 test lines,
  possibly more for recursive IH provenance.

## Deliverables

- **D1, design to the Architect before code.** It must cover:
  - **One reducer.** Today's `normalize` becomes the no-op-observer
    instance. A new trace API is the collecting instance. Neither observer
    can select branches or change a reduct.
  - **Role identity** = a unique compiler-authored call occurrence plus the
    template role. A shared template marker id is not enough.
  - **Reporting.** Beta and let report zero, one or many descendants. Delta
    instantiates the template under its call. Iota drops untaken branches
    and carries taken arguments and recursive IHs.
  - **Failing closed.** Any required identity with zero or several final
    descendants fails closed, and so does any collision.
  - **Measurement first.** Measure one-to-one transport on the `px7f`
    fixture before the full build.
- **D2, after approval.** Build the kernel mode. Add a compiler-boundary
  gate that refuses before checked package emission unless
  `traced.term == normalize(env, ctx, unmarked_input)` (full structural
  equality). The child-1 repair that consumes the trace is the bracket
  tree's own work, not this node's.

## Acceptance criteria

- **AC-1 (identity).** Differential tests over `k2c_conversion.rs`,
  `k1p5_wstyle.rs`, `nested_inductives_*.rs`,
  `recursive_head_totality_d0.rs`, generated scoped beta/delta/iota/let
  inputs, and the `px7f` fixture. Each asserts `Term` equality with
  `normalize` and the expected trace multiplicities.
- **AC-2 (falsifiers).** Mutating only the traced reduct turns the equality
  gate red. Dropping, copying or colliding a role path turns the one-to-one
  gate red.
- **AC-3 (trust root).** The kernel diff gets the Architect's review plus
  one reviewer independent of the author's context. The handback states
  plainly that identity rests on shared construction plus finite tests,
  not a proof. Every existing kernel suite stays green.
- **AC-4.** Targeted builds only, through `scripts/ken-cargo -p ken-kernel`
  and the named tests. No-regression means green in CI.

## Stop conditions

- If D1's measurement shows exact one-to-one transport cannot be specified,
  or the trace would need an observer that changes reduction, STOP and
  return to the Steward. Child 1 then takes its `§5` STOP.
- **Not authorized:** a second normalizer, guard suppression, weakening the
  two-routes guard, or any change to `normalize`'s output.
- **Held work:** never move `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the
  child-2 checkpoint (`21c039918` / `65abba517`).
