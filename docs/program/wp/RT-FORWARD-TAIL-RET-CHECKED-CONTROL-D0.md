# RT-FORWARD-TAIL-RET-CHECKED-CONTROL D0: sink and checked-control census

Measurement only. The probe began at approved predecessor
`47bcb744464009e5e856a8d86206fe0e25c5b431`. Its nine product-file
blobs are byte-identical on landed `origin/main`
`61661201b33278bbf74158a3ea5f1f4558d6fc9f`. The D0 branch was moved
onto that main before committing this record. No
production sink, planner, test expectation, or ignored status is changed.

## Method and scope

Disposable probes widened only the carried `ComputationalMatch` Ret sink's
marker-free filter at `lowering/core.rs`, leaving the other checked-answer
fallback gate untouched. Instrumentation logged each fully validated Tail
route at the end of `checked_ih_fresh_result_route`, the selected member's
actual sink request immediately before `resolve_composed_return_ret_sink`,
sink installation, the forward edge, and the pending source continuation at
`SourceCallOutcome::Complete`. The marker census walks the Ret body's planned
occurrence subtree and records checked subcontinuation frames, checked
recursive invocations, IH slots and IH invocations in that order. These are
compiler/emission observations, not runtime effect observations.

A plan instance is keyed by the fixture/test source, complete
`ContinuationCallIdentity`, active frame and Ret body, not by the number of
rederivations or generated functions that print it. A lowering request is
keyed to that same member. The one ambiguous short key in the 189-test run
was two distinct source-result origins (798 and 838) in one test that compiles
two programs; a focused full-identity request probe confirmed that **both**
were requested. The corpus is the 189 `rt_parity_native` tests, all ten
`rt_escape_second_resource_native` test bodies (six active, four separately
run ignored bodies, including three live-Nat variants), five executable
`px8*.rs` CLI targets (fourteen active, two separately run ignored bodies),
and 26 `ken-runtime --lib px8` tests. The three `ken-elaborator/tests/px8*.rs`
targets only elaborate or export checked IR and never enter the native planner;
they are not Tail-plan arrivals in this census.

| Population measured | formed Tail instances | requested | marker-bearing formed / requested |
|---|---:|---:|---:|
| `rt_parity_native`, 189/189 under probe | 87 | 78 | 0 / 0 |
| Five PX8 CLI targets, active and ignored | 7 | 7 | 0 / 0 |
| `ken-runtime --lib px8`, 26/26 | 0 | 0 | 0 / 0 |
| Escape file: six active, four ignored, Nat variants separate | 24 | 24 | 6 / 6 |
| Total plan instances in the named native corpus | 118 | 109 | 6 / 6 |

The nine parity instances formed but not requested are all marker-free. The
short-key report and its per-test list are in
`local/rt-forward-tail-ret-d0/census-summary.txt`; the full-identity
collision is resolved in `parity-cap41-request-identity.log`. The PX8
LIFO ignored row first reaches its pre-existing non-LIFO release assertion
and contributes one marker-free requested Tail. The other ignored PX8
resource-cost row refuses at an unrelated HostResult ObjectEmission before
any Tail is formed or requested; it contributes zero, not a presumed absence
from a successfully built program. The separately run ignored R2 escape row
forms and requests two marker-free Tails before its unchanged runtime `-1`.

The six marker-bearing instances come from the two target source shapes:

| Source | marker-bearing active frame → Ret body | frame / recursive / slot / call counts | requested |
|---|---|---|---|
| `:616` escaped buffer | 520 → 766 | 1 / 0 / 1 / 1; frame marker 756 | yes |
| live Nat, baseline | 716 → 1081 | 3 / 0 / 3 / 3 | yes |
| live Nat, Zero-extra | 887 → 1054; 716 → 1219 | 1 / 0 / 1 / 1; 3 / 0 / 3 / 3 | both |
| live Nat, Suc-extra | 887 → 1054; 716 → 1220 | 1 / 0 / 1 / 1; 3 / 0 / 3 / 3 | both |

The Zero- and Suc-extra programs have different checked source and Ret-body
identities; repeated coordinates inside separate programs are separate
instances. Each variant still has the interpreter's arm-sensitive read count
(2 baseline/Zero-extra, 3 Suc-extra). The uninstrumented landed base refuses
before native execution for all three: baseline first requests `(716,1081,0)`
with active `[(716,None)]`; the two extras first request `(887,1054,0)` with
active `[(716,None),(887,None)]`. All three name
`ComposedReturnForwardRetAuthority` caused by `ComposedReturnRetSink` with
`the active carried frame has no installed strict Ret sink`.

This is a census **of tested source programs and issued plans**, not a claim
that every possible Ken source or future marker topology was enumerated. It
is also not a baseline-execution success claim: widening was disposable and
unsafe; the unmodified compiler still refuses the marker-bearing requests.

## D0(a): the edge and the distinct Nat trap

Under the widened sink, `:616` frame 520/Ret body 766 has marker frame 756.
In generated function `funcid50`, the selected non-governed Tail takes
`SourceCallOutcome::Complete` at `lowering/source.rs`, with source
continuations still pending in this order:
`CheckedComputationalIHInvocationReturn → ConstructArgument → Terminal`.
`emit_composed_return_ret_kmatch_closeout` in `lowering/core.rs` then emits the
forward jump `block1392 → block60` (a second emission has
`block3140 → block1808`), where the target is the shared Ret-body block.
The `Complete` return skips the pending continuation handling instead of
resuming the nested checked control. A linked run confirms the consequence:
native operations are `[FsOpen, BufferAllocate, ResourceRelease,
ResourceRelease]`; interpreter operations add `FsReadAt` before the last
release. The probe does not establish that an Effect node was absent from
Ret body 766: its subtree has checked markers but no direct Effect node;
the skipped read is in the continuation's next computation.

For the reached Nat baseline, a disposable `#[track_caller]` per-guard code
first changed the runtime terminal from `-1` to `-251140`, identifying
`require_i64` at `lowering/joins.rs:1140`. A second, per-Match-origin code
changed it to `-301074`: generated function `funcid52`, carried Match origin
1074, expecting the Constructor boundary class (value 4). This locates the
failing runtime check in the widened linked artifact; it does **not** prove
that this particular forward edge dynamically preceded it, establish the
actual observed carrier class, or provide a valid conversion. The same widened
build emits a forward edge from `block7262` to the shared Ret block `block280` for
frame 716/body 1081, then the linked native run fails. None of these terminal
codes is an accepted user behavior or a proposed repair.

## D0(d): escaped FsHandle sibling

`escaped_resource_used_by_fanning_host_op_matches_interpreter` (`:584`)
has **no current first refusal** on the landed predecessor. With only a
disposable 256 MiB diagnostic thread for this ignored fixture, its original
exit/terminal/operation-sequence assertion passes on unmodified product
bytes; an additional disposable check confirms equal stdout and full
`EffectEvent` vectors (one test passed). It also passes under the widened-sink
probe. It forms and requests no Tail plan in either instrumented run. Its
`RT-NATIVE-TREE-MATCH D1: BoundaryCarrier` ignore reason is stale.
This sibling does **not** join this WP's AC-1/2/4 sink-repair population;
removing or rehoming its ignore requires a separate disposition. Its diagnostic
stack size is not a default-thread adequacy claim.

## D0(c): predicate proposal for Architect ruling

Replace the planner's `checked_ih_strict_ret_sink` and the carried lowerer's
independent `return_case` sink-install filter with **one planning-owned
assessment**, keyed by the exact active-frame occurrence. Its result is a
single descriptor with the logical Ret body and constructor-child binder,
plus an executable status: `Ready` for exact Ret+Vis topology with a proved
executable marker-free route; `PendingTopology` for a logical Ret input whose
emitted match topology is not yet supported; `PendingCheckedControl` for exact
topology whose marker-bearing continuation has not been proved executable;
or absent only when the old planner's logical Ret input does not exist. The
body profile belongs to the *planned source occurrence*, not a spelling or
an emission-side approximation.

Tail-plan formation may retain the logical Ret-input descriptor under all
three present statuses; it must not newly refuse a plan merely
because lowering never asks for it (nine marker-free non-requests already
show that formation and request differ). Lowering reads the same assessment,
installs a strict sink only for `Ready`, and keeps the existing sink refusal
for a requested `PendingCheckedControl`. A future ruled repair may promote a
marked descriptor to `Ready` only with a checked completion proof that every
nested frame and pending source continuation runs once before any forward
exit, preserving effects and the Nat scrutinee class. Topology alone or a
marker count is not that proof. The source-machine checked-answer fallback's
own marker gate is a separate consumer; do not remove it as a side effect of
unifying the sink predicates.

This is a proposal, **not an implemented predicate or authorization to widen**.
The Architect must rule the executable marked route before any production
change. The current two ignored rows remain ignored; no native parity,
full-workspace CI or QA approval is claimed.

## Reproduction and restoration

All builds/tests used scoped `scripts/ken-cargo`; key outputs and patch
provenance are under `local/rt-forward-tail-ret-d0/`:

- `buffer-widen-continuation.log` and `buffer-widen-effect-structure.log`:
  the exact forward edge, pending continuation and missing `FsReadAt`.
- `nat-widen-distinct-guards.log` and `nat-widen-distinct-match.log`:
  `-251140` and `-301074` terminal attribution; the latter returns 101.
- `parity-widen-census.log`: 189/189, four threads, 4145.97 s;
  `parity-cap41-request-identity.log`: both source-result identities requested.
- `px8-cli-widen.log`: 14 passed/2 ignored; `px8-runtime-widen.log`:
  26 passed; two separately run `px8ta-ignored-*.log` account for ignores.
- `escape-active-widen-census.log`: six passed/four ignored;
  `escape-r2-widen-census.log`: existing unrelated runtime `-1`.
- `nat-three-variants-widen-census.log` and
  `nat-three-variants-uninstrumented-base.log`: independently build all
  variants despite the first native failure, retaining interpreter
  discrimination. The disposable test harness caught each native failure
  solely to continue the census; it was restored afterward.
- `escaped-file-widen.log`, `escaped-file-uninstrumented-base.log` and
  `escaped-file-uninstrumented-full-observation.log`: ignored sibling passes
  with diagnostic stack provisioning; the last log asserts full vectors.
- `disposable-probe-a.patch`, `disposable-probe-final.patch`,
  `disposable-request-identity.patch`, and
  `disposable-three-variant-probe.patch` show all diagnostic edits.

Every modified source/test file was restored byte-identically against
`pre-probe.sha256` after the last run; `git diff --quiet` was true before
writing this D0 record. The branch was rebased from the exact predecessor
onto landed main only after verifying all nine predecessor product blobs
were identical. No probe is part of the committed tree.

Checks: 1 fired: planner Ret-input derivation and lowering installation were
traced to different predicates and disagree on marked frames; no mutual
agreement was credited. 2 fired: sink formation, selected D2 request and D3
`Complete` jump were separated; the pending checked continuation is the
missing path. 6 fired: all test populations and the two distinct sources in
one parity test were counted; both Nat arm variants were run separately.
8 fired: widening links but drops an effect or traps, whereas unmodified
product refuses; linking is not parity. 9 fired: marker formation, requests,
execution effects, and the ignored sibling's diagnostic stack were measured
on their separate axes.
