---
id: LANG-MOD-NAT-PROVIDER-INTERFACE
title: "Nat's canonical home by prelude-floor membership: amend the GENERAL prelude membership rule (30-taxonomy §4) to a bootstrapping criterion that admits kernel-provided vocabulary the surface must reach and cannot re-derive with identity, with the coupled normative amendments (30 §5, 33 §3.3, 39, 50-stdlib) and identity/strict-accept/zero-allocation conformance pins. The Component B Nat prerequisite (spec WP)."
status: merged
owner: spec
size: L
gate: none
depends_on: []
blocks: [LANG-MOD-NAT-FLOOR-REALIZATION, LANG-MOD-CATALOG-COMPLETENESS]
github: null
origin: "Component B hard stop (language-implementer evt_4757hgk2t2mj6 / language-leader evt_71apgcrce8fqf lineage), ruled a Spec+build prerequisite by the enclave (spec-author evt_33bwgcx226bxv, spec-leader evt_7nvtrx1fs6wf0, Architect deferred the mechanism to Spec evt_22r45y0x8nzbh). Material mechanism/scope escalated to the operator as Decision dec_1kqwn6hdvn7d2 and RESOLVED there 2026-08-25: the prelude membership rule itself is the defect; realize Nat by prelude-floor membership, NOT the provider-registry. Steward-reframed under [[LANG-MODULE-IMPORT-SYSTEM]] to the ruled approach."
---

> # LANDED 2026-08-25 (b7f73f1d, "spec: add Nat nine-floor provider contract")
>
> The spec WP is MERGED. The strict floor closed set landed as NINE names, not
> the four this node first drafted: the enclave closed the executable population
> by walking every `Decl::Primitive` signature and found the signature arm names
> `{Auth, Bool, Char, List, Option, ResourceKind, Result, Utf8Error}` (eight —
> `bytes_at`/`bytes_slice` name `Option`, `bytes_decode` names `Result`/
> `Utf8Error`, `Cap` names `Auth`, `Resource` names `ResourceKind`); the
> bootstrap-identity arm adds exactly `Nat`. Landed `30-taxonomy §4`: "today's
> Ken-defined surface floor is the closed set `{Auth, Bool, Char, List, Nat,
> Option, ResourceKind, Result, Utf8Error}` ... floor installation reuses all
> nine existing `GlobalId`s and allocates nothing." The Steward ruled the
> four->nine as mechanical closure of the resolved operator ruling
> dec_1kqwn6hdvn7d2 (zero-TCB, surface-preserving); flagged to the operator as an
> FYI on return. Wherever the coupled-amendments section below still writes the
> four-set `{Bool, Char, List, Nat}`, the LANDED set is the nine above. The build
> half [[LANG-MOD-NAT-FLOOR-REALIZATION]] realizes the full nine-name floor.

> # OPERATOR-RULED — provider-registry mechanism SUPERSEDED
> by prelude-floor membership
>
> Decision dec_1kqwn6hdvn7d2 is RESOLVED. The operator ruled that the prelude
> membership rule (`30-taxonomy §4`) is WRONG as written, and that Nat's home is
> a bootstrapping problem, not a new compiler mechanism. This node is re-framed
> from its prior "compiler-realized package-provider interface / closed registry
> / head-ownership arm" mechanism (now DISSOLVED) to the ruled approach:
> **amend the general membership rule, and admit the existing kernel `{Nat, Zero,
> Suc}` into the strict prelude floor, reusing the kernel identity.** Nat becomes
> ambiently present like `Bool`. The prior registry/provider-path/`Data.Numeric.
> Nat` framing and the §5.3/§6.1 head-ownership sub-mechanism are NOT built.
> This is the SPEC WP; the elaborator realization is
> [[LANG-MOD-NAT-FLOOR-REALIZATION]].

## The operator ruling (dec_1kqwn6hdvn7d2, verbatim scope)

The prelude is the essential BOOTSTRAP interface — the vocabulary that cannot
otherwise be defined, which enables the language kernel to be useful by the
language surface. It is not a platonic minimal set derived from primitive
signatures. Nat is kernel-provided, unreachable from a strict surface scope, and
not re-derivable with the same identity — a bootstrapping problem — ergo Nat
belongs. Amend the GENERAL rule (not a Nat carve-out) so later judgements are
handled identically. The defect clause is **"and is not already provided by the
kernel"** (`30-taxonomy §4`, the normative membership rule): it excludes exactly
the kernel-provided vocabulary the surface must reach. The amended criterion
INCLUDES kernel-provided vocabulary the surface must reach and cannot re-derive
with identity; Nat (with `Zero`/`Suc`) is its first member. Realize it by
admitting the EXISTING kernel `{Nat, Zero, Suc}` into the strict resolution
floor — reuse the kernel identity (no second family), zero `trusted_base()`
delta, fail-closed on non-canonical origin.

## Why this is sound, not a floor-bloat regression

The current rule (`30-taxonomy §4`, lines 99-105 at main) admits a type **iff**
it is "named in a built-in primitive's type signature, **and is not already
provided by the kernel**." The amendment adds a second, equally-checkable
admission arm — the bootstrapping arm — and it has a standing precedent one
layer down: the kernel's own closed `is_prelude` set `{Top, Bottom, tt}` (`64
§1`) is "Ken-vocabulary excluded from `trusted_base()` yet always present." The
surface prelude floor gains the identical shape for `Nat`: kernel-checked, out
of `trusted_base()`, always present, closed. The criterion stays mechanically
checkable (it is not a catch-all): a member must be (a) kernel-provided with a
canonical identity, (b) reachable-required by the surface, and (c) not
re-derivable at the surface with that same identity (a fresh `data Nat` mints a
SECOND family — structural similarity is not canonical identity). The current
"bloat vector" guard (a prelude type no primitive names) is preserved for the
signature arm; the bootstrapping arm's analog guard is the identity requirement.

## The coupled normative amendments (enclave authors the exact text)

The membership-rule amendment forces coupled edits. The enclave owns the precise
normative wording; the Architect is the soundness reviewer. Reconcile at least:

1. `30-taxonomy §4` — the membership rule itself: add the bootstrapping arm;
   the closed derived set is the nine-name floor `{Auth, Bool, Char, List, Nat,
   Option, ResourceKind, Result, Utf8Error}` (the eight signature-arm names + the
   bootstrap-arm `Nat` with constructors `Zero`/`Suc` reachable as its data
   constructors). State the checkable criterion for the new arm. (LANDED as nine;
   this item first drafted the four-set — see the top banner.)
2. `30-taxonomy §5` (standard-package tier) and `50-stdlib/README §1` — these
   currently classify `Nat` as **package-tier, not prelude**. That classification
   was the stated reason floor-membership was "foreclosed"; it is now overturned.
   Nat moves from package-tier to prelude-tier; reconcile both statements.
3. `33 §3.3` — the closed floor set the strict scope contains: the implemented
   `{Bool, Char, List}` becomes the landed nine `{Auth, Bool, Char, List, Nat,
   Option, ResourceKind, Result, Utf8Error}` (Nat + its constructors, plus the
   five signature-arm names the implemented floor under-counted). (LANDED as
   nine; first drafted as the four-set.)
4. `39` and `33 §4.3/§5.3` — the `Ord Nat` / instance-provenance coupled
   question. With Nat prelude-owned (kernel identity, no provider module), decide
   and state who head-owns `instance Ord Nat` for `§4.3` provenance / `§5.3`
   orphan checking. The prior node routed this through a provider-path head-owner
   map; that map is dissolved. This sub-question must be answered, not dropped —
   it is the reason `Ord Nat` was coupled to Nat in dec_1kqwn6hdvn7d2. If the
   enclave finds prelude-ownership does not lawfully carry `Ord Nat` under the
   current orphan contract, surface that to the Steward (it may re-scope the
   Order half), do not silently descope it.

No `32` grammar edit (no new surface syntax — Nat is ambient, like Bool).

## Deliverables

- D0 (buildability probe FIRST, the M6/WP-2 pattern). Confirm the existing
  kernel `{Nat, Zero, Suc}` identities are reachable to admit into the strict
  floor at the seam WP-2 established (`install_prelude_floor` /
  `capture_strict_builtin_names` / `PRELUDE_FLOOR_NAMES`, `modules.rs`), and that
  admitting them reuses the kernel GlobalIds rather than minting new ones. A
  grounded question the probe answers before the normative flip.
- D1. The coupled normative amendments (section above), authored as exact spec
  text with the derivation-path/`taxonomy` conformance table updated to the new
  closed set.
- D2. Conformance/acceptance pins (section below), specified so the build WP
  ([[LANG-MOD-NAT-FLOOR-REALIZATION]]) has an executable target.

## Acceptance criteria (spec-side; the build WP realizes them)

- AC-1 (identity). Strict resolution of `Nat`/`Zero`/`Suc` in a root-loaded unit
  yields the EXACT pre-existing kernel GlobalIds (identity assertion, not text /
  not structural equality). A fresh `data Nat = Zero | Suc Nat` in a unit is a
  distinct family and does NOT satisfy the identity.
- AC-2 (strict-ACCEPT — the flipped control). The landed strict-Nat REJECTION
  control (`crates/ken-elaborator/tests/lang_mod_catalog_realization.rs:81-117`)
  pins the CURRENT behaviour (Nat rejected under Strict). When Nat joins the
  floor this control inverts: it must be re-authored to assert strict ACCEPT +
  the AC-1 identity. Its red on the build branch is the port landing, NOT a
  regression — flag it as such so QA does not read the flip as a break.
- AC-3 (closed floor, not catch-all). A non-floor, kernel-provided-but-not-
  surface-required name (or a convenience global) still rejects strict — the
  bootstrapping arm admits Nat and nothing the criterion does not name.
- AC-4 (zero trust / zero allocation). Admitting Nat adds zero `trusted_base()`
  entries and allocates no new `Decl`/`GlobalId`; the flat-Σ pin stays green.
- AC-5 (Ord Nat provenance). Per section-4 resolution: the one canonical
  `instance Ord Nat` resolves against the exact native Nat identity; a
  byte-identical second-family instance rejects. (Scope may narrow to the Order
  half if the enclave's §4 finding routes it back to the Steward.)
- AC-NO-REGRESSION. Whole-suite green in CI; local targeted `-p` only.

## Reviewers

Architect (soundness: the amended rule must admit exactly the bootstrapping
vocabulary and preserve the closed-set / no-catch-all discipline; the identity
reuse must not grow the TCB) + conformance-validator (the derivation-path table
and the strict accept/reject discriminators with the new closed set).

## Capability tier

T1 (a soundness-bearing amendment to a normative membership rule with coupled
cross-section reconciliation and an instance-provenance sub-question; review
turns on an argument, not a diff). Size L.

## Sequencing

Release FIRST (spec-surface); the build WP [[LANG-MOD-NAT-FLOOR-REALIZATION]]
depends on it. Together they are the prerequisite to
[[LANG-MOD-CATALOG-COMPLETENESS]]'s Nat criterion (AC-B5a): Component B's Nat
home, strict-caller migration, and whole-catalog strict-green stay STOPPED until
the build WP lands. Component B's OrdResult home, the homeless census, and Gcd's
non-Nat reuse proceed as the authorized partial meanwhile (COORDINATION §10-).
This chain in turn gates [[CAT-GCD-REFACTOR]]'s Nat import (lane 3).
