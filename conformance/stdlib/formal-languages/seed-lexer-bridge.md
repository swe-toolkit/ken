# Cursor elements and the DFA lexer bridge

Format: `../../README.md`.
Spec: `spec/50-stdlib/61-formal-languages.md` §6.

These are **runtime oracles** for the bounded cursor view and streaming
DFA runner. Evaluate the closed values with the Ken interpreter, as in
the argument-cursor decoder fixtures; do not replace them with
`Proved` terms about byte conversions. The Architect's D0
`evt_2mn6kw5efep62` checked a separate development and evaluated
`bytes_to_list (bytes_encode "ab")` to octets 97, 98. It did not run
this seed or deliver the catalog packages. No Ken check, interpreter
run of these cases, Cargo test, or CI run is claimed by this authored
seed. Every case has promise class **durable invariant**.

## Shared closed fixtures

Use `q = Bool`, `c = ArgCursor`, `el = UInt8`, `loc = ArgLocation`, and
`ops = arg_cursor_ops`, with its separately checked `arg_cursor_laws`.
The existing argument cursor reads the structural `List UInt8` view of
`Bytes` and normalises empty arguments. Its `cursor_peek` and
`cursor_advance` expose the bytes in order. The fixtures are:

```ken
even_length : Dfa Bool UInt8 =
  MkDfa Bool UInt8 (λs. λx. bool_not s) True (λs. s)
last_is_b : Dfa Bool UInt8 =
  MkDfa Bool UInt8
    (λs. λx. eq_int (uint8_to_int x) (98 : Int)) False (λs. s)
```

`even_length` toggles once per input byte regardless of its value;
`last_is_b` accepts exactly when the last consumed byte is ASCII `b`
(98). The interpreter must expose the resulting `Bool` and cursor
list values; equality of two computed lists or DFA states below is a
comparison of **runtime results**, not a kernel proof term. Bracketed
argument lists are `List Bytes` inputs to `arg_cursor_start`; `[]`
denotes the empty list of the stated element type.

## Accept, reject, and end position

### stdlib/formal-languages/lexer-accepts-two-bytes

- spec: `spec/50-stdlib/61-formal-languages.md` §6.1–§6.2.
- promise class: durable invariant (runtime oracle).
- given: `cur = arg_cursor_start [bytes_encode "aa"]` and
  `d = even_length`. Evaluate `cursor_elements ops cur`,
  `cursor_peek ops cur`, `cursor_elements ops (cursor_advance ops cur)`,
  and `dfa_cursor_accepts Bool ArgCursor UInt8 ArgLocation d ops cur`.
- expect: the element list is `[97, 97]` as `UInt8`; the first peek is
  `Some UInt8 97`, and the advanced element list is `[97]`. The
  original list equals the first byte consed onto the advanced list,
  a positive instance of `cursor_elements_peek_some` under
  `arg_cursor_laws`. Acceptance is `True` after two toggles.
- measured: the runtime list head/tail and accepted Boolean, not a
  `Proved` literal-byte equation.
- claimed: a successful peek contributes one element, advance
  exposes the suffix, and the streaming runner processes both bytes.
- the gap: a closed byte instance does not prove the universally
  quantified cursor-progress law or runner bridge.
- why: omitting one of the two steps produces rejection; a missing
  advanced element changes the observed suffix list.

### stdlib/formal-languages/lexer-rejects-one-byte

- spec: `spec/50-stdlib/61-formal-languages.md` §6.2.
- promise class: durable invariant (runtime oracle).
- given: `cur = arg_cursor_start [bytes_encode "a"]`, `d = even_length`,
  and the same operations and start state as the two-byte case.
- expect: `cursor_elements ops cur` evaluates to `[97]` as `UInt8`;
  `dfa_cursor_run Bool ArgCursor UInt8 ArgLocation d ops True cur`
  evaluates to `False`, and `dfa_cursor_accepts` evaluates to `False`.
- measured: one-byte result and rejected Boolean on the same DFA.
- claimed: `False` is a rejection, not an error or an empty result.
- the gap: one odd-length word does not establish all-word agreement.
- why: skipping `step` leaves the initial `True` state and incorrectly
  accepts this word; the adjacent two-byte positive case also rules
  out a constant-`False` matcher.

### stdlib/formal-languages/lexer-accepts-empty-input

- spec: `spec/50-stdlib/61-formal-languages.md` §6.1–§6.2.
- promise class: durable invariant (runtime oracle).
- given: `cur = arg_cursor_start (Nil Bytes)` and `d = even_length`.
- expect: `cursor_peek ops cur` evaluates to `None UInt8` and
  `cursor_elements ops cur` evaluates to `Nil UInt8`, instantiating
  `cursor_elements_peek_none`. `dfa_cursor_run` from `True` returns
  `True`, and `dfa_cursor_accepts` returns `True` because the initial
  state is final; `run Bool UInt8 d True (Nil UInt8)` also returns
  `True`.
- measured: runtime end peek, empty list, unchanged state, and
  acceptance, not a checked proof of byte conversion.
- claimed: empty input consumes no transition and uses the start
  state's final test.
- the gap: this is a closed end position, not a proof of the generic
  `None` theorem for every `CursorOps`.
- why: applying an unconditional first transition would reject the
  empty input, unlike the expected unchanged state.

## Agreement and cursor traversal

### stdlib/formal-languages/lexer-run-agrees-with-list-run

- spec: `spec/50-stdlib/61-formal-languages.md` §6.2.
- promise class: durable invariant (runtime oracle).
- given: `cur = arg_cursor_start
  [bytes_encode "aa", bytes_encode "a"]`, `d = even_length`,
  and supplied state `s = True`. Independently obtain the three-byte
  word by appending the two arguments' `bytes_to_list` views; do not
  construct it by calling `cursor_elements` under test.
- expect: `cursor_elements ops cur` evaluates to `[97, 97, 97]` as
  `UInt8`, the independent concatenation. Both
  `dfa_cursor_run Bool ArgCursor UInt8 ArgLocation d ops s cur` and
  `run Bool UInt8 d s` on the independent list evaluate to `False`.
  `run` on `cursor_elements ops cur` also returns `False`, and
  `dfa_cursor_accepts` returns `False`. Compare these computed
  interpreter values, not an attempted `Proved` byte-list equality.
- measured: runtime state equality on three bytes across an argument
  boundary and the same start state's acceptance.
- claimed: the state and acceptance bridge laws agree with `run` and
  `accepts` over the cursor's bounded elements; the independent list
  guards against the runner and cursor view dropping the same input.
- the gap: a closed three-byte observation does not assert the
  deferred general concatenation theorem for every argument list or
  prove either generic bridge law.
- why: stopping at the first argument yields `True`, opposite the
  required three-step `False`; dropping the final byte from both the
  runner and `cursor_elements` is exposed by the independent list.

### stdlib/formal-languages/lexer-crosses-empty-argument

- spec: `spec/50-stdlib/61-formal-languages.md` §6.1–§6.2.
- promise class: durable invariant (runtime oracle).
- given: `cur = arg_cursor_start
  [bytes_encode "a", bytes_encode "", bytes_encode "a"]` and
  `d = even_length`.
- expect: the runtime `cursor_elements ops cur` list is `[97, 97]`
  as `UInt8`; after one `cursor_advance ops cur` the next peek is
  `Some UInt8 97`, crossing the empty middle argument.
  `dfa_cursor_accepts` is `True`, and list-based `accepts` on the
  two-byte runtime word is also `True`.
- measured: list length/content, next peek, and paired acceptance
  across an empty argument.
- claimed: cursor normalisation preserves later byte input; an empty
  argument is not a stream-termination signal.
- the gap: this closed case does not specify argument locations or
  a general byte-concatenation equation.
- why: stopping at the middle empty argument observes one byte and
  rejects, and a runner that silently skips the first byte also
  rejects; both differ from the expected two-byte acceptance.

### stdlib/formal-languages/lexer-uses-advanced-byte

- spec: `spec/50-stdlib/61-formal-languages.md` §6.2.
- promise class: durable invariant (runtime oracle).
- given: `d = last_is_b`; compare `arg_cursor_start
  [bytes_encode "ab"]` with `arg_cursor_start [bytes_encode "aa"]`.
  Both words have length two and begin with byte 97; their second
  bytes are 98 and 97 respectively.
- expect: `dfa_cursor_accepts d ops` returns `True` on `"ab"` and
  `False` on `"aa"`. List-based `accepts d` over each closed
  argument's independent `bytes_to_list` view agrees. The runtime
  `cursor_elements` lists are `[97, 98]` and `[97, 97]`.
- measured: opposite decisions at the same length, with the first
  byte and DFA held fixed.
- claimed: the runner feeds the **advanced** cursor's next symbol to
  the next transition rather than repeating the first peek.
- the gap: these two concrete words cannot prove the general bridge.
- why: repeating the first byte or ignoring `cursor_advance` makes
  both words end as `"aa"` and rejects both; a parity-only DFA could
  not detect this byte-value fault.

## Law-side-condition control

### stdlib/formal-languages/cursor-stuck-advance-is-not-lawful

- spec: `spec/50-stdlib/61-formal-languages.md` §6.1–§6.2.
- promise class: durable invariant (runtime oracle).
- given: a `CursorOps Unit Bool Unit` dictionary with
  `remaining = λu. Suc Zero`, `peek = λu. Some Bool True`,
  `advance = λu. u`, and `locate = λu. u`; evaluate from `MkUnit`.
  Use `toggle : Dfa Bool Bool = MkDfa Bool Bool
  (λs. λx. bool_not s) True (λs. s)` for the runner comparison.
  This dictionary has no `CursorLaws` witness: advancing does not
  decrease the remaining count. No byte conversion is involved.
- expect: `cursor_elements` evaluates to `[True]` from fuel one,
  whereas `Cons Bool True (cursor_elements (advance MkUnit))`
  evaluates to `[True, True]`. The `cursor_elements_peek_some`
  conclusion is **not** asserted for this unlawful dictionary.
  Without a `CursorLaws` argument, both `dfa_cursor_run` from `True`
  and `run Bool Bool toggle True (cursor_elements ops MkUnit)`
  return `False` after exactly one transition.
- measured: two unequal runtime lists, the absent progress premise,
  and equal bounded runner/list results without a laws parameter;
  not a rejected byte theorem or a package proof.
- claimed: `dfa_cursor_run_elements` is still a bounded-scan bridge
  for arbitrary `CursorOps`, but `peek_some` must retain its
  `CursorLaws` parameter.
- the gap: runtime inequality illustrates the necessity of a law;
  it does not manufacture `CursorLaws` or a generic countertheorem.
- why: dropping the law parameter would require equality of the
  visibly different one- and two-element lists.

## Coverage boundary

The public Cursor laws each have a positive closed argument-cursor
instance: `cursor_elements_peek_none` at empty input and
`cursor_elements_peek_some` at `"aa"` under `arg_cursor_laws`. The
DFA state and acceptance bridges have closed paired runtime
observations, including an independent byte-list comparison. These
oracles do not substitute for four kernel-checked public theorems.
They say nothing about maximal munch, an end cursor, location
coordinates, cross-instance cursor equivalence, or a generic
concatenation of all argument bytes. The stuck-advance control keeps
the progress side condition visible.
