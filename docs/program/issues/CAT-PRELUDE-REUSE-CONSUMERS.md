---
id: CAT-PRELUDE-REUSE-CONSUMERS
title: "Drain catalog-reuse census group 1 (prelude functional-floor reuse), P-provider subset only — remove three local reimplementations (Derived#map, Derived#filter, Property#gen_map_list) that shadow the ambient compiler-prelude map/filter, letting each reference fall through to the installed provider. Un-shadow, not selective import: P is [installed]/ambient, so there is NO import edge to add. Shaped on the landed CAT-BOOL/CAT-DERIVED per-package increment pattern; the source-equation-isomorphic subset of group 1, with the instance-bound and [higher]-module items deferred. Acceptance recut to a candidate-specific migration property (Architect evt_7spzy25qqdsqx on Spec evt_3z8y6pf6b6m5p): separately declared recursive heads are non-convertible, so this is not a kernel-equivalence drain."
status: active
owner: foundation
size: S
gate: none
tier: T2
depends_on: []
blocks: []
github: https://github.com/swe-toolkit/ken/pull/3146
origin: "Steward, 2026-08-30, filed on the CAT-BOOL-REUSE-CONSUMERS landing (group 6 drained, d95bc2df4) as the next L3 catalog-reuse objective. Group 1 membership quoted verbatim from docs/program/cat-reuse-census.md §4.4 item 1 at origin/main e71ddb479; the [low] consume tags read from §3 rows 36 (Derived) and 48 (Property). Scoped to the three definitionally-clean P-provider sites after a pre-frame measurement (below) found the other three group-1 members instance-bound or [higher]-module. The provider P is the compiler prelude (§3 row 63, [ambient]/[installed]) and needs NO pub-export prerequisite, so depends_on is empty. Steward-filed per COORDINATION section 2."
---

> # AMENDED — Architect ruling `evt_7spzy25qqdsqx` on the Spec ruling
> # `evt_3z8y6pf6b6m5p`, 2026-08-30. The acceptance property is RECUT; the drain
> # itself is unchanged. Foundation resumes from its clean branch under the new
> # AC once this amendment lands.
> #
> # The refuted premise: local recursive `map`/`filter`/`gen_map_list` and the
> # prelude `map`/`filter` are NOT kernel-definitionally identical. Separately
> # declared transparent RECURSIVE globals are distinct rigid heads even when
> # their source clauses/types are isomorphic and their only body difference is
> # the self `GlobalId`; Ken has no cyclic/bisimulation quotient, so their
> # conversion observable is `false` and halts. (This is why CAT-BOOL's helper
> # was sound there and is NOT here: `is_some`/`bool_and` are NON-recursive; a
> # recursive body references its own self-id, which differs between the two
> # declared globals.) So `AC-NO-EQUIVALENT-LOCAL` is REMOVED — its
> # exact-recursive-clone RED / zeta-clone RED / nonconvertible GREEN controls
> # demand the wrong verdict, and on the current kernel they drive the
> # cross-identity stack overflow the Spec erratum addresses. It is replaced by
> # `AC-RECURSIVE-UNSHADOW-MIGRATION`, a candidate-specific migration proof (not
> # a kernel-equivalence claim). The kernel conversion-totality bug is a SEPARATE
> # non-blocking follow-on framed after the Spec §17 erratum lands (Sequencing).
> #
> # D1 was HELD at `f8b2dd642` (implementer stopped before commit, branch clean)
> # pending this amendment. On landing, Foundation resumes from that branch; the
> # kernel fix need NOT land first — the drain neither invokes nor depends on
> # cross-identity conversion and removes the duplicate local recursive heads.
> # On each candidate: fresh Foundation QA + CV on the exact SHA, then Steward
> # M1-M4.

## Why this scope, not all of group 1 (pre-frame measurement, `e71ddb479`)

Census §4.4 item 1 "Prelude and functional-floor reuse" lists SIX members. This
node drains only the three whose local body is SOURCE-EQUATION ISOMORPHIC to the
ambient prelude provider — same clauses, same reduction behavior, differing only
in the self `GlobalId`. That isomorphism is what makes the un-shadow behaviorally
safe (the module's dependent declarations re-elaborate against the ambient
provider, which reduces the same way). It is NOT kernel definitional equality of
the two declared globals — separately declared recursive heads are non-convertible
(Spec `evt_3z8y6pf6b6m5p`; see the amendment banner) — so the acceptance property
is a candidate-specific migration proof, never an equivalence check over the two
recursive providers:

- `Data/Collections/Derived.ken.md#map` — local body byte-identical to prelude
  `map` (measured below); IN SCOPE.
- `Data/Collections/Derived.ken.md#filter` — local body byte-identical to prelude
  `filter`; IN SCOPE.
- `Tooling/Testing/Property.ken.md#gen_map_list` — local body byte-identical to
  prelude `map` (alpha-renamed binders only); IN SCOPE.

DEFERRED, each for a named reason — do NOT touch them here:

- `Core/Classes/LawfulFunctors.ken.md#list_map` and `#list_foldr` — these are
  BOUND INTO A FUNCTOR/FOLDABLE TYPECLASS INSTANCE with attached laws
  (`map = list_map; id_law = proof id for list_map; fusion_law = proof fusion for
  list_map`, `LawfulFunctors.ken.md:308-310`; `foldr = list_foldr; ...;
  foldr_to_list = list_foldr_to_list`, `:428-431`). Draining requires rebinding
  the instance field to the ambient provider and re-establishing the attached
  laws about it — an instance-dictionary move, not a private-helper deletion. AND
  the LawfulFunctors module is itself `[higher]` (§3 row 70: standalone rejects
  `UnresolvedCon list_append` at `4204..4215`), so it is fixture-backed like Map.
  Two independent reasons to hold it for a separate, measured node.
- `Core/Classes/EffectfulClasses.ken.md#compose` — its provider is `LF.comp`
  (census row for module 28), and LF (`Core.Classes.LawfulFunctors`) is a
  `[higher]` module needing its own standalone/ownership prerequisite first.

The measured source-equation isomorphism (why the three are safe to un-shadow —
NOT a claim of kernel definitional equality between the two declared globals):

```
Derived fn map (:139)              prelude map (prelude.rs:486)
  match xs {                         match xs {
    Nil ↦ Nil b;                       Nil |-> Nil b ;
    Cons h t ↦ Cons b (f h)            Cons h t |-> Cons b (f h)
      (map a b f t) }                    (map a b f t) }
```

`(a : Type) (b : Type)` vs `(a b : Type)` is the same telescope; `↦` vs `|->` is
the same surface form. Derived `filter` (`:145`) and Property `gen_map_list`
(`:44`, binders `samples/sample/rest` = prelude `xs/h/t`) are likewise isomorphic.
So every downstream proof that mentions the local symbol (e.g. Derived
`theorem map_length` at `:190`, which inducts on `map a b f t`) re-elaborates
against the ambient provider — the name now resolves to `P.map`, which reduces
the same way, so the proof re-checks. Note this is single-head re-elaboration
against `P.map`, NOT a cross-identity conversion of local-`map` against `P.map`;
the latter is exactly what the Spec ruling classifies as `false`+halts and what
the removed AC wrongly demanded.

## The un-shadow shape — this drain is NOT a selective import

**Read this before shaping any control.** Every prior CAT drain
(`CAT-DERIVED`/`CAT-BOOL`) added `import <Module> (<name>)` for a now-public
PACKAGE provider. This one is different because the provider `P` is the compiler
prelude, `[ambient]`/`[installed]` (§3 row 63: `ambient=filter,fold,map`,
"Installed before source; no package attached-owner edge"). The names
`map`/`filter` are already in scope in every module ambiently; the local
definitions merely SHADOW them.

⇒ The drain is: DELETE the local definition; add NO import line; the reference
falls through to the installed prelude binding. There is no new dependency edge
to assert, and no import to withdraw. The positive evidence is a RESOLUTION
FLIP: the name resolved to a module-owned global on the base and resolves to the
installed prelude GlobalId on the candidate, with no module-owned local left
behind. Do not import-alias the prelude; do not add a package edge.

## Fixed inputs (measured at `origin/main` `e71ddb479`)

Census group 1 P-subset, three `[low]` consumer sites across two packages.
Coordinates decay — re-measure (`git fetch`; re-grep the `fn` headers) before
editing.

| consumer site | local reimplements | ambient provider | provider identity |
|---|---|---|---|
| `Data/Collections/Derived.ken.md#map` (`:139`) | `map` | `P.map` | prelude, installed |
| `Data/Collections/Derived.ken.md#filter` (`:145`) | `filter` | `P.filter` | prelude, installed |
| `Tooling/Testing/Property.ken.md#gen_map_list` (`:44`) | `gen_map_list` | `P.map` | prelude, installed |

Census consume tags (§3): Derived row 36 — `map->P.map [low]`,
`filter->P.filter [low]`; Property row 48 — `gen_map_list->P.map [low]`. All
three `[low]`.

Internal call sites to fix (the Derived sites are same-name un-shadows; the
Property site is a RENAME because the local name `gen_map_list` differs from the
provider `map`):

- Derived `map`/`filter`: local names equal the provider names, so deleting the
  local leaves every internal reference (`map_length` at `:190-195`, and the
  §4.1-floor consumers) resolving to the ambient provider unchanged.
- Property `gen_map_list`: called once, by `gen_map` at `Property.ken.md:51`
  (`gen_from_list b (gen_map_list a b f (gen_samples a generator))`). Draining
  deletes `fn gen_map_list` and repoints that call to the ambient `map`.

Standalone baselines (MEASURE FIRST — the CAT-BOOL D2 lesson):

- Derived is `[ok]` standalone (§3 row 69, `ken check` exit 0), so its
  raw-standalone AC is real and must stay green after the drain (the prelude is
  always installed, so `map`/`filter` remain resolvable).
- Property is a consumer that imports `Data.Collections.Derived (length)`
  (`:33`); its standalone status is NOT recorded in §3 (that table is provider
  modules). The D2 increment MUST establish Property's standalone baseline on the
  untouched base BEFORE choosing its acceptance control — do not assume it
  elaborates standalone, and do not silently substitute a fixture green for a
  standalone green (exactly the false premise CAT-BOOL D2 hit on Map).

## Deliverable

Two per-package increments, released and verified one at a time (the
`CAT-BOOL`/`CAT-DERIVED` pattern), each deleting the named local reimplementation
and letting the reference fall through to the ambient prelude:

- **D1 — Derived** (`Data/Collections/Derived.ken.md`): delete `fn map` (`:139`)
  and `fn filter` (`:145`); add no import; internal references unchanged (same
  names, now resolving to `P.map`/`P.filter`).
- **D2 — Property** (`Tooling/Testing/Property.ken.md`): delete `fn gen_map_list`
  (`:44`); repoint the single call in `gen_map` (`:51`) to the ambient `map`; add
  no import. Touch ONLY `gen_map_list` — leave the other Property consumes
  (`property_list_length->D.length`, the cursor/uint8/bytes items in row 48)
  alone; they are separate providers and separate groups.

## Acceptance criteria (each increment)

- **AC-RECURSIVE-UNSHADOW-MIGRATION** — the single operative acceptance property
  (Architect `evt_7spzy25qqdsqx`, on Spec `evt_3z8y6pf6b6m5p`). It REPLACES the
  removed `AC-CENSUS-ROW-DRAINED` / `AC-UNSHADOW-RESOLUTION` /
  `AC-NO-EQUIVALENT-LOCAL` / `AC-SAME-BEHAVIOUR`. A candidate-specific migration
  proof — NOT a universal anti-duplication theorem and NOT a kernel-equivalence
  claim over the recursive providers.

  D1, untouched base `B = f8b2dd6420f7a344d2439a6ab6658523346753af`, candidate `C`:
  - On B, roots-loading identifies the installed prelude IDs `P.map` and
    `P.filter`, the distinct module-owned IDs `Data.Collections.Derived.map` and
    `.filter`, and each local body's self edge to its OWN local ID.
  - On C, the two module-owned IDs are ABSENT; the same retained source references
    that resolved to the local IDs on B now resolve to the installed prelude IDs.
    No package-global or residual local may intervene, and no import may be added.
  - C's complete module-owned global-name inventory EQUALS B's inventory minus
    exactly `{map, filter}`. No new declaration under any fresh name is permitted.
    Together with a product diff deleting precisely those two declarations, no
    other product edit, and an inverse patch that reconstructs B, this closes
    rename laundering for this candidate without claiming semantic equivalence.
  - Raw `ken check Derived` exits 0 on B and C. Retained dependent declarations —
    including `map_length` and the named sort/string/byte headlines — elaborate on
    C. Nondegenerate evaluation controls for `map`/`filter` cover Nil AND recursive
    Cons cases, with `map` using a nonidentity function and `filter` using both
    true and false outcomes. Repointing a governed reference to a DIFFERENT ambient
    provider must RED.

  D2: the same relation, with C's inventory equal to B minus exactly
  `{gen_map_list}`, the one measured `gen_map` call changed to installed `P.map`,
  no import and no new declaration, plus the nondegenerate `gen_map` behavior
  control and the standalone-or-raw-boundary controls (AC-STANDALONE-BASELINE).

  PROHIBITED replacements (Architect, explicit): do NOT substitute raw `Term ==`,
  an occurrence/name census, a larger stack, self-`GlobalId` rewriting, recursive-
  graph bisimulation, or another normalizer — each would either assert the refuted
  equality or create a second conversion relation.

  EXPLICIT RESIDUAL (write it beside the AC; do NOT relabel it kernel-backed):
  this proves the candidate REMOVES the named reimplementations without adding a
  renamed replacement. It does NOT mechanically prohibit a differently-named,
  behaviorally-isomorphic recursive helper in an arbitrary future change — kernel
  definitional equality cannot express that property, because two separately
  declared recursive globals with isomorphic bodies but distinct self-`GlobalId`s
  are distinct rigid heads and non-convertible (`false` and halts). That residual
  stays review/census-enforced unless Ken later gains a separately specified
  recursive-scheme relation.
- **AC-STANDALONE-BASELINE** — establish the module's raw `ken check` baseline on
  the UNTOUCHED base first, then choose the gate:
  - If the module is standalone-green on base (Derived, `[ok]`), the candidate
    MUST remain standalone-green: `scripts/ken-cargo run -p ken-cli -- check
    <module>` exits 0 on base and candidate. A drain that turns a standalone
    module non-standalone HARD-STOPS to the Architect.
  - If the module has a PRE-EXISTING standalone failure (measure Property; do not
    assume), gate instead on RAW-BOUNDARY-PRESERVED: base and candidate reach the
    SAME pre-existing first unresolved-name failure (compare variant/name, not
    source span); a NEW earlier failure is drain-caused and RED. This is
    negative-scope evidence only — do NOT call it standalone success, and do NOT
    substitute a fixture green for it. Filing any newly-discovered pre-existing
    dependency defect as a distinct follow-on (the `CAT-MAP-DEPENDENCY-CLOSURE-
    REPAIR` precedent) is in scope; repairing it inside this drain is NOT.
- **AC-NO-OTHER-DRAIN** — only the named site(s) are drained. Control: every other
  local definition and its call sites in the module are byte-unchanged; no other
  census row moves.

## Reviewers

Foundation QA — `AC-RECURSIVE-UNSHADOW-MIGRATION` holds: the inventory equals
base minus exactly the drained name(s), the module-owned ID is absent on the
candidate and the retained references resolve to the installed prelude IDs with no
intervening package-global or residual local, the inverse patch reconstructs base,
the retained dependents (`map_length`, the sort/string/byte headlines; Property's
`gen_map`) elaborate on C, the nondegenerate Nil/Cons evaluation controls pass, and
repointing a reference to a DIFFERENT ambient provider reddens; the standalone
baseline is measured on the untouched base FIRST and the correct gate chosen
(Derived stays standalone-green; Property's baseline is measured, not assumed).
conformance-validator — the loader actually resolves the drained reference to the
installed prelude provider, not to a shadowing local or a package global (the
consumer mirror of the loader-visibility inventory the CV owns). A module turned
non-standalone by a drain, or a NEW raw failure earlier than a pre-existing one,
HARD-STOPS to the Architect. Do NOT reintroduce any kernel-equivalence /
recursive-clone control — the migration property is the whole gate, and asserting
convertibility of the two recursive heads is the refuted claim.

## Capability tier

T2 — a mechanical, precedent-shaped catalog reuse drain (three sites, two files),
reviewed differentially on the candidate-specific migration property (inventory
delta, resolution flip, inverse patch, retained-dependent elaboration, nondegenerate
evaluation) and the unworsened standalone/raw boundary, not on an argument. The
source-equation isomorphism of local and provider bodies is measured, not argued.
The kernel conversion-totality bug the ruling surfaced is a SEPARATE T1 follow-on
(Sequencing), not part of this drain. Size S (three sites, matching CAT-BOOL).

## Sequencing

Lane 3 (foundation). `depends_on: []` — the provider `P` is the installed
compiler prelude and needs no pub-export prerequisite (§3 row 63). Released on the
`CAT-BOOL-REUSE-CONSUMERS` (group 6) landing so the lane does not idle. This
drains the P-provider subset of census group 1.

Deferred group-1 members (`LawfulFunctors#list_map`/`#list_foldr`,
`EffectfulClasses#compose`) are NOT framed here (§4c — frame on need, not ahead of
it): the LawfulFunctors items are instance-bound with attached laws and sit in a
`[higher]` module, and `compose`'s provider LF is `[higher]`. Measure them for a
separate node when this lands. Groups 5 and 7 remain unmeasured.

**Kernel conversion-totality follow-on (separate, non-blocking).** The ruling
surfaced a real kernel defect: converting two separately-declared source-isomorphic
recursive globals stack-overflows instead of returning `false` and halting (§17's
SCT admission argument does not license unbounded symbolic cross-identity unfolding
beneath stuck eliminators). Spec is routing a narrow §17/conformance erratum
(spec-author, `evt_35x7q85z1wk6t`). AFTER that erratum lands, the Steward frames a
kernel repair (T1, kernel team) whose black-box matrix must include: two well-typed
separately-admitted source-isomorphic recursive maps (`convert_type == true`, value
`convert == false`, halts on the ordinary test stack); same recursive head with
equal spines (true, halts, no delta expansion); distinct nonrecursive transparent
heads with a finite common reduct (existing true behavior preserved); a genuinely
different recursive body (false, halts). No `RUST_MIN_STACK`, timeout-as-success, or
stack increase counts as evidence; the implementation technique follows the Spec
erratum. This drain does NOT wait on it — it neither invokes nor depends on
cross-identity conversion, and it removes the duplicate local recursive heads.

## Symptom inventory (append one line per hard-stop; never rewrite history)

```text
1. distinct recursive self identities make the inherited kernel-equivalence control diverge — keyed on cross-identity symbolic delta unfolding
```
