---
id: CAT-CONFIGURATION-DECODER-LAWS
title: "prove that a Valid decode never reaches env_config_values' empty-Bytes placeholder -- the decoder runs the schema traversal and the value traversal INDEPENDENTLY over the same fields and entries, discards the validation's own values, and recomputes them, so nothing today connects Valid to lookup success; if that agreement fails, a missing required field decodes to empty Bytes instead of an error. Instantiates the landed schema_validate_fields::valid_coverage at the two decoder entry points and carries raw-Bytes identity and per-entry-point provenance with it."
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

# Decoder laws: a Valid decode returns looked-up bytes, never a placeholder

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## 1. Objective

State and prove, over the existing decoder representation and with no new
trust, that a `Valid` result from either decoder entry point returns exactly
the bytes found in `entries` for each schema field, in field order. The
`None` branch of `env_config_values` must be shown unreachable on that path.

## 2. Settled inputs -- measured at `8472b78aa`. Do not re-derive.

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
  `env_config_field_check`, which accepts iff
  `env_config_lookup (bytes_encode (schema_field_name field)) entries` is
  `Some`, and otherwise yields `env_config_missing_field`.

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

**AC-1 -- Valid returns looked-up bytes, and the placeholder is unreachable.**
For `decode_environment_entries schema entries` returning
`Valid values`: for every index `i` in range of `schema_fields schema`,
`env_config_lookup (bytes_encode (schema_field_name field_i)) entries` is
`Some v_i` and `nth i values = Some v_i`. The bytes are the entry's own bytes
with no re-encoding, which is where raw-`Bytes` preservation is discharged.

**AC-2 -- the law must be refutable, and by the right mutation.** Two controls.
Changing `env_config_values`' `None` branch to a different placeholder must
leave the proof GREEN, because that branch is unreachable on the Valid path --
a proof that reds here is keyed on the placeholder's value rather than on its
unreachability. Changing `env_config_field_check` to accept a missing field
must turn it RED. A law that survives both is vacuous and does not pass.

**AC-3 -- provenance stays distinct per entry point.** For an `Invalid`
result, the carried issues' origins are `EnvVariableOrigin` for
`decode_environment_entries` and `ConfigEntryOrigin` for
`decode_config_entries`, one per failing field, matching the behavior already
fixture-checked by
`crates/ken-elaborator/tests/cc8_env_config_decoder_acceptance.rs::config_failures_keep_config_key_origins_distinct_from_environment`.

## 5. Stop condition

Stop and report if AC-1 cannot be discharged from `valid_coverage` without a
new primitive, postulate, `Axiom`, or trusted-base entry; if it requires
changing any existing function body rather than adding proofs; or if the
agreement turns out to be FALSE -- that is, if `Valid` can hold while a lookup
returns `None`. A false agreement is a product defect, not a proof exercise,
and it comes back to me before any repair is attempted.

## 6. Not this node

Do not prove general `env_config_lookup` laws beyond what AC-1 needs, do not
touch `Application/Input/Schema.ken.md`, do not change `env_config_help`'s
delegation to `schema_help`, and do not widen any export or selector list.
Total ordering and shadowing behavior of duplicate keys in `entries` is not in
scope.
