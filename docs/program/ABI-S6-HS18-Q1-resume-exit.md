# HS18 Q1: pop the source-machine frame before outer resume

Architect, 2026-09-13. This follows the diagnostic-first HS18 determination.
It is a bounded repair ruling, not candidate approval or HS18 completion.

The subsequent [Q2 path-proof ruling](ABI-S6-HS18-Q2-path-proof.md) resolves
the binding-only hold recorded below and governs bounded Q2 authoring. The Q1
repair and its preservation constraints are unchanged. Historical Q2 questions
below are not a renewed hold or permission to relabel the Vis.

## Decision

Authorize the compiler-private `ResumeOuter` exit below. Return from
`lower_source_machine_with_continuation_inner` BEFORE calling
`resume_active_continuation`, then perform that same call in its existing thin
wrapper. Do not install a return-context arena, change identity representation,
box unrelated structures, or rewrite the producer/source-machine drivers.

This closes the measured retention edge, not every possible Rust recursion.
It does not claim constant stack use for arbitrary programs. A different
remaining overflow requires fresh localization, not speculative expansion.

## Grounding

Runtime's preserved WIP is HEAD
`37390dfcf89c3930751b6b9eea521ef650b70b34` plus the 24-path diff hashing
`c3e86a7688bb5ae438cfaa9a39a8858376bb0df11cb7fd5b093c2beefe16cb5c`.
Architect independently matched the live diff and all 15 hashes listed in
Runtime's localization summary.

The ordinary failing row is `row4-depth-3/suppressed`. Declaration completed:
four predeclared units (funcid41–44), one specialization (funcid45), and no
context, response owner or fusion target. Lowering entered root funcid41.
The last observed continuation was one `LetBody` over `ResumeOuter`, depth one,
with zero active recursive invocations. LLDB shows four retained large
source-machine inner frames across source/resume/composed recursion, not a deep
return-context equality/clone/drop chain. Individual retained inner frames are
about 279–285 KiB; the crash-to-fixture span is 2,081,112 bytes. The faulting
entry to the unwind replacement is where stack runs out, not evidence for an
arena repair.

Architect then independently tested ONLY the three-site change below in an
isolated copy of the exact WIP, with no observers, fixture changes or stack
provision. The unchanged targeted fixture passed 1/1. Restoring the exact WIP
source made the same command run one test and abort with stack overflow,
SIGABRT, exit 101. The probe source was restored and the scratch removed.
This is causal evidence for the named repair, not a full-runtime/native gate.

Both runs used the same existing unoptimized test profile and:

```sh
env -u RUST_MIN_STACK CARGO_TARGET_DIR=/workspaces/ken/.worktrees/architect/target scripts/ken-cargo test -p ken-runtime --lib required_consumer_route_manufactures_the_depth_two_plus_closure_crossing -- --nocapture
```

No `--exact` is used with the unqualified module test name. Runtime's earlier
zero-test invocation was explicitly excluded from evidence.

## Exact checked change

All three edits are in `crates/ken-runtime/src/cranelift_backend/lowering/source.rs`.
Add this variant to the existing private `SourceMachineExit<'a>`:

```rust
ResumeOuter {
    value: LoweringOperand,
    active: ActiveContinuationFrame<'a>,
},
```

In `lower_source_machine_with_continuation`, add this arm beside `Complete`:

```rust
Ok(SourceMachineExit::ResumeOuter { value, active }) => {
    self.resume_active_continuation(builder, value, active)
}
```

In the inner machine's existing `Terminal(ResumeOuter { .. })` arm, replace ONLY
its final direct resume call and `.map(SourceMachineExit::Complete)` by:

```rust
return Ok(SourceMachineExit::ResumeOuter {
    value,
    active: *active,
});
```

Keep the existing expected-cursor check, root-authority restoration and Trap
return BEFORE this exit. Keep the wrapper's `source_control_root` and
`live_source_continuations` scope around the dispatched call, with restoration
on both success and error. Do not pop that logical scope when popping the large
Rust frame. `ActiveContinuationFrame` is already `Copy`; preserve every field
and borrowed parent, not just its cursor. No new clone or identity lookup is
needed. Keep `resume_active_continuation` unchanged, including its
RecursiveBackedge no-suffix guard, cursor mint and exact successor linkage.

This is a compiler dispatch exit, NOT a new runtime tag, answer role, semantic
route, owner key or emitted function class. The existing `#[inline(never)]`
inner boundary must remain effective. No ABI/schema/frame/runtime allocation,
stack setting, fixture bound, or refusal changes are authorized.

## Current population and Q2 boundary

Architect independently regenerated the current forwarding census from the
provided CLIF and the same-run `v26 = Param(block7,0)` diagnostic:

- px8f `u3:60`: 29 values, 28 incoming edges, seven leaves. The six distinct
  call-result leaves bind to `fn56 = u0:55`. Runtime's typed UnitBundle marker
  binds funcid55 to specialization3, producer-owner14, emission-owner
  specialization2, consumer-owner15, occurrence None, construct874,
  result1605, alternative1, continuation661, recursive position/set 1/{1},
  ordinary parameters10, ABI parameters10/captures6/frame160/align8.
- Mapping `u3:52`: five values, four edges, THREE leaves: `v123`, `v219`,
  `v274`. It emits no call to u0:55. This is not the px8f seven-leaf graph.

These are named, observed populations, not a universal finite-context theorem.
No new population or owner quotient is needed for the selected Q1 change.
Both CLIF dumps precede finished Result closure: their reached block7 is not
yet displayed as a finished block. They cannot themselves be consumed as
finished-unit certificates.

Q2 production remains gated. In current Mapping, ungoverned `v219` comes from
`ss28`, followed by `fn17(v7,v219,0x0a14_0000_0025)` and two `fn19` writes;
it is not a missing u0:55 call-result seed. The canonical identity encoding
would decode that word to start2580/len36, whereas the current demanded contract
is start3606/len36. This is a concrete potential identity conflict, not permission
to annotate it 3606. Bind the actual helper roles, allocation/tag/field writes,
source constructor and demanded contract in THIS artifact before deciding its
repair. Observe first; do not change Q2 production or weaken the refusal.

The earlier modular finished-Result design remains conditional on those actual
bindings. Q1 recovery neither supplies a terminal certificate nor mints an HS17
source-stage receipt. Preserve the existing no-conversion rule in both directions.

## Acceptance and routing

Runtime owns the bounded source change after Steward folds this ruling into the
live frame. Require the unchanged ordinary fixture's real assertions to pass,
not merely absence of an abort. Preserve cursor/root/checked-prefix refusals and
RecursiveBackedge no-suffix behavior. Preserve px8f parity, bytes, ten ordered
effects and its EXISTING 256 MiB Builder stack. Recheck Mapping separately:
a continued exact pre-object refusal is honest Q1 preservation, not Q2 closure.
No old library-control red or aborted library run is discharged by this probe.

Keep the source delta separate from Q2. Return its exact patch and targeted
results, plus the bounded current Mapping binding question. Broader gates are
CI-only. Source edits, stack changes or identity substitutions outside this
ruling require a new determination. No operator gate is added.

Evidence: Runtime summary SHA-256
`3adfdb21581727bbe79c1507e2aa4e8c99baadbcc2805d2dfb79baf8954c3c8e`;
Architect's `local/hs18-q1-review/` retains the exact delta, green and restored-red
logs, current WIP snapshot and independently generated census. Reviewed Runtime
executions and Architect's own two executions are distinct evidence.
