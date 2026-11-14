---
id: LANG-PROCESS-CHILD-PIPE-WRITE-HOME
title: "The prelude defines write_pipe, close_pipe_write and close_pipe_read and spec 38 §1.11.2 types them, but §1.11.1 gives Capability.Process.Child only sixteen exports and no package homes the three, so Strict source cannot write a child's stdin or close a pipe early, and ResourceStreamWrite has no reaching witness in either engine. Home the three in Capability.Process.Child at their existing IDs, with the roster at nineteen and seed rows"
status: ready
owner: language
size: S
tier: T2
gate: architect
depends_on: []
blocks: []
github: null
origin: "Architect evt_6ykbqn2mrxr4t and evt_2n42pc3vef8xz on the PX10 B2 stop evt_70swd255qh2zx: B2 holds the ResourceStreamWrite promotion; homing the three pipe operations in Child is a separate Spec and catalog node. It enables L1's stream-write native promotion (docs/program/10-linux-abi-completion.md). Steward-filed per COORDINATION section 2."
---

# A child's stdin can be written

## Objective

Strict isolated source imports `write_pipe`, `close_pipe_write` and
`close_pipe_read` from `Capability.Process.Child` and uses them on the
pipes `take_stdin`, `take_stdout` and `take_stderr` return. Spec 38
§1.11.1 rosters nineteen names.

## Settled inputs (main `fe2e8ce01`)

- **The roster.** Spec 38 §1.11.1 (`spec/30-surface/38-ffi-io.md:1022`)
  homes sixteen names and says the other pipe operations have no ruled
  home. It already homes `take_stdin`, which yields a `PipeWrite` that no
  homed operation consumes, and `read_pipe`.
- **The operations exist.** §1.11.2 types all three (`:1107`, `:1111`,
  `:1113`). The prelude defines them (`prelude/process.rs:243`, `:257`);
  `write_pipe` lowers to `PrivateResourceStreamWrite`.
- **No home.** `catalog/packages/Capability/Process/Child.ken.md` exports
  none of the three. A bare use refuses `UnboundName close_pipe_write`
  and an explicit import refuses `UnboundName
  Capability.Process.Child.write_pipe`
  (`/workspaces/ken/tmp/px10-b2-native-pipe-write-first.log`,
  `/workspaces/ken/tmp/px10-b2-native-pipe-write-import-probe.log`).
- **Seed prose exists.** `conformance/surface/ffi-io/seed-spawn-plan-wait.md`
  :235-274 already states the write, direction and early-close
  obligations.
- **The pattern.** Increment 3a added four names to the same home at
  their existing IDs; its test is
  `crates/ken-elaborator/tests/lang_strict_isolated_routes_inc3a.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 38 §1.11.1's roster sentence, the three exports in `Child.ken.md`,
a package test and seed rows. The three keep their prelude IDs, types and
lowering. No authority, trust, CLI route or native availability change:
`ResourceStreamWrite` stays `RepresentedUnavailable`, and its promotion
is the L1 runtime increment that follows.

## Acceptance

- **AC-1.** Under Strict, a client importing the three from
  `Capability.Process.Child` checks; each import resolves to the
  prelude's existing checked identity, not a new declaration.
- **AC-2.** The same client without the import refuses with
  `UnboundName` at each name, on the same source.
- **AC-3.** The interpreter runs a client that spawns with `ChildPipe`
  stdin, writes bytes through `write_pipe`, closes with
  `close_pipe_write` before `wait_child`, and observes the child's exit.
  A `PipeRead` passed to `write_pipe` refuses at type checking.
- **AC-4.** Spec 38 §1.11.1 names nineteen exports; the CV reviews the
  spec sentence and the seed rows with a Decision.

## Stop conditions

- A home needs a new declaration, ID or lowering for any of the three.
- AC-3 needs a native or availability change.

Checks: 4 fired: the import obligation was written against main; both
the bare and the explicit form refuse, by the B2 implementer's two logs.
8 fired: AC-1's identity check separates a re-declared copy, which would
also check, from a home at the existing ID; AC-2 pairs the import's
presence with its absence on one source.
