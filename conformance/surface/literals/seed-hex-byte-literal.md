# Hex byte-literal lexing conformance — seed cases

Format: `../../README.md`. These cases pin the lexical form of the `0x[…]`
hex byte-literal in `spec/30-surface/38-ffi-io.md §1.1` (authoritative) and its
cross-referenced note in `spec/30-surface/31-lexical.md §3` (Escape
repertoire). The pin, landed by `LANG-BYTES-HEX-CONTIGUOUS-spec`, is that the
`0x[…]` body is a **contiguous, even-length run of ASCII hexadecimal digits
with no internal whitespace or separator**, lexed as consecutive nibble pairs.
`0x[de ad]` is a lexical error, not a spelling of `0x[dead]`.

Escape recognition and the `b"…"` byte-string form are owned by
`seed-escapes.md`; `Bytes` values, operations, and binary I/O are owned by
`../bytes-io/seed-bytes-io.md` (which already exercises accepted `0x[…]`
literals as inputs). This seed owns only the `0x[…]` body's lexical shape.

The error boundary is asymmetric, matching `seed-escapes.md`: the **verdict**
(accepted with a `Bytes` token vs. rejected with no literal token) and the
decoded byte sequence of an accepted literal are normative here; the **name and
span** of the lexical-error diagnostic for a malformed `0x[…]` body are not
fixed by this contract.

## Accepted contiguous form and its byte sequence

### surface/literals/hex-byte-literal-contiguous-even-accepts
- spec: `38 §1.1`; `31 §3` (Escape repertoire, `0x[…]` note)
- given: the byte literals `0x[dead]` and `0x[deadbeef]`, each in a complete
  parseable carrier (e.g. `fn f : Bytes = 0x[dead]`).
- expect: each is accepted and lexes to exactly one `Bytes` literal token whose
  bytes are the consecutive nibble pairs of its body — `0x[dead]` ⇒ the
  two-byte sequence `[0xde, 0xad]`, and `0x[deadbeef]` ⇒ the four-byte sequence
  `[0xde, 0xad, 0xbe, 0xef]`. Upper- and lower-case hex digits in the body are
  both accepted and denote the same bytes.
- why: this is the positive control the two rejection cases below flip against.
  Asserting the decoded byte sequence (a structural output), not merely
  "accepted", catches a lexer that tokenizes `0x[…]` but mis-pairs the nibbles.

## Contiguity: internal whitespace or separator is a lexical error

### surface/literals/hex-byte-literal-internal-whitespace-rejects
- spec: `38 §1.1` ("no internal whitespace or separator"; `0x[de ad]` is a
  lexical error, not a spelling of `0x[dead]`)
- given: the source `0x[de ad]` — the byte sequence of the accepted control
  `0x[dead]`, with a single ASCII space inserted between the two nibble pairs.
- expect: it is **rejected** as a lexical error and **emits no literal token**;
  in particular it is not lexed as, and does not normalize to, the accepted
  `0x[dead]`.
- why: this is a verdict flip against
  `hex-byte-literal-contiguous-even-accepts` on a **shared** body — the two
  differ only by the internal space, so a flipped or whitespace-stripping lexer
  reddens exactly here. The rejection is gated on the contiguity rule, not on
  invalid hex: `de` and `ad` are each valid hex, so a coincidental
  "invalid-digit" rejection cannot pass this case — only rejecting the
  **separator** does. This is the case the pin exists to witness.

## Even length: an odd number of hex digits is a lexical error

### surface/literals/hex-byte-literal-odd-length-rejects
- spec: `38 §1.1` ("an even number of ASCII hexadecimal digits", "lexed as
  consecutive nibble pairs")
- given: the source `0x[abc]` — three ASCII hex digits, an odd body length.
- expect: it is **rejected** as a lexical error and **emits no literal token**;
  it is not silently completed to `0x[0abc]`, `0x[abc0]`, or `0x[ab]` with a
  trailing digit dropped.
- why: `a`, `b`, and `c` are each valid hex digits, so the rejection is gated on
  the **odd nibble count** (an incomplete final byte), not on an invalid digit —
  a coincidental invalid-digit rejection cannot pass this case. Paired with the
  even-length control above, this pins the even-length half of the rule
  independently of the no-separator half.
