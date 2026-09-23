---
id: CAT-CONFIGURATION-DECODER-LAWS
title: "prove the required-field agreement the decoder actually has, and characterize the optional lane it does not -- env_config_validation discards the validation payload and recomputes values in a second independent traversal whose None branch emits an empty-Bytes placeholder; the REQUIRED lane is guarded (schema_check_presence rejects SchemaRequired), so the reachable placeholder is an ABSENT OPTIONAL field, whose empty Bytes is indistinguishable from a present-but-empty value"
status: merged
owner: foundation
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "One of the seventeen proof-backfill follow-ons named by docs/program/CATALOG-PROOF-COMPLETENESS-SURVEY.md, under operator ruling 2026-09-13 / PRINCIPLES #16. Framed by the Steward 2026-09-21 as L3's successor AFTER measuring the package, NOT adopted from the survey's one-line recommendation -- the survey proposes four parallel targets (lookup, accumulation, provenance, raw-Bytes) and does not name the independent-traversal agreement, which is the one that carries a wrong answer if it fails. Deferred until now because its precursor is CAT-SCHEMA-LAWS, which merged at 47d770216; framing it earlier would have grounded a frame on an unmerged commit. All current-code facts measured at origin/main 8472b78aa2a2ef7bb9216d6ea518606c2b0e2393."
---

# Decoder laws: the required-field agreement, and the optional lane it excludes

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## 1. Objective

State and prove, over the existing decoder representation and with no new
trust, that a `Valid` result from either decoder entry point returns exactly
the bytes found in `entries` **for every REQUIRED field**, in field order, and
state as its own law what happens in the optional lane, where the `None` branch
of `env_config_values` IS reachable.

## 2. Settled inputs. Do not re-derive.

Facts below were measured at `8472b78aa`; the presence-split facts and the
refutation at `d5d7e6299`.

The package is `catalog/packages/Application/Configuration/Decoder.ken.md`,
187 lines. It declares roughly twenty functions and **zero proofs**: the
survey's "the proof suite is absent" is confirmed by measurement, not assumed.

The structure that motivates this node:

- `decode_environment_entries schema entries` calls `env_config_validation`
  with `schema_fields schema`, `entries`, and
  `schema_validate EnvConfigOrigin Bool (environment_field_check entries) schema`.
  `decode_config_entries` is identical but for `config_field_check`.
- `schema_validate origin value inspect schema` is definitionally
  `schema_validate_fields origin value inspect (schema_fields schema)`. So
  **both traversals run over the same field list and the same `entries`.**
- `env_config_validation` binds the `Valid values` payload and then **discards
  it**, returning `env_config_values fields entries` instead. The values are
  recomputed by a second, independent traversal.
- `env_config_values` looks the field up again and, on `None`, emits
  `Cons Bytes (list_to_bytes (Nil UInt8))` -- an **empty-Bytes placeholder**,
  not an error. Nothing in the package connects `Valid` to that branch being
  unreachable.
- `environment_field_check`/`config_field_check` reduce to
  `env_config_field_check`, which accepts when
  `env_config_lookup (bytes_encode (schema_field_name field)) entries` is
  `Some`, and on `None` yields `env_config_missing_field`.
- **`env_config_missing_field` does NOT reject unconditionally.** It calls
  `schema_check_presence ... (schema_field_presence field)`, and
  `schema_check_presence` at `Schema.ken.md:97` splits:
  `SchemaRequired ↦ schema_field_reject`, `SchemaOptional ↦
  schema_field_accept optional_value` with `optional_value = True`. **So an
  absent REQUIRED field makes validation `Invalid` and no decode happens; an
  absent OPTIONAL field is ACCEPTED and reaches the placeholder.**
- ⇒ **`valid_coverage`'s accepted `True` is not evidence of lookup success.**
  It is satisfied by "present and found" and by "optional and absent" alike,
  so it cannot carry a universal lookup implication. Measured by
  foundation-leader at `d5d7e6299` with one `SchemaOptional`/`SchemaBytes`
  field and `entries = Nil`, green under private `Refl` witnesses at both
  entry points, reproduction at `evt_7wgfandr6fd2e`; independently confirmed
  by the Steward against `Decoder.ken.md:95-101,118-128` and
  `Schema.ken.md:90-100`.

The precursor is landed and is the lever: `CAT-SCHEMA-LAWS` merged at
`47d770216` and published `schema_validate_fields::valid_coverage`, which
states that a `Valid values` result implies, for every in-range index `i`,
that `inspect field_i` accepted and `nth i values` is the aligned accepted
value. Instantiating it at `inspect = environment_field_check entries` is the
intended route; this node is not expected to re-prove coverage from scratch.

## 3. Deliverable

Attached `pub proof` terms in
`catalog/packages/Application/Configuration/Decoder.ken.md`, on the exported
subjects, plus the one production visibility change they require.

**A proof travelling with its subject's selector does not make the names in
its STATEMENT resolvable at a client.** Those must be published separately.
AC-1 states the agreement against `env_config_lookup`, so that authority must
be nameable.

Authorized by the Architect at `evt_44c2n2qh6y1c1`, exhaustively:

- `fn env_config_lookup` becomes `pub fn env_config_lookup`. Preserve its
  name, type, body, declaration position, and canonical identity. This is a
  visibility change, not a body change.
- `env_config_lookup_choice`, `env_config_entry_key`, and
  `env_config_entry_value` stay private.
- No source `export` declaration changes: `pub` contributes directly to the
  interface under `spec/30-surface/33-declarations.md §4.1`.
- The published population is exactly EIGHT identities -- four direct names
  (`decode_process_environment`, `decode_config_entries`, `env_config_help`,
  `env_config_lookup`) and four attached laws
  (`decode_process_environment::{required_lookup,optional_absence}` and
  `decode_config_entries::{required_lookup,optional_absence}`). Attached names
  are selector-only and do not become ambient bare names.
- The Tier-E controls in `crates/ken-elaborator/tests/cat_tier_e_decoder_import.rs`
  MUST change: remove `env_config_lookup` from the private-refusal roster,
  positively import and use its canonical identity, and pin that exact
  4-direct/4-attached population.
- `decoder_checked_provider_and_schema_closure_is_exact` is an exact ledger,
  not a prohibition. Section 2 requires instantiating `valid_coverage`, so the
  provider growth is a consequence of the mandated deliverable. Updating it to
  its new true value is part of this node. The authorized additions are
  `MkSchemaField`, `SchemaFieldAccepted`, `SchemaOptional`, `SchemaPresence`,
  `SchemaRequired`, `SchemaValueShape`, `schema_validate_fields`,
  `schema_validate_fields::valid_coverage`, and `Data.Collections.Derived.nth`.

Add no wrapper, second traversal, proposition family, datatype, postulate,
primitive, or trusted-base entry.

### 3a. One authorized behavior-preserving refactor

Ruled at `evt_7rd3bv9p81acm`. The required law is semantically independent of
the optional placeholder, but its CERTIFICATE was not: a required field may
follow absent optional fields, and the old `env_config_values_tail` certificate
reconstructs both lookup arms, so the placeholder spelling entered AC-1's proof
dependency closure. Factor the placeholder choice BELOW the list spine:

```ken
fn env_config_value_or_empty (choice : Option Bytes) : Bytes =
  match choice {
    Some value ↦ value;
    None ↦ list_to_bytes (Nil UInt8)
  }

fn env_config_values
      (fields : List SchemaField) (entries : List (Prod Bytes Bytes))
    : List Bytes =
  match fields {
    Nil ↦ Nil Bytes;
    Cons field rest ↦
      Cons Bytes
        (env_config_value_or_empty
          (env_config_lookup (bytes_encode (schema_field_name field)) entries))
        (env_config_values rest entries)
  }
```

Use that exact helper name. Do NOT introduce a second helper or an alternate
carrier. This is a refactor, not the presence repair: output bytes, field
order, first-match lookup, shadowing, issue behavior, both entry points, and
the second traversal are all unchanged. The helper is private, so the public
population stays four direct plus four attached identities. Add it to the
external private-refusal inventory; that private roster grows 17 -> 18. The
helper is deleted together with `env_config_values` by the already-draft
`CAT-CONFIGURATION-DECODER-PRESENCE-CARRIER`.

The resulting proof boundary is structural, and it is the deliverable:

1. `env_config_values_tail` proves ONLY that `nth (Suc i)` crosses the outer
   `Cons`. It is `Refl` or a generic `nth`/`Cons` lemma. Its type and body must
   not mention `None`, the empty placeholder, or either arm of
   `env_config_value_or_empty`.
2. The required head bridge uses only the `Some` equation of
   `env_config_value_or_empty` plus the supplied lookup equality. Its
   dependency closure contains no `None` equation, no empty-placeholder
   literal, and no optional-law helper.
3. The optional head bridge alone uses the `None` equation and the
   empty-placeholder endpoint. The two public `optional_absence` laws depend on
   it; the two public `required_lookup` laws do not.
4. Do not replace `env_config_values` with the validation payload, change a
   result type, widen the public surface, or add trust.

Moving the match underneath the always-present `Cons` makes list position
independent of head payload. A proof-only rewrite against the old body cannot
achieve this separation, because its motive must reconstruct the neutral lookup
match.

## 4. Acceptance criteria

**AC-1 -- the required-field agreement, at both entry points.** For
`decode_environment_entries schema entries` returning `Valid values`: for every
index `i` in range of `schema_fields schema` **whose `schema_field_presence` is
`SchemaRequired`**, `env_config_lookup (bytes_encode (schema_field_name
field_i)) entries` is `Some v_i` and `nth i values = Some v_i`. The bytes are
the entry's own bytes with no re-encoding, which is where raw-`Bytes`
preservation is discharged. The same law for `decode_config_entries`. This is
the safety property: it is what makes a required field's value trustworthy.

**AC-2 -- the optional lane is stated, not left implicit.** A law that says
what the placeholder means: for an index `i` whose presence is `SchemaOptional`
and whose lookup is `None`, a `Valid` result has `nth i values = Some
(list_to_bytes (Nil UInt8))`. State it as a published law rather than a comment,
because it is the only thing that makes the conflation visible to a reader: an
absent optional field and a present field holding empty bytes produce the same
element, and nothing in the returned `List Bytes` distinguishes them. **This
law characterizes the PREDECESSOR and is not a permanent semantic promise.**
The Architect has ruled the placeholder a representation defect
(`evt_43cf1x0808egr`); `CAT-CONFIGURATION-DECODER-PRESENCE-CARRIER` replaces
the carrier with `List (Option Bytes)` and retires this law atomically. Write
it as a true statement about today's behavior, not as an endorsement that
empty means absent.

**AC-3 -- the split is measured by a two-observation campaign over ONE
identical mutation.** AC-3 is preserved, not weakened: weakening it would hide
the certificate coupling rather than measure it. It is population-side.

1. Establish the full unmutated candidate GREEN.
2. Mutate only `env_config_value_or_empty`'s production `None` result from
   empty bytes to a compile-valid `[0]`. Record that the POPULATION operand
   moved, not a test or an oracle.
3. **Required observation.** In a scratch copy, remove only the two public
   `optional_absence` declarations and the exact private dependency closure
   exclusive to them, NAMING that removed set. Do not edit either required law,
   its type, its body, the shared lookup/value machinery, or the mutation. Both
   external `required_lookup` queries must elaborate GREEN. This is AC-1's
   positive observation under the changed production value.
4. **Optional observation.** From the full candidate, apply the identical
   one-site production mutation. It must REJECT at the `None` equation or at an
   `optional_absence` dependency, with the exact span and error recorded. A
   generic package red is insufficient.
5. Restore byte-exactly and rerun the full candidate. The existing
   accept-missing-field mutation must still independently red AC-1.

The scratch subtraction does not rewrite the detector. The kernel remains the
detector and the required public claims and certificates stay byte-identical;
the subtraction only isolates one promise class from the intentionally failing
optional promise, exactly as separate test targets would. A staged certificate
edit that merely substitutes `[0]` for the old placeholder is INVALID evidence:
it edits the witness to follow the mutation without removing the coupling.

Report the three mutation-discipline fields: property, operand moved, observed
boundary.

Provenance is retained: for an `Invalid` result the carried issues' origins are
`EnvVariableOrigin` for `decode_environment_entries` and `ConfigEntryOrigin`
for `decode_config_entries`, one per failing field, matching
`crates/ken-elaborator/tests/cc8_env_config_decoder_acceptance.rs::config_failures_keep_config_key_origins_distinct_from_environment`.

## 5. Stop condition

Stop and report if AC-1 or AC-2 cannot be discharged without a new primitive,
postulate, `Axiom`, or trusted-base entry, or if either requires a production
change beyond the two authorized in section 3: the `env_config_lookup`
visibility change, and the section 3a `env_config_value_or_empty` refactor. No
further widening and no further body change.

**If the factored shape still cannot produce the split, STOP AGAIN.** Do not
add another traversal and do not weaken either law. **Do not repair the
optional/empty conflation in this node.** Whether an absent optional field may
be represented by empty `Bytes` is a design question about the decoder's return
type, and it is routed to the Architect separately. This node proves what is
true today and makes the conflation legible; it does not decide whether the
representation should change.

## 6. Not this node

Do not prove general `env_config_lookup` laws beyond what AC-1 and AC-2 need,
do not touch `Application/Input/Schema.ken.md`, do not change `env_config_help`'s
delegation to `schema_help`, and widen the public surface no further than the
eight identities section 3 enumerates.
Total ordering and shadowing behavior of duplicate keys in `entries` is not in
scope. Changing the decoder's return type so that optional absence is
distinguishable from a present empty value is NOT in scope and is the
Architect's to rule on.

## 7. Symptom inventory

Architect section 1a hard-stop count for this chain is 1. Entry 1 was a frame
contradiction surfaced before any Architect technique ruling existed and did
not advance the count; stop 1 is the genuinely new proof-structure wall hit
while building the publication ruling. Neither the third-stop Research trigger
nor the third-entry predicate review fires yet. A new wall after the section 3a
ruling is stop 2 and entry 3, and entry 3 requires the section 1b
shared-predicate check before another ruling.

1. Public attached-law signatures could not close while their exact lookup
   authority remained private and visibility changes were forbidden.
2. Required-index transport reconstructed the optional `None` arm, coupling
   AC-1's certificate to the placeholder.
