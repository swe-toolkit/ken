# Nested strictly-positive inductive conformance

Format: `../../README.md`. These are the conformance obligations for the
nested-positive extension of `spec/10-kernel/14-inductive.md`: structural
admission through checked strictly-positive parameter positions, intrinsic
source-indexed `All^Type` / `All^Omega` families whose leaves cover every
contained recursive occurrence, nested ι, and the fail-closed boundary.

Ground truth: `14 §3.2` (lifted IH), `§7.8` (nested ι), `§8.5` (structural
parameter-polarity rule), and `§9.5` (subject reduction, termination, and
required population). The stable declaration entry is `18 §4.3`. Frame:
`docs/program/issues/SPEC-NESTED-IND.md`; build node:
`docs/program/issues/KERNEL-NESTED-IND.md`. Clean-room: derived from those
Ken-owned objects and first principles; no reference implementation or
`local/refs/` source was consulted.

Cases marked `[KERNEL-NESTED-IND]` remain design-locked and
implementation-gated. DS-9, fresh-carrier, and composed-carrier admission
execute un-gated, as do negative-under-positive and transparent-Sigma-negative
rejection, unknown-head and nonpositive rejection, and unchanged direct/W-style
behavior. The former bare-Pair vectors use a fresh checked transparent Sigma
alias here; the canonical named floor-Pair instantiation is RED-UNTIL the floor
realization pinned in `../../surface/modules/seed-pair-strict-boundary.md`.
`nested-size-uses-lift` also executes un-gated through the named full-pipeline
witnesses below. Both
structural-result selector sorts are landed. The unary Ω residual positive and
the binary final-recheck transition sentinel execute in
`lang_structural_result_elab.rs`. `nested-dependent-motive-uses-lift` remains
gated only at that stronger binary-residual generated-`All` method re-check.
Independently gated non-D6 residual cases remain marked. The existing direct
and W-style controls remain live throughout, so staging the residual
completeness class does not suspend the positivity posture.

The custom positive carrier used below is deliberately not a standard-library
name:

```ken
data Bag (A : Type 0) : Type 0 where
  empty : Bag A
  one   : A -> Bag A
  join  : Bag A -> Bag A -> Bag A

data Rose : Type 0 where
  leaf : Rose
  node : Bag Rose -> Rose

data Box (A : Type 0) : Type 0 where
  box : A -> Box A

data Slot (A : Type 0) : Type 0 where
  vacant   : Slot A
  occupied : A -> Slot A
```

Each carrier parameter is checked strictly positive from its own constructor
telescopes. No later rule may recognize `Bag`, `Box`, or `Slot` by name.

---

## AC1 — structural admission, never a constructor-name allow-list

Spec: `14 §8.5` clauses 1–3.

### kernel/inductive/nested-ds9-shapes-admitted

Status: executing binding established in
`crates/ken-kernel/tests/nested_inductives_remaining.rs` as
`checked_transparent_sigma_aliases_admit_renamed_nested_paths`. The test admits
two distinct ordinary `declare_def` aliases with the same Sigma body and sends
both through the production positivity path.

- spec: `14 §8.5`; `18 §4.3`
- given: previously admitted positive `List` plus a fresh checked transparent
  `Product A B = (x : A) × B`, followed by an ordinary `Json` declaration
  containing both `JsonArray : List Json -> Json` and
  `JsonObject : List (Product String Json) -> Json`
- expect: **accepted** by `declare_inductive`; `Json`, its constructors, and
  `elim_Json` are admitted
- why: the first path follows `List`'s checked strictly-positive parameter. The
  second then transparently reduces Product to primitive Sigma and reaches Json
  in its positive second component. Renaming Product while preserving its body
  leaves the verdict unchanged. This is the concrete DS-9 consumer and proves
  finite structural paths compose; the custom-carrier case below independently
  prevents a `List` allow-list from satisfying the corpus. The canonical floor
  Pair must instantiate this same representation-derived arm; its named
  floor-realization vector is homed in the Pair-boundary seed.

### kernel/inductive/nested-fresh-carrier-admitted

- spec: `14 §8.5` clauses 1–3; `18 §4.3`
- given: admit the `Bag A` declaration above, then submit the `Rose` declaration
  through the same `declare_inductive` API without changing the kernel
- expect: **accepted**; `Rose`, `leaf`, `node`, and `elim_Rose` are generated
- why: the producer must derive `Bag`'s sole parameter as strictly positive when
  `Bag` is admitted, and the later `Rose` check must consume that recorded fact.
  **Verdict flip:** a name allow-list containing only pre-existing formers
  rejects this fresh `Bag Rose`; the structural rule accepts it. Conversely,
  accepting every application argument would fail AC4's controlled negative
  cases.

### kernel/inductive/nested-positive-chain-composes

Status: executing binding established in
`seed_fresh_bag_rose_and_deep_paths_admit_structurally`. The same production
test that admits fresh `Bag Rose` also admits `Bag (Wrap Deep)`.

- spec: `14 §8.5` clause 3
- given: after admitting `Bag`, admit a second fresh positive carrier
  `Wrap A` with `wrap : A -> Wrap A`, then declare
  `Deep` with `deep : Bag (Wrap Deep) -> Deep`
- expect: **accepted** and `elim_Deep` generated
- why: the path through `Bag`'s and then `Wrap`'s sole parameters is a finite
  composition of two independently checked positive positions. A checker that
  inspects only the immediate
  application or caps traversal at one former rejects this case, while the
  normative compositional rule accepts it.

---

## AC2 — the lifted IH is generated, reachable, and load-bearing

Spec: `14 §3.2`, `§7.8`, `§9.5`; `34 §3.1`; `39 §2.2`; `43 §1`.

### kernel/inductive/nested-method-structured-lift [KERNEL-NESTED-IND]

- spec: `14 §3.2`
- given: the generated `elim_Rose` for the `Bag`/`Rose` declarations above,
  under motive `M : Rose -> Type 0`
- expect: the `node` method receives the original `b : Bag Rose` and exactly
  one structured hypothesis
  `ih : All^Type_{Bag,0} (λr. M r) b`. The displayed family name is
  metanotation for a kernel-internal declaration, not a surface name. Its
  constructors preserve `b`'s `empty`/`one`/`join` topology and carry exactly
  one leaf of type `M r` for every contained `r : Rose`; the `empty` topology
  carries no evidence leaf
- why: this asserts the spec's newly locked representation granularity: one
  intrinsic, source-indexed `All` application per nested field, with leaves in
  bijection with dynamic recursive occurrences. **Structural discriminator:**
  a metatheoretic placeholder, a decorated copy of `b`, a single unstructured
  summary, or one leaf per constructor field produces a different method type.

### kernel/inductive/nested-iota-preserves-topology [KERNEL-NESTED-IND]

- spec: `14 §7.8`; `42 §3.3`
- given: `elim_Rose M ml mn (node b)` where
  `b = join (one r1) (join empty (one r2))`
- expect: the outer ι-step selects only `mn` and supplies
  `lift-elim_Rose(M, ml, mn, Bag Rose, b)`. Reducing that lift preserves the
  source index and constructs the aligned internal topology (metanotationally
  `all_join` / `all_one` / `all_empty`), placing `elim_Rose M ml mn r1` and
  `elim_Rose M ml mn r2` at its two recursive leaves
- why: nested ι is the admitted host former's constructor/eliminator traversal
  followed by `Rose` elimination at each child. **Structural discriminator:**
  dropping either leaf, changing the enclosing topology, evaluating an untaken
  `Rose` method, or passing `b` unchanged yields a different reduct. This pins
  reduction without over-specifying an internal runtime representation.

### kernel/inductive/nested-size-uses-lift

Status: executing binding established. The named kernel structured-IH/iota
witness and both full-pipeline surface witnesses below execute the selector and
its residual recursive `Bag.join` topology. The positive result is `3`; the
dropped-fold control reaches `1`.

- spec: `14 §3.2`, `§7.8`, `§9.5`; `34 §3.1.1`; `39 §2.3`, `§4`; `43 §1`
- given: define `size : Rose -> Nat` by the generated
  eliminator. The `leaf` method returns `1`; the `node` method folds the
  supplied `All^Type_{Bag,0} (λ_. Nat) b` inhabitant and adds `1`. In a
  `Bag.join xs ys` branch, combine `recursive result for xs` with
  `recursive result for ys`. Each operand must be the resolved surface binding
  whose checked method telescope supplies exactly one recursive result for the
  same field occurrence and support evidence. The association must be
  one-to-one in both directions, in range, and from that same method and support
  provenance. Evaluate
  `size (node (join (one leaf) (one (node empty))))`
- expect: **reduces-to `3`** after surface elaboration,
  kernel checking, erasure, and interpretation. The two contained children
  contribute `1` each, and the outer node contributes `1`. The selector emits
  each exact associated hidden result; it does not expose a hidden binder,
  change constructor-pattern arity, coerce an ordinary field reference, or
  reinterpret an owner self-call
- fail-closed boundary: a missing, duplicate, swapped,
  or foreign association rejects with the corresponding
  `StructuralResultAssociationMissing`, `...Duplicate`, `...Swapped`, or
  `...Foreign` diagnostic. Shadowing, copying, projecting, or merely reusing a
  spelling does not transfer an association. An ordinary resolved binding with
  no association rejects with `StructuralResultOutOfScope`; an unresolved
  operand rejects with `UnboundName`
- sort boundary: the selected hidden Nat result is
  classified by `Type`, so `induction hypothesis for xs` rejects with
  `RecursiveResultSortMismatch` naming `recursive result for xs` as the exact
  required spelling. If metavariables leave the selected result ambiguous
  between `Type` and `Omega`, `RecursiveResultSortAmbiguous` rejects without a
  guessed or default selector
- executing binding: the kernel structured-IH/iota witness is
  `production_nested_lift_is_consumed_and_iota_computes` and executing surface
  witnesses
  `nested_recursive_bag_rose_elaborates_checks_erases_and_interprets_at_nat_three`
  and
  `nested_recursive_bag_join_residual_folds_all_leaves_at_nat_three`. Those
  surface witnesses must use the selector through the full pipeline for both
  the Nat-3 `Bag`/`Rose` computation above and a deeper residual `Bag.join`
  topology. The source methods consume generated recursive results rather than
  finite-unrolling nested matches; the deeper witness requires those residual
  results rather than observing a non-consuming depth snapshot; and the `join`
  branch names and combines the independently associated results for both `xs`
  and `ys`, ruling out a one-sided header
- discriminating binding:
  `nested_recursive_bag_dropped_join_fold_reaches_nat_one` replaces the `join`
  fold by `Zero` while preserving the recursive constructor and well-typed
  lifted method; the full pipeline then observes `1`, so the two required
  `3`-result witnesses red under that fold mutation
- sort binding:
  `nested_recursive_bag_type_result_rejects_induction_hypothesis_spelling`
  reaches `RecursiveResultSortMismatch` and requires `recursive result for`.
  `RecursiveResultSortAmbiguous` has no production witness: `MetaCtx` solves
  levels only, zonking preserves the `Term::Type` versus `Term::Omega`
  constructor, and the reserved diagnostic has zero production construction
  sites
- why: this is the load-bearing value flip. With the correct lifted IH the
  result is `3`; a guard-deletion-only implementation that admits `Rose` but
  supplies no lift cannot type-check the definition, and an implementation that
  supplies a lift but drops/ignores its leaves computes `1`. Surface
  elaboration and termination must preserve the generated hypotheses; they may
  not reconstruct unrestricted self-calls.

### kernel/inductive/nested-dependent-motive-uses-lift [KERNEL-NESTED-IND]

Promise class: **transition sentinel**. The paired unary positive is a durable
invariant and remains after the binary sentinel retires.

Status: the selector is **not** the blocker. Both `recursive result for` and
`induction hypothesis for` are landed. The durable full-pipeline positive
`omega_selector_accepts_unary_residual_all_through_full_pipeline` proves that a
unary `ProofWrap xs` theorem accepts the Ω selector and passes the completed
method's kernel re-check. The stronger binary-residual case remains unavailable
at a later production seam, durably located by
`binary_omega_residual_method_recheck_is_a_transition_sentinel`.

The exact failing shape uses
`ProofJoin : ProofBag a -> ProofBag a -> ProofBag a`. Its proposition fold
accepts both `recursive result for xs` and
`recursive result for ys`. The corresponding theorem reaches both recorded
associations and both `induction hypothesis for` selections, then fails in
`elab.rs::check_match_with_lift` when the generated `All^Omega` method receives
its final `kernel_check`: the method conclusion and dependent branch goal report
a type mismatch. A unary residual variant passes the same full parse → resolve
→ elaboration → kernel-check pipeline, so this is not an Ω-selector or general
dependent-motive gate.

The existing NC14 control
`nested_dependent_motive_consumes_correlated_child_proofs` is a direct-leaf
association discriminator, not an executing binding for this row. Its flat
`ProofJoin : a -> a -> ProofBag a` gives every child `support: None`. The
landed Type-valued `Bag.join` size case proves two residual associations and
results can be consumed; the unary Ω case proves the proof selector can consume
a residual result. Neither proves that one dependent generated-`All` method can
combine **both** residual Ω results and still kernel-check.

⇒ **The residual is method construction after successful association and
selection.** The committed transition sentinel requires the exact final
`generated All method failed kernel re-check: type mismatch` refusal; an earlier
or different failure reds it. It needs a kernel-checkable dependent branch-goal
specialization for multiple residual host fields at `check_match_with_lift`,
plus a persistent full-pipeline binary positive and controlled wrong-result/
association negatives. Erasure is downstream of the current refusal and
supplies no evidence for this row.

- spec: `14 §3.2`, `§9.5`; `34 §3.1.1`; `39 §2.3`, `§4`
- given (executing transition sentinel): a dependent motive
  `AllGood : Rose -> Omega_0` whose `node` proof matches the `Bag Rose` field
  and its `All^Omega_{Bag,0} (λr. AllGood r) b` inhabitant in lockstep. In a
  `Bag.join xs ys` branch, `induction hypothesis for xs` and
  `induction hypothesis for ys` select the exact residual proofs associated by
  the checked method telescope with those surface bindings, field occurrences,
  and support evidence. Each association is one-to-one in both directions, in
  range, and from the same method and support provenance
- expect (required post-repair binding): **accepts** through elaboration and
  kernel checking. Each exposed `Rose` child is accompanied by its exact
  `AllGood child` proof; each residual `Bag Rose` value is accompanied by the
  correspondingly indexed residual `All^Omega` inhabitant. The leaf proofs
  remain irrelevant at their `Omega_0` proposition types, while the
  topology-carrying `All^Omega` application itself is in `Type 0`. The
  selector emits the exact associated hidden result without exposing hidden
  binders or changing ordinary direct and W-style match behavior
- fail-closed boundary (required post-repair binding): missing, duplicate,
  swapped, and foreign associations reject with the corresponding D0
  diagnostic. A same-spelled, copied, projected, or shadowed binding has no
  authority unless it independently carries exactly one validated
  same-occurrence association;
  neither its type nor an owner self-call may be used to guess one
- sort boundary (landed independently; required unchanged here): classify the
  selected hidden proof result, not its `All^Omega` support evidence. Its type
  is `Omega`-classified
  and therefore requires `induction hypothesis for`; the
  support application's residence in `Type 0` does not change that spelling.
  `recursive result for xs` rejects with `RecursiveResultSortMismatch` naming
  `induction hypothesis for xs` as the exact required spelling, and an
  unresolved `Type`-versus-`Omega` classification rejects with
  `RecursiveResultSortAmbiguous` rather than guessing
- why: a non-dependent recursor or an elaborator that binds the field but loses
  the correlated lift cannot construct the branch proof. This pins that the
  feature is induction, not merely a constant-result fold.

Stage audit: the composed-carrier marker is retired because its production
binding accepts `Bag (Wrap Deep)`. The `nested-dependent-motive-uses-lift`
marker remains solely for the binary-residual method-recheck gap above, not for
selector availability. Its transition sentinel retires exactly when the same
binary `ProofJoin` fixture returns `Ok`; that retirement candidate must replace
it with the durable binary positive and add independent wrong-result and wrong-
association negatives. An earlier/different error does not retire it. Heading
counts are not acceptance authority.

---

## AC2a — a neutral source uses one literal, method-independent family

Spec: `14 §3.2`, `§7.8`, `§9.5` item 5.

### kernel/inductive/nested-neutral-source-same-family [KERNEL-NESTED-IND]

- spec: `14 §3.2`, `§7.8`, `§9.5` item 5
- given: the admitted `Box` above, a context variable `v : Box Bool`, and the
  constant type-valued predicate `P = λx. Bool`
- expect: the generated method binder has the literal type
  `All^Type_{Box,0} P v`; after the guest method vector exists, the neutral
  `lift-elim` term also checks at exactly `All^Type_{Box,0} P v`. Neither side
  reduces on neutral `v`, and no equality or transport is inserted
- why: source indexing makes the two types syntactically the same family
  application before conversion. **Structural discriminator:** replacing the
  source index by a decorated copy, rebuilding the type from method terms, or
  using two separately generated host eliminators changes the neutral type and
  fails this exact-type check.

### kernel/inductive/nested-neutral-method-dependent-substitute-rejected [KERNEL-NESTED-IND] (soundness)

- spec: `14 §3.2`, `§9.5` item 5
- given: the same neutral `v : Box Bool`. As a mutation, replace the binder's
  `All^Type_{Box,0} (λx. Bool) v` with the first projection of the
  method-dependent decoder
  `(elim_Box (λ_. Sigma (T : Type 0). T)
             (λx. (Bool, true)) v).1`
- expect: **rejected with a type mismatch** when the generated lift inhabitant
  is checked against the mutated binder type. The neutral eliminator projection
  does not convert to `All^Type_{Box,0} (λx. Bool) v`
- why: the joint decoder is internally meaningful—its second projection checks
  at its first—but it cannot replace the public, method-independent family.
  **Controlled mutation:** `v`, the leaf type `Bool`, and the host topology are
  unchanged; only the binder's authority changes from the generated `All`
  declaration to a method-dependent neutral eliminator. Acceptance would
  silently reintroduce the representation-contract failure this revision
  resolves.

---

## AC2b — sort and level behavior is exact on both sides of the boundary

Spec: `14 §3.2`, `§9.5` item 6; `12 §2`; `16 §1.1`.

### kernel/inductive/nested-ordinary-omega-lifts-stay-omega [KERNEL-NESTED-IND]

- spec: `14 §3.2`; `12 §2`; `16 §1.1`
- given: a motive `P : D -> Omega_l`, exercised separately at a direct child
  `r : D`, a W-style child `k : B -> D` with `B : Type b`, and a primitive
  Sigma containing two direct `D` components, both at leaf level `l`
- expect: the direct lift is `P r : Omega_l`; the W-style lift is
  `(x : B) -> P (k x) : Omega_(max b l)`; and the two-component
  primitive-Sigma lift is `Omega_l`. None is reclassified into `Type` merely
  because nested lifting exists
- why: the deliberate `Type` crossing belongs only to a generated declared-
  former `All` boundary. This three-part control prevents an implementation
  from applying that exception to the direct, Pi, or primitive-Sigma clauses.

### kernel/inductive/nested-declared-former-omega-crosses-to-exact-type [KERNEL-NESTED-IND]

- spec: `14 §3.2`; `§9.5` item 6; `12 §2`; `16 §1.1`
- given: a level-polymorphic positive host `Box_h (A : Type h) : Type h`, a
  predicate `P : A -> Omega_l`, and `v : Box_h A`, with independent symbolic
  levels `h` and `l`
- expect: `All^Omega_{Box_h,0} P v : Type (max l h)`. Its leaf proofs remain
  irrelevant at `P x : Omega_l`, but the source-topology family is not itself
  a proposition
- why: this pins every locked axis. Mutations to `Omega_(max l h)`,
  `Type (suc (max l h))`, `Type l`, or `Type h` all produce a different
  classifier. Keeping `h` and `l` symbolic makes dropping either maximum
  operand observable.

### kernel/inductive/nested-declared-former-type-crosses-to-exact-type [KERNEL-NESTED-IND]

- spec: `14 §3.2`; `12 §2`
- given: the same `Box_h`, now with `P : A -> Type l`
- expect: `All^Type_{Box_h,0} P v : Type (max l h)`; the successor needed by
  the generated former's own classifier does not appear in this applied result
- why: this is the type-valued sibling of the Omega control. Together they
  prevent an implementation from sharing the two families by smuggling a
  wildcard sort or from applying the Omega exception only accidentally.

---

## AC2c — zero-evidence topology and trust accounting remain observable

Spec: `14 §3.2`, `§7.8`, `§9.5` item 7; `18 §5`.

### kernel/inductive/nested-zero-evidence-topology [KERNEL-NESTED-IND]

- spec: `14 §3.2`, `§7.8`, `§9.5` item 7
- given: the admitted `Slot` above, a predicate `P : A -> Type l`, and the two
  source values `vacant` and `occupied x`
- expect: lifting `vacant` constructs the aligned `all_vacant` inhabitant at
  source index `vacant` with **zero evidence fields**; lifting `occupied x`
  constructs the distinct aligned inhabitant at source index `occupied x` with
  exactly one field `px : P x`
- why: the zero-leaf case still carries source topology. Omitting the vacant
  constructor, inventing an arbitrary `P` inhabitant for it, or flattening it
  into the occupied case either makes the indexed family incomplete or produces
  a term at the wrong source index.

### kernel/inductive/nested-generated-all-has-zero-ledger-delta [KERNEL-NESTED-IND] (soundness)

- spec: `14 §3.2`, `§7.8`, `§9.5`; `18 §5`
- given: record the exact `trusted_base()` set, transactionally admit `Box`, and
  generate and check its `All^Type_{Box,0}` and `All^Omega_{Box,0}` families,
  their constructors, method-type use, and nested lift/ι machinery
- expect: the before/after `trusted_base()` sets are **identical**, not merely
  equal in length. The generated families are ordinary checked `Inductive`
  declarations; no generated declaration is `Opaque` or `Primitive`
- why: `trusted_base()` enumerates postulates and real primitives, while checked
  inductives add no environment assumption. This does **not** classify the
  feature as untrusted: the generator, transactional rollback, method-type
  construction, and nested ι implementation are new audited kernel TCB code.
  A mutation that registers either `All` family as a postulate/primitive changes
  the ledger set and fails this control.

### kernel/inductive/nested-generated-all-support-is-terminal [KERNEL-NESTED-IND] (soundness)

- spec: `14 §1`, `§3.2`, `§7.8`, `§9.5` item 8; `18 §4.2`, `§4.3`
- given: record the exact `Σ` declarations, next-id state, and host-to-support
  relation, then transactionally admit the one-positive-carrier `Box`
  declaration above
- expect: `Σ` gains exactly three `Inductive` declarations: `Box`,
  `All^Type_{Box,0}`, and `All^Omega_{Box,0}`. Exactly six fresh `GlobalId`s
  are allocated: one former id and one constructor id for each declaration.
  Each stored declaration carries exactly one constructor record, aligned with
  `box`. The support relation has exactly two outgoing edges from `Box`, one to
  each support family, and no outgoing edge from either support. Each of the
  three family ids supports an ordinary, successfully checked
  `Term::Elim { fam, … }` use. Those eliminators are derived term forms, not
  declarations, and allocate no ids. Both support families are terminal and
  absent from the general enclosing-former lookup; no `All`-of-`All` declaration
  exists
- why: ordinary positivity checking of either first-order support family again
  sees the carrier `A` positively. Terminal kernel provenance must stop that
  fact from re-entering host generation while retaining ordinary checking. A
  mutation that feeds either support declaration back through host generation
  adds declarations, ids, and a support-to-support edge or fails to terminate.
  A mutation that invents a global eliminator declaration or id also changes
  the locked carrier. Neither can satisfy this exact finite-delta control.

### kernel/inductive/nested-all-generation-is-transactional [KERNEL-NESTED-IND] (soundness)

- spec: `14 §7.8`; `18 §4.2`
- given: an admission mutation under which one required generated `All` family
  fails ordinary inductive checking after the host declaration has been checked
- expect: the entire host admission rejects. Declaration lookup and the exact
  `trusted_base()` set are unchanged: no host former, constructor, partial
  `All` family, or audit entry is observable
- why: positive-path metadata without its checked consumer would license a
  later nested declaration whose required method type or ι cannot be built.
  Transactional rollback keeps admission and eliminator availability one
  indivisible contract.

---

## AC3 — unknown and non-positive positions fail closed separately

Spec: `14 §8.5` clauses 1, 2, 4, and 6.

### kernel/inductive/nested-unknown-head-rejected (soundness)

- spec: `14 §8.5` clauses 1 and 6; `18 §4.3`
- given: declare
  `data UnknownNest (F : Type 0 -> Type 0) : Type 0 where
  mk : F (UnknownNest F) -> UnknownNest F`. The application head `F` is a bound
  variable, not a previously admitted former with checked parameter polarity
- expect: **`Err(PositivityViolation)`**; the declaration and eliminator are not
  admitted
- why: `F` cannot be resolved to checked polarity metadata, so its argument
  position is **unknown** and the retained `occurs` guard rejects it.
  **Disconfirming check:** a buggy fallback `unknown => positive` admits this
  exact declaration, while the correct rule rejects. The nearby fresh-`Bag`
  accept proves the corpus is not merely demanding blanket rejection of every
  nested application.

### kernel/inductive/nested-nonpositive-rejected (soundness)

- spec: `14 §8.5` clauses 2 and 4; `18 §4.3`
- given: first admit
  `data Contra (A : Type 0) : Type 0 where
  contra : (A -> Bool) -> Contra A`, whose parameter `A` is checked
  non-positive; then declare `data Bad : Type 0 where mk : Contra Bad -> Bad`
- expect: **`Err(PositivityViolation)`** at the `Contra` parameter boundary;
  `Bad` and `elim_Bad` are not admitted
- why: this position is known and classified **non-positive**, distinct from the
  unknown-head case. **Controlled verdict flip:** replace only `Contra` by the
  identically kinded positive `Bag` and the declaration admits; a checker that
  records every parameter as positive accepts `Contra Bad` incorrectly.

---

## AC4 — negative polarity remains negative under a positive carrier

Spec: `14 §8.3`, `§8.5` clause 5.

### kernel/inductive/nested-negative-under-positive (soundness)

- spec: `14 §8.3`; `§8.5` clauses 3 and 5
- given: with positive `Bag` already admitted, declare
  `data Bad : Type 0 where mk : Bag (Bad -> Empty) -> Bad`
- expect: **`Err(PositivityViolation)`**; no `Bad` declarations are committed
- why: the checker may traverse `Bag`'s sole parameter, but then reaches `Bad`
  in the domain of `Bad -> Empty`, at negative polarisation. **Disconfirming
  check:** a bug
  that treats a positive outer parameter as making its entire argument positive
  admits this term. The positive control changes only the payload to `Bag Bad`
  and is accepted, so the rejection is specifically the inner arrow polarity,
  not the outer application.

### kernel/inductive/nested-negative-transparent-sigma-control

Status: executing binding established in
`crates/ken-kernel/tests/nested_inductives_remaining.rs` as
`checked_transparent_sigma_alias_rejects_inner_arrow_negative`. Its two direct
recursive payloads are the positive controls.

- spec: `14 §8.3`; `§8.5` clause 5
- given: a fresh checked transparent non-dependent Sigma alias `Product`, then
  the independently controlled pairs `Product Good1 Unit` versus
  `Product (Bad1 -> Empty) Unit`, and `Product Unit Good2` versus
  `Product Unit (Bad2 -> Empty)`
- expect: both direct recursive payloads are **accepted**. Both corresponding
  inner-arrow payloads are **rejected with `PositivityViolation`**
- controls: independently suppress checked-transparent head unfolding,
  primitive-Sigma first-component descent, and primitive-Sigma second-component
  descent at their production seams. Each mutation must leave the fixture
  buildable and redden the control for the exact dimension it removed
- why: after nested-positive admission lands, the old case is no longer
  explained by a blanket ban on application arguments. Ordinary reduction
  unfolds the checked transparent Product to primitive Sigma; structural
  descent then checks both components and rejects when either reaches the
  recursive occurrence at negative polarity. Renaming Product must retain the
  same verdicts. This preserves the original soundness result without a Pair
  spelling or parameter allow-list, and neither component can be discarded.

---

## AC5 — mutual remains separate; direct and W-style stay live

Spec: `14 §8.6`; `§3`, `§3.1`, `§7.3`, `§7.7`, `§8.4`.

### kernel/inductive/mutual-family-block-still-rejected

- spec: `14 §8.6`; `18 §4.1`, `§4.3`
- given: an attempted simultaneous inductive block for two families that refer
  to one another, such as `Even` and `Odd`
- expect: **rejected before either family, constructor set, or eliminator is
  committed**
- why: nested-only does not add a simultaneous declaration API, joint polarity
  judgment, jointly generated eliminators, or joint termination argument. A
  frontend syntax for such a block, if introduced later, cannot elaborate it
  into two independently admitted declarations that temporarily trust the
  other. This asserts the semantic boundary without pinning a future surface
  spelling.

### kernel/inductive/nested-direct-and-wstyle-controls-unchanged

- spec: `14 §3`, `§3.1`, `§7.3`, `§7.7`, `§8.4`
- given: rerun `kernel/seed-k1.md`'s direct `Nat`/`Vec` admission and ι cases,
  plus this directory's `seed-wstyle.md` admission, Π-abstracted-IH, ι, and
  termination cases with their executable fixtures and expected behavior
  unchanged
- expect: **all unchanged-green**
- why: nested lifting extends only constructor arguments reached through
  checked positive parameter paths. Direct recursive arguments retain one
  direct motive instance; Π-bound arguments retain their function-shaped IH.
  The existing live suites are the adjacent enforcement while the new cases are
  implementation-gated.

---

## Internal consistency and stage audit

- Correct custom-positive and DS-9 paths accept; unknown, known non-positive,
  nested-negative, and mutual paths reject for distinct named guards.
- The future `size` computation and the structural nested-ι case both require
  the same source-indexed `All^Type_{Bag,0}` inhabitant. Neither can pass by
  deleting the old `occurs` guard.
- On neutral `v : Box Bool`, the method binder and generated inhabitant share
  one literal `All^Type` application; the method-dependent eliminator mutation
  rejects rather than being papered over by an equality or transport.
- Direct, Pi, and primitive-Sigma proposition lifts retain their ordinary
  Omega classifiers. The declared-former crossing alone lands in the exact
  `Type (max leaf-level host-level)` result, for both `All^Type` and
  `All^Omega`, with no successor in the applied result.
- `Slot.vacant` produces a source-aligned zero-evidence constructor, distinct
  from the one-leaf `occupied` topology.
- Generated `All` declarations leave the exact `trusted_base()` set unchanged,
  while the generator, transaction, and nested iota remain audited kernel TCB
  code. The one-constructor, one-carrier `Box` transaction adds exactly three
  `Inductive` declarations and six `GlobalId`s, with no eliminator declaration
  or `GlobalId`. Its two support families are terminal and no `All`-of-`All`
  exists; failure to generate either rolls the whole admission back.
- DS-9, fresh-carrier, and composed-carrier admission execute un-gated;
  negative-under-positive, transparent-Sigma-negative, unknown-head, and
  nonpositive controls reject; direct/W-style behavior remains unchanged. The
  canonical floor-Pair instantiation remains RED-UNTIL its realization in the
  Pair-boundary seed without gating these representation controls. Their paired
  positive controls admit while each exact negative/unknown mutation flips the
  verdict.
- `nested-size-uses-lift` executes un-gated through the named kernel and
  full-pipeline witnesses, including the deeper residual-`Bag.join` fold and its
  `3`-versus-`1` discriminator. Both selector sorts are landed. The durable
  unary dependent-motive positive reaches the Ω selector and kernel-checks. The
  binary transition sentinel reaches both residual associations and selections,
  then requires the exact final `check_match_with_lift` type-mismatch refusal.
  Its post-repair positive and fail-closed contract remain stated without
  misclassifying the selector as missing. Independently gated non-D6 residual
  rows remain marked; blanket nested rejection is no longer the live boundary.
