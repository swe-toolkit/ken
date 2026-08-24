---
id: LANG-MOD-NAT-PROVIDER-INTERFACE
title: "Compiler-realized package-provider interface for the canonical Nat home: a closed registry binding one designated module path to the existing kernel-checked {Nat, Zero, Suc} identities under Strict, with the coupled 30/33/39 normative amendments and identity/strict-rejection/zero-allocation conformance pins. The Component B Nat prerequisite."
status: draft
owner: spec
size: L
gate: none
depends_on: []
blocks: [LANG-MOD-CATALOG-COMPLETENESS]
github: null
origin: "Component B hard stop (language-implementer evt_4757hgk2t2mj6 / language-leader evt_71apgcrce8fqf lineage), ruled a Spec+build prerequisite by the enclave: spec-author evt_33bwgcx226bxv, spec-leader evt_7nvtrx1fs6wf0, Architect deferred the mechanism to Spec evt_22r45y0x8nzbh. Steward-filed under [[LANG-MODULE-IMPORT-SYSTEM]]; material mechanism/scope escalated to the operator as Decision dec_1kqwn6hdvn7d2."
---

> # DRAFT — release HELD on operator Decision dec_1kqwn6hdvn7d2
>
> This node introduces a NEW compiler mechanism + normative spec sections
> beyond the original module/import decomposition, on the critical path to the
> foundation catalog trial ([[CAT-GCD-REFACTOR]]). Per COORDINATION §3 the
> material mechanism/scope is the operator's call, escalated as Decision
> dec_1kqwn6hdvn7d2 (build the registry vs descope Nat from the trial;
> Steward recommendation: build). Do NOT release/decompose into WPs until the
> operator resolves it. The enclave ruling below is durable regardless of that
> resolution — it is the home for the ruling, not the convo thread.

## The gap (enclave-ruled, grounded at 3a7114cf7 / re-grounded 6cadce775)

The catalog cannot go whole-catalog strict-green while `Nat` has no
strict-checkable, identity-preserving public home. At current main NO mechanism
provides one for a NON-FLOOR native inductive:

- Under STRICT, `export Nat, Zero, Suc` has no lawful source: `Nat` is
  non-floor native, unimported, non-ambient; the facade unit's own strict scope
  does not contain it (`33 §3.3`, closed floor is exactly {Bool, Char, List}).
- Legacy is not a provider a Strict dependency may traverse; recursive
  `load_unit` passes one coherent `ResolutionMode` throughout
  (`roots_resolution_mode` forbids mixing).
- A fresh `data Nat = Zero | Suc Nat` allocates a SECOND family and constructor
  identities; structural similarity is not canonical identity. The Architect's
  ES2-Bool native-inductive-recognition lean is FORECLOSED by this.
- Adding `Nat`/`Zero`/`Suc` to the ambient strict vocabulary or the prelude
  floor contradicts the closed-floor/package contract (`Nat` is normatively
  package-tier, not prelude — `50-stdlib/README §1`).

The as-built path matches the law: `register_prelude` creates the existing
kernel-checked `Nat`/`Zero`/`Suc` before source; `capture_strict_builtin_names`
admits trusted native names + closed-floor constructors, not `Nat`;
`resolve_ref` rejects `Nat` under Strict; the landed strict-Nat control already
pins the rejection (`lang_mod_catalog_realization.rs:81-117`).

## The ruled mechanism (spec-author evt_33bwgcx226bxv, spec-leader evt_7nvtrx1fs6wf0)

A narrowly-scoped compiler-realized package-provider interface. It must specify:

1. A closed registry binds an exact designated module path to existing, already
   kernel-checked canonical identities. For Component B the inventory is exactly
   `{Nat, Zero, Suc}`, and the constructor entries are validated as children of
   that exact `Nat` family.
2. Those bindings enter ONLY that provider unit's local scope under Strict —
   never arbitrary units' ambient scope. The provider source then uses the
   existing checked declaration `export Nat, Zero, Suc`; ordinary import/re-export
   carry the same identities thereafter. No new surface syntax.
3. Loading the provider allocates no `Decl` or `GlobalId`, changes no
   `trusted_base()` entry, and does not relax the one-`ResolutionMode` rule.
   Missing registry entries, wrong declaration kinds, mismatched constructor
   parents, duplicate public origins, or unregistered modules fail closed
   before publication.
4. The taxonomy states explicitly that early compiler realization for internal
   use does NOT promote a Ken-definable package to built-in/prelude status:
   availability remains explicit-import-only through its one defining public
   interface.
5. HEAD-OWNERSHIP ARM (spec-author evt_6qqa946cnv0ja, spec-leader
   evt_5z3tm4dfakwvv). The one registered provider path `Data.Numeric.Nat.Nat`
   is the SOLE surface defined-at/head-owner of the existing Nat identities for
   `33 §4.3` provenance and `§5.3` orphan checking (the compiler stays the
   realization origin; import/re-export never transfers ownership). This makes
   `instance Ord Nat` orphan-valid ONLY head-side in the provider module;
   `Data.Numeric.Nat.Order` is a reader-facing re-export surface that carries
   the one canonical instance under `§5.5.1` and mints no wrapper/second
   registration. Extends the normative delta with `33 §5.3` (a registered
   provider is the sole head-owner) and `39 §6.1` (instance registration
   consults the closed provider-owner map before the unchanged overlap rule),
   plus conformance: provider-local `instance Ord Nat` accepts against the exact
   native ID; byte-identical Order-local and unregistered-facade instances
   reject `OrphanInstance`; Order re-export carries the one instance/provenance;
   a names-only import without admission rejects direct dispatch; the dictionary
   adds one transparent declaration and zero `trusted_base()` delta. CONSEQUENCE
   for the operator Decision: this is the build arm of dec_1kqwn6hdvn7d2 — if the
   provider arm is DECLINED, `Ord Nat` is descoped with Nat (Order cannot
   lawfully publish it under the current orphan contract).

## Deliverables (decompose at release, after the operator Decision)

- Spec-surface WP (enclave): coupled normative amendments to `30-taxonomy §5`,
  `33 §3.3/§4.3`, and `39 §2.0`, plus conformance/acceptance pins — strict
  provider + selective-consumer accepts and observes the pre-existing three IDs;
  the identical facade at an UNREGISTERED module and a bare strict consumer
  reject; provider loading adds zero declarations/trust; a redeclaration cannot
  satisfy the identity assertion. No `32` grammar edit.
- Build WP (language ring): the registry realization in the elaborator/loader,
  the designated `Data/Numeric/Nat` provider source, and the roots-loader
  controls. Cross-cutting invariant: zero `trusted_base()` delta, flat-Σ pin
  stays green.

## Sequencing

Prerequisite to [[LANG-MOD-CATALOG-COMPLETENESS]]'s Nat criterion (AC-B5a):
Component B's Nat home, strict-caller migration, and whole-catalog strict-green
stay STOPPED until this lands. Component B's OrdResult home, the homeless
census, and Gcd's non-Nat reuse proceed as the partial increment meanwhile
(COORDINATION §10-). This node in turn gates [[CAT-GCD-REFACTOR]]'s Nat import.
