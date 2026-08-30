---
id: CAT-PRELUDE-REUSE-CONSUMERS
title: "Drain catalog-reuse census group 1 (prelude functional-floor reuse), P-provider subset only — remove three local reimplementations (Derived#map, Derived#filter, Property#gen_map_list) that shadow the ambient compiler-prelude map/filter, letting each reference fall through to the installed provider. Un-shadow, not selective import: P is [installed]/ambient, so there is NO import edge to add. Shaped on the landed CAT-BOOL/CAT-DERIVED per-package increment pattern; the definitionally-transparent subset of group 1, with the instance-bound and [higher]-module items deferred."
status: ready
owner: foundation
size: S
gate: none
tier: T2
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-08-30, filed on the CAT-BOOL-REUSE-CONSUMERS landing (group 6 drained, d95bc2df4) as the next L3 catalog-reuse objective. Group 1 membership quoted verbatim from docs/program/cat-reuse-census.md §4.4 item 1 at origin/main e71ddb479; the [low] consume tags read from §3 rows 36 (Derived) and 48 (Property). Scoped to the three definitionally-clean P-provider sites after a pre-frame measurement (below) found the other three group-1 members instance-bound or [higher]-module. The provider P is the compiler prelude (§3 row 63, [ambient]/[installed]) and needs NO pub-export prerequisite, so depends_on is empty. Steward-filed per COORDINATION section 2."
---

> # RELEASED to foundation-leader 2026-08-30 (Steward), the next lane-3 drain.
> # `ready`; will flip `active` on the leader's kickoff.
>
> Scoped to the definitionally-transparent P-provider subset of census group 1.
> The pre-frame measurement below (Steward, 2026-08-30, at `e71ddb479`) is what
> narrowed the six-member group to three; read it before touching the deferred
> items. Build against the amended ACs from current main; on each candidate,
> fresh Foundation QA + CV on the exact SHA, then Steward M1-M4.

## Why this scope, not all of group 1 (pre-frame measurement, `e71ddb479`)

Census §4.4 item 1 "Prelude and functional-floor reuse" lists SIX members. This
node drains only the three whose local body is KERNEL-DEFINITIONALLY IDENTICAL to
the ambient prelude provider, so the swap is a pure un-shadow with no
proof-transfer obligation:

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

The measured definitional identity (why the three are `[low]` and safe):

```
Derived fn map (:139)              prelude map (prelude.rs:486)
  match xs {                         match xs {
    Nil ↦ Nil b;                       Nil |-> Nil b ;
    Cons h t ↦ Cons b (f h)            Cons h t |-> Cons b (f h)
      (map a b f t) }                    (map a b f t) }
```

`(a : Type) (b : Type)` vs `(a b : Type)` is the same telescope; `↦` vs `|->` is
the same surface form. Derived `filter` (`:145`) and Property `gen_map_list`
(`:44`, binders `samples/sample/rest` = prelude `xs/h/t`) are likewise identical.
So every downstream proof that mentions the local symbol (e.g. Derived
`theorem map_length` at `:190`, which inducts on `map a b f t`) reduces
identically against the ambient provider and stays green with no re-proof.

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

- **AC-CENSUS-ROW-DRAINED** — the increment's census §3 row no longer names the
  reimplementation, established by CANDIDATE-SPECIFIC migration evidence (the
  Architect HS3 shape from CAT-BOOL), NOT a universal property: the candidate diff
  deletes PRECISELY the named former definition(s) and nothing else, repoints only
  the measured internal call site(s) (D1: none; D2: the one `gen_map` call), adds
  no import line, and the candidate product inversely reconstructs the base
  product. This proves what THIS candidate did, not a property under arbitrary
  future edits.
- **AC-UNSHADOW-RESOLUTION** — the resolution flip is exhibited: on the base the
  drained name resolves to the module-owned global
  (`Data.Collections.Derived.map`/`.filter`,
  `Tooling.Testing.Property.gen_map_list`),
  and on the candidate the corresponding reference resolves to the INSTALLED
  prelude GlobalId for `map`/`filter`, with the former module-owned global ABSENT.
  Control: the module-owned global is present on base and absent on candidate;
  roots-loading the module resolves the reference to the prelude identity, not to
  any package global and not to a residual local. This is the un-shadow analogue
  of CAT-BOOL's one-edge import/withdrawal pair — there is no import to withdraw,
  so the evidence is presence-on-base / absence-on-candidate plus prelude
  resolution.
- **AC-NO-EQUIVALENT-LOCAL** — the NARROW kernel-definitional anti-duplication
  control from CAT-BOOL (Architect `evt_3k9km6125h088`), REUSED UNCHANGED with the
  ambient prelude symbol as the provider. The property, closed over the module's
  own definitions:

  > no module-owned admitted transparent declaration `d` such that
  > `type(d) ≡ provider_ty` and `body(d) ≡ provider_body : provider_ty`
  > (`≡` = kernel definitional equality).

  Bind `provider` to the installed prelude `map` / `filter` GlobalId and run
  `module_transparent_kernel_equivalents` (the exact helper printed in
  CAT-BOOL-REUSE-CONSUMERS `AC-NO-EQUIVALENT-LOCAL`) over each module-owned
  admitted `Decl::Transparent` under `Context::new()`, provider and locals
  restricted to zero declaration-level parameters. The prelude `map`/`filter` are
  elaborated as ordinary no-spec `fn`s (`elaborate_decl("fn map ...")`,
  `prelude.rs:486`/`:501`), so they take the V0 admission path with
  `level_params == []`; the `provider_level_params.is_empty()` assertion does not
  panic. Do NOT reimplement kernel conversion — beta/zeta/delta/typed-eta and
  proof irrelevance are the kernel's own. Required controls (per increment):
  baseline inventory empty AFTER the drain; re-introducing an exact-bodied local
  RED; a zeta-redex-equivalent local RED; a same-typed but non-convertible local
  (e.g. `fun x y => x` for map) GREEN. What this does NOT prove (Architect,
  explicit): it does NOT establish that any governed computation FLOWS THROUGH the
  provider — no causal-flow claim rests on it and none is required for this T2
  drain. HARD STOP: if a legitimately-needed local is kernel-equivalent to the
  prelude provider, or the equivalence check has a real gap, that is a hard stop
  to the Architect — never a raw `Term ==` or occurrence-census respin.
- **AC-SAME-BEHAVIOUR** — the module elaborates to the same result through the
  ambient provider as through the deleted local. Control: the module's existing
  checked declarations and dependent headlines still elaborate (Derived's
  `map_length` and the sort/derived string-byte headlines; Property's `gen_map`
  and the generator operations); a mutation that repoints the reference to a
  DIFFERENT ambient name (e.g. `fold` in place of `map`) reddens.
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

Foundation QA — the census row is drained; the un-shadow resolution flip holds
(module-owned global present on base, absent on candidate, reference resolves to
the installed prelude identity); the same-behaviour control reddens when the
reference is repointed to the wrong ambient name; the standalone baseline is
measured on the untouched base FIRST and the correct gate chosen (Derived stays
standalone-green; Property's baseline is measured, not assumed). conformance-
validator — the loader actually resolves the drained reference to the installed
prelude provider, not to a shadowing local or a package global (the consumer
mirror of the loader-visibility inventory the CV owns). A module turned
non-standalone by a drain, or a NEW raw failure earlier than a pre-existing one,
HARD-STOPS to the Architect. A legitimately-needed local found kernel-equivalent
to the prelude provider is a hard stop, not a respin.

## Capability tier

T2 — a mechanical, precedent-shaped catalog reuse drain (three sites, two files),
reviewed differentially on census-row-drained plus the un-shadow resolution flip
and the unworsened standalone/raw boundary, not on an argument. The definitional
identity of local and provider bodies is measured, not argued, which is what keeps
it T2. Size S (three sites, matching CAT-BOOL).

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

## Symptom inventory (append one line per hard-stop; never rewrite history)

```text
(none yet)
```
