# Kernel-2 (observational) conformance -- K2-scoped seed cases

Format: `../../README.md`. These pin the K2 acceptance criteria
(`../../docs/program/wp/K2-observational.md` par. 2) and the soundness
commitments from `../../spec/10-kernel/README.md` par. 6 (#9-14).
All cases are K2-scoped; K1 conformance must continue to pass (no
regression).

Cases tagged **(oracle)** are to be validated against the prototype at
build time by the Spec enclave.

**Quotient-term convention.** Every quotient in this seed is formed as
`A / R / e` with `R : A → A → Ω_l` and checked `e : IsEquiv A R`
(`16 §5`); `A/R` abbreviates that formed type.

---

## Acceptance criterion: Omega proof-irrelevance (frame par. 2 item 5, README #9)

### observational/omega-pi-convertible (soundness)
- spec: `spec/10-kernel/16-observational.md` par. 1.2
- given: `P : Omega_l`, `p : P`, `q : P` in context; conversion check
  `p` vs `q` at type `P`
- expect: **convertible** (conversion returns true)
- why: Omega-PI is definitional -- any two proofs of a proposition are
  equal. The checker must not inspect the terms. Conversion at an
  Omega_l type is a constant-time "yes."

### observational/omega-skip-prop-args
- spec: `spec/10-kernel/16-observational.md` par. 1.2, 8.2
- given: `f g : ((x:A) -> P x -> B)` where `P x : Omega`;
  `f a p` vs `g a q` at type `B[a/x]` where `p /= q` as terms but
  both `p, q : P a`
- expect: `f a p ≡ g a q` (if `f ≡ g` pointwise and `a` matches);
  the propositional argument `p`/`q` is skipped
- why: propositional-argument-skip shortcut. The arguments at Omega
  type are exempt from structural comparison. This saves agents from
  synthesising coherence proofs.

---

## Acceptance criterion: Eq-by-type computes (frame par. 2 item 1, README #10-11)

### observational/funext-definitional (soundness)
- spec: `spec/10-kernel/16-observational.md` par. 2.2
- given: `f g : (x : A) -> B x` where `A : Type 0`, `B : A -> Type 0`;
  `Eq ((x:A) -> B x) f g`
- expect: **reduces-to** `(x : A) -> Eq (B x) (f x) (g x)`
- why: Eq at Pi is pointwise equality -- funext is definitional.
  The result type is in Omega. Test with both neutral `f`/`g`
  (reduction still applies to the Eq node) and lambda-headed `f`/`g`.
  Frame AC: "exercise with open terms and >=2 distinct type variables."

### observational/funext-with-levels
- spec: `spec/10-kernel/16-observational.md` par. 2.2
- given: `A : Type 1`, `B : A -> Type 2`, `f g : (x:A) -> B x`;
  `Eq ((x:A) -> B x) f g` — the type is at `Type (max 1 2) = Type 2`
- expect: **reduces-to** `(x : A) -> Eq (B x) (f x) (g x)` at
  `Omega_2` — the Omega level is the predicative `max` of the domain
  and codomain levels
- why: Eq-by-type is level-polymorphic — `Eq A a b : Omega_l` for
  `A : Type l`. The funext Pi lands in `Omega_2`, not `Omega_0`.
  Tests distinct level variables (≥2 distinct levels) per the K1
  retro lesson.

### observational/propext-definitional (soundness)
- spec: `spec/10-kernel/16-observational.md` par. 2.2
- given: `P Q : Omega`; `Eq Omega P Q`
- expect: **reduces-to** `(P -> Q) and (Q -> P)`
- where: `P -> Q := (x : P) -> Q` (par. 1.3), `and` is Sigma
- why: Eq at Omega is mutual implication -- propext is definitional.
  Test with neutral `P`/`Q` and with known Omega propositions.

### observational/eq-open-universe-levels-stay-neutral (soundness)
- spec: `spec/10-kernel/16-observational.md` §2.2;
  `spec/10-kernel/12-universes.md` §4
- given: raw level variables `u` and `v`; let `U(l)` be either `Type l`
  or `Omega_l`. For each former, query `whnf` directly on
  `Eq (Type (max (suc u) (suc v))) (U u) (U v)`, and on the same
  carrier with endpoint levels `(0, u)` and `(max (suc u) v, suc u)`.
  This is a raw reducer query; do not run `infer` or `check` or claim
  these open `Eq` terms satisfy `Eq-Form`. For each former, also query
  `Eq (Type 1) (U 0) (U 1)` and
  `Eq (Type (suc u)) (U u) (U u)` directly with `whnf`.
- expect: each failed level-equivalence comparison with an open side
  leaves the exact `Eq` term neutral. The closed unequal control reduces
  to `Bottom`; the identical open-level control reduces to `Top`.
- why: the old universe arm returned `Bottom` for every failed
  comparison, including open levels that may become equal on instantiation.
  The exact raw input distinguishes the new neutral result from that old
  reduct; the controls preserve closed inequality and open equality. This
  supplements `conversion/level-distinct-not-convertible`, which tests
  level conversion rather than the Eq reduction arm.

### observational/eq-inductive-same-ctor
- spec: `spec/10-kernel/16-observational.md` par. 2.2
- given: `data Nat : Type 0 where { zero : Nat ; suc : Nat -> Nat }`;
  `Eq Nat (suc (suc zero)) (suc x)` where `x : Nat` in context
- expect: **reduces-to** `Eq Nat (suc zero) x` (conjunction of one
  argument equality for `suc`)
- why: same constructor => conjunction of argument-equalities. One
  recursive argument for `suc`.

### observational/eq-inductive-diff-ctor
- spec: `spec/10-kernel/16-observational.md` par. 2.2
- given: `data Nat : Type 0 where { zero : Nat ; suc : Nat -> Nat }`;
  `Eq Nat zero (suc n)` where `n : Nat`
- expect: **reduces-to** `Bottom` (i.e. `Empty`)
- why: different constructors => the empty proposition. No possible
  proof.

---

## Acceptance criterion: Eq at Omega carriers and components

These six cases carry `SPEC-EQ-FORM-OMEGA-CARRIER`, deliverable 2.

### observational/eq-subset-sigma-omega-field (soundness)
- spec: `spec/10-kernel/16-observational.md` §§2.1, 2.2, 8.4;
  `spec/10-kernel/13-pi-sigma.md` §4
- given: In context `P : Nat -> Omega_0`, `a b : Nat`, `p : P a`,
  `q : P b`, `h : Eq Nat a b`, let `S := (x : Nat) × P x`,
  `s := (a, p)`, and `t := (b, q)`. Query `infer(Eq S s t)`,
  `whnf(Eq S s t)`, `infer(whnf(Eq S s t))`, and
  `check((h, refl q), Eq S s t)`.
- expect: `infer(Eq S s t) = Omega_0`; its WHNF is
  `Eq Nat a b and Eq (P b) q q`, with inferred sort `Omega_0`;
  the witness check accepts.
- why: A subset Sigma stays Type-classified. Its reduct keeps the
  relevant base equality and forms the second conjunct at the
  target-side Omega type, with no transport.

### observational/eq-inductive-omega-field-dependent-type (soundness)
- spec: `spec/10-kernel/16-observational.md` §§2.1, 2.2, 8.4;
  `spec/10-kernel/14-inductive.md` §1; `spec/10-kernel/13-pi-sigma.md`
  §§3-4
- given: With declarations `P : Omega_0` and `T : Nat -> Type 0`,
  let `prefix := (k : Nat) × P` and declare:
  ```text
  data D : Type 0 where
    node : (n : Nat) -> P -> T n -> D
  ```
  In context `n m : Nat`, `h : Eq Nat n m`, distinct `p q : P`,
  `x : T n`, `y : T m`, and `e_prefix : Eq prefix (n, p) (m, q)`, set
  `e_T := cong (λ (z : prefix). T z.1) e_prefix`, and assume
  `r : Eq (T m) (cast (T n) (T m) e_T x) y`. Let
  `left := node n p x`, `right := node m q y`. Query
  `infer(Eq D left right)`, `whnf(Eq D left right)`,
  `infer(whnf(Eq D left right))`, and
  `check((h, (refl q, r)), Eq D left right)`.
- expect: `infer(Eq D left right) = Omega_0`; its WHNF is:
  ```text
  (e_n : Eq Nat n m) ×
    (e_p : Eq P q q) ×
    Eq (T m) (cast (T n) (T m) eq_earlier' x) y
  ```
  Here `eq_earlier'` is the §2.2 prefix-transport evidence from
  `(e_n, e_p)`. The witness supplies `h` and `refl q`; the reduct
  infers `Omega_0`, and the check accepts.
- why: R3 forms the Omega-field conjunct at target proof `q`, without
  transporting that field. The later dependent `T n` field still
  transports through the preceding-field equality evidence.

### observational/eq-inductive-omega-field-irrelevant-dependency (soundness)
- spec: `spec/10-kernel/16-observational.md` §§2.2, 8.2, 8.4;
  `spec/10-kernel/14-inductive.md` §1
- given: With declarations `P : Omega_0` and
  `U : Nat -> P -> Type 0`, declare:
  ```text
  data E : Type 0 where
    mk : (n : Nat) -> (p : P) -> U n p -> E
  ```
  In context `n : Nat`, distinct `p q : P`, `x : U n p`,
  `y : U n q`, and `r : Eq (U n q) x y`, let `left := mk n p x`,
  `right := mk n q y`. Query `infer(Eq E left right)` and
  `check((refl n, (refl q, r)), Eq E left right)`.
- expect: `infer(Eq E left right) = Omega_0`, and the check accepts.
- why: The later field's types `U n p` and `U n q` are convertible
  only by Omega proof irrelevance (§8.2). R3 discards the source
  Omega proof; the later field is compared directly, with no
  transport. Structural comparison of `p` and `q` would leave `r`
  ill-typed and this check would not accept.

### observational/eq-trunc-omega-level-one (soundness)
- spec: `spec/10-kernel/16-observational.md` §§2.1, 2.2, 6, 8.4
- given: `A : Type 1`, `u v : ‖A‖`. Query `infer(Eq ‖A‖ u v)`,
  `whnf(Eq ‖A‖ u v)`, and `check(refl u, Eq ‖A‖ u v)`.
- expect: inference returns `Omega_1`; WHNF remains `Eq ‖A‖ u v`;
  the Refl check accepts.
- why: Eq at an Omega carrier is neutral even at Trunc. No
  `Trunc`-to-`Top` reduction can lower `Omega_1` to `Omega_0`.

### observational/eq-canonical-omega-carriers-neutral (soundness)
- spec: `spec/10-kernel/16-observational.md` §§1.1, 1.3, 2.1, 2.2,
  8.2; `spec/10-kernel/13-pi-sigma.md` §4
- given: In context `P Q : Omega_0`, let
  `C := (p : P) × Q` (the conjunction `P and Q`), with `u v : C`.
  Also let `A : Type 0`, `F := (x : A) -> Q`, and `f g : F`. Query
  `infer(Eq C u v)`, `whnf(Eq C u v)`, `check(refl u, Eq C u v)`,
  `infer(Eq F f g)`, `whnf(Eq F f g)`, and
  `check(refl f, Eq F f g)`.
- expect: both Eq inference results are `Omega_0`; each WHNF is its
  unchanged `Eq` term; both Refl checks accept.
- why: canonical Sigma and Pi heads do not trigger Type-carrier Eq
  reduction when the carrier is Omega-classified. Omega-PI admits
  both Refl terms.

### observational/eq-omega-carrier-boundary-pair (soundness)
- spec: `spec/10-kernel/16-observational.md` §§2.1, 2.2, 3.1, 8.2
- given: In context `P : Omega_0`, `u v : P`, `e : Eq Omega_0 P P`,
  and `n : Nat`, query `infer(Eq P u v)`, `whnf(Eq P u v)`,
  `check(refl u, Eq P u v)`, `infer(cast P P e u)`, and
  `infer(Eq n n n)`.
- expect: Eq inference returns `Omega_0`, its WHNF is unchanged, and
  Refl accepts. Eq-Form at `P` is admitted; the proposition-to-
  proposition Cast and non-type-carrier Eq remain refused.
- why: Eq-Form at `P` is refused on the framed main, so its admitted
  side carries the pair's flip. Cast admission stays Type-only, and
  `n : Nat` is not a type or proposition carrier; both refusals are
  controls.

---

## Acceptance criterion: cast regularity and computation (frame par. 2 items 2-3, README #12)

### observational/cast-refl (soundness)
- spec: `spec/10-kernel/16-observational.md` par. 3.2
- given: `A : Type 0`, `a : A`; `cast A A refl a`
- expect: **reduces-to** `a`
- why: regularity -- cast on a reflexive type equality is the identity.
  Test with neutral `A` (cast stays neutral) and canonical `A` (cast
  reduces). Frame AC: "exercise with >=2 distinct type variables."

### observational/cast-computes-pi
- spec: `spec/10-kernel/16-observational.md` par. 3.2
- given: `f : (x : A1) -> B1 x` where `Eq Type A1 A2` and
  `Eq ((x:A1) -> B1 x) ((x:A2) -> B2 x)` are canonical
  (built from structural components); `cast ((x:A1)->B1 x) ((x:A2)->B2 x)
  proof f` applied to `x : A2`
- expect: **reduces** -- the cast produces a lambda that computes when
  applied
- why: cast-by-type at Pi decomposes and transports. This is the
  canonicity test for cast at Pi.

### observational/cast-computes-sigma
- spec: `spec/10-kernel/16-observational.md` par. 3.2
- given: `p : (x : A1) x B1 x`; canonical type equality
  `Eq Type ((x:A1)xB1 x) ((x:A2)xB2 x)`;
  `cast ((x:A1)xB1 x) ((x:A2)xB2 x) proof p`
- expect: **reduces-to** a pair (constructor form), not stuck
- why: cast-by-type at Sigma decomposes into componentwise casts.
  Canonicity: the result is a constructor form.

### observational/cast-computes-inductive
- spec: `spec/10-kernel/16-observational.md` par. 3.2
- given: `data Vec (A : Type 0) : Nat -> Type 0 where
  { vnil : Vec A zero ; vcons : (n:Nat) -> A -> Vec A n -> Vec A (suc n)
  }`; canonical type equality `Eq Type (Vec A n) (Vec A m)` with
  `e_nm : Eq Nat n m`; `cast (Vec A n) (Vec A m) proof (vcons n a xs)`
- expect: **reduces-to** `vcons m (cast A A refl a) (cast ... xs)`
  (constructor form, with recursive casts)
- why: cast-by-type at inductive preserves constructor structure. Each
  argument is transported individually.

### observational/cast-computes-quotient
- spec: `spec/10-kernel/16-observational.md` par. 3.2
- given: formed `Q := A / R / e : Type 0`, `a : A`; canonical type
  equality `Eq Type Q Q`; `cast Q Q proof [a]`
- expect: **reduces** (class preserved)
- why: cast at quotient preserves the class structure, transporting
  the representative.

### observational/cast-rejects-unequal-levels (soundness)
- spec: `spec/10-kernel/16-observational.md` §3.1–§3.2
- given: a one-constructor `D : Type 1` with `c : Bool → D`; let
  `Large := (x : Bool) × Eq D (c x) (c x)` and
  `Small := (x : Bool) × Eq Bool x x`, which infer at `Type 1` and `Type 0`.
  For `v : Large`, query `convert(Type 1, Large, Small)` and try
  `cast Large Small (refl Large) v`; compare with
  `cast Large Large (refl Large) v`.
- expect: `convert(Type 1, Large, Small)` is **true**. The cross-level cast is
  **rejected** at formation; the equal-level control **accepts** and reduces
  to `v`. `trusted_base()` is unchanged.
- why: §3.1 requires both endpoints to inhabit the same `Type l`. A cast that
  drops the level comparison would accept the first arm while the control stays
  green.

---

## Acceptance criterion: J on non-refl (frame par. 2 item 4, README #13)

### observational/j-on-refl (soundness, beta)
- spec: `spec/10-kernel/15-identity.md` par. 4.2
- given: `J A a P d a (refl a)` well-typed
- expect: **reduces-to** `d`
- why: J-beta -- the base computation rule for the eliminator.

### observational/j-nonrefl (soundness, the headline)
- spec: `spec/10-kernel/15-identity.md` par. 4.3
- given: `J A a P d b e` where `e : Eq A a b` is a canonical non-refl
  equality (e.g. `trans (refl a) (refl b)` or a proof built by
  `cong` on a constructor)
- expect: **reduces** -- to a constructor/lambda/pair form, **not** stuck
  at a neutral `J` node
- why: J on non-refl MUST reduce via cast. Fails on any kernel that
  only reduces J on refl. This is the headline non-reproduction of
  the prototype's J-only-on-refl gap. Test with at least two distinct
  non-refl shapes (symmetry, transitivity, congruence).

---

## Acceptance criterion: Quotients (frame par. 2 item 7, README #14)

### observational/quotient-eq (soundness, C8)
- spec: `spec/10-kernel/16-observational.md` §2.2, §5
- given: (a) an open context `A : Type l`, `R : A → A → Ω_l`,
  `e : IsEquiv A R`, and `a,b : A`, with canonical classes `[a]`, `[b]`;
  query `whnf(Eq (A / R / e) [a] [b])`; (b) closed `Nat`, total `R`, and
  checked total-equivalence witness `e` for the `Top` control.
- expect: (a) **reduces to** `R a b : Ω_l`, even if that application is
  neutral because `R` is opaque; the outer `Eq` is gone. (b) reduces to
  `Top`.
- why: two class-headed endpoints expose the relation only after `e` has
  checked. An invalid relation is rejected at formation; see
  `../seed-kernel.md` `kernel/formation/quotient-requires-checked-equivalence`.
  Non-class endpoints stay neutral in the adjacent case.

### observational/quotient-eq-neutral-endpoint (soundness, C8)
- spec: `spec/10-kernel/16-observational.md` §2.2
- given: a formed `Q := A / R / e`, open `q : Q`, and `a : A`; query
  `whnf(Eq Q q [a])`.
- expect: remains neutral as `Eq Q q [a]`; it does not reduce to `R`.
- why: the quotient-Eq rule requires both endpoints to be canonical classes.
  This pins the non-class endpoint arm, not the class-pair C8 reduct.

### observational/quotient-elim (soundness)
- spec: `spec/10-kernel/16-observational.md` par. 5
- given: `M : (z : A/R) -> Type 0`, `f : (x:A) -> M [x]`,
  `r` (respect proof), `a : A`; `elim_/ M f r [a]`
- expect: **reduces-to** `f a`
- why: quotient eliminator computes on a class -- the i-reduction.

### observational/quotient-elim-omega-free
- spec: `spec/10-kernel/16-observational.md` par. 5
- given: `M z : Omega` for all `z : A/R`; `f : A -> M [_]`;
  `elim_/ M f (free by Omega-PI) [a]` -- the respect proof is
  auto-filled
- expect: **accepts** and **reduces-to** `f a`
- why: respect-free elimination when the target is in Omega. No manual
  coherence proof required.

---

## Acceptance criterion: Truncation (frame par. 2 item 8)

### observational/trunc-elim
- spec: `spec/10-kernel/16-observational.md` par. 6
- given: `P : Omega`, `f : A -> P`, `a : A`; `elim_trunc P f |a|`
- expect: **reduces-to** `f a`
- why: truncation eliminator computes. Since `P : Omega`, no respect
  condition is required.

### observational/trunc-quot-elim-target-sort-boundary (soundness)
- spec: `spec/10-kernel/16-observational.md` §5 (Quot-Elim-Ω and the
  Type-target restriction), §6 (Trunc-Elim), and §8.2
- given: `A : Type l`, `a : A`, `t := |a| : ‖A‖`; one context also
  contains `MΩ : ‖A‖ → Ω_j`, `fΩ : (x : A) → MΩ |x|`,
  `MT : ‖A‖ → Type k`, and `fT : (x : A) → MT |x|`. Let `r := a`
  (well-scoped only). Query `infer` on core `QuotElim MΩ fΩ r t` and
  `QuotElim MT fT r t`; also query `whnf` on the Ω-target term.
- expect: Ω-target `infer` **accepts** at type `MΩ t`, and `whnf`
  reduces to `fΩ a`. Type-target `infer` **rejects at the
  Quot-scrutinee requirement**; no error variant or diagnostic text
  is fixed.
- why: The paired inputs share `A`, `a`, `t`, and `r`; only the
  motive's target sort and corresponding method type differ. Both
  methods are well-typed for the truncation injection. The method
  check compares `M` arguments at `‖A‖ : Ω` and passes by Ω-PI
  (`§8.2`); the Type-target guard is the first refusing premise. A
  Type-target path that wrongly uses respect-free Trunc elimination
  would accept the second input. The Ω arm remains the positive
  control for `Quot-Elim-Ω` over truncation.

### observational/trunc-or-exists
- spec: `spec/10-kernel/16-observational.md` par. 6
- given: `P :|| Nat + Bool ||` (i.e. `P or` with `Nat` and `Bool`),
  eliminate into `Unit : Omega`
- expect: **accepts** and **computes** (elimination works)
- why: truncation backs Omega's or/exists. Derived operations
  must compute.

---

## Acceptance criterion: UIP (frame par. 2 item 6)

### observational/uip-definitional
- spec: `spec/10-kernel/15-identity.md` par. 5, `16` par. 1.2
- given: `p q : Eq A a b`; conversion check `p` vs `q` at type
  `Eq A a b : Omega`
- expect: **convertible** (definitionally equal)
- why: Eq lands in Omega, so there is no nontrivial equality of
  equalities. UIP holds definitionally.

---

## Regression: K1 conformance unchanged

### observational/k1-subset-still-green
- spec: `spec/10-kernel/README.md` par. 6, K1 commitments #1-8
- given: all K1-scoped seed cases from `../seed-k1.md` + untagged
  `../seed-kernel.md` cases
- expect: **all pass** (K2 does not regress K1)
- why: K2 extends, does not rewrite, K1's check/infer/whnf/conv.
  The entire K1 conformance suite must stay green.
