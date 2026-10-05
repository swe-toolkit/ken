---
id: CAT-CONSOLE-TEXT-LAWS
title: "Console.Text's four helpers (print, printLine, eprint, eprintLine) have no proof: their stream choice, exact bytes, single trailing newline and Result preservation are only executed by i2_console_floor.rs. Prove each helper's write tree in the package at zero TCB"
status: ready
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [CAT-SYSTEM-RESOURCE-LAWS]
blocks: []
github: null
origin: "The last unframed row of docs/program/CATALOG-PROOF-COMPLETENESS-SURVEY.md (Capability/Console/Text.ken.md, recommendation CAT-CONSOLE-TEXT-LAWS). L3 proof backfill (operator 2026-09-13; Architect evt_5f1ewknxv3m6h: L3 turns to proof backfill). L3's successor to CAT-SYSTEM-RESOURCE-LAWS. Steward-filed per COORDINATION section 2."
---

# Console.Text helpers, proven

## Objective

`catalog/packages/Capability/Console/Text.ken.md` carries checked theorems
for what its prose claims: each helper issues one `Write` to its stream with
its exact bytes, the line forms append one newline, and the helper returns
`write`'s `Result IOError Unit` unchanged.

## Settled inputs (read at `669a5cf91`)

- The package is four `proc` definitions in one Ken fence and has no
  theorem. The test `crates/ken-interp/tests/i2_console_floor.rs::
  package_helpers_route_exact_bytes_and_one_newline` executes them.
- `write` is transparent (`crates/ken-elaborator/src/prelude.rs:1502`):
  `Vis ConsoleOp console_resp (Result IOError Unit) (Write stream bytes)
  (\r. Ret … r)`. So each helper unfolds to one `Vis` node with a `Ret`
  continuation.
- `bytes_encode` is a primitive (spec 38). The newline is
  `bytes_encode (list_char_to_string (Cons Char (10 : Int) (Nil Char)))`.

## Deliverable

One theorem per helper in the package fence, stating its exact tree:

    print text = Vis ConsoleOp console_resp (Result IOError Unit)
                   (Write Stdout (bytes_encode text)) (\r. Ret … r)

The line forms state the `bytes_concat` of the text's encoding and the
newline's encoding as the payload. The package prose names the theorems. No
new primitive, postulate or axiom, and no prelude change.

## Acceptance

- **AC-1.** `ken check` and `ken fmt --check` pass on the package.
  `trusted_base()` is unchanged, as an independently written client test
  confirms.
- **AC-2.** That client states each theorem's type independently and
  checks it against the package's. Each of these, applied in isolation to
  the package source, compiles and fails the matching theorem:
  - swap a helper's stream;
  - drop the newline from a line form;
  - replace the continuation with one that discards `r`.

## Stop conditions

- A law needs a fact about `bytes_encode`'s output, such as the newline
  being one byte: that is a primitive contract (spec 38), not a Ken proof.
  Stop to the Architect, and do not add an axiom.
- `trusted_base()` changes: an operator question.
