# Formalizing continuation-call alternative realization

**Research status:** advisory campaign brief, not a language, architecture, or
work-program ruling

**Grounding:** `origin/main` at
`89676797829700dd75918d81580d3d5e768eb6bd`

**Date:** 2026-08-09

## Executive assessment

The first recommended research step in the parent calculus report is a bounded
and tractable formal-methods project. It is not yet a unification of affine type
theory and observational type theory. Its purpose is narrower: turn one compiler
protocol into a precise calculus, prove its safety properties, and determine
whether the current Rust ledgers decide the same acceptance predicate.

The implementation has advanced since the parent report was written. The
correct formal target now has two levels:

1. every planner-minted binding candidate receives exactly one disposition;
   and
2. the direct and composed dispositions induce the smaller population of call
   obligations, which must be claimed and realized exactly once.

`InlineNoCall` is a positive disposition of a binding candidate. It is not a
third form of call discharge. This distinction is the load-bearing result of
`RT-CONTINUATION-EDGE-DISPOSITION`, and it replaces the older shorthand
`planned = direct disjoint-union composed` as the formalization target.

A useful paper specification and executable reference model should take about
20--32 active T1 agent-hours. A machine-checked calculus should take about
38--62 cumulative hours. Adding a production-trace extractor and a substantive
correspondence result raises the complete campaign to about 55--90 cumulative
hours. These estimates carry roughly 30 percent uncertainty, concentrated in
proof-assistant integration and evidence extraction rather than in writing the
rules.

The recommended result is a separate research artifact, a mechanized proof
development, and a test-side trace checker. It should not begin as a symbolic
engine in the production compiler. Production integration should be considered
only after the external model has proved useful and its correspondence boundary
is understood.

The product posture is unchanged. `R2` remains closed, ADR 0021 remains in
force, and this report neither files a work-program node nor authorizes a
compiler refactor.

## 1. Why this is a separate report

The parent calculus report describes a family of compiler-local obligation
systems and the possible later relationship to ATT and OTT. This report scopes
one research campaign inside that larger program. Keeping it separate has three
advantages:

- the general calculus remains stable while the continuation implementation
  evolves;
- effort, deliverables, assumptions, and stop conditions can be revised without
  changing the theoretical claim; and
- a later mechanization can cite one exact domain contract rather than a survey
  section that also covers allocation, authorization, and visit closure.

This report therefore refines, but does not amend away, the sequence proposed
in the parent report.

## 2. The domain to freeze

### 2.1 Successful-artifact boundary

The domain is one selected `FunctionizedUnits` compilation that reaches
successful artifact closeout. It does not quantify over:

- non-selected `RecursiveDescent` plans;
- candidates observed only in an exploratory census;
- compilation attempts that refuse before artifact closure;
- source-language evaluation in general; or
- every continuation-like object in the compiler.

The successful-artifact restriction is semantic, not a convenient test filter.
The candidate ledger exists inside the selected lowering path and closes only
on success. A formal model that silently includes failed or non-selected
attempts would prove a different property.

The landed observational census supplies non-vacuity evidence: it classified
637 candidates with one disposition each and zero orphans, including 21
`InlineNoCall` members. That 637-member observational superpopulation is not
itself `K` for one successful artifact. The formal quantifier remains one
selected artifact and its own live planner projection.

### 2.2 Opaque identity and immutable authority

Let `K` be the finite set of continuation binding candidates minted by the
validated planner for the artifact. Each `k` in `K` carries immutable planner
facts:

```text
producerOwner(k) : ProducerOwner
emissionOwner(k) : EmissionOwner
target(k)        : Target
coordinates(k)   : ProducerConstruct x Alternative x CallSiteSequence
                   x RecursivePosition
```

The producer owner is provenance; the emission owner is the authority checked
when a call is claimed. They must not be collapsed into one generic owner. The
call-site sequence has no lowering constructor or accessor. The calculus should
preserve that opacity: lowering may compare, carry, and look up an identity, but
no lowering rule may mint or reconstruct one.

### 2.3 Candidate disposition

Each candidate must be settled exactly once:

```text
Disposition ::= DirectCall | ComposedCall | InlineNoCall

delta : K partial-> Disposition
```

The close rule first requires `dom(delta) = K`. Single-valuedness must hold at
the settlement transition itself, so a second settlement is refused even when
it repeats the same disposition.

Only after totality is established may the model derive:

```text
O = { k in K | delta(k) = DirectCall or delta(k) = ComposedCall }
```

`O` is the call-obligation population. Deriving it before checking totality is
unsound as an implementation discipline: an unsettled candidate would occur in
neither call class and would silently disappear from the subset.

### 2.4 Call claims and realizations

For the call-obligation population, define:

```text
H : finite set Identity    claimed call obligations
D : finite set Identity    validated direct realizations
C : finite set Identity    validated composed realizations
R : finite set Identity    resolved candidates
L : finite set Identity    declared candidates
```

The full candidate population remains relevant to resolution and declaration.
Only the derived call-obligation population is relevant to claim and discharge:

```text
R = K
L = K
H = O
D intersection C = empty
D union C = O
```

The two population levels must not be collapsed. Requiring `R = L = O` would
incorrectly exclude a lawful `InlineNoCall` candidate. Requiring `H = K` would
incorrectly turn an inline non-call back into a call obligation.

## 3. Proposed transition system

The paper calculus should define explicit states and refusal outcomes rather
than encode the protocol as final set equalities alone. A suitable abstract
state is:

```text
Sigma = <P, phase, delta, R, L, H, D, C, evidence>
```

`P` is the immutable validated plan. `phase` distinguishes open lowering from
closeout. The finite maps and sets are mathematical relations; `BTreeMap`,
`BTreeSet`, Cranelift handles, and Rust module boundaries do not belong in the
core semantics.

The first rule set should contain at least the following transitions.

### 3.1 Plan mint

The planner introduces a fresh opaque candidate together with its owner,
target, and provenance. No lowering transition can enlarge `K`. This is the
no-forgery boundary.

### 3.2 Open artifact

Opening projects `K` from the same validated plan relation used by resolution
and declaration. It creates empty settlement, claim, realization, and evidence
relations. Opening from a separately reconstructed population is not a rule.

### 3.3 Resolve and declare

Resolution binds each candidate to its planner-issued target. Declaration
records the corresponding backend declaration. These transitions range over
all of `K`, including eventual `InlineNoCall` members.

### 3.4 Claim

A call-producing unit may claim `k` only if:

- `k` is planner-minted;
- the ambient emission seat is `emissionOwner(k)`;
- `k` has not already been claimed; and
- the relevant lowering path has established that a call is being produced.

Claiming does not itself constitute realization evidence.

### 3.5 Settle direct

After the direct producer and call seat succeed, settle
`delta(k) = DirectCall`. The rule must reject an unplanned identity, a wrong
owner, or any prior settlement.

### 3.6 Validate direct realization

After emission finishes, decode the emitted callee from the concrete IR and
compare it with `target(k)`. Only that independently inspected fact may add `k`
to `D` and attach a direct evidence witness.

### 3.7 Settle composed

After a raw-worker call has been emitted and has entered the existing composed
verification path, settle `delta(k) = ComposedCall`. Settlement and artifact
validation should remain separate judgments even if the implementation invokes
them close together.

### 3.8 Validate composed realization

Inspect the finished IR to establish the raw-worker call, operand run, and
downstream return route. Only that evidence may add `k` to `C`.

### 3.9 Settle inline

Settle `delta(k) = InlineNoCall` only after the exact deferred bridge scope
completes successfully with the candidate still unconsumed. This transition
adds nothing to `H`, `D`, or `C`.

### 3.10 Close candidate classification

Require that every member of `K` occurs exactly once in `delta`. Only then
derive `O` from the two call-producing dispositions.

### 3.11 Close call obligations

Require exact set equality, not matching cardinalities:

```text
R = K
L = K
H = O
D intersection C = empty
D union C = O
```

For each claimed or realized identity, require agreement with its immutable
planner owner. A failure of any premise produces an explicit refusal; it is not
an undefined or silently ignored transition.

## 4. The theorem package

The first mechanization should prove the following results.

### 4.1 No forgery

Every identity mentioned by resolution, declaration, settlement, claim, or
realization belongs to `K`. Since only plan mint enlarges `K`, lowering cannot
manufacture an authority by reconstructing coordinates.

### 4.2 At-most-once disposition

No reachable state assigns two dispositions to one candidate. This includes
attempting the same disposition twice, not only assigning two different
constructors.

### 4.3 Disposition totality at successful close

If closeout succeeds, `delta` is a total function on `K`. Consequently an
unobserved consumption path cannot disappear merely because the obligation
subset is derived from observed call dispositions.

### 4.4 At-most-once call realization

No successful derivation contains an identity in both `D` and `C`, or two
realization events for the same identity within either set.

### 4.5 Mandatory call closure

Successful closeout implies that `D` and `C` form a disjoint and exhaustive
partition of `O`, and that `H = O`. No call obligation is unclaimed,
unrealized, doubly realized, or spuriously added.

### 4.6 Owner preservation

Every claim and realization of `k` is performed by `emissionOwner(k)`, while
`producerOwner(k)` remains an unchanged provenance fact. This theorem should be
stated even where the current selection structure makes it hold by construction,
because a future representation may decouple selection from ownership.

### 4.7 Evidence faithfulness

Membership in `D` or `C` implies the existence of the corresponding concrete
artifact witness. This theorem will initially be conditional on an abstract
evidence-validation relation. The production correspondence phase must state
which parts are proved inside the calculus and which are delegated to the IR
decoder and verifier.

### 4.8 Close characterization

The reference close function returns success if and only if all of the domain
equalities and owner conditions above hold. This connects the inductive
transition semantics to an executable finite-map checker.

## 5. Compiler correspondence

### 5.1 The target statement

The strongest practical result is not that the Rust types resemble the
calculus. It is a decision-procedure correspondence statement:

> For every selected successful-artifact trace in the defined extraction
> domain, the Rust continuation closeout returns success if and only if the
> formal reference checker accepts the extracted trace.

The quantification boundary and extraction assumptions must be written beside
the theorem. In particular, the result must not claim correspondence for events
that the extractor cannot observe independently.

### 5.2 Trace shape

A versioned research trace should contain stable semantic facts rather than
portable backend handles:

```text
ArtifactOpened artifact plan_digest
CandidateMinted identity producer_owner emission_owner target provenance
Resolved identity target
Declared identity
Claimed identity emission_owner
DispositionSettled identity disposition
DirectEvidence identity emission_owner decoded_target evidence_digest
ComposedEvidence identity emission_owner raw_worker route evidence_digest
CloseAttempt artifact outcome
```

The trace should preserve ordering where order is causal, but the close checker
should compare identity sets rather than counts. Cranelift instruction handles
may appear in diagnostic side data; they should not be part of the portable
formal identity.

### 5.3 Independence requirement

The extractor must not restate the ledger's final sets and then ask the formal
checker to accept them. That would validate the ledger against itself. Candidate
population should come from the live planner projection, while direct and
composed evidence should come from the independent post-emission inspection
sites already used by the compiler.

The same rule applies to owner and target facts. A correspondence test is useful
only when the compared facts have independent producers or when the shared
assumption is named explicitly.

### 5.4 Mutation parity

The current implementation campaign supplies an unusually strong starting
corpus. At minimum the reference checker and correspondence harness should
reproduce the five established defect classes:

1. suppress binding installation;
2. settle inline before bridge completion;
3. settle inline after a composed call;
4. omit a final disposition; and
5. present one candidate in two dispositions.

Each mutation must alter its intended causal fact and independently change the
checker outcome. A common terminal error string is corroboration, not by itself
five different proofs.

Additional model-level counterexamples should cover a forged identity, wrong
owner, wrong decoded direct target, malformed composed return route, overlap of
`D` and `C`, and equal-cardinality but unequal identity sets.

## 6. Campaign stages and T1 effort

The estimates below are active T1 agent-hours. They are not human work-days and
do not include asynchronous human review, Steward publication, or CI queue
latency.

| stage | result | hours | cumulative |
|---|---|---:|---:|
| 1. Freeze and reconcile | domain boundary, implementation correspondence ledger, assumptions | 4--7 | 4--7 |
| 2. Paper calculus | syntax, judgments, transitions, refusals, theorem statements | 10--16 | 14--23 |
| 3. Executable model | deterministic finite-map checker and counterexample corpus | 6--9 | 20--32 |
| 4. Mechanization | checked proofs of the theorem package | 18--30 | 38--62 |
| 5. Production correspondence | trace extractor, equivalence checks, mutation parity, final report | 17--28 | 55--90 |

The ranges carry about 30 percent uncertainty. Three factors dominate:

- whether the chosen proof environment already has suitable finite-map and
  freshness libraries;
- whether concrete evidence can be exported without coupling the trace to the
  ledger it is meant to check; and
- whether the Rust closeout has incidental ordering or error-precedence behavior
  that must be excluded from, or represented in, the correspondence statement.

A frontier model reduces search, drafting, and routine proof construction time.
It does not remove proof-checker iteration, toolchain friction, or the need for
independent evidence. Those fixed costs are why the mechanized and
correspondence stages remain materially larger than the paper calculus.

## 7. Durable outputs

The campaign should produce five independently reviewable artifacts.

### 7.1 Formal specification

A research document containing the domain boundary, state space, judgments,
transition and refusal rules, invariants, theorem statements, and explicit
assumptions.

### 7.2 Mechanized development

A small proof-assistant project containing finite-map definitions and checked
proofs. Tool selection should be made by a short preliminary spike and should
not affect the mathematical statement. Model checking or property testing may
assist exploration but does not substitute for the stated proofs.

### 7.3 Executable reference checker

A deterministic checker over abstract traces, suitable for examples,
counterexamples, and differential tests. It should remain outside the
production compilation path during the research campaign.

### 7.4 Compiler trace and correspondence harness

A test-only extractor plus a harness comparing current Rust closeout with the
reference checker. Its schema, evidence origins, and independence assumptions
must be documented.

### 7.5 Epistemic and correspondence report

A matrix mapping every formal premise to its implementation producer, closing
check, discriminating mutation, and status: proved, tested, delegated, assumed,
or unknown. This is the artifact that prevents a machine-checked abstract model
from being misreported as a proof of the production compiler.

## 8. Gates and stopping conditions

### Gate A: semantic adequacy

The paper calculus must represent `InlineNoCall` without calling it a discharge,
must check disposition totality before deriving `O`, and must preserve the two
population levels at close. If it cannot do all three cleanly, stop before
mechanization and revise the model.

### Gate B: mechanized safety

The proof development must establish no forgery, at-most-once settlement and
realization, totality, owner preservation, mandatory closure, and close
characterization. A finite test corpus is not a substitute for this gate.

### Gate C: non-vacuous correspondence

The extractor must observe at least one real member of every disposition class
used by the theorem, and mutations must change independently sourced facts. If
the extractor merely serializes final ledger contents, stop and redesign the
evidence boundary.

### Gate D: production-integration decision

Only after the first three gates should Architecture consider whether an
explicit obligation IR or symbolic checker belongs in production. The research
result can remain valuable as an external specification and test oracle even if
the answer is no.

## 9. What the result would not establish

Even a fully successful campaign would not establish:

- a Ken source-language affine type system;
- a general ATT--OTT unification;
- observational equality for linear or affine values;
- resource-sensitive rules for `Eq`, `cast`, substitution, or conversion;
- correctness of allocation, effect-seat, or visit-closure ledgers; or
- correctness of all continuation lowering outside the stated trace domain.

It would establish something narrower and still substantial: one real compiler
protocol has a small linear causal-obligation semantics, its principal safety
laws are machine checked, and the current implementation agrees with the model
over a precisely stated evidence boundary.

## 10. Recommendation

Charter the research, if desired, as three separately valuable gates rather
than one monolithic ATT--OTT project:

1. paper calculus plus executable model;
2. machine-checked safety theorems; and
3. compiler-trace correspondence.

The first gate is already justified and should be the initial commitment. The
second is justified if the first yields a clean two-level model. The third
should proceed only if evidence extraction can remain independent of the ledger
being checked.

Do not couple the campaign to a production refactor. Its result can later inform
the obligation-IR refactor proposed in
[the compiler refactor report](compiler-obligation-ir-refactor.md), but the
research should first supply the semantic contract against which such a
refactor would be judged.

## Sources

- [A linear causal-obligation calculus for compiler lowering](linear-causal-obligation-calculus.md)
- [Affine and observational type theory in Ken's compiler](causal-obligation-calculus.md)
- [`ContinuationCandidateLedger`](../crates/ken-runtime/src/cranelift_backend/lowering/units.rs)
- [`ContinuationDischarge`](../crates/ken-runtime/src/cranelift_backend/lowering/mod.rs)
- [`ContinuationCallIdentity`](../crates/ken-runtime/src/cranelift_backend/planning/static_transition.rs)
- [`RT-CONTINUATION-EDGE-DISPOSITION`](../docs/program/wp/RT-CONTINUATION-EDGE-DISPOSITION.md)
- [ADR 0021](../docs/adr/0021-resource-lifetime-and-ward-delegation.md)
