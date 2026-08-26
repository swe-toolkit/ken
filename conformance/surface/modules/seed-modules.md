# Modules & imports conformance — seed cases (ES3)

Format: `../../README.md`. These pin the **module / namespace substrate** of
`spec/30-surface/33-declarations.md §3–4` (ES3, the bounded L4 slice):
`module` / `import` / `pub` / abstract-export are a **surface +
elaboration-time only** device that **elaborates away** to the kernel's
**single flat append-only `Σ`** (`../../spec/10-kernel/11 §4`), with
**abstract export = the existing opaque constant** (`11 §4`: an opaque
constant "is how axioms, FFI signatures, and **abstract interfaces** are
represented"). **Zero new kernel feature, zero `trusted_base()` delta** — the
ES1 minimality invariant (surface built-in set ≡ `trusted_base()` delta,
`../taxonomy/minimality.md`) carries verbatim. N2 extends this same surface
substrate with an **in-repo cross-file loader**. N4 adds the source-world
`program` / `package` admission boundary over that loader. The
content-addressed package manager remains out of scope.

**Build state — N2 LANE B LANDED; contract completion mixed.** The cross-file
accept and active-stack cycle arms in §D drive the landed roots loader. Its
existing producer already supplies the valid anchor, one-root refusal, lazy
poison control, and per-run cache used by §D1–§D4. Unsupported `pub` placement,
the arbitrary-global fallback, and catalog-entry front-end routing remain
**RED UNTIL the module/import repair campaign**. Each new case records its own
reachable gate; a direct-loader success is not credited for a missing boundary.

The original ES3 module resolver, the N2 file-discovery producer, and the kernel
mechanisms they ride — the flat `Σ`, opaque constants, and `trusted_base()` —
are landed. The cases retain the ES3 design discriminants, the N2 controlled
cross-file accept↔cycle-reject pair, and add verdict-flipping controls for the
sharpened surface/loader contract without changing the kernel mechanisms.

**Build state — RED UNTIL N4 LANE B.** The §E source-world cases state N4's
anonymous boundary headers, direct-use admission gate, self-admission,
closure-wide coherence, and provenance contract. No current parser accepts the
new headers, so every §E case is red at N4's surface before Lane B. Compiled
instance manifests and re-export-carried instance surfaces are deliberately not
asserted as live: they remain package-manager/post-MRES-9 work.

Grounding (landed `§`-bodies + landed code, content-reconciled — not the
plan): `32 §1` (exclusive import suffixes and declaration-specific `pub`
eligibility), `33 §3` (`module`/`import M`/`import M as N`/
`import M (foo, Bar as Baz)`; dependency-closed source scopes; the kernel sees
a single flattened `Σ`), `33 §4` (visibility: module-private by default +
`pub`; abstract export = opaque interface), `39 §2.0` (the source-unit loader
algorithm and catalog-entry reachability), `11 §4` (the append-only acyclic
flat `Σ`; the opaque constant `c : A` that introduces abstract interfaces),
ES1 `minimality.md` (the `trusted_base()`-delta invariant ES3 must not
perturb), ADR 0014 MRES-1/MRES-2/MRES-3a, and the total role-blind dotted-path
↔ leaf-file bijection in `docs/program/07-catalog-style-guide.md §13`.

## Reading these cases — the ES3 disciplines

**Modules elaborate away — the load-bearing invariant (`33 §3`, AC1).** A
`module`/`import`/`pub`/abstract-export program and its **fully-qualified,
single-namespace equivalent** (every `M.foo` written flat as `M_foo`, no
module) elaborate to the **identical** flat append-only `Σ` — and therefore
the **identical `trusted_base()`**. The witness is **`Σ`-identity /
`trusted_base()`- identity**, not "it type-checks": a module is an environment
fragment resolved at elaboration, invisible to the kernel. The discriminating
direction: a design that put a **kernel-level module or visibility primitive**
into `Σ` (a new `trusted_base()` entry) **fails** the identity; the
elaborates-away form passes. This is the ES1 minimality net pointed at modules
— modules cost the trust root **nothing**.

**Abstract export IS the opaque constant, not a new mechanism (`11 §4`,
AC2).** A type exported name-only (constructors hidden) maps to the
**existing** opaque constant `T : Type` (`11 §4` — the same mechanism as an
FFI signature or an axiom), byte-identical to a hand-written opaque constant.
Information hiding is enforced at **elaboration** (the hidden constructors are
simply **not exported**, so not in scope for a client), **never** by a kernel
"abstract" flag. A client's `match` on a hidden constructor is a **surface**
name-resolution error, not a kernel type error.

**Visibility + resolution are surface-only (`33 §3`/`§4`, AC3/AC4).** Every
resolution failure — unresolved name, private-name access from outside,
ambiguous `use`-open — is a **surface diagnostic that never reaches the
kernel**. The visibility default is **private-by-default + `pub`** (AC4,
settled in `/spec` `33 §4`; the private-name-rejection case (§C) is its
conformance witness).

**Producer-grep (design-time, forward to ES3-build).** Drive the **real
import-resolution** — a case that asserts `M.foo` resolves by **constructing**
the resolved binding itself (rather than exercising the import rule) is
green-vs-green; pin resolution + the visibility rejection against the **stated
rules**, and the `Σ`/`trusted_base()` identity against the **landed** kernel.

## A. Modules elaborate away — zero TCB delta (AC1 ★)

### surface/modules/module-elaborates-to-identical-flat-sigma (soundness)
- spec: `33 §3`/`§3.1` (module = environment fragment → flattened `Σ`),
  `11 §4` (append-only flat `Σ`), ES1 `../taxonomy/minimality.md` (the
  `trusted_base()` invariant)
- given: two programs — (a) `module M { def foo : Nat := 0 }` with `import M`
  and a use of `M.foo`; (b) the **fully-qualified single-namespace
  equivalent** `def M_foo : Nat := 0` with a use of `M_foo` — each elaborated
  to core
- expect: the two produce the **identical** flat append-only `Σ` (same
  declarations, same order — the module is resolved away) and therefore the
  **identical `trusted_base()`** (`11 §4` filter); **no** module / visibility
  / namespace entry appears in `Σ` or the base
- why: (soundness) AC1, the elaborates-away invariant. A module is an
  elaboration-time **environment fragment**; the kernel sees one flat `Σ`
  (`33 §3`, `11 §4`). **Discriminating on `Σ`/`trusted_base()`-identity:** a
  design that admitted a **kernel module or visibility primitive** (a new
  `trusted_base()` entry) would make (a) ≠ (b) — this case **fails** it; the
  elaborates-away form passes. Grounds the ES1 minimality invariant: **adding
  ES3 leaves the surface `trusted_base()` delta unchanged.** Assert the **`Σ`
  / `trusted_base()` equality**, not "both type-check" (that passes
  vacuously).

## B. Abstract export IS the opaque constant (AC2)

### surface/modules/abstract-export-is-the-opaque-constant
- spec: `33 §4` (abstract export), `11 §4` (opaque constant introduces
  abstract interfaces)
- given: a `module M { pub data T = MkT ; … }` exporting `T` **abstractly**
  (name only, `MkT` hidden), vs a hand-written **opaque constant** `T : Type`
- expect: the abstractly-exported `T`'s **kernel representation is
  byte-identical to the opaque constant** — an opaque `T : Type` in `Σ`
  (`11 §4`), δ-blocking, no constructors visible; **no** kernel "abstract"
  flag or new `Decl` variant
- why: AC2, abstract export = the **existing** opaque-constant mechanism
  (`11 §4` — "how … abstract interfaces are represented"). **Discriminating:**
  a design that added a kernel-level "abstract" marker (a new `Decl`/`Σ` form)
  would make the rep **differ** from a plain opaque constant — this case pins
  them **identical**. Information hiding is surface/elaboration, not a kernel
  concept.

### surface/modules/client-match-hidden-ctor-rejected-at-surface (soundness)
- spec: `33 §4` (abstract export hides constructors), `33 §3.3` (resolution)
- given: `module M { pub data T = MkT }` exporting `T` **abstractly**, and a
  **client** module `import M` that attempts `match t { MkT => … }` on a
  `t : T`
- expect: **rejected at the surface** — `MkT` is **not in scope** (not
  exported; the abstract export withholds the constructor), a
  **name-resolution / surface diagnostic**, **not** a kernel type error, and
  the client is **not admitted**
- why: (soundness) AC2, the information-hiding enforcement is **surface**. The
  hidden constructor never enters the client's scope, so the `match` fails
  resolution **before** the kernel. **Discriminating:** the reject is a
  **surface** name error (`MkT` unresolved), **not** a kernel `TypeMismatch` /
  elaboration of a constructor — a design leaking `MkT` into scope (or
  enforcing the hiding by a kernel check) fails. Pairs with the `Σ`-identity
  of the abstract-export case: hiding is real **and** costs the kernel
  nothing.

## C. Visibility + resolution — surface-only, well-defined (AC3/AC4)

### surface/modules/private-name-access-rejected-at-surface (soundness)
- spec: `33 §4.1` (private-by-default + `pub`, settled), `33 §3.3`
  (resolution)
- given: `module M { def secret : Nat := 0 ; pub def api : Nat := 1 }` (no
  `pub` on `secret`), and a **client** `import M` that references
  **`M.secret`**
- expect: **rejected at the surface** — `secret` is **private**
  (module-private by default, not `pub`-exported), an **unresolved /
  not-exported surface diagnostic**, **not** a kernel error; `M.api`
  (exported) **resolves**
- why: (soundness) AC3 + the **AC4 witness** (private-by-default). Visibility
  is a **surface** predicate on the module interface; a private name is not in
  the export set, so a cross-module reference fails resolution **before** the
  kernel. **Discriminating flip:** `M.api` (pub) accepts, `M.secret` (private)
  rejects — keyed on the **`pub` export set**, on the real resolution rule,
  not a hand-fed visibility flag. Confirms the settled **private-by-default**
  default (`33 §4`, `OQ-syntax` resolved).

### surface/modules/top-level-local-import-clash-rejected
- spec: `33 §3.3` (module-level local/import clash, fail-closed)
- given: `M` exports distinct `foo` and `keep`; one client imports `M.foo`
  unqualified with `import M (foo)` and also declares a top-level `foo`. Run
  both declaration orders. In one arm, never reference either `foo` after
  declaring them.
- expect: both arms reject at the surface with the specific
  **`AmbiguousReference`** diagnostic for `foo`, identifying the distinct
  current-module `foo` and `M.foo` sources. The latent, never-referenced arm
  rejects at the same binding-time gate. Neither arm reaches the kernel.
- why: N3 AC2, the module-level reversal. The two declaration orders and the
  unused arm prevent a use-site-only ambiguity check or silent last-writer/local
  precedence from passing. **RED UNTIL N3 LANE B:** current `bind_import`
  silently keeps the local binding instead of raising the clash.

### surface/modules/import-de-selection-leaves-local-sole-binding
- spec: `33 §3.2`/`§3.3` (positive selection; omission resolves a clash)
- given: `M` exports distinct `foo` and `keep`; the client declares a top-level
  `foo`, then selects only `keep` with `import M (keep)`
- expect: **accepted**. Bare `foo` resolves to the client's local `GlobalId`,
  `keep` resolves to `M.keep`, and `M.foo` is not introduced unqualified.
- why: N3 AC2, the accept side of the name-only clash flip. Relative to the
  reject fixture, the imported `foo` is omitted; a resolver that imports the
  whole module despite the positive list still clashes or resolves `foo` to the
  wrong target.

### surface/modules/per-name-rename-resolves-distinct-targets
- spec: `33 §3.2`/`§3.3` (per-name rename resolves a clash)
- given: the same `M` exports distinct `foo` and `keep`; the client declares
  its own top-level `foo` and writes `import M (foo as bar)`, then references
  both bare names
- expect: **accepted**. Bare `foo` resolves to the local declaration's
  `GlobalId`; bare `bar` resolves to the distinct `GlobalId` of `M.foo`. The
  import creates no new declaration and leaves the two target IDs unequal.
- why: N3 AC2, a structural target discriminator. A parser that confuses the
  inner `as` with module aliasing, or a resolver that binds both spellings to
  one target, cannot satisfy both target assertions. **RED UNTIL N3 LANE B.**

### surface/modules/lexical-binder-still-shadows-imported
- spec: `33 §3.3` (narrower lexical scope, innermost wins)
- given: a client selectively imports `M.foo` with `import M (foo)` and defines
  `fn pick (foo : Nat) : Nat = foo`
- expect: **accepted**. The `foo` in `pick`'s body resolves to the parameter
  (the innermost local/de-Bruijn binding), not the imported `M.foo` `GlobalId`;
  no `AmbiguousReference` is raised.
- why: N3 AC2, the negative boundary of the reversal. This differs from the
  module-level reject by moving only the competing binding into a narrower term
  scope. A resolver that over-applies the new clash rule into lexical binders
  rejects and fails this case.

### surface/modules/prelude-clash-rejected-rename-local-resolves
- spec: `33 §3.3`/`§4` (prelude is an unshadowable primitive floor)
- given: paired clients: (a) declare `def Bool = Nat`; (b) instead declare
  `def LocalBool = Bool`, leaving the registered prelude `Bool`
  untouched
- expect: (a) rejects at the surface with **`AmbiguousReference`** for `Bool`,
  identifying the local and prelude sources; (b) accepts, with `LocalBool` and
  the prelude `Bool` resolving to distinct `GlobalId`s. There is no prelude
  exclusion input or positive opt-out arm.
- why: N3 AC2. Renaming only the local changes reject to accept while the
  prelude environment is fixed. A warn-and-allow policy, silent local win, or
  resolver that aliases `LocalBool` to prelude `Bool` fails. **RED UNTIL N3 LANE
  B.**

### surface/modules/per-name-rename-parses-hiding-is-syntax-error
- spec: `32` import EBNF, `33 §3.2` (selection item rename; no `hiding` form)
- given: parse `import M (foo as bar)` and, as the controlled negative, parse
  `import M hiding (foo)` against the same exported-name fixture
- expect: the first reaches import resolution as a selective per-name rename;
  the second rejects specifically with **`ParseError`** at `hiding`, before
  module loading or name resolution. It is not `UnboundName`,
  `AmbiguousReference`, or a later elaboration error.
- why: N3 AC2 pins both grammar orientations. Treating `as` only as a
  module-level alias rejects the positive arm; admitting any exclusion
  production accepts or advances the negative arm. **RED UNTIL N3 LANE B.**

### surface/modules/import-spellings-resolve-to-one-binding
- spec: `33 §3.2` (three import forms plus selective per-name rename)
- given: `module M { pub def foo : Nat := 0 }` and four clients — `import M`
  (uses `M.foo`), `import M as N` (uses `N.foo`), `import M (foo)` (uses
  `foo`), and `import M (foo as bar)` (uses `bar`)
- expect: all four spellings **resolve to the same underlying binding** `M`'s
  `foo` (the same core `GlobalId` in the flattened `Σ`); alias, selection, and
  per-name rename are surface names for one declaration
- why: AC3 plus N3 AC2, the accept anchor. A resolver that duplicates the
  declaration per spelling perturbs the flat `Σ`; one that mistakes per-name
  rename for module aliasing resolves the fourth arm differently. Drive the
  real resolver, not a hand-constructed `M.foo → GlobalId` map. The fourth arm
  is **RED UNTIL N3 LANE B**; the retained three arms remain live.

## D. In-repo cross-file loader (N2; landed producer)

These two cases are one controlled experiment. The root list, `A` unit, and
exported declaration in `B` are fixed. The reject arm changes only `B` by
adding the back-edge `import A`. Thus the observable flips from acceptance to
the named cycle rejection solely on the acyclic-versus-cyclic import-graph
axis; an implementation that never loads either file cannot make both arms
pass.

The harness supplies the resolver a **list of roots** containing exactly one
entry, `roots = [<fixture-root>]`. The spelling of the future Rust entry point
is not pinned here; its semantic input is pinned as the plural root list. Under
the strict, role-blind bijection, `A` and `B` name the unique leaf files
`<fixture-root>/A.ken.md` and `<fixture-root>/B.ken.md`. No in-file module
header or declaration-kind exception participates in resolution.

The harness designates `A` as the entry unit to elaborate in both arms. The
closed cycle is named in import-edge order rooted at that entry, fixing this
fixture's payload as `A → B → A`.

### surface/modules/cross-file-import-resolves-through-single-root-list

- spec: `33 §3.2` (N2 in-repo loader), ADR 0014 MRES-1/MRES-2/MRES-3a,
  `docs/program/07-catalog-style-guide.md §13` (total path↔file bijection)
- fixture: the resolver receives `roots = [<fixture-root>]`, with exactly these
  files:

  `<fixture-root>/A.ken.md`:

  ```ken
  import B

  const answer : Bool = B.value
  ```

  `<fixture-root>/B.ken.md`:

  ```ken
  pub const value : Bool = True
  ```

- expect: **accepted**. Loading `A` follows its `import B` edge lazily, maps
  `B` to the unique `B.ken.md` leaf under the sole populated entry of the
  plural root list, and resolves `B.value` to that file's exported `value`.
  Each unit is loaded and elaborated once in this run. A loaderless
  `UnboundName` for `B` does not satisfy the case.
- why: this drives the real cross-file producer: the consumer does not declare
  or pre-load `B.value`, and the harness does not hand-feed an export map.
  A singleton-only API, an eager whole-tree scan, a declaration-role-dependent
  path rule, or a resolver that stops at `UnboundName` fails at least one pinned
  observation. The case exercises the plural API with one root without
  specifying multi-root precedence.

### surface/modules/import-cycle-rejected-naming-closed-path

- spec: `33 §3.2` (cycle = hard surface error), ADR 0014 MRES-2
- fixture: keep the preceding root list and `A.ken.md` byte-identical. Keep
  `B`'s exported declaration byte-identical and add only the back-edge:

  `<fixture-root>/B.ken.md`:

  ```ken
  import A

  pub const value : Bool = True
  ```

- expect: **rejected at the surface** with the specific `ImportCycle`
  diagnostic kind and the closed cycle payload **`A → B → A`**. At diagnostic
  granularity, a loaderless `UnboundName`, a warning, silent SCC acceptance, or
  a bare `is_err` does not satisfy the case. The
  diagnostic must arise from the active import-stack cycle gate before either
  unit is admitted to the flattened `Σ`.
- why: this is an absence/rejection assertion with an exact gate. The active
  load of `A` requests `B`; the active load of `B` requests `A`, so the second
  `A` closes the named cycle. If Lane B had the precise target bug — accepting
  import SCCs or omitting the active-stack check — this arm would not reject at
  that gate. The acyclic arm above disconfirms coincidental `UnboundName` or
  fixture-syntax rejection: identical `A`, root input, and `B.value` accept
  when the sole back-edge is absent.

## D1. Completed grammar and visibility boundaries

These cases add only the boundaries sharpened by `32 §1`, `33 §3.3`, and
`39 §2.0`. They reuse the existing import, visibility, identity, cycle, and
flat-`Σ` fixtures above rather than making a second home for those properties.
Every rejection has an accepted control that reaches the same production or
loader path; a generic parse failure, missing fixture, or loaderless
`UnboundName` cannot satisfy the set.

### surface/modules/import-module-alias-and-selection-are-exclusive

- spec: `32 §1` (mutually exclusive `import_suffix` alternatives), `33 §3.2`
  (the three import forms)
- given: reuse the `M.foo` provider and the separately accepted
  `import M as N` and `import M (foo)` arms of
  `import-spellings-resolve-to-one-binding`. Against that same provider, a
  third client writes `import M as N (foo)`.
- expect: the two existing controls continue to reach module-alias and
  selective resolution respectively. The combined-suffix client rejects at
  the surface grammar gate before import resolution or kernel admission. The
  spec does not lock a diagnostic message or token span, so the case pins only
  the surface rejection and its phase.
- why: the controls prove that neither suffix is being rejected wholesale. The
  third arm changes only their forbidden combination. A grammar that parses an
  alias and then also consumes a selection list accepts the third arm and
  fails this case; a grammar that rejects every suffix fails a control.

### surface/modules/pub-eligibility-rejects-enumerated-ineligible-placements

- spec: `32 §1` (`visibility` is factoring, not blanket eligibility), `33 §4`
  (public module-interface names)
- given: first parse and elaborate `pub const visible : Bool = True`; it is the
  positive control that proves `pub` itself is live. Then take the smallest
  independently valid positive fixture for each row below and apply only the
  `pub` insertion shown. The source without that insertion must still reach its
  ordinary production in the same run.

  | ordinary accepted form | controlled mutation |
  |---|---|
  | `import M` from `import-spellings-resolve-to-one-binding` | `pub import M` |
  | `import M (foo)` from the same case | `import M (pub foo)` |
  | `export M (foo)` from `facade-reexport-preserves-global-id` | `pub export M (foo)` |
  | `export M (foo)` from the same case | `export M (pub foo)` |
  | the module from `module-elaborates-to-identical-flat-sigma` | `pub module M { … }` |
  | a positive `instance C T { … }` from `../classes/seed-classes.md` | `pub instance C T { … }` |
  | locked postfix derive `data T = MkT derive (DecEq)` (`33 §5.6`) | `data T = MkT pub derive (DecEq)` |
  | the ordinary anonymous `program` header fixture below | `pub program` |
  | the ordinary anonymous `package` header fixture below | `pub package` |
  | a positive `infixl 6 +` declaration `(gated: fixity surface)` | `pub infixl 6 +` |

- expect: `pub const visible` accepts and exports `visible`. Every ordinary row
  retains its independently pinned behavior. Every `pub`-bearing mutation
  rejects as a surface error attributable to an unsupported `pub` placement;
  **RED UNTIL the module/import repair campaign** at this eligibility gate;
  no mutation is accepted and ignored, and none reaches the kernel. The
  ordinary postfix-derive arm must drive the real derive generator and its
  kernel-checked candidate; parser-only recognition is not a positive control.
  The exact diagnostic variant is not pinned because `32 §1` locks eligibility
  and phase, not error spelling. The fixity row becomes live with its ordinary
  positive control when `fixity_decl` is reachable; until then both arms are
  explicitly gated rather than crediting an unrelated unknown-keyword error.
- why: this is an allowed-inventory check over the forms explicitly forbidden
  by `32 §1`, not a token grep. The eligible declaration prevents an
  implementation from satisfying the matrix by rejecting all `pub`. The
  per-row ordinary controls prevent a missing base production from masquerading
  as visibility enforcement. The matrix catches both a blanket `Decl::Pub`
  wrapper that accepts and later ignores unsupported declarations and item
  parsers that silently admit per-item visibility.

## D2. Fresh dependency-closed scopes

The next three cases distinguish a fresh per-unit scope from a shared
compilation-run global map. They use the real loader and real imported
interfaces. A harness that hand-feeds an export map or starts each source in an
unrelated `ElabEnv` does not satisfy them.

### surface/modules/dependency-cannot-borrow-callers-selective-import

- spec: `33 §3.3` (per-unit dependency closure), `39 §2.0` step 4
- fixture: the resolver receives `roots = [<fixture-root>]` and entry `A`, with
  these units:

  `<fixture-root>/C.ken.md`:

  ```ken
  pub const helper : Bool = True
  ```

  `<fixture-root>/A.ken.md` in both arms:

  ```ken
  import C (helper)
  import B

  pub const result : Bool = B.value
  ```

  `<fixture-root>/B.ken.md` in the reject arm:

  ```ken
  pub const value : Bool = helper
  ```

  The accept control adds only `import C (helper)` to `B` before its
  declaration.
- expect: the reject arm reports `helper` unbound while resolving `B`'s surface
  body and admits neither `B` nor `A`. The control accepts and resolves
  `B.value`. `A`'s selective import is unchanged in both arms.
- why: loading `B` because `A` imports it is not scope inheritance. A loader
  that reuses `A`'s import bindings while elaborating `B` accepts both arms;
  adding the one explicit edge to `B` must be what flips rejection to
  acceptance.

### surface/modules/dependency-import-does-not-leak-back-to-caller

- spec: `33 §3.3` (imports are non-transitive in both directions), `39 §2.0`
  step 4
- fixture: use the same `C.ken.md`. `B.ken.md` is fixed in both arms:

  ```ken
  import C (helper)

  pub const value : Bool = helper
  ```

  The reject arm's entry unit is:

  ```ken
  import B

  pub const result : Bool = helper
  ```

  The accept control adds only `import C (helper)` to `A`.
- expect: `B` accepts in both arms. The reject arm then reports `helper` unbound
  in `A`'s surface scope; the control accepts. Loading `B` does not itself bind
  `helper` in `A`.
- why: this is the converse orientation of the preceding case. A loader that
  unions a dependency's selective bindings into its caller accepts the reject
  arm. Keeping `B` byte-identical and changing only `A`'s explicit import makes
  the verdict attributable to caller scope construction.

### surface/modules/closed-floor-accepts-arbitrary-global-does-not

- promise class: **normative compatibility vector** — the exact floor is closed
  in both directions
- stage: **RED-UNTIL `LANG-MOD-CANONICAL-PAIR-PACKAGE` floor realization**
- spec: `30-taxonomy §4` (closed prelude floor), `33 §3.3` (exact ten-type
  floor, three Pair companions, and no convenience-global fallback), `39 §2.0`
  step 4
- given: in a fresh harness arm, first use the ordinary non-loader elaboration
  path to register the transparent Ken definition `def Ambient = Bool` under
  the bare implementation-global spelling `Ambient`. Then invoke the strict
  roots loader. The controlled arms are:

  1. entry `Floor` contains checked aliases reaching all ten floor types,
     values and exhaustive matches reaching every floor constructor, all three
     Pair companion bindings, `bytes_at`/`bytes_slice` uses whose results are
     matched as `Option`, a `bytes_decode` use matched as
     `Result Utf8Error String`, and functions typed by `Cap a` and
     `Resource k`;
  2. entry `Leaky` contains only `pub def X = Ambient`;
  3. entry `Explicit` contains `import Provider (Ambient)` followed by
     `pub def X = Ambient`, while `Provider.ken.md` contains
     `pub def Ambient = Bool`;
  4. entry `Convenience` contains only `pub def P = Prod Bool Bool`.

  Arms 2 and 3 both retain the pre-registered bare `Ambient` in the same kind
  of `ElabEnv`; arm 4 retains the compiler-installed checked `Prod`. No
  test-only export map is installed.
- expect: arm 1 accepts and every resolved type, constructor, and Pair-companion
  reference carries the recorded pre-source `GlobalId`. Arm 2 rejects `Ambient`
  as unbound even
  though the implementation global exists. Arm 3 accepts and resolves `X`
  through the imported provider declaration, not the bare global. Arm 4 rejects
  `Prod` as unbound. None changes `trusted_base()`.
- why: the positive reaches each floor member through ordinary source and the
  public primitive signatures that require the added types. `Bool` and
  `Ambient` are definitionally the same in this fixture, so kernel typing cannot
  distinguish arm 2; only source scope can. `Prod` separates a pre-installed
  checked convenience from a later arbitrary global. Widening all globals,
  removing the floor, or forbidding explicit imports each fails a different arm.

The nine-member Nat realization is landed. The Pair additions to this floor arm
and the Pair-specific rows below are **RED UNTIL the redirected
`LANG-MOD-CANONICAL-PAIR-PACKAGE` floor-realization build**. The former
strict-Pair rejection is historical evidence, not a sentinel retained beside
the new contract.

### surface/modules/prelude-floor-reuses-exact-types-and-constructors

- promise class: **durable invariant** — availability changes, canonical
  identity and trust do not
- spec: `30-taxonomy §4` (both membership arms), `33 §3.3`, `39 §2.0`
- given: create a fresh `ElabEnv` and record the `GlobalId` and declaration kind
  of `Auth`, `Bool`, `Char`, `List`, `Nat`, `Option`, `Pair`, `ResourceKind`,
  `Result`, and `Utf8Error`; record every constructor id and kernel parent;
  record `mk_pair`, `pair_fst`, and `pair_snd` plus their exact reference to the
  Pair id; and snapshot `declarations().len()`, `next_global_id()`, and
  `trusted_base()`. Through strict roots elaborate an entry with checked
  declarations that reach every recorded id, including actual
  `bytes_at`/`bytes_slice`/`bytes_decode`, `Cap`, and `Resource` signatures.
- expect: every emitted type/body contains the corresponding recorded id. The
  inductive members and their exact constructors are:

  | parent | constructors |
  |---|---|
  | `Auth` | `ANone`, `APartial`, `AFull` |
  | `Bool` | `True`, `False` |
  | `List` | `Nil`, `Cons` |
  | `Nat` | `Zero`, `Suc` |
  | `Option` | `None`, `Some` |
  | `ResourceKind` | `FsHandle`, `Buffer` |
  | `Result` | `Err`, `Ok` |
  | `Utf8Error` | `InvalidUtf8` |

  `Char` and checked-transparent `Pair` have no constructors. Pair's three
  companions are bindings, not type members, and each resolves to its recorded
  id. Every constructor still reports the recorded parent. Declaration count
  and allocator advance by exactly the number of source declarations, with no
  extra family, constructor, or companion allocation. No recorded floor id
  enters `trusted_base()`.
- why: ids and parentage are the property. Comparing names or data shapes cannot
  distinguish a replacement family, and a source-only positive that never uses
  the byte/capability/resource primitives would not prove their public result
  and parameter types are nameable.
- **MEASURED:** the landed nine-type floor reuses all recorded ids and preserves
  accounting and trust. At base `c1945c6fbbd7b0d8422123904fc6f7138fc85df9`,
  the four transparent Pair-family declarations already exist as `g232`–`g235`
  and remain untrusted. **CLAIMED:** the ten-type floor and three-companion
  inventory expose those existing checked identities without creating or
  trusting anything. **THE GAP:** the build must capture the Pair family through
  the closed floor path, derive every constructor from recorded parentage, and
  reject equal-shaped or same-spelling substitutions.

### surface/modules/prelude-floor-clash-and-lookalike-matrix

- promise class: **durable invariant** — the immutable floor fails closed on
  every non-canonical same-spelling origin
- spec: `33 §3.3` (top-level local × prelude clash), `39 §2.0`
- given: use a fresh strict-roots environment per row. For each of the eight
  inductive parents in the table above:

  1. keep only the parent spelling canonical while renaming all constructors;
  2. in separate entries, keep only one constructor spelling canonical while
     renaming the parent and all sibling constructors;
  3. as the reaching positive, rename the parent and every constructor while
     preserving the same declaration shape.

  For constructor-free `Char`, pair `def Char = Int` with the same-production
  positive `def LocalChar = Int`. For transparent `Pair` and its three companion
  bindings, run the independent four-row collision matrix in
  `seed-pair-strict-boundary.md`; pair every same-spelling reject with an
  all-renamed transparent positive.
- expect: every same-spelling row raises `AmbiguousReference` naming the one
  retained floor spelling before any declaration or `GlobalId` is allocated.
  Every all-renamed positive accepts. Each renamed inductive former and
  constructor has an id distinct from every floor id, and each constructor's
  parent is its renamed local former; `LocalChar` is likewise a distinct checked
  transparent id. The Pair-family rows likewise reject before allocation, while
  every renamed transparent lookalike receives a fresh id. Every row preserves
  `trusted_base()`.
- why: a generic `expect_err`, or one all-names-collide fixture, can pass at the
  parser, positivity checker, or the wrong collision. One-axis rows plus
  same-production positives establish reachability and exact error phase for
  every binding. The companion matrix prevents a type-only floor check from
  leaving helper replacement open. The renamed lookalikes prove structural
  equality is not canonical identity.
- **MEASURED:** all-renamed same-shape families elaborate under distinct ids;
  current root loading still admits and shadows a same-spelling floor
  declaration. **CLAIMED:** every floor name is immutable and rejects before
  allocation. **THE GAP:** add the fail-closed collision at the fresh root-unit
  scope; accepting a lookalike is the present implementation defect this case
  must redden on.

### surface/modules/ord-nat-class-owner-and-reexport-use-one-dictionary

- promise class: **durable invariant** — one canonical dictionary and one
  defining provenance
- spec: `33 §4.3`, `§5.3`, `§5.5.1`; `39 §6.1`;
  `50-stdlib/51 §7`
- given: strict-load the realized `Core.Classes.LawfulClasses` and
  `Data.Numeric.Nat.Order` units. Record the kernel-origin floor `Nat` id and the
  `Ord_instance_Nat` dictionary declared by `LawfulClasses`. Exercise implicit
  `where Ord Nat` once through the class package's public surface and once
  through `Order`'s reader-facing re-export/admission surface.
- expect: the instance record's head is the exact floor `Nat`; both use
  sites select the same dictionary `GlobalId`; successful-resolution provenance
  names `Core.Classes.LawfulClasses`; and the environment contains one
  `(Ord, Nat)` structure entry. Loading/re-exporting `Order` adds no second
  dictionary. The one instance is transparent and kernel-rechecked, adding zero
  `trusted_base()` entries.
- why: selection and provenance are asserted structurally, not inferred from a
  generated name or export text. A build that redeclares the instance in
  `Order`, keys it on a same-shaped family, or rewrites provenance fails at
  least one independent assertion.
- **MEASURED:** both public routes select one id whose registered head and
  defining package are fixed. **CLAIMED:** re-export carries rather than owns or
  duplicates `Ord Nat`. **THE GAP:** §5.5.1 carry must be the route that grants
  the second use; the fixture supplies no direct admission of a second provider.

### surface/modules/prelude-head-does-not-transfer-orphan-ownership

- promise class: **durable invariant** — compiler-installed floor availability
  creates no orphan exception
- spec: `33 §4.3`/`§5.3`, `39 §6.1`
- given: use a minimal structure class `FloorOrder a` with one identity
  operation so the ordinary class/instance production is reached without the
  full `Ord` proof payload. Controlled roots place
  `instance FloorOrder Nat` (a) in the module that defines `FloorOrder`, and
  (b) in an unrelated module that only imports/re-exports `FloorOrder` and sees
  ambient `Nat`. A third control changes only the head to a `LocalNat` data
  family defined beside the instance.
- expect: (a) accepts by the class-owner arm; (b) rejects specifically with
  `OrphanInstance { class = FloorOrder, head_type = Nat }` before registration;
  and (c) accepts by the head-owner arm under a distinct `(FloorOrder,
  LocalNat)` key. Re-exporting either name in (b) does not change its verdict.
- why: the three loci exercise both lawful orphan arms and the forbidden middle
  from the same production. The actual `Ord Nat` positive is separately pinned
  above, so this smaller fixture isolates the general owner predicate rather
  than hand-feeding the desired dictionary.
- **MEASURED:** declaration locus alone selects class-owner accept, unrelated
  reject, and local-head accept. **CLAIMED:** the prelude floor and re-export do
  not transfer head ownership. **THE GAP:** `FloorOrder` stands for the generic
  orphan rule, while the preceding actual-package case binds that rule to
  `Ord Nat`.

## D3. Root, source-path, and lazy-discovery boundaries

The valid anchor for this matrix is
`cross-file-import-resolves-through-single-root-list`: one root, entry `A`, one
unique `A.ken.md` source leaf, and a real `A → B` dependency. The rows below
modify only the named root/path axis. They do not choose precedence among
multiple roots.

### surface/modules/root-and-source-leaf-refusal-matrix

- spec: `33 §3.2` (one-root source-world round and strict path bijection),
  `39 §2.0` steps 1-2
- given: apply these mutations independently to the valid anchor:

  | row | root/path mutation |
  |---|---|
  | zero-root | pass `roots = []` with entry `A` |
  | two-root | pass `[r1, r2]`, with a valid byte-identical `A` leaf in both |
  | invalid-component | pass entry `A.lower`, whose lowercase component is not a module component |
  | no-leaf | pass entry `Missing`, with no corresponding source leaf |
  | dual-extension | add byte-identical valid `A.ken` beside `A.ken.md` |
  | leaf-directory | add directory `A/` beside the valid `A.ken.md` leaf |

- expect: the unchanged anchor accepts. Each mutation rejects at the surface
  before any declaration from the offending entry is admitted. Zero-root and
  two-root reject at root cardinality without probing for precedence.
  Invalid-component rejects path validation; no-leaf rejects source lookup;
  dual-extension and leaf-directory reject the corresponding non-unique source
  identity. The spec locks these concepts and phases, not diagnostic strings,
  so exact messages are not part of the oracle.
- why: the accepted anchor proves the loader, source syntax, and file contents
  are otherwise viable. Each row isolates one guard. Picking the first root,
  preferring `.ken` over `.ken.md`, treating a path as both leaf and directory,
  normalizing a lowercase component, or falling through after no leaf makes its
  row accept or reach the wrong phase.

### surface/modules/unimported-poison-is-lazy-imported-poison-rejects

- spec: `33 §3.2` (lazy edge discovery), `39 §2.0` steps 1-2
- fixture: retain the accepted `A → B` root from
  `cross-file-import-resolves-through-single-root-list`. Add an otherwise
  unrelated module `Z` with both `Z.ken` and `Z.ken.md` present and
  byte-identical, so resolving `Z` has the dual-extension surface error from
  the preceding matrix.
- expect: with `A.ken.md` unchanged and no edge to `Z`, elaborating entry `A`
  accepts; the poison is not inspected. Add only `import Z` to `A` and the same
  run rejects at `Z`'s dual-extension source-identity gate. It does not accept,
  report a later name error, or admit `A` first.
- why: a pure successful result cannot reveal an eager scan, so the inert
  poisoned sibling is the structural discriminator. An eager tree walk rejects
  both arms; a loader that never follows imports accepts or misattributes the
  second; only entry-rooted lazy discovery produces accept then reject when the
  sole edge is added.

## D4. Catalog-entry front-end reachability

### surface/modules/catalog-root-entry-check-drives-real-loader

- spec: `39 §2.0` (catalog-root-addressed front ends use the source loader),
  `33 §3.2` (entry-rooted cross-file graph)
- given: reuse the exact root, `A.ken.md`, and `B.ken.md` from
  `cross-file-import-resolves-through-single-root-list`, but invoke the public
  front-end operation that checks module `A` as an entry addressed through that
  catalog root. Supply only the root and entry identity; do not pre-load `B`,
  hand-feed exports, or pass `A` as an isolated file. The concrete CLI/API
  token spelling is `(oracle)` because the spec locks the addressing mode and
  behavior, not a command-line flag.
- expect: the front end accepts and `A`'s reference resolves to `B.value`
  through the same loader contract as the direct N2 harness. Deleting or
  bypassing catalog-root loader routing makes this fixture reject with `B`
  unbound and therefore fails the case. **RED UNTIL the catalog-entry front-end
  route lands:** an isolated-file checker is not a partial pass.
- why: the direct roots-loader case proves the producer works when called; this
  case proves the public catalog-addressed consumer actually calls it. The
  input contains no second route by which `B.value` can appear. The adjacent
  poison pair separately prevents an eager whole-tree scan from satisfying
  reachability by accident.

## D5. Existing module properties retained by reference

This fold does not restate the existing module substrate:

- canonical provider identity and no replacement `GlobalId` remain pinned by
  `import-spellings-resolve-to-one-binding` and
  `../declarations/seed-namespace-export.md`;
- active-stack cycle rejection remains pinned by
  `import-cycle-rejected-naming-closed-path`;
- the flat append-only `Σ` and identical `trusted_base()` delta remain pinned by
  `module-elaborates-to-identical-flat-sigma` and
  `../taxonomy/minimality.md`.

The new cases compose with those homes. A passing strict-scope, path, or
front-end row never substitutes for identity, cycle, or trust-root evidence,
and those existing cases do not substitute for the new boundary flips above.

For pin design, these are the explicit measured/claimed seams. “Measured” here
names what the completed case observes; it does not claim a red-until arm is
already built.

| case group | MEASURED by the case | CLAIMED | THE GAP closed by |
|---|---|---|---|
| suffix | two separate suffixes accept; their combination rejects | suffix alternatives are exclusive | same provider and two independently reaching controls |
| `pub` | eligible `pub` accepts; each named insertion rejects | eligibility is declaration-specific | unmodified production control per row; fixity row gated |
| caller import | adding only the dependency's own import flips its verdict | a dependency cannot borrow caller imports | caller and provider sources otherwise fixed |
| dependency import | adding only the caller's own import flips its verdict | dependency imports do not leak back | dependency source byte-identical |
| floor/global | nine exact type identities plus parent-derived constructors accept; bare ambient and `Prod` reject; explicit import accepts | floor is closed and arbitrary/pre-installed non-members do not resolve | actual primitive uses, exact ids/parentage, definitionally equal ambient type, imported control |
| root/path | valid anchor accepts; each one-axis mutation rejects at its guard | root and source identities fail closed | one valid loader input plus independent matrix rows |
| laziness | inert poison accepts; adding its sole edge rejects at the poison | discovery follows only entry-rooted edges | identical poisoned tree in both arms |
| front end | root-addressed entry resolves its otherwise unavailable dependency | catalog entry reaches the roots loader | only root and entry supplied; direct-loader producer control separate |

## E. Source-world program/package admission (N4; RED UNTIL LANE B)

These fixtures use packages delivered from source through the N2 loader. This
describes their delivery form, not ADR 0014's boundary-less “source package.”
A package path is the defining-package identity reported by the oracle.
`import` remains the ordinary-name channel; `admits` is independently the
instance channel. The class and head declarations named below are ordinary
valid §5 declarations, and every non-collision provider satisfies the existing
orphan rule before the N4 gate is exercised.

For each successful dictionary lookup, the harness records the resolved
dictionary `GlobalId`, canonical `(class, head)` key, and defining package. The
literal harness field names are not pinned. Where N4 locks diagnostic
provenance, the harness inspects structured package paths and the canonical key;
matching only rendered prose is insufficient. Every case in this section is
**RED UNTIL N4 LANE B** unless stated otherwise.

### surface/modules/two-explicit-admits-resolve-ambient-with-provenance

- spec: ADR 0014 MRES-4/MRES-4c; WP N4 AC2 (source-world admission and
  provenance)
- fixture: package `P`, delivered from source, defines class `Render`, head
  `PItem`, and the sole canonical `instance Render PItem`. Package `Q`, also
  delivered from source, defines head `QItem`; its instance unit explicitly
  writes `import P (Render)` before declaring the sole canonical
  `instance Render QItem`. This is the only inter-package edge, **`Q → P`**;
  `P` has no edge to `Q`. Together with the program-to-provider imports below,
  the complete ordinary import graph is acyclic. The program unit is:

  ```ken
  program
  admits P, Q

  import P (Render, PItem)
  import Q (QItem)
  ```

  Two ordinary declarations in that unit independently require
  `Render PItem` and `Render QItem`, forcing both real instance-search paths.
- expect: **accepted**. Both lookups are ambient—no per-use instance import is
  present—and return the unique canonical dictionary. The first lookup records
  `defining_package = P`; the second records `defining_package = Q`. Their
  `GlobalId`s are distinct and both provenance fields are present.
- why: this is the admitted success anchor. Removing `Q` from only the
  `admits` line must not silently keep the second success merely because
  `import Q` makes `QItem` nameable. Conversely, removing `import Q` may make
  the name unavailable but does not change which instances the boundary admits.

### surface/modules/transitive-coherence-does-not-grant-direct-dispatch

- spec: ADR 0014 MRES-4c (coherence set versus direct-use set); WP N4 AC2
- fixture: package `Q` defines the sole canonical `instance QMark QItem`.
  Package `P` is also delivered from source, declares its own boundary, and has
  this anonymous package file:

  ```ken
  package
  admits Q
  ```

  One of `P`'s units explicitly writes `import Q (QMark, QItem)` and dispatches
  that instance. This is the only provider edge, **`P → Q`**; `Q` has no edge
  to `P`. Together with the program unit's import below, the complete ordinary
  import graph is acyclic and `Q` is genuinely transitive from the program's
  admitted root.

  The program admits only `P`, while its own unit imports the ordinary names
  from `Q` and directly dispatches `QMark QItem`:

  ```ken
  program
  admits P

  import Q (QMark, QItem)
  ```

- expect: **rejected** at instance dispatch with the specific
  `UnadmittedInstance` variant carrying
  `defining_package = Q` and `instance = (QMark, QItem)`. `Q` is nevertheless
  present in the full coherence closure through `P`; the diagnostic is not
  `UnboundName`, `MissingInstance`, `OrphanInstance`, or an overlap error.
- why: this is the two-set discriminator. A buggy gate keyed on the transitive
  coherence closure accepts; a buggy coherence pass filtered to the explicit
  root never observes `Q`. The correct implementation observes `Q` for total
  coherence but rejects the program unit's direct dispatch because `Q` is not
  in the explicit root. As the controlled accept arm, changing only the program
  line to `admits P, Q` accepts and records `defining_package = Q`.

### surface/modules/single-package-self-admits-without-program

- spec: ADR 0014 MRES-4b and package extension; WP N4 AC2
- fixture: one package `Solo`, delivered from source, contains class `SoloMark`,
  head `SoloItem`, its valid canonical instance, and an ordinary declaration
  that dispatches `SoloMark SoloItem`. There is **no program file**, no synthetic
  `admits Solo`, and no second instance-providing package in the source graph.
- expect: **accepted**. The lookup returns the canonical dictionary with
  `defining_package = Solo`; absence of a program header is not an error.
- why: this pins zero-ceremony self-admission. An implementation that requires a
  program for every build rejects; one that disables admission checking
  globally cannot also satisfy the transitive-unadmitted reject above.

### surface/modules/intra-package-duplicate-canonical-rejected

- spec: ADR 0014 MRES-4/MRES-4f (source coherence by construction still
  retains intra-package overlap); `33 §5.3`/`§5.5`
- fixture: one package `P` owns both structure class `Render` and head `PItem`.
  In the same owning module, two declaration sites each define
  `instance Render PItem`; each declaration separately satisfies the orphan
  predicate. An anonymous `package` boundary contains the module, with no
  inter-package import edge.
- expect: the first canonical instance registers; the second rejects at the
  existing `OverlappingInstances` gate for `(Render, PItem)`, carrying both
  declaration spans. Both declarations are in package `P`; no later dispatch or
  import order chooses one silently.
- why: this is the source-constructible §5.5 control after MRES-4f. It reaches
  registration without `UnboundName`, `ImportCycle`, or `OrphanInstance`, then
  flips on only the second same-key declaration. Admission does not replace the
  intra-package overlap gate.

### surface/modules/cross-package-overlap-attempt-is-import-cycle

- spec: ADR 0014 MRES-4f; `33 §3.2` (N2 cycle gate), `§5.3`
- fixture: module `P.Class` owns structure class `Render`; module `R.Head` owns
  head `RItem`. `P.Class` imports `R.Head` to name `RItem` for its candidate,
  while `R.Head` imports `P.Class` to name `Render` for its candidate. The
  harness designates `P.Class` as the entry unit. Both packages are listed by
  the anonymous program boundary, but the ordinary source graph necessarily is
  **`P.Class → R.Head → P.Class`**.
- expect: **rejected upstream** with the specific `ImportCycle` diagnostic and
  closed payload `P.Class → R.Head → P.Class`. Neither candidate registers, so
  the source run does not emit `OverlappingInstances` or a both-package
  collision diagnostic.
- why: this positively asserts MRES-4f's constructive guarantee. The two legal
  orphan loci force opposite import edges; deleting either edge changes the
  failure to `UnboundName`, and accepting both edges would violate N2. Thus at
  most one package can legally define a `(class, head-constructor)` key in one
  acyclic source graph.

### surface/modules/admission-does-not-waive-orphan-rejection

- spec: ADR 0014 MRES-4 (admission composes with orphan and overlap);
  `33 §5.3`
- fixture: the program explicitly admits package `Bad`, but `Bad` declares
  `instance Render RItem` in module `Bad.Orphan`, which owns neither name.
  `Bad.Orphan` explicitly imports `P.Class` for `Render` and `R.Head` for
  `RItem`: edges **`Bad.Orphan → P.Class`** and
  **`Bad.Orphan → R.Head`**. Neither owner imports `Bad.Orphan`, so the complete
  ordinary import graph is acyclic and both names resolve before the orphan
  check. The control relocates the declaration to the module owning `RItem` and
  leaves the program's `admits Bad` line unchanged.
- expect: the first arm rejects at declaration with the specific
  `OrphanInstance` variant and its class/head/declaration provenance; it never
  becomes a registered candidate. The relocated control passes the orphan gate
  and is eligible for registration. It makes no claim that later direct-use
  admission succeeds, because the relocated declaration is not defined by
  package `Bad`.
- why: admission is additive, not a replacement coherence policy. The pair
  changes the declaration locus after the class and head have resolved. The
  explicit one-way edges disconfirm an earlier `UnboundName` or `ImportCycle`;
  an implementation treating `admits` as permission to register an orphan
  flips the wrong arm to registration.

### surface/modules/boundary-headers-are-anonymous

- spec: ADR 0014 MRES-4e/MRES-4a; WP N4 AC2
- fixture: parse two controlled pairs with the same following `admits` section:
  bare `program` versus `program App`, and bare `package` versus `package Lib`.
  No arm contains or implies an entry-point declaration.
- expect: each bare header reaches its `admits` list. `program App` rejects with
  the specific `ParseError` variant at `App`; `package Lib` rejects with
  `ParseError` at `Lib`, both before package lookup, admission, or instance
  search. Neither `App` nor `Lib` becomes an identity or provenance label.
- why: the only identity is the file/package path and the header's presence is
  the role marker. A parser accepting documentary names creates a second,
  divergent identity source. The bare controls disconfirm a parser that simply
  rejects both new keywords.

## F. Compiled-manifest collision (RED UNTIL PACKAGE-MANAGER ROUND)

This forward oracle is normative but **not live in N4 Lane B**. Unlike the
source cases, independently compiled manifests meet at an admission boundary
without sharing one N2 import graph; MRES-4f's source-cycle theorem therefore
does not preclude a genuine cross-package collision here.

### surface/modules/compiled-manifest-collision-names-both-packages

- spec: ADR 0014 MRES-4c/MRES-4f and PKG-3
- fixture: independently compiled package manifests for `P` and `R` each commit
  a canonical structure instance under the same `(Render, RItem)` key. A parent
  boundary admits both manifests and performs PKG-3 cross-boundary coherence
  re-checking. No source import edge connects the two already-compiled packages.
- expect: **rejected** at manifest admission with the canonical-instance
  collision diagnostic carrying key `(Render, RItem)` and both
  `defining_package = P` and `defining_package = R`. Neither manifest order nor
  parent import order may choose a winner. **RED UNTIL the compiled-manifest /
  package-manager round:** N4 Lane B has no manifest input and cannot run this
  oracle.
- why: this is the genuine home of both-package collision provenance. A
  package manager that trusts internal commitments but omits PKG-3's
  cross-boundary re-check silently composes two canonical dictionaries and
  fails this case. The kernel's later dictionary re-check cannot restore
  coherence, so this remains a required package-manager diagnostic.

## G. Program-header capability manifest (I-4 §C)

These fixtures extend N4's anonymous `program` boundary with the capability
manifest settled by the I-4 ruling. The selected source spelling is
`capabilities`, with one family and authority per item:
`capabilities FS AFull`. It declares source-owned authority and does not
suggest an external grant counterparty; it is also distinct from
expression-level `requires`. The grammar and spec artifact remain the authority
for the production.

The `admits` and `capabilities` clauses share a header but not a namespace or
reader.
The former supplies package paths to the instance-admission gate; the latter
supplies effect-family authorities to the runner's capability mint. Harness
field names for the two manifest projections are not pinned, but their values,
separation, and consumers are.

I-4 §D lands the source parser, boundary AST, loader projection, and separate
reader seams. I-4 §B is not landed on this seed's base. Each gate below names
the remaining reachability precondition. A harness that directly constructs a
boundary record, inserts a capability into an `ElabEnv`, or calls the raw I-3
producer does not satisfy any gate.

The I-4 §C RESHAPE keeps `readFile` authority-polymorphic and `writeFile`
monomorphic at `AFull`. Both wrappers only consume the opaque capability minted
from the parsed header. Ken source has no capability constructor,
capability-producing `attenuate`, or callable `revoke`; the separate
monotone-downward and revocation management actions remain runner/host-internal
(`62 §3`/`§4`).

### surface/modules/program-capabilities-clause-carries-declared-authority

- spec: I-4 §C frame deliverable 1 and the accepted I-4 ruling §C/A.2
- fixture: elaborate this boundary and import prefix through the real unit
  parser and loader:

  ```ken
  program
  admits P
  capabilities FS AFull

  import P (Render, PItem)
  ```

  Package `P` is the same valid, acyclic provider used by
  `two-explicit-admits-resolve-ambient-with-provenance`. The unit contains an
  ordinary declaration that requires `Render PItem`, so the `admits P` half is
  observed rather than decorative. It also contains I-4 §B's canonical valid
  Program-I entry, which passes its minted `fullCap : Cap AFull` directly to
  `readFile AFull fullCap path`. No user-level attenuation or capability
  construction intervenes, so the capability half reaches the real runner
  rather than stopping at a parsed record.
- expect: the parser accepts the anonymous header and the boundary manifest
  records exactly `P` in its admitted-package set and exactly `(FS, AFull)` in
  its declared-capability map. The two entries remain separately addressable.
  Once I-4 §B consumes that parsed manifest, the runner reads the `FS` entry
  and produces `ProgramCaps AFull`; it does not derive the authority from `P`,
  a CLI option, or a default.
- gate: the parser/manifest assertion is **REALIZED by I-4 §D** through the
  source parser, boundary AST, loader projection, and separate reader seams.
  The runner assertion remains **RED UNTIL I-4 §B**. Its reachability is
  parser → boundary AST → loader manifest → runner manifest read →
  `ProgramCaps AFull`. A hand-built manifest or a directly minted `Cap AFull`
  is not evidence.
- why: changing only `AFull` to `APartial` must change the recorded authority
  and resulting `ProgramCaps` index while leaving the admitted package and
  dictionary identity unchanged. A parser that drops the new clause, a runner
  that keeps the old fixed `APartial`, or a shared-list implementation cannot
  satisfy that controlled flip.

### surface/modules/fs-effect-without-capability-clause-is-ill-typed

- spec: I-4 §C frame deliverable 5 and the accepted I-4 ruling §A.5
- fixture: use the same program unit, imports, entry signature, and a real
  `readFile APartial partialCap path` call as the accepted `FS APartial`
  control. The negative removes only this header line:

  ```ken
  capabilities FS APartial
  ```

  It leaves `program`, `admits P`, the resolved typed wrapper, and the body
  performing the `FS` effect unchanged. The control and negative are compiled
  from source; the harness does not inject a capability binding.
- expect: the control elaborates through capability passing. The no-clause arm
  rejects before execution with the specific structured
  **`MissingCapability`** diagnostic carrying `effect = FS`. The error is the
  elaborator's named presentation of the kernel-backed ill-typedness;
  it is not `ParseError`, `UnboundName`, `UnadmittedInstance`,
  `CapabilityDenied`, or a bare error result.
- gate: **RED UNTIL I-4 §B.** I-4 §D realizes the parser and boundary-reader
  dependency, but the loader must still resolve the typed API and elaboration
  must reach §B's capability-binding step for the otherwise-identical `FS`
  call. Failure to load `readFile`, calling the raw I-3 producer, or observing
  only an op-time denial does not satisfy this oracle.
- why: correct and buggy paths have opposite verdicts. With the one declared
  family the binding exists and the read is well typed; without it the binding
  is absent and the named static gate rejects. A runner-only check would accept
  both source units and defer the negative to execution, so it fails the flip.

### surface/modules/readfile-is-authority-polymorphic

- spec: `38 §1.3.1` and the I-4 §C RESHAPE ruling
- fixture: compile two source programs through the parsed-header-to-runner path.
  They have identical entry bodies modulo the authority index and header item:

  ```ken
  program capabilities FS APartial
  -- partialCap comes only from ProgramCaps APartial
  readFile APartial partialCap path
  ```

  ```ken
  program capabilities FS AFull
  -- fullCap comes only from ProgramCaps AFull
  readFile AFull fullCap path
  ```

  Neither program names `attenuate`, constructs `Cap`, or calls the raw I-3
  producer.
- expect: both calls elaborate against the one authority-polymorphic signature
  `readFile : (a : Auth) -> Cap a -> Bytes -> FS a (Result FileError Bytes)`.
  With readable files in the capture host, both execute through the wrapper and
  return the file bytes. A fixed `Cap APartial` wrapper rejects the `AFull` arm;
  a fixed `Cap AFull` wrapper rejects the `APartial` arm.
- gate: **RED UNTIL I-4 §B.** Reachability is source header → I-4 §D boundary
  projection → §B runner mint → `ProgramCaps a` pattern → the actual `readFile`
  wrapper → unchanged I-3 `read_bytes` producer → driver. A hand-created `Cap`,
  a direct `read_bytes` call, or a host-side attenuation call is not evidence.
- why: the two authorities are a controlled signature discriminator. The
  direct `AFull` read also rejects the deleted coarse-v1 surface: an
  implementation that still requires `readFile (attenuate fullCap) path`
  cannot satisfy the case.

### surface/modules/anone-readfile-is-typed-but-denied-at-operation

- spec: `38 §1.3.1`, the I-4 §C RESHAPE ruling, and the I-3 FS driver contract
- fixture: a source program declares `capabilities FS ANone`, obtains only the
  resulting `noneCap : Cap ANone` from `ProgramCaps ANone`, and calls
  `readFile ANone noneCap path` through the actual wrapper. The capture host
  contains a readable file at `path`.
- expect: the call is **well typed** and reaches the driver. The driver returns
  the named `Err (MkFileError ... CapabilityDenied)` result before any host
  read; the FS trace remains empty. It is not a static `TypeMismatch`,
  `MissingCapability`, parse error, or successful read.
- gate: **RED UNTIL I-4 §B.** The parser/reader prerequisites are REALIZED by
  I-4 §D, but the oracle requires §B's real polymorphic wrapper, header-derived
  `Cap ANone`, driver authority check, named error payload, and pre-host trace.
  A directly invoked authority predicate or hand-fed error is not evidence.
- why: authority polymorphism deliberately moves the read floor from the type
  signature to operation time. A stale static `APartial` floor rejects before
  the driver; a missing driver check reads the file. The named result and empty
  trace distinguish both bugs.

### surface/modules/writefile-requires-afull-statically

- spec: `38 §1.3.1` and the I-4 §C RESHAPE ruling
- fixture: compile a pair differing only in the declared authority and the
  resulting `ProgramCaps` index. The `AFull` arm passes its minted
  `fullCap : Cap AFull` to `writeFile`; the `APartial` arm passes its minted
  `partialCap : Cap APartial` to the same wrapper call. Both name the same path,
  policy, contents, and `FS` row.
- expect: the `AFull` arm elaborates and writes through the capture host. The
  `APartial` arm rejects before execution with the named kernel-backed
  `TypeMismatch` diagnostic: actual capability type `Cap APartial`, required
  type `Cap AFull`. It does not reach `CapabilityDenied`, and its host trace is
  empty.
- gate: **RED UNTIL I-4 §B.** Reachability requires the actual checked
  `writeFile : Cap AFull -> ...` wrapper and both capabilities minted from their
  parsed headers. Calling raw `write_file`, constructing either capability, or
  asserting only an op-time denial does not satisfy the oracle.
- why: this is the non-degenerate static pair for the write guarantee.
  Accidentally making `writeFile` authority-polymorphic accepts the `APartial`
  arm; retaining the ruled monomorphic signature makes only `AFull` type-check.

### surface/modules/no-ken-callable-capability-introduction

- spec: `38 §1.3.1`, `62 §2.2`/`§3`/`§4`, and the I-4 §C RESHAPE ruling
- fixture: after §B registers `readFile` and `writeFile`, inspect the real Ken
  source environment and separately attempt to resolve `attenuate` and `revoke`.
  Enumerate the opaque `Cap` declaration's constructor surface rather than
  guessing a constructor spelling.
- expect: the two consuming wrappers are present; resolution fails with the
  named `UnboundName { name = attenuate }` and
  `UnboundName { name = revoke }` surface diagnostics; and `Cap` exposes no
  constructor. No other Ken-callable global has a result headed by `Cap`. The
  runner/host's internal mint, attenuation, and revocation management remain
  outside this source environment.
- gate: **RED UNTIL I-4 §B.** Before the positive wrappers land, absent
  management names alone are vacuous. The gate is reachable only when the same
  real environment contains both wrapper consumers while still exposing no
  capability producer, constructor, attenuator, or revoker.
- why: the positive-and-absence conjunction prevents green-vs-green. A build
  that drops the wrappers or exposes either raw action or a differently named
  capability producer fails the structural enumeration.

### surface/modules/admits-only-does-not-mint-capability

- spec: I-4 §C frame deliverable 5(c) and the accepted ruling §C
- fixture: an acyclic program imports the valid `P` provider and dispatches its
  canonical instance, with this complete boundary header:

  ```ken
  program
  admits P
  ```

  The unit performs no `FS` effect and contains no `capabilities` clause.
- expect: instance search accepts and records `defining_package = P`, exactly
  as in the N4 admitted control. The separately projected capability manifest
  is empty. The program boundary does not infer `FS`, `APartial`, or `AFull`
  from the admitted package, its imports, or the presence of `program`.
- gate: the admission half is **REALIZED by N4 Lane B**, and the separate empty
  capability projection is **REALIZED by I-4 §D**. If I-4 §B observes the
  program, its no-`FS` behavior remains **RED UNTIL I-4 §B** and must be read
  from that parsed empty projection, not from a constructed runner input.
- why: a shared manifest or `admits`-implies-authority coupling spuriously
  populates the capability projection. Removing only `admits P` changes the
  dictionary verdict to `UnadmittedInstance`; it must not change the already
  empty capability projection.

### surface/modules/capability-only-does-not-admit-instances

- spec: I-4 §C frame deliverable 5(c), ADR 0014 MRES-4, and the accepted
  I-4 ruling §C
- fixture: parse this capability-only boundary, with no synthetic or empty
  `admits` line:

  ```ken
  program
  capabilities FS AFull
  ```

  The positive body performs an `AFull` operation through the typed I-4 API and
  uses no ambient instance. A controlled negative adds only an ordinary import
  of `P (Render, PItem)` and a declaration requiring `Render PItem`; it still
  has no `admits P`.
- expect: the positive header is accepted, its admitted-package set is empty,
  and its declared-capability map is exactly `(FS, AFull)`. At I-4 §B the
  runner reads that entry and supplies `ProgramCaps AFull`. The controlled
  instance-use arm rejects independently with the existing structured
  `UnadmittedInstance` diagnostic carrying `defining_package = P`; the valid
  capability declaration does not admit `P` or alter that diagnostic.
- gate: header parsing and its two independent projections are **REALIZED by
  I-4 §D**. The controlled instance reject is **REALIZED by N4 Lane B** through
  the ordinary loader and admission gate. The mint and typed-operation
  assertions remain **RED UNTIL I-4 §B** and must be reached from the parsed
  header. Hand-feeding either map is not evidence.
- why: this is the converse controlled experiment to the admits-only case.
  Adding only `admits P` flips the instance-use arm to acceptance and leaves
  `ProgramCaps AFull` unchanged. Treating `FS` as a package path, `AFull` as an
  admitted identity, or one clause as enabling the other's reader fails one of
  the two orientations.

## Coverage map (AC → cases)

- **AC1** (modules add zero to the TCB):
  `module-elaborates-to-identical-flat-sigma` (soundness).
- **AC2** (abstract export = opaque constant):
  `abstract-export-is-the-opaque-constant`,
  `client-match-hidden-ctor-rejected-at-surface` (soundness).
- **AC3** (visibility + resolution surface-only):
  `private-name-access-rejected-at-surface` (soundness),
  `import-spellings-resolve-to-one-binding`.
- **AC4** (visibility default settled): witnessed by
  `private-name-access-rejected-at-surface` (private-by-default); the OQ
  resolution itself is `/spec §33 §4` + `90-open-decisions.md`.
- **N2** (cross-file path resolution + cycle hard-error + plural-ready roots):
  `cross-file-import-resolves-through-single-root-list` and
  `import-cycle-rejected-naming-closed-path`.
- **Module/import contract completion** (`32 §1`, `33 §3.3`, `39 §2.0`):
  `import-module-alias-and-selection-are-exclusive`,
  `pub-eligibility-rejects-enumerated-ineligible-placements`,
  `dependency-cannot-borrow-callers-selective-import`,
  `dependency-import-does-not-leak-back-to-caller`,
  `closed-floor-accepts-arbitrary-global-does-not`,
  `root-and-source-leaf-refusal-matrix`,
  `unimported-poison-is-lazy-imported-poison-rejects`, and
  `catalog-root-entry-check-drives-real-loader`.
- **Exact prelude floor and `Ord Nat` provenance** (`30 §4`, `33 §3.3`/
  `§4.3`/`§5.3`, `39 §2.0`/`§6.1`, `51 §7`):
  `prelude-floor-reuses-exact-types-and-constructors`,
  `prelude-floor-clash-and-lookalike-matrix`,
  `ord-nat-class-owner-and-reexport-use-one-dictionary`, and
  `prelude-head-does-not-transfer-orphan-ownership`.
- **N3** (module clash error + explicit resolution, lexical boundary, prelude
  floor, and grammar): `top-level-local-import-clash-rejected`,
  `import-de-selection-leaves-local-sole-binding`,
  `per-name-rename-resolves-distinct-targets`,
  `lexical-binder-still-shadows-imported`,
  `prelude-clash-rejected-rename-local-resolves`,
  `per-name-rename-parses-hiding-is-syntax-error`, and the renamed arm of
  `import-spellings-resolve-to-one-binding`.
- **N4** (source-world admission boundary):
  `two-explicit-admits-resolve-ambient-with-provenance`,
  `transitive-coherence-does-not-grant-direct-dispatch`,
  `single-package-self-admits-without-program`,
  `intra-package-duplicate-canonical-rejected`,
  `cross-package-overlap-attempt-is-import-cycle`,
  `admission-does-not-waive-orphan-rejection`, and
  `boundary-headers-are-anonymous`.
- **N4 forward / package-manager gate** (compiled collision + both-package
  provenance): `compiled-manifest-collision-names-both-packages` (**RED UNTIL
  the compiled-manifest/package-manager round**).
- **I-4 §C** (program-header capability manifest, static family gate, and
  orthogonality): `program-capabilities-clause-carries-declared-authority`,
  `fs-effect-without-capability-clause-is-ill-typed`,
  `readfile-is-authority-polymorphic`,
  `anone-readfile-is-typed-but-denied-at-operation`,
  `writefile-requires-afull-statically`,
  `no-ken-callable-capability-introduction`,
  `admits-only-does-not-mint-capability`, and
  `capability-only-does-not-admit-instances` (parser / N4 / I-4 §B gates are
  labeled per assertion).

## Cross-case consistency sweep

- **The kernel never sees a module (`33 §3`, `11 §4`).** AC1 (`Σ`-identity),
  AC2 (abstract = opaque constant, no kernel flag), AC3 (every reject is a
  **surface** diagnostic) are one story: **modules/visibility/abstract-export
  exist only at elaboration; the kernel sees one flat `Σ` and nothing
  module-shaped.** A case asserting a kernel-level module entry, a kernel
  "abstract" flag, or a **kernel** (not surface) visibility error would
  contradict this class.
- **Rejects are surface name-resolution, not kernel type errors.**
  `client-match-on-hidden-constructor-…` and `private-name-access-…` are one
  class: the failure is an **unresolved / not-exported** *surface* diagnostic
  that **never reaches the kernel** — never a `TypeMismatch` or an
  admitted-then-caught kernel state.
- **Import is re-naming, not re-declaration.**
  `import-spellings-resolve-to-one-binding` and
  `module-elaborates-to-identical-flat-sigma` agree: every import form
  resolves to **one** underlying `GlobalId`; a form that re-declared per
  import would perturb the flat `Σ` (contradicting AC1).
- **Module clashes and lexical shadowing are disjoint gates.** A top-level
  local plus unqualified import rejects at binding time, including when unused;
  moving only that local into a function parameter accepts and resolves the
  body structurally to the innermost binder. Neither verdict can be implemented
  by a blanket "locals win" or blanket "same spelling errors" rule.
- **Both explicit module-clash resolutions leave one target per bare name.**
  De-selection leaves local `foo` as the sole bare binding. Per-name rename
  leaves local `foo` and imported `bar` as two names for two distinct target
  IDs. Both agree with import-as-renaming and the flat-`Σ` invariant.
- **Prelude remains present in both prelude arms.** The reject arm conflicts
  with the fixed prelude `Bool`; the accept arm changes only the local spelling.
  No import list, `hiding` form, or prelude opt-out participates.
- **The N2 pair differs only by one import edge.** The same one-entry root
  list, `A` source, `B.value` declaration, strict bijection, and qualified use
  appear in both arms. With no `B → A` edge, `B.value` resolves and accepts;
  with that sole edge, the active stack closes `A → B → A` and the specific
  cycle gate rejects. No other case in this seed changes that verdict.
- **Fresh scopes cut both transitive directions.** One pair holds the caller's
  selective import fixed and changes only whether the dependency imports it;
  the converse holds the dependency fixed and changes only the caller's import.
  The floor/global case independently separates allowed ambient vocabulary from
  an arbitrary pre-registered Ken global. None can be implemented by clearing
  all imports or disabling the prelude.
- **Floor availability, identity, and instance ownership are separate axes.**
  Strict uses of all ten floor types, every exact constructor, and all three
  Pair companions accept on the recorded ids, including the public primitive
  signatures that require the signature arm. Every same-spelling floor binding
  rejects while each all-renamed lookalike gets distinct ids and local
  parentage. Ambient availability still does not make an unrelated module a
  head owner. The actual `Ord Nat` and Pair-instance cases then require
  class-owned provenance. Widening globals, comparing only shapes, or treating
  re-export as ownership satisfies at most one axis.
- **Root refusal does not decide multi-root precedence.** The root/path matrix
  accepts the one-root anchor and rejects zero or two populated roots in this
  source-world round. It says nothing about how a later package-manager round
  orders two roots after that round makes the input legal.
- **Lazy discovery is observable through poison reachability.** The same
  dual-extension poison exists in both arms and only the import edge changes.
  Acceptance without the edge and source-identity rejection with it are
  incompatible with both eager scanning and never following imports.
- **Front-end reachability and loader correctness are separate.** The direct
  N2 case proves the roots loader's behavior. The catalog-entry case supplies
  the same semantic input only through the public front end; both must pass,
  while the poison pair prevents eager scanning from standing in for routing.
- **N4 keeps names, admission, and coherence as three distinct gates.**
  `import` makes a package's exported names available; it does not admit the
  package's instances. The explicit root grants direct dispatch; it does not
  filter the transitive coherence closure or waive orphan/overlap. The
  transitive reject and its one-line accept control prove the two N4 sets are
  neither conflated nor disconnected.
- **N4 provenance follows the defining declaration, never the importing unit.**
  Both admitted successes name their distinct provider packages; the
  unadmitted error names `Q`. The deferred manifest collision enumerates `P`
  and `R`. These observations use structured package-path fields rather than
  import aliases or header labels; the live intra-package overlap retains its
  established both-declaration-span payload.
- **MRES-4f separates a source theorem from manifest defense-in-depth.** In one
  source graph, opposite class-owner/head-owner edges reject at `ImportCycle`
  before a cross-package duplicate can register; the same package still reaches
  `OverlappingInstances`. Independently compiled manifests have no shared N2
  graph, so their both-package collision remains a required deferred check.
- **I-4 keeps admission and authority as orthogonal header projections.** The
  admits-only and capability-only cases are converse controls. Adding or
  removing `admits P` changes only the instance verdict; adding or removing
  `capabilities FS a` changes only the capability binding and `ProgramCaps a`.
  A
  package import never mints authority, and a capability family never admits a
  package. The combined-header case observes both readers in one source unit.
- **Static absence and op-time insufficiency are different gates.** No `FS`
  clause reaches `MissingCapability { effect = FS }` during elaboration. A
  declared `ANone` read is well typed but reaches the separately specified
  `CapabilityDenied` driver backstop. An `APartial` write instead fails the
  monomorphic `Cap AFull` type gate before execution. These three outcomes are
  distinct; a parse or loader error satisfies none.
- **Capability introduction is runner-only.** The polymorphic read pair and
  monomorphic write pair consume only header-derived `ProgramCaps` fields. The
  adjacent surface-enumeration case requires those consumers to exist while
  `attenuate`, `revoke`, and every `Cap` constructor/producer remain absent.
  Semantic management in `62 §3`/`§4` stays at the runner/host boundary, never
  re-exported to Ken.

## Subsumed / not-duplicated (one home per property)

- **Generic effect capability presence** remains EFF3's
  (`../effects/seed-effects.md`, `36 §2.5/§7.3.2`). Section G does not re-pin
  the general `Cap E` rule; it pins the new program-header clause as the source
  of that existing binding. Its controlled pair changes only the header line
  and must reach the same landed `MissingCapability { effect = FS }` gate.
- **Generic `§5` constraints / typeclasses-as-subobjects** remain **Lc's**
  (`../classes/seed-classes.md`, `33 §5`, landed). The two Nat cases here do not
  re-pin class elaboration or the general orphan suite; they pin the new
  bootstrap-head interaction: no source head owner exists, so the existing
  class-owner arm is the only canonical `Ord Nat` placement and re-export does
  not change it.
- **The opaque constant + the flat `Σ` + `trusted_base()`** are the
  **kernel's** (`11 §4`; `../taxonomy/minimality.md` for the delta). ES3
  observes abstract export **as** the opaque constant and modules **as**
  transparent over the flat `Σ`; the mechanisms are the kernel's home.
- **The content-addressed package manager / registry / persisted manifests**
  remain a later round (`63` supply-chain). N4 asserts only source-world
  `program` / `package` / `admits`, instance visibility, and provenance.
  Section F records one explicit RED-UNTIL forward oracle for cross-manifest
  collision provenance; it does not claim a live manifest input. Compiled-
  manifest source-equivalence remains normative forward compatibility.
- **Generic re-export-carried instance surfaces** are specified in `33 §5.5.1`
  and realized by the landed carry computation. The Nat case composes that
  mechanism with a bootstrap head and asserts only identity/provenance; it does
  not duplicate the broader carry suite.
- **Runtime entry selection** is separate from admission (MRES-4a). No fixture
  invents an entry declaration or treats a `program` header as one. I-4's
  §B-dependent cases consume the Program-I entry contract only after its real
  source path lands; this seed does not invent entry syntax.
- **The N3 clash/rename suite does not re-pin the loader.** Its fixtures use
  loaded module interfaces but assert only binding-time diagnostics and target
  identities. The N2 pair remains the home for cross-file success and active-
  stack cycle behavior; §D3 owns the newly explicit root/path/laziness
  boundaries.
- **Canonical identity, cycle behavior, and flat-`Σ` trust posture are not
  duplicated by §D1–§D4.** Their existing homes are enumerated in §D5. The new
  cases assert only grammar eligibility, scope closure, loader refusal, lazy
  discovery, and public front-end reachability.
- **Multi-root precedence** is deferred. The valid current input has one
  populated root. Rejecting the two-populated-root matrix row in this round does
  not choose how a future round resolves or orders that input once it becomes
  legal.

## Build realization (N2 Lane B)

N2 Lane B implements the in-repo loader. Its producer gate is the real
import-edge traversal from the plural root input: the accept arm resolves `B`,
and the cycle arm rejects specifically at `ImportCycle` with `A → B → A`. The
existing `Σ` / `trusted_base()` identity (AC1), abstract-export identity (AC2),
and visibility diagnostics (AC3) remain unchanged. No hand-constructed export
map satisfies the pair.

## Build-forward (module/import contract completion)

The follow-on implements §D1–§D4 without replacing the landed N2 producer. It
makes import suffixes exclusive, rejects `pub` outside the allowed declaration
inventory, constructs a fresh strict scope for every root-loaded unit, fails
closed on root/source-path ambiguity, preserves lazy edge discovery, and routes
the public catalog-entry checker through the roots loader. The three existing
identity/cycle/flat-`Σ` homes in §D5 remain unchanged. A direct call to the
loader does not discharge the front-end row, and an eager scan does not
discharge the lazy-poison pair.

## Build-forward (closed signature + internal-provision floor)

`LANG-MOD-NAT-FLOOR-REALIZATION` landed the signature-eight plus kernel-origin
`Nat` nine-type floor. The redirected
`LANG-MOD-CANONICAL-PAIR-PACKAGE` build extends the same mechanism with
compiler-origin `Pair`, producing the explicit ten-type inventory
`{Auth, Bool, Char, List, Nat, Option, Pair, ResourceKind, Result, Utf8Error}`
and the separate three-companion inventory
`{mk_pair, pair_fst, pair_snd}`. The executable closure check derives the
signature eight and exact internal `{Nat, Pair}` independently; it never widens
resolution from compiler-global presence. Constructor capture remains exact
parent-derived. Pair-family capture reuses the four pre-source ids and checks
that every companion type references the exact Pair id. The build flips the
four strict-Pair rows, keeps non-members such as `Prod` unavailable, and closes
every Pair-binding clash before allocation. Canonical `Ord Nat`, `Ord Pair`,
and `DecEq Pair` placement follows the class-owner rule. Floor installation adds
zero declarations, ids, or trusted entries.

## Build-forward (N3 Lane B)

This N3 addition is conformance-only. Lane B adds the per-name rename parser
arm and replaces `bind_import`'s silent local-wins behavior with the specific
binding-time clash error, including latent clashes and the fixed prelude floor.
It must not change narrower lexical resolution. No N3 case reaches the kernel,
adds a declaration to `Σ`, or changes `trusted_base()`.

## Build-forward (N4 Lane B)

N4 Lane B implements only the source-world boundary. It parses anonymous
headers, forms the explicit direct-use set, retains the unfiltered transitive
coherence closure, applies one package-membership check after real instance
search, and reports defining-package provenance. It retains §5.5's
intra-package `OverlappingInstances` gate. Cross-package duplicate candidates in
one source graph reject earlier at N2 `ImportCycle` by MRES-4f; Lane B does not
promise an unreachable both-package collision diagnostic. All §E rejects are
surface/elaboration diagnostics and add nothing to the flat `Σ` or
`trusted_base()`. Section F's both-package diagnostic remains RED UNTIL compiled
manifests meet at the package-manager admission boundary. Registries, lockfiles,
content addressing, and test-scoped admission stay unbuilt; the landed
re-export carry computation is reused rather than deferred here.

## Build-forward (I-4 §B + parser dependency)

The parser dependency must populate both header projections from source before
§B consumes them. I-4 §B then reads only the capability projection to construct
`ProgramCaps a`, resolves the typed capability API, and emits
`MissingCapability` when an otherwise-reachable effect family has no
declared binding. The N4 admission reader remains unchanged. Tests that insert
either projection directly, mint a `Cap` outside the runner path, or exercise a
raw authority-polymorphic I-3 producer do not discharge §G.
