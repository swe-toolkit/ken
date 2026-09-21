---
id: CAT-CONFIGURATION-DECODER-LAWS
title: "prove the required-field agreement the decoder actually has, and characterize the optional lane it does not -- env_config_validation discards the validation payload and recomputes values in a second independent traversal whose None branch emits an empty-Bytes placeholder; the REQUIRED lane is guarded (schema_check_presence rejects SchemaRequired), so the reachable placeholder is an ABSENT OPTIONAL field, whose empty Bytes is indistinguishable from a present-but-empty value"
status: ready
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

The pre-refutation facts were measured at `8472b78aa`; the presence-split facts
and the refutation below were measured at `d5d7e6299`.

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
- **`env_config_missing_field` does NOT reject unconditionally**, and this is
  the fact an earlier version of this frame missed. It calls
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
subjects. An attached `pub proof` travels with the function selector, so **no
client import, export list, or selector list changes.**

## 4. Acceptance criteria

**AC-1 -- the required-field agreement, at both entry points.** For
`decode_environment_entries schema entries` returning `Valid values`: for every
index `i` in range of `schema_fields schema` **whose `schema_field_presence` is
`SchemaRequired`**, `env_config_lookup (bytes_encode (schema_field_name
field_i)) entries` is `Some v_i` and `nth i values = Some v_i`. The bytes are
the entry's own bytes with no re-encoding, which is where raw-`Bytes`
preservation is discharged. The same law for `decode_config_entries`. This is
the safety property: it is what makes a required field's value trustworthy, and
it is TRUE, unlike the universal form this frame carried before.

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

**AC-3 -- the laws must be refutable, and by the right mutations.** Changing
`env_config_field_check` to accept a missing field must turn AC-1 RED. Changing
`env_config_values`' `None` branch to a different placeholder must turn AC-2 RED
and leave AC-1 GREEN -- AC-1 must not be keyed on the placeholder's value, and
AC-2 must be. A pair that does not split on that mutation is not measuring the
two lanes separately. Provenance is retained from the prior frame: for an
`Invalid` result the carried issues' origins are `EnvVariableOrigin` for
`decode_environment_entries` and `ConfigEntryOrigin` for
`decode_config_entries`, one per failing field, matching
`crates/ken-elaborator/tests/cc8_env_config_decoder_acceptance.rs::config_failures_keep_config_key_origins_distinct_from_environment`.

## 5. Stop condition

Stop and report if AC-1 or AC-2 cannot be discharged without a new primitive,
postulate, `Axiom`, or trusted-base entry, or if either requires changing an
existing function body rather than adding proofs. **Do not repair the
optional/empty conflation in this node.** Whether an absent optional field may
be represented by empty `Bytes` is a design question about the decoder's return
type, and it is routed to the Architect separately. This node proves what is
true today and makes the conflation legible; it does not decide whether the
representation should change.

## 6. Not this node

Do not prove general `env_config_lookup` laws beyond what AC-1 and AC-2 need,
do not touch `Application/Input/Schema.ken.md`, do not change `env_config_help`'s
delegation to `schema_help`, and do not widen any export or selector list.
Total ordering and shadowing behavior of duplicate keys in `entries` is not in
scope. Changing the decoder's return type so that optional absence is
distinguishable from a present empty value is NOT in scope and is the
Architect's to rule on.
