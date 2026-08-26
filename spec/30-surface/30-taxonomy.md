# The surface taxonomy: built-in, prelude, standard-package

> Status: **Normative** for the built-in/prelude/package **line** and the
> minimality invariant; the concrete membership lists grow with WS-L (each entry
> settled by the derivation-path table, `../../conformance/surface/taxonomy/`).
> This is the organizing chapter for the everyday surface: it fixes *what must
> be primitively provided* versus *what is ordinary Ken*, mirroring the kernel
> discipline at the surface. It elaborates the operator principle — **built-ins
> = the minimum set from which everything else is built; everything beyond is a
> standard package** — and makes it a **soundness** property, not just an
> ergonomic one.

## 1. The principle — the surface is a small generating core plus derived Ken

The kernel is a small audited trust root; everything else is re-checked. The
surface mirrors this: a **minimal built-in set** (the surface analog of the TCB)
generates the everyday surface, and everything else is **ordinary Ken** the
kernel re-checks. Minimality is not an aesthetic — it is **TB-Sound +
TB-Complete at the surface** (`../60-security/64 §1.1`):

- a "built-in" that is actually Ken-**derivable** is **bloat** — a surface
  over-claim, the over-large-TCB failure (surface TB-Sound: no phantom
  built-in);
- a "package" with **no** derivation path from the built-ins is a **hidden
  built-in** — an assumption that slipped in unlisted (surface TB-Complete: no
  hidden built-in).

### 1.1 The invariant (normative)

> **The surface built-in set ≡ the surface `trusted_base()` delta.** A prelude
> or standard entry that has a Ken-**derivation witness** lands as a re-checked
> `definition` (a `declare_def`, **out** of `trusted_base()`); only an entry
> with a genuine **irreducibility witness** stays a `postulate`/`primitive`
> (**in** `trusted_base()`). So the set of surface built-ins and the audited
> trusted base **coincide** — the surface analog of TB-Complete's
> choke-point-equals-filter (`64 §1.2`).

This makes "built-in vs package" **the same line** as "audited trust-root vs
re-checkable Ken." The derivation-path table (CV's `/conformance` artifact) that
proves the taxonomy *is* simultaneously the TCB-hygiene proof: it drives the
invariant in **both** directions — no built-in has a derivation path
(irredundant) and no package/prelude entry lacks one (complete).

## 2. The three tiers

| Tier | What it is | Trust level | In `trusted_base()`? |
|---|---|---|---|
| **Built-in** | irreducible — cannot be defined in Ken from other built-ins | the **surface TCB**: audited primitives / assumed at the boundary | **yes** (primitive/postulate) |
| **Prelude** | Ken-defined and **always present**: a primitive signature names it, or internal provision exposes one canonical checked identity | re-checked `definition` | **no** |
| **Standard package** | Ken-defined, **optional**, explicit `import` | re-checked `definition` | **no** |

The two lower tiers are both **re-checked Ken** (out of the trusted base); they
differ only in **availability**. A prelude entry is always in scope because the
primitive-signature closure needs it or because source must reach one canonical
internally provided identity. A package is imported. "Always there" ≠
"irreducible" — a prelude entry is a definition the kernel re-checks, not a
trust-root assumption.

## 3. The built-in set — the surface TCB (irreducible)

Exactly these are primitively provided; each is irreducible (there is no more
primitive Ken to define it from):

- **Primitive types + the literal affordance** — the types
  `Int`/`Float`/`String`/`Bytes` (`35`, `37`) are admitted via
  `declare_primitive` (item-2, in the base once); the parser's reading of a
  literal token is base syntax. `Char` is instead the checked transparent
  refinement over `Int` (`18a §5.9`, and therefore the signature-arm prelude
  member in §4), not a surviving primitive type. Each literal *value* is a
  **primitive-constant term** of its type — computed, so **out** of
  `trusted_base()` (§6; the current per-literal `declare_postulate` is the
  highest-volume hygiene item). The type
  is irreducible; nothing is more primitive than the machine representation the
  primitive ops compute on.
- **Audited primitive operations** (`../10-kernel/14 §5`) — machine
  arithmetic/comparison and the `String`/`Bytes` primitives, each a
  `Decl::Primitive` whose registered `Op` symbol dispatches runtime evaluation
  on values. These bottom out in the interpreter's audited `prim_reduce`
  surface and are **not** Ken-definable. They remain opaque to kernel conversion
  until K3; an operation equation over literals therefore needs a visible proof
  assumption rather than `Refl` today.
- **The effect / FFI boundary** — `foreign` and the base `IO`/effect primitive
  (`[Console]`/`[FS]` etc., `38`, L5/L7). I/O cannot be pure Ken; the boundary
  is a listed assumption.
- **Base elaborator syntax** — the language forms themselves: λ, application,
  `let`, `match`, annotation, `data`/`view`/`instance`, refinement types, the
  **operator-infix + fixity** affordance, **`if`-sugar**, and the **minimal
  `module`/`import`** (F3a). These are how source becomes core; they are not
  values one could define.

Everything else on the everyday surface is one of the two re-checked tiers
below.

## 4. The prelude tier — Ken-defined, always-present, closed

Some Ken-definable types must be present before ordinary source-unit
resolution. There are two reasons. A built-in primitive may name the type in
its signature: comparison primitives have type `Int → Int → Bool`, so `Bool`
must already exist even though it is ordinary `data Bool = True | False` Ken
(§6, F1). Or the implementation may already have installed one canonical,
kernel-checked identity that the surface contract requires source to name and
that a source declaration cannot recreate. That identity may originate at the
kernel boundary or in the compiler bootstrap: both are language internals that
the prelude bridges to the surface. This is the surface analog of the kernel's
`Top`/`Bottom`/`tt` prelude (`64 §1`): fixed Ken vocabulary excluded from
`trusted_base()` yet always present in a closed set.

> **Prelude membership rule (normative, checkable).** A Ken-defined,
> source-resolved type is in the prelude **iff** at least one of these arms has a
> witness:
>
> 1. **Signature arm.** A built-in primitive's type signature names the type.
> 2. **Internal-provision arm.** Before source-unit elaboration, the
>    implementation has installed one canonical identity through ordinary
>    kernel checking. The witness records whether the identity originates at
>    the kernel boundary or in the compiler bootstrap. The surface contract
>    independently requires programs to name that exact identity, and a source
>    declaration with the same structure would allocate a distinct `GlobalId`
>    rather than reproduce it.
>
> The prelude is the **closed union** of the two witnessed inventories, never a
> fallback to arbitrary compiler globals. Presence in the compiler's global map
> is not an internal-provision witness. Each member has an explicit internal
> origin and exact pre-source identity; a missing or substituted identity fails
> closed. A signature-arm type that no primitive signature names is bloat. An
> internal-provision type with no independent source-reachability requirement,
> or whose identity source can reproduce, is likewise bloat. A missing witnessed
> member is a gap. Kernel syntax and native formers remain built-ins and are
> referenced directly rather than duplicated as prelude bindings.
>
> A companion operation joins an internal-provision member's floor-binding
> closure only when that member's surface contract names it, its checked type is
> keyed to the member's exact `GlobalId`, and floor installation reuses the
> companion's exact pre-source `GlobalId`. This is a closed companion inventory,
> not authority to expose arbitrary compiler helpers.

The prelude is a **second minimality target** — the same TB-Sound discipline
(`is_prelude` is exactly `{Top, Bottom, tt}`, no catch-all) applied at the
surface. The signature inventory is obtained by traversing the type of every
built-in primitive declaration and collecting its Ken-defined type identities;
it is not a census of selected registration helpers. That inventory is exactly
**`{Auth, Bool, Char, List, Option, ResourceKind, Result, Utf8Error}`**:

- comparisons name `Bool`, and the `String ↔ List Char` operations name `List`
  and `Char`;
- `bytes_at`/`bytes_slice` name `Option`, while `bytes_decode` names `Result`
  and `Utf8Error`;
- the opaque primitive former `Cap : Auth → Type` names `Auth`, and
  `Resource : ResourceKind → Type` names `ResourceKind`.

The internal-provision arm adds exactly `{Nat, Pair}`. This is one general arm
over internal origin, not a Pair-specific exception or third membership route.
`Nat` is the kernel-origin member: the ordinary checked inductive
`data Nat = Zero | Suc Nat` that source must use as the canonical natural/index
carrier. `Pair` is the compiler-bootstrap member: one checked transparent type
identity whose surface meaning is the non-dependent kernel Sigma (`34`). Thus
the Ken-defined **type floor** is the closed ten-member set
**`{Auth, Bool, Char, List, Nat, Option, Pair, ResourceKind, Result,
Utf8Error}`**.

The type count is ten, not twelve or thirteen. `Pair`'s floor-binding closure
also contains the exact three companions `{mk_pair, pair_fst, pair_snd}`. They
are operations, not type members. Their checked types reference the canonical
`Pair` identity, and their bodies use the kernel pair-introduction and
projection formers. The four-name surface
`{Pair, mk_pair, pair_fst, pair_snd}` reuses the compiler-installed transparent
`GlobalId`s; floor installation declares nothing, allocates no identity, and
adds no `trusted_base()` entry. Kernel `Sigma`/`Pair`/`Proj1`/`Proj2` remain
representation and computation authority, not provider declarations or another
identity family.

For an inductive floor member, a constructor enters only when its
kernel-recorded parent is that exact member; `Char` and transparent `Pair` are
constructor-free. A same-shaped source family or definition has a different
identity and is not the floor member. Every floor type, constructor, and
companion is re-checked and **out** of `trusted_base()`.

`Ordering` is **not** prelude — no built-in primitive returns it (comparisons
return `Bool`, and 3-way `compare` is an `Ord` **class method**, a package, F2),
and it has no internal-provision witness. It is therefore a standard-package
type; adding a `compare : A → A → Ordering` primitive *would* make it prelude,
but that enlarges the built-in set for no minimality gain and is **not** taken.
The derivation-path table (`../../conformance/surface/taxonomy/`) pins the exact
closed inventories and flags any over-inclusion as bloat (§6, `OrdResult`).

**Implementation staging.** The specification fixes the ten-type and
three-companion target. Until the floor-realization build captures and admits
the four existing Pair-family identities, current Strict loading may still
reject their bare names. That implementation gap is not a package boundary and
does not authorize a second identity or fallback route.

## 5. The standard-package tier — the dissolved stdlib

Everything Ken-definable that **neither** prelude arm admits is a **standard
package**: optional, explicitly imported, ordinary Ken with its **derivation
path from the built-ins stated in-spec**. `Nat` and `Pair` are therefore not
package carriers: they are the kernel-origin and compiler-bootstrap members of
the internal-provision arm. `Option` and `Result` are likewise not packages:
public primitive signatures name their canonical compiler-installed
identities. `Unit`, `Empty`, and `Either` remain packages. A same-shaped source
definition of `Pair` allocates a distinct identity; it neither replaces the
floor family nor converts structural equality into floor provenance.

The reframed catalog is
`../50-stdlib/README.md` — the lawful classes (`Num`/`Ord`/`Eq`/`Monoid`/
`Functor`/`Monad`/`Foldable`), the collection combinators
(`map`/`filter`/`fold`/ `range`), and formatting (`show`/`split`/`join`/`pad`).
The monolithic **L8 stdlib dissolves** into this catalog
(`docs/program/wp/L8-stdlib-core.md` superseded); its "laws are **proved, not
postulated**" discipline carries to the package builds (ES4). Pair's admission
under the general internal-provision rule is part of the prelude boundary above,
not a package exception.

**The derivation-path discipline (normative).** Every catalog entry states a
real Ken definition path from the built-ins. A catalog entry with **no** path is
a spec bug — a **hidden built-in** — caught jointly with CV's table; a built-in
with a path is **bloat** and must be reclassified. This is the surface
TB-Complete net made a documentation obligation.

## 6. Classification rulings — making the invariant true on the real code

The current `crates/ken-elaborator/src/prelude.rs` violates the invariant: it
carries **derivable** entries as postulates (needless `trusted_base()` entries)
and **mis-classifies** two runtime types. This chapter specifies the
corrections; ES2 implements the `prelude.rs` demotion. Each is a
`trusted_base()`-shrinking move that makes the built-in set and the trusted base
coincide.

- **`Equal` → delete; use the kernel's native `Eq`.** The prelude postulates
  `Equal : Π(A). A → A → Ω`, which **shadows the kernel's computing `Eq`**
  (`../10-kernel/16 §2` — a real former with `refl`/`J`, reduced by recursion).
  An assumed equality over the real one is not merely a phantom `trusted_base()`
  entry — it **forfeits `Eq`'s computational and `J`-elimination behavior**. The
  ruling is **delete and reference `Eq`**, not re-define. (A postulate
  duplicating a real kernel construct is the surface form of the
  name-shadows-the-mechanism trap.)
- **`And` → the derived connective, not a postulate.** Ω-conjunction is already
  a **derived operation** from the K1 formers (`16 §1.3`). The prelude postulate
  is redundant; reference the derived connective (or
  `data And (A B : Ω) : Ω = conj A B`, which lands in Ω by the
  both-components-keyed `sort_sigma`, `13 §4`).
- **`is_sorted` / `Perm` → definitions (see `37 §6`).** These are **not**
  prelude: no primitive signature names them, and neither has an independent
  internal-provision witness. They are the verified-`sort` showcase's
  predicates, and they **must be definitions**, specified in `37 §6` (§below).
  As postulates the flagship proof proves nothing.
- **opaque `Bool` → `data Bool = True | False`** (F1). The current opaque
  primitive `Bool` is not matchable, which forced the `OrdResult` branch
  workaround (`prelude.rs`); as a 2-constructor inductive it is derivable,
  matchable, and `if` is `match` sugar (`34`, `42 §2`). Removes a built-in and
  the workaround.
- **`Map` / `Set` → proved inductives, out of `trusted_base()` (OQ-A
  supersession).** These were originally slated to *stay* in `trusted_base()` as
  **audited runtime primitives** (`declare_primitive` OpaqueType, item-2, like
  `String`/`Bytes`) — correct *given* the premise that they must be the O(1)
  content-addressed, insertion-order-independent heap form (`41 §3a`). Operator
  decision **OQ-A** (2026-07-03) **changes that premise**: it chooses a
  **proved, pure, `Ord k`-keyed program-level tree** (`../50-stdlib/52-map.md`)
  over the runtime-O(1) heap form — accepting O(log n) and extensional identity
  to gain real proofs and zero trust. So `Map`/`Set` become **ordinary inductive
  `data` + `view` defs, re-checked, out of `trusted_base()`** — the opaque
  `Map`/`Set` primitive is **retired**, shrinking `trusted_base()` (a
  net-negative delta; "proved" *requires* a transparent carrier, since an opaque
  primitive has no eliminator and its laws could only be `Axiom`). The ordinary
  transport boundary preserves extensional equality and ordered `to_list`
  through round-trip. Internal bytes, hashes, and deduplication outcomes are
  not observable. This runtime specification defines no Map-specific codec; a
  future portable serialization would be a separate ordinary-package Ken WP,
  out of `trusted_base()`. The
  content-addressed heap form is **parked** as a possible later fast-map
  (the "HAMT-later" analog, `37 §3.2`), proved if it lands. (The item-2/item-3
  audited-vs-assumed accounting distinction below still governs the entries that
  genuinely *stay* runtime, e.g. `Array`/`String`/`Bytes`.)

- **`OrdResult` → remove (bloat).** `data OrdResult = Lt | Eq | Gt`
  (`prelude.rs`) sits in the prelude but **no primitive signature names it**
  (the comparisons return `Bool`) and it has no independent internal-provision
  witness, so by the membership rule (§4) it is a **bloat vector**. It exists
  only as a workaround for the opaque `Bool` (not
  matchable); F1's `data Bool` obviates it. Remove it; where a 3-way result is
  wanted, the `Ord` package's `Ordering` (§4/§5) is it.
- **`reg_novf` — split the predicate from the per-operation obligations (only
  one is bloat; Architect pre-flag).** Two distinct things live under the
  `numbers.rs` no-overflow registration (`OQ-1a`), and they must not blur:
  - The no-overflow **predicate** (`Fits`/`inBounds : Int → Ω`, the decidable
    bound `MIN_w ≤ (a +_ℤ b) ≤ MAX_w` over the **unbounded** `Int` primitive) is
    **derivable** → a **definition**, out of `trusted_base()`. The prover / SMT
    bitvector theory is the **discharge engine** for the *defined* predicate
    (re-checked, oracle-not-authority, G3), not a trusted prover-theory atom.
  - The L1 **per-operation "no silent wrap" obligations** (the
    `declare_postulate` goal each fixed-width op emits, awaiting prover
    discharge) are **legitimate live obligation-holes**, **not**
    derivable-postulate bloat: making *these* definitions would be circular or
    **eliminate the overflow net**. An undischarged obligation is an honest
    typed hole in `trusted_base_delta` (`21 §5.2`, the four-way `unknown`
    status) — it **stays** until discharged.

  So `trusted_base()` legitimately holds **genuine irreducibles + live
  obligation-holes**; the demotion target is the **predicate only**, never the
  per-op obligation — the one place "derivable bloat" and "live obligation" must
  not be conflated.

**The rulings above are worked examples; CV's derivation-path table
(`../../conformance/surface/taxonomy/`) is the exhaustive net.** Every
`trusted_base()` entry it surfaces is classified by the **same two rules**: a
**derivable** entry demotes `postulate → definition` (out of the base); a
**genuinely runtime** entry that is *audited* (computed), not *assumed*,
re-classes `item-3 postulate → item-2 primitive` (stays, correctly described).
The highest-volume case is **literals**: today each numeric/string literal in a
program is a per-program `declare_postulate` (`elab.rs`) — an **assumed value
for a computed constant**. A literal is a **primitive-constant term** of its
built-in type (§3), computed, so it belongs **out** of `trusted_base()` entirely
(the base lists the `Int`/`String` *primitive type* once, item-2, never each
literal value). Whatever the table rules, ES2 lands it.

**Net.** Of the surface soundness entries,
`Equal`/`And`/`is_sorted`/`Perm`/`Bool`/`OrdResult` and the `reg_novf`
**predicate** become re-checked definitions or are removed (**out** of
`trusted_base()`), literals become primitive-constant terms (out), and
`Map`/`Set` — under OQ-A — become **proved package inductives, retired from
`trusted_base()`** (out; `../50-stdlib/52-map.md`), the opaque primitive gone.
The **assumed-axiom** surface trusted base shrinks toward **zero** — leaving
only the genuinely-audited primitives (`Array`/`String`/`Bytes`) **and the live
proof obligations** (e.g. the per-op no-silent-wrap holes, honest until
discharged) — and the verified-`sort` showcase (§37 §6) becomes a real proof.
