# The automated prover

> Status: **V3 specified**. Normative for the prover
> contract, the **verdict trichotomy** and its projection to V1's four-way
> epistemic status (§1), the **exhaustive** classifier (§2), the soundness
> discipline (the de Bruijn re-check, §1.5), and the Kripke embedding +
> **reflective certificate route (a)** (`OQ-12` DECIDED, §4). The embedding's
> frame, domain, forcing, quotation, and certificate contracts are closed in
> §4; the two kernel-facing theorem statements are required but their artifact
> placement is not decided here. **Untrusted:** every certificate is kernel-
> re-checked (`../10-kernel/18 §4`), so a prover bug is a failed/weaker verdict,
> never unsoundness. Contract for WS-V **V3** (third WP of the spine V1→V2→V3).

## 1. Contract

### 1.1 Input — one obligation

The prover consumes **one obligation at a time**: a triple
`⟨id, Γ ⊢ φ, provenance⟩` (`22 §1`) with goal `φ : Ω_ℓ` in a local
hypothesis context `Γ`. Obligations are **independent** — provable in any
order / in parallel (`22 §5`/§6) — so the prover is a per-obligation function
with no cross-obligation state. `id` and `provenance` are threaded to the verdict for
diagnostics (`24`) and the protocol (`25`); the proof search reads only `Γ ⊢ φ`.

### 1.2 Output — the verdict trichotomy (`21 §5.1`)

Attempting `Γ ⊢ φ` yields exactly one **verdict** — the kernel/Heyting
trichotomy V1 fixes (`21 §5.1`, `12 §5`), each carrying the evidence that makes
it actionable and (for `proved`) re-checkable:

| Verdict | Evidence produced | Kernel re-check |
|---|---|---|
| `proved` | a **certificate** — a core term `p` with `Γ ⊢ p : φ` | `check(env, Γ, p, φ)` accepts (`18 §4.5`) — the de Bruijn criterion; the *sole* reason `proved` is believed |
| `disproved` | a **countermodel** — a finite Kripke model forcing `¬φ` at some world (`24 §1`); where the backend yields a proof of `¬φ`, the cert `q : ¬φ` is `check`ed too | proof of `¬φ`: `check(env, Γ, q, ¬φ)`; bare countermodel: prover-asserted refutation — untrusted, a concrete falsifying witness (`21 §5.1`) |
| `unknown` | a **typed hole** `?id : φ` in `Γ`, admitted as a **postulate** of `φ` (`22 §1`, `24 §2`) | none — the hole is *assumed*; its goal appears in `trusted_base()` (§1.3) |

There is **no fourth verdict and no `failure` catch-all**: a search that neither
closes nor refutes `φ` is `unknown`-with-hole (honest — the program still runs,
`21 §5.1`), never a silent drop. (This sharpens the earlier
`certificate | failure` split: "failure" resolves into `disproved` vs `unknown`,
which carry *different evidence* and *different downstream meaning* — a refuted
claim is fixed, an open one is left running.)

### 1.3 The honesty guard — `proved` is kernel-structural, not a prover flag

The prover **cannot mark** an obligation `proved`. Per the V1 honesty guard
(`21 §5.4`, `18 §5`): `Γ ⊢ φ` is `proved` **iff** a certificate `p` `check`s
**and** no postulate carrying `φ` sits in `GlobalEnv::trusted_base()`
(`18 §4.1`/§5). An undischarged obligation *is* a `declare_postulate` of `φ`, so
its goal is enumerated by `trusted_base()`; discharging retires the postulate
(the certificate replaces the assumption). The verdict is therefore decidable
from the **kernel's own state**, with **no side-channel / parallel "proved"
store** the prover could write — a prover bug can leave a hole (`unknown`) or
emit a cert the kernel rejects, but can **never** forge `proved`. This is the
V1-build kernel-structural-status carry, preserved.

### 1.4 Projection to V1's four-way epistemic status (the reconcile)

The frame's "four-way" is V1's **epistemic status** (`21 §5.2`, `OQ-spec`
DECIDED) — `proved` / `tested` / `delegated` / `unknown` — which is **not** the
prover's output. The prover produces the **verdict trichotomy** (§1.2); V1's
projection (`21 §5.3`) resolves *verdict × disposition* into the epistemic
status. The prover therefore realizes **three** of the four labels and **never**
the other two:

- `proved` verdict → **`proved`** status (the default disposition, discharged);
- `disproved` verdict → *no* exported status — a refuted claim is a hard
  verification error (`24 §3`), fixed not shipped (`21 §5.3`);
- `unknown` verdict → **`unknown`** status (an open typed hole);
- **`tested`** and **`delegated`** are **dispositions** (`test`/`assume` and
  temporal clauses, `21 §5.2`) that **bypass the prover entirely** — they are
  not static obligations V3 attempts, so V3 never produces them.

So "wire the prover to V1's four-way status" means: emit the trichotomy verdict
with its evidence, keyed by `id`, for `21 §5.3` to project — **not** that the
prover itself emits four outcomes. Conflating the two is this chapter's central
reconcile hazard (the analog of `21 §5`'s verdict-vs-status separation).

### 1.5 The cardinal rule (de Bruijn criterion)

The prover is **untrusted**. No backend's "yes" is accepted on its own authority
— a kernel-checkable certificate is always produced and re-checked (§1.2). A bug
in the classifier, the embedding, Z3, cvc5, or a tactic can only cause a
*failure to prove* (→ `unknown`) or a *rejected certificate* (→ not `proved`),
**never a false `proved`.** This is what licenses using a classical solver under
an intuitionistic logic (§4), and it is the single property the rest of this
chapter exists to preserve.

## 2. The fragment classifier

Each obligation is routed by a syntactic analysis of `φ` (and the atom theories
it mentions) to the cheapest **sound** method. Three fragments:

| Fragment | What's in it | Method |
|---|---|---|
| **D — decidable** | atoms where `φ ∨ ¬φ` holds: equality/disequality of scalars & handles, `Int`/`Decimal` arithmetic comparisons, Boolean combinations, finite/bounded membership and quantifiers | **direct** decision (reflective) + Z3 to *search* |
| **FO — first-order intuitionistic** | the closed source grammar and uninterpreted persistent relations fixed in §4 | **Kripke embedding** → Z3 (§4) |
| **HO — higher-order / inductive** | quantification over types or predicates, goals needing induction, anything outside FO | **native intuitionistic** (prop skeleton) + **tactics / manual**; typed hole if open |

- For **D**, classical and intuitionistic logic **coincide** (excluded middle is
  available *because the atom is decidable*), so the classical solver is sound
  with no embedding (the key fragment boundary).
- The classifier is **conservative**: when unsure whether an atom is decidable
  or a formula is FO, it routes *upward* (to the more general, more expensive
  method). Misclassification downward would risk unsoundness *if the certificate
  weren't re-checked* — and it is, so the worst case of even a buggy classifier
  is wasted work or a failed proof, never unsoundness.

### 2.1 Routing is total — the completeness backstop

The classifier is a **total function** `classify : Obligation → Route` over the
*fixed* obligation form (`22 §1`: a goal `φ : Ω` built from the kernel's
proposition formers, `16 §1`). It is **exhaustive by construction**, mirroring
V2's extraction discipline (`22 §2.5`/§5): every `φ` routes to D, FO, or **HO as
the default** — HO is the catch-all that always applies (a typed hole is always
a legal HO outcome), so an unrecognized or future formula shape **falls to HO**,
never to a silent skip:

```
classify(⟨id, Γ ⊢ φ, _⟩) → Route:
  case shape(φ) of
    decidableAtoms(φ)       → D       -- §3: φ ∨ ¬φ holds on every atom
    firstOrderIntuit(φ)     → FO      -- §4: the Kripke embedding
    _                       → HO      -- §5: tactics + typed hole — the DEFAULT arm
  -- NO `_ ⇒ skip`. Every obligation receives a route; one the classifier cannot
  -- place lands in HO and is attempted (or left an honest typed hole), never
  -- dropped as if discharged.
```

**Why totality is load-bearing (the two-soundnesses split, V2 carry).** A
*wrong* route is harmless: a misclassified-downward `φ` either yields a
certificate the kernel **re-checks** (still sound) or fails and becomes an
honest `unknown` — wasted work, never a false `proved`. But a **never-routed**
obligation is **not** backstopped: it supplies *no* certificate and leaves *no*
hole, so its goal never enters `trusted_base()` and the claim reads as
discharged though never attempted — a silent verification-soundness gap (the
exact V2 *omission* hazard, `22`). The kernel re-checks what the prover
*supplies*; it cannot see what the classifier *omits*. So **exhaustiveness of
routing is the sole safeguard against a dropped obligation**, asserted
**structurally** (a total `case` with a default arm, no `_ ⇒ skip`) — the
discriminating conformance case drives an obligation of *every* shape through
`classify` and asserts each receives a route (no silent unrouting), the
omission-guard analog of V2's `exhaustive-traversal-no-silent-skip`.

## 3. Fragment D — decidable atoms

Two cooperating mechanisms, both yielding kernel certificates:

1. **Reflective decision (preferred).** For atoms with a kernel-verified
   **decision procedure** `dec : (x : A) → Decidable (φ x)`, the certificate is
   *by computation*: the kernel evaluates `dec a` (**canonicity**, `16 §9` C6:
   closed canonical terms compute) to `inl proof` or `inr refutation`. Here
   `Decidable P` is the **derived** sum `P + (P → Empty)` (`16 §1.3` connectives
   + `Empty`) — *not* a kernel primitive — and `dec` is an ordinary
   kernel-checked function; "kernel-verified" means `dec`'s type `check`s like
   any term (`18 §4`). Because the kernel **computes**, "decide it" produces a
   real proof term with no external solver in the trusted path — Ken's computing
   core is a verification asset here. Used for concrete/closed decidable goals.
2. **SMT-assisted search + reconstruction.** For decidable goals with free
   variables (e.g. linear arithmetic over `Int` with universally-quantified
   parameters), Z3 *searches*; on success the result is turned into a kernel
   certificate by reflection (instantiating a verified arithmetic decision
   procedure) or by reconstructing the proof (SMTCoq-style) and re-checking. The
   solver finds the witness/cut; the kernel re-derives validity.

## 4. Fragment FO — the closed Kripke contract

A genuinely intuitionistic first-order obligation **cannot** be sent to Z3
directly: a classical solver may use excluded middle and accept a goal that is
not intuitionistically valid. Route (a), fixed by `OQ-12`, instead checks a
classical proof of the obligation's exact Kripke translation. This section
fixes that translation and its proof objects. It does not build them.

### 4.1 Supported source fragment and total quotation

The FO source signature is finite and many-sorted. Quotation synthesizes it
from the eligible heads in the obligation; no new surface declaration is
required. Each object sort names a closed, rigid Ken type `A : Type l`, after
level instantiation and canonicalization by kernel conversion. It may not
depend on a term or proof in the obligation context. Each predicate symbol is
a global `Const` with a fixed checked telescope
`A1 -> ... -> An -> Omega_l` over those sorts. Its applications are quoted
atomically whether or not the constant has a body; the FO route does not unfold
that body.

Quotation recognizes the registered logical heads and normalizes only their
canonical expansions; it does not unfold arbitrary definitions. The accepted
proposition shapes are exactly these canonical core shapes:

| source proposition | accepted core `Term` shape |
|---|---|
| truth / falsity | the registered `Top` / `Bottom` `Const` |
| `P t1 ... tn` | an `App` spine headed by an eligible predicate `Const`; every `ti` is an in-scope object `Var` of the declared sort |
| `p and q` | `Sigma(p, q)` with both components in Omega and the proof binder absent from `q` |
| `p => q` | `Pi(p, q)` with `p` in Omega and the proof binder absent from `q` |
| `not p` | the preceding implication shape with `q = Bottom` |
| `p or q` | the canonical `Trunc` of the two-constructor Omega-parameterized sum from `16 §1.3` |
| `forall x : A. p` | `Pi(A, p)` for a declared rigid object sort `A`, with `p` in Omega |
| `exists x : A. p` | `Trunc(Sigma(A, p))` for a declared rigid object sort `A`, with `p` in Omega |

An obligation context is quoted from oldest binder to newest. An object binder
becomes `forall`; a proof hypothesis becomes implication. Thus quotation first
produces a closed intuitionistic `IForm`, even when the extracted obligation
was an open `Gamma |- phi`. This closure preserves dependency on earlier
object binders but refuses dependency on proof terms. A context entry that is
neither an eligible object binder nor an accepted proposition hypothesis
refuses the whole obligation.

The result package is

```
FOProblem ::= problem
  (Sigma : Signature)
  (C : Carriers Sigma)
  (rho : AtomEnv Sigma C)
  (f : IForm Sigma)

FoBoundary ::= unsupported-term-shape
             | unsupported-atom-theory
             | non-rigid-sort
             | higher-order-use
             | dependent-proof-use
             | ill-scoped-or-ill-sorted

quote_fo : Obligation -> Accepted FOProblem | Refused FoBoundary
```

It returns `Accepted` only after arity, sort, scope, and Omega checks succeed.
Every other well-formed Ken `Term` returns `Refused`; it never drops a subterm,
invents an atom, or silently treats a form as classical. The refusal boundary
includes:

- equality, arithmetic, order, collection membership, and every other
  interpreted atom theory;
- constants, constructors, literals, projections, functions, or eliminators
  in object-term position;
- a predicate-valued variable, quantification over a type or predicate, a
  higher-order application, or a predicate head without an eligible checked
  telescope;
- a dependent object sort, a dependent proposition over a proof, multiple
  uses of one predicate head at inconsistent arities or sorts, or an ill-scoped
  variable; and
- every remaining `Term` constructor, including `Type`, `Omega`, `Eq`, `Lam`,
  `Pair`, `Let`, `Ascript`, `Elim`, `Cast`, `J`, quotient forms, and a `Trunc`
  that is not one of the two canonical logical expansions above.

Those forms route to HO under §2.1. The supported atom theory is therefore
only a finite family of **uninterpreted persistent relations**. Equality,
functions, constants, algebraic datatypes, arithmetic, order, and solver
theory atoms are refused here; D may still accept its independently specified
closed decidable cases under §3. This boundary is deliberately smaller than a
general first-order solver language.

### 4.2 Exact classical Kripke theory

For a quoted signature `Sigma`, `K(Sigma)` is a closed, two-level many-sorted
classical first-order theory. It contains one nonempty sort `World` and one
possibly empty classical sort `Obj(A)` for each source object sort. Allowing an
object sort to be empty matches the source carrier: quotation does not require
a closed rigid Ken type to have an inhabitant. The theory has only these
relations:

```
Le        : World -> World -> Prop
Dom_A     : World -> Obj(A) -> Prop
Force_P   : World -> Obj(A1) -> ... -> Obj(An) -> Prop
```

There are no classical function symbols, object constants, or built-in
equality atoms in this theory. Its axioms are exactly:

```
preorder-reflexive:
  forall w. Le w w

preorder-transitive:
  forall w v u. Le w v and Le v u => Le w u

domain-growth-A:
  forall w v x. Le w v and Dom_A w x => Dom_A v x

atom-domain-P:
  forall w x1 ... xn.
    Force_P w x1 ... xn => Dom_A1 w x1 and ... and Dom_An w xn

atom-persistence-P:
  forall w v x1 ... xn.
    Le w v and Force_P w x1 ... xn => Force_P v x1 ... xn
```

`domain-growth-A` occurs once for each object sort; the two atom axioms occur
once for each predicate. There is deliberately no domain-inhabitedness axiom:
neither an empty source carrier nor an empty `Dom_A` entails an existential.
For a nullary predicate the domain conclusion is `Top`. These are emitted
premises of the classical formula, not trusted propositions or solver
assumptions hidden outside the certificate.

In particular, `K(Sigma)` does not entail the translation of
`exists x : A. Top`: that translation still requires a `Dom_A` witness. This
is the required empty-carrier control, not an implementation convention.

Write `w |= f` for the following classical formula, with object variables
interpreted in their declared `Obj` sorts:

```
w |= Top           := Top
w |= Bottom        := Bottom
w |= P xs          := Force_P w xs
w |= (p and q)     := (w |= p) and (w |= q)
w |= (p or q)      := (w |= p) or (w |= q)
w |= (p => q)      := forall v. Le w v => ((v |= p) => (v |= q))
w |= (forall A p)  := forall v. Le w v =>
                        forall x : Obj(A). Dom_A v x => (v |= p[x])
w |= (exists A p)  := exists x : Obj(A). Dom_A w x and (w |= p[x])
```

Negation uses the implication clause because `not p` is `p => Bottom`. In
particular it is not translated as classical negation at the current world.
Implication and universal quantification inspect every accessible future
world; existential quantification uses the current world's domain.
The clauses and the displayed axioms entail, by structural induction on every
accepted `f`, the required persistence property

```
forall w v. Le w v and (w |= f) => (v |= f)
```

This is a theorem of the emitted theory and translation, not an additional
forcing axiom. In particular, the existential case uses domain growth and atom
persistence; neither may be omitted.

For the closed `IForm f` produced by `quote_fo`, the complete target is

```
embed(Sigma, f) := K(Sigma) => forall w : World. w |= f
```

Because `K(Sigma)` is inside the target formula, no frame or forcing premise
exists outside `embed`. The proof-theoretic meaning of
`classically_valid(embed(Sigma, f))` is fixed with the certificate calculus in
§4.3; it is not a backend verdict or a semantic oracle.

### 4.3 Quoted formulas and certificates

`IForm` is the source-side inductive used above:

```
IForm ::= top | bottom | atom PredId (Vector IVar arity)
        | and IForm IForm | or IForm IForm | imp IForm IForm
        | forall SortId IForm | exists SortId IForm

IVar  ::= bound Nat
```

`Signature` is the finite pair of the `SortId` population and the
arity-indexed predicate profiles. `Carriers Sigma` is a sort-indexed family of
the corresponding closed rigid Ken types, including types with no inhabitants.
`IForm Sigma` admits only identifiers and vectors well-formed under that
signature.

`Form` is the target classical inductive. A relation carries its complete sort
profile, so `check_cert` needs no ambient, unchecked signature:

```
QSort ::= world | object SortId
QTerm ::= bound QSort Nat | parameter QSort Nat
QRel  ::= access
        | domain SortId
        | forcing PredId (Vector SortId arity)

Form  ::= top | bottom | rel QRel (Vector QTerm relation_arity)
        | and Form Form | or Form Form | imp Form Form
        | forall QSort Form | exists QSort Form
```

Indices are de Bruijn indices. `parameter` is a certificate-local free
parameter, not a source constant or target function symbol. Formula
well-formedness checks every binder, relation arity, relation argument sort,
and consistent use of each identifier. `embed` produces a closed, well-formed
`Form`.

A sequent is a pair of finite multisets of well-formed `Form`s, written
`Gamma => Delta`; its classical meaning is that the conjunction of `Gamma`
implies the disjunction of `Delta`. The `world` sort has the nonempty meaning
fixed in §4.2; each `object` sort has possibly empty meaning. No rule infers an
object-sort inhabitant. `Cert` is an inductive proof tree whose node stores a
conclusion sequent, one of these rule tags, its explicit principal-formula
occurrence, any witness or eigenparameter, and the indicated child
certificates:

```
Sequent ::= sequent (List Form) (List Form)

Rule ::= init Nat Nat | top-right Nat | bottom-left Nat
       | and-left Nat | and-right Nat
       | or-left Nat | or-right Nat
       | imp-left Nat | imp-right Nat
       | forall-left Nat QTerm | forall-right Nat QTerm
       | exists-left Nat QTerm | exists-right Nat QTerm
       | weaken-left Nat | weaken-right Nat
       | contract-left Nat | contract-right Nat
       | cut Form

Cert ::= node Sequent Rule (List Cert)
```

Each `Nat` selects the named formula occurrence on the indicated side; the two
indices of `init` select its left and right occurrences. The quantifier term is
respectively the witness or eigenparameter described below.

| `Cert` rule | premises checked for conclusion `Gamma => Delta` |
|---|---|
| `init` | closes when the same formula occurs in both multisets |
| `top-right` / `bottom-left` | closes when `Top` occurs on the right / `Bottom` on the left |
| `and-left` | replace left `p and q` by both `p` and `q` in one child |
| `and-right` | replace right `p and q` by `p` and by `q` in two children |
| `or-left` | replace left `p or q` by `p` and by `q` in two children |
| `or-right` | replace right `p or q` by both `p` and `q` in one child |
| `imp-left` | for left `p => q`, check `Gamma => Delta,p` and `Gamma,q => Delta` |
| `imp-right` | for right `p => q`, check `Gamma,p => Delta,q` |
| `forall-left` | instantiate a left universal with a same-sorted quoted term |
| `forall-right` | instantiate a right universal with a fresh same-sorted eigenparameter |
| `exists-left` | instantiate a left existential with a fresh same-sorted eigenparameter |
| `exists-right` | instantiate a right existential with a same-sorted quoted term |
| `weaken-left` | for conclusion `Gamma,A => Delta`, check child `Gamma => Delta` |
| `weaken-right` | for conclusion `Gamma => Delta,A`, check child `Gamma => Delta` |
| `contract-left` | for conclusion `Gamma,A => Delta`, check child `Gamma,A,A => Delta` |
| `contract-right` | for conclusion `Gamma => Delta,A`, check child `Gamma => Delta,A,A` |
| `cut` | check `Gamma => Delta,p` and `Gamma,p => Delta` for the recorded `p` |

In those four structural rows, the displayed `A` is the selected occurrence
and `Gamma` or `Delta` is the residual multiset after removing it. Thus the
table specifies the checker direction from a node's stored conclusion to its
child, rather than merely naming the conventional forward inference. Contexts
are canonical multisets, so exchange is representation equality, not a rule.
Quantifier substitution is capture-avoiding. An eigenparameter is fresh when
it occurs in neither the conclusion sequent nor any parameter recorded above
that node. A witness must be well-sorted in the conclusion's parameter
context. The checker rejects any other child count, context change, principal
occurrence, substitution, freshness claim, free index, or sort mismatch.

`Derivation(Gamma => Delta) : Type` is the indexed proof-tree family generated
by exactly the same rules, with each premise represented by a derivation of its
checked child sequent. Its propositional reflection is truncated, rather than
declared as a proof-relevant inductive in Omega (`16 §1.3`):

```
Derives(s) : Omega := || Derivation(s) ||
```

It is proof data, not an assumed classical oracle. For this route, the name in
the adequacy theorem is defined proof-theoretically:

```
classically_valid : Form -> Omega
classically_valid q := Derives([] => [q])
```

Thus "classically valid" means derivable in this fixed two-sided classical
first-order calculus. The calculus has the ordinary classical sequent meaning
given above; a separate completeness result against model-theoretic semantics
is not used by the discharge path.

The executable meaning is fixed by

```
check_cert : Form -> Cert -> Bool
check_cert q pi = check_tree ([] => [q]) pi
```

where `check_tree` performs exactly the local checks in the table and returns
`True` only if every leaf closes. This is a Ken-level total function over
ordinary derived data, distinct from the kernel API `check` in `18 §4`. A
solver proof format has no authority: an adapter must produce this `Cert` or
the outcome is `unknown`. `FormRef` and `KripkeCountermodel` from §1 and
`24 §1` remain advisory diagnostic data and are not aliases, constructors, or
input evidence for `Form`, `IForm`, or `Cert`.

### 4.4 Required theorem statements and trust boundary

For `C : Carriers Sigma`, let `AtomEnv Sigma C` be the arity- and sort-indexed
family interpreting each quoted predicate as a Ken proposition over the
corresponding carrier types. Let

```
denote : (C : Carriers Sigma) -> AtomEnv Sigma C -> IForm Sigma -> Omega
```

interpret the `IForm` constructors by Ken's connectives and quantifiers from
`16 §1.3`. Quotation also owes preservation: if
`quote_fo(o) = Accepted(problem Sigma C rho f)`, then `denote C rho f` is the
Pi-closed proposition of the original obligation `o`, up to the kernel's
definitional equality.

Route (a) requires these two theorem statements for exactly the data above:

```
embedding_adequacy :
  (Sigma : Signature) ->
  (C : Carriers Sigma) ->
  (rho : AtomEnv Sigma C) ->
  (f : IForm Sigma) ->
  classically_valid (embed Sigma f) -> denote C rho f

checker_soundness :
  (q : Form) -> (pi : Cert) ->
  check_cert q pi = True -> classically_valid q
```

The first statement is the validity-to-Ken direction needed by discharge; this
contract does not require the unused converse. The second ranges over every
quoted target formula, not only an `embed` result. Their composition is the
well-typed discharge:

```
sound Sigma C rho f pi ok :=
  embedding_adequacy Sigma C rho f
    (checker_soundness (embed Sigma f) pi ok)

-- where ok : check_cert (embed Sigma f) pi = True
-- hence sound Sigma C rho f pi (refl True) : denote C rho f
```

After quotation preservation identifies `denote C rho f` with the extracted
goal, the ordinary kernel `check` re-checks the resulting proof at that goal.
A backend `unsat` with no constructible, accepted `Cert` is `unknown`, never
`proved`.

This section states the proof obligations; it does not prove, admit, or place
them. The concrete home of `IForm`, `Form`, `Cert`, `check_cert`,
`embedding_adequacy`, and `checker_soundness`, and the resulting evaluator/TCB
boundary, remain an Architect and operator placement decision. Until both
theorems are kernel-checked in an approved home, route FO cannot return
`proved`. No new kernel primitive or trusted axiom is authorized here.

Route (b), reconstruction of external proof evidence into native kernel terms,
remains only the feasibility hedge recorded by `OQ-12` and the decomposition
report. This contract neither specifies nor changes that route.

### 4.5 First route-(a) vertical slice

The smallest coherent first slice has one rigid object sort `A`, one unary
uninterpreted predicate `P : A -> Omega`, and source forms `Bottom`, atom,
`or`, `imp`, and `forall`. It retains the complete `World` preorder, possibly
empty `Dom_A` with growth, and the `Force_P` domain and persistence axioms. Its
emitted target uses `bottom`, relation, `and`, `or`, `imp`, and `forall`. The
positive proof needs exactly the `init`, `imp-right`, and
`forall-right` certificate rules. The slice theorem restricts `Cert` to that
constructor subset; the full §4.3 theorem remains owed for the remaining
constructors and is not implied by this slice.

The end-to-end positive is the closed intuitionistic identity
`forall x : A. P x => P x`. The classical-only negative control is
`forall x : A. P x or not (P x)`: it must not obtain an accepted certificate
or a `proved` verdict merely because the backend reasons classically. A slice
is complete only when quotation accepts both, the positive certificate
computes to `True` and yields a kernel-checked Ken term through the two stated
theorems, and the negative remains honestly not proved. A translation-only,
checker-only, or solver-only increment is not this slice.

Route (a) remains the `OQ-12` target on intrinsic merits. This named slice
prices its residual mechanization risk without changing priority, selecting a
proof-artifact home, building route (b), or extending the atom theories.

Cost note: the embedding adds a `World` argument to every predicate and emits
the closed frame/domain theory, so it is reserved for FO; D uses the direct
decision route in §3.

## 5. Fragment HO — native intuitionistic + tactics

- **Propositional skeleton.** The intuitionistic propositional structure of a
  goal is decided by a kernel-verified **IPC decision procedure** (an
  `Itauto`/`intuit`-style reflective tactic): it returns a **proof term** (→
  `proved`) or a **Kripke counter-model**. The counter-model's verdict follows
  §1.2 and `24 §1`/§3 — **not** "invalid ⇒ disproved": a model that **forces
  `¬φ`** (the `S_{¬φ}` region, `24 §3`) is `disproved`; a model that merely
  **fails to force `φ`** while `¬¬φ` still holds — the `¬¬φ ⇒ φ` gap,
  e.g. an abstract-atom LEM instance `p ∨ ¬ p` (intuitionistically
  invalid but **not refutable**, since `¬(p ∨ ¬ p)` is itself false) — is
  **`unknown`**, not `disproved`. (The de Bruijn discipline still holds either
  way: `proved` requires the returned proof term to `check`.) This handles the
  connective scaffolding even when atoms are abstract.
- **Induction / higher-order.** Goals needing induction over an inductive
  family, or quantifying over types/predicates, are out of SMT scope. The prover
  applies a small library of **tactics**
  (intro/apply/induction/rewrite-by-`Eq`/`decide`) and, where automation stops,
  leaves a **typed hole** with the remaining goal and context for an agent or
  human to fill (`24 §2`, the REPL loop `21 §3`).
- **Sub-obligation descent + certificate composition (the V2-descend carry).** A
  tactic that **decomposes** a structured goal generates **sub-obligations**,
  each itself routed (§2.1) and discharged, with the certificate **composed**
  from the parts — never a single opaque obligation over the whole structured
  term:
  - **∧-split / all-prop record goal** `φ ∧ ψ` → subgoals `φ`,
    `ψ`; certificate is the pair `(p_φ, p_ψ)` (`16 §1.3`).
  - **⇒/∀-intro** → move the antecedent / binder into `Γ`, discharge the body;
    certificate is the `λ`.
  - **induction over an inductive family** → **one subgoal per constructor**,
    and each recursive-field subgoal carries the **induction hypothesis** — a
    direct motive instance `M zᵢ`, a Π-abstracted instance, or the structurally
    lifted instances for nested content (`14 §3`–`§3.2`) — in its `Γ`, exactly
    the body-as-motive structure V2 builds at extraction (`22 §4`); here the
    *tactic* synthesizes it. A single goal over the whole inductive structure
    carries **no IH** and cannot be discharged — the descent is **required, not
    an optimization**. The cert is the eliminator application `elim_D M methods…
    z` whose methods are the per-constructor sub-certificates (`14 §3`).

  The composed certificate is **one core term**, `check`ed once at the top goal
  (`18 §4.5`); a sub-certificate the prover cannot build leaves a **typed hole
  at that subgoal** (precisely located, `24 §2`), turning *that* leaf `unknown`
  while its siblings stay `proved` — partiality is per-subgoal, not
  all-or-nothing. (This is the prover-side instance of the obligation-descend
  discipline V2 applies at extraction: an obligation over an eliminator must
  split per-branch with the IH, or it is unprovable.)
- Full higher-order *automated* proving is an explicit non-goal
  (`../../docs/program/01-strategy.md`); interactive tactics + the agent loop
  serve instead.

## 6. Backend scope (the V3 work)

V3 builds the SMT-backed tiers in full: arbitrary decidable atoms over
`Int`/`Decimal`/`Bool`/handles and finite domains (D); the full Kripke embedding
for FO; the IPC tactic and the induction tactics for HO. There is **no external
proof-checker dependency**: Ken's own kernel is the proof checker, so an
external Coq dependency would enlarge the trusted base against the
small-permanent-Rust-kernel principle (ADR 0001/0004). Z3 is the primary solver;
**cvc5** is an optional second solver (proof-friendly Alethe/LFSC output, useful
for the (a) checker and for cross-checking).

## 7. Soundness obligations (what must actually be proved/ensured)

1. **Kernel re-checks every certificate** (§1) — the backbone; nothing else here
   can break soundness if this holds.
2. **The Kripke embedding's adequacy theorem** (§4) — mechanized once as a
   kernel meta-lemma (route a, the target), paired with the verified certificate
   checker; reconstruction (b) is the feasibility hedge. Needed for the FO tier
   either way.
3. **Reflective decision procedures are kernel-verified** (§3, §5) — `dec`
   returns a genuine `Decidable φ`, checked by the kernel like any term.
4. **The classifier is exhaustive** (§2.1) — every obligation is routed to
   *some* outcome (no silent drop). This is **NOT kernel-enforced** and is the
   verification-soundness linchpin on the prover side: a never-routed obligation
   supplies no cert and no hole, so it escapes `trusted_base()` and reads as
   discharged though never attempted (the two-soundnesses *omission* gap, `22`).
   Backstopped **only** by the structural totality of `classify` (§2.1).
5. **The classifier is conservative** (§2) — routes *upward* when unsure. A
   **quality** property only; even a non-conservative classifier cannot break
   (1), since the certificate is re-checked regardless.

Only (1) and (3) are *enforced* by the kernel automatically; (2) is a proof
obligation on the prover's construction (§4.4); (4) is a **structural
completeness** obligation on the classifier — *not* kernel-caught, discharged by
exhaustive-by-construction routing (§2.1); and (5) is a quality property. The
solver is never trusted. Whether the approved home and evaluator for the two
kernel-facing route-(a) theorems change the trusted-base account remains the
Architect/operator placement question in §4.4; this chapter does not settle it.

**Two classical bridges, not one (contrast with the Ward seam).** This chapter's
bridge uses a classical solver to discharge an obligation **here, with a kernel
certificate** — the result becomes `proved`. The downstream **behavioral seam**
(`../70-behavioral/71`) also runs a classical engine under Ken's logic, but its
results are **not re-checkable as Ken proofs** (a depth-`k` model-check is not a
proof for all `N`; a green monitor is not a proof). So that bridge is
**one-way** (`OQ-classical-bridge`): Ken exports obligations + assumptions, Ward
discharges them, and the outcome returns as a signed **discharge attestation**
(`../60-security/63 §5a`) tagged `delegated`/`tested` — **never promoted to
`proved`**. Soundness is by *assume-guarantee construction* (Ken proves `Q ⊣ P`;
the discharge of `P` is a separate, lower-trust artifact), and the strong,
*Ken-checked* part is **translation faithfulness** (`../70-behavioral/71 §5`),
the exact analog of §4's adequacy lemma — proved once at the compiler level.

## 8. Level-discipline reconcile

Per the standing directive, the level computations are made explicit and
reconciled against `12`/`16 §1.1`. V3 introduces **no new kernel former or
universe**, so the reconcile is mostly accounting that nothing bumps a level:

- **Goals stay in Ω at V2's level.** Every obligation goal is `φ : Ω_ℓ`
  (`22 §1`/§7); the prover *consumes* it and *produces* a proof `p : φ` at the
  **same** `Ω_ℓ` — proof terms in Ω are proof-irrelevant and erasable
  (`16 §1.2`), so the certificate adds no level. `check(env, Γ, p, φ)` is an
  ordinary kernel check (`18 §4.5`); no level appears beyond the goal's.
- **The Kripke theory is external; its quotation is data.** The meanings of
  `World`, `Le`, `Dom`, and `Force` (§4.2) live in the classical FO problem and
  have no Ken universe level. `IForm`, `Form`, and `Cert` (§4.3) are Ken data
  that quote that problem; the certificate and final discharge are Ken terms.
- **The reflective types are ordinary data.** `IForm`, `Form`, `QTerm`, `Cert`,
  `Decidable P` (§3), and the IPC proof terms (§5) are **derived inductives**
  (`14`, `16 §1.3` connectives + `Empty`) at their natural `Type l`. The
  checker `check_cert : Form -> Cert -> Bool` is an ordinary total function.
  None introduces a universe or a proposition former.
- **The adequacy + checker-soundness meta-lemmas** (§4.4) are themselves
  kernel-checked terms whose statements are propositions (`→`/`∀` over the
  reflective data, landing in Ω by codomain-keying, `16 §1.1`); they reuse Ω and
  `Eq`, introducing no new universe. Consistent with `12`'s predicative,
  non-cumulative regime — no implicit lifts anywhere in the prover.

## 9. What WS-V must deliver here (V3)

The per-obligation contract emitting the **verdict trichotomy** (§1.2) keyed by
`id` for V1's status projection (`21 §5.3`), with the honesty guard
kernel-structural via `trusted_base()` (§1.3); the **exhaustive** classifier
(D/FO/HO with HO the default, §2.1); reflective decision for D + SMT
search/reconstruction; the Kripke embedding + the **reflective certificate route
(a)** — the closed theory, mechanized adequacy, and a verified `check_cert`
(§4) — with (b) reconstruction as a feasibility hedge; the IPC reflective
tactic and the core induction/rewrite tactics with **per-branch sub-obligation
descent + certificate composition** (§5); generalization beyond the naturality
domain; and the documented guarantee (G3) that the classical solver cannot yield
a false `proved`. Acceptance ties to **G3**. Conformance:
`../../conformance/verify/prover/` — a decidable arithmetic goal (reflective);
an FO-intuitionistic goal via the embedding (re-checked certificate); an IPC
propositional goal; an `unknown` goal whose typed hole is `trusted_base()`-
distinct from `proved` (the absence-assertion, §1.3, naming its guard —
postulate membership); an **exhaustive-classifier** case driving an obligation
of each shape through `classify` with none silently unrouted (§2.1, structural);
and a **soundness regression** in which Z3 "proves" a classically-valid-but-
topos-invalid `φ` whose certificate the kernel **rejects** — the verdict-flip
(`proved` → not `proved`) showing the de Bruijn criterion is load-bearing.
