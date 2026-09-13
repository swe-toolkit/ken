# ABI-S6 HS18 determination

Architect, 2026-09-13. This is a component-design ruling, not a release vote.
The Steward owns the diagnostic-first recut and release to the runtime ring.

## Current Q2 follow-up

The [Q2 path-proof ruling](ABI-S6-HS18-Q2-path-proof.md) supersedes the
binding-only hold for bounded authoring. Keep the whole-function Ret contract;
use independently certified infeasible edges, not Vis relabeling or contract
relocation. Finalized-body staging, real call/Result proofs and a small
well-founded tag-observation analysis are part of that scope. Current declared
metadata is not a finished certificate. All no-conversion and refusal conditions
remain; object acceptance still requires the complete proof graph.

## Current Q1 follow-up

The diagnostic prerequisite has returned. The
[Q1 resume-exit ruling](ABI-S6-HS18-Q1-resume-exit.md) supersedes the initial
Q1 observation hold below: it authorizes only popping the existing large source
inner frame before its outer-resume call. Architect independently checked that
change green and the restored baseline stack-overflow red. It does not authorize
an arena or a general identity representation change. Mapping's three-leaf
population remains distinct from px8f's seven leaves. Q2 now follows the
successor ruling above; all Q2 authority conditions below still bind.

## Initial decision

Do **not** release the proposed rational return-context arena as the diagnosed
stack-overflow repair. Its finite-identity premise and its causal relevance to
this failure have not been established. Release a bounded observation pass to
locate the failure on the unchanged stack and close the actual static
population. Return that evidence for the Q1 representation decision.

Adopt **modular finished-Result verification and predecessor-indexed proof
composition** for Q2, within the existing emitted-function population. First
bind the current failing caller and its actual callee definitions. A missing
certificate is not evidence that the required certificate exists. Production
implementation may proceed when that binding shows the intended relation; a
counterexample or missing definition returns a bounded seam question instead.

Both decisions remain compiler-internal. There is no operator gate. Neither
allows a new ABI, schema, frame, owner key, runtime field, tag, route, function
class, runtime allocation site, generic decoder, stack provision, or enlarged
bound.
There is no weakening of the pre-object refusal.

## Grounding and corrections

The reviewed WIP is over `37390dfcf89c3930751b6b9eea521ef650b70b34`.
Its preserved and live diffs both hash:

```text
c3e86a7688bb5ae438cfaa9a39a8858376bb0df11cb7fd5b093c2beefe16cb5c
```

It is 24 changed paths, +9179/-5479, not a clean candidate. Whitespace checks
pass. The final summary hashes
`3b8b4745f7831d7cfa6bdd41d199e4db4be6364a424407e1c3f65bba44b83851`.
The retained stack, Mapping-refusal, and px8f logs match their reported hashes.
The latter reports 7/7, including HS17 receipt controls and native/interpreter
parity; these are reviewed runtime executions, not new Architect runs.

Research's initial Q1 recommendation assumed a finite specialization/call
population. Research withdrew the stronger causal claim after the Architect
checked the interner:

- `ContinuationSpecializationKey` contains the exact generated emission owner.
  A descent carries the newly interned target as its next enclosing
  specialization. Successively fresh targets therefore produce distinct owner
  keys; replacing a recursive value with an integer cannot equate them.
- The call-site sequence advances as calls are installed. It is not a finite
  source-occurrence proof merely because it is an integer.
- The existing fixed-point bound guarantees bounded refusal if execution is
  stack-safe. It does not prove successful closure before that refusal.
- Diagnostic precursor `hs17-stackdiag2.log` reports shallow, terminating
  planner frontiers before the overflow: 4 or 5 steps, one unit, context depth
  0 or 1. Its hash is
  `abd459d2954c84b1c893cda2c4181e7d17a7079024e1ad6a7a84494b4c05858d`.
  It is not intrinsically bound to the final WIP. The final retained overflow
  log proves an abort, not whether clone, equality, validation, lowering, or
  drop overflowed. The reverted Arc experiment proves only its insufficiency.

Recursive owned contexts are a real representation hazard. They are not yet the
localized cause of this fixture's failure. Do not preserve that causal claim in
the recut or its acceptance criteria.

The Architect independently expanded the preserved `u3:60` CLIF from
`/tmp/hs16-context-1605.clif`, hash
`829ecdfb5d03329bb8dec681721f7aee37e12d64363d41e5ecd9f064699e92c2`.
Entry-reachable Jump/Brif block-argument expansion from `v26` visits 29 values
and 28 incoming edges, ending at exactly seven leaves:

| Leaf | Origin in this preserved function |
|---|---|
| `v3653` | Local allocation, constructor-header initialization and field write |
| `v7798` | Result load from `ss682+128` after call `fn56` |
| `v11913` | Result load from `ss1013+128` after call `fn56` |
| `v16136` | Result load from `ss1361+128` after call `fn56` |
| `v20271` | Result load from `ss1694+128` after call `fn56` |
| `v24384` | Result load from `ss2025+128` after call `fn56` |
| `v26481` | Result load from `ss2194+128` after call `fn56` |

The six calls decode to module external target `u0:55`. They are six distinct
call/load/predecessor occurrences, not one fact about a callee name. Each load
follows status and Trap checks. The caller's final Result slot is `frame+112`;
the callee Result and Trap offsets are respectively 128 and 144. Never carry an
offset across frames merely because both slots are called Result.

This ledger belongs to the preserved CLIF, not to an independently finalized
HS18 caller. Equal `v26`/`block7` spelling across runs is not such a binding.
The recut must regenerate the current census and join the external module
`FuncId` to its actual typed unit and definition. A displayed `u0:55` is not
proof of a source-function class or of a particular context ID.

## Q1: bounded observation, not an owner quotient

Use only targeted runs through `scripts/ken-cargo`, with the existing fixture,
flags and stack unchanged. Preserve the ordinary failure. Record exact source,
observer diff, command/environment and artifact hashes. Observer runs are not
normal-path acceptance runs.

Locate the last completed phase and active call chain across planning, context
validation/projection, lowering, finished verification and destruction. A
permitted debugger or minimal test-only phase markers may provide that evidence;
record their effect on the observation. Do not add recursive Debug printing or
recursive depth measurement. Measure the context chain iteratively and report
admitted discoveries, exact units, call occurrences, maximum retained context
depth and unresolved frontier. Do not rerun the whole library merely to reproduce
this one abort.

The population obligation is explicit: pair every newly installed call with its
actual finite emission occurrence; show each owner transition either reuses a
full-key-equal unit or belongs to a finite source-derived exact population.
Distinct generated IDs cannot be merged by body, constructor, local head, or
truncated caller history. Bound exhaustion remains refusal.

Return the localized cause and population ledger before changing Q1 production
representation. If the failure is a large lowering frame or another recursion,
repair that measured cause rather than installing an unrelated arena. The
ordinary-stack fixture must recover its legitimate disposition without stack,
fixture-depth, bound, or semantic-guard changes.

If later evidence supports a return-context graph, the admissible family is a
plan-owned arena of nonrecursive node handles, finite local source segments and
exact labelled worker-return edges. Handles are locators, not owner identities
or authority. Intern before edge expansion; preserve source-child roles,
consumer, specialization, selected-call and transport identities. Equal local
heads with different caller edges retain different equations or refuse. Caller
suffixes cannot hide an unfolded call string. Equality, validation, queries,
cloning and destruction cannot recursively unfold the graph.

This family is conditional, not authorized by this determination as the repair.
A query needing dynamic activation identity or an unbounded ordered residual is
unavailable under this envelope. It must refuse, not manufacture a history key
or silently skip a cycle.

## Q2: private proof surface

Separate these private roles in Rust; names are illustrative, distinctions bind:

- `DeclaredResultContract`: an obligation for an existing declared unit and
  exact `ConstructorIdentity`, not a certificate. The current
  `DeclaredUnitCall.result_contract` must not be described or consumed as
  finished proof merely because its value is `Some`.
- `FinishedUnitResultContract`: produced only by an independent verifier of the
  actual finalized function. It binds module function identity, the existing
  typed unit identity, frame descriptor, Result/Trap locations, successful
  terminals and any call-summary dependencies.
- `CallResultObligation` / `CallResultSeed`: respectively a pending reference
  and a discharged proof at one exact call, success edge, Result load and SSA
  word. Distinct types prevent a pending reference from becoming authority.
- `ResultForwardingProof`: a function-scoped proof graph with explicit
  predecessor edges, identity-preserving forwarding, and grounding sources.
  It is not just the former `(valid, grounded)` pair.

Attach these to the existing declaration/emission records and finalization
transaction. Do not introduce an independent owner-authority registry or alias
existing unit-ID domains. Resolve identities through `UnitBundle` and decode the
actual direct callee from finished CLIF. Local `Inst`, `Value` and stack-slot
numbers are scoped by their function, never treated as module-global names.

### Successful return means both checks succeeded

The postcondition is:

```text
returned status == 0 AND that call frame's Trap == 0
  implies its Result is initialized with the exact certified identity
```

Status zero alone is not success. `emit_current_trap`'s UnitFrame arm writes a
nonzero Trap and returns zero; the caller Trap branch also propagates Trap and
returns zero without reading Result. Preserve both cases. Failure and Trap
terminals must retain the existing propagation protocol and never manufacture
Result authority.

The finished verifier must check every reachable successful terminal, the actual
Result initialization/store and reaching word, its frame/offset, and the status
and Trap conditions. Reuse independently justified constructor-production and
carrier facts; an expected tag, successful status, declared target, or generic
Cranelift verifier success is not a constructor-production proof. A target-wide
certificate requires a uniform postcondition. Mixed identities require already
justified call-specific proofs or refusal, never an annotation chosen to suit
the caller's demand.

### Bind at the call and compose universally

A discharged call seed requires the exact direct call target and existing frame
descriptor, the actual returned status, both success checks, and the exact Result
load from that frame. Check control-flow dominance and memory reaching facts;
a same-slot spelling, an earlier call, or an intervening clobber cannot serve as
the load's producer. Missing or unsupported provenance refuses.

For a block argument retain every reachable incoming edge as
`(predecessor instruction, destination ordinal, target block, argument index,
incoming word)`. Destination ordinal distinguishes two branch destinations that
happen to target the same block. Every incoming edge must prove the same full
identity. `Union` likewise requires both inputs. Missing arguments, unsupported
edges and unknown executable producers reject. Excluding an edge needs finished
control-flow evidence, not a source expectation.

Use iterative finite dataflow/SCC processing, not recursive SSA unfolding.
Internal SCC edges must be identity-preserving forwarding; every external input
must be proved for the same identity and a real external/direct seed must ground
the component. A pure cycle and a cycle with an uncontracted external input stay
unproved. One local ground does not excuse six missing call-result proofs.

At each contracted successful terminal, bind the exact proven SSA word to the
exact enclosing Result store. Preserve the terminal-use discipline without
converting this proof into a source-cut receipt.

### Finish the entire contract dependency graph

Extend derivation and finished verification over the existing declared-unit
population actually reached by these calls; add no emitted function class.
Do not populate ordinary declarations from the caller's demanded shape and call
that proof. Inspect the real callee first. If it returns actual identity 3380
where 4442 is required, or has mixed successful terminals, report that exact
counterexample rather than asserting a uniform 4442 contract.

Acyclic dependencies can discharge immediately. Recursive references may remain
explicit obligations while finalized bodies are collected. No obligation is a
grounding source. Close the entire function/SSA dependency graph, including
recursive SCCs, before object/artifact finalization. Every reference must resolve
to the same exact finished definition and contract; a cycle of declarations or
missing bodies cannot self-certify. Retain/refinalize evidence if a body changes.

This permits provisional compilation data, not provisional acceptance. Failure
to close any dependency preserves pre-object refusal. Neither clearing a
contract to `None` nor installing the demanded identity closes it.

## Authority separation and acceptance

Terminal Result proof says what a successful exact callee/frame load denotes.
HS17 source-stage receipt says which exact selected caller/response-owner/
checked-IH transport crossed which exits and leaves which ordered residual.
There is no conversion in either direction. A tag guard may refine an existing
terminal proof; it cannot establish either authority from outer shape alone.

The recut must include these reaching differentials:

1. Q1 phase/population evidence and unchanged-stack recovery of the named
   fixture. Any subsequently authorized graph must reuse a real repeated
   configuration rather than unfold it; disabling that interning must expose
   growth. Do not manufacture a repeated configuration by dropping owner keys.
2. Equal local heads with different caller/selecting edges remain distinct or
   refuse. Deleting/transplanting a source-child, call, transport or response
   owner edge fails the exact graph proof, not an unrelated counter.
3. Recompute the current equivalent of all seven preserved leaves individually.
   Withhold the callee proof at each call/load edge in turn: that predecessor
   must refuse while local ground and the other call edges remain visible.
4. A seeded identity-preserving loop closes; a pure cycle, a foreign-identity
   input and an uncontracted external call do not. Apply the same grounding
   discipline across recursive function-summary dependencies.
5. Same-tag foreign callee, wrong Result frame/offset/word, omitted status or
   Trap check, and intervening clobber refuse. Status-zero Trap propagation
   must remain a valid non-Result path.
6. Result closure neither mints nor replays an HS17 source cut. Retain all six
   new HS17 receipt controls, the non-emitting Tail `None` discriminator,
   residual483, and disjoint Active1097/check6. Completed699 is not replayed;
   only the exact selected caller exit661/check2 is consumed on its ruled path.
7. Preserve px8f parity, bytes, all ten ordered effects and the existing 256 MiB
   Builder stack; restore the frozen Mapping COW differential without file or
   source-semantic changes. Local gates remain targeted; full CI is separate.

The frame must distinguish these acceptance properties from conditional future
representation options. Neither the older library-control reds nor the aborted
library run is discharged by px8f7/7. No release claim is made here.
