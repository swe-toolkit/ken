---
name: a-gate-change-is-hunted-on-the-axis-its-direction-leaves-open
description: A gate or resolver change moves the accepted set in one direction, and that direction decides where to hunt. A narrowing gate that only adds rejections cannot over-accept, so hunt regression - did the rejected marker mean anything at any consumer. A widening that unifies two resolver modes is over-accept-safe only if the strict arm reduces byte-identical and the widened arm already defaulted permissive. Instances - LANG-MOD-PUB-ELIGIBILITY (f554abfd4) and Component A reduced (16f064381).
metadata:
  type: feedback
---

# A gate change is hunted on the axis its direction leaves open

Before hunting a change to a gate, a resolver or a mode check, decide which way
it can move the accepted set. A change that only adds rejections has closed
the soundness axis by construction; its risk is over-strict regression. A
change that widens has closed nothing, but whether it can over-accept is
decided by the default of the arm it widens, not by how many names it binds.
State the direction in the verdict
([[state-the-DIRECTION-of-a-weakness-over-strict-or-unsound]]).

## Narrowing: what did the rejected marker mean at every consumer?

**Measured 2026-08-23 on `f554abfd4`** (LANG-MOD-PUB-ELIGIBILITY WP-3,
`dec_1mbphr5j36g4`). Change-triggered hunt, CLEAN. `evt_5mcd6aex79dgx`
(Steward side thread `thr_4g49g6pqvhq7x`).

The change added `pub_eligibility` (`crates/ken-elaborator/src/parser.rs`), a
no-catch-all classifier called from `parse_pub_decl`: every `Decl` variant is
Eligible, Ineligible(kind) or PublicSpace, and Ineligible turns a previously
accepted `pub <form>` into a hard `ParseError` for instance, law, foreign,
temporal, prove, import, export, module, program, package, space and pub-pub.

The regression surface is not "did the parser tolerate `pub X`" (it tolerated
everything) but **"did `pub` mean anything on X at any downstream consumer?"**
The tell that it was inert: a consumer that `unwrap_pub()`s defensively but
never **branches on `is_pub()`** for that kind. In `modules.rs` at the time:

- instance, law, foreign, temporal, prove: the `other` arm elaborated them
  unqualified and published only a `ClassDecl` on `is_pub`;
- import, export, module: the live arms matched the **bare** variant, so a
  `Decl::Pub(ImportDecl)` fell to the inert arm and never imported or
  re-exported (re-export is the bare `export` form);
- program, package: already rejected at `admission_boundary`
  (`if decl.is_pub() ... Err`);
- space: the classifier emits the same
  `UnsupportedSpacePlacement{placement:"public"}` the elaborator already did.

A grep of `catalog/` and `library/` found zero uses of any rejected form. So
no feature was lost; silent-ignore became a surface error.

How to apply to a narrowing gate:

1. Do not hunt for an over-accept it cannot produce.
2. Trace each newly rejected kind through every consumer's flag branch.
   "Tolerated" (defensive stripping) confers nothing; "carried semantics" (a
   branch on the flag) is a regression if foreclosed.
3. Grep the product corpus for live uses of each rejected form.
4. A no-catch-all match gives compile-time classification coverage, not
   positive-accept test coverage. Here 2 of 10 Eligible arms had a direct
   accept control, so a refactor that mis-slots an Eligible kind surfaces as an
   over-strict rejection caught only by the compile obligation.

Also from this node: the postfix `data T = ... derive (C)` lowers to the same
`Decl::DeriveDecl{class_name, data_name, span}` as the prefix
`derive C for D`, more constrained (data_name bound to the preceding data decl
via `unwrap_pub`). The test asserts the generated instance is
`KernelDecl::Transparent` and `trusted_base()` unchanged, the right two
assertions for a lowering that must add no trust.

## Widening: reduce the strict arm, read the widened arm's default

**Measured 2026-08-24 on `16f064381`** (Component A reduced,
`wp/LANG-MOD-CATALOG-REALIZATION`, Steward §10a gate). CLEAN.
`evt_1ssbm8sh1vvng` (Steward gate thread `thr_4gwq2q3r2tt4c`).

`prebind_scope_declarations` (`modules.rs`) had bound a module's own class
names and data constructors only in strict mode, through a
`strict_unqualified_local` guard and an
`if scope.mode != ResolutionMode::Strict { continue; }`. The change dropped
both, so legacy binds them too. The reflex worry is over-accept.

**Move 1: reduce the strict arm.** Substitute `mode == Strict` at each edited
site. New `unqualified_local = matches!(inner, ClassDecl)` equals old
`strict_unqualified_local = mode==Strict && matches!(inner, ClassDecl)` when
strict; the deleted `continue` never fired in strict. So the strict authorized
set cannot widen. Confirm with a strict negative test. Then it was
`strict_slice_stays_closed_and_external_nat_stays_rejected` (external `Nat`
gave `ElabError::UnboundName{"Nat"}`); after the later Nat-floor inversion the
test is `strict_slice_stays_closed_and_floor_nat_reuses_existing_identity` in
`lang_mod_catalog_realization.rs`, and the non-member reject lives in
`lang_mod_nat_floor_realization`.

**Move 2: read the widened arm's default.** Legacy `resolve_ref` ended
`None => Ok(name.to_string())`, a flat fallback that already accepted every
bare name. Adding local bindings cannot enlarge that set; it only redirects a
module's own names to `qualify(prefix, name)`, the correct identity (and at
root `prefix == ""`, a no-op). An over-accept needs an external name bound or a
name resolved to a wrong existing global, and binding only `decls`' own names
does neither. **If the widened arm had failed closed by default, the widening
would genuinely enlarge the accepted set, and every newly bound name would need
chasing.**

Controls still worth checking on such a change:

- Collision stays identity-based:
  `distinct_import_identities_with_one_spelling_still_collide` imports
  `A.item` and `B.item` under one spelling and expects
  `AmbiguousReference{"item"}` through `bind_import`; `bind_local`'s silent
  overwrite is reachable only for the same qualified string.
- Provider identity is real: `assert_alias_reuses` checks the alias body is
  `Term::Const{id}` with `id == env.globals[qualified_provider]`, and a
  compute-forcing `Proved` theorem (`add 2 3 ≡ 5`, `mul 2 3 ≡ 6`) exercises
  reduction; `trusted_base` delta empty.
- The stack claim is pinned:
  `local_prebinding_preserves_legacy_map_union_stack_budget` uses a fixed
  `D1_LEGACY_MAP_STACK_BYTES = 2*1024*1024` through `thread::Builder::stack_size`,
  documented as independent of `RUST_MIN_STACK`, with a bisected boundary so
  the inline-parent mutation SIGABRTs
  (the `stated-stacks` skill).

Related:
[[a-scope-install-is-only-pinned-by-a-fixture-that-references-what-only-that-scope-provides]],
[[a-gate-flag-bounds-its-own-path-not-every-consumer-of-the-mechanism]],
[[a-consolidate-onto-canonical-home-migration-is-anti-fabricated-by-global-uniqueness-plus-load-bearing-imports]],
[[removing-an-ambient-fallback-can-close-the-only-working-route-when-the-intended-route-was-already-broken]]
(a narrowing whose regression was real).
