---
id: LANG-MOD-PAIR-FLOOR-PROVIDER
title: "Generalize the prelude membership rule to ONE internal-provision arm (kernel OR compiler origin) and admit Pair as its first compiler-provided member: the type floor becomes the closed TEN {Auth, Bool, Char, List, Nat, Option, Pair, ResourceKind, Result, Utf8Error} with Pair's {mk_pair, pair_fst, pair_snd} a separate closed three-companion inventory, reusing the four existing compiler-installed Pair GlobalIds (prelude.rs:951-1041); with the coupled normative flips (30 §4/§5, 33 §3.3/§4.3/§5.3, 34, 39 §2.0/§6.1, 51, 50-stdlib) and the conformance-pin contract. The SPEC WP; the elaborator realization is the build WP. Supersedes the merged LANG-MOD-PAIR-STRICT-BOUNDARY exact-nine/non-provider pins."
status: ready
owner: spec
size: L
gate: none
depends_on: []
blocks: [LANG-MOD-CANONICAL-PAIR-PACKAGE]
github: null
origin: "Operator ruling to the Steward 2026-08-26: the prelude is the internals->surface BRIDGE; the bootstrapping arm landed for Nat (LANG-MOD-NAT-PROVIDER-INTERFACE, merged b7f73f1d, 30-taxonomy §4 = 'kernel-provided') must EXTEND to compiler/elaborator-provided identities too, because the compiler is equally internal; Pair is the first compiler-provided instance and the STRONGER case because multiple internal Pair definitions should be unified into one canonical identity. Steward released the shaping (evt_6yvwdxw1tmg1x); Architect bound the mechanism (evt_7d0ecgkd8ate3, grounded origin/main@a09878026, no material fork). Steward-filed under [[LANG-MODULE-IMPORT-SYSTEM]], mirroring the Nat spec/build split."
---

> # OPERATOR-RULED + ARCHITECT-BOUND 2026-08-26 — one general internal-provision arm; Pair admitted; exact-nine SUPERSEDED
>
> This EXTENDS the merged Nat bootstrapping arm rather than adding a parallel
> mechanism. The operator ruled (to the Steward) that the prelude is the bridge
> between the language INTERNALS and the surface, and that the arm the Nat
> Decision `dec_1kqwn6hdvn7d2` installed for kernel-provided vocabulary must not
> stop at the kernel: the compiler/elaborator is equally internal, so a canonical
> compiler-installed identity the surface must reach and cannot re-derive
> qualifies the same way. Pair is the first compiler-provided member and the
> stronger case because multiple internal Pair definitions collapse to one
> canonical floor identity.
>
> The Architect bound the mechanism (`evt_7d0ecgkd8ate3`, grounded
> `origin/main@a09878026`, scope confirmed as ONE general arm, no material fork,
> no Pair-only carve-out). This is the SPEC WP: it lands the general rule, the
> reconciliation across the coupled sections, and the conformance-pin CONTRACT.
> It does NOT make Strict green — the elaborator realization (reuse the four
> Pair GlobalIds, split the ten-type / three-companion inventories, flip the
> strict rows) is the build WP [[LANG-MOD-CANONICAL-PAIR-PACKAGE]], which lands
> AFTER this and only through normal exact-SHA gates. This supersedes the merged
> [[LANG-MOD-PAIR-STRICT-BOUNDARY]] exact-nine/non-provider pins in operative
> text, not in a correction banner beside them.

## Objective

Rewrite the prelude membership rule (`30-taxonomy §4`) from the current
bootstrap-identity (kernel-only) wording to ONE general internal-provision arm
whose origin may be the kernel boundary OR the compiler bootstrap; admit Pair as
its first compiler-provided member, unifying the existing compiler-installed Pair
family into the single canonical floor identity; and reconcile every coupled
normative section and conformance pin. The deliverable is normative spec text
plus the conformance-pin contract, not elaborator code.

## The operator ruling (verbatim shape, 2026-08-26)

The prelude is the bridge between the language internals and the surface. What is
in the floor now came from the kernel, but the compiler is also internal and
should also qualify. The pattern is a type defined within the Rust-language
implementation that the surface must use; and here the case is STRONGER than
Nat's because of the multiplicity of internal definitions that should be unified.

## Authorized mechanism (Architect evt_7d0ecgkd8ate3, grounded a09878026)

1. **Canonical identity.** The ONE public Pair identity is the existing
   compiler-installed checked family at
   `crates/ken-elaborator/src/prelude.rs:951-1041`: `Pair`, `mk_pair`,
   `pair_fst`, `pair_snd`, each created once by `declare_def`.
   `Pair` transparently unfolds to non-dependent `Sigma`; the helpers use kernel
   `Term::Pair` / `Term::Proj1` / `Term::Proj2`. Floor realization MUST reuse
   those four existing `GlobalId`s — no package Pair, no copied bodies, no alias,
   no consumer migration to fresh ids. Kernel `Term::Sigma`/`Pair`/`Proj1`/`Proj2`
   (`ken-kernel/src/term.rs:294-303`) are representation/computation authority,
   NOT provider declarations or `GlobalId`s; they are unchanged. "Unification"
   means one public transparent declaration family reflecting the kernel forms —
   not turning kernel syntax into another provider.

2. **General criterion (the §4 rewrite).** Replace the bootstrap-identity wording
   with: a Ken-defined source-resolved type is prelude when, before source can
   speak, the implementation has installed one canonical kernel-checked identity —
   from the kernel boundary OR the compiler bootstrap — the surface independently
   requires that exact identity, and ordinary source cannot reproduce it because
   a declaration allocates a distinct `GlobalId`. The arm stays closed and
   witnessed: presence in the compiler global map is NOT a witness; each member
   has an explicit internal origin and exact pre-source identity; absence or
   substitution fails closed. The signature arm is unchanged. Nat is the
   kernel-provided member; Pair is the compiler-provided member. A companion joins
   a member's floor binding closure only when the admitted type's contract names
   it, its checked type is keyed to that exact type id, and installation reuses
   its exact pre-source id (this bars arbitrary compiler helpers).

3. **Counts and inventories.** The new TYPE floor count is TEN:
   `{Auth, Bool, Char, List, Nat, Option, Pair, ResourceKind, Result, Utf8Error}`.
   The signature inventory remains eight; the internal-provision type inventory is
   exactly `{Nat, Pair}`. Pair's four-name binding surface is
   `{Pair, mk_pair, pair_fst, pair_snd}`. The floor is NOT twelve or thirteen:
   constructors and companion operations are bindings, not type members.
   Transparent Pair has no constructors; its three helpers are a closed companion
   inventory. This distinction governs the build (current
   `PRELUDE_FLOOR_NAMES: [&str; 9]`, `modules.rs:83-145`, conflates the type
   inventory with the constructor-parent source and must be split), but it is
   stated here so the spec's counts are unambiguous.

4. **Strict installation contract (the build realizes; the spec states it).**
   Extend the existing floor path; add no provider registry or fallback lane.
   Capture all four Pair ids from the canonical pre-source environment; admit Pair
   as a type and the helpers as companion bindings; make all four
   immutable/unshadowable; resolve them only through the closed inventory (missing
   or substituted ids reject; never search ambient globals for lookalikes);
   installation performs no `declare_def`, allocates no id, changes no trusted-base
   entry. `Prod` and other compiler conveniences remain non-floor negative
   controls.

5. **Normative flips (operative rules replaced, not banner-corrected).**
   - `30 §4`: the internal-provision arm, the ten-type floor, Pair's companion
     closure.
   - `30 §5` and `50-stdlib/README`: Pair is no longer a package.
   - `33 §3.3`: the exact-ten floor plus companion bindings; non-transitive
     loading unchanged for everything else.
   - `33 §4.3`: compiler-origin floor ids have no source `defined-at` owner;
     re-export preserves ids and creates no owner.
   - `33 §5.3`, `39 §6.1`, `51`: compiler-floor Pair has no source head owner, so
     canonical `Ord Pair` / `DecEq Pair` instances are lawful only in the
     class-owning `LawfulClasses`, exactly like `Ord Nat` — no orphan exception.
   - `34`: keep transparent-Sigma β/η behavior, reclassified as a floor family.
   - `39 §2.0`: admit ten type ids, parent-derived constructors, and the three
     Pair companion ids; remove package-import dependence.
   - Supersede [[LANG-MOD-PAIR-STRICT-BOUNDARY]]'s rejection / exact-nine pins;
     redirect draft [[LANG-MOD-CANONICAL-PAIR-PACKAGE]] to the floor-realization
     build WP.

6. **Surviving boundaries.** No kernel former, syntax, trust entry, registry,
   arbitrary fallback, or alias is added. `Ord Nat` ownership and attached-proof
   obligations survive. CAT's `Order -> LawfulClasses -> Compare -> Pair`
   deferral remains until the Pair FLOOR build lands — spec text alone does not
   make current Strict green. Ambient re-export hygiene remains a separate matter.

## Deliverables

- D1 — the `30 §4` rewrite: the general internal-provision arm (item 2),
  mechanically checkable, with the identity requirement as the anti-bloat guard,
  Nat and Pair as its kernel- and compiler-provided members.
- D2 — the ten-type floor and Pair's three-companion inventory stated exactly
  (item 3), with the type-vs-binding distinction explicit so downstream controls
  do not treat helpers as type formers.
- D3 — the coupled normative flips (item 5) authored as operative text across
  `30 §5`, `33 §3.3/§4.3/§5.3`, `34`, `39 §2.0/§6.1`, `51`, `50-stdlib/README`,
  each replacing the contrary rule rather than annotating it.
- D4 — the conformance-pin CONTRACT: the derivation-path table and strict rows
  updated to the pins in the ACs below (stated as the contract the build must
  satisfy; green-ness is the build WP's AC).
- D5 — the explicit supersession of [[LANG-MOD-PAIR-STRICT-BOUNDARY]]'s
  exact-nine / native-Pair-non-provider pins, in operative text.

## Acceptance criteria

- AC-ARM (general, witnessed, closed) — `30 §4` reads as ONE internal-provision
  arm over kernel-OR-compiler origin, not a Pair carve-out and not a per-type
  list; compiler-global-map presence is explicitly NOT a witness; each member
  carries an explicit internal origin + exact pre-source identity; absence or
  substitution fails closed. The signature arm is unchanged.
- AC-COUNT (exact ten types, exact three companions) — the type floor is the
  closed TEN `{Auth, Bool, Char, List, Nat, Option, Pair, ResourceKind, Result,
  Utf8Error}`; the internal-provision type inventory is exactly `{Nat, Pair}`;
  Pair's companion inventory is exactly `{mk_pair, pair_fst, pair_snd}`; the text
  states the floor is TEN types (not twelve/thirteen) and that companions are
  bindings, not type members.
- AC-IDENTITY (reuse, no new identity) — the spec requires realization to reuse
  the four existing compiler-installed Pair `GlobalId`s (prelude.rs:951-1041);
  it forbids a package Pair, an alias, a second `data Pair`, and any consumer
  migration to fresh ids. Kernel `Term::Sigma`/`Pair`/`Proj1`/`Proj2` are named
  as representation authority, unchanged, not a provider.
- AC-FLIP (operative supersession, no orphan banners) — every contrary operative
  rule in item 5 is REPLACED: `30 §5`/`50-stdlib` no longer class Pair as a
  package; `33 §4.3`/`§5.3`/`39 §6.1`/`51` give compiler-floor Pair no source head
  owner with canonical `Ord Pair`/`DecEq Pair` lawful only in `LawfulClasses`
  (no orphan exception); `34` reclassifies transparent-Sigma β/η as a floor
  family; the merged [[LANG-MOD-PAIR-STRICT-BOUNDARY]] exact-nine/non-provider
  pins are superseded in operative text, not annotated.
- AC-PINS (conformance contract) — the conformance pins are authored: the four
  strict bare-name rows (`Pair`/`mk_pair`/`pair_fst`/`pair_snd`) flip to ACCEPT
  asserting the exact existing ids; zero declaration/allocator/trusted-base
  movement is asserted; same-shaped/different-id negatives are kept and
  same-spelling local collision rejects for every floor binding; the derived
  signature-eight, exact internal `{Nat, Pair}`, and their ten-type union are
  derivable; the three companions and their canonical Pair/kernel-former
  references are asserted; positivity is preserved through floor Pair; `Prod`
  addition breaks closure (negative control); package-import / later-package
  assertions are retired, with re-export retained only as identity-preserving
  republication.
- AC-SURVIVE (boundaries intact) — no kernel former/syntax/trust/registry/alias
  added; `Ord Nat` ownership + attached-proof obligations survive; the CAT
  `Order -> LawfulClasses -> Compare -> Pair` deferral is stated as remaining
  until the Pair FLOOR build lands; the spec explicitly notes spec text alone
  does not make current Strict green.
- AC-NO-REGRESSION — the spec renders/builds; whole-suite green in CI; local
  targeted `-p` only, never `--workspace`. (This WP changes no crate behavior;
  the strict-green flip is the build WP's regression criterion.)

## Reviewers

spec-author + spec-leader author the normative text and the pin contract.
Architect reviews mechanism fidelity (ONE general internal-provision arm keyed by
internal origin + exact pre-source identity, not a Pair carve-out; the four
existing Pair `GlobalId`s reused with no package/alias/second identity; the
ten-type / three-companion split exact; kernel formers unchanged; the coupled
flips replace operative rules; boundaries survive). conformance-validator
independently reproduces the counts and the pin contract (signature-eight,
internal `{Nat, Pair}`, ten-type union, three companions, the four ACCEPT rows +
exact ids, the negatives/mutations, positivity through floor Pair, the `Prod`
negative control) and confirms the supersession of the exact-nine pins is
operative, not a banner. A material fork hard-stops to the Architect, then the
operator.

## Capability tier

T1 — a soundness-bearing normative rule change that generalizes a membership
criterion and reopens a landed exact-floor ruling, reviewed on the
provenance/identity argument (internal origin + exact pre-source identity, the
type-vs-binding split, one reused identity), not a mechanical diff. Size L.

## Sequencing

Lane-2 (language), under [[LANG-MODULE-IMPORT-SYSTEM]]; mirrors the Nat split
([[LANG-MOD-NAT-PROVIDER-INTERFACE]] spec WP ->
[[LANG-MOD-NAT-FLOOR-REALIZATION]] build WP). This SPEC WP lands the general
rule, the reconciliation, and the pin contract; the build WP
[[LANG-MOD-CANONICAL-PAIR-PACKAGE]] (redirected from its package plan to
floor realization) realizes the split inventories and flips the strict rows AFTER
this lands, gating foundation [[CAT-GCD-REFACTOR]] and the CAT
`Order/LawfulClasses/Compare/Pair` re-entry. No build realization is authorized
before normal exact-SHA gates. The Spec team was compacted at this WP boundary
(COORDINATION §15) before the authoring kickoff.
