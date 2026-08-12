# Literal-escape conformance — seed cases

Format: `../../README.md`. These cases pin the closed ordinary-literal escape
repertoire in `spec/30-surface/31-lexical.md §3`. They cover escape recognition,
decoded scalar or byte payloads, and lexical rejection only. The existing
String, Char, and Bytes carriers remain owned by `../collections/`,
`../numbers/`, and `../bytes-io/`; source-lexeme preservation remains owned by
`../formatting/`.

The error boundary is intentionally asymmetric. `InvalidEscape`, its primary
span, and its precedence after a backslash commits to an escape production are
normative here. The names and spans of character-cardinality, non-ASCII
byte-body, and ordinary unterminated-literal diagnostics are not fixed by this
contract.

## Closed repertoire and exact decoding

### surface/literals/common-escape-matrix-decodes-exactly
- spec: `31 §3` (escape-repertoire table and decode-before-validity rule)
- given: each escape in the table below, embedded separately in an ordinary
  String, Char, and byte-string literal:

  | source escape | String/Char scalar | byte-string byte |
  |---|---:|---:|
  | `\\` | U+005C | `0x5C` |
  | `\"` | U+0022 | `0x22` |
  | `\'` | U+0027 | `0x27` |
  | `\0` | U+0000 | `0x00` |
  | `\n` | U+000A | `0x0A` |
  | `\r` | U+000D | `0x0D` |
  | `\t` | U+0009 | `0x09` |

- expect: every literal is accepted and contributes exactly the scalar or byte
  shown. Each Char fixture contains just that one decoded scalar. No escape is
  retained as two source characters, and no literal kind uses a different
  simple-escape mapping.
- why: the matrix fails if a nominally shared repertoire is implemented in only
  one scanner or if any scanner decodes the same spelling differently.

### surface/literals/escape-repertoire-is-closed-and-kind-selected
- spec: `31 §3` (closed repertoire, literal-kind selection, `InvalidEscape`)
- given: a finite sweep over every ASCII character immediately after a
  backslash, with complete literal fixtures for each kind. Complete the
  selected production when the character is `u` or `x`; the boundary-only
  cases are covered separately below. Include `\q` as a named unrecognized
  control, well-shaped `\u{41}` in all three kinds, and well-shaped `\x41` in
  all three kinds.
- expect: the only simple escapes accepted in every kind are `\\`, `\"`,
  `\'`, `\0`, `\n`, `\r`, and `\t`. A complete `\u{H…H}` is accepted only in
  String and Char; a complete `\xHH` is accepted only in a byte string. Every
  other discriminator rejects with `InvalidEscape` and emits no literal token.
  The `\q` primary span is exactly `\q`. A wrong-kind, well-shaped escape also
  rejects with `InvalidEscape`, but its primary span is the complete escape:
  exactly `\u{41}` in a byte string and exactly `\x41` in String or Char.
- why: exhaustive discriminator coverage pins closedness by construction. The
  paired well-shaped controls distinguish kind selection from malformed-shape
  rejection and catch a scanner that accepts the union in every literal kind.

## Kind-specific productions

### surface/literals/unicode-escape-shape-scalar-and-char-cardinality
- spec: `31 §3` (Unicode shape, scalar domain, decode-before-Char validity)
- given: String and Char fixtures containing `\u{0}`, `\u{1F600}`, and
  `\u{10FFFF}`; malformed fixtures `\u{}`, `\u{0000041}`, `\u{4_1}`, and
  `\u{G}`; well-shaped invalid-scalar fixtures `\u{D800}`, `\u{DFFF}`, and
  `\u{110000}`. Also compare Char literals containing one decoded scalar with
  empty and two-decoded-scalar Char literals.
- expect: the three valid escapes decode respectively to U+0000, U+1F600, and
  U+10FFFF. Zero digits, seven digits, separators, non-hex digits, surrogates,
  and values above U+10FFFF reject with `InvalidEscape` and emit no literal
  token. The malformed primary spans end after the first character that makes
  the production invalid: exactly `\u{}`, `\u{0000041`, `\u{4_`, and `\u{G`.
  Each well-shaped invalid-scalar span is its complete escape, including `}`.
  After valid decoding, Char accepts exactly one scalar; empty or two-scalar
  content rejects under the existing Char-validity diagnostic, whose name and
  span this case does not pin.
- why: the boundary set distinguishes digit-count validation from numeric-value
  validation and prevents either from admitting surrogates. The cardinality
  controls ensure escape decoding does not bypass the existing Char invariant.

### surface/literals/byte-string-ascii-and-x-domain
- spec: `31 §3` (ASCII byte bodies, byte-only fixed-width `\xHH`)
- given: all 256 byte escapes `b"\xHH"`, with upper- and lower-case hex
  controls; `b"\x41BC"`; ordinary byte strings spanning the permitted
  unescaped ASCII body characters; a byte string with an unescaped non-ASCII
  scalar; and malformed controls `b"\x4"` and `b"\xG0"`.
- expect: each `\xHH` contributes exactly the denoted byte. In particular,
  `b"\x41BC"` denotes exactly the byte sequence `0x41 0x42 0x43`: `\x41`
  consumes two digits, then `B` and `C` contribute their ordinary ASCII bytes.
  Letter case does not alter a hex value. Permitted unescaped ASCII content
  contributes its ASCII bytes.
  An unescaped non-ASCII scalar rejects rather than being UTF-8-encoded; its
  diagnostic name and span are not pinned here. The short and non-hex escapes
  reject with `InvalidEscape` and emit no literal token. Their primary spans are
  exactly `\x4` (the closing delimiter is excluded) and `\xG` respectively.
  The separate `0x[…]` literal form is unchanged and is not exercised here.
- why: the exhaustive value sweep catches truncation and signed-byte mistakes;
  the `\x41BC` control flips if the scanner greedily consumes later hex digits.
  The non-ASCII control prevents an implicit String-to-UTF-8 path.

### surface/literals/raw-triple-backslashes-are-data
- spec: `31 §3` (raw triple strings perform no escape processing)
- given: the raw string `"""\n\q\u{D800}\xGG\\"""`.
- expect: it is accepted and decodes to exactly the source-scalar sequence
  `\n\q\u{D800}\xGG\\`: every backslash is ordinary data. It raises no
  `InvalidEscape`; only the pre-existing raw-string delimiter and multiline
  rules apply.
- why: the body deliberately combines a valid common escape, an unrecognized
  escape, an invalid scalar escape, a malformed byte escape, and a doubled
  backslash. Running the ordinary escape scanner over raw strings changes the
  value or rejects this single fixture.

## Error-span and termination boundary

### surface/literals/invalid-escape-span-precedes-unterminated
- spec: `31 §3` (`InvalidEscape` span and committed-escape precedence)
- given: three incomplete ordinary literals: a String whose closing quote
  arrives after `\u{41`, a Char whose line boundary arrives immediately after
  a backslash, and a byte string whose input ends after `\x4`. Pair them with
  ordinary String, Char, and byte-string literals that end without a closing
  delimiter but have no pending escape production.
- expect: every incomplete-escape fixture rejects with `InvalidEscape`, emits no
  literal token, and does not raise the ordinary unterminated-literal error. Its
  primary span starts at the backslash and is exactly `\u{41`, `\`, or `\x4`;
  the closing quote, line boundary, or end of input is excluded. Each paired
  no-pending-escape fixture retains its existing unterminated-literal behavior
  and is not reclassified as `InvalidEscape`; this contract does not name or
  respan that existing diagnostic.
- why: the boundary pair flips if delimiter, line, or EOF handling runs before
  the escape state, while the ordinary-unterminated controls prevent the new
  precedence rule from swallowing a distinct error family.
