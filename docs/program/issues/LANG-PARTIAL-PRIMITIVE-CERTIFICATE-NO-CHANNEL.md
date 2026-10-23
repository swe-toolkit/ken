---
id: LANG-PARTIAL-PRIMITIVE-CERTIFICATE-NO-CHANNEL
title: "A refined-divisor division is refused with ObligationWithoutChannel in a context that has no obligation channel (the REPL's elaborate_expr route), although the evidence search finds and kernel-checks the certificate: admit_certificate_hole always mints a hole first, and declare_obligation_hole refuses under PremiseHoles::Refused before the certificate is used. Admit a found certificate there, as the sibling subset introduction already does"
status: active
owner: language
size: S
tier: T2
gate: architect
depends_on: [LANG-PARTIAL-PRIMITIVE-REFINED-EVIDENCE]
blocks: []
github: null
origin: "Adversary M8 finding evt_gdf5xqpc18vq on LANG-PARTIAL-PRIMITIVE-REFINED-EVIDENCE (1361da588). It fails closed and is not a regression: the parent refuses the same row. Steward-filed per COORDINATION section 2."
---

# Refined-divisor certificates in a context with no obligation channel

## Objective

In a context with no obligation channel, a refined-divisor `/` or `%`
whose certificate the evidence search finds is admitted, as subset
introduction already is. An undischarged one is still refused.

## Settled inputs (main `1361da588`, Adversary `evt_gdf5xqpc18vq`)

- **The row.** Through `ElabEnv::new()` and then `elaborate_expr`, the
  REPL's eval and `:t` route (`repl.rs:193`, `:206`):
  - `(\n . \d . n / d) : (n : Int) -> (d : {z : Int | Not (Equal Int z
    0)}) -> Int` gives `Err(ObligationWithoutChannel { span: 11..16 })`;
  - the sibling `(\d . d) : (d : {z : Int | Not (Equal Int z 0)}) -> {z :
    Int | Equal Int z 0 -> Bottom}` gives `Ok`, discharged by the same
    `Proj2 d`.
- **The mechanism.**
  - On a found certificate, the Div/Mod arm calls
    `admit_certificate_hole` (`elab.rs` ~:14335).
  - That function mints through `declare_obligation_hole` (~:14946),
    which refuses under `PremiseHoles::Refused` (~:6735) before the
    certificate is used.
  - `emit_subset_evidence` returns its certificate without minting in
    that context (~:14977).
  - The Div/Mod term does not reference the hole.
- **Controls, green on `1361da588`.**
  - The `fn` form of the row gives one closed obligation.
  - A plain `Int` divisor stays open in a `fn`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

`elab.rs` admits a found certificate in a no-channel context and records
it wherever a channel exists. A test pins the rows in
`int_div_mod_surface.rs`.

## Acceptance

- **AC-0 (route, Architect).** Rule whether a discharged PartialPrim
  obligation in a no-channel context is admitted unrecorded, as subset
  introduction is, or needs a record. Spec 22 §2.5 item 3 says
  "recorded, never silent".
- **AC-1.** The row is `Ok` through `elaborate_expr`. The `fn` rows of
  LANG-PARTIAL-PRIMITIVE-REFINED-EVIDENCE are unchanged, and
  `trusted_base()` is unchanged.
- **AC-2.** A plain `Int` divisor and `Not (Equal Int 0 z)` are still
  refused with `ObligationWithoutChannel` through `elaborate_expr`.
- **AC-3 (mutation, QA).** Restoring the mint-first order reddens AC-1's
  row back to `ObligationWithoutChannel`.

## Stop conditions

- Any kernel or `trusted_base()` change.
- A changed verdict on a row with a channel.
