# Packages and Provenance

Chapters [01](01-anatomy.md)–[04](04-effects-capabilities-and-authority.md)
taught you to read a declaration's shape, its contract, its assurance class,
and the authority it requires — all from the fragment sitting in front of
you. This chapter asks where the fragment's own pieces came from: what does
it stand on, where is it filed, and how would it reach another package's
declarations if it needed to? As with chapter 04's authority half, this is
the chapter most tempted to overclaim its own evidence — provenance is
exactly the kind of claim that is easy to state confidently from spec prose
and hard to back with a checked instance. Every claim below says
plainly which of those two it is.

## Package Paths

Catalog package files follow
`<Section>/<Domain>/[<Subdomain>/]<Pkg>.ken.md`,
and a package's dotted import path is the *same* path spelled with dots
instead of slashes — a mechanical, total identity: an `N`-component dotted
path names the unique leaf file reached through `N − 1` directories
([catalog](../../../catalog/packages/README.md), and
[§13](../../../docs/program/07-catalog-style-guide.md#13-path--import--the-normative-rule)).
The [EmptyDec fragment](../../../catalog/packages/Core/Logic/EmptyDec.ken.md)
— the fragment chapter
[03](03-assurance-and-trust.md) grounds `proved` in — sits at
`Core/Logic/EmptyDec`, so its import path is `Core.Logic.EmptyDec`, and no
lookup table is needed to go from one spelling to the other. A file is an
implicit module named by its own path — there is no in-file `module
Core.Logic.EmptyDec` header to check against, the directory structure *is*
the declaration
([§3.1](../../../spec/30-surface/33-declarations.md#31-declaring-modules)).

## Derivation and Tiers

Chapter [01](01-anatomy.md) pointed you at a selected fragment's Trust &
derivation section without asking what "derivation" meant there. Read it
for what a fragment stands on. The
[sums combinators](../../../catalog/packages/Data/Sums/Combinators.ken.md)
states it plainly: "`Either` is a checked `data` inductive (kernel-admitted
by positivity) — the only new carrier this package introduces, and
`Option`/`Result` are reused from the prelude unchanged." That sentence
names two different kinds of inheritance in one breath — a brand-new
inductive this file itself declares, and two carriers it takes, unchanged,
from elsewhere. `Tooling/Testing/Property.ken.md` states the same shape
about itself: its implementation "derives from the prelude's `List`,
`Result`, `Unit`, `Nat`, `Bytes`, and `UInt8` values" — every one of those
names is inherited, none re-declared. Both entries also state, in their own
words, that this reuse costs nothing new: Combinators' own "`trusted_base()`
delta: zero" and Property's own "The declared `trusted_base()` delta is
**zero**" are each a literal, unqualified zero. Read this against chapter
[03](03-assurance-and-trust.md), precisely: not every fragment's zero is
the *same* zero. `Core/Logic/EmptyDec.ken.md` reports something narrower —
"**Zero new trust category**" — and immediately qualifies it: "Instantiating
`dec_eq_decides` at a carrier whose `DecEq` instance has an audited
assumption retains that instance's declared delta." That is not a literal
zero, and it is a statement that *this entry itself* adds no new postulate or
primitive, while explicitly preserving whatever delta a caller's own choice
of carrier already carries. Provenance, read this way, is not a vague
pedigree claim, and it is not always the same claim twice — it is each
fragment's own stated boundary between "reused, already accounted for" and
"new here," and that boundary can be a flat zero or a conditional one
depending on what the fragment says.

The built-ins/prelude/package distinction those fragments each
draw on is itself a specified, closed structure, not an informal habit. A
type is **built-in** only if no more primitive Ken could define it — the
surface's own irreducible floor, and the built-in set explicitly names
`Int`/`Float`/`Char`/`String`/`Bytes` as the primitively-provided types
([§2](../../../spec/30-surface/30-taxonomy.md#2-the-three-tiers),
[§3](../../../spec/30-surface/30-taxonomy.md#3-the-built-in-set--the-surface-tcb-irreducible)).
A type is **prelude** only if a built-in primitive's own type signature
names it and it is not already provided by the kernel — a closed set the
taxonomy pins as exactly `{Bool, Char, List}`
([§4](../../../spec/30-surface/30-taxonomy.md#4-the-prelude-tier--ken-defined-always-present-closed)).
Everything else Ken-definable, with no primitive forcing its presence, is a
**standard package**: optional, explicitly imported, with its derivation
path from the built-ins stated in-spec
([§5](../../../spec/30-surface/30-taxonomy.md#5-the-standard-package-tier--the-dissolved-stdlib)).

Read those two fragments against this taxonomy carefully, rather than
taking their own wording as the classification: Property's prose calls
`List`, `Result`, `Unit`, `Nat`, `Bytes`, and `UInt8` all "the prelude's" —
but that is the fragment's own informal shorthand for "the ambient
environment," not the taxonomy's technical membership test. Checked
against the closed set above, `List` genuinely is prelude, and `Bytes` is
**built-in**, not prelude — it is one of the five primitively-provided
types named in §3. `Result`, `Unit`, `Nat`, and `UInt8` are not named in
either closed list this taxonomy chapter states, so this page does not
classify them, and that is a gap in what the cited spec section pins,
not a fact this chapter can derive by itself. Read a fragment's own
vocabulary as its author's
convenience, and check any technical-sounding word — "prelude" included —
against the section of the normative source that defines it
before repeating it as fact.

## Imports

`import`, `module`, and `export` are supported, checked constructs.
`import M` brings
`M`'s exported names into scope under `M.foo`, and `import M as N` aliases the
qualifier, and `import M (foo, Bar)` brings exactly those names in unqualified,
each optionally renamed
([§3.2](../../../spec/30-surface/33-declarations.md#32-importing-and-exporting)).
A `program` header may declare which packages it `admits` and which
authority it `capabilities`, and these are two independent manifests, one for
instance-dictionary coherence, one for effect authority — a change to one
cannot alter the other
([§3.2.1](../../../spec/30-surface/33-declarations.md#321-admission-boundary-headers)).
A colliding unqualified name — between a local definition, an import, and
the prelude — is rejected as `AmbiguousReference` before any expression
references it, and that check is order-independent and fail-closed
([§3.3](../../../spec/30-surface/33-declarations.md#33-name-resolution-surface-only-never-reaches-the-kernel)).
And none of it costs anything: a module/import program and its fully
flattened, single-namespace equivalent elaborate to the identical
`trusted_base()` and the identical declaration count — modules, imports,
and visibility are surface-only, adding no kernel feature
([module acceptance](../../../crates/ken-elaborator/tests/es3_modules_acceptance.rs),
`module_elaborates_to_identical_flat_sigma`).

This is not merely a single-file mechanism: the loader that follows an
`import` edge across two **separate files on disk** is supported and tested.
An acceptance test writes an `A` file that `import`s a `B` file
as a genuine, separate compilation unit, and elaborating `A` through the
elaborator's `elaborate_module_from_roots` resolves `A`'s reference to
`B`'s declaration — lazily (an unrelated third file with invalid
syntax is never touched, because nothing imports it) and with a cache, so
loading the same module twice reuses the first result rather than
re-elaborating it
([§3.2](../../../spec/30-surface/33-declarations.md#32-importing-and-exporting), and
[loader acceptance](../../../crates/ken-elaborator/tests/n2_in_repo_loader.rs),
`cross_file_import_resolves_lazily_through_plural_root_api_and_caches`).
Cross-file import is not an unimplemented language feature.

Catalog package code has live `import` and `export` declarations.
`Core/Logic/EmptyDec.ken.md` imports and uses `sym` and `trans` from
`Core/Logic/Transport.ken` in `dec_eq_decides`. It also imports class helpers
and exports its declarations.

For catalog-addressed paths, `ken check` finds the module root with
`catalog_module_from_path`, loads its import closure through
`elaborate_module_from_roots_v1`, and checks the entry's own fences
(`crates/ken-cli/src/main.rs`). Catalog `ken check` therefore follows
cross-file imports. It is not the isolated-file route used for other paths.

You can now derive a package's import name from its path, separate inherited
definitions from new trust, and check informal tier labels against the
taxonomy.

---

**Sources:**
[package catalog](../../../catalog/packages/README.md), and
[path rule §13](../../../docs/program/07-catalog-style-guide.md#13-path--import--the-normative-rule), and
[taxonomy §§2–5](../../../spec/30-surface/30-taxonomy.md#2-the-three-tiers), and
[module declarations §§3.1–3.3](../../../spec/30-surface/33-declarations.md#31-declaring-modules), and
[module acceptance](../../../crates/ken-elaborator/tests/es3_modules_acceptance.rs), and
[loader acceptance](../../../crates/ken-elaborator/tests/n2_in_repo_loader.rs), and
[catalog check path](../../../crates/ken-cli/src/main.rs), and
[registered fragments](fragments.md).
The path rule establishes module naming; the CLI path and loader acceptance
establish how catalog `ken check` follows imports.
