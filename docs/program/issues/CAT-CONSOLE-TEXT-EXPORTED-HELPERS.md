---
id: CAT-CONSOLE-TEXT-EXPORTED-HELPERS
title: "Console.Text's four helpers are private procs, so no client module can call print, printLine, eprint or eprintLine, and the four pub laws CAT-CONSOLE-TEXT-LAWS added name private subjects a client cannot write. Export the helpers and prove the laws usable from another module"
status: active
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [CAT-CONSOLE-TEXT-LAWS]
blocks: []
github: null
origin: "Adversary finding evt_365c9t8caz8gp on a0a3f703f. Fails closed, no soundness or behavior impact; the helpers were never importable, and CAT-CONSOLE-TEXT-LAWS's flat-environment client could not see visibility. Steward-filed per COORDINATION section 2."
---

# Console.Text helpers are callable from a client module

## Objective

A module that imports `Capability.Console.Text` can call each helper and
state each law about it.

## Settled inputs (Adversary `evt_365c9t8caz8gp`, on `a0a3f703f`)

- **The gap.** `print`, `printLine`, `eprint` and `eprintLine` are `proc`
  declarations without `pub` (`Text.ken.md:10`, `:13`, `:20`, `:23`). The
  laws at `:30`, `:35`, `:43`, `:48` are `pub`, and two of them name the
  private `console_line_payload` (`:56`).
- **Repro.** Through `elaborate_module_from_roots(catalog/packages,
  "Capability.Console.Text")`, `import Capability.Console.Text (printLine,
  print_line_write_tree)` gives `UnboundName`, and calling `printLine t`
  after a whole-module import gives `UnresolvedCon`.
- **Why the gates passed.** `cat_console_text_laws.rs:11-38` and
  `i2_console_floor.rs` elaborate package and client in one flat
  environment, where exported and private names check alike.
- `pub proc` exports a proc across modules (`modules.rs:7337`, where a
  client calls `K.f` from an imported `pub proc f`);
  no catalog package uses it yet.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

Ruled by the Architect (`evt_2t6eg0v4btbj5`): export the payload helper,
do not inline it.

- `pub proc` on the four helpers, and `pub fn console_line_payload (text :
  String) : Bytes` with its body unchanged.
- `printLine` and `eprintLine` are each one `write` of `console_line_payload
  text`. The four law types and their proofs stay byte-identical.
- `console_tree_refl` stays private.
- Order: the four helpers, the four laws, `console_line_payload`, then
  `console_tree_refl`. The prose names the nine exports and states that
  `console_line_payload text` is `bytes_encode text` followed by the encoding
  of character 10.

No new primitive, postulate or axiom, and no prelude change.

## Acceptance

- **AC-1.** A client module, elaborated through the module loader from the
  package roots, imports the four helpers and four laws, calls each helper,
  and states each law's type with the helper as its subject. `ken check` and
  `ken fmt --check` pass on the package; `trusted_base()` is unchanged.
- **AC-2 (controls).** The existing AC-2 falsifiers of CAT-CONSOLE-TEXT-LAWS
  keep their results, including all four conversions against
  `client_console_line_bytes` in `cat_console_text_laws.rs`.
  `i2_console_floor`'s output bytes are identical, and the ambient
  passthrough census stays green.
- **AC-3 (mutation, QA).** Removing `pub` from any one helper reddens the
  AC-1 client. Removing it from `console_line_payload` reddens the client's
  two line-law statements with `UnboundName`.

## Stop conditions

- The export changes any package's resolution or a prelude collision
  census: stop to the Architect.
- `trusted_base()` changes: an operator question.
- The Console.Text prelude-name set pinned at
  `lang_mod_strict_resolution_d0.rs:520` (`{IO, IOError, Stderr, Stdout,
  Unit, write}`) changes, or any hash, snapshot or artifact pin on
  `printLine` or `eprintLine` moves: stop to the Architect.
