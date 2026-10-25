# Ken specification — chapter status index

This file is the human-readable reconciliation index for the Ken
specification. It records what each chapter says about its own provenance or
delivery stage and how strongly that chapter says its text binds. Sequencing
readers use those declarations and their stated scope; this index does not
invent a separate maturity judgment.

The chapters remain authoritative for their own declarations. This index is a
measured view of all 78 Markdown files under `spec/` at this update. It is not
a build tracker, a next-action queue, or a substitute for the work-package
nodes under `docs/program/issues/`.

Implementation gap, route C (`LANG-REFINEMENT-SUBSET-SIGMA`): the checked
scrutinee equation required in recursive unindexed constructor branches is
not delivered. `LANG-RECURSIVE-BRANCH-EVIDENCE-BELOW` must deliver it. This
is an as-built conformance gap, not a weakening of `20-verification/22 §3`
or `30-surface/34 §3.3`; those contracts remain normative.

## Status convention

This index adopts **shape B: provenance/stage plus binding force**. The former
`TODO · DRAFT · REVISED · DONE` ladder is retired because no rule derives
drafting maturity from declarations such as `K1 elaborated`, `impl-ready (L2)`,
or scope-qualified `Normative`. In particular, an `elaborated` work-package
marker says who or what elaborated a chapter; it does not prove that the
chapter passed through a hidden `REVISED` rung.

Shape A would require that unsupported maturity mapping. Shape C would call
this table derived without supplying a derivation mechanism, and a
declaration-only derivation would omit the three status-less files. Shape B
instead keeps one explicit, total inventory while using only quantities the
chapter declarations actually state.

Apply the convention as follows:

1. Read the declaration's complete opening status sentence, not its first
   Markdown-emphasized span. Record every clause in that sentence that
   predicates a provenance or delivery stage of the chapter or one of its
   named sections. Stage predicates are `DRAFT vN`, `elaborated`, a WP that
   `extends` or `completes` a fragment, `impl-ready` or
   `implementation-ready`, `contract-pinned`, `implementation-gated`,
   `CAPSTONE COMPLETE`, and an explicitly numbered audit phase. Preserve
   adjacent qualifiers that identify the WP, section, series, or delivery
   scope. Normalize multiple clauses into reading order with semicolons; do
   not translate one predicate into another.
2. Read binding force from the same declaration. Statements that text is
   normative, fixes or pins a contract, records a settled rule, or defines an
   operative boundary bind at the scope they name. Proposal-level, starter,
   `(oracle)`-tagged, deferred, and implementation-gated portions remain
   qualified exactly as the declaration says.
3. A `DRAFT` marker does not weaken an explicitly normative contract, and an
   `impl-ready` or `elaborated` marker does not strengthen a proposal-level
   spelling.
4. `Normative`, proposal-level, and operator-facing role statements belong to
   the binding-force axis, not the provenance/stage axis. If a declaration
   supplies force but no stage predicate, record `not declared` for the first
   axis. If a file has no `> Status:` declaration, assign neither axis by
   inference and record it in the unclassifiable report.

This convention has no unreachable rung: it preserves declarations that
chapters actually use and states their force independently. It adds no checker
or generated-status mechanism.

## Per-file inventory

The middle column applies the semantic provenance/stage rule to each file's
own `> Status:` declaration. The final column applies the binding-force rule
to that same declaration, including its qualifications.

| File | Declared provenance/delivery stage | D1 binding-force assignment |
|---|---|---|
| `00-overview.md` | `DRAFT v0` | Normative for terminology and scope; subject chapters prevail on disagreement. |
| `10-kernel/11-syntax.md` | `K1 elaborated` | Normative for kernel grammar and scoping; typing is assigned to `13`–`16` and `18`. |
| `10-kernel/12-universes.md` | `K1 elaborated` | Normative for the universe hierarchy, checking, and level polymorphism; K1 scope is §§1–4 and §6. |
| `10-kernel/13-pi-sigma.md` | `K1 elaborated` | Normative for Π and Σ rules and K1-scoped conversion. |
| `10-kernel/14-inductive.md` | `K1 elaborated; K1.5 extends it; nested-positive partially landed` | Normative for inductives, positivity, and elimination; fresh/composed admission and both selector sorts execute, while individually marked generated-family/method/topology/sort residuals remain under `KERNEL-NESTED-IND`. |
| `10-kernel/15-identity.md` | `K2 elaborated` | Normative for the observational-equality interface. |
| `10-kernel/16-observational.md` | `K2 elaborated; K2c series-2 completes the three obs-reduction seams (§3.2 inductive index rewrite, §4.1 non-constant-motive J, §5.1 full quotient respect); K5 completes the observational fragment` | Normative for the observational interface, computation rules, and reduction behavior; `(oracle)` rules retain that qualification. |
| `10-kernel/17-conversion.md` | `K2c elaborated (series 1 — conversion-hardening)` | Normative for kernel conversion and the SCT gate. |
| `10-kernel/18-judgments.md` | `K-api elaborated; implementation-ready` | Normative kernel API contract; its freeze claim is conditional on the declared landed admission gates. |
| `10-kernel/18a-primitive-registry.md` | `Phase 1 — registry audit (WS-K / BUILTINS)` | Conditional native registry seal: an operation is native iff its stated correctness AC holds. |
| `10-kernel/README.md` | `K2 elaborated` | Normative WS-K contract and WS-V re-checking target. |
| `20-verification/21-spec-syntax.md` | `V1 elaborated; implementation-ready` | Normative for verification forms, meaning, elaboration, and status model; concrete spelling is cross-referenced. |
| `20-verification/22-obligations.md` | `V2 elaborated; implementation-ready` | Normative for obligations and their extraction algorithm. |
| `20-verification/23-prover.md` | `V3 specified` | Normative for the prover contract, verdicts, classifier, and certificate route; the route-(a) theorem statements are required, while their artifact placement and resulting evaluator/TCB boundary remain an Architect/operator decision. |
| `20-verification/24-diagnostics.md` | `V4 elaborated; implementation-ready` | Normative for diagnostic mechanisms and meaning; serialization belongs to `25-protocol.md`. |
| `20-verification/25-protocol.md` | `T1 elaborated; implementation-ready` | Normative for message shape, verdict cross-walk, and stability; exact JSON names remain a finalization surface. |
| `20-verification/README.md` | `DRAFT v0` | Normative for the verification interface and soundness obligations; named prover internals remain team work. |
| `30-surface/30-taxonomy.md` | not declared | Normative for the built-in/prelude/package boundary and minimality invariant; membership grows through its derivation process. |
| `30-surface/30-intrinsic-ledger.md` | not declared | Open evidence ledger, not a floor-admission authority; the normative floor remains in `30-taxonomy §4`. |
| `30-surface/31-lexical.md` | `DRAFT v0`; reserved-infix admission contract-pinned | Settled for `OQ-syntax` principles; the exact six-name reserved-infix admission binds, while the remaining concrete token table is a revisable starter. |
| `30-surface/32-grammar.md` | `DRAFT v0`; reserved-infix grammar contract-pinned | Normative intent for which productions exist; the six-name operator-name spellings bind, while other exact spelling remains proposal-level. |
| `30-surface/33-declarations.md` | not declared | Normative for features and the named module, visibility, and class contracts; concrete spelling remains proposal-level. |
| `30-surface/34-data-match.md` | `impl-ready (L2)` | Normative for the feature; nested-positive admission and selectors are partially landed, with the binary residual dependent-method re-check separately staged. |
| `30-surface/35-numbers.md` | `impl-ready (L1)` | Normative for the numeric model and L1 scope; user-defined numeric instancing remains gated on L-classes. |
| `30-surface/36-effects.md` | `L5 elaborated; implementation-ready for Team Language` | Normative for the model and elaboration; concrete spelling remains proposal-level. |
| `30-surface/37-strings-collections.md` | `impl-ready (L3)` | Binding for concepts, laws, lowering, and staging; exact API spelling and internal representations remain `(oracle)`-tagged. |
| `30-surface/38-ffi-io.md` | `§1.1–§1.6 impl-ready (L6); §1.7 contract-pinned (PX8-T); §2–§4 impl-ready (L7); elaborated to team-ready rigor` | Normative for trust and effect discipline with the stated per-section delivery stages. |
| `30-surface/39-elaboration.md` | `DRAFT v0; §5 (V0) elaborated to implementation rigor for the G1 minimal slice` | Normative for elaboration output and guarantees; §5 is V0 implementation-rigorous and the other named sections remain frame-level. |
| `30-surface/README.md` | `DRAFT v0` | Normative for constructs and meaning; concrete syntax remains an explicitly revisable proposal except where a subject chapter marks a spelling contract-pinned. |
| `40-runtime/41-values.md` | `Elaborated (X2 contract)` | Normative for values, equality, callable boundaries, and durable canonical encoding; in-process representation is private. |
| `40-runtime/42-evaluation.md` | `X1 elaborated; implementation-ready for Team Runtime; pure-core G1 scope` | Normative reference-evaluation contract for the declared pure-core and effect-driver scope. |
| `40-runtime/43-termination.md` | `DRAFT v0` | Normative for the totality/partiality boundary. |
| `40-runtime/44-capacity.md` | `Elaborated (X2 contract)` | Normative for observable capacity failure, logical isolation, and semantics-invisible reclamation; storage organization is private. |
| `40-runtime/45-native-backend.md` | `DRAFT v0 (spec-design)` | Binding backend design for model, trust posture, and differential discipline; target choice remains open. |
| `40-runtime/46-checked-core-package.md` | `DRAFT v0 (NC1)` | Normative for the stable checked-core compiler input and its stated exclusions. |
| `40-runtime/47-erasure-runtime-ir.md` | `DRAFT v0 (NC5)` | Normative for erasure, runtime IR, loud refusal, and interpreter comparison; backend details are excluded. |
| `40-runtime/48-executable-artifact-contract.md` | `DRAFT v0 (NC19)` | Normative for the executable-artifact contract and loud rejection rules; emission and ABI details are excluded. |
| `40-runtime/README.md` | `DRAFT v0` | Normative for the value/equality/callable/reference-semantics contract; private representation choices remain outside it. |
| `50-stdlib/51-lawful-classes.md` | `DRAFT v0 (ES4-classes)` | Binding pattern for lawful catalog entries: ordinary Ken, proved laws, and zero trusted-base delta. |
| `50-stdlib/52-map.md` | `DRAFT v0 (VAL2 #8 / OQ-A)` | Binding operator-locked Map shape and proof discipline; the named staged proof history remains scoped as declared. |
| `50-stdlib/53-transport.md` | `DRAFT v0 (surface-transport WP, Map Gap A)` | Binding library contract for five derived equality combinators over `J`; no new former or trusted-base entry. |
| `50-stdlib/54-map-verified-laws.md` | `DRAFT v3 — CAPSTONE COMPLETE: all five inductive laws landed` | Binding proof-strategy and landed-capstone record for five inductive laws; permutation remains explicitly out of scope. |
| `50-stdlib/55-lawful-functors.md` | `DRAFT v0 (CAT-1)` | Binding CAT-1 pattern for value algebra and constructor classes; no new kernel feature. |
| `50-stdlib/56-effectful-classes.md` | `DRAFT v0 (CAT-2)` | Binding CAT-2 contract with the declared SURF-1/SURF-2 gates for Traversable. |
| `50-stdlib/57-collections-and-views.md` | `DRAFT v0 (CAT-3)` | Binding CAT-3 contract for collection laws and views; build work remains staged as declared. |
| `50-stdlib/58-maps-sets-relations.md` | `DRAFT v0 (CAT-4)` | Binding CAT-4 contract for keyed collections, sets, and the relations frontier; build scope remains split as declared. |
| `50-stdlib/58a-priority-queues.md` | `LANDED COMPUTATION + GENERAL LAWS (CAT-PRIORITY-QUEUE and CAT-PRIORITY-QUEUE-LAWS)` | Normative for the abstract six-name interface, priority-queue semantics, validity obligations, and structural cost account; the computational implementation and private general semantic proof suite are landed, while machine-checked complexity remains separate. |
| `50-stdlib/58b-membership.md` | `DRAFT v0 (SPEC-MEMBERSHIP-CLASS-CONTRACT)` | Binding contract for the `Membership` class, standard providers and law model; `∈` spelling and completion policy are specified elsewhere. |
| `50-stdlib/59-parsing-syntax-diagnostics.md` | `DRAFT v0 (CAT-5)` | Binding CAT-5 package contract with the stated compiler/reflection exclusions. |
| `50-stdlib/60-length-indexed-vectors.md` | not declared | Normative for the Vec family and total indexed operations; `zip` and `lookup` now check in the catalog Vector module, so `DS-5c` is not a live gate. |
| `50-stdlib/61-formal-languages.md` | `DRAFT v0 (SPEC-FORMAL-LANGUAGES-DFA-CONTRACT; SPEC-FORMAL-LANGUAGES-FINITE-REACHABILITY-CONTRACT; SPEC-FORMAL-LANGUAGES-NFA-CONTRACT; SPEC-FORMAL-LANGUAGES-REGEX-CONTRACT; SPEC-FORMAL-LANGUAGES-MINIMISATION-CONTRACT; SPEC-FORMAL-LANGUAGES-LEXER-BRIDGE-CONTRACT)` | §§1–2 bind Dfa, finite certificates and reachability; §3 binds Nfa acceptance and finite-mask determinization; §4 binds independent Regex denotation and six checked laws; §5 binds Dfa equivalence and minimisation without `DecEq q` or a cardinality claim. §6 binds Cursor's bounded element list and the streaming DFA bridge, with four checked laws and no cross-instance cursor claim. §§5–6 package delivery is not claimed. |
| `50-stdlib/62-graphs.md` | `DRAFT v0 (SPEC-GRAPHS-DEPENDENCY-CONTRACT §1; SPEC-GRAPHS-CYCLE-REFUTES-ORDER §2; SPEC-GRAPHS-WALK-COMPOSITION-ORDER §3; SPEC-GRAPHS-WALK-APPEND-UNIT-ASSOC §4)` | §1 binds Boolean adjacency, finite reachability via §61 §2's fixed point, real walks, and a complete distinct topological list or real cycle; the §1 catalog module and seed are present on base. §2 adds three generic checked laws: `before_irreflexive` and `before_transitive` need no uniqueness, coverage or DecEq-law premises, and `cycle_refutes_order` uses only `all_edges_forward` to refute a cycle. The §2 catalog proofs and seed are present on base; CV exercised its seed blocks separately as scratch clients, not through a shipped seed harness or CI. §3's `walk_append` and `ordered_walk_forward` now have checked catalog exports and a package-owned seed test; these are not official conformance-runner execution, CI or a landed trust measurement. §4 states generic `walk_append_here_left` and `walk_append_assoc` over walk equality, with no right-unit theorem because that law reduces. Architect D0 on `d4f2b202c` checked both generic proofs and a client with no new `Axiom` and scratch trust identities 114 → 114. A closed §4 seed is authored; the §4 catalog exports and shipped-seed execution, official conformance run, CI, publication, landing and landed trust are not established by this candidate. |
| `50-stdlib/63-statistics.md` | `DRAFT v0 (SPEC-STATISTICS-EXACT-DISTRIBUTION-CONTRACT; SPEC-STATISTICS-EMPIRICAL-SAMPLE-CONTRACT; SPEC-STATISTICS-RATIONAL-VALUES-CONTRACT; SPEC-STATISTICS-EVENT-MASS-INCLUSION-EXCLUSION §4)` | §1 binds positive natural-weight atom lists and nine checked laws for event/score numerators; §2 binds `NonEmpty q` empirical samples and seven bridges/append laws. §3 specifies Rational-valued probability, expectation and empirical mean in those modules, with twelve additional proved laws for numerator/denominator agreement, forward cross bridges, bounds and complements; no §3 CAT delivery is claimed. §4 adds public event intersection, additive Nat inclusion-exclusion and event/score monotonicity with three checked candidate catalog proofs; AC-1 does not author a §4 conformance seed or claim CI or landed trust measurement. `pure`/`bind`, uniform enumeration, variance and approximate statistics remain deferred. |
| `50-stdlib/64-rational.md` | `DRAFT v0 (SPEC-NUMERIC-RATIONAL-CONTRACT)` | Normative for exact non-negative rational values as unnormalized natural/positive-denominator pairs, cross-product `Prop` equivalence, total arithmetic and Boolean order with checked relation/respect/algebra/order laws; the Nat provider exports positive-factor cancellation and order laws. The Rational and Nat packages are landed; §64 itself does not supply a value-typed statistics adapter, specified separately in §63 §3. No quotient or Gcd public API is claimed. |
| `50-stdlib/65-linear-algebra.md` | `DRAFT v1 (SPEC-LINEAR-ALGEBRA-DIMENSION-SAFE-CONTRACT §1; SPEC-LINEAR-ALGEBRA-PRODUCT-LAWS §2; SPEC-LINEAR-ALGEBRA-PRODUCT-COMPATIBILITY §3; SPEC-LINEAR-ALGEBRA-MATRIX-SUM-DISTRIBUTIVITY §4; SPEC-LINEAR-ALGEBRA-SUM-COMM-ASSOC §5)` | §1's seven `Nat`-valued dimension-safe operations and five laws, §2's two product laws, and §3's two compatibility laws and public `dot_scale` are catalog exports with checked bodies. The existing seed includes closed §3 cases; this row claims no fresh run, CI or landed trust measurement. §4's row-wise `matrix_add` and four generic distribution/transpose laws are now catalog exports with checked bodies and a package-owned seed test. Architect's historical §4 D0 had rc 0, no new Axiom, 109 → 109 exact trust identities but its census did not reach a verdict; that D0 was not shipped execution or landed trust. §5 specifies four new public checked-law obligations: `vector_add_comm`, `vector_add_assoc`, `matrix_add_comm`, `matrix_add_assoc`, over all dimensions and zero indices, with matrix laws derived row-wise from vector laws. Architect D0 checked their generic proofs on `4e7cf78a6` with no new `Axiom`, unchanged scratch trust identities 114 → 114, and four private helpers. The §5 catalog exports are not delivered by this spec/seed candidate; neither D0 nor author overlay checks establish shipped-seed execution, CI, publication, landing or landed trust for §5. Signed and approximate tiers remain deferred. |
| `50-stdlib/66-symbolic-algebra.md` | `DRAFT v0 (SPEC-SYMBOLIC-ALGEBRA-CONTRACT §1; SPEC-SYMBOLIC-ALGEBRA-CANONICAL-EQUALITY §2; SPEC-SYMBOLIC-ALGEBRA-SUBSTITUTION §3; SPEC-SYMBOLIC-ALGEBRA-SUBSTITUTION-COMPOSITION §4; SPEC-SYMBOLIC-ALGEBRA-DERIVATIVE §5)` | §1 binds landed `Nat`-coefficient `Expr`, ordered distributed `Monomial` lists and five checked normalization laws. §2's separate canonical Horner `Poly`, Boolean `poly_eq` decision, five checked laws, catalog implementation and seed are also present on main. §3 adds public `compose_env` and one-pass `substitute` with evaluation under the composed environment and forward-only preservation of a `True` poly_eq answer; its catalog extension and seed are present on main, but D0 trust 114→114 does not establish shipped seed-harness execution, CI or landed trust measurement. §4 specifies public `compose_subst`, structural `substitute_compose` and constructor-identity `substitute_var`; Architect D0 checked the generic proofs with no `Axiom` and trust identity sets 114→114 in scratch. §4 catalog exports and seed are present on main; its D0 alone does not establish shipped seed-harness execution, CI or landed trust measurement. §5 specifies structural `deriv`, generic `deriv_respects_eval` for universal equality, and forward-only `poly_eq_deriv`. Architect D0 checked the Taylor-bump/remainder-extraction route with rc 0, no new `Axiom`, and identical five-provider trust sets 114→114. The §5 conformance seed is authored as a separate source candidate; §5 catalog exports are absent on its base. Shipped-seed execution, CI, publication, landing and landed trust are not claimed. Sorted-list canonicalization and signed/field operations remain deferred. |
| `50-stdlib/67-geometry.md` | `DRAFT v0 (SPEC-GEOMETRY-EXACT-CONTRACT §1; SPEC-GEOMETRY-MANHATTAN-METRIC §2; SPEC-GEOMETRY-BOX-CONTAINMENT §3; SPEC-GEOMETRY-BOX-AREA §4; SPEC-GEOMETRY-BOX-INSIDE-ORDER §5)` | §1's `NatBox`, six laws, seed and required Nat Order bounds are landed. §2's triangle and zero-distance laws, catalog code and seed are also present on main. §3 specifies public corner-wise Boolean `inside` and seven checked laws: soundness for point membership, reflexivity, each intersection inside its operands, each operand inside their hull, and a point-witnessed converse. An inverted inner box may have vacuous point-set inclusion without `inside`, so no unwitnessed converse or canonical empty representation is claimed; axis helpers stay private. Architect D0 checked §3 in scratch with no Axiom, trust identity sets 114 → 114, ambient census 1/1 with no residuals. The §3 seed, `inside` and seven law exports are present on main. §4 specifies public natural-valued `area`, `area_inside` without an outer-validity premise, and generic `area_inverted_x`/`area_inverted_y` zero laws. Intersection-area bounds are corollaries of `area_inside` and §3.2, not extra exports. Architect checked the §4 generic laws on scratch D0 with rc 0, no local Axiom and exact trust identity sets 114 → 114; §4 catalog exports and seed are present on main, with a package-owned test executing the seed fences and measuring unchanged trusted identities; this is not official conformance-runner execution. §5 specifies three unconditional laws over corner-wise `inside`: `inside_trans`, `intersect_greatest`, and `hull_least`, including inverted boxes. Architect checked generic proofs in scratch with `ken check` rc 0, no new Axiom and 114 → 114 identical trusted GlobalIds. The §5 catalog exports are not yet delivered; its authored seed and isolated D0 overlay checks do not establish shipped-seed execution, CI, publication, landing or landed trust for §5. New per-axis order helpers stay private; `inside`, `intersect`, `hull`, `area` and their earlier laws are unchanged. Signed and approximate tiers, higher dimensions, canonical empty boxes and point-set equality remain deferred. |
| `50-stdlib/68-numerical.md` | `DRAFT v0 (SPEC-NUMERICAL-RATIONAL-INTERVAL-CONTRACT §1; SPEC-NUMERICAL-INTERVAL-HULL §2; SPEC-NUMERICAL-INCLUSION-ISOTONICITY §3; SPEC-NUMERICAL-INTERVAL-DISTRIBUTIVITY §4)` | §1–2's interval catalog operations, ten checked laws, two closed seeds and required Rational monotonicity/min-max exports are present on main. §3 specifies public endpoint-wise Boolean `interval_inside` and nine checked laws: member soundness, reflexivity, addition and multiplication isotonicity, hull-left/right and intersection-left/right inclusions, and a converse requiring an inner-interval member witness. Inverted empty intervals may have vacuous member inclusion without endpoint inclusion; no unwitnessed converse, structural interval algebra, canonical empty interval or signed tier is claimed. Architect D0 checked §3 on scratch; the Boolean operation and nine laws are now catalog exports with checked bodies and no new `Axiom`/`Opaque`. The §3 seed at `conformance/stdlib/numerical/seed-interval-inclusion.md` runs in targeted package acceptance, including both independent refusals. Exact candidate trusted GlobalId sets match at 114 → 114 and the ambient census row remains unchanged; CI, publication and a landed trust measurement are not claimed. §4 specifies four checked laws giving mutual endpoint-value inclusion for both orientations of interval multiplication over addition, without structural `Equal Interval` or an interval-commutativity law. Architect AC-0 checked the four generic proofs in scratch (rc 0, no `Axiom`), but this spec-only candidate does not deliver §4 catalog exports or seed and does not establish shipped-seed execution, CI, publication, landing or landed trust. |
| `50-stdlib/69-time-and-events.md` | `DRAFT v0 (SPEC-TIME-EVENT-TRACE-CONTRACT §1; SPEC-TIME-WINDOW-MEMBERSHIP §2; SPEC-TIME-WINDOW-MERGE-DISTRIBUTION §3; SPEC-TIME-MERGE-MEMBERSHIP §4)` | §1–2's `Data.Time.EventTrace` operations, five checked laws, two generic filter prerequisites and two existing seeds are present on main. §3 specifies public `trace_window_merge` as structural `Equal (List (Event p))` between window-of-merge and merge-of-windows, requiring `time_ordered p xs` and `time_ordered p ys`. Either one-sided-order weakening fails on `[5,1]`, `[3]` in window `[0,4)` or its swapped counterpart. The count consequence follows under the same premises from this law and §1.3; no premise-free structural window/merge equality or count-distribution law is claimed. Architect D0 checked the generic law in scratch (rc 0, no `Axiom`, zero added trust reported); the checked §3 catalog export is present and package acceptance runs the authored seed at `conformance/stdlib/time-and-events/seed-window-merge-distribution.md`; CI and a landed trust measurement are not established by this status. §4 specifies `trace_merge_member_sound`, `trace_merge_member_left` and `trace_merge_member_right` over Ω-valued truncated list membership, with no ordering premise or payload equality decision. A one-way window-of-merge soundness consequence follows privately from §2.3 and §4, without another public law. Architect AC-0 checked all three generic proof bodies and that private corollary in scratch (rc 0, no code `Axiom`); this spec-only candidate does not deliver §4 catalog exports or seed, shipped-seed execution, CI, publication, landing or a landed trust measurement. Clock and causality effects remain outside scope. |
| `50-stdlib/70-diagnostic-codes.md` | `DRAFT v0 (SPEC-DIAGNOSTIC-CODE-CONTRACT)` | §1 binds controlled batch `ken` failure-code presence, exact five-character ASCII grammar, carrier-selected C/E/K/X/N/H domains, append-only ledger identity, seven N families, one-code stderr lines and coded-failure exit 1. Admitted `ken run` program outcomes are uncoded; multiple diagnostic lines are allowed; CLI JSON, warnings and elaboration-recovery ordering remain deferred. This tooling interface is not a Ken package; no renderer or shipped seed is claimed. |
| `50-stdlib/71-serialization.md` | `DRAFT v0 (SPEC-SERIALIZATION-ROUND-TRIP-CONTRACT)` | §1 binds a pure token codec over the existing six-constructor `Json`, with opaque `String`/`Int` payloads, ordered arrays/objects and duplicate keys preserved. Four D0-checked theorems cover decode-after-encode and typed trailing, empty and stray-close refusals; this is Architect-reported scratch evidence, not a delivered codec or seed, CI, or measured trusted-base delta. The JSON text/byte and UTF-8 layer, generic class derivation, duplicate-key rejection, Merkle hashing, error positions and truncation-prefix laws are deferred. |
| `50-stdlib/72-optimization.md` | `DRAFT v0 (SPEC-OPTIMIZATION-SEARCH-CONTRACT §1; SPEC-OPTIMIZATION-ARGMIN-OPTION §1.4; SPEC-OPTIMIZATION-BOUNDED-SEARCH §2; SPEC-OPTIMIZATION-ARGMAX §3; SPEC-OPTIMIZATION-ARGMAX-OPTION §3.4)` | §1 binds `argmin` on `NonEmpty a` under `a → Nat`, selecting the earliest minimum without `DecEq a`; its seven laws and seed are present on main. Section 1.4's `argmin_option` and four laws with `Nil`/`Cons` equations and direct-to-List corollaries are present on main. §2's finite strict-bound `find_below` and three laws and seed are present on main. §3 binds `argmax` and seven laws selecting the earliest greatest-cost member of `NonEmpty a`; these catalog exports and their seed are now present on main. §3.4 specifies `argmax_option : (a : Type) → (a → Nat) → List a → Option a`, with exact `Nil`/`Cons` equations and two direct-to-List corollaries conditional on `Some y`, using the same earliest-maximum tie rule. Architect checked the four generic laws and closed probes in scratch (rc 0, no new `Axiom`, 114→114 identical trusted identities); an isolated D0 overlay accepted this candidate's positive seed block and rejected its five controlled false blocks with K0004. This candidate delivers neither §3.4 catalog exports nor shipped-seed or official conformance execution; scratch checks do not establish CI, publication, landing, or a landed trust measurement. Continuous and unconstrained search, heuristics, approximations, arbitrary-order or multi-objective costs, top-k/stable multi-result selection, and complexity claims remain deferred. |
| `50-stdlib/README.md` | `DRAFT v0` | Binding standard-package-tier boundary and derivation-path requirement. |
| `60-security/61-information-flow.md` | `Sec1 + Sec1ct elaborated; implementation-ready for WS-Sec` | Normative for the declared IFC and constant-time discipline; surface spelling remains proposal-level. |
| `60-security/62-authority.md` | `Sec2 elaborated; implementation-ready for Team Verify (WS-Sec)` | Normative for authority, attenuation, revocation contract, and audit points; spelling and named runtime mechanisms remain qualified. |
| `60-security/63-supply-chain.md` | `DRAFT v0` | Normative for consumption and artifact shape under the declared provenance decision. |
| `60-security/64-trust-model.md` | not declared | Normative for the TCB, security reading, trusting-trust invariant, and honest limits. |
| `60-security/65-policy.md` | not declared | Normative for policy shape, binding guarantee, and semantics; concrete policy syntax remains deferred. |
| `60-security/README.md` | `DRAFT v0` | Normative security frame, threat model, guarantees, and limits. |
| `70-behavioral/71-assumption-boundary.md` | `impl-ready (B1)` | Normative for projection, schema, and seam discipline; literal wire spellings remain `(oracle)`-tagged. |
| `70-behavioral/72-temporal.md` | `impl-ready (B2)` | Normative for the temporal-as-data discipline and export flow; listed encoding spellings remain `(oracle)`-tagged. |
| `70-behavioral/73-conformance.md` | `impl-ready (B3)` | Normative for Ken's runtime-seam half; downstream checking policy and literal wire spellings remain outside or qualified. |
| `70-behavioral/74-agentic.md` | `impl-ready (B4); WS-B capstone` | Normative for the agentic-assurance reduction; it adds no new agentic mechanism. |
| `70-behavioral/README.md` | `DRAFT v0` | Normative for Ken's half of the behavioral seam. |
| `90-open-decisions.md` | not declared | Operator decision register; draft recommendations do not resolve the genuine forks they record. |
| `README.md` | no declaration | Unclassifiable: no provenance/stage or binding force is declared. |
| `SPEC-PROGRESS.md` | no declaration | Unclassifiable: this index has no chapter self-declaration. |
| `_notes/analysis-digest.md` | no declaration | Unclassifiable: the background note has no status declaration. |

## Unclassifiable report

Exactly three inputs lack a `> Status:` declaration and therefore receive no
inferred axis value:

- `README.md` — the spec index declares neither provenance/stage nor binding
  force.
- `SPEC-PROGRESS.md` — this reconciliation index is not a self-declaring
  chapter.
- `_notes/analysis-digest.md` — the background design note declares no status.

The other 75 inputs are classifiable by the operative rule. This report does
not repair or add chapter declarations; those declarations are inputs to this
reconciliation.

## Reconciliation controls

- **Different declaration shapes:** `10-kernel/18-judgments.md` records both
  `K-api elaborated` and its suffix qualifier `implementation-ready`, then
  binds the kernel API contract; `30-surface/34-data-match.md` records
  `impl-ready (L2)` and binds its feature; `60-security/65-policy.md` has a
  force-only `Normative` declaration, so its first axis is `not declared`
  while its policy shape, guarantee, and semantics bind and syntax stays
  deferred. Each result follows the same two-axis rule.
- **Selection controls:** `10-kernel/16-observational.md` retains the K5 clause
  outside its first bold span; `30-surface/39-elaboration.md` retains its
  section-specific V0 clause; `50-stdlib/57-collections-and-views.md` retains
  the adjacent `(CAT-3)` qualifier. Markdown emphasis selects none of them.
- **Changed and retained rows:** `10-kernel/11-syntax.md` previously read
  `DRAFT` in this index; its chapter declaration yields `K1 elaborated` plus
  normative grammar/scoping. `00-overview.md` previously read `DRAFT`; its
  declaration retains the `DRAFT v0` marker while exposing its normative
  terminology/scope. Neither result is copied from the old row.
- **No `REVISED` inheritance:** the retired ladder and its unused `REVISED`
  rung do not appear in the operative vocabulary.
- **No gate:** this reconciliation changes one Markdown artifact. It adds no
  CI configuration, script, test, or documentation-content oracle.
