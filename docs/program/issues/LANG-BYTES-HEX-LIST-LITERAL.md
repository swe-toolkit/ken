---
id: LANG-BYTES-HEX-LIST-LITERAL
title: "the bracketed `0x[deadbeef]` Bytes literal is normative in two spec sections and absent from the lexer, so the only landed way to write a Bytes value is `b\"…\"` and any `0x[` source fails as an invalid radix integer"
status: ready
owner: language
size: S
gate: none
depends_on: []
blocks: []
github: null
origin: "Surfaced by CONF-FMT8-LEVELTOK's census (merged 2026-08-15 at 2ed8bbfd8): the conformance-validator found `surface/formatting/all-literal-lexemes-are-verbatim` unproducible and it landed marked `BLOCKED-ON-HEX-BYTE-LIST-SURFACE (no blocker node exists)`. Steward-filed per COORDINATION §2 to supply that blocker. Not a new finding so much as a collected one -- LANG-LEX-HEX-FLOAT recorded the same gap at its own line 70 and routed it to `neither node`."
---

> # THIS GAP WAS KNOWN AND DELIBERATELY UNROUTED. THAT IS WHY IT NEEDS A NODE.
>
> `LANG-LEX-HEX-FLOAT` is `merged`, and its own table at line 70 carries the
> row `| 0x[deadbeef] | bracketed Bytes | 38-ffi-io, neither node |`. Its line
> 190 says the same in prose: *"`0x[deadbeef]` byte literals are
> `38-ffi-io`'s"*. `LANG-LEX-NUMERIC-FORMS:151` independently says *"not a
> numeric form. Do not fold"*.
>
> **Two merged nodes each correctly excluded this from their own scope and
> named the section that owns it. Neither could file the successor** — agents
> cannot create tracked work (`COORDINATION §2`) — **so the exclusion was
> recorded twice and the work was filed zero times.** The conformance census
> then hit the same wall from the other side and had to write *"no blocker node
> exists"* into a landed seed.
>
> **The tell is worth keeping: a gap named in a `merged` node's prose is
> invisible to every status query, because the node reads as done.**

## What this is

`spec/30-surface/38-ffi-io.md §1.1` states two surface forms for `Bytes`:

| form | example | status in the tree |
|---|---|---|
| byte string | `b"GET / HTTP/1.1\r\n"` | **landed** — `Token::ByteStr(Vec<u8>)`, `lexer.rs:91`, produced at `lexer.rs:979` |
| hex | `0x[deadbeef]` | **absent** |

`spec/30-surface/31-lexical.md:511` lists both in the literal token table, and
`38-ffi-io.md:304`'s **AC1** names both: *"A `b"…"`/`0x[…]` literal elaborates
to the `Bytes` primitive"*.

**What source `0x[` actually does today:** it enters `lex_radix_integer`
(`lexer.rs:1231`), which finds no digit before the `[`, and fails at
`lexer.rs:1247` with `"invalid radix integer"`. A grep for `0x[` under
`crates/` returns **two hits, both doc comments** in
`tests/l6_acceptance.rs:509,511` — the notation is already the project's own
vocabulary for describing bytes, and it is not accepted as source.

## The shape of this is smaller than it looks, and that is the design judgment

**The token already exists and so does everything downstream of it.**
`b"…"` lexes to `Token::ByteStr(Vec<u8>)` and elaborates to the `Bytes`
primitive today. **`0x[…]` is a second surface spelling of a value the
elaborator already constructs.**

⇒ **The expected shape is a lexer change producing the existing
`Token::ByteStr`** — no new token kind, no new AST node, no elaborator or
kernel change. If you find yourself adding a token variant, stop and say why;
that is a signal the reading above is wrong, and it is worth more than a
workaround.

**The `0x` prefix collides with radix-integer lexing, and the spec states the
discriminator itself.** `38-ffi-io.md:84`: *"The **bracketed** `0x[…]` is the
`Bytes` form; the **un-bracketed** `0xFF` is an **`Int`** literal ... the two
are different tokens with different types and **must not be conflated**."*
The dispatch is one character of lookahead after `0x`.

## The one thing the spec does not settle — surface it, do not decide it

**Whether whitespace is permitted inside the brackets.** `38-ffi-io.md:82`
writes the literal as `0x[deadbeef]` and glosses its *meaning* as `de ad be ef`
— the spacing there describes the resulting bytes, not the accepted source.
But `tests/l6_acceptance.rs:509` writes `0x[65 cc 81]` in a doc comment, which
is the same notation with internal spaces.

⇒ **Report which you implemented and why, as a finding.** Do not treat the test
comment as normative and do not silently pick the permissive reading. If the
answer is not derivable from `31 §3`, that is a spec question routed to the
Steward, and the strict form (contiguous nibble pairs) is the safe landing
while it is asked.

## Deliverables

**`D1` — lex `0x[…]` to the existing `Token::ByteStr`.** Hex nibble pairs, an
even number of digits, producing the same `Vec<u8>` a `b"…"` literal with those
bytes produces.

**`D2` — the un-bracketed form is untouched.** `0xFF` remains an `Int`, and
`0x1p-3` remains the hex float `LANG-LEX-HEX-FLOAT` landed. Report the
discriminating point in the lexer at `file:line`.

**`D3` — the malformed cases reject with a diagnostic that names the form.** An
odd digit count, a non-hex character inside the brackets, and an unterminated
`0x[` are three distinct inputs. `"invalid radix integer"` is the **wrong**
message for all three — it names a form the author did not write.

## Acceptance criteria

**`AC-1`.** `0x[deadbeef]` elaborates to the `Bytes` primitive, asserted
structurally on the elaborated value and type per `38-ffi-io.md:304` AC1 — not
"it compiles". **Control:** the elaborated value equals that of the
corresponding `b"…"` literal.

**`AC-2`.** `0xFF` is still an `Int` and `0x1p-3` is still a `Float`.
**Control:** the existing numeric-form and hex-float tests stay green, named
individually in the handback. This is the regression the `0x` collision makes
likely, and a green suite reported as a total is not evidence.

**`AC-3`.** Each of the three malformed inputs in `D3` produces a message
naming the bytes form. **Control:** assert on the message text, not merely on
`is_err()`.

**`AC-4`.** No new `Token` variant and no elaborator change, **or** a stated
reason why the reading in this node is wrong. Either is an acceptable outcome;
an unexplained widening is not.

**`AC-5`.** No-regression, in CI (`COORDINATION §12`). Targeted locally:
`-p ken-elaborator`.

## What this unblocks, and the follow-through that is NOT yours

`conformance/surface/formatting/seed-canonical-format.md` carries
`BLOCKED-ON-HEX-BYTE-LIST-SURFACE (no blocker node exists)` on
`all-literal-lexemes-are-verbatim`, and FMT1's aggregate marker enumerates the
same surface. **Once this lands, that marker can name this node and the row can
be re-adjudicated as producible.**

**Do not edit `conformance/`.** That seed is the spec enclave's, the
re-adjudication is a census question rather than a find-and-replace, and it is
routed by the Steward. Say in the handback that it is now unblocked.

## Sizing

**`S`.** One lexer form, one dispatch decision, three rejection cases. If it is
not `S` — in particular if `Token::ByteStr` turns out not to be reachable from
the `0x` dispatch without restructuring the numeric path — **that is the hard
stop and the finding**, and it should arrive well inside one turn.
