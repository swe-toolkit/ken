# Linear causal obligations in compiler lowering

## A calculus extracted from Ken's native backend

> **Status:** research reference; partial and non-normative. This paper
> reconstructs a compiler protocol from the implementation and the supporting
> research reports. It is not peer reviewed, its metatheory is not mechanized,
> and it does not add affine or linear types to Ken. The specification and
> [ADR 0021](../../docs/adr/0021-resource-lifetime-and-ward-delegation.md)
> remain authoritative for the product language and its resource posture.

## Abstract

Compiler lowering often creates obligations that are more precise than ordinary
control-flow invariants: a planner authorizes an event, one exact owner must
realize it, the realization may take one of several disjoint forms, and the
compiler must refuse to publish an artifact if the obligation is lost,
duplicated, forged, or attributed to the wrong owner. Ken's native backend
implements several such protocols for continuation calls, checked calls,
aggregate allocation, effect-operand visits, and control-flow edges.

This paper extracts those protocols into a small linear causal-obligation
calculus. Its central judgment is a checked transition over an immutable plan,
an unrestricted context, and an obligation context:

```text
P ; Gamma ; Delta |- S --r / b / epsilon--> S' ; Delta'
```

The calculus separates copyable identity from consumable authority, semantic
rule selection from backend mutation, and intended action from independently
checked evidence. It also distinguishes four closure policies that a single
"affine ledger" would conflate: exact realization, alternative realization,
event authorization, and visit closure. We state the principal rules, formulate
six safety properties, and map the abstractions to executable checks in Ken's
current compiler.

The result is best understood as a specification for a compiler-internal
protocol, not as a source-language type system. Its apparent novelty is narrow:
we are not aware of a published calculus that combines planner-minted opaque
identities, exact owner-indexed discharge, independently observed backend
evidence, and domain-specific closure laws in this compiler-lowering form. This
is a bounded literature finding, not a priority claim. Establishing novelty or
publication-grade results would require a systematic review, a mechanized
semantics, and proofs connecting extracted traces to emitted artifacts.

**Keywords:** compiler verification, lowering, linearity, resource protocols,
translation validation, typestate, causal provenance, evidence

## 1. Introduction

The difficult resource problem inside a compiler is not always memory. A
lowering pass can own facts that must be spent exactly once: the right to emit a
particular continuation call, the authority to connect one predecessor to a
join, or the obligation to account for an allocation before publishing a
function. These objects behave like resources even when their Rust identities
are small, copyable values.

Ken's native backend makes this pattern unusually visible. A validated static
plan establishes identities, owners, targets, and source relationships before
lowering begins. Lowering carries those identities through a defunctionalized
source machine. Domain-specific ledgers then compare planned populations with
claims and with facts recovered from emitted Cranelift IR. Missing, duplicate,
or inconsistent evidence is a compilation refusal.

The recurring structure is:

```text
validated mint
  -> opaque identity with immutable owner
  -> checked state transition
  -> concrete backend event
  -> independently validated evidence
  -> domain-specific closure
```

This paper asks whether that structure can be stated independently of the
backend's present Rust containers. The answer is yes, with two qualifications.
First, the common object is a family of obligation systems, not one universal
ledger. Second, the calculus specifies a compiler protocol; it does not alter
Ken's kernel, equality, or source-level resource model.

The paper makes five contributions:

1. It defines a checked lowering judgment with separate unrestricted and
   obligation contexts.
2. It distinguishes causal identity, consumable authority, backend command,
   and independently checked evidence.
3. It classifies four closure laws already present in the compiler and explains
   why they cannot be replaced by one exact-use policy.
4. It states the main safety properties and the proof obligations needed for a
   mechanized account.
5. It gives an as-built correspondence between the calculus and current Ken
   compiler structures and mutation controls.

## 2. Motivation and problem setting

### 2.1 From traversal bookkeeping to causal authority

An expression-tree traversal can often use local recursion and ordinary maps.
Ken's lowering problem is stricter. Planning and emission occur at different
times, generated functions split one source computation across several owners,
and a continuation may be realized either by a direct call or by verified
composition through an existing worker. A local "visited" bit cannot answer
all of the relevant questions:

- Did the planner authorize this event?
- Is this exact function the owner allowed to realize it?
- Was it realized once, rather than merely selected once?
- Did the emitted instruction target the planned callee?
- Was it realized directly or compositionally, but not both?
- Did every mandatory obligation close before artifact publication?

The qualifier *causal* matters. An identity records why an event exists and
which planning fact owns it. It is not merely a globally unique number. If a
coordinate is removed from an identity, two distinct causes can collapse into
one key; a subsequent duplicate-consumption error then truthfully reports the
collision while misidentifying its cause.

### 2.2 Names are not resources

The calculus separates an identity `kappa` from the obligation indexed by that
identity. A name may be copied, compared, transported, and used as a map key.
Those operations do not duplicate authority. Authority resides in the unique
entry of the obligation context and changes state only through checked rules.

This is the same high-level separation visible in several systems—handles from
owners, references from permissions, and tokens from typestate—but the present
use is compiler-internal. It governs the legality of lowering decisions and
artifact construction, not access to an operating-system resource.

### 2.3 Mandatory obligations and authorizations differ

Some planned entries must be realized exactly once. A continuation call is the
canonical example: omission and duplication are both defects. Other entries
authorize events that may never occur. An aggregate record can describe an
allocation reachable in a body that the current compilation never emits.

Calling both cases "affine" hides the difference. An affine policy rejects
reuse but permits discard. A linear policy rejects both reuse and discard. An
authorization policy instead requires every observed event to have a valid
record while allowing unused records. The closure law is therefore part of an
obligation kind's type, not a flag inferred at closeout.

## 3. Context and related work

### 3.1 Linear and quantitative type systems

Linear logic removes unrestricted contraction and weakening, giving a logical
foundation for exact resource use [Girard 1987]. Linear and affine programming
languages adapt that idea to values: linear values must be used exactly once,
while affine values may be discarded but not duplicated.

Quantitative Type Theory records a usage quantity for every variable in a
dependent typing judgment and uses semiring operations to account for compound
terms [Atkey 2018]. Idris 2 demonstrates that quantitative use can support
erasure and resource protocols in a full-scale dependently typed language
[Brady 2021]. These systems motivate the split between unrestricted `Gamma` and
resource-sensitive `Delta`, as well as the need to distinguish zero, one, and
unrestricted use.

The compiler calculus differs in level and evidence. It does not type Ken
terms, and its obligations do not participate in type formation. It also
requires post-emission observations such as decoding an actual callee from an
instruction. A typing derivation alone cannot supply those observations.

### 3.2 Linear dependent type theories

Lundfall's linear dependent type theory places cartesian dependent types beside
linear fibers, with both ordinary and linear types depending on cartesian terms
[Lundfall 2018]. Fu and Xi instead separate logical and program levels, erase
types and proofs from programs, and permit programs to be reflected into the
logical layer for verification [Fu and Xi 2023]. Recent work continues to
develop models of linear dependency, including dependent multiplicities [Doré
2025] and impredicative universes [Speight and van der Weide 2026].

These systems provide plausible frameworks for a future source-language
embedding. The calculus here takes the more conservative compiler boundary:
ordinary terms may index obligations, but obligations do not affect Ken's
kernel judgments or equality.

### 3.3 Typestate, ownership, and Rust

Typestate associates permitted operations with an object's current state
[Aldrich et al. 2009]. Rust adds an ownership and borrowing discipline whose
formal accounts are substantially richer than bare affinity. Oxide gives a
substructural account of ownership and lifetimes [Weiss et al. 2021], while
RustBelt uses higher-order concurrent separation logic to justify safe
abstractions containing unsafe code [Jung et al. 2018].

Ken's compiler is implemented in Rust and benefits from move-only structures,
but this calculus is deliberately smaller than Rust's discipline. It omits
loans, aliasing, lifetime inclusion, destructors, unwinding, provenance,
interior mutability, and unsafe abstraction. Mutable ledgers enforce several
of its exact-use properties dynamically during compilation; Rust's type checker
does not prove them all.

### 3.4 Translation validation and proof-producing compilation

Translation validation checks a particular compiler run rather than proving an
entire optimizer correct [Necula 2000]. The closest analogy here is the demand
that intended emission and actual emission remain independent facts. Ken's
checked-call and continuation mechanisms record evidence only after an
instruction exists and, where applicable, after its callee or return route has
been recovered from finished Cranelift IR.

The analogy has limits. The present ledgers validate selected lowering
invariants, not semantic equivalence of a complete source and target program.
They are local proof-producing or proof-checking protocols inside an untrusted
backend, not a whole-compiler correctness theorem.

### 3.5 Observational type theory

Observational Type Theory gives propositional equality type-directed,
extensional behavior while retaining computational properties such as
canonicity and decidable checking [Altenkirch, McBride, and Swierstra 2007].
TTobs and CCobs develop modern observational equality with proof irrelevance,
normalization, conversion, quotients, and inductive types [Pujet and Tabareau
2022; Sirman, Lennon-Bertrand, and Krishnaswami 2025].

Ken's kernel is in this observational tradition. The compiler calculus does not
solve the hard interaction between observational equality and linear values.
In particular, it gives no account of applying two allegedly equal linear
functions to the "same" owned input, and no proof that `cast` transports one
owner without duplication or loss.

### 3.6 Bounded novelty claim

The ingredients are established: linear contexts, typestate, ownership,
proof-producing compilation, translation validation, and compiler IR
semantics. The synthesis claimed here is narrower: a compiler-lowering
transition system in which a validated planner mints opaque causal identities,
an exact owner discharges them, concrete backend events yield independently
checked evidence, and each obligation kind selects one of several closure
algebras.

Targeted searches across those literatures did not identify a published system
with that exact combination. Terminology varies widely, however, and adjacent
work on typed assembly, proof-carrying code, session types, effect protocols,
and verified compilation is extensive. The responsible claim is therefore
"apparently novel as a compiler protocol," not "the first such calculus."

## 4. Formal setting

### 4.1 Sorts and plans

Assume abstract sorts:

```text
kappa   in Identity          o       in Owner
q       in ObligationKind    t       in TermId
g       in GeneratedTermId   u       in UnitId
e       in EventId           p       in PlanRecord
b       in BackendCommand    epsilon in Evidence
```

An identity is opaque. Only plan construction can mint one. Lowering may
compare, transport, and select identities but cannot synthesize one from its
coordinates.

A plan `P` is an immutable finite structure:

```text
P.identities : finite set Identity
P.owner      : Identity -> Owner
P.kind       : Identity -> ObligationKind
P.target     : partial Identity -> Target
P.origin     : TermId -> SourceOrigin
P.children   : TermId -> finite ordered sequence TermId
P.closeLaw   : ObligationKind -> ClosureLaw
```

The calculus assumes `P` has already passed checks for identity uniqueness,
owner and target consistency, occurrence integrity, and source-child
relations. Lowering cannot repair, extend, or reinterpret it.

### 4.2 Unrestricted and obligation contexts

The unrestricted context admits weakening and contraction:

```text
Gamma ::= empty
        | Gamma, term(t)
        | Gamma, generated(g)
        | Gamma, inert(kappa)
        | Gamma, worker(w)
        | Gamma, backendHandle(h)
```

These entries remain distinct despite sharing structural rules. A source term
has a planned origin; a generated term does not. An inert identity is a name,
not authority. A worker is reusable. A backend handle is scoped to the function
whose backend builder created it.

The obligation context is a finite map whose entries record their protocol
state:

```text
Delta ::= empty | Delta, kappa : O
O     ::= open(Q(o, payload))
        | claimed(Q(o, payload))
```

Map uniqueness prevents two live entries with the same identity. `Delta`
admits exchange but does not generally admit contraction or weakening.
Domain-specific coordinates remain in `kappa` or `payload`; erasing them can
destroy injectivity.

Lowering begins with one open entry for each identity in the validated plan.
No lowering rule can add an entry. Closure interprets any remaining open entry
according to its kind's closure law: a mandatory remainder refuses, while an
unused authorization or affine entry may be lawful.

### 4.3 Closure laws

Each obligation kind selects one closure law:

```text
ClosureLaw ::=
    ExactRealization
  | AlternativeRealization(dischargeForms)
  | EventAuthorization
  | VisitClosure
  | AffineAtMostOnce
```

`ExactRealization` and `AlternativeRealization` are linear. `AffineAtMostOnce`
permits discard. `EventAuthorization` permits unused plan records but requires
every event to be governed. `VisitClosure` requires completeness separately
for each visit and then checks a global authorization bound.

### 4.4 Machine states

A minimal lowering machine has evaluation, return, success, and refusal states:

```text
S ::= Eval(term, env, K)
    | Return(value, route, K)
    | Closed(output)
    | Refused(attribution)
```

`K` is a closed continuation datatype with one constructor per pending
evaluation context. Exhaustive matching over the source form and continuation
datatype is part of the completeness argument: a wildcard that silently sends
a new form down an obligation-free path invalidates it.

### 4.5 Commands and evidence

Rules choose explicit backend commands, for example:

```text
b ::= NoCommand
    | EmitCall(target, args)
    | EmitBranch(test, yes, no)
    | EmitJump(block, args)
    | EmitAggregate(layout, fields)
    | EmitHostDispatch(operation, operands)
    | EmitTrap(reason)
```

Evidence is typed and refers to an observed artifact:

```text
epsilon ::= NoEvidence
          | Claimed(kappa, owner)
          | DirectCall(kappa, function, inst, decodedTarget)
          | ComposedCall(kappa, function, inst, decodedTarget, returnRoute)
          | Allocation(event, record)
          | SeatClaim(group, seat, observedPhase)
          | Partition(left, right)
          | ClosedLaw(kind, summary)
```

Backend-local entities such as instructions and values are meaningful only in
their defining function. They may appear in evidence under that scope, but they
are not portable causal identities.

## 5. Judgments and reduction rules

### 5.1 Validity and state formation

Plan validity is written:

```text
|- P valid
```

State well-formedness is written:

```text
P ; Gamma ; Delta |- S state
```

It requires source term identifiers in `S` or `K` to occur in `P`, generated
terms to occur in the compiler-generated arena, environment bindings to agree
with the occurrence, backend handles to belong to the active function, and
every causal name carried by the state to be inert or backed by its matching
entry in `Delta`.

### 5.2 Checked reduction

The central judgment is:

```text
P ; Gamma ; Delta |- S --r / b / epsilon--> S' ; Delta'
```

Rule `r` chooses the semantic transition and declares its obligation effect.
The backend interprets `b`. An evidence checker validates `epsilon` against the
resulting artifact. Only then is `Delta'` committed.

This order makes refusal atomic. A planned intention cannot satisfy a
realized-event law, and a failed command or evidence check does not discharge
an obligation.

### 5.3 Planner minting

Minting occurs before lowering:

```text
kappa fresh
P' = P + plan(kappa, Q, owner, payload)
------------------------------------------------ Plan-Mint
P ==> P'
```

No lowering rule concludes with a larger planned identity population. This is
the no-forgery boundary.

### 5.4 Ordinary structural step

An ordinary traversal step preserves `Delta`:

```text
child(P, t, 0) = t0
------------------------------------------------ Eval-Let
P ; Gamma ; Delta |- Eval(Let(t0, t1), env, K)
  --let / NoCommand / NoEvidence-->
  Eval(t0, env, LetBody(t1, env, K)) ; Delta
```

Analogous rules cover constructors, calls, projections, matches, and return
frames. Generated terms use an explicit generated-term rule rather than a
fabricated source origin.

### 5.5 Exact claim

```text
Delta(kappa) = open(Q(owner, payload))
ambientOwner(S) = owner
------------------------------------------------ Claim
P ; Gamma ; Delta |- S
  --claim(kappa) / NoCommand / Claimed(kappa, owner)-->
  S ; Delta[kappa := claimed(Q(owner, payload))]
```

If `kappa` is absent, already claimed, or owned by another unit, the transition
refuses and attributes the failed premise. Claiming does not by itself satisfy
a realization law.

### 5.6 Direct discharge

```text
Delta(kappa) = claimed(Emit(owner, target))
ambientOwner(S) = owner
interpret EmitCall(target, args) = artifact(inst)
decodeCallee(inst) = target
------------------------------------------------ Discharge-Direct
P ; Gamma ; Delta |- S
  --direct(kappa) / EmitCall(target, args)
    / DirectCall(kappa, function, inst, target)-->
  Return(result(inst), Direct(kappa), K) ; Delta - kappa
```

The decoded callee is intentionally a post-emission premise. The target the
planner intended and the target the instruction contains are independent
facts.

### 5.7 Composed discharge

```text
Delta(kappa) = claimed(Emit(owner, continuationTarget))
ambientOwner(S) = owner
compositionContract(S, kappa) = (worker, args, downstream)
interpret EmitCall(worker, args) = artifact(inst)
decodeCallee(inst) = worker
verifyReturnRoute(inst, downstream)
------------------------------------------------ Discharge-Composed
P ; Gamma ; Delta |- S
  --compose(kappa) / EmitCall(worker, args)
    / ComposedCall(kappa, function, inst, worker, downstream)-->
  Return(result(inst), Composed(kappa), downstream) ; Delta - kappa
```

Direct and composed rules consume the same unique entry. A derivation therefore
cannot use both. An implementation still checks evidence-set disjointness at
closeout because its mutable representation can contain bugs the abstract map
rules out by construction.

### 5.8 Affine consumption

```text
Delta(kappa) = open(Affine(owner, payload))
ambientOwner(S) = owner
------------------------------------------------ Consume-Affine
P ; Gamma ; Delta |- S
  --consume(kappa) / b / epsilon-->
  S' ; Delta - kappa
```

The close law does not require every affine entry to disappear, but a second
consumption has no derivation.

### 5.9 Static branch partition

```text
Delta1 disjoint Delta2
Delta = Delta1 union Delta2 union DeltaShared
P ; Gamma ; Delta1 union DeltaShared |- S1 ==> S1' ; DeltaShared'
P ; Gamma ; Delta2 union DeltaShared |- S2 ==> S2' ; DeltaShared'
------------------------------------------------ Static-Branch-Partition
P ; Gamma ; Delta |- EmitBothBodies(S1, S2) ==> ...
```

This is a compiler-static partition, not runtime exclusive choice. If lowering
emits both bodies, each receives its own mandatory obligations. Only genuinely
unrestricted or authorization-only material may be shared.

### 5.10 Closure

```text
mandatory(Delta, owner) = empty
noOpenGroups(owner)
allEvidenceLocalTo(owner)
------------------------------------------------ Close-Unit
P ; Gamma ; Delta |- Return(v, route, Terminal(owner))
  --close-unit / Return(v) / ClosedLaw(owner, summary)-->
  Closed(v) ; Delta
```

A body-local close is necessary when detecting an incomplete transaction after
publication would cross the desired refusal boundary. Whole-pass closure then
checks the cross-unit population.

## 6. The closure algebra

### 6.1 Exact realization

For planned identities `Pq`, consumed identities `Cq`, and identities recovered
from emitted artifacts `Eq`:

```text
Cq = Pq
Eq = Pq
```

Set equality, rather than equal cardinality, rules out a missing member hidden
by an extra one.

### 6.2 Alternative realization

For direct evidence `Dq` and verified composed evidence `Kq`:

```text
Dq intersect Kq = empty
Dq union Kq = Pq
```

Disjointness rejects double answers. Coverage rejects missing and unplanned
answers. Neither equation implies the other.

### 6.3 Event authorization

Let `E` be independently observed events, `Pq` planned authorization records,
and `R` the event-to-record relation:

```text
domain(R) = E
image(R) subset-of Pq
R is functional from E to Pq
```

`image(R) = Pq` is deliberately absent. An unused authorization record is
lawful; an ungoverned event is not.

### 6.4 Visit closure

For each visit group `g`, with planned seats `Pg` and claims `Cg`:

```text
Cg = Pg
observedPhase(c) in availability(c) for every c in Cg
image(all claims) subset-of global planned seats
opened groups = committed groups
```

Local equality prevents two incomplete visits from masking one another by
union. The global subset permits seats belonging to bodies not emitted by this
compilation.

## 7. Safety claims and proof obligations

The standalone calculus is intended to support six results. At present these
are theorem statements and proof sketches, not mechanized theorems.

### 7.1 No forgery

If `kappa` occurs in a reachable `Delta`, then it occurs in the validated plan,
and its kind, owner, and payload agree with the plan.

The proof is by induction on reduction. Minting is outside lowering, and no
lowering rule introduces an identity.

### 7.2 At-most-once discharge

For any derivation trace and mandatory identity `kappa`, at most one discharge
evidence item names `kappa`.

Unique map membership and removal on discharge establish the abstract result.
The implementation additionally needs duplicate-insertion and evidence-set
checks because its state is represented by several mutable collections.

### 7.3 Owner preservation

Every claim and discharge for `kappa` occurs under `P.owner(kappa)`. Transport
through a continuation may retain the identity but cannot change its owner.

### 7.4 No missing mandatory discharge

If compilation reaches whole-pass `Closed`, every mandatory identity has
evidence satisfying its closure law.

This is a closure theorem, not a consequence of at-most-once use. It is what
makes the strongest mechanisms linear rather than affine.

### 7.5 Artifact agreement

Every evidence item describing a backend event corresponds to an event in the
finished function, and every governed event in the finished function appears
in the relevant relation.

This claim cannot follow from a symbolic command trace alone if the command
interpreter may select the wrong callee, reorder operands, or omit an
instruction. It requires a concrete-artifact observation.

### 7.6 Progress or attributed refusal

For a valid state, either one reduction rule applies, the state is lawfully
closed, or a failed premise identifies an absent authority, owner mismatch,
unsupported representation, or backend inconsistency.

This is not source-language progress. Compilation refusal is a valid outcome;
silent fallthrough is not.

## 8. Correspondence with Ken's implementation

The calculus is extracted from current compiler structures, but it does not
freeze their Rust representation.

| Calculus object | Current implementation role |
|---|---|
| validated `P` | `StaticTransitionPlan` and its validated projections |
| opaque `kappa` | `ContinuationCallIdentity` and other plan-issued ids |
| immutable owner | `ContinuationEmissionOwner` and other plan-owned coordinates |
| `Gamma` source entry | `OwnedSourceOccurrence` plus planner lookup |
| closed `K` | `SourceContinuation` / `SourceContinuationTerminal` |
| machine `S` | `SourceMachineState` |
| exact realization | `CheckedCallLedger` |
| alternative realization | `ContinuationClaimLedger` |
| event authorization | `AggregateAllocationLedger` |
| visit closure | `EffectSeatLedger` |
| affine transition | `AffineSpliceCapability` and move-only edge authorities |

Three details are load-bearing.

First, `ContinuationCallIdentity` is planner-issued and opaque to lowering. Its
semantic identity includes producer construct, alternative, call-site sequence,
and recursive position; lowering can query the target and owner but cannot
reconstruct the identity from those projections.

Second, `ContinuationClaimLedger` distinguishes declaration, exact claim,
verified direct emission, and verified composed discharge. A composed discharge
is promoted only after the raw-worker call, operand run, and downstream return
route have been checked against finished function IR. Its closeout requires the
direct and composed populations to be disjoint and exhaustive.

Third, aggregate and effect-seat plans are authorizations, not mandatory global
populations. `AggregateAllocationLedger` checks `domain(R) = E` locally and
globally while accepting unused plan records. `EffectSeatLedger` closes each
visit independently and rejects an incomplete or discarded group before its
body is defined, then restates the opened-versus-committed law globally.

The calculus deliberately omits `BTreeMap`, `BTreeSet`, Cranelift entity types,
and module boundaries. They implement finite relations and scoped evidence; they
are not semantic rules.

## 9. Executable support

The current implementation supplies an executable decision procedure for
fragments of the calculus. Its evidence is stronger than a clean test run where
mutation controls change one premise at a time and demonstrate the expected
refusal.

| Property | Representative executable support |
|---|---|
| planned, consumed, and emitted sets agree | `d5_the_checked_call_closeout_rejects_omission_duplication_and_a_substituted_callee` |
| direct and composed forms are disjoint and exhaustive | `d8k_the_causal_population_is_a_disjoint_partition_of_direct_and_composed` |
| every allocation event has one governing relation entry | `the_aggregate_allocation_relation_holds_its_laws` |
| every effect visit claims its full seat population | `an_incomplete_duplicate_discarded_or_misobserved_visit_rejects` |
| incomplete visits refuse before publication | `a_discarded_visit_refuses_before_its_body_is_defined` |
| affine capabilities reject a second use | splice-capability controls in `lowering/core/tests/control.rs` |

These tests support an implementation correspondence, not a proof of the
calculus. Several checks operate directly on ledger APIs; others exercise a
full compile with a mutation installed. A mechanized account must distinguish
the two and prove that extraction from production traces covers every relevant
event.

The strongest evaluation plan is premise-oriented:

1. Give every formal premise a negative witness that changes only that premise.
2. Pair every negative absence check with a positive reachability control.
3. Require the refusal to identify the intended failed premise.
4. Verify that mutations are armed on the production route they claim to test.
5. Compare symbolic evidence with concrete emitted IR wherever backend mutation
   could diverge from rule selection.

## 10. Scope, limitations, and threats to validity

### 10.1 This is not Ken's resource type system

Ken does not currently have affine or linear source types. Its public resource
handles remain copyable generation-checked names backed by Rust-owned state,
with exactly-once settlement delegated to Ward under ADR 0021. The compiler
calculus neither replaces nor weakens that decision.

### 10.2 This is not Rust's ownership model

The calculus says nothing about borrowing, lifetime inclusion, aliasing,
destruction, unwinding, concurrency, or unsafe abstraction. Describing it as a
formalization of Rust would be false.

### 10.3 The metatheory is proposed, not proved

The safety results have proof sketches only. There is no mechanized syntax,
substitution lemma, preservation proof, trace semantics, or verified extractor.
The Rust implementation is evidence for the design and a candidate decision
procedure, not a proof assistant.

### 10.4 Artifact agreement is the hardest bridge

A symbolic transition can say `EmitCall(target)`, yet buggy backend code can
emit another callee. The current implementation closes some instances by
decoding actual instructions. A complete account needs a scoped trace model and
a theorem connecting every evidence constructor to finished backend artifacts.

### 10.5 The implementation is evolving

The correspondence is grounded in the repository revision recorded by the
library manifest and status machinery. Names and module boundaries may change;
the mathematical distinctions should survive only if their source mechanisms
still do. This page must be revised when a cited producer, ledger, closure
boundary, or evidence path changes semantically.

### 10.6 The literature review is not systematic

The related-work search covered the most immediate literatures and exact
terminology combinations. It did not perform a database protocol, citation
snowball to saturation, or independent expert review. The novelty statement is
therefore provisional and intentionally narrow.

## 11. Research program

A credible path from technical reference to academic result is:

1. Formalize continuation-call alternative realization as the first domain.
2. Mechanize the small-step system with finite maps and scoped evidence.
3. Prove no forgery, at-most-once discharge, owner preservation, closure, and
   attributed refusal.
4. Define an extractor from compiler traces and prove the continuation ledger
   decides the same closure predicate.
5. Add event authorization and visit closure as distinct typed policies.
6. Connect evidence to finished Cranelift IR without treating function-local
   handles as portable identities.
7. Evaluate whether an explicit lowering IR can make rule coverage structural.
8. Only then test a two-context program judgment beside Ken's frozen
   observational layer.

The last step has a separate research gate. A source-language extension must
explain equality of linear values, ownership-preserving `cast`, quantitative
substitution, erasure, normalization, and decidable conversion. Failure there
does not invalidate the compiler calculus.

## 12. Conclusion

Ken's native backend contains more than a collection of Rust ownership idioms.
It implements a repeated protocol in which a validated planner establishes
causal authority, exact owners realize that authority, concrete backend events
produce independently checked evidence, and closure refuses missing,
duplicated, forged, or misattributed realizations.

The durable abstraction is a family of typed causal-obligation systems. Some
members are linear, some affine, and some authorize optional events. Their
shared algebra is useful precisely because it preserves those differences.

The calculus is already useful as a design and review vocabulary for compiler
lowering. Turning it into an academic result requires mechanization, a verified
implementation correspondence, evaluation beyond Ken, and a systematic
literature review. Until then, its honest status is a promising, apparently
novel compiler protocol with unusually strong executable support.

## References

- Jonathan Aldrich, Joshua Sunshine, Darpan Saini, and Zachary Sparks.
  [Typestate-Oriented Programming](https://doi.org/10.1145/1639950.1640073).
  OOPSLA Companion, 2009.
- Thorsten Altenkirch, Conor McBride, and Wouter Swierstra.
  [Observational Equality, Now!](https://doi.org/10.1145/1292597.1292608).
  PLPV, 2007.
- Robert Atkey.
  [The Syntax and Semantics of Quantitative Type
  Theory](https://doi.org/10.1145/3209108.3209189). LICS, 2018.
- Edwin Brady.
  [Idris 2: Quantitative Type Theory in
  Practice](https://arxiv.org/abs/2104.00480). ECOOP, 2021.
- Maximilian Doré.
  [Dependent Multiplicities in Dependent Linear Type
  Theory](https://arxiv.org/abs/2507.08759). 2025.
- Qiancheng Fu and Hongwei Xi.
  [A Two-Level Linear Dependent Type
  Theory](https://arxiv.org/abs/2309.08673). 2023.
- Jean-Yves Girard.
  [Linear Logic](https://doi.org/10.1016/0304-3975(87)90045-4).
  *Theoretical Computer Science* 50, 1987.
- Ralf Jung, Jacques-Henri Jourdan, Robbert Krebbers, and Derek Dreyer.
  [RustBelt: Securing the Foundations of the Rust Programming
  Language](https://doi.org/10.1145/3158154). POPL, 2018.
- Martin Lundfall.
  [A Diagram Model of Linear Dependent Type
  Theory](https://arxiv.org/abs/1806.09593). 2018.
- George C. Necula.
  [Translation Validation for an Optimizing
  Compiler](https://doi.org/10.1145/349299.349314). PLDI, 2000.
- Loïc Pujet and Nicolas Tabareau.
  [Observational Equality: Now for
  Good](https://doi.org/10.1145/3498693). POPL, 2022.
- Matthew Sirman, Meven Lennon-Bertrand, and Neel Krishnaswami.
  [Implementing a Type Theory with Observational Equality, Using
  Normalisation by Evaluation](https://doi.org/10.4230/LIPIcs.TYPES.2024.5).
  TYPES, 2025.
- Sam Speight and Niels van der Weide.
  [Impredicativity in Linear Dependent Type
  Theory](https://arxiv.org/abs/2602.08846). 2026.
- Aaron Weiss, Olek Gierczak, Daniel Patterson, and Amal Ahmed.
  [Oxide: The Essence of Rust](https://arxiv.org/abs/1903.00982). 2021.

## Repository sources

- [Standalone research formalization](../../research/linear-causal-obligation-calculus.md)
- [Parent ATT/OTT assessment](../../research/causal-obligation-calculus.md)
- [Static-transition planning](../../crates/ken-runtime/src/cranelift_backend/planning/static_transition.rs)
- [Unit and continuation ledgers](../../crates/ken-runtime/src/cranelift_backend/lowering/units.rs)
- [Lowering state machine and domain ledgers](../../crates/ken-runtime/src/cranelift_backend/lowering/mod.rs)
- [Lowering implementation](../../crates/ken-runtime/src/cranelift_backend/lowering/core.rs)
- [Constructor and allocation controls](../../crates/ken-runtime/src/cranelift_backend/lowering/core/tests/constructors.rs)
- [Continuation and effect-seat controls](../../crates/ken-runtime/src/cranelift_backend/lowering/core/tests/control.rs)
- [Ken's settled resource posture](../../docs/adr/0021-resource-lifetime-and-ward-delegation.md)
