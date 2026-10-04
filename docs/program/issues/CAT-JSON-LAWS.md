---
id: CAT-JSON-LAWS
title: "json_size is checked only by one nested fixture. Prove its constructor equations over the existing six-constructor Json, so the fold is specified by theorems rather than by a test"
status: merged
owner: foundation
size: S
tier: T2
gate: none
depends_on: []
blocks: []
github: null
origin: "One of the seventeen proof-backfill follow-ons named by docs/program/CATALOG-PROOF-COMPLETENESS-SURVEY.md (row Data/Serialization/Json.ken.md), under operator ruling 2026-09-13 / PRINCIPLES #16. Framed by the Steward 2026-10-04 as L3's successor to CAT-VECTOR-DEFERRED-LAWS. Of the three survey follow-ons still unframed, CAT-CONSOLE-TEXT-LAWS was rejected 2026-09-20 (see CAT-PROPERTY-LAWS origin) and CAT-SYSTEM-RESOURCE-LAWS concerns withResource, which lives in the prelude rather than the package. Measured at origin/main ca2855abd."
---

# Json size laws

## Objective

`Data/Serialization/Json.ken.md`'s private `json_size` fold is stated by
theorems that determine it on every constructor, with no new trust.

## Settled inputs (measured at `ca2855abd`)

- `json_size` (`:59`) is private structural recursion. The four leaves give
  `Suc Zero`. `JsonArray values` gives `Suc` of an inner match over
  `values`, and `JsonObject members` gives `Suc` of the same shape over
  `members`. Each `Cons` step is `json_nat_add` (`:53`, recursion on the
  left argument) of the head's recursive result and the tail's.
- The tail's recursive result is the inner match on the tail, not
  `json_size (JsonArray rest)`, which is one `Suc` larger. So the cons
  equation needs `json_nat_add m (Suc n) = Suc (json_nat_add m n)`, by
  induction on `m`. No such lemma exists in the package.
- An object member's recursive result is taken over the whole
  `Pair String Json` (`:112`). Keys do not count.
- Vocabulary: equality is `Equal`, pairs are `mk_pair String Json k v`.
  The package already has a public surface. `json_size` is private, so its
  laws are private too. Do not make anything new `pub`.
- The survey's "recursive decoder" is a test-only probe
  (`ds9_json_codec_acceptance.rs:656`), not a package function. It is out of
  scope.
- The evidence being replaced is
  `ds9_json_codec_acceptance.rs::json_size_consumes_array_and_pair_nested_object_results`.
  That test stays.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

Private theorems in the package's §4, over the existing definitions:

1. Leaves: `json_size` of each of the four leaf constructors is `Suc Zero`.
2. `json_size (JsonArray (Nil Json)) = Suc Zero`, and
   `json_size (JsonArray (Cons Json c r)) =
   json_nat_add (json_size c) (json_size (JsonArray r))`.
3. `json_size (JsonObject (Nil (Pair String Json))) = Suc Zero`, and
   `json_size (JsonObject (Cons (Pair String Json) (mk_pair String Json k v) r))
   = json_nat_add (json_size v) (json_size (JsonObject r))`.
4. The `json_nat_add` successor lemma the cons cases need.

Together these determine `json_size` uniquely. Update the package's §4 prose
and §7 validation evidence to cite them.

## Acceptance

- **AC-1.** The package checks with the new theorems. `trusted_base()` is
  unchanged, and the public API list in §7 is unchanged. A focused test
  resolves each theorem as a registered private kernel global with its exact
  checked statement.
- **AC-2 (falsifiers).** M1: make the object branch count keys (add one
  `Suc` per member). The object cons theorem reddens. M2: change the array
  `Nil` arm from `Zero` to `Suc Zero`. The array nil theorem reddens. Each
  mutation is restored byte-identically.

## Stop conditions

- Stating or checking the cons equation hits an elaborator or kernel gap
  (for example in nested recursion through `List` or `Pair`): stop to the
  Architect with the failing declaration.
- The package fails the default 2 MiB test worker after the addition.

## Closeout

Merged `4d22f8a5f` from exact `e7f091129` (PR #4499, run 37237871485).
Foundation QA `evt_6vrek0z1m38f5`, Architect `evt_2m3njc6gmvpvk`, Decision
`dec_69rnqc426wxrv`.

- Nine private checked equations determine `json_size`: the four leaves, the
  array and object nil and cons laws, and the `json_nat_add` successor lemma.
- The proofs import the zero-trust `Core.Logic.Transport.{cong, sym}`.
  `trusted_base()` and the public API are unchanged.
- The first candidate, `77b6ee7ea`, failed CI on the two Tier-E consumer
  pins in `cat_tier_e_json_import.rs`. The respin updated them: the import
  ledger gains `cong` and `sym`, and the external closure reads Json's
  constructors from the checked family as local.

Successor on L3: `CAT-SYSTEM-RESOURCE-LAWS`.
