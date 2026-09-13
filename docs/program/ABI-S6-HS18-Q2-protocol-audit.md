# HS18 Q2: protocol audit and semantic residuals

## Disposition and evidence

This supplements the [path-proof ruling](ABI-S6-HS18-Q2-path-proof.md).
The reviewed checkpoint is `01d2ccb117151c468cb8a7f06f6e3a5fd8667e33`,
against Q1 `30d35f6256421a0b86cf772c739794365b3402be`: nine paths,
+2540/-434. The independently matched patch SHA-256 is
`32894c6afd82022937dbe18f0d69d248ee7008015a5e60afd2d769433412b210`.
Q1 `source.rs` is unchanged. This is not a candidate approval.

**Repair the finished protocol verifier before treating Mapping's object as
certified. Preserve the px8f refusal. Localize, rather than guess, the remaining
source-result mismatch.** These are follow-ups to inventory entry18 and the
existing Q2 acceptance boundary, not authority to widen the implementation.

The Architect independently ran two private-verifier probes against unchanged
production verification functions. Every input passed Cranelift verification;
each exact control was accepted. Four malformed variants were also accepted:

- a helper output load with an extra incoming path bypassing its status guard;
- a unit Trap load reached while bypassing the unit-status guard;
- a unit Result load reached while bypassing the Trap guard;
- an actual call header pointing to frame B while the obligation and reads
  name the distinct frame A.

The rejection assertions failed because those variants were accepted. These
are verifier-boundary counterexamples, not claims that the unchanged Mapping
fixture executes those malformed paths. The logs and reproducible probe diff
are retained in the Architect's `local/hs18-q2-return-review/` as
`own-query-guard-bypass.log`, `own-call-protocol-bypasses.log`, and
`guard-probes.diff`. Both named tests ran once; neither was a zero-test result.
All scratch production files were restored and the worktrees removed.

The shared defect is **a locator or layout position being treated as an
all-path execution/memory fact**. `units.rs::unique_status_successor` finds an
appropriate branch; `verify_carrier_queries` finds a load in its named success
block. Neither excludes an additional predecessor bypassing that branch.
`verify_call_result_obligation` similarly compares successor/load block
identities. It does not trace the actual call header's frame pointer to the
recorded payload. Adding four fixture-specific checks would not close this
predicate.

Here is the checked helper counterexample. `fn0` denotes the nominated helper;
its protocol permits nonzero status without initializing the output. The
unchanged query verifier accepts this valid CLIF despite the bypass:

```clif
function u0:0(i64, i64, i64) -> i64 system_v {
    ss0 = explicit_slot 8, align = 8
    sig0 = (i64, i64, i64) -> i64 system_v
    fn0 = colocated u0:17 sig0
block0(v0: i64, v1: i64, v2: i64):
    v3 = stack_addr.i64 ss0
    v4 = call fn0(v0, v1, v3)
    v5 = icmp_imm eq v4, 0
    brif v2, block2, block1
block1:
    brif.i8 v5, block2, block3
block3:
    v6 = iconst.i64 -1
    return v6
block2:
    v7 = stack_load.i64 ss0
    return v7
}
```

Replacing the first `brif` with `jump block1` is the accepted exact control.
The other probe preserves the exact status/Trap comparisons and their named
successor blocks while independently adding a bypass into each successor. Its
foreign-frame variant changes the actual header's payload pointer, not the
obligation's payload locator. All three were accepted separately.

## Bounded proof repair

Runtime may repair the existing compiler-private proof machinery. No source
semantics or runtime representation change is authorized by this section.

1. Bind every fact to its actual finalized function, producer instruction,
   word, memory location and consumer instruction. Replay the decoded callee,
   actual call arguments and header construction. Establish that the header
   passed to this call points to the same payload whose exact Trap and Result
   offsets are read, using the existing UnitBundle layout. An emitter-recorded
   payload is a locator, not proof that the callee received it.
2. Establish successful guard traversal on **every** path reaching each
   dependent observation: call status before Trap, Trap before Result, helper
   status before output. The property is edge/path domination, not a unique
   predecessor count or block-layout shape. Check producer-to-use paths as
   well as entry-to-use paths: a prior successful loop iteration must not
   justify an unchecked later call or query. Unknown paths retain no fact.
3. Establish reaching initialization and no intervening clobber by CFG memory
   flow, not maximum layout position. Follow actual addresses through the
   existing header/stack representation. Include relevant raw `Store`,
   stack stores, aliases, calls and loop re-execution. Unsupported address or
   effect provenance refuses; an unrelated textual initializer cannot supply
   the fact. Preserve legitimate status-zero/nonzero-Trap exits with no Result.
4. Apply that same protocol floor to call seeds, audited queries, response
   owners and final publications. Audit the local constructor and join seed
   issuers as well: a predecessor list cannot bypass finalized all-input
   replay. Unknown control-flow terminators must refuse or have all actual
   successors enumerated; `successor_edges` must not silently treat them as
   exits. Retain the ruling's iterative dataflow/SCC requirement, not recursive
   SSA expansion or a bare success Boolean in lieu of its dependencies.
5. A cut may use only facts established by that floor. Keep its exact
   function/branch/ordinal/target and dependency provenance. A pending call
   cannot justify a cut that is then used to discharge that same call. Do not
   globally remove an obligation merely because another call to its target
   lies on a certified dead path. Exclusion is call-site-specific and requires
   its own independently grounded path proof.

This is a structural verification repair, not a new generic symbolic executor,
SMT dependency, ABI, schema, frame, owner key, runtime field, tag, route,
function class, allocation site, decoder, stack provision or enlarged bound.
It does not authorize deleting a program edge during code generation.

Acceptance must include each of the four counterexamples above with fixed
verification and a matching exact positive. Add a backedge that makes an old
successful guard insufficient for a later producer execution; a reaching
initializer bypass; raw-address/header/output alias clobbers; and an extra
successor through each supported control-flow form. Demonstrate refusal at the
corresponding protocol proof, not an unrelated earlier failure. A semantically
equivalent block reordering/subdivision must not fail merely for its layout.
Then rerun the existing owner and caller/query/cut grids on the repaired
checkpoint. A grid pass does not by itself discharge untested protocol roles.

## The px8f continuation mismatch

Own observer-only execution independently reproduced the pre-object refusal
on the unchanged fixture and its existing 256 MiB Builder stack. The local
producer is not an alternative Ret spelling:

- function `u0:55`, `funcid55`, specialization3;
- origin667, aggregate occurrence94;
- `ctor:px8f_write_all_native::ITree::Vis`, identity3380/38;
- allocated arity2, actual tag `0x0d3400000027`, published word `v77`.

The finished body constructs a nine-field captured environment, then constructs
Vis with its operation input and that environment. Its successful terminal
stores `v77` to Result. The actual tag store agrees with the independently
observed constructor spelling and identity.

The six demands are issued in `core.rs::validate_checked_ih_detached_result_shape`
for caller `u3:60`: `(inst11294,v7798)`, `(inst17203,v11920)`,
`(inst23283,v16150)`, `(inst29225,v20292)`, `(inst35130,v24412)`, and
`(inst38145,v26516)`. The consumer relation itself records actual3380/38 versus
demanded4442/38. Its source specialization is3, destination specialization2,
source worker body859, destination body1605, construct874, continuation661,
closure seat869. It has no immediate consumers and two selected-case exits:
699/check3/body706 to demanded body874, then661/check2/body668 to body692.

A diagnostic use of currently available finished seeds `v451` and `v13703`
produced cut `(inst644, ordinal1, block171)`. Only call `inst23283` of those six
became unreachable under that cut. The remaining five are still possible in
that limited CFG calculation. This is not a proof they execute, nor an
exhaustive proof that no further sound cut exists. It forbids the proposed
shortcut of discarding all six because one known cut exists. That diagnostic
used the current verifier and is not a final certificate after the audit above.

Keep both identities truthful. Before any source-return repair, return a
bounded ledger distinguishing: the raw specialization's result, any existing
response-owner retarget, the selected source exits actually executed, the
residual still owed, and the final context's Result. For each still-possible
call, locate the first stage where the 3380-to-4442 demand is unsatisfied. If a
further edge is impossible, supply its noncircular proof and exact call-site
exclusion instead. Do not turn a demand, a tag test or a source-stage receipt
into an assertion that the raw Vis was Ret.

In particular, do not replay completed699 or blindly apply both recorded exits;
preserve the distinct selected exit661/check2 and residual483 obligations and
the disjoint Active1097/check6 case. Fixing the verifier does not authorize a
new response owner or a different semantic call target.

## The Mapping semantic residual

The reviewed native observation reports both Mapping views and both releases,
then `PatternMatchFailure` at `ResourceBodyResult` before `FsReadFile`, with
native/interpreter exits `(1,0)`. The observation log hash independently matches
`66b522d58e1c91fc8f310d73a327d20a830af02b0b2326f4046989eb4e4e83da`.
The Architect independently reproduced Mapping object emission, not that native
run. Object emission and an outer Ret contract do not establish correct payload
or source-stage execution.

Authorize observer-only localization of the first wrong value. Bind the exact
failing match occurrence, executing unit/source owner, expected constructor
identities and binder positions, and native scrutinee word/tag/arity/arena.
Trace its reaching store, field projection, frame slot and call argument back
to the producer. Compare the interpreter at the corresponding source occurrence,
not just the final exit code. Distinguish a wrong family, an extra/missing
constructor layer, a wrong captured value, and duplicated or omitted source
consumption by those observations. Do not assume which explanation is true.

Use existing helper interfaces or debugger observations; introduce no production
decoder or diagnostic runtime route. Do not repair this by peeling a plausible
Ret/Result wrapper, changing the source fixture, suppressing the failing match,
or replaying a source exit without its exact residual authority.

## Return and preservation

Return the clean checkpoint, verifier repair evidence, and both bounded
semantic ledgers through Runtime leader. If they expose a source-routing repair
outside this envelope, stop for a new determination; do not silently broaden
this proof repair. No QA candidate or merge Decision is authorized yet.

Preserve Q1 source control and its unchanged-stack fixture, px8f's existing
256 MiB stack, byte observations, all ten effects and order, Mapping COW and
release semantics, complete identity domains, Tail `None`, and all HS17 receipt
controls. A Result certificate and a source-stage receipt remain distinct with
no conversion either way. No claim of repaired native parity, complete Q2
acceptance, or green publication CI is made here.
