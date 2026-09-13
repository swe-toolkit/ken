# HS18 Q2: certified infeasible edges, not a relocated contract

Architect, 2026-09-13, grounded at Runtime
`30d35f6256421a0b86cf772c739794365b3402be`. This is a design ruling for
bounded Q2 authoring after Steward folds the frame, not proof completion.

## Choose the path-proof fork

Keep context0's whole-function Ret obligation. Do not relocate it to `v211`,
clear it, annotate the Vis as Ret, or replace the final return by an inner value.
The missing relation is a certified infeasible control edge. Extend the existing
modular finished-Result design with a small proof-carrying feasibility analysis.
No code-generation branch is deleted on the strength of a provisional fact.

The Vis construction is real. Conditional on a genuinely certified Ret input,
its path is impossible; this does not turn a Vis word into a Ret word. Moving
the obligation to `v211` would stop proving the advertised outer Result while
changing neither the actual outer return nor its consumers.

## What the current evidence establishes

Mapping module funcid52 is context0, enclosing specialization1, raw-owner4,
body788. Response row0/MappingWriteView binds K body788 to the unique one-binder
Ret case, identity3606/36. Actual `v219` is the source580/aggregate103 two-field
ITree::Vis, identity2580/36, built by allocation/tag/field helpers. Neither
its tag nor arity satisfies the Ret contract.

The predecessor is `v87` in block36, whose sole incoming word is `v86` from
block35. That word is Result+144 from the first fn37 call after status-zero
and the same frame's Trap+160-zero checks. fn37 targets funcid49, response
owner1/MappingReadView, whose K is context2/funcid54, not context0. Its existing
emitter queries and requires the returned word's exact Ret tag and field count
one before storing that same word to its Result and returning status zero.

The actual discriminator sequence is:

```
block36: fn9 tag(v87) -> ss9; require helper status zero
block37: v94 = load ss9; fn10 field_count(v87) -> ss10;
         require helper status zero
block39: v102 = Ret ABI word; v103 = icmp eq v94,v102;
         brif v103, block43, block44
```

The non-Ret destination is ordinal1 of that block39 terminator. From it the
Vis discriminator and construction can lead through block93, block41 and
block7. The exact incoming edge, polarity, SSA arguments and memory versions
matter; a block name or numeric tag alone is not a certificate.

Architect matched the three supplied log hashes and inspected the actual source
issuer/consumer. An independent conditional CFG calculation retained all three
leaves with no cut, retained only `v123` and `v274` with that certified-Ret cut,
and retained `v219` with the opposite-polarity control. This measures the
conditional graph consequence, not the premise or a native execution.

Two important limits invalidate an immediate map-based pruning patch:

- `calls.rs` currently registers the Result authority from
  `target.result_contract`. That is a declaration, not a finished-body proof.
- `core.rs` defines contexts before response owners. This failing run stops
  before emitting the response-owner bodies. The provided owner binding is
  not a finished funcid49 certificate. The existing response verifier checks
  emitter-supplied facts, layout positions and a branch count; it does not yet
  independently prove exact tag/arity guards dominate all successful Result
  publications. Do not promote its current success into that stronger theorem.

The counterfactual genuine-Vis path excludes relabeling. It was not reported
as an executed guard-bypass mutation and is not counted as one here.

## Produce the facts before excluding edges

Use the existing declared/finished/call-obligation distinction. The bounded
implementation consists of these stages, all before accepting an object:

1. Record pending call and terminal Result obligations in the existing typed
   unit records. Stage the affected finalized CLIF bodies, including owners,
   rather than failing while context0 still prevents its callees from being
   emitted. Pending is NOT certified: no optimization, authority conversion,
   branch deletion or object publication may consume it. Do not clear contracts
   or mint placeholder certificates to obtain the later bodies.
2. Independently verify finished Result postconditions against those bodies.
   An owner certificate must bind its actual typed target, frame, Result store,
   initialized returned word, exact tag/arity validation and every reachable
   status-zero AND Trap-zero terminal. Read real helper targets/arguments,
   comparison constants/polarities, CFG dominance and memory-reaching; recorded
   instruction IDs are locators only. Layout order, a count of branches, a
   `ret_abi_word` field, or an emitter Boolean is not proof. The owner's tag
   guard may refine separately established returned-word realization, not
   conjure that realization or HS17 stage authority. Preserve the
   status-zero/nonzero-Trap path without a Result. Close genuine callee/SSA
   dependencies; a declaration cycle cannot certify itself.
3. Discharge the caller's exact call-result obligation using the finished
   certificate, actual call target, same-frame status/Trap checks, Result load
   and no clobber. Only this makes `v86` a usable constructor fact. Propagate
   such facts through block parameters only when ALL applicable incoming
   operands agree, using the existing grounded SCC discipline. A second unknown
   or foreign input to `v87` defeats this fact; one good incoming edge is not
   enough.
4. From that fact, prove the exact tag query observes that same word in that
   arena. Use only the existing typed, audited helper bindings: tag and the
   intervening field-count query, including their success checks and output
   stores. Prove the reaching write to the exact output slot, with no aliasing
   clobber before the SSA load. A constructor fact must remain valid until the
   query; unknown mutation of its header invalidates it. A captured SSA tag
   value is distinct from a mutable word's later identity.
5. Recognize only the needed equality/inequality comparison of that observed
   i64 tag with an exact constant, including this actual `icmp`/`iconst` form.
   Issue a function-scoped infeasible-edge certificate binding the terminator,
   destination ordinal/target, predicate, query and complete proof dependencies.
   No general symbolic executor, SMT engine, runtime check or generic decoder
   is authorized.
6. Compute reachability with only certified cuts. Every other CFG edge remains
   possible. The final predecessor-indexed Result proof consumes the cut proofs
   as dependencies; it still covers ALL possibly reachable inputs and both
   `Union` inputs. Do not register `v219` as Ret. Its incoming path is omitted
   only because the certified entry-to-edge analysis excludes it.

Keep fact derivations well founded. Begin with independent ground, not guessed
reachable blocks or the function's own desired Result. Each new exclusion must
use already justified facts, including previously justified exclusions. A fact
cannot rely on pruning the very foreign predecessor that prevents proving it.
Unseeded cycles, conflicting joins, missing bodies or unknown helper effects
remain unproved. Unsupported cases preserve the existing pre-object refusal.

Store obligations and proofs with the existing exact unit identities; add no
parallel owner registry or function class. Preserve the complete selected
caller/transport domains. Finalized CLIF collection is not object acceptance;
close the whole relevant function/SSA/cut proof graph before object emission.
This also applies to the retained px8f seven-leaf obligations: Mapping's two
surviving leaves do not discharge that separate population.

## Acceptance is two-sided

Require real Mapping native/interpreter COW behavior, file preservation and
existing effects, not only a vacuously true postcondition on a failing program.
Require the original whole-function Ret contract, and retain the Vis definition
without false authority. Keep Q1's unchanged-stack fixture and px8f's existing
256 MiB stack, parity, bytes, ten ordered effects and all HS17 receipt controls.

Independently exercise these proof boundaries:

- Remove/bypass or corrupt the owner's actual Ret tag/arity check while leaving
  declarations and emitter facts intact: no finished certificate or dependent
  edge exclusion. Likewise wrong Result word/frame/offset or missing call
  status/Trap protocol. Preserve legitimate status-zero Trap propagation.
- Substitute the tag helper, queried word, arena or output slot; omit its status
  check; clobber header/output before observation: no applicable cut proof.
- Add a genuine Vis or unknown predecessor to the input phi: no ANY-input
  propagation. Retain an independent positive and other surviving grounds.
- Alter comparison constant, polarity, destination ordinal or function identity:
  the old edge certificate must fail replay. If a different program admits a
  different valid proof, that is not a reason to accept the stale one.
- Remove the genuine root proof, or create a fact/pruning dependency cycle:
  closure must not bootstrap itself. A genuinely reachable Vis remains foreign.
- Disable the cut detector's checks while retaining populated positive and
  one-axis negative certificates: the negative controls must expose it.

These controls complement, not replace, inspection of the finished proof
algorithm. No new ABI/schema/runtime tag/route/allocation, owner quotient,
stack/bound increase, source-stage/Result conversion or weakened refusal is
permitted. A terminal tag fact proves only the represented Result property;
it never proves which source exits were processed. Runtime returns the exact
candidate and targeted evidence to Architect and QA; broader gates are CI-only.
