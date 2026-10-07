# Specification syntax

> Status: **V1 elaborated** (implementation-ready). Normative for the forms,
> their meaning, their grammar/AST, their **elaboration to core**, and the
> **verification status model**; concrete surface spelling cross-refs
> `../30-surface/`. Contract for WS-V **V1** (the first WP of the verification
> spine V1→V2→V3). **★★ (untrusted):** everything this layer emits is re-checked
> by the kernel (`../10-kernel/18 §4`); a bug here is a wrong verdict or a poor
> diagnostic, **never** unsoundness. How a programmer or agent attaches a
> *correctness specification* to code — the surface the whole verification loop
> hangs off. This is the L1→L2 bridge at the surface level.
>
> **Staging.** Subset-Σ refinement elaboration (§2/§6.3) and transitive
> proof honesty (§5.4) are the normative W1 contract. The current
> carrier-only implementation is transitional until W5 replaces it;
> the read-only reachability query arrives with W4. This is not a claim
> that either implementation already matches these rules.

A **specification** is one or more **propositions** (`../10-kernel/12 §5`,
elements of Ω) attached to a definition, asserting how it must behave. Ken
offers three layered ways to state one, from lightest to most expressive (§1–3),
discharged through one smooth **gradient** — declarative contract → automatic
proof → typed hole → tactic/term (`23`), never a cliff between "automatic" and
"hand-written."

**The defining discipline (`OQ-spec`, DECIDED).** Because Ken is a *software
engineering* language — written by agents, read by humans — every claim carries
a **visible, exportable epistemic status**: **proved** (kernel-discharged),
**tested** (assumed, with a runtime/test obligation), **delegated** (a
temporal/behavioral property Ken can *state* but not close as a static
proposition — exported downstream), or **unknown** (an open typed hole). The
source distinguishes proof from test from model from gap *on its face* (§5); the
delegated/tested set is emitted as the **assumption boundary** consumed by the
behavioral sibling (`../70-behavioral/`, ADR 0006). This is the surface form of
"prove what can be proven and state what must be tested."

**What V1 elaborates (this chapter's scope).** The three spec forms (§1–§3) with
**concrete grammar and AST** (§6.1–§6.2) extending the V0 surface
(`../30-surface/39-elaboration.md §5`); their **elaboration to core** as
defensive pseudocode (§6.3–§6.5); the `old`-capture rule (§6.4); the
**verification status model** — the per-obligation *verdict* and the per-claim
*epistemic status*, with each verdict's carried evidence and the honesty guard
(§5); and the **V1→V2 interface** the obligation generator consumes (§7). The
concrete clause spelling for the **disposition tags** `tested`/`assume`/`test`
stays **reserved** (`OQ-syntax`, deferred — §5.5): V1 fixes what those statuses
*mean* and how they project to the assumption boundary, not their surface
grammar. The *prover* (`23`), *obligation extraction* (`22`), and *diagnostics*
(`24`) are downstream WPs.

## 1. Function contracts — `requires` / `ensures`

The everyday form: pre- and post-conditions on a function.

```
fn divide (n : Int) (d : Int) : Int
  requires  Not (Equal Int d 0)
  ensures   Equal Int (result * d + (n % d)) n
= n / d
```

- **`requires φ`** — a **precondition**: a proposition `φ : Ω` over the
  function's parameters. Callers must establish it; inside the body it may be
  assumed.
- **`ensures ψ`** — a **postcondition**: a proposition `ψ : Ω` over the
  parameters *and* the special binder **`result`** (the return value). The body
  must establish it.
- Multiple `requires`/`ensures` clauses conjoin. Clauses may mention earlier
  parameters (they are checked in the telescope `../10-kernel/13 §3`).
- Contracts are **erasable**: they generate obligations (`22`) and assumptions
  but no runtime code by default (runtime-checked contracts are an opt-in, §5).

The clause grammar (the `fn`-declaration addendum; the full grammar is §6.1):

```
contract ::= requires-clause* ensures-clause*
requires-clause ::= "requires" prop        -- a precondition proposition (: Ω)
ensures-clause  ::= "ensures"  prop         -- a postcondition (: Ω, may use `result`)
prop ::= expr                               -- any expression that checks at Ω (§4)
```

Semantically, a contract **denotes** a **refined function type**
(`../30-surface/39-elaboration.md`): `divide` above denotes the dependent type

```
(n : Int) (d : Int) → { Not (Equal Int d 0) }
  → (r : Int) × (Equal Int (r * d + (n % d)) n)
```

i.e. **written** preconditions are extra Π proof arguments and the
postcondition is a **kernel subset-Σ result** pairing the returned value
with its checked proof (§6.3). For an ordinary `fn`, each body leaf
constructs that pair; a `space` operation instead pairs its *residual
tree* with one `AllRet` proof (§6.4). An open hole is applied as the
proof component and remains visible in `trusted_base()` (§6.5). The
programmer writes `requires`/`ensures`; erasure removes the proof only
from the runtime representation (`42 §3.2`), never from the core type.

## 2. Refinement types — `{ x : A | φ x }`

A **refinement type** is the comprehension subobject (`../10-kernel/12 §5`,
`../30-surface/34-data-match.md`): the type of `x : A` *for which `φ x` holds*.

```
def Pos = { n : Int | IsTrue (leq_int 1 n) }
fn head (xs : { l : List A | Not (Equal (List A) l (Nil A)) }) : A = …
  -- non-empty by type
```

- `{ x : A | φ x }` requires `A : Type ℓ_A` and `φ x : Ω_ℓ_φ` under
  `x : A`. It elaborates to the **core dependent pair** `Σ(x:A').φ'` at
  `Type (max ℓ_A ℓ_φ)` (`13 §4`). The first component is relevant; only its
  proposition-valued proof component is proof-irrelevant (`16 §1.2`).
- Refinements are the **route to L2 at the surface**: introducing a plain
  `a : A` at the refined type constructs `(a', π)` with a kernel-checked
  `π : φ[a'/x]`, recording the refinement obligation and discharging it with
  checked evidence or an explicit typed hole (`22 §2.1`). An open obligation
  is a typed postulate, not permission to omit the pair's proof term.
- They compose with Π/Σ: arguments, results, and record fields may be
  refined **at their stated type**. A refined domain remains `Σ(x:A).φ` in
  its Π type; it is not normalized to an `A` binder and an implicit proof
  binder. Explicit written `requires` still contributes a separate Π proof
  argument; an ordinary `fn`'s `ensures` refines the result to a kernel
  Σ with per-leaf introductions (§6.3). A `space` operation pairs its
  residual tree with `AllRet` evidence instead (§6.4).

**The core encoding (normative).** `elabType({x:A|φ}) = Σ(x:A').φ'`, with `A'`
checked in `Type` and `φ'` in Ω under `x:A'`. A transparent named refinement
unfolds to that Σ, not to its carrier. Operationally,
`is_refinement(Γ,T)` holds **iff** `whnf(T) = Σ(x:A).φ` and
`classify(Γ,x:A,φ) = Ω_ℓ` for some `ℓ`; neither a type's spelling nor an
elaborator-only refinement fact decides the coercion. The kernel
distinguishes a refined
value from a bare carrier (`Σ(x:A).φ` is not convertible to `A`); it checks
both the pair's carrier and its proof. This is a **subset** Σ in `Type`, not
an Ω-valued conjunction: `sort_sigma(Type ℓ_A, Ω_ℓ_φ) =
Type (max ℓ_A ℓ_φ)` (`13 §4`). Proof irrelevance identifies pairs with
convertible carrier components and different proofs, **not** pairs with
unequal carrier components. No conversion rule is added for this case: typed
Σ-η, Ω proof irrelevance of the second component, and the existing typed Eq
endpoint comparison already suffice (`13 §2`, `17 §3`).

The kernel has **no subtyping**; the elaborator inserts only outermost,
type-directed coercions (§6.3). Introduction `A ≤ {x:A|φ}` constructs the
checked pair and owes `φ a`. Forgetting `{x:A|φ} ≤ A` is **free of obligations**
but emits `Proj1`, not the identity core term. To convert one refinement to
another, project the carrier and introduce the target proof under the source
pair's `Proj2` hypothesis. No coercion occurs **inside** `List`, a function
type, a binder, or any other type former; no function η-expansion invents a
proof for arbitrary arguments. Runtime erasure of the Ω proof recovers the
carrier value without changing the core type (`42`).

## 3. Propositional goals — `prove` / `law`

A **goal** is a standalone proposition to be discharged — a lemma, an invariant,
or an algebraic law — not attached to a single function's body.

```
prove  add_comm : (a b : Int) → Equal Int (a + b) (b + a)
law    Monoid (M) { assoc : … ; unit_l : … ; unit_r : … }   -- a property bundle
```

- **`prove name : φ`** registers `φ : Ω` as an obligation; on success `name`
  becomes a usable proof term of `φ` in the environment (`../10-kernel/11 §4`).
  Until discharged, `name` is an open obligation — a typed hole / visible
  postulate (status `unknown`, §5).
- **`law`** bundles related propositions (the algebraic-law form, stated via
  `law`/`verify`); proving a `law` for a type makes the bundle available as a
  record of proofs, usable by constraint resolution (typeclasses-as-subobjects,
  `../30-surface/33-declarations.md`). A `law` whose fields are all propositions
  is a conjunction of props — the sound `Σ`-of-Ω-into-Ω case (`16 §1.3`), so the
  bundle is itself a proposition.
- **Named proof claims** (`prop` / `theorem` / attached `proof`) are the same
  proof lane with different namespacing. `prop` declares a proposition family,
  `theorem` declares a standalone theorem in the ordinary module namespace, and
  `proof` attaches a theorem to a resolved subject path. All three are Ω-typed
  proof claims; none adds a new kernel declaration class or a trusted proof
  table.
- Goals are where Ken is used as a *proof assistant*, and where the REPL's
  "Little Prover" loop (`../30-surface/`, strategy T2) lives.

The clause grammar (the declaration addendum; full grammar §6.1):

```
goal-decl ::= "prove" ident ":" prop                  -- a named goal proposition
            | "law"   ConId "(" ident ")" "{" law-field (";" law-field)* "}"
law-field ::= ident ":" prop                          -- a named bundled proposition
prop-decl ::= "prop" ConId tyvar* binder* ":" type prop_block?
theorem-decl ::= "theorem" ident binder* ":" type "=" expr
proof-decl ::= "proof" ident "for" path binder* ":" type "=" expr
```

## 4. What the binders mean (precise)

| Binder / form | Scope | Type |
|---|---|---|
| parameters `x : A` | the whole contract + body | as declared |
| `result` | `ensures` clauses only | the function's return type |
| `old(e)` *(scoped to `space` ops)* | `ensures` of a `space` operation | value of `e` in the pre-state |
| `φ` in `requires`/`ensures`/`{·|φ}`/`prove` | as above | **must be `: Ω`** |

- Every specification proposition MUST type-check at `Ω` (`12 §5`, `16 §1.1`) in
  its scope; a `requires`/`ensures` whose body is not a proposition is a
  **surface type error** (caught at elaboration, §6.3), not a verification
  failure. This is a load-bearing guard: the elaborator `check`s each clause
  body at Ω and rejects a non-Ω body *before* any obligation is formed.
  Bool-valued comparisons such as `==` and `≠` (`33 §6.1`) do not become
  propositions implicitly: use `Equal`, `Not (Equal …)`, or `IsTrue` of a
  Bool comparison in a proposition position.
- A `prop` family result, standalone `theorem`, or attached `proof` theorem also
  MUST type-check at `Ω` in its scope. The attached form is still just a proof
  term; its canonical export name is `subject::proof_name`, and there is no
  separate proof-table lane.
- **`result`** is in scope only in `ensures` clauses; referencing it in a
  `requires` or a refinement predicate is a scope error.
- **`old(e)`** (referring to a pre-state value in a postcondition) is meaningful
  only for **`space` operations** (`../30-surface/36-effects.md §4.3`); for pure
  `fn`s the pre/post states coincide. **`OQ-Space` DECIDED:** `old(e)` is
  admitted, **scoped to a `space` operation's `ensures`** (a cell's pre-call
  value), well-defined because a space's denotation has an explicit
  `s_pre:S` input and a residual `ITree F (R × S)` result; it collapses
  to `S → R × S` when `F=𝟘` (§6.4, `36 §4.2`). There is **no global
  `\old`/heap** and **no separation logic** — a space's cells are non-aliased,
  so reasoning is bounded per-space Hoare. An `old` outside a `space`-op
  `ensures` is a scope error (§6.4). For explicitly-threaded state you simply
  name the pre/post values.

## 5. The verification status model

A spec'd program returns, per claim, an **actionable status** — the feature that
makes Ken "Ken." Two distinct classifications are in play, and conflating them
is the chapter's central hazard, so they are separated here:

- the **verdict** (§5.1) — the *operational* outcome of attempting one
  obligation, with its **carried evidence**. Per-obligation, produced by `22`/
  `23` and rendered by `24`/`25`. The kernel/Heyting **trichotomy**:
  `proved` / `disproved` / `unknown`.
- the **epistemic status** (§5.2) — the *export-facing* label a claim carries
  (`OQ-spec` DECIDED). Per-claim, visible in the source and in the assumption
  boundary. The **four-way**: `proved` / `tested` / `delegated` / `unknown`.

§5.3 is the projection between them; §5.4 is the honesty guard (an `unknown`
must never read as `proved`); §5.5 is the V1 scope ruling on disposition-tag
syntax.

### 5.1 The verdict (per-obligation, operational)

Attempting an obligation `Γ ⊢ φ` (`22 §1`) yields one of **three** verdicts —
the surface rendering of the kernel trichotomy (`16 §1`, `24 §3`), the spine
of the protocol's verdict (`25`) — each carrying the evidence that makes it
actionable and (for `proved`) re-checkable:

| Verdict | Meaning | Carried evidence | Kernel re-check |
|---|---|---|---|
| `proved` | the obligation is discharged | a **certificate**: a core proof term `p` with `Γ ⊢ p : φ` | `check(env, Γ, p, φ)` accepts (`18 §4.5`) — the de Bruijn criterion |
| `disproved` | the proposition is refuted | a **countermodel**: a finite Kripke model forcing `¬φ` at some world (`24 §1`) | where the prover yields a proof of `¬φ`, `check(env, Γ, p, ¬φ)` certifies it; else the countermodel is a prover-asserted refutation (untrusted, but a concrete falsifying witness) |
| `unknown` | undecided / not discharged | a **typed hole** `?h : φ` in `Γ`, admitted as a **postulate** of `φ` (`24 §2`) | none — the hole is *assumed*; it appears in `trusted_base()` (§5.4) |

- A **`proved`** verdict has **no reachable open obligation hole** in the
  checked certificate's dependency closure (§5.4). Audited contract axioms
  remain visible assumptions without demoting the verdict; a wrong
  certificate fails kernel checking (`18 §5`).
- A **`disproved`** verdict is a hard **verification error** (`24 §3`, the
  `S_{¬φ}` region: *fix the code or the spec*). It is **never** an exported
  guarantee — you do not ship a known-false claim — so it has *no* epistemic
  status (§5.3); it surfaces as a diagnostic (`24`).
- An **`unknown`** verdict leaves the program **running**: the hole is a visible
  postulate and evaluation propagates the runtime third value `unknown`
  (`24 §2`, `../40-runtime/42-evaluation.md`). Verification is *incremental*.

### 5.2 The epistemic status (per-claim, export-facing — `OQ-spec` DECIDED)

Every specification claim carries one of four **statuses**, each visible in the
source and (for the latter three) carried in the **assumption-boundary export**
(`../70-behavioral/`). This four-way distinction is the heart of `OQ-spec` and
the feature that makes Ken a *software engineering* language rather than a
programming language: a reader sees, per claim, whether it is *proved*, merely
*tested*, *delegated* to behavioral checking, or still *open*.

- **`proved`** — the obligation (`22`) was discharged, the kernel re-checked
  the certificate (`23`, `../10-kernel/18 §4`), and no **open obligation
  hole** is reachable through its dependencies (§5.4). The default for
  a contract that goes through. No annotation; the claim remains relative
  to any audited contract axioms on its dependency closure, which stay
  visible in the assumption boundary.
- **`tested`** — a property that **cannot (yet) be proven** but is **asserted
  with a runtime/test obligation**: an `assume`/`test`-tagged clause (the
  keywords are reserved, `../30-surface/31 §4`; the exact clause grammar is
  `OQ-syntax`, §5.5) that lowers `requires`/`ensures` to a runtime assertion
  (boundaries, FFI, untrusted input) *and* registers a test/generator
  obligation. It is **visible** — a reader knows this guarantee rests on tests,
  not proof — and it is **exported** as part of the assumption boundary (the
  refinement predicate becomes a generator/oracle spec, `../70-behavioral/`,
  §2/Layer 2).
- **`delegated`** — a **temporal/behavioral** property (liveness, fairness,
  ordering, eventual consistency, an interleaving safety property) that is **not
  a static proposition over a pure function** and so cannot be closed in the
  kernel. Ken can *state* it — as ordinary **deeply-embedded temporal-logic
  data** (an inductive `Temporal`/μ-calculus value, *not* a kernel modality, so
  the TCB is untouched, `../70-behavioral/`) — and **exports** it to the
  behavioral sibling for model-checking and runtime monitoring. Stated here,
  discharged there. *(This is "the fourth" status the verification spine adds
  beyond the proof trichotomy: it carries a model-checking/monitoring
  obligation, not a kernel proof obligation.)*
- **`unknown`** — the obligation is *not* discharged and no test/delegation is
  given: the definition is admitted with a **typed hole** and the program
  **still runs**, the result carrying `unknown` where the unproven property is
  observed (`24-diagnostics.md §2`). Verification is *incremental*, not
  all-or-nothing (Hazel-style); a hole is the honest "not done yet."

By default `proved` specs are static-only (erased); `tested` adds runtime code
by construction; `delegated` adds none to Ken (it is exported); `unknown` adds
none. The **assumption boundary** (`../70-behavioral/`) carries the
`tested`/`delegated`/open-`assume` claims **and** the audited
`trusted_base_delta` of contract axioms and open holes, with trusted
primitive dependencies visible through `trusted_base()` (`18 §5`).
A checked proof may be `proved` relative to a recorded contract axiom;
that axiom remains an assumption entry, not a demotion of the claim.
The boundary tells the sibling what must be modelled, tested, monitored,
or accepted as an explicit trust premise.

### 5.3 How the verdict and the status relate (the projection)

The epistemic status of a claim is the claim's **disposition** (how the author
chose to establish it — prove it, test it, or delegate it) resolved with the
**verdict** of attempting it:

| Disposition | Verdict (§5.1) | Epistemic status (§5.2) | Carried evidence |
|---|---|---|---|
| prove (default) | `proved` | **`proved`** | certificate (kernel-checked) |
| prove (default) | `unknown` | **`unknown`** | typed hole = postulate |
| prove (default) | `disproved` | *(none — a verification error, `24`)* | countermodel |
| `test`/`assume` | — *(not statically attempted)* | **`tested`** | runtime/test + generator obligation |
| `delegate` (temporal) | — *(not a static proposition)* | **`delegated`** | temporal-logic export to `../70-behavioral/` |

So the frame's "four-way status" is the **epistemic status** (four labels,
DECIDED); the operational **verdict** is the trichotomy (three outcomes). A
`disproved` verdict has no exported status because a refuted claim is fixed, not
shipped; the `tested`/`delegated` statuses sit beside the proof axis entirely,
carrying downstream (test/monitor) obligations rather than a kernel verdict.

### 5.4 The honesty guard (`unknown`/`tested`/`delegated` never read `proved`)

The load-bearing property of the whole model (§the framing in
`docs/wp/V1-spec-syntax.md`): a partially verified claim must **never**
masquerade as an unconditional proof. A valid proof term can depend on a
**different** open obligation through a transparent definition. In particular,
`c : Σ(x:A).φ x` may contain `(a, ?h)` and `c.2 : φ a` is well-typed, but its
use in a proof does not discharge `?h`. Checking only the current goal's own
hole or looking for a postulate *with that goal* in `trusted_base()` would
falsely mark such a proof as `proved`.

The discriminator is the kernel's checked term **and its transitive
assumptions**, not a V-layer status string:

- The certificate must pass `check(env, Γ, p, φ)` (`18 §4.5`); a plausible
  self-reported proof without a checked term never suffices.
- A read-only kernel query `postulates_reachable(env, p)` traverses the
  certificate and the δ-closure of its transparent dependencies, returning
  the postulate identities it actually relies on. It is off the kernel's
  checking and conversion paths: dependency inspection adds no reduction or
  equality rule. A claim is `proved` only if this set contains **no open
  obligation hole**. Audited contract axioms (`Ord Int`, StringBijection)
  and trusted primitives on the dependency closure remain visible in the
  assumption boundary; they do **not** by themselves demote a checked
  certificate.
- The same rule applies to prover certificates and term proofs (`theorem` and
  attached `proof`). A valid term using the second projection of a refined
  constant with an open proof component is **not** unconditionally `proved`.
  `tested`/`delegated` remain their distinct downstream dispositions (§5.2).

An open hole is still a postulate in `trusted_base()` (`24 §2`); retiring it
requires a checked certificate. But a postulate's membership alone is not a
proof-dependency test — reachability from the **claim's term** decides whether
an **open obligation hole** contaminates that claim. "Shipping a verified
artifact" requires no such reachable open holes on each exported proof's
dependency closure, while every audited contract axiom remains listed in the
assumption boundary. Conformance must contrast a certificate with no reachable
open hole against one that checks yet reaches an open hole through a transparent
refined constant; checking the goal's own hole only cannot distinguish them.

### 5.5 Scope ruling — disposition-tag syntax is deferred

V1 delivers the **status model** above (all four statuses' meaning, the verdict
trichotomy, the projection, the honesty guard) and **concrete grammar** for the
proof-disposition forms `requires`/`ensures`/`{x:A|φ}`/`prove`/`law` (§6.1).
The **disposition-tag clause spelling** — how `tested`/`assume`/`test` and a
`delegated` temporal clause attach to a declaration — stays **reserved**
(`assume`/`test` are reserved keywords, `31 §4`; the clause grammar is
`OQ-syntax`). It is deferred because the `tested`/`delegated` paths depend on
the behavioral sibling (`../70-behavioral/`) and the test/generator framework
(`../50-stdlib/`), which are downstream of V1. Conformance for V1 therefore
exercises the *status model* (verdict-distinct, the honesty guard) and tags any
`tested`/`assume`-spelling case **deferred** rather than asserting un-landed
grammar.

## 6. Surface syntax, AST, and elaboration to core

V1 **extends** the V0 surface (`../30-surface/39-elaboration.md §5`) — its
lexer, AST, parser, resolver, and bidirectional elaborator — with the spec
forms, and **reuses** the kernel term constructors V0 never emits (`Pi`,
`Sigma`, `Pair`, `Proj`, `Omega`, `Eq`; all already in the kernel `Term`, none
`[K2]`-reserved). V1 **introduces** the verdict/obligation/status vocabulary
(none exists in the codebase today). Like all of V0, the output is
**kernel-re-checked**: a bug yields a rejected program or an open hole, never an
unsound acceptance (`39 §1`).

### 6.1 Grammar and lexer addendum

**Lexer (`31 §4`, the V0 lexer of `31 §8`).** V1 reserves the keywords
`requires ensures prove law` (already in the §4 keyword table) and the
refinement brackets `{ } |` (the spec brace, `31 §2` punct). `assume test` stay
reserved but their clause grammar is deferred (§5.5). `old` is a **contextual**
keyword — recognized only inside a `space`-op `ensures` (§6.4) — not globally
reserved.

**Grammar (the `32` brace form, extending `39 §5.2`).** The spec forms attach to
declarations and types:

```
fn-decl ::= "fn" ident binder+ (":" type)? contract? "=" expr
contract  ::= requires-clause* ensures-clause*
requires-clause ::= "requires" prop
ensures-clause  ::= "ensures"  prop

type      ::= … (V0 type forms) …
            | "{" ident ":" type "|" prop "}"       -- refinement {x : A | φ}

goal-decl ::= "prove" ident ":" prop
            | "law" ConId "(" ident ")" "{" law-field (";" law-field)* "}"
law-field ::= ident ":" prop

prop-decl  ::= "prop" ConId tyvar* binder* ":" type prop_block?
theorem-decl ::= "theorem" ident binder* ":" type "=" expr
proof-decl ::= "proof" ident "for" path binder* ":" type "=" expr

prop      ::= expr   -- an ordinary expression; elaboration checks it at Ω (§6.3)
```

A `prop` is syntactically just an expression (no separate proposition grammar);
its **Ω-typing** is enforced at elaboration, not parse time (§6.3). `result` and
`old` are ordinary identifiers at parse time; resolution (§6.3/§6.4) gives them
their binder meaning in `ensures` scope. `prop`, `theorem`, and attached `proof`
all elaborate as proof claims in the same Ω-checked lane.

### 6.2 Surface AST extension

Grounded against the landed V0 AST (`crates/ken-elaborator/src/ast.rs`), V1 adds
spec-carrying fields and one type variant (and the resolver/`RType` mirrors):

```
Decl  ::= ViewDecl name (binder list) (Type option) (Expr list) (Expr list) Expr
                                       -- params, result, REQUIRES, ENSURES, body
        | LetDecl  name (Type option) Expr
        | ProveDecl name Prop          -- new: prove name : φ
        | LawDecl   name name (LawField list)   -- new: law Name (M) { … }
        | PropDecl  name (tyvar list) (binder list) Type (PropIntro list)
        | TheoremDecl name (binder list) Type Expr
        | ProofDecl name Path (binder list) Type Expr
LawField ::= name Prop
PropIntro ::= name Type

Type  ::= …                            -- V0 forms (TPi, TArr, TUniv, TCon, TVar)
        | TRefine name Type Prop span  -- new: { x : A | φ }

Prop  ::= Expr                         -- a proposition is an expression (checked at Ω)
```

The `requires`/`ensures` lists hang on `ViewDecl` (the existing
function-declaration node gains two `Expr list` fields); the refinement gains a
`Type` variant; `prove`/`law` are new top-level `Decl`s. `ViewDecl` is the
retained internal AST variant name, not the retired source keyword: the surface
spelling is `fn` (§6.1). Nothing changes in the V0 term-only nodes — non-spec
programs parse to exactly the V0 AST (acceptance §5, no regression).

### 6.3 Elaboration to core (the algorithm)

Elaboration extends the bidirectional V0 walk (`39 §5.4`: `infer`/`check`/
`elabType` over a kernel `Context`). The spec forms lower as follows; the
pseudocode is **defensive** — every position that *must* be a proposition is
explicitly `check`ed at Ω (a non-Ω body is a surface error, never silently
admitted), and every obligation site explicitly emits a typed hole.

**Function contract** — `requires`/`ensures` on an `fn`. Refined parameters
retain their subset-Σ types in the Π telescope. **Written** `requires` clauses
become Π proof arguments, assumed in the body and discharged at call sites;
`ensures` forms a **real subset-Σ result type**, checked at each body leaf:

```
elabFn(Σ, ⟨ fn f (Δ) : B requires φ̄ ensures ψ̄ = body ⟩) → (coreDef, obls):
  Δ' := elabTelescope(Δ)                         -- refined domains stay Σ(A,φ)
  Γ  := extendTelescope(·, Δ')
  for φᵢ in φ̄:
    φᵢ' := check(Γ, φᵢ, Ω)                      -- written precondition MUST be Ω
    Γ   := extend(Γ, pᵢ : φᵢ')                  -- explicit proof argument
  B' := elabType(Γ, B)                          -- B' : Type ℓ_B
  for ψⱼ in ψ̄:
    ψⱼ' := check(Γ, result : B', ψⱼ, Ω)          -- ψⱼ : Ω under result : B'
  resultTy := if ψ̄ = ∅ then B'
              else Σ(result : B').(ψ₁' ∧ … ∧ ψₙ')
  b := check(Γ, body, resultTy)                  -- push expected Σ to each leaf
  coreTy := Π(Δ'). Π(p̄ : φ̄'). resultTy
  coreTm := λ(Δ'). λ(p̄). b
  return (declare_def-checked coreTm : coreTy, obligationsFromLeaves(b))
```

- `Π(p : φ : Ω). Rest` keys its formation sort on `Rest` (`16 §1.1`), so an
  Ω domain does not collapse a Type-valued function type. These **written**
  proof arguments are erased at runtime and owed at the call (`22 §2.3`);
  they are distinct from a refined domain, which stays a Σ.
- The postcondition is **not** an obligation over a bare `B` and is **not** a
  second, verification-only motive. Each leaf constructs `(bₖ,πₖ)` at the
  checked `resultTy`; each `ψⱼ[bₖ/result]` has its own obligation identity
  and provenance, combined as the Ω proof `πₖ` (`22 §2.2`, §4). A direct
  self-call has the declared Σ result type, so its `Proj2` supplies evidence
  for its postcondition; an eliminator's Σ induction hypothesis does likewise.
  If an obligation stays open, its applied, typed hole is the second component
  — the pair is never left without a kernel-checkable proof (`§6.5`).
- Erasure removes the proof from the **runtime representation**, not from
  `coreTy`: the returned value runs as `B'`, while the checked core keeps its
  subset-Σ type (`42`).

**Refinement type and coercion** — only the outer type is inspected:

```
elabType(Γ, {x : A | φ}) → Term:
  A' := elabType(Γ, A)                         -- A' : Type ℓ_A
  φ' := check(Γ, x : A', φ, Ω_ℓ_φ)              -- MUST check in Ω, not Type
  return Σ(x : A').φ'                          -- Type (max ℓ_A ℓ_φ)

coerce(Γ, e : S, T) → Term:
  if S ≡ T: return e
  if T whnf's to Σ(x:B).ψ with ψ : Ω:
    a' := coerce(Γ, e, B)
    π  := checkedEvidenceOrHole(Γ, ψ[a'/x], e)
    return Pair(a', π)                         -- checked at the target Σ
  if S whnf's to Σ(x:A).φ with φ : Ω:
    return coerce(Γ, Proj1 e, T)
  reject TypeMismatch
```

The checked-evidence search tries, in order, a kernel-checked `tt`, context
variables, `Proj2 y` for a refined variable `y`, and **term evidence** for
path conditions (`22 §3`). If none checks, it declares a typed obligation
hole closed over Γ, path conditions and hypotheses, and applies it to their
**actual terms** (`§6.5`). `Pair(a',π)` must check at the Σ whether the
obligation is already discharged or is still a visible postulate. The
fallback hole has closed type `Π Γ. Π conds. Π hyps. ψ[a'/x]`; its
application supplies **each** context variable and each term witness for
condition and hypothesis in order, rather than passing unchecked
propositions as if they were proofs. Testing the
**target first** means a refinement-to-refinement coercion can forget its
carrier and use the source's `Proj2` when introducing the target proof.

A check-only λ, literal, hole or pair literal expected at a refinement first
checks at its carrier, then introduces the checked pair **once at the check
entry**. An `if` or `match` checked at a refinement pushes that expected type
to its leaves; each leaf introduces under its own bound branch equation.
A `let` equation carries `refl`, since the kernel substitutes it. A
branch on scrutinee `s : S` binds its equation by the convoy motive
`λ y. Eq S s y → T`, applied to `refl s`, so each branch's path fact has
a **term**, not just a proposition in the obligation extractor (`22 §3`).
Forgetful use of a subset at its carrier inserts `Proj1` and incurs **no**
obligation. Before an elimination inspects an inferred type head
(application head, match scrutinee, binop operand, field projection or
condition), it similarly forgets an outer subset by projection, never
by equating Σ with A.

A **refined parameter** `(x : {y:A|φ})` remains a binder of type
`Σ(y:A').φ'`; its proof assumption inside the function has the checked term
`Proj2 x`. Calling that function with a plain `a:A` introduces the argument
pair at the **call**, owing `φ a` (`22 §2.1`). A refined domain in a written
higher-order function type stays a Σ domain too: `(x:{y:A|φ}) → B` is
`Π(x:Σ(y:A).φ).B`, **not** `(x:A) → (_:φ x) → B`. No coercion is inserted
under `List`, another type former, or a Π binder; no function η-expansion
manufactures proofs for unconstrained arguments. Thus `List {x:A|φ}` and
`List A`, or `(Five → Int)` and `(Int → Int)`, remain distinct kernel types.
A higher-order callee with a refined domain still owes the checked pair at
its call; a body-local hypothesis or known callee name cannot waive it.

**Goals** — `prove`/`law` lower to standalone obligations:

```
elabProve(Σ, ⟨ prove name : φ ⟩):
  φ' := check(Γ_binders, φ, Ω)                   -- the goal proposition; MUST check at Ω
  h  := freshHole();  emit ⟨h, Γ_binders ⊢ φ', prov⟩   -- standalone obligation (22, degenerate)
  bind name ↦ postulate(φ')                       -- name : φ usable as a proof term (11 §4)
  -- on discharge: certificate p with Γ ⊢ p : φ' is check'd; name ↦ p (postulate retired, §5.4)

elabLaw(Σ, ⟨ law Name (M) { fᵢ : φᵢ } ⟩):
  for fᵢ: φᵢ' := check(Γ_M, φᵢ, Ω)               -- each field a proposition; MUST check at Ω
  emit one obligation per field; the proved bundle is a record of proofs (33 §5)
```

`theorem` and attached `proof` are the same Ω-checked proof path with different
namespacing: `theorem` binds a standalone theorem in the module namespace, while
`proof` binds the theorem under `subject::proof_name`. `prop` family helpers
are checked at Ω in the family namespace and are ordinary proof terms, not a
separate kernel class.

### 6.4 `old`-capture for `space` operations

A `space` operation denotes a **state-transformer with residual effects**:
`run_state s ⟦body⟧ : ITree F (R × S)` (`36 §4.2`). Its `ensures ψ` is
an Ω predicate over **every return of that tree**, not a proof attached
to a user-written `Ret r` (the state-passing pair `(r,s_post)` is created
*inside* `run_state`). Define `AllRet` by the Ω-motive `elim_ITree` fold
in `36 §4.3`: `AllRet P (Ret rs) = P rs` and
`AllRet P (Vis e k) = Π(r:E.Resp e).AllRet P (k r)`.

```
elabSpaceEnsures(Γ, f, ψ):                 -- s_pre : S at transformer input
  Γ' := extend(Γ, s_pre : S, rs : R × S)
  resolve in ψ:
    old(e)       ↦ ⟦e⟧ at s_pre             -- pre-state value (36 §4.3)
    result       ↦ rs.1
    bare cell cᵢ ↦ proj_i(rs.2)           -- post-state value
  ψ' := check(Γ', ψ, Ω)                     -- MUST check at Ω
  t := run_state s_pre ⟦body⟧             -- t : ITree F (R × S)
  target := AllRet (λ rs.ψ') t             -- target : Ω
  π := checkedEvidenceOrHole(Γ, s_pre:S, target, t)
  resultTy := Σ(t : ITree F (R × S)).AllRet (λ rs.ψ') t
  return λ(s_pre : S).Pair(t,π)           -- checked at the outer Σ
```

For multiple written `ensures ψⱼ`, the outer pair's second component
is `∧ⱼ AllRet (λ rs.ψⱼ(s_pre,rs)) t`; each clause retains its own
obligation identity/provenance and contributes a checked proof term.
The single-clause form above is its one-member case. Written
`requires φ` remains a **separate Π proof argument** at the
space operation's call: its checked type is
`Π p̄. Π(s_pre:S).Π(h̄:φ̄(s_pre,p̄)).resultTy`, with one proof
argument per written `requires` clause. The outer proof `π` carries
**one recorded obligation per written `ensures` clause** for the
computed tree; V2 reduces each `AllRet` goal by ι over any known tree
prefix and leaves residual `Π`-quantified responses and return goals to
proof search (`22 §2.2`). An open hole remains an applied proof term in
the *outer* pair. A caller receives `Proj2 : AllRet (ψ s_pre) t` and may
compose it using `all_ret_bind` (a provable `elim_ITree` lemma in W5, not
an elaborator coercion). A recursive call likewise supplies `Proj2` at
`AllRet`, not at a bare leaf predicate. **No** pair is inserted under
`ITree F`, and `ITree F (R × S)` never converts to
`ITree F (Σ(rs:R × S).ψ)`.

When `F = 𝟘`, **elaboration collapses the whole result type** using
`ITree 𝟘 X ≅ X` (`36 §2.4`), yielding
`S → Σ(rs : R × S).ψ(s_pre,rs)` (with written `requires` arguments
retained). This is a consequence of the general rule, not a kernel
conversion `ITree 𝟘 X ≡ X` or an ill-typed `AllRet P` applied to a bare
`X`. The Ω proof is erased at runtime, not evaluated as data.

**The scope guard (discriminating, not coincidental).** `old(e)` is admitted
**only** when the enclosing declaration is a `space` operation — the one place a
distinct pre-state `s_pre` exists. In a pure `fn`'s `ensures` there is no
`State` effect, `s_pre ≡ s_post`, and there is no pre-state to bind: `old(e)` is
a **scope error**, rejected at elaboration before the kernel (`36 §7.3`). The
guard is the *kind of the enclosing declaration*, asserted explicitly — so the
conformance verdict flips on it (`old(c)` in a `space`-op `ensures` resolves to
`proj_i(s_pre)`; `old(x)` in a pure-`fn` `ensures` is rejected), never passing
vacuously. Worked example (`36 §4.3`): `inc`'s
`ensures Equal Int n (old(n) + 1)` gives the second component of the
checked result pair the goal
`Equal Int ((s_pre with .n := s_pre.n + 1).n) (s_pre.n + 1)`, which
computes by record-β/η (`13 §3`) to
`Equal Int (s_pre.n + 1) (s_pre.n + 1)`, discharged by `refl`
(`16 §2`).

### 6.5 The obligation-hole encoding (the `22` input)

Each contract/refinement/goal point emits an **obligation** — a triple
`⟨id, Γ ⊢ φ, provenance⟩` (`22 §1`) realized as a **typed hole** `?id : φ` in
`Γ`, admitted as a **postulate** of `φ` (`24 §2`). This is the single
representation that unifies "obligation," "typed hole," and "visible postulate":

- The program **still type-checks and runs** with open holes — each is an
  applied postulate in the checked pair and a visible trusted-base entry
  (§5.4), so the system is honest about what is assumed. Its Ω proof
  component is erased before runtime evaluation (`42 §3.2`); an open
  proof-only hole does not make the carrier value `unknown`.
- **Discharging** a hole means a certificate `p` with `Γ ⊢ p : φ` that the
  kernel `check`s (`18 §4.5`); its postulate is retired and the pair remains
  unchanged. The claim turns `proved` only if no other open obligation
  hole is reachable through `p` (§5.4); audited axioms remain visible.
- The holes are **precisely located** (provenance) and independent — provable in
  any order / in parallel (`22 §5`).

The interaction of the spec forms is uniform through this encoding: specs
elaborate to checked core (Σ, Π, Ω, `Eq`) **with typed proof terms** plus an
obligation set. Refinement introduction produces the checked `Pair(a,π)`;
when π comes from an open hole, its declaration is closed over Γ, condition
equations and hypotheses, then **applied to their kernel terms**. An
obligation-only fact without term evidence cannot be put into the pair.
Discharging the hole supplies a checked body in place of its postulate; the
pair remains intact. A spec proposition may use earlier checked lemmas,
`law`s, and refinements; transitive honesty (§5.4) still applies to any
reachable open hole.

## 7. The V1→V2 interface

V1 produces, per definition, exactly what obligation generation (`22`, V2)
consumes. The interface is four things:

1. **The elaborated core term** — kernel-checkable, with the contract
   encodings of §6.3: refined parameters as subset-Σ binders, written
   preconditions as Π proof arguments, and `ensures` as a subset-Σ result
   whose branches introduce checked pairs. V0 re-checks it (`18 §4`); a
   spec program with a type error has no core image and is rejected (`39 §3`).
2. **The obligation-hole set** — the ordered set of `⟨id, Γ ⊢ φ, provenance⟩`
   (§6.5), one per `ensures`/refinement-introduction/`prove`/`theorem`/`proof`/
   `law`-field site, each a typed hole `?id : φ` admitted as a postulate.
3. **The at-introduction hypotheses** — each hole's `Γ` already carries the
   facts in scope where the obligation *arose*: written preconditions and
   refined-parameter projections (`22 §3`). V2 **extends** each `Γ` with
   path-sensitive facts **and their term evidence** (let-equations,
   convoy-bound branch equations, Σ-result induction hypotheses —
   `22 §3`/`§4`); V1 provides the seed context, V2 the accumulation.
4. **The provenance** — source span + responsible clause per hole, for the
   diagnostics (`24`) and the protocol (`25`).

V1 does **not** generate verification conditions or walk the body for
path-sensitivity (that is V2's extractor) and does **not** discharge anything
(that is V3's prover). It hands V2 a spec-annotated, kernel-checked elaborated
form with the obligation sites marked. This is the V1→V2 contract acceptance
ties to (`docs/wp/V1-spec-syntax.md` acceptance §4).

## 8. Level-discipline reconcile

Per the standing directive, every formation in this chapter that produces or
manipulates a universe level is given its explicit level computation and
reconciled against `10-kernel/12-universes.md` and `16 §1.1` — *reconciled*, not
merely cited.

- **A spec proposition lands in Ω.** Every `requires`/`ensures`/refinement-
  predicate/`prove`/`law`-field body MUST check at `Ω_ℓ` for some `ℓ`
  (`12 §5`, `16 §1.1`: `Ω_ℓ : Type (suc ℓ)`, predicative, non-cumulative) in its
  scope; `Eq A a b : Ω_ℓ` for `A : Type ℓ` (`16 §2.1`). A body at `Type ℓ`
  rather than `Ω_ℓ` is a `TypeMismatch` at elaboration (§6.3), **not** a silent
  coercion — Ken has no `Type → Ω` inclusion (`16 §1.3`: only genuine
  sub-singletons enter Ω, and even those by explicit prelude declaration).
- **Precondition `Π` proof-argument.** `Π(p : φ : Ω_ℓ). Rest` with
  `Rest : Type ℓ'` has formation sort keyed on the **codomain** (`16 §1.1`;
  landed `sort_pi_sigma`): result `Type (max ℓ ℓ')`. The Ω **domain** does not
  lower the sort to Ω — so threading preconditions preserves the function type's
  `Type`-hood. This is the sound half of the codomain-keying (a Π *into* a
  relevant type stays relevant).
- **Subset-Σ refinement level.** `A : Type ℓ_A` with `φ x : Ω_ℓ_φ` under
  `x:A` forms `Σ(x:A).φ x : Type (max ℓ_A ℓ_φ)` (`13 §4`), not Ω. The
  relevant first component prevents proof-irrelevance from erasing A at the
  **kernel** level; an Ω-valued second component is proof-irrelevant but may
  raise the type's universe through predicative `max`. A higher-level
  predicate cannot silently remain at the carrier's level (`12 §2`/§3).
- **Postcondition certificate.** `ψ[b/result] : Ω_ℓ_ψ`; for an `ensures`
  result `B : Type ℓ_B`, the result is the checked core
  `Σ(result:B).ψ : Type (max ℓ_B ℓ_ψ)`. Its proof `p : ψ` is Ω-irrelevant,
  erasable at runtime and kernel-checked inside each result pair; an open
  typed hole is a visible postulate. For a `space` operation,
  `B = ITree F (R × S) : Type ℓ_T` and the Ω predicate is
  `AllRet (ψ s) t : Ω_ℓ_All`; its outer result is
  `Type (max ℓ_T ℓ_All)`. The `AllRet` fold's Π response domains
  contribute to `ℓ_All` predicatively (`36 §4.3`). Neither case is a
  separate obligation over a bare-returned carrier.
- **`prove`/`law`.** `prove name : φ` gives `name : φ : Ω_ℓ`. A `law` of all-Ω
  fields is a conjunction — the sound `Σ`-of-Ω-into-Ω case (`16 §1.3`,
  `sort_pi_sigma` with **both** components Ω) — so the bundle is itself a
  proposition; this is the *correct* use of codomain-keying (both sides props).
- **No new universes or formers.** V1 introduces no universe or proposition
  former — it reuses Ω (`16 §1`) and the derived connectives (`16 §1.3`). So the
  reconcile reduces to: every spec body lands in Ω at its scope's level, and the
  contract encoding uses the kernel's Π/Σ formation levels. Consistent with
  `12`'s predicative, non-cumulative regime — no implicit lifts; a
  level-mismatched proposition is a `TypeMismatch`, not a coercion.
- **Named proof claims stay in the same Ω lane.** `prop`, `theorem`, and attached
  `proof` bodies all check at `Ω`; the attached form is still an ordinary proof
  term attached to a subject path, not metadata, not a proof search table, and
  not a new kernel declaration kind.

## 9. What WS-V must deliver here (V1)

The spec syntax (`requires`/`ensures`, `{x:A|φ}`, `prove`/`law`) with concrete
grammar (§6.1) and AST (§6.2); its **Ω-typing** of every proposition (§4); its
**elaboration to core** as subset-Σ plus checked proof obligations (§6.3), the
`old`-capture rule for `space` ops (§6.4), and the obligation-hole form (§6.5);
the **verification status model** — the per-obligation verdict
trichotomy and the per-claim four-way epistemic status, the projection, and the
honesty guard (§5); and the **V1→V2 interface** (§7). The disposition-tag clause
spelling (`tested`/`assume`/`test`) stays reserved (§5.5).

Acceptance ties to **G2**: a real function with an `ensures` whose correct proof
is accepted (verdict `proved`, certificate kernel-`check`ed) and whose wrong
proof is rejected (verdict `not proved` — `disproved` or `unknown`, the
verdict-flip); a refinement introduction emits its obligation; an `incomplete`
claim is distinguishable from `proved` by a checked certificate **and**
`postulates_reachable` with no open obligation hole, including a hole reached
through another refined constant's `Proj2` (§5.4); `old` resolves in a
`space`-op `ensures` and is rejected out of scope (§6.4); and V0's behavior
is unchanged for non-spec programs (§6.2). Conformance:
`../../conformance/verify/spec-syntax/`. Subset-Σ core-shape expectations
remain W5-deferred while the carrier-only implementation is current;
transitive-honesty query expectations are W4-deferred (§intro).
