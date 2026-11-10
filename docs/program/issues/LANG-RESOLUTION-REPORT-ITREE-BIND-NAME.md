---
id: LANG-RESOLUTION-REPORT-ITREE-BIND-NAME
title: "Under Strict, the resolution report names the floor ITree sequencer `bind`, a spelling Strict leaves unbound or gives to the user's own declaration, because the itree_bind alias is installed after the pre-source name snapshot without a report display name. Give the checked bind identity the display name `itree_bind`, so a Strict report names the spelling that resolves to it and a user `fn bind` is never confused with it"
status: ready
owner: verify
size: S
tier: T2
gate: architect
depends_on: []
blocks: []
github: null
origin: "Adversary report evt_681mr5jjdkpx5 on LANG-STRICT-ISOLATED-ROUTES Inc 3a (644cc093c): a Strict report display leak, carried in the L2 row. L2 objective: one resolution mode (operator 2026-09-25); Verify runs L2's Strict-surface repairs. Steward-filed per COORDINATION section 2."
---

# The report names `itree_bind`

## Objective

A Strict resolution report that references the checked ITree bind
identity names it `itree_bind`, the floor spelling that resolves to it.
A user declaration named `bind` is reported as `source-declared`, and
nothing names the floor identity `bind`.

## Settled inputs (main `69e3cb106`)

- **The snapshot.** `pre_source_names` is captured from the globals at
  `lib.rs:516`-`525`, when the bind identity is still bound as `bind`.
- **The alias.** `itree_bind` is inserted after the snapshot
  (`lib.rs:526`-`540`) and gets no display name.
- **The display map.** `pre_source_display_names` already renames the
  ConsoleStream family for the report only (`lib.rs:557`-`570`). The
  report reads it first and falls back to the snapshot name
  (`resolution_report.rs:190`-`197`).
- **The promise.** Spec 33 (`33-declarations.md:399`-`422`): the report
  gives pre-source entries their canonical names and does not change the
  availability or resolution of any name.
- **The repro.** Under `elaborate_isolated_file_strict_v1`, `fn seq`
  calling `itree_bind ConsoleOp console_resp Unit Unit tree (λv. ...)`
  reports `{"class":"pre-source","name":"bind"}`. With a user
  `fn bind (n : Nat) : Nat = n` beside it, both identities print `bind`.
- **Pins that read `bind`.** `lang_prelude_collections.rs:274`-`277`
  and `lang_strict_isolated_routes_inc3a.rs:115`-`158` pin the Legacy
  binding and the Strict UnboundName. Neither reads the report.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

One candidate: the bind identity's report display name is `itree_bind`.
Report membership, the native stable symbol and every name's resolution
are unchanged. If Inc 3b of `LANG-STRICT-ISOLATED-ROUTES` lands first,
rebase onto it; its `lib.rs` hunk is elsewhere.

## Acceptance

- **AC-1, the repro.** In a committed test, the Strict `seq` report
  references `pre-source` `itree_bind` and no entry named `bind`.
- **AC-2, the user shadow.** With the user `fn bind` added, `use_user`
  references `source-declared` `bind`, and `seq` still references
  `pre-source` `itree_bind`. A candidate that renames by spelling instead
  of by identity fails here.
- **AC-3, mutation.** Drop the display insertion: AC-1 reddens. A
  mutation that does not redden is reported as non-discriminating, not
  replaced.
- **AC-4.** Every committed `lang_cli_resolution_report.rs` row that
  passed before still passes, and full-workspace CI is green on the
  exact candidate.

## Stop conditions

- The fix changes report membership, a native stable symbol, or the
  resolution of `bind` or `itree_bind` on any route.

Checks: 3 fired: grepped `crates/*/tests` for `"bind"` and report
readers. Five hits pin the binding or the UnboundName, none the report;
`lang_cli_resolution_report.rs` is the only report suite. 8 fired: AC-2
separates a fix keyed on the identity from one keyed on the spelling,
which passes AC-1 alone.
