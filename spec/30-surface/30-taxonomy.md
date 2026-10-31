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

The **built-in name set B** for resolution and binding protection (`33 §3.3`)
spans two trust tiers: the irreducible kernel/built-in vocabulary of §3 and the
closed, re-checked prelude of §4. A name being permanently available and
unshadowable does **not** put its Ken-defined declaration in `trusted_base()`;
the invariant above applies to the irreducible tier, not to every name in B.
Keeping the prelude minimal limits the names authors cannot bind.

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
  surface and are **not** Ken-definable. Registration alone does not make an
  operation compute in kernel conversion. The specified exceptions are
  `leq_int` on two WHNF `IntLit` operands (`../10-kernel/16 §2.2`) and the
  checked-literal `string_to_list_char` view (`../10-kernel/17 §1`); all other
  registered Ops remain opaque until K3. An equation over a neutral Op result
  needs a visible proof assumption rather than computation by `Refl`.
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
`trusted_base()` yet always present in a closed set. The
internal-provision arm also admits a checked operation when an exact-ID
native reader and independent source-name need both hold; it is not
limited to inductive type identities.

> **Prelude membership rule (normative, checkable).** A Ken-defined,
> source-resolved identity is in the prelude **iff** at least one of these
> arms has a witness:
>
> 1. **Signature arm.** A built-in primitive's type signature names the type.
> 2. **Internal-provision arm.** Before source-unit elaboration, the
>    implementation has installed one canonical identity through ordinary
>    kernel checking, and a **native mechanism is keyed to that exact
>    identity** — the kernel, the elaborator's exact-ID reader, the effect
>    reifier, erasure/lowering, or the host runner — so that a source
>    declaration of the same structure allocates a distinct `GlobalId`
>    that **fails to interoperate** with that mechanism
>    rather than reproducing the identity. The witness records which mechanism
>    does the keying and whether the origin is the kernel boundary or the
>    compiler bootstrap. The surface contract independently requires programs to
>    name that exact identity — even at a single boundary, such as the Program-I
>    entrypoint. The two are a conjunction: a bootstrap-installed identity that
>    no mechanism is keyed to (a source equivalent would serve) is bloat, and so
>    is a machinery-keyed identity that no surface contract makes source name.
>
> The prelude is the **closed union** of the two witnessed inventories, never a
> fallback to arbitrary compiler globals. Presence in the compiler's global map
> is not an internal-provision witness. Each member has an explicit internal
> origin and exact pre-source identity; a missing or substituted identity fails
> closed. A signature-arm type that no primitive signature names is bloat. An
> internal-provision identity with no independent source-reachability
> requirement, or whose identity source can reproduce, is likewise
> bloat. A missing witnessed member is a gap. Kernel syntax and native
> formers remain built-ins and are referenced directly rather than
> duplicated as prelude bindings.
>
> A companion operation joins an internal-provision member's floor-binding
> closure only when that member's surface contract names it, its checked type is
> keyed to the member's exact `GlobalId`, and floor installation reuses the
> companion's exact pre-source `GlobalId`. This is a closed companion inventory,
> not authority to expose arbitrary compiler helpers.

**Reserved-name specificity (normative for every internal-provision
admission).** Because authors cannot bind a name in B anywhere, a new
reserved spelling must closely enclose the identity's meaning, not consume a
generic name for a narrow intrinsic. A generic constructor can be made
specific by type qualification; a generic name not covered by that route
needs a rename before admission. Spec owns public spelling in `35`/`38`
and the [ledger](30-intrinsic-ledger.md); any rename lands atomically with
the floor identity, roster/count, catalog callers and fixtures.
`NotFound`, `Other` and `Unsupported` are admitted only as
`IOError.C` paths, with no bare reservation or rename. `write` is a
package name, not a reserved floor spelling; package qualification
resolves its specificity probe. The Console carrier is source-renamed
from generic `Stream` to `ConsoleStream`; its `Stdout` case resolves
only as `ConsoleStream.Stdout`, not as a bare reservation.
`Instant` passes specificity as a type with a scoped `Instant.MkInstant`
constructor, not a bare `MkInstant` reservation.
`Success` and `Failure` have passed it as the qualified paths
`ExitCode.Success` and `ExitCode.Failure`; neither bare spelling joins B.
A per-type scoped-constructors choice supplies specificity
structurally: `IOError.NotFound` is unambiguous without reserving
bare `NotFound`. Type qualification is supported in expressions
and patterns for every data type (`34 §1`), even when its constructors
remain bare. The property changes only source visibility and the B
inventory, never a constructor's checked identity or the number of floor
identities.

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

The internal-provision arm adds `{Nat, Pair}` and the five kernel-machinery-keyed
proposition and equality members `{Equal, Prop, Proved, Top, Bottom}`. This is
one general arm over internal origin, not a Pair-specific exception or third
membership route. `Nat` is the kernel-origin member: the ordinary checked
inductive `data Nat = Zero | Suc Nat` that source must use as the canonical
natural/index carrier. `Pair` is the compiler-bootstrap member: one checked
transparent type identity whose surface meaning is the non-dependent kernel
Sigma (`34`). The five proposition/equality members are each keyed to the
kernel's own machinery, so a source-defined equivalent fails to interoperate
(`../10-kernel/16 §1`): `Top` and `Bottom` are the kernel's `Ω₀` truth and
falsity prelude constants and `Proved : Top` their canonical proof; `Equal` is
the surface-nameable definition whose body **is** the kernel's computing
observational equality `Eq` (a source `data Equal` forfeits `Eq`'s `J`/`refl`
computation); `Prop` is the surface-nameable alias for the proposition sort
`Ω₀` (its own kernel type is `Type (suc 0)`, its body is `Ω₀`), present only
because the surface grammar has no `Ω` token and used only in type-annotation
position. Each is a kernel constant or a re-checked definition — **out** of
`trusted_base()` exactly as the original ten, adding no entry.

The entrypoint-ABI contract (`33 §3.2.1`) adds five checked, pre-source
identities under the same internal-provision arm: `ExitCode`,
`ProcessInput`, `ProgramCaps`, `HostIO`, and `Prod`. The admission check
constructs its expected `main` type from those exact identities;
`ProcessInput`'s environment field contains `Prod Bytes Bytes`. A
same-shaped source declaration cannot replace them. `HostIO` is a
**type-level function** of the authority and result type, not an inductive
family with a constructor. `Prod` is retained as a distinct inductive
family, not retargeted to the existing `Pair`: its `MkProd` constructor
is selected by native environment-pair consumers, whereas `Pair` is a
kernel Sigma with no constructor ID. Retargeting that representation is
outside this admission.

Three effect/clock source-required checked identities also satisfy the
internal-provision arm: `Unit` is the canonical effect-response result named
by `36 §1.1` and §2.1, with its exact pre-source type ID captured in
`PreludeEnv.unit_id` and its constructor keyed by the effect runtime;
`Coproduct` is the canonical signature sum required by `36 §2.3` and
§4.5.7, with its exact pre-source type ID in `PreludeEnv.coproduct_id`
and its injection IDs selected by native composition;
`Instant` is the source-named clock result in `36 §1.1`, with its
pre-source `MkInstant` constructor ID selected by the native clock response
and its exact `Instant` parent. A same-shaped source declaration of any of
the three cannot replace the checked identity. The three type names pass
the specificity rule: the generic injections and clock constructor are
qualified as `Coproduct.InL`, `Coproduct.InR`, and
`Instant.MkInstant`; `Unit` keeps its bare `MkUnit` constructor.

`ITree` satisfies the same internal-provision conjunction. The
compiler installs one kernel-checked inductive before source resolution
and captures its `PreludeEnv.itree_id` with the checked `Ret` and `Vis`
constructor IDs. Effect lowering and the host interpreter select that
family and its constructors by exact identity; a same-shaped source
`ITree` cannot substitute for the installed family. The effect surface
independently requires programs to name `ITree` and construct its
`Ret`/`Vis` nodes (`36 §2.1`, §2.4). The source-required type enters the
floor under its existing spelling, not as a primitive or a new kind of
membership witness. Its complete constructor family is scoped as
`ITree.Ret` and `ITree.Vis`: the unchanged checked constructor IDs
remain reachable, while bare `Ret` and `Vis` are neither floor names
nor reserved spellings. Other source declarations may bind those bare
names under the ordinary non-B clash and visibility rules (`33 §3.3`).

Six FS/resource families also satisfy the internal-provision conjunction.
Native code produces or reads their exact checked constructor IDs, which
fix their parent type identities; independently, `38 §1.3.1` and
`§1.7.1–2` require source to name those types. `FileError` carries public
file-operation failures; `IOError` carries their thirteen distinct causes,
including `Revoked`; and `CreatePolicy` selects the `writeFile` creation
mode. `BufferSpan` crosses the bounded transfer operations, `TransferCount`
is carried by read/write progress, and `ResourceError` carries distinct
resource failures. A same-shaped source type cannot replace any of these
six families. None is a new primitive or a trusted-base entry. Native
`CreatePolicy` suffix readers still require the unchanged artifact leaf
`CreatePolicy::CreateNew` (and its sibling policy leaves); constructor
scoping changes source paths, not these checked identities or leaves.

The Console stream carrier and the character comparator meet the same
internal-provision conjunction. The host interpreter selects the exact
checked `Stdin`, `Stdout`, and `Stderr` constructor IDs and their parent;
native lowering reads their pre-source stable-symbol leaves from an
operand typed by the canonical Console carrier. Source must name that
carrier and its selectors for console input and output. The generic
source spelling `Stream` conflicts with the distinct §37 library
`Stream`, so the admitted source type is `ConsoleStream`, with the
complete constructor family available only as `ConsoleStream.Stdin`,
`ConsoleStream.Stdout`, and `ConsoleStream.Stderr`. The native artifact
leaves retain their pre-source `Stream::C` spelling and exact checked
IDs; this source rename does not allocate another family.
`eqChar` is a checked comparator operation, not a type: the Char
`==` plan and character literal matching select its registered exact
ID, while §34's literal equality and §37's character operations
require the canonical comparator to be source-nameable. String
literal patterns must read that same registered ID rather than a
mutable spelling lookup. The candidate rekeys that reader, so a
same-spelling source declaration cannot change its comparator.

Thus the closed **floor-identity roster** has thirty-two names:
**`{Auth, Bool, Bottom, BufferSpan, Char, ConsoleStream, Coproduct,
CreatePolicy, Equal, ExitCode, FileError, HostIO, IOError, ITree,
Instant, List, Nat, Option, Pair, ProcessInput, Prod, ProgramCaps,
Prop, Proved, ResourceError, ResourceKind, Result, Top, TransferCount,
Unit, Utf8Error, eqChar}`**. The earlier thirty identities plus
`ConsoleStream` and `eqChar` remain re-checked; floor admission alone
adds no postulate or `trusted_base()` entry. The three scoped Console
constructors and the Pair companions are separate from this 32-name
count. These identities are not all types: `Top` and `Bottom` are
proposition constants, `Proved` is a proof binding, `HostIO` a
type-level function, and `eqChar` a checked comparator operation.
The definable `And`/`Dec`/`Yes`/`No` stay **out**
(source-constructible, with a `Core/Logic` package home, and no
internal-provision witness). The `Empty` needed by `space` desugaring
is different: its identity is captured before source elaboration and read by exact ID
(`prelude.empty_id`), but no surface contract independently requires
source to name it. It belongs internally, not in the floor. The
source name `Empty` is provided by the explicit-import catalog package
`Core.Logic.EmptyDec`, not by the prelude; its checked identity
is distinct from the compiler's. `Pair`'s floor-binding closure contains the
exact three companions `{mk_pair, pair_fst, pair_snd}`. They are operations,
not type members. Their checked types reference the canonical
`Pair` identity, and their bodies use the kernel pair-introduction and
projection formers. The four-name surface
`{Pair, mk_pair, pair_fst, pair_snd}` reuses the compiler-installed transparent
`GlobalId`s; floor installation declares nothing, allocates no identity, and
adds no `trusted_base()` entry. Kernel `Sigma`/`Pair`/`Proj1`/`Proj2` remain
representation and computation authority, not provider declarations or another
identity family.

For an inductive floor member, constructor names are governed **per type** by
the scoped-constructors property (`33 §3.3`, `34 §1`). An unscoped floor type
reserves its whole kernel-recorded constructor family as bare names in B; a
scoped floor type reserves **only the type name**, and its constructors are
reachable solely as `T.C`, not reserved as bare spellings. Neither mode admits
a partial constructor family, and each constructor's parent remains the exact
floor identity. Abstract or constructor-private visibility still applies
(`33 §4.2`); qualification cannot expose a hidden constructor. The existing
`ResourceKind` floor is scoped: `ResourceKind.FsHandle`,
`ResourceKind.Buffer`, `ResourceKind.Mapping`, `ResourceKind.SpawnPlan`,
`ResourceKind.ChildProcess`, `ResourceKind.PipeRead`, and
`ResourceKind.PipeWrite` are the seven public constructor paths, not
additional bare names in B. `ExitCode` is scoped:
`ExitCode.Success` and `ExitCode.Failure` are available, not bare
`Success`/`Failure`. `ProgramCaps` is scoped: its runner-minted
`MkProgramCaps` has no bare reservation; when visible, it uses the
qualified path `ProgramCaps.MkProgramCaps`. Its existing visibility is
unchanged. `Coproduct` and `Instant` are scoped:
`Coproduct.InL`, `Coproduct.InR`, and `Instant.MkInstant` resolve to
their exact checked constructor IDs, whereas bare `InL`, `InR` and
`MkInstant` do not enter B. `ITree` is scoped as a complete family:
`ITree.Ret` and `ITree.Vis` retain their exact constructor IDs, while
bare `Ret` and `Vis` do not enter B and may be bound by source.
`ConsoleStream` is scoped as a complete family: only the type name
enters the counted floor, while `ConsoleStream.Stdin`,
`ConsoleStream.Stdout`, and `ConsoleStream.Stderr` reach their exact
pre-source constructor IDs. Bare `Stdin`, `Stdout`, and `Stderr` do not
enter B; the unrelated source name `Stream` is not reserved.
`FileError` is unscoped: `MkFileError` is bare and reserved. `IOError`
is scoped as the complete thirteen-cause family: each source path is
`IOError.C` (including `IOError.NotFound`, `IOError.Other`,
`IOError.Unsupported` and `IOError.Revoked`), and no bare cause enters B.
`CreatePolicy` is scoped: `CreatePolicy.CreateNew`,
`CreatePolicy.CreateOrTruncate` and `CreatePolicy.CreateOrKeep` resolve to
their unchanged checked IDs; no bare policy case enters B. `ResourceError`
is scoped as the complete twelve-constructor family: source uses
`ResourceError.C` paths, and no bare error name enters B. `BufferSpan`
and `TransferCount` are scoped; their `PrivateBufferSpan` and
`PrivateTransferCount` constructors retain their checked IDs and private
visibility, with no bare reservation or newly public qualified path.
`ProcessInput`, `Prod` and `Unit` are unscoped: their whole constructor
families, respectively `MkProcessInput`, `MkProd` and `MkUnit`, are bare
reserved names. `HostIO`, `Char`, and transparent `Pair` have no
constructor family; `eqChar` is an operation, not a family. A
same-shaped source family or definition has a different identity and
is not the floor member. Every floor identity, constructor, and
companion is re-checked and **out** of `trusted_base()`.

**Other effect-surface and Program-I identities remain candidates.** The
internal-provision arm can be applied to identities the effect reifier, erasure,
or host runner keys, but only when the independent source-naming requirement
also holds. `Unit`, `Coproduct`, `Instant` and `ITree` are admitted on exact-ID
keying and independent `36` source needs; the six FS/resource families
above are admitted on exact-ID keying and independent `38` source
needs. `ConsoleStream`'s constructor family and `eqChar` also have
separate exact-ID keying and source-need witnesses. Mere registration
does not admit any identity. A source wrapper over an ABI (for example
`Capability/Process/Exit`'s `exit_with`/`exit_from_result`) does not become
an intrinsic identity by using one. `host_exit` is keyed but has no independent
source-name requirement: it remains compiler-internal; an example using that
name does not grant it floor membership. `elaborate_space_decl` uses
`prelude.empty_id` when it builds the `space` effect and state fold
(`36 §4.1–4.2`). The compiler captures that identity before source
elaboration, and a same-program `data Empty` cannot replace the desugarer's
exact ID. Its use is internal output: no current source contract requires
programs to name that exact identity. Source-level `Empty` is provided
by the explicit-import `Core.Logic.EmptyDec` package with a distinct
identity, not by this compiler identity. A future `36` contract
requiring source to name the desugarer's exact `Empty` would be a
separately gated source-contract change. Once keying and that independent
source need both hold, the existing rule admits `Empty` to
the floor without per-name operator approval; the catalog `Empty` then retires,
with the roster/count changed atomically. This
contingency adds no present floor member.

The [open per-name intrinsic ledger](30-intrinsic-ledger.md) records candidate
names, possible keying mechanisms, missing witness arms, and the separate
spec-promised package surface. The membership rule above is the complete
criterion: **once the exact pre-source identity is proven keyed by a native
mechanism and independently required by the surface contract, that name joins
the always-present, reserved floor**. The witnessed roster and its count change
atomically with realization. The final list is reported to the operator for
information, not submitted for per-name permission. No compiler-provided
intrinsic import module exists as an alternative route: a keyed identity that
source must name cannot be made optional without surprising shadowing.

Until D0 proves both witness clauses for another candidate, the floor
remains the thirty-two-identity set above and that candidate has no new
ambient availability. `BufferWindow`, `ResourceBodyResult`,
`ResourceBracketResult`, the `FS` source alias, `read_bytes` and `write_file`
are source-promised but unkeyed: they remain outside B as package material,
not internal-only identities. In particular, `BufferWindow` is unreserved;
a same-name source declaration is allowed in strict mode, while legacy
resolution stays unchanged. `FS` as an effect/capability label does not
make the distinct type alias keyed. A keyed identity without an independent
source-name requirement stays compiler-internal; an unkeyed, promised
definition belongs in an explicit package; an unkeyed, unpromised
registration may be removed after its uses are measured. A ledger row
alone does not admit a name, and registering it in `globals` proves
neither clause. Each added checked floor member remains out
of `trusted_base()`.

`Ordering` is **not** prelude — no built-in primitive returns it (comparisons
return `Bool`, and 3-way `compare` is an `Ord` **class method**, a package, F2),
and it has no internal-provision witness. It is therefore a standard-package
type; adding a `compare : A → A → Ordering` primitive *would* make it prelude,
but that enlarges the built-in set for no minimality gain and is **not** taken.
The derivation-path table (`../../conformance/surface/taxonomy/`) pins the exact
closed inventories and flags any over-inclusion as bloat (§6, `OrdResult`).

**Implementation staging.** The specification fixes the
thirty-two-identity floor and Pair's three checked companions as the
current target. The thirty prior identities are present on this base;
this candidate adds `ConsoleStream` and `eqChar`, the complete scoped
Console constructor family, and the source-name migration atomically
with the roster/count. The five unkeyed, source-promised Console/Char
operations or aliases remain package identities: `IO`, `write`, and
`print_line` are exported at their exact pre-source IDs by
`Capability.Console.Text`; `charToInt` and `leqChar` by
`Core.Classes.LawfulClasses`. These existing homes export the captured
checked IDs, not copied definitions or synthetic substitute globals.
Catalog clients migrate to explicit provider imports on both roots routes.
Strict roots (`elaborate_module_from_roots_strict`) refuse an unimported
bare use of any of the five with `UnboundName`; importing the provider
resolves each to its exact pre-source ID. Legacy roots
(`elaborate_module_from_roots` and `_v1`) also resolve explicit imports
to those exact IDs, but an unimported bare use is neither promised nor
refused and carries no exact-ID guarantee. Until the L2-4 strict-entry
flip, standalone Legacy entries (`elaborate_file_v1`,
`elaborate_ken_md_file_v1`, `elaborate_decl`, and CLI paths outside
`catalog/packages/`) guarantee bare access to the five exact pre-source
package IDs. Examples and inline sources using those entries follow the
same route. Other bare Legacy globals may resolve, but their availability
is neither guaranteed nor refused by the surface contract. This does not
make those names floor members or expand Strict resolution. The
`ConsoleStream` rename and qualified constructor paths apply in every
mode. This slice changes neither CLI routing nor standalone roots loading
and does not include the strict-default flip.

The implementer's token-aware census classified 1,045 candidate rows
across 164 paths. It predicts 127 ambient-pair deletions, not 127
observed deletions; seven directly affected catalog roots were measured
with zero ambient names, but the full-catalog transition sentinel was
not run locally. This Spec co-turn does not establish independent QA,
official conformance, CI, publication or landed trust. No implementation
gap authorizes a second identity or fallback route.

## 5. The standard-package tier — the dissolved stdlib

Every Ken-definable identity that the specified surface makes available to
source and that **neither** prelude arm admits is a **standard package**:
optional, explicitly imported, ordinary Ken with its **derivation path from
the built-ins stated in-spec**. A keyed identity needed only by compiler
machinery is not a source-facing package entry; under §4 it stays internal.
`Nat` and `Pair` are therefore not package carriers: they are the kernel-origin
and compiler-bootstrap members of the internal-provision arm. `Option` and
`Result` are likewise not packages: public primitive signatures name their
canonical compiler-installed identities. The source-level `Empty` is provided
by the `Core.Logic.EmptyDec` package with an identity distinct from the
compiler's; `Either` is also a package. The `Empty` required by `space`
desugaring is compiler-internal, not a package export. `Unit` instead
belongs to the internal-provision floor: the source-required effect
response needs the canonical checked identity, not an optional package
lookalike. A same-shaped source definition of `Pair` allocates a
distinct identity; it neither replaces the floor family nor converts
structural equality into floor provenance.

The reframed catalog is
`../50-stdlib/README.md` — the lawful classes (`Num`/`Ord`/`Eq`/`Monoid`/
`Functor`/`Monad`/`Foldable`), the collection combinators
(`map`/`filter`/`fold`/ `range`), and formatting (`show`/`split`/`join`/`pad`).
The monolithic **L8 stdlib dissolves** into this catalog
(`docs/program/wp/L8-stdlib-core.md` superseded); its "laws are **proved, not
postulated**" discipline carries to the package builds (ES4). Pair's admission
under the general internal-provision rule is part of the prelude boundary above,
not a package exception.

A compiler-owned identity keyed by native machinery cannot be replaced by a
same-shaped ordinary package declaration. If the surface independently needs
to name that exact identity, it joins the protected prelude by §4; otherwise
it remains internal and unresolvable from source. There is no optional
intrinsic-import module. A checked helper with no identity reader remains
ordinary package material when its behavior is promised.

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

- **`Equal` → checked prelude alias of the kernel's native `Eq`, not a
  postulate.** The former prelude postulate `Equal : Π(A). A → A → Ω` shadowed
  the kernel's computing `Eq` (`../10-kernel/16 §2` — a real former with
  `refl`/`J`, reduced by recursion). An assumed equality over the real one
  forfeits `Eq`'s computational and `J`-elimination behavior and adds an
  unnecessary `trusted_base()` entry. Retire that duplicate postulate, **not**
  the source spelling: §4 admits `Equal` as the exact checked, transparent floor
  definition whose body is native `Eq`. It is re-checked and adds no trust
  entry. A postulate duplicating a real kernel construct is the surface form
  of the name-shadows-the-mechanism trap.
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

**Net.** Of the surface soundness entries, `Equal` remains as the checked
prelude floor alias, while `And`/`is_sorted`/`Perm`/`Bool`/`OrdResult` and the
`reg_novf` **predicate** become re-checked definitions or are removed (all
**out** of `trusted_base()`); literals become primitive-constant terms (out);
and `Map`/`Set` — under OQ-A — become **proved package inductives, retired from
`trusted_base()`** (out; `../50-stdlib/52-map.md`), the opaque primitive gone.
The **assumed-axiom** surface trusted base shrinks toward **zero** — leaving
only the genuinely-audited primitives (`Array`/`String`/`Bytes`) **and the live
proof obligations** (e.g. the per-op no-silent-wrap holes, honest until
discharged) — and the verified-`sort` showcase (§37 §6) becomes a real proof.
