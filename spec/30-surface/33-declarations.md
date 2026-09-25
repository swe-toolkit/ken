# Declarations, modules, and constraints

> Status: **normative for the features**; the concrete syntax spelling is
> `OQ-syntax` (proposal-level). §3 modules / imports / name-resolution and §4
> visibility (**private-by-default, settled**) + abstract-export are
> **normative** (ES3); §5's class/constraint core is landed, and §5.5.1 is the
> normative N4 source-world admission contract —
> **typeclasses-as-subobjects-of-the-universe** (Lc). The module system
> **elaborates away** to the kernel's flat append-only `Σ`: **zero
> `trusted_base()` delta** (`30-taxonomy.md §1.1`, the ES1 minimality
> invariant).

## 1. Definitions

A definition's keyword **declares its static purity** — a checked signal, not a
convention (`36 §1.6`; operator ruling, SURF-1). Three keywords, split at purity
and arity:

- **`const c … = e`** — a **pure value** (zero explicit value parameters);
  elaborates to a global definition. Subsumes the former value definition (the
  old nullary `let`). A `const` may still take **implicit** type/level/instance
  parameters — it is a constant *family* (`36 §1.6.3`), e.g. `const nil {A :
  Type} : List A = Nil A`.
- **`fn f … = e`** — a **pure function** (≥1 explicit value parameter);
  elaborates to a global Π/λ definition, gated by SCT (`../10-kernel/17 §4`).
  The verification layer may treat an `fn` as a mathematical function (`36 §1`).
- **`proc p … visits ρ = e`** — a **potentially impure/imperative** definition
  at any arity: it carries an effect row `ρ` (concrete, or a row variable, `36
  §1.5`), or is a `space` operation (`36 §4`). A `proc` is the *only* keyword a
  `visits` row (non-empty, or a variable) may sit on.
- **`def T … = …`** — a **definition**: a base type narrowed by conditions (a
  refinement/`Σ`/Π type abbreviation); a plain alias is the zero-condition case
  (transparent; unfolds by δ). Was spelled `type` before `SURF-def-refinement`;
  `type` is now reserved, not a declaration keyword.
- All top-level definitions are **mutually recursive within a module** if the
  SCT check accepts the group; otherwise the offending recursion is reported
  (`17`). This grouping **includes `theorem` and attached `proof`** declarations
  (`§8`), not only `const`/`fn`: proof declarations enter the **same**
  signatures-first, dependency-ordered SCC + SCT admission run, so a recursive
  or mutually-recursive proof is admitted **iff SCT accepts** it, on identical
  terms to `const`/`fn` (`§8.4`). A recursive cycle that **mixes** a proof
  declaration with a `const`/`fn` is **rejected** (`§8.4`, out of scope this
  iteration).
- Definitions may be **generic** (implicit type/level parameters, `39`): `fn id
  {A : Type} (x : A) : A = x`.

Every top-level definition name in one compilation unit occupies the same flat
namespace, independent of the role that introduced it. A second top-level
definition of a name already defined in that unit is a **hard surface error**
(ADR 0014, MRES-5/MRES-8). Thus `class Widget` and a data constructor named
`Widget` collide by this rule: types are terms, so class and constructor names
do not inhabit separate namespaces (D8-③; MRES-7). A name in B instead
rejects on first introduction by §3.3, without waiting for a second
declaration.

The arity-gated `Eq`/`J` sugar is kernel/built-in vocabulary (§3.3): no
source declaration or binder may reuse either protected name, at any arity.
This supersedes MRES-8's former lower-arity coexistence exception; the
ordinary duplicate-definition rule still governs names outside B.

The keyword is **checked bidirectionally** against the signature and the body's
inferred effects (`36 §1.6.2`); a mismatch — an `fn` that performs an effect, a
`proc` that is provably pure, a `const`/`fn` at the wrong arity — is a **hard
error** (`36 §1.6.3`). The single definition keyword `view` is **retired**
(`36 §1.6.4`); the local `let … in …` **expression** (`32 §3`) is unchanged.

## 2. Records (products)

```
record Point { x : Int, y : Int }
record User  { name : String, age : { n : Int | n ≥ 0 } }   -- refined field
```

- Elaborates to right-nested Σ with definitional η (`../10-kernel/13 §3`), so
  field access `p.x`, record literals `{ x = 1, y = 2 }`, punning `{ x, y }`,
  and **functional update** `{ p | y = 3 }` all have their expected definitional
  behaviour.
- Fields may be **dependent** (a later field's type mentions an earlier field)
  and **refined** (carry a proposition) — records are Σ, so this is free.

## 3. Modules and imports

A **module** is a pure surface namespacing + information-hiding device. It
**elaborates away**: after name resolution the kernel sees the single flat,
append-only global environment `Σ` (`../10-kernel/11 §4`) it would see for the
same program written in one fully-qualified namespace. Modules, imports, and
visibility are **surface + elaboration-time only** — they add **no** kernel
feature and **nothing** to `trusted_base()`. The ES1 minimality invariant
(`30-taxonomy.md §1.1`, surface built-in set ≡ `trusted_base()` delta) carries
verbatim: a module program's trusted-base delta is **identical** to its
flattened equivalent's.

### 3.1 Declaring modules

**`module M { … }`** groups declarations under the namespace `M`. A file is an
implicit module named by its path. Modules **nest**
(`module M { module N { … } }` gives `M.N`). A module is an **environment
fragment**: its declarations elaborate into `Σ` in dependency order under their
qualified names, exactly as if written flat.

### 3.2 Importing and exporting

Within a module, an `import` brings another module's **exported** names (`§4`)
into scope. Three forms:

- **`import M`** — qualified: `M`'s exports are accessible as `M.foo`, `M.Bar`.
- **`import M as N`** — aliased: the same, under `N.foo` (`M` itself unbound).
- **`import M (foo, Bar)`** — selective: exactly `foo`, `Bar`, brought
  **unqualified**; nothing else of `M`.

A selective item is either a name or a per-name rename. Thus `import M (foo,
Bar as Baz)` brings `foo` unqualified and brings `M.Bar` unqualified under the
name `Baz`; it does not also bind `Bar`. The per-name `as Baz` is inside the
selection list and is distinct from the module alias in `import M as N`.

An `export` declaration adds names to the current module's public interface.
It has two forms:

- **`export M (foo, Bar)`** is a facade republish. It takes the selected names
  directly from `M`'s exports, requires no prior `import M`, and does not bind
  `foo` or `Bar` in the current module's body scope. The declaration is itself
  a loader dependency edge to `M`, using the role-blind dotted-path identity
  below.
- **`export foo, Bar`** republishes names already resolved in the current
  module's scope, whether imported or defined locally. A listed name that does
  not resolve in that scope is an ordinary unresolved-name surface error.

Both forms permit per-name renaming: `export M (foo as bar)` and `export foo as
bar` publish the source declaration under the surface name `bar`. Renaming
does not change its canonical identity. To use and republish a name, import it
and then use the in-scope form; a facade export alone deliberately does not
make the name available to the module body.

For in-repo compilation units, dotted module paths and source-file paths obey a
total, role-blind bijection under a catalog root. A path with `N` components
names the unique leaf source file reached through `N - 1` directories: for
example, `import Data.Collections.Map` resolves exactly one of
`Data/Collections/Map.ken` and its `.ken.md` form. A path component is treated
only as a module component; resolution does not depend on whether declarations
in the file are values, types, constructors, classes, or any other role. A path
position is a leaf or a directory, never both. This is the catalog taxonomy
WP's pinned path/import identity and strict leaf-file rule (ADR 0014,
MRES-2/MRES-3(a strict)).

Resolution takes a **list of catalog roots**. This round populates that list
with exactly one in-repo root; resolution and precedence among multiple roots
remain deferred to the package-manager round (MRES-1(a)/MRES-2). The plural form
is nevertheless the normative resolver input, so adding roots changes the
input data rather than the module-path contract.

The loader discovers compilation units lazily, following `import` and facade
`export M (…)` edges from units already being compiled; it does not scan a
catalog tree eagerly. Each unit is loaded and elaborated at most once in a
compilation run, and later dependency edges reuse the per-run cached result. A
dependency edge to a unit already on the active chain is the hard surface error
**`ImportCycle`**. The diagnostic names the closed cycle in edge order rooted
at the entry unit. The conformance harness elaborates `A` as its entry unit, so
its cycle payload is `A → B → A`. Cycles are not accepted as recursive module
groups (MRES-2).

The in-repo loader discovers the source graph; the `admits` / `program` /
`package` boundary below adds the instance-admission rule over that graph. The
compiled-package manifest and content-addressed package manager remain a later
round. Multi-root precedence is likewise deferred. The built-in name
protection and ordinary non-built-in clash rules are specified in §3.3;
neither the loader nor the admission boundary bypasses them.

#### 3.2.1 Admission-boundary headers

An anonymous **`program`** header marks its file as the admission root for a
multi-package build. An anonymous **`package`** header marks its file as a
package admission boundary. Either header may carry an `admits` section listing
dotted package paths using the same role-blind path identity as `import`. A
`program` may independently carry a `capabilities` section listing an effect
family and its authority:

```ken
program
admits Core.Classes.LawfulClasses, Data.Collections.Map
capabilities FS AFull
```

`capabilities` is chosen because it names the manifest directly. `grants` would
suggest that a program grants authority to itself, `requires` is already the
logical-precondition keyword, and `caps` would abbreviate a security-relevant
boundary. The exact production is in `32 §1`; the v1 authority family is
`Auth = ANone | APartial | AFull`. At most one authority may be declared for a
given effect family.

The two sections are orthogonal manifests with one home:

- `admits` names packages whose instance dictionaries may resolve ambiently.
  It is a coherence and dispatch channel, carries no authority, and is read by
  the elaborator's admission gate (§5.5.1).
- `capabilities` names effect families and authority levels. It is a security
  channel, affects no instance search, and is read by the runner when it mints
  `ProgramCaps`.

The clauses have separate namespaces and separate checks. Either may be absent
while the other is present. An `admits` change cannot authorize an effect, and a
`capabilities` change cannot make an instance eligible for dispatch.

For `capabilities FS a`, the runner mints exactly
`ProgramCaps a = MkProgramCaps (Cap a)` from the declaration. There is no
external launch grant to compare. The entry point has the authority-monomorphic
shape

```ken
proc main (input : ProcessInput) (caps : ProgramCaps a) : HostIO ExitCode
```

where `a` is the header-declared authority. A program whose body performs an FS
effect without declaring FS has no corresponding capability value in scope.
The capability-passing translation (`36 §2.5`) therefore produces an unbound
capability reference, which is ill-typed; `62 §1` specifies the earlier
missing-capability surface diagnostic. Opaque `Cap`, driver-only minting, and
downward-only attenuation bound reachable authority by the declaration. This
is ordinary Π/λ checking, not a new kernel rule or trusted primitive.

Neither header takes a name token. The file path is the single identity of the
program or package boundary; a spelling such as `program App` or `package Lib`
is a syntax error. The header's presence is the signal, and an ordinary comment
may carry documentary intent without creating a second identity.

`program` and `package` establish elaboration-time instance-admission
boundaries. A `program` additionally declares the authority manifest read by
the runner. Neither header designates nor declares a runtime entry point. An
entry declaration is a separate construct that a program file may co-host;
this section defines no entry syntax (MRES-4a/4e).

### 3.3 Name resolution (surface-only; never reaches the kernel)

Resolution is a **surface / elaboration** pass; a name still unresolved after it
is a **surface error** (`24`) — it never reaches the kernel:

- **Qualified paths** may name a module export (`M.C`) or a data
  constructor (`T.C`, `34 §1`) in an expression or pattern. `T` must resolve
  to a visible data type and `C` to one of its exact registered constructors;
  qualification preserves that constructor's canonical `GlobalId` and checks
  its ordinary visibility. If the same written path could denote both a module
  export and a type constructor, reject **`AmbiguousReference`**; neither
  lookup order, capitalization, nor an identical target ID breaks the tie.
  Qualified and aliased module imports otherwise preserve the imported
  declaration's identity. Selectively imported or renamed names do too.
- **One strict mode, for every source unit.** Resolve a source name through its
  lexical locals (including declarations already admitted to the same
  incremental session), explicit imports, the kernel/built-in vocabulary, or
  the closed prelude of `30-taxonomy §4`. The same rule applies to roots-loaded
  and isolated-file compilation and to incremental elaboration. No other
  lookup in an implementation-global table can make a spelling visible. A
  loaded dependency is not implicitly imported and neither unit inherits the
  other's imports. A registered convenience outside the prelude needs an
  explicit public provider import; without one it is unbound.
- **Closed built-in name set B.** For resolution and name protection, B is the
  kernel/built-in vocabulary (`30-taxonomy §3`: audited native names, `Omega`,
  `Eq`/`J`, and reserved `Refl`/`Axiom`/`absurd`/`trunc_intro`) plus the exact
  checked prelude members of `30-taxonomy §4`: `{Auth, Bool, Bottom, Char,
  Equal, List, Nat, Option, Pair, Prop, Proved, ResourceKind, Result, Top,
  Utf8Error}`. The latter is a fifteen-member type floor, not the superseded
  ten-member inventory. For an inductive floor type, its per-type
  **scoped constructors** property determines whether its whole exact
  constructor family also occupies bare B names. When enabled, only `T`
  enters B and public constructors resolve as `T.C`; no bare `C` is reserved
  by that family. When disabled, its entire constructor family stays bare and
  reserved. There is no per-constructor subset choice. `ResourceKind` is scoped:
  `ResourceKind.FsHandle`, `ResourceKind.Buffer`, and
  `ResourceKind.Mapping` replace its former three bare reserved names.
  `Auth`'s `ANone`/`APartial`/`AFull`, `Bool`'s `True`/`False`,
  `List`'s `Nil`/`Cons`, `Nat`'s `Zero`/`Suc`, `Option`'s `None`/`Some`,
  `Result`'s `Err`/`Ok`, and `Utf8Error`'s `InvalidUtf8`
  remain bare. `Char`, transparent `Pair`, and the proposition/equality
  members have no constructors. Constructor-private names remain inaccessible
  under §4.2 in either mode. `Pair`'s three checked companions
  `{mk_pair, pair_fst, pair_snd}` still enter B at its exact identity. Kernel
  native names and reserved formers/sugar remain kernel vocabulary, not extra
  prelude identities. Neither the closed roster nor B includes arbitrary
  compiler globals; future floor membership requires §4's two witnesses and
  an atomic roster and per-type property change. The dedicated L2 qualified-
  constructor/scoped-property slice precedes L2-3's floor additions and
  reconciles existing `ResourceKind` users; until then the implementation's
  older bare behavior is a staged gap, not a second normative mode.
- **No binding of a built-in name.** No declaration or binder may introduce a
  name in the built-in set B. This is a hard surface error at introduction,
  even if the proposed target has the same canonical identity as the built-in
  and even if no later occurrence uses the binding. It covers top-level
  `const`/`fn`/`proc`/`def`/`prop`/`theorem` and other definitions; `data`
  types and constructors; records and fields; classes and methods; attached
  proof identities and synthesized instance dictionaries; selective, renamed,
  or re-export import aliases (including a module alias named in B); and
  `λ`, `let`, function/constructor/record parameters, Π, match-pattern,
  `eqn`, and `where` dictionary binders. A scoped constructor's bare spelling
  is not in B merely because its type is reserved; it can be introduced by
  other source declarations subject to ordinary non-B clashes. Exporting or
  importing a floor identity by a *different* name outside B preserves its
  canonical identity;
  it does not create a second floor identity. Qualified use of an existing
  floor name is not a new binding. Lexical shadowing applies only to names
  outside B; an inner binder cannot override B.
- **`Eq`/`J` staging.** The former MRES-8 lower-arity coexistence exception
  is superseded for both protected kernel names. The existing conformance
  case `arity-gated-eq-j-sugar-coexists-with-lower-arity-definitions` and
  `n1_duplicate_definition.rs` still test the pre-enforcement behavior;
  their inversion is owed by L2-4, **not** by this specification change.
  L3 first renames the catalog's `class Eq` and its users, with Spec and
  Foundation choosing a non-colliding name. L2-4 may enforce the ban only
  after that rename lands; S0 chooses no replacement name.
- **Non-B top-level identity clash.** A top-level local, a selective or
  renamed import, and a re-exported name may supply the same unqualified
  name outside B. If two bindings name **distinct canonical declarations**,
  reject the latent clash as **`AmbiguousReference`** even when unused,
  independent of declaration order. Multiple paths to the same canonical
  declaration remain idempotent. Omit a colliding selective item, rename it
  outside B, qualify it, or rename the local. The B-introduction rule above
  takes precedence over this ordinary ambiguity rule; neither `rename` nor a
  same-identity path authorizes a source binding in B. A same-shaped local
  `Pair` declaration has a different `GlobalId` and never substitutes for the
  protected floor identity (`34 §"Canonical non-dependent pair floor family"`).
- **Effect-row and capability vocabulary is a separate namespace and is not
  subject to this rule.** An `EffectName` in a `visits [...]` row or a
  `capabilities` clause (`32 §1` capability-declaration syntax; `36 §1` the
  effect row, `§1.1` its lattice) is resolved by the effect-row system, never by
  import, and
  is not a name-resolved value or type identity — it is outside the
  floor-versus-import binary entirely. A program writes `visits [FS]` /
  `capabilities FS` with no import, and that ambient tag must not be conflated
  with the import-required catalog package that provides the corresponding
  filesystem surface (its authority, error, and path *types*), which are
  ordinary identities under the closure rule above.
- Every failure — unresolved name, forbidden B introduction,
  **`AmbiguousReference`** from a non-B top-level clash, or an out-of-scope
  private name (`§4`) — is a **surface diagnostic**; the flattened `Σ` the
  kernel receives contains only resolved, in-scope references.

## 4. Visibility and abstract export

### 4.1 Visibility — private by default, `pub` to export

Top-level names in a module are **module-private by default**; **`pub`** exports
names at their definition. A module's **interface** is the union of its own
`pub` definitions and the names introduced by `export` declarations. A private
name absent from that union is invisible outside its module, and accessing it
from outside is a **surface error** (name not in scope), *not* a kernel error.

**The default is private — settled (was `OQ-syntax`).** Rationale:
private-by-default is the **least-surface, information-hiding-forward** choice —
a module exposes only its intended interface, so the coupling surface a client
depends on is exactly the `pub` set: small and auditable (the module-level echo
of small-auditable-TCB). Accidental exposure requires an explicit `pub`, never
an explicit hide; and it **matches abstract export** (`§4.2`), the same
information-hiding default one tier down (hide the constructors). The inverse
(public-by-default) is **not** taken. `../90-open-decisions.md` records the
visibility default as resolved — no longer part of the `OQ-syntax` iterating
set.

### 4.2 Abstract export — the opaque constant, not a new mechanism

A type may be exported **abstractly**: its name is `pub`, its **constructors are
not**. Clients see the type but cannot `match` on or construct its hidden
constructors. This is **exactly** the kernel's existing **opaque constant**
(`../10-kernel/11 §4` — an opaque `c : A` is "how … abstract interfaces are
represented") — information hiding with **no new kernel feature**:

- It is enforced at **elaboration** (surface): the hidden constructors are
  simply **not in scope** at the client, so a client `match` on one is a
  **surface error** (name not in scope), never a kernel rejection.
- The abstract type's **kernel representation is byte-identical** to a
  hand-written opaque constant — there is **no** kernel "abstract" flag and no
  visibility concept in `Σ`. A design that added a kernel-level
  abstract/module/visibility primitive (a new `trusted_base()` entry) is
  **rejected** by this spec; the elaborates-away form is the whole mechanism.

This is the AC1/AC2 invariant made concrete: `module` / `import` / `pub` /
`export` / abstract-export cost the trust root **nothing** — surface namespacing +
information-hiding over the unchanged flat `Σ`.

Two consequences follow directly from the framing above and from `§4.1`; they
are stated here because the account so far addressed only the non-parameterized
case.

- **A parameterized abstract export preserves its telescope exactly.** When the
  exported type is itself parameterized (e.g. `NonEmpty`), the opaque constant it
  elaborates to carries the type former's **full declared kind** — byte-identity
  to a hand-written opaque constant is byte-identity *at that same kind*
  (`(A : Type) → …`), so the parameter is never collapsed or erased and
  `NonEmpty a` still elaborates at a client. An opaque constant of any kind,
  Π-kinded included, is the existing kernel mechanism (`../10-kernel/11 §4`,
  `13`); this remains "no new kernel feature."
- **The defining module retains transparent constructor access.** Constructor
  hiding is a **client-side** effect: the constructors are ordinary
  module-private globals (`§4.1`), in scope *within* their defining module and
  invisible only outside it — name resolution is surface-only and never reaches
  `Σ` (`§3.3`). The defining module therefore keeps transparent `match` and
  construct on the type while clients see only the opaque view; this is the same
  information-hiding default as §4.1, applied to constructors rather than to a
  top-level name.

### 4.3 Re-export preserves identity and visibility

Every declaration has one canonical identity, owned by its **defined-at**
module. An `export` declaration records a **re-exported-at** public path to that
identity; it republishes the existing `GlobalId` and never mints another one.
This remains true through renaming and any number of re-export hops. The export
statement keeps the provenance grep-recoverable at the republishing module.

A compiler-installed floor identity has an internal origin rather than a source
**defined-at** module. Its ambient availability and a public alias **named
outside B** preserve that identity; neither operation manufactures a source
owner. This holds for signature-arm members and for kernel- or compiler-origin
members of the internal-provision arm. A source `export`/`import` may not
introduce a name in B, even to republish the identical `Nat` or
`Pair`/`mk_pair`/`pair_fst`/`pair_snd` identity under its protected spelling
(§3.3). A same-shaped source declaration has a different identity, never
another path to the floor identity.

Two distinct identities may not occupy one surface name in a module interface:
that is a hard surface error at the re-export site, reported with both the
defined-at declaration and the conflicting re-exported-at path. Republishing
an identity already present under the same surface name is idempotent.

Visibility travels with canonical identity, not with a public path. Re-exporting
an abstract type therefore republishes the same opaque constant while its
constructors remain hidden; `export` cannot widen constructor visibility. For
a locally defined name, `export foo` has the same interface effect as declaring
`pub foo`, without creating a second identity. `pub` on the local definition
remains the idiomatic spelling.

## 5. Constraints — typeclasses as subobjects of the universe

> **Status of this section: impl-ready (Lc).** The surface grammar is `32 §1`
> (`class`/`instance`/`where`/`derive`); the **elaboration shapes** and the
> **coherence policy** are pinned here to algorithm level; the **resolution
> algorithm** (constraint insertion, instance search, its termination metric,
> and the diagnostics) is `39 §6`. **`OQ-classes` DECIDED, ADR 0008.** **No new
> kernel feature** — a class is a **record** (`../10-kernel/13 §3` Σ + η), a law
> is an **Ω** proposition (`../10-kernel/16 §1`), search termination is the
> **landed SCT** (`../10-kernel/17 §4`) via the reified-dictionary definition
> (`39 §6.4`). If the build adds a kernel rule/judgment/"class" former, it has
> **mis-scoped**.

Ken's constraint/trait mechanism is **structure on types**. A "class" is a
structure `C : Type → Type` (its members carve out a subobject of the universe,
`../10-kernel/12 §5`); an "instance" exhibits that a given type carries it. This
"typeclasses-as-subobjects" design is the most category-faithful account of open
user typeclasses.

> **That shape is the common case, not the formation rule.** The rule is `§5.2`
> — a class elaborates to the right-nested Σ over its field telescope — and it
> fixes no level; `§5.1` then keys the property/structure discriminant on the
> record's kernel-computed **sort family**, not on a level. A class may
> therefore depart from `C : Type → Type` on either axis and remain an ordinary
> class. Both departures exist: `class Traversable (f : Type → Type)` (`§5.2`)
> takes a higher-kinded **parameter**, and a class carrying a `Type`-valued
> field lands one universe up, at `C : Type ℓ → Type (suc ℓ)`, because the
> field's type is itself at `Type (suc ℓ)` (`12 §1`) and the Σ-sort takes the
> maximum (`13 §4`). Read the shape above as orienting; read `§5.1`/`§5.2` as
> binding.

```
class DecEq (A : Type) {              -- a record of operations + their laws
  eq    : A → A → Bool                 -- (the propositional equality is the
  ok    : (x y : A) → eq x y == true → Eq A x y   --  kernel's Eq, 10-kernel/15)
}
instance DecEq Int { eq = int_eq, ok = … }

fn nub {A : Type} (xs : List A) : List A  where DecEq A = …   -- a constraint
```

### 5.1 The two kinds — and the sort *is* the discriminant

Two kinds of class, split by **where the dictionary lives** — the distinction
that governs coherence (`OQ-classes`, ADR 0008):

- **Property classes** (Ω-valued: `Decidable p`, `IsHom f`). Proof-irrelevant
  (`../10-kernel/16 §1`), so **any two instances are definitionally equal** —
  the subobject framing is literal, and **coherence is free** (the kernel
  guarantees it; *no resolver convention applies*).
- **Structure classes** (`Type`-valued, dictionary with computational content:
  `DecEq`, `Monoid`, `Ord`). Genuinely *many* can exist on one carrier (ℤ under
  `+` and under `×`), so the subobject reading is "∃ a dictionary" and coherence
  is a **resolver convention** (§5.5), not a theorem.

**The discriminant is the class record's kernel-computed *sort*, not an author
flag (`AC4`).** A class elaborates to a right-nested Σ (§5.2); the kernel's
`sort_sigma` is **both-components-keyed** — `sort_sigma(s₁, s₂) = Ω` **iff**
`s₁ = Ω ∧ s₂ = Ω` (`../10-kernel/13 §4`). So the whole record lands in **Ω iff
*every* field is Ω-valued** (a property class), and in **`Type`** the moment
**any** field is relevant/`Type`-valued — an operation like `eq : A → A → Bool`
(a structure class). The elaborator reads the sort off the emitted record; it
does not carry a separate kind tag.

**Soundness note — never force a structure class into Ω (`AC4`, Architect
gate).** A class with a relevant operation field is `Type`-sorted; classifying
it Ω to "get coherence for free" would fire Ω-PI (`16 §1`) on the whole
record and make its *computational* content proof-irrelevant — two
observationally-distinct dictionaries would be definitionally equal, collapsing
the very content the prover's lemmas depend on. This is the Σ-sort trap
(`13 §4`: sending a relevant-carrier Σ to Ω is unsound): the
both-components-keyed `sort_sigma` is what *prevents* it, so the discriminant
must be the real kernel sort, computed over **all** fields — a mis-keyed
(codomain-only) sort is the soundness bug the Architect gates.

### 5.2 Class declaration → a record type

`class C (A : Type) { op₁ : T₁ ; … ; law₁ : P₁ ; … }` elaborates to a **record
type** — the **right-nested Σ** over the field telescope (`../10-kernel/13 §3`),
parameterised by the class head `A`. A class field may also carry an optional
leading purity keyword:

```
class Traversable (f : Type → Type) {
  functor  : Functor f
  foldable : Foldable f
  proc traverse :
    (g : Type → Type) → Applicative g → (a b : Type) →
    (a → g b) → f a → g (f b)
}
```

The field forms are:

```
class_field ::= field_name : type
              | (const | fn | proc) field_name : type
```

An unmarked field is **unclassified** and keeps the pre-SURF-2 behaviour: the
instance field is checked only against its declared type, and projection carries
no extra static-purity signal. A marked field's keyword is a **checked
signature** over the field declaration itself, reusing the definition-level
SURF-1 purity discipline (`§1`, `36 §1.6`): `const`/`fn` fields must be pure
with the corresponding explicit-value-arity classification, and `proc` fields
must be potentially effectful/effect-row-polymorphic under that same
classifier. Because class fields have no separate value-binder list, the
arity/effect telescope is read from the field's **declared type**; implicit
parameters still do not count (`36 §1.6.3(b)`). A later instance expression must
satisfy the stored field classification — by **covariant subsumption**, not
exact equality: a pure witness inhabits a `proc` field, but the reverse (an
effectful witness for a `fn`/`const` field) rejects (`36 §1.6.2`, DS-8b) — but
it cannot redefine what the class field marker meant. The marker is
surface/elaboration metadata attached to the field declaration, not a field of
the class record.

**AC4 non-interference.** A class-field purity marker does **not** enter the
Type/Ω discriminant. The class's sort is still the kernel-computed sort of the
right-nested Σ over the field **types** alone (§5.1): an operation type such as
`traverse : …` is relevant because of its type, not because the author wrote
`proc`; an Ω law field remains Ω because its type is Ω, not because it is
unmarked. The elaborator may store `Some(proc)`/`Some(fn)`/`Some(const)` beside
the field for checking and projection, but it must not feed that marker into
`sort_sigma`, class-kind selection, or instance-coherence policy.

The record type itself is unchanged:

```
C A  ≡  (op₁ : T₁) × … × (opₙ : Tₙ) × (law₁ : P₁) × … × (lawₘ : Pₘ)
```

- **Operation fields** are `Type`-valued (`eq : A → A → Bool`); **law fields**
  are **Ω-valued propositions** (`../20-verification/21 §3` `law`/`verify`;
  e.g. `assoc : (x y z : A) → op x (op y z) == op (op x y) z`). A mixed record —
  relevant ops beside Ω proofs — is well-formed and lands in `Type`
  (`13 §4`); a record whose fields are *all* Ω is itself a proposition (the
  sound Σ-of-Ω-into-Ω case, `16 §1.3`) — that is exactly a **property class**.
- **Definitional η (`13 §2`)** gives the record its expected behaviour: a
  dictionary `d : C A` satisfies `d ≡ (d.op₁, …, d.lawₘ)`, so field projection
  and reconstruction round-trip definitionally — the prover cites `d.assoc`
  directly (`§5.3`, `AC8`).
- The record's **sort** (Ω vs `Type`, §5.1) classifies the class; the kernel
  computes it — no new former, just a Σ over the existing telescope machinery.

### 5.3 Instance declaration → a record value (+ the orphan check)

`instance C T { op₁ = e₁ ; … ; law₁ = p₁ ; … }` elaborates to a **record value**
of type `C T` — a right-nested pair (`13 §2` Σ-Intro) of the operation
implementations **and the law proofs**:

```
inst_C_T : C T  ≡  (e₁ , … , eₙ , p₁ , … , pₘ)
```

Each `pⱼ` is a **real kernel proof** of `Pⱼ[T/A]`, checked at its Σ-Intro
position `B[a/x]` (`13 §2`) — not a stub. A `Monoid Int` instance therefore
carries genuine `assoc`/`unit` proofs the prover can **cite** (`AC8`): the
lawful-by-construction dictionary is the verification win. The value is admitted
through the real `declare_def` path (`../10-kernel/…`, `check.rs`), so the
kernel re-checks the ops *and* the proofs.

**The orphan check — at declaration, per-module (`AC2`).** An `instance C T`
declaration is **accepted only if its module defines class `C` or defines the
head-type `T`'s constructor**; an instance whose module owns **neither** (an
*orphan*) is a **hard error at the declaration site** (`39 §6.1`). Mentioning,
importing, or re-exporting a class or head is not definition and does not
transfer ownership. This is a purely **syntactic, per-module** predicate on the
declaration — decidable without whole-program information — and it is what keeps
canonicity (§5.5) *un-break-able by accident*: every ordinary instance is
co-located with either its class or its head-type, so the canonical instance for
a `(class, head-type)` pair is discoverable from those two modules alone. The
check is an **elaborator** check (it constrains *where* a well-typed value may
be declared), not a kernel rule.

A compiler-installed floor head has no source module that can satisfy the
head-owner arm. Its canonical structure instance must therefore be declared in
the class's defining module; neither floor arm creates an orphan exception. The
canonical `instance Ord Nat` is consequently declared in
`Core.Classes.LawfulClasses`, which defines `Ord`, and is keyed by the exact
floor `Nat` `GlobalId`. `Data.Numeric.Nat.Order` may expose that existing
instance through the re-export carry rule (§5.5.1), but it neither owns nor
redeclares it. An `instance Ord Nat` declaration in `Order` remains an orphan.
For the compiler-origin floor `Pair`, canonical parameterised `Ord (Pair a b)`
and `DecEq (Pair a b)` instances likewise use the class-owner arm and are
lawful only in `Core.Classes.LawfulClasses`; no package or facade becomes a
head owner. A separately declared same-shaped family may support its own
head-owned instance under its distinct key, but that dictionary is not an
instance for the floor identity.

### 5.4 Constraint `where C A` → an implicit instance argument

A constraint `where C A` on a definition (`fn`/`proc`/`const`, grammar `32 §1`)
elaborates to an **implicit instance argument** — an implicit `Π` over the class
record inserted ahead of the explicit parameters:

```
fn nub {A : Type} (xs : List A) : List A  where DecEq A = …
      ⟶  nub : {A : Type} → {d : DecEq A} → List A → List A
```

The `{d : C A}` binder is threaded like any implicit (`39 §2.2`): at a **use
site** the elaborator inserts a metavariable for `d` and **discharges it by
instance search** (`39 §6`). Inside the body, a class operation `eq x y` is
`d.eq x y` (projection from the resolved dictionary). Multiple constraints
`where C A, D B` (comma-separated, `32 §1`) insert one implicit each, left to
right. The concept is ordinary dependent-implicit insertion — the *resolution*
of `d` is the only new step, and it lives in `39 §6`.

**Multiple constraints — one dictionary per constraint, deterministically
named (shared by the definition path and instances).** This multi-constraint
contract — the deterministic `d<v>` naming, explicit named binders, and
comma-separated list below — is the **shipped surface of every `where`-clause**:
both the **definition path** (`const`/`fn`/`proc`, and the legacy `view`) and the
**instance** `where`-clause (`instance C Head where …`) parse through the **one
shared production** (`32 §1`'s `constraint_clause`; the landed def-path
constraint-binder unification). The definition path additionally retains `;` as a
separator for existing declarations (`37 §6`, L3b) and instances additionally
tolerate a trailing `,` before `{` — spelling compatibilities, not distinct
grammars. The multi-constraint, `d<v>`, and explicit-binder cases below therefore
hold **uniformly on definitions and instances alike**.

On an instance, with more than one constraint each dictionary needs a distinct,
deterministic name, and the singular `d` generalizes. A constraint **of the form
`C v`** — a class applied to a **single type variable** `v` (`32 §1`:
`constraint ::= ConId atype+`) — binds its dictionary as **`d<v>`**, the reserved
prefix `d` immediately followed by that variable's identifier, projected by
explicit `.field`:

The following example uses the canonical compiler-origin floor `Pair` and its
companion bindings; no provider import or ambient-global fallback participates.

```
instance DecEq (Pair a b) where DecEq a, DecEq b { … }
   ⟶  {a b : Type} → {da : DecEq a} → {db : DecEq b} → DecEq (Pair a b)
      -- fields project da.eq / db.eq, da.sound / db.complete
```

- **Uniform, deterministic, projection-preserving.** The name is a pure
  function of the constraint's type argument — `DecEq a → da`, `DecEq b → db`,
  `Ord a → da` — identical for one or many constraints, and identical on the
  definition path and the instance `where`-clause (the shared production above).
  Explicit `.field` projection is unchanged (`da.eq`, `db.complete`);
  only the *name* generalizes. This is the coherent completion of the singular
  model — a named dictionary per constraint, **not** one reserved `d` for many,
  and **not** type-directed member resolution (a distinct, implicit paradigm;
  reflect-don't-extend).
- **Source-order binding (the contract).** Constraints bind **left to right**:
  the i-th `where` constraint is the i-th implicit `Π` position, and `d<v>`
  denotes exactly that one dictionary at that one position. The field body's
  `d<v>` and the elaborated type's `Π`-position must agree by this order; a
  mismatch elaborates a `Σ`-dictionary that fails its declared type and the
  **kernel rejects it** — the elaborator is untrusted, so a naming/position bug
  is **fail-closed** (rejects a good dictionary, never admits a bad one).
- **Singular reconciliation — `d<v>` canonical, bare `d` the retained
  sole-constraint spelling.** The uniform rule makes the single-constraint case
  `d<v>` on **both** paths (`where DecEq a` → `da`, `da.eq`). The reserved bare
  `d` is **retained for the sole-constraint case on the definition path and on
  instances alike** (`37 §6`/`51 §4`, L3b) — so landed catalog proofs that
  project it (`Core/Logic/EmptyDec`'s `d.eq`/`d.sound`/`d.complete`,
  `Core/Classes/LawfulClasses`'s `d.leq`) stay valid — and it stays available even when
  that sole constraint is written with an explicit named binder (`where (chosen :
  Flag Int)` still admits `d`). With **two or more** constraints, bare `d` is not
  bound — every dictionary is its `d<v>` auto-name or explicit name.
- **Explicit named binders — required wherever the auto-name is unavailable.**
  The `d<v>` auto-name is defined **only** for the single-type-variable form
  `C v`. **Any other grammatical constraint (`32 §1`) takes no auto-name and
  requires an explicit binder** `where (name : C τ)`. Two cases arise:
  - a **compound- or multi-argument** constraint — `where DecEq (List a)`,
    `where C a b` — has no single variable `v` to key on;
  - a **same-variable collision** — `where DecEq a, Ord a`, where two
    constraints would both auto-name `da`.
  In each, the surface **requires** the explicit form — `where (dla : DecEq
  (List a))`; `where (da : DecEq a), (oa : Ord a)` — projected by the same
  explicit `.field` (`dla.sound`, `oa.leq`); a **bare** `where` in these cases is
  a **surface error** (no auto-name exists, or an ambiguous one). The
  named-binder form `where (name : C τ)` is available generally — any dictionary
  may be user-named — and explicit binder names must be **pairwise distinct** (a
  duplicate is a surface error); an unnamed single-type-variable constraint takes
  the deterministic `d<v>`. So the common `d<v>` names stay stable: reaching for
  a compound/multi-arg or a same-variable constraint is a deliberate move to
  explicit names, never a silent rename of an existing `d<v>`.

(This is the `33 §5.4` naming contract the `constrained-instance-elaboration`
capability co-lands against: the Architect ruled the model — deterministic
named per-constraint projection — and deferred this user-visible spelling to
Spec.)

### 5.5 Coherence policy (`OQ-classes`, ADR 0008 — do not reopen)

For **structure** classes, the resolved dictionary is **semantically
load-bearing** (it carries law proofs the prover *uses*), so implicit resolution
is governed by a canonicity convention — the property a client lemma about "the
`Monoid A`" relies on:

- **One canonical instance per `(class, head-type)`** participates in implicit
  search: "the `Ord A`" is a **function of `A`**, stable program-wide (`AC1`).
- **No overlapping instances**; **ambiguity is a compile error naming both**
  candidates (`AC3`), never a silent pick.
- **Orphans are rejected at declaration** (§5.3, `AC2`) — the structural
  precondition that makes canonicity per-module-decidable.

For **property** classes none of this applies: proof irrelevance (`16 §1`) makes
**any** instance do — all are definitionally equal, so resolution may return the
first found and two instances **never** conflict (`AC4`). The policy split is
**exactly** the sort split of §5.1 — the resolver consults the class's sort to
decide whether canonicity is even in play.

**Named instances are first-class values, passed explicitly (`AC5`).** Because
an instance *is* a record value, you may define a **non-canonical**
`byLength : Ord String` and pass it explicitly (`sortBy byLength xs`) — the
dependent-types escape hatch Haskell lacks (no `newtype` gymnastics). Explicit
passing is **ordinary value application** at the dictionary `Π`; it **bypasses
search** and therefore **does not perturb** implicit canonicity: at the same
type, *implicit* `where Ord String` still resolves to the **canonical**
`Ord String`. The resolver may pick only one canonical thing silently; you may
deliberately use any value. That split is the whole point.

#### 5.5.1 Program/package admission and cross-package coherence

Instance resolution is ambient only inside an explicit admission boundary. For
each `program` or `package` boundary, the elaborator computes two distinct
sets:

- The **coherence set** is the unfiltered transitive closure of the boundary's
  complete source graph, seeded by its own units and admitted roots. Every
  structure instance in this closure remains subject to §5.5; property-class
  instances remain proof-irrelevant and do not conflict. The closure is total:
  neither selective import nor any other name-resolution form removes a package
  from it. In one acyclic source graph, the orphan rule constructively ensures
  that at most one package may legally define a given `(class, head-type)` key.
  The existing §5.5 overlap check retains its intra-package duplicate coverage;
  no separate source-world cross-package collision can arise. The source-closure
  coherence pass nevertheless performs one keyed collision test per structure
  instance: O(total instances in the closure), not O(packages²).
- The **direct-use set** contains the boundary's self-admitted package, when
  applicable, plus the packages named by its explicit `admits` section and the
  canonical instances carried by their re-exported public surfaces. An
  instance that one of the boundary's own units dispatches must be granted by
  this set. Transitive membership in the coherence set alone does not grant
  dispatch rights.

**Re-export carries the instance surface (MRES-4d).** Re-exporting a name
carries into an admitting consumer's direct-use set every canonical structure
instance whose `(class, head-type)` key's head-type or class is part of the
re-exported public surface. These instances may be defined anywhere in the
re-exporting package's coherence closure. Property instances are Ω-valued and
proof-irrelevant, so they carry trivially. A transitive instance not carried by
a re-export remains coherence-only; direct dispatch still requires admitting
its defining package and otherwise raises **`UnadmittedInstance`**. The carry is
computed by the elaborator from the re-export set at the admission boundary. It
is a direct-use-set computation, not a kernel rule; every dictionary value is
still kernel-checked, so it adds no TCB.

After implicit search selects an instance, the elaborator checks that the
current boundary's direct-use set grants it, either through its defining
package or as a carried canonical instance. A miss is the hard surface error
**`UnadmittedInstance`** and names both the defining package and the selected
instance. Thus reaching directly for an instance that was present only because
an admitted dependency used it, and was not carried by a re-export, makes its
provider a direct instance dependency that must itself be listed in `admits`.

This admission check is additive to the existing rules. The orphan check still
runs at the instance declaration (§5.3), and the §5.5 overlap check still
rejects a second canonical structure instance within the defining package.
Admission cannot make an orphan or overlapping instance acceptable; conversely,
passing orphan and overlap checks does not admit an instance for direct use.

**Self-admission and when a root is required.** A single package implicitly
self-admits its own instances, so single-package and catalog development need
no `program` file. A `package` boundary also self-admits its own package and
uses its `admits` roots as its direct instance dependencies, so a
library-with-dependencies is buildable and testable in isolation. A build that
combines two or more instance-providing packages across unit boundaries and is
not already rooted at a `package` boundary requires a `program` file to make
the direct-use choice explicit. Transitive dependencies used only inside an
admitted package flow into the coherence set automatically and are not repeated
in the parent's `admits` list.

**Provenance is observable.** A successful implicit resolution reports the
defining package alongside the selected instance. `UnadmittedInstance` reports
the unlisted defining package and instance. Diagnostics must retain this
provenance through source loading; package identity is the dotted path fixed by
§3.2.1, never a header label. Both-package collision provenance belongs to the
compiled-manifest/package-manager boundary specified below, where such a
collision can genuinely arise.

**SPEC-NOW / BUILD-LATER package rules.**

The following rules are normative forward-compatibility requirements, but are
**not part of the current source-world implementation round**:

- A package file explicitly enumerates its member modules. Membership is
  explicit, while package identity and its root remain path-inferred
  (MRES-4e/MRES-2b). The concrete member-list grammar and spelling, and the
  corresponding build, are deferred to the package-manager round. Until that
  round, the admission gate operates over the existing N2 path-based source
  graph; no member-list keyword or production is introduced here.
- In the package-manager round, a compiled package records an instance manifest
  containing the canonical instances its own boundary commits to. Loading that
  compiled package and rebuilding it from source must contribute the same
  instance environment and produce the same admission and coherence outcomes.
  The manifest is generated from the package's own `admits`; delivery from
  source never substitutes the parent's boundary for that declared boundary.
  The one `admits` relation targets the package regardless of whether its
  delivery is source or compiled; authors do not select a delivery form.
- A parent trusts an admitted compiled package's manifest for that package's
  internally checked commitments and re-checks only cross-boundary coherence.
  At this admission boundary, a genuine canonical-instance collision across
  packages is a hard error that names both defining packages and the conflicting
  `(class, head-type)` key.
  The kernel still re-checks every instance dictionary value, so there is no
  new TCB. Signed or attested manifest validation belongs to the package-manager
  and supply-chain round.
- Persisted content-addressed manifests, registries, lockfiles, and
  supply-chain validation are package-manager concerns. Test-scoped admission
  is likewise deferred; the package's ordinary `admits` section is the only
  test boundary specified in this round.

No compiled manifest, registry, lockfile, or test-only admission syntax is
required by the current source-world build. These deferred rules do not widen
the direct-use set until their package-manager mechanisms exist.

### 5.6 `derive` — an untrusted, kernel-re-checked candidate

`derive (DecEq, Show)` on a `data`/`record` (grammar `32 §1`) requests an
**elaborator-generated structural instance**. Generation is **untrusted**: the
elaborator builds a candidate instance *value* (structural recursion over the
type's constructors) and emits it through the **real `declare_def` path**, where
the kernel **re-checks** the ops and law proofs like any other instance (§5.3).
A malformed generated instance is therefore **caught by the kernel**, never
admitted by a trusted insertion (`AC7`, soundness) — `derive` is a *convenience
that cannot widen the trusted base*. Which classes are derivable is a fixed
structural list (`DecEq`, `Show`, …, §`39 §6.6`); a class needing non-structural
content is not `derive`-able and must be written by hand.

### 5.7 What is landed vs. net-new (honest scope)

Classes/instances are **entirely net-new** surface + elaborator machinery; the
Lc build creates the class/instance desugaring, the orphan check, and the
search. What it **reuses unchanged** (the real producers the soundness-critical
ACs bottom out in) is all **landed**: the kernel **Σ/record** primitives
(`Term::Sigma`/`Term::Pair`/`Term::Proj1`/`Term::Proj2`, `13 §2`/`§3`) the
class-record/instance-value
target; **Ω** proof-irrelevance (`16 §1`) that makes property coherence free;
the **`declare_def` re-check path** (`check.rs`) every instance — and every
`derive`d candidate (`AC7`) — traverses; and the **landed SCT** (`17 §4`) that
bounds recursive-instance resolution once it is reified as a dictionary
definition (`39 §6.4`, `AC6`). **No new kernel rule, judgment, or former** —
subsume-don't-proliferate.

The program/package admission gate of §5.5.1 is a net-new source/elaboration
layer over the N2 loader graph. Its current build contract stops at anonymous
headers, the constructive source-coherence invariant, direct-use admission,
self-admission, and resolution/admission provenance. Explicit package
membership, compiled manifests, cross-package collision detection and
both-package provenance, registries, lockfiles, and test-scoped admission remain
the clearly marked SPEC-NOW / BUILD-LATER rules above. The `export` declaration
and public re-export propagation are instead current normative surface and
elaboration rules (§3.2, §4, §5.5.1), with their build in the named Language
follow-on.

## 6. Fixity and operators

`infixl N op` / `infixr N op` / `infix N op` declare operator fixity (`32 §6`).
Operators are ordinary `fn`/`proc`/`const` definitions with symbolic names;
there is nothing special about them semantically.

Declared fixity is a property of the operator's **canonical identity**, not of
any surface path or alias that reaches it. It therefore **travels with import
and re-export**: a client, an aliased import, or a re-exported path sees the
declaring module's fixity, because import and re-export retain that canonical
identity and republish the same `GlobalId` rather than minting another (`§3.3`,
`§4.3`). Fixity is **not re-scopable at a use site** — an importer cannot
re-declare an imported operator's precedence or associativity, since the
declaration binds the identity, not the local binding. When two **distinct**
declarations that share an operator spelling are imported unqualified, that is
an ordinary identity clash and resolves by the existing **`AmbiguousReference`**
rule (`§3.3`), not by any fixity-specific merge; qualified or aliased access, or
a selective import, disambiguates exactly as for any other name. This is
surface-and-elaboration only: fixity guides parsing into the same core term the
kernel re-checks regardless of which path named the operator, so it adds nothing
to `trusted_base()`.

### 6.1 Standard operator bindings

`31 §1c` admits the spellings `≤`/`<=`, `≥`/`>=`, `≠`/`/=`, `∧`/`/\`, `∨`/`\/`
and `∈` as ordinary symbolic names, each paired spelling one token. Admission
fixes **names**, not meanings. This section fixes the standard **meanings** of
five of them here, and `∈`'s in `§6.3`.

Each standard meaning is an ordinary checked top-level function reached by
ordinary import. None is a built-in, a kernel rule, or a primitive, and this
section adds nothing to `trusted_base()`.

    ∧   bool_and     (a : Bool) (b : Bool) : Bool
    ∨   bool_or      (a : Bool) (b : Bool) : Bool
    ≤   ord_leq_at   (a : Type) (d : Ord a) (x : a) (y : a) : Bool
    ≥   ord_geq_at   (a : Type) (d : Ord a) (x : a) (y : a) : Bool
    ≠   the negation of the comparator the `==` path selects (§6.2)

`bool_and`, `bool_or` and `ord_leq_at` **already exist** as ordinary public
functions in the standard package. The standard-operator home **re-exports**
them; it does not define second copies. By `§4.3` a re-export republishes the
existing `GlobalId` and never mints another, whereas *"a same-shaped source
declaration has a different identity"* — so a redefinition would give one
meaning two identities, and a policy keyed on identity (`39 §6.9`) would then
hold between an occurrence and only one of them. The standard meaning of `∧`
**is** `bool_and`'s canonical identity, reached through an additional public
path.

`≥` is **derived over `leq`, not a class field.** `class Ord a` (`§5.2`)
declares `leq`, `refl`, `antisym`, `trans` and `total`; there is no `geq`
member, and none is added here. `ord_geq_at` **reverses its two
already-evaluated argument values inside its own body** and never reverses the
operand expressions at the use site. The distinction is observable: operand
evaluation stays single and left-to-right, so a `≥` occurrence evaluates its
left operand first exactly as every other application does. A definition that
rewrites `x ≥ y` into `y ≤ x` at the surface is **not** this binding and
changes evaluation order.

Every standard meaning here yields `Bool`. None yields an `Ω` proposition, none
introduces an equality refinement, and none converts a `Bool` result into a
proof witness. `∧` and `∨` are ordinary saturated applications: both operands
are evaluated, left to right, and **no short-circuit is guaranteed**. Legacy
`==` is unchanged by this section.

That is consistent with the `Bool` eliminator's arm laziness recorded at
`18a §5.4`, and the two are easy to conflate. `bool_and`'s body is a `match` on
its first argument, so exactly one arm is forced and the untaken arm is not.
But `x ∧ y` is a saturated application under call-by-value: **both operands are
already evaluated when the body runs**, and the arms then range over values.
The eliminator's non-forcing is therefore a property of the **body's arms**,
never of the occurrence's **operands**, and it yields no operand
short-circuiting at a use site. This is the same distinction `≥` turns on
above — a property of already-evaluated values is not a property of the operand
expressions — and reading `18a`'s "short-circuit inherent" as a claim about
operands moves it from one to the other.

Standard fixities, declared of the **bindings** and therefore travelling with
import and re-export exactly as `§6` states:

    ∧          infixr 3
    ∨          infixr 2
    ≤ ≥ ≠ ∈    infix 4

### 6.2 The `≠` carrier inventory, and its two distinct refusals

`≠` is the negation of **the exact comparator the existing `==` path selects**,
on the same carriers, with the same refusals. It is **not** universal
disequality and **not** `DecEq`: it is carrier-directed, and a carrier with no
registered comparator has no `≠`.

The comparator registry is closed by construction rather than by survey — the
table is a private field with a single mutator — and its rows are:

    carrier   key identity     comparator     availability
    Int       int_id           eq_int         always
    Float     float_id         eq_float       always
    Float32   float32_id       eq_float32     always
    Decimal   decimalpair_id   decimal_eq     when the decimal/char registration runs
    Char      char_id          eq_char        when the decimal/char registration runs

**Decimal is keyed on its registered representation, `decimalpair_id`.**
`Decimal := DecimalPair` is a transparent alias (`18a §5.6.1`), so
normalisation delivers the *representation's* identity and a lookup keyed on a
`decimal_id` is never performed. A rule naming only "Decimal" would describe a
key nothing looks up.

`Nat`, `Bool` and `String` are **absent** from the registry. That is the
measured inventory, not a sample.

Two refusals follow, and they are **not the same refusal**:

- **Table miss.** The operand carrier's head *is* a constant or an inductive
  former, and the registry has no row for it — `Nat`, `Bool`, `String` today.
  This is a **closable gap**: registering a comparator for that carrier closes
  it, and doing so is a separate decision this section does not take.
- **Structural refusal.** The operand carrier's head is neither a constant nor
  an inductive former — a bound type variable, an application, a `Π`. **No
  lookup is attempted at all.** This is not a gap and no future registration
  closes it: it is what `≠` *is*. Disequality at an abstract carrier is not
  expressible as a comparator selection, and a program that needs it must say
  which carrier it means.

An implementation that collapses these into one "unsupported carrier" outcome
is non-conforming even when both inputs are rejected, because the two states
differ in what would change them.

### 6.3 `∈` — the membership binding

`∈`'s standard meaning is the binding `membership_member_at`, whose dictionary
is a `Membership` provider (`../50-stdlib/58b §1`):

    membership_member_at (c : Type) (d : Membership c) (q : d.Query) (x : c) : Bool

It is an ordinary checked top-level binding, exactly as `§6.1`'s other four
are, and `§6.9`'s completion policy keys on its `GlobalId` on the same terms.
It is **not** a class method: `d.member` is a field projection and has no
`GlobalId` for the policy to key on (`39 §6.9`, `../50-stdlib/58b §5`).

`∈` takes `infix 4`, in the comparison band with `≤ ≥ ≠`.

**The query type is reached as a projection from an earlier parameter.** The
kernel admits this — with `Query` as the provider's first field the telescope
`[c, d, Proj1(d), c]` is closed by the binding's own parameter `d`, ordinary
dependency exactly like `(a : Type) (x : a)` — and this section specifies the
binding at that level.

> **The surface form now exists, and this prerequisite is discharged.** This
> clause previously read that Ken's surface type grammar had no projection
> form, so the catalog binding could not be *written* until one existed. That
> was true when the contract was authored and is no longer: `32 §2`'s `tproj`
> admits `d.Query` in type position, landed by
> `LANG-TYPE-PROJECTION-SURFACE-FORM`. The prerequisite is recorded rather
> than deleted because it was load-bearing — it is why
> `LANG-MEMBERSHIP-OPERATOR-SURFACE` could not be authored earlier, and the
> distinction it drew was the right one: the telescope was closed and
> kernel-checkable all along, and only its spelling was missing. **The
> spelling arrived; nothing about the telescope changed.**
>
> **It must not be discharged by making `∈` an elaborator builtin.** `§6.1`
> requires every standard meaning to be an ordinary top-level binding, and
> `§6.9`'s policy rests on that premise — a builtin would have no `GlobalId`
> to key on and would falsify the precondition the whole completion policy is
> built on, for `∈` and by precedent for every operator after it. The escape
> hatch is the one thing this clause exists to close.

## 7. What WS-L must deliver here

Definitions (incl. generic + mutually recursive under SCT), records (dependent +
refined fields, update/pun), the module/import system + package manager with
content-addressed lockfiles, visibility/abstraction, and the
class/instance/constraint mechanism with **lawful** instances and `derive`.
Conformance: `../../conformance/surface/declarations/`.

## 8. Named proof claims — `prop`, `theorem`, and attached `proof`

These declarations are surface/elaboration vocabulary over existing checked
terms only. They add no new kernel declaration class, no trusted proof table,
and no ambient proof search. The proof-claim surface has three roles:

- `prop` names a proposition family / claim shape.
- `theorem` names a reusable standalone checked proof theorem.
- `proof <name> for <subject>` names a checked proof term attached to a
  resolved subject API.

### 8.1 Proposition families — `prop`

`prop` is the claim-shape spelling for an `Omega`-valued family. A `prop`
declares a family whose telescope ends in `Omega`; the declaration is private
by default and may be exported with `pub prop`.

The family can carry an optional constructor-style `where` block:

```ken
prop AppendsTo (A : Type) : List A -> List A -> List A -> Omega where {
  nil  : AppendsTo A xs nil xs;
  cons : ...
}
```

The `where` block elaborates to ordinary checked introduction helpers over an
`Omega`-clean encoding. The public proposition remains proof-irrelevant; the
elaborator may keep an internal witness relation, but only the public `Omega`
proposition and checked intro helpers escape. If the elaborator cannot produce
an `Omega`-clean checked encoding, it rejects the `prop`.

Intro helper names are not bare module names. They are addressed canonically
through the family name, as `AppendsTo.nil`, `AppendsTo.cons`, and so on, and
import/export follows the family's visibility. A bare `nil` in the module
namespace is still just the ordinary value constructor if one exists; `prop`
does not reserve that bare space.

### 8.2 Attached proofs — `proof`

Attached proofs are checked proof definitions attached to a resolved subject.

```ken
proof appends for list_append
  (A : Type) (xs : List A) (ys : List A)
  : AppendsTo A xs ys (list_append A xs ys) = ...
```

The canonical path is `subject::proof_name` - for example
`Collections.List.list_append::appends`. The equivalent selector atom is:

```
proof_ref ::= "proof" ident "for" path
```

A `proof_ref` is a primary expression atom. Its subject is exactly one `path`,
so application binds outside the selector: `proof p for s a b` parses as
`((proof p for s) a) b`. The bare form `proof appends for list_append` and the
grouped form `(proof appends for list_append)` produce the identical
`Expr::EAttachedProofRef { subject, proof_name }` expression and desugar to the
same `subject::ident` global, where `ident` is the proof name. Parentheses are
optional grouping, not part of the selector atom.

Subject resolution runs first; attached lookup runs only after the subject is
resolved. A bare `appends` never resolves to the attached proof. This
expression-position atom does not change the declaration-position `proof`
head described above.

An attached `proof p for s` is well-formed iff the subject `s` **occurs
applied** somewhere in the proof's claim type φ — in a hypothesis or the
conclusion. The reading is broad: `proof p for s` names "a checked property
**of** `s`," not a proof whose telescope mirrors the subject's. A claim φ that
never mentions `s` applied is rejected before the proof is used. This is a
**surface well-formedness** condition only: attachment is namespacing over an
already-checked theorem (`§8.4`), so the theorem is checked identically whether
or not the condition holds — it carries **zero soundness weight**. The subject
must still be an **already-resolved definition** (a real precondition; a bare
or unresolved subject rejects). Duplicate proof names on the same resolved
subject reject. The same proof name may appear on different subjects because
the canonical paths differ.

Attached proofs are private by default, just like other declarations. `pub
proof p for s` is exportable only if `s` is exported. Importing an exported
subject makes its exported attached proofs available only through the explicit
attached path (`s::p`, or `M.s::p` under qualified import); it does not
ambiently import `p` as a bare value.

Same-subject attached proofs are **ordinary dependencies of one another**. An
attached proof **may** reference a sibling `subject::q` (directly or through a
helper): proving `s::antisym` via `s::trans`, or a mutually-recursive sibling
pair, is natural and admitted. Because proof declarations now go through the
shared SCC + SCT admission run (`§8.4`), a sibling reference is just an edge in
the dependency graph — an acyclic reference resolves in dependency order, a
mutual sibling cycle is admitted **iff SCT accepts** it, and a non-terminating
one **fails closed** with the SCT diagnostic. (The earlier blanket
no-sibling-dependency rejection is **withdrawn**: under the SCC + SCT run it
was both redundant and wrong.)

### 8.3 Standalone theorems — `theorem`

`theorem` is the standalone checked proof-definition form. It is a reusable proof
theorem in the ordinary module namespace, parameterized like a function and
instantiated by ordinary application.

```ken
theorem append_nil_right
  (A : Type) (xs : List A)
  : AppendsTo A xs nil xs = ...
```

The result annotation is required and must classify at `Omega`. The body is an
ordinary checked proof term; `theorem` is not a bundled `prop + proof`, not an
attached proof, and not a new kernel concept. If authors want an open
obligation, the existing `prove` path remains the status-bearing form.

A `theorem` (or attached `proof`) body **may self-recurse** — structural
induction on an argument — **and may mutually recurse** with other proof
declarations, **iff the recursion passes SCT** (`../10-kernel/17 §4`), on
identical terms to a recursive `const`/`fn`. Recursion is admitted **iff SCT
accepts**; an SCT-rejected proof recursion — a non-descending self-reference
such as `theorem bad : φ = bad`, or a mutual proof cycle with no decreasing
measure — **fails closed** with the SCT diagnostic, and no such body is ever
declared. The admission run and its soundness are stated in `§8.4`.

`theorem` obeys ordinary module visibility: private by default, `pub theorem` to
export, and imports/shadowing/ambiguity follow the `33 §3-4` module rules. A
theorem is never addressed as `subject::name` unless it is separately declared as
an attached proof, which is a distinct declaration.

Caller use is ordinary application:

```ken
let h1 = append_nil_right A xs
let h2 = (proof appends for list_append) A xs ys
```

Both are ordinary proof terms after resolution, so they can be passed to
transport, rewrite, congruence, induction, or class-law fields.

### 8.4 Admission and soundness of recursive proof claims

Wiring `theorem`/`proof` into the recursive-definition machinery is an
**elaborator admission** change only. It adds **no kernel rule, no trusted
proof table, no ambient proof search, and no `trusted_base()` entry** —
recursive `theorem`/`proof` declarations are admitted by the *same* checks a
recursive `const`/`fn` already passes. This section states the soundness of
that admission normatively; each claim is a property of the admission path, not
of any test.

**The admission path.** All top-level declarations in a scope — `const`/`fn`
and `theorem`/`proof` alike — are admitted through one shared, dependency-ordered
call-graph pass (`§1`):

1. **Dependency-ordered components deliver forward references.** The scope's
   declarations form a call graph — an edge is "the body **or type** of A
   mentions B" — condensed to strongly-connected **components** and processed in
   **dependency order**: every callee component before the callers that depend on
   it, where a component's dependencies are taken as the **union of all its
   members'** out-edges (so a dependency named only by a *non-entry* member of a
   mutual cycle is still ordered first). This ordering — not a scope-wide
   signature pre-pass — is what lets a declaration reference a sibling defined
   **later in source** (a **forward reference**): the sibling's component is
   fully elaborated first. A `theorem`/attached-`proof` type is required to classify
   at `Omega`, and an attached proof's subject must occur applied in its claim
   (`§8.2`); these gates run before that declaration's body in either branch
   below.
2. **SCT is bypassed only for a singleton with no self-edge at all.** A singleton
   component with **no self-edge** is genuinely non-recursive: its signature and
   body elaborate in one step and SCT is **not** consulted. A singleton *with* a
   self-edge is self-recursive and **is** SCT-gated — a **computational**
   (`const`/`fn`) self-recursion takes the **pre-existing singleton recursive
   path**, which SCT-gates it there; a **proof** self-recursion is newly routed
   to the group/SCT seam of point 3. Only the truly acyclic singleton skips SCT.
3. **A recursive component is signatures-first, then SCT-gated, before any body
   is committed.** A component that is a genuine cycle (size > 1) or a proof
   self-reference is **recursive**: **all** its members' signatures are
   pre-admitted **before any member body** — this is the signatures-first step,
   and it is what lets self- and mutual references *within the cycle* resolve —
   then the member bodies are elaborated and kernel-checked against their declared
   types, then **SCT is run on the whole component as one termination problem**.
   Only if SCT **accepts** are the bodies committed (made available for
   reduction); if SCT **rejects**, or any member fails its kernel check, the
   **entire component is rolled back and the declaration is rejected** — no
   recursive proof body is ever committed without passing SCT.

The four soundness properties this preserves:

- **(a) SCT is result-sort-agnostic.** The termination check is the *same*
  check `const`/`fn` use; it reasons about the call graph and structural descent
  of the bodies and **does not branch on whether a definition's codomain is
  `Type` or `Omega`**. An Ω-valued proof is admitted on exactly the termination
  obligation a `Type`-valued function of the same recursive shape would face.
- **(b) An SCT-accepted Ω definition is a valid proof.** For a total type
  theory, the whole obligation on a recursive inhabitant of an `Omega`
  proposition is that it be **strongly normalizing**; SCT-acceptance is that
  guarantee. There is no additional burden from the `Omega` codomain, so an
  SCT-accepted recursive `theorem`/`proof` is a sound proof of its proposition —
  and, conversely, the fail-closed SCT rejection is what keeps a **looping
  "proof"** of a false proposition (`theorem bad : φ = bad`) out.
- **(c) Proof-irrelevance is preserved — Ω proofs are never δ-unfolded in
  conversion.** Admitting recursive Ω-proofs imposes **no new conversion or
  reduction burden**, because the kernel's conversion check short-circuits on
  `Omega` by proof-irrelevance (`../10-kernel/16 §1`): two proofs of the same
  proposition are already convertible without unfolding either. This is
  unchanged by this iteration — the kernel is untouched — so a recursive proof
  body is never δ-unfolded during `conv`, and its recursion introduces no
  non-termination risk into conversion.
- **(d) Erasure and extraction are unchanged.** A proof at `Omega` carries no
  computational content and is erased at its use sites exactly as before;
  admitting proof recursion adds nothing an extractor must handle. This
  iteration touches no erasure or extraction path.

**Explicit out-of-scope boundary — mixed cycles fail closed.** A recursive
component that **mixes** a proof declaration with a computational (`const`/`fn`)
declaration is **rejected explicitly**, never silently admitted. Such a cycle
would couple a δ-unfolded, `Type`-relevant definition with a proof-irrelevant
Ω partner across the same termination measure — a nontrivial interaction
deferred out of this iteration; the boundary is made fail-closed so a program
that reaches for it gets a clear rejection rather than an unchecked admission.
(Homogeneous proof↔proof and computation↔computation cycles are the admitted
cases.)

**Precise boundary.** Everything above is realized in the elaborator's
admission path; the kernel, its conversion/erasure rules, and the trusted base
are untouched. The SCT check, the `Omega`-classification gate, and the per-body
kernel check are **pre-existing** kernel-and-elaborator machinery — this
iteration only routes proof declarations through the same dependency-ordered
SCC + SCT admission path `const`/`fn` already use. No claim here rests on a
test; the tests witness these properties but do not establish them.
