---
id: CAT-BOOL-REUSE-CONSUMERS
title: "Drain catalog-reuse census group 6 (Boolean computational reuse) — replace three reimplementations (Derived#bool_and, Derived#bool_leq, Map#option_is_some) with selective imports of the now-public LC.bool_and, LC.bool_leq and SC.is_some. The consumer half of CAT-BOOL-PUB-EXPORT, shaped on the landed CAT-DERIVED-REUSE-CONSUMERS per-package increment pattern."
status: merged
owner: foundation
size: S
gate: none
tier: T2
depends_on: [CAT-BOOL-PUB-EXPORT]
blocks: []
github: https://github.com/swe-toolkit/ken/pull/3136
origin: "Steward, 2026-08-29, filed on the CAT-BOOL-PUB-EXPORT landing (providers public at 4faa97bfb, PR #3108) so lane 3 does not idle. Group 6 membership quoted verbatim from docs/program/cat-reuse-census.md §4.4 item 6 (lines 317-320) at origin/main 4faa97bfb; the three [low] consume tags read from §3 rows 36 (Derived) and 37 (Map). The two providers CAT-BOOL-PUB-EXPORT just published (LC.bool_and, LC.bool_leq, SC.is_some) are exactly the three names group 6 consumes, so the prerequisite covers the consumers with nothing left over. Steward-filed per COORDINATION section 2."
---

> # D2 AMENDED — Architect ruling `evt_em72d9eh6ndg`, 2026-08-30, base
> # `0ddd49b3`. D1 landed; this amends D2's acceptance criteria only.
> #
> # The foundation-implementer took a valid PRE-EDIT hard stop: `AC-STANDALONE-
> # GREEN` does not reproduce, because Map fails raw `ken check` on the untouched
> # base with a pre-existing `UnresolvedCon list_append` (Map imports only
> # `Core.Logic.Or`; its acceptance path is fixture-backed). The raw-standalone AC
> # is WITHDRAWN for D2 and replaced by AC-LEGACY-CLOSURE-PRESERVED +
> # AC-D2-DEPENDENCY-EDGE + AC-RAW-BOUNDARY-PRESERVED (see Acceptance criteria).
> # The pre-existing Map dependency-closure defect is filed as a DISTINCT
> # follow-on (`CAT-MAP-DEPENDENCY-CLOSURE-REPAIR`), NOT a D2 prerequisite. After
> # this lands, the clean held D2 branch is re-released through Foundation Leader.

> # AMENDED — Architect HS3 ruling `evt_3k9km6125h088`, 2026-08-29. After this
> # lands, Foundation may respin D1 TEST-ONLY from exact `bc56f2f7`.
>
> D1 candidate `bc56f2f7ba2c65a5bc7f1ed67e1b70b29a8282cf` stays REJECTED: its
> module-owned inventory compares raw `Term ==`, so CV's admitted zeta-redex is
> kernel-convertible and stayed green — the object does not satisfy its advertised
> AC, and no prior approval transfers. Three hard stops
> (`evt_6frnqf7qpzw6`, `evt_7yqc1rh8g2805`, `evt_3ms5vh7szf17p`) share ONE
> predicate: each substituted a representation proxy for a causal provider-
> authority claim this drain never needed. This amendment SEPARATES the three
> properties — candidate-specific migration evidence (`AC-CENSUS-ROW-DRAINED`), a
> NARROW kernel-definitional anti-duplication control (`AC-NO-EQUIVALENT-LOCAL`),
> and NO route-authority claim at all — and strips every causal-flow overclaim.
> Research advisory `evt_7zan5cghe95jy`. The candidate's SOURCE (the Derived
> import + local deletion) was already correct; the defect is the TEST/control, so
> the respin is test-only. Provider prerequisite `CAT-BOOL-PUB-EXPORT` MERGED
> (`4faa97bfb`, PR #3108); three names `pub`/loader-visible. Two consumer modules,
> three `[low]` sites. D2 stays unstarted until D1 closes.

## Fixed inputs (re-measured at `origin/main` `4faa97bfb`)

Census group 6, **"Boolean computational reuse"**, quoted verbatim from
`docs/program/cat-reuse-census.md` §4.4 item 6 — three `[low]` consumer sites
across two packages:

| consumer site (§4.4) | reimplements | provider (now `pub`) | provider module |
|---|---|---|---|
| `Data/Collections/Derived.ken.md#bool_and` | `bool_and` | `LC.bool_and` | `Core/Classes/LawfulClasses.ken.md` |
| `Data/Collections/Derived.ken.md#bool_leq` | `bool_leq` | `LC.bool_leq` | `Core/Classes/LawfulClasses.ken.md` |
| `Data/Collections/Map.ken.md#option_is_some` | `is_some` | `SC.is_some` | `Data/Sums/Combinators.ken.md` |

Census consume tags (§3): Derived row 36 — `bool_and->LC.bool_and [low]`,
`bool_leq->LC.bool_leq [low]`; Map row 37 — `option_is_some->SC.is_some [low]`.
All three are `[low]`. **The other Map boolean consumes in row 37
(`bool_and`, `cat4_bool_or`, `bool_dichotomy`, `leq_nat`, `total_leq_nat`) are
`[higher]` and are NOT group 6 — do NOT touch them here.**

**The Map site is a RENAME, the Derived sites are not.** Derived's local
`bool_and`/`bool_leq` share the provider's names, so an unqualified selective
import leaves internal call sites unchanged. Map's local is named
`option_is_some` while the provider is `is_some`, so draining it renames the
reference at every internal call site (or aliases the import) — measure the call
sites before deleting the local.

## Deliverable

Two per-package increments, released/verified one at a time (the
`CAT-DERIVED-REUSE-CONSUMERS` pattern), each replacing the named
reimplementation(s) with a selective import of the now-public provider and
deleting the local definition:

- **D1 — Derived** (`Data/Collections/Derived.ken.md`): import `bool_and` and
  `bool_leq` from `Core.Classes.LawfulClasses`; delete the two local
  reimplementations; internal references unchanged (same names).
- **D2 — Map** (`Data/Collections/Map.ken.md`): import `is_some` from
  `Data.Sums.Combinators`; delete the local `option_is_some`; update its
  internal call sites to the imported name (or alias the import). Touch ONLY
  `option_is_some` — leave the `[higher]` Map consumes alone.

## Acceptance criteria (each increment)

- **AC-CENSUS-ROW-DRAINED** — the increment's census §4.4 group-6 row(s) no
  longer name a reimplementation, established by CANDIDATE-SPECIFIC migration
  evidence (Architect HS3 ruling), NOT a universal property: the candidate diff
  adds the selective import, deletes PRECISELY the named former definition(s) and
  nothing else, and leaves the internal references byte-unchanged (D1: same
  names; D2: the renamed references, measured); the candidate product INVERSELY
  reconstructs the base product; roots loading resolves the imported identit(ies)
  to the canonical transparent provider(s); WITHDRAWING either import produces the
  expected unresolved-name failure; standalone behaviour and the trust delta stay
  green with zero delta. This proves what THIS candidate did; it does not pretend
  to a universal property under arbitrary future edits.
- **AC-NO-EQUIVALENT-LOCAL** (a NARROW kernel-definitional ANTI-DUPLICATION
  control — NOT a causal route proof; re-scoped 2026-08-29 by Architect HS3
  ruling `evt_3k9km6125h088`, research advisory `evt_7zan5cghe95jy`, after three
  hard stops). The property, closed over the module's own definitions:

  > no module-owned admitted transparent declaration `d` such that
  > `type(d) ≡ provider_ty` and `body(d) ≡ provider_body : provider_ty`
  > (`≡` = kernel definitional equality).

  The sound D1 mechanism IS kernel conversion — `convert_type(local_ty,
  provider_ty) && convert(provider_ty, local_body, provider_body)` under
  `Context::new()`, over each module-owned admitted `Decl::Transparent`, provider
  and locals restricted to zero level parameters. `convert_type` establishes the
  common type before type-directed `convert` uses `provider_ty`; the empty
  context is correct for closed top-level terms. Do NOT reimplement kernel
  conversion with a custom normalizer — beta/zeta/delta/typed-eta and proof
  irrelevance are the kernel's own. The Architect compiled and probed the exact
  helper (`/tmp/architect-cat-bool-convert`):

  ```rust
  use ken_kernel::{convert, convert_type, Context, Decl, GlobalId};

  fn module_transparent_kernel_equivalents(
      env: &ElabEnv,
      module: &str,
      provider: GlobalId,
  ) -> BTreeSet<String> {
      let (provider_level_params, provider_ty, provider_body) =
          match env.env.lookup(provider) {
              Some(Decl::Transparent { level_params, ty, body, .. }) =>
                  (level_params, ty, body),
              other => panic!("provider must be transparent, got {other:?}"),
          };
      assert!(
          provider_level_params.is_empty(),
          "group-6 providers in this control must have zero declaration-level parameters"
      );
      let prefix = format!("{module}.");
      let context = Context::new();
      env.globals
          .iter()
          .filter_map(|(name, id)| {
              let local_name = name.strip_prefix(&prefix)?;
              let (level_params, ty, body) = match env.env.lookup(*id) {
                  Some(Decl::Transparent { level_params, ty, body, .. }) =>
                      (level_params, ty, body),
                  _ => return None,
              };
              if !level_params.is_empty() { return None; }
              (convert_type(&env.env, &context, ty, provider_ty)
                  && convert(&env.env, &context, provider_ty, body, provider_body))
              .then(|| local_name.to_owned())
          })
          .collect()
  }
  ```

  Required D1 controls: baseline inventory empty; exact-bodied `bool_and` and
  `bool_leq` locals RED; CV's zeta-redex local (`let chosen = y`) plus an unused
  provider alias padding RED (`left: {"local_bool_leq"}`); a same-typed but
  non-convertible local (`fun x y => x`) GREEN. The assertion message reads
  "kernel-definitionally equal checked type and body at the provider type", NOT
  "exact type and body". Kernel conversion's own semantics need not be
  reimplemented in the test.

  **What this control does NOT prove (Architect, explicit): it does NOT establish
  that any governed computation FLOWS THROUGH the import.** Inline
  reimplementation, a specialized helper of another type, an opaque helper, a
  non-definitionally-equal but behaviorally equal implementation, or unused
  provider padding can all coexist with an empty inventory. No universal
  causal-flow or per-route-authority claim rests on this AC, and NONE is required
  for this T2 mechanical drain; a future route-authority requirement would be a
  distinct design object with its own route binding, not an inference from the
  absence of equivalent locals. Differently-spelled semantic duplication remains
  the Architect catalog-factoring review's judgment surface — not decidable here.

  **D2 REUSES THE ZERO-DECLARATION-LEVEL HELPER UNCHANGED** (Architect ruling
  `evt_6zvw2txpw69nm`, base `11ce6f3aa`). The earlier warning that `SC.is_some` is
  polymorphic and escapes the helper was FALSE. `is_some`'s `(a : Type)` is a
  TERM-level parameter, represented inside the closed declaration type/body as
  `Pi`/`Lam`, NOT a `Decl::Transparent.level_params` binder. Both the provider
  (`Data.Sums.Combinators.is_some`, blob `56530688`, `pub fn is_some (a : Type)
  ...`) and the Map local (`option_is_some`, `fn option_is_some (v : Type) ...`)
  therefore have `level_params == []` — an ordinary no-spec `fn` takes the V0
  admission path and `declare_def(env, vec![], ty_core, body_core)`. So the
  `provider_level_params.is_empty()` assertion does not panic, the
  `level_params.is_empty()` filter does not skip `option_is_some`, and
  `Context::new()` stays correct: the compared top-level terms are closed and their
  `a`/`x` binders are internal de Bruijn binders kernel conversion handles. Do NOT
  add symbolic level substitution or a "matching level-param context" for D2 — that
  solves an absent population and enlarges the mechanism without increasing its
  reach.

  D2 controls (Architect `evt_6zvw2txpw69nm`): roots-load `Data.Sums.Combinators`
  and `Data.Collections.Map`; bind the provider by exact GlobalId
  `Data.Sums.Combinators.is_some`; baseline Map inventory empty; an exact-bodied
  differently-named Map local REDs; a zeta-redex equivalent Map local REDs; a
  same-typed but non-convertible local (e.g. constant `False`) stays GREEN; the
  former global `Data.Collections.Map.option_is_some` is absent, the selective
  import resolves to the canonical transparent provider, removing/wrong-naming the
  import REDs, and Map's existing behavior plus zero trust delta stay green; all
  `[higher]` Map consumers and call sites remain byte-identical.

  HARD STOP: if the implementer observes a non-empty `level_params` vector for
  either exact current declaration, that contradicts the grounded admission path
  above — stop with the printed `Decl`, do not silently generalize. The general
  rule also stands: if a sound closed mechanism cannot be built for an increment (a
  legitimately-needed local is kernel-equivalent to the provider, or the
  equivalence check has a real gap), that is a HARD STOP to the Architect — never
  another occurrence-census or raw-`Term ==` respin.
- **AC-SAME-BEHAVIOUR** — the consumer module elaborates to the same result
  through the imported provider as through the deleted local. Control: the
  module's existing checked declarations and any dependent headline
  (Derived's sort/derived string-byte headlines; Map's `Tree` map operations)
  still elaborate; a mutation that imports the WRONG provider name reddens.
- **AC-STANDALONE-GREEN — WITHDRAWN for D2** (Architect `evt_em72d9eh6ndg`, base
  `0ddd49b3`). The raw-standalone premise is FALSE for Map: on the untouched base,
  `scripts/ken-cargo run -p ken-cli -- check
  catalog/packages/Data/Collections/Map.ken.md` exits 1 with a PRE-EXISTING
  `UnresolvedCon { name: "list_append" }` (Map imports only `Core.Logic.Or` at
  line 80, then consumes undeclared `list_append` at 90-93), and Map's established
  acceptance path is fixture-backed — `map_build_acceptance.rs` blob
  `82576c772e5be8b76cc829f0ab5c2ca7948c1cab`, `mk_env` lines 38-47 preload
  Compare/Transport/Derived/Or before elaborating the real Map source. So D2
  CANNOT be gated on Map elaborating standalone, and "fixture green" must NOT be
  silently substituted for it. D1 (Derived) landed under its own gates and is
  unaffected. For D2 this AC is replaced by the three observations below.
- **AC-LEGACY-CLOSURE-PRESERVED** (D2) — base and candidate real Map source
  elaborate under the SAME established Map dependency fixture. Load
  `Data.Sums.Combinators` canonically, but do NOT expose an unqualified `is_some`
  alias outside Map's own selective import. Candidate stays behavior-green with
  zero trust delta. This proves the migrated computation.
- **AC-D2-DEPENDENCY-EDGE** (D2) — the candidate adds exactly ONE dependency edge:
  Map's selective import resolves `is_some` to the exact transparent GlobalId
  `Data.Sums.Combinators.is_some`. Removing the import or naming the wrong provider
  must FAIL at `is_some`, and the fixture must NOT pre-bind that unqualified name.
  The former `Data.Collections.Map.option_is_some` global is absent. This is the
  product observation; fixture padding cannot satisfy it.
- **AC-RAW-BOUNDARY-PRESERVED** (D2) — run the raw `ken check` command on base and
  candidate; both must reach the SAME pre-existing first unresolved constructor
  name `list_append` (compare variant/name, NOT source span). A new earlier failure
  at the `Data.Sums.Combinators` import or at `is_some` is D2-caused and RED. This
  is negative-scope evidence ONLY — do NOT call it standalone success.
- **AC-NO-OTHER-DRAIN** (D2 only) — Map's `[higher]` boolean consumes are
  untouched; only `option_is_some` is drained. Control: those local definitions
  and their call sites are byte-unchanged.

## Reviewers

Foundation QA (the census row is drained; for D1 the module stays
standalone-green, for D2 the three D2 observations hold — legacy closure preserved
under the established fixture, the exact one-edge import/withdrawal pair, and the
raw `list_append` boundary unworsened — and the same-behaviour control reddens on
the wrong provider) + conformance-validator (the loader actually resolves the
selective import to the public provider, not a shadowing local — this is the
consumer mirror of the loader-visibility inventory the CV owns on the provider
side). For D2, a NEW raw failure earlier than the pre-existing `list_append` (at
the Sums import or at `is_some`) HARD-STOPS to the Architect; a consumer module
turned non-standalone by a D1 drain still HARD-STOPS.

## Capability tier

T2 — a mechanical, precedent-shaped catalog reuse drain (three sites, two files),
reviewed differentially on census-row-drained (plus, for D2, the exact one-edge
import/withdrawal and the unworsened raw `list_append` boundary), not on an
argument. Size S (smaller than group 4's six sites / five packages).

## Sequencing

Lane 3 (foundation). Released 2026-08-29 on the `CAT-BOOL-PUB-EXPORT` landing so
the lane does not idle. `depends_on: [CAT-BOOL-PUB-EXPORT]` (merged). This closes
census group 6. Groups 1, 5 and 7 are not re-measured and are not framed here
(§4c — frame on need, not ahead of it); group 1's provider is the compiler
prelude and may need no prerequisite, but that is a measurement for when this
lands, not now.

The exact D1 candidate `bc56f2f7`'s three-path intersection with `origin/main`
since merge-base `ba1c92214` is empty (Architect), so no rebase is justified;
Foundation respins D1 test-only from that SHA after this amendment lands. D2
remains unstarted until D1 closes.

**Distinct follow-on, NOT a D2 prerequisite** (Architect `evt_em72d9eh6ndg`): Map
(`Data/Collections/Map.ken.md`) does not elaborate from its own imports — it
imports only `Core.Logic.Or` yet consumes undeclared `list_append` (and relies on
the `map_build_acceptance.rs` fixture preloading Compare/Transport/Derived/Or). A
Map dependency-closure repair is warranted but is pre-existing and potentially
spans the full legacy fixture inventory. It is filed separately as
`CAT-MAP-DEPENDENCY-CLOSURE-REPAIR` (`draft`, queued); this one-provider `is_some`
drain must NOT be held behind it or smuggle a fix into D2.

## Symptom inventory (append one line per hard-stop; never rewrite history)

```text
1. One selected provider occurrence missed eight direct consumers — keyed on a chosen declaration.
2. A complete occurrence census accepted unused provider padding — keyed on provider presence.
3. Raw `Term ==` accepted a zeta-equivalent local — keyed on representation identity.

Shared predicate: each detector substituted a representation proxy for causal provider authority. The structural closure is to separate candidate-specific migration evidence, kernel-definitional anti-duplication, and any future route-authority property; never infer one from another.

4. D2 pre-edit stop: AC-STANDALONE-GREEN's raw-standalone premise is false — Map fails raw `ken check` on the untouched base (pre-existing UnresolvedCon list_append) and its acceptance path is fixture-backed. Different class from 1-3: a frame premise falsified by a pre-existing consumer-module dependency defect, not a detector proxy. Fix: withdraw the raw-standalone AC for D2, gate on legacy-closure-preserved + one dependency edge + unworsened raw boundary; file the Map closure repair as a distinct follow-on.
```
