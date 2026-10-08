# NFA path acceptance, determinization, and emptiness

Format: `../../README.md`.
Spec: `spec/50-stdlib/61-formal-languages.md` §3.

This seed covers the public NFA contract added by
`SPEC-FORMAL-LANGUAGES-NFA-CONTRACT`. Its cases are grounded in §3 and the
Architect's measured D0 ruling `evt_j3gq9c7y5ya5`. The D0 check established
feasibility; it is not execution evidence for this seed.

No §3 package implementation or execution is claimed here. These cases are
authored oracles, not reported test runs. No test, Ken check, Cargo test, or
CI run is claimed.

## Fixture notation

- In finite fixtures, `0`, `1`, `2`, and `3` abbreviate distinct values of
  `Fin n`. They are not `Int` states or equality operations. `a` denotes
  `False`; `b` denotes `True`.
- An edge set lists exactly the triples `(s, x, t)` for which
  `nfa_step s x t` is `True`. Every unlisted triple is `False`.
- `I` and `F` name the states where `nfa_initial` and `nfa_final` are
  `True`; `[]` is the empty list. Each path lists the state after each input
  symbol, as §3.1 requires.
- Positive `nfa_accepts` expectations mean the Ω proposition is inhabited by
  an initial state and path. Negative expectations name a finite reason no
  such witness exists; they do not treat the Ω result as a Boolean.
- All cases are durable behavioral invariants. Literal state and word values
  are fixed fixture data. Equivalent fixture refactorings may remain green;
  changing a result for these fixtures requires a contract change.

## Path acceptance

### stdlib/formal-languages/nfa-empty-word-checks-final-state

- spec: `spec/50-stdlib/61-formal-languages.md` §3.1.
- promise class: durable invariant.
- given: `q = Bool`, `a = Bool`, no edges, `I = {False}`, and word `[]` with
  path `[]`. Compare two NFAs that differ only in `F`: first `F = {False}`,
  then `F = {True}`.
- expect: in the first NFA, `path_accepts ... False [] []` and
  `nfa_accepts ... []` are proved. In the second, the path proposition reduces
  to `Equal Bool False True` and `nfa_accepts ... []` is disproved.
- measured: inhabitation of the empty-path proposition and the enclosing NFA
  acceptance proposition, with the final predicate toggled.
- claimed: an empty word is accepted from a start state exactly when that
  state is final.
- the gap: `I = {False}` fixes the only possible start; `q = Bool` and the
  empty word/path leave no transition or alternate path to explain the result.
- why: the Nil/Nil case checks the current state's final predicate. The
  one-field pair catches a path checker that accepts the empty word without
  consulting `nfa_final`.

### stdlib/formal-languages/nfa-path-length-mismatch-is-bottom

- spec: `spec/50-stdlib/61-formal-languages.md` §3.1.
- promise class: durable invariant.
- given: `q = Bool`, `a = Bool`, no edges, `I = {False}`, and
  `F = {False}`. Check both unequal length shapes: word `[]` with path
  `[False]`, and word `[False]` with path `[]`.
- expect: the specified `path_accepts` proposition reduces to `Bottom` for
  each mismatched pair. Neither specified list can serve as the witness path
  of `NfaAcceptance`; no claim is made about a different, correctly sized
  path for the same word.
- measured: the result for each explicit word/path length pair.
- claimed: `path_accepts` rejects both a path longer than its word and a path
  shorter than its word.
- the gap: these two closed fixtures reach the two mismatch arms directly;
  they make no claim about other inputs or about NFA acceptance by another
  path of matching length.
- why: the cases reach each mismatched-length branch independently: path
  longer than word, and path shorter than word. A catch-all that accepts
  either mismatch changes the expected result.

### stdlib/formal-languages/nfa-nonempty-path-requires-final-endpoint

- spec: `spec/50-stdlib/61-formal-languages.md` §3.1.
- promise class: durable invariant.
- given: `q = Fin 2`, `a = Bool`, edge set `{(0, a, 1)}`, `I = {0}`, word
  `[a]`, and path `[1]`. Compare `F = {1}` with `F = {}`; only the final
  predicate changes.
- expect: with `F = {1}`, both `path_accepts ... 0 [a] [1]` and
  `nfa_accepts ... [a]` are proved. With `F = {}`, the path proposition is
  uninhabited because its endpoint is not final, and `nfa_accepts ... [a]` is
  disproved.
- measured: acceptance of the same one-edge path under the two final
  predicates.
- claimed: the last state in a nonempty path must satisfy `nfa_final`.
- the gap: the edge, initial state, word, and path are fixed; the negative
  fixture has no alternate edge or initial state that could explain refusal.
- why: this controlled pair reaches the recursive Nil/Nil endpoint after one
  edge. It catches D0 mutant M3, which ignores `nfa_final` at the end of the
  word: that mutant would accept both sides of the pair.

### stdlib/formal-languages/nfa-acceptance-requires-an-initial-state

- spec: `spec/50-stdlib/61-formal-languages.md` §3.1.
- promise class: durable invariant.
- given: the same edge, final predicate, word, and path as the preceding
  case. Compare `I = {0}` with `I = {}`; only the initial predicate changes.
- expect: `path_accepts ... 0 [a] [1]` remains proved in both NFAs, but
  `nfa_accepts ... [a]` is proved only with `I = {0}`. With no initial state,
  no `NfaAcceptance` witness exists.
- measured: `path_accepts` is unchanged while NFA acceptance changes with
  the initial predicate.
- claimed: NFA acceptance requires a path's starting state to be initial.
- the gap: the path and final predicate are held fixed and the fixture has no
  other initial state.
- why: the path predicate is relative to its supplied starting state;
  `NfaAcceptance` separately requires proof that this state is initial. This
  pair catches implementations that omit that field from acceptance.

## Determinization directions

For the following fixtures, `fq = fin_finite 4`; no `DecEq` argument is
provided. The word `[a, b]` is read in that order.

### stdlib/formal-languages/nfa-determinize-complete-from-accepting-path

- spec: `spec/50-stdlib/61-formal-languages.md` §3.2–§3.3.
- promise class: durable invariant.
- given: `q = Fin 4`, `a = Bool`, edge set
  `{(0, a, 1), (0, a, 3), (1, b, 2)}`, `I = {0}`, `F = {2}`, word `[a, b]`,
  and path `[1, 2]`.
- expect: `nfa_accepts ... [a, b]` is proved from the displayed path.
  Applying `determinize_complete` proves
  `Equal Bool (accepts (subset_state q fq) a
  (determinize q a fq n) [a, b]) True`; direct evaluation of that acceptance
  also reduces to `True`.
- measured: one explicit NFA witness and the determinized DFA's acceptance
  for the same fixed word.
- claimed: every accepting NFA path yields an accepting determinized-DFA
  result.
- the gap: the displayed path is independently provided, and `fq` covers
  every state of `Fin 4`; this fixture tests one instance, not the theorem's
  universal quantification.
- why: the input NFA has two successors on its first symbol, but only one
  branch reaches a final state. Independently tracing the fixture gives
  `{0}` →a `{1, 3}` →b `{2}`, so the final predicate is true. This tests
  existential path acceptance and the NFA-to-DFA direction without deriving
  the expected path from the determinized result.

### stdlib/formal-languages/nfa-determinize-sound-from-dfa-acceptance

- spec: `spec/50-stdlib/61-formal-languages.md` §3.2–§3.3.
- promise class: durable invariant.
- given: the same NFA and word as
  `nfa-determinize-complete-from-accepting-path`.
- expect: `accepts (subset_state q fq) a (determinize q a fq n) [a, b]`
  reduces to `True`. Applying `determinize_sound` to that independently
  evaluated `Equal Bool ... True` produces a proof of
  `nfa_accepts ... [a, b]`.
- measured: the computed DFA acceptance and the Ω result supplied by the
  theorem on that same word.
- claimed: an accepting determinized-DFA run corresponds to an accepting NFA
  path.
- the gap: the Boolean antecedent is independently observed as `True`, and
  the explicit NFA path is a separate witness; one fixture does not prove
  the theorem for every word.
- why: this reaches the antecedent of the DFA-to-NFA theorem with a true
  observation; it does not count an implication whose antecedent is false.
  The explicit path is an independent witness for its Ω conclusion, and the
  set trace above is an independent oracle for the Boolean antecedent.

### stdlib/formal-languages/nfa-determinize-uses-forward-edges

- spec: `spec/50-stdlib/61-formal-languages.md` §3.2–§3.3.
- promise class: durable invariant.
- given: `q = Fin 4`, `a = Bool`, edge set
  `{(0, a, 1), (0, a, 3), (1, b, 2)}`, and `I = F = {0, 2}`. For word
  `[a, b]`, path `[1, 2]` is accepted from state `0`.
- expect: `nfa_accepts ... [a, b]` and the determinized DFA's acceptance are
  both proved/`True`. If subset construction reverses the edge test to
  `nfa_step t x s`, its first step from `{0, 2}` is empty and the DFA
  acceptance reduces to `False`.
- measured: the NFA witness and DFA result on the same word under the
  forward relation; the predicted reversed-edge result is the opposite.
- claimed: subset transitions follow `nfa_step s x t`, from source `s` to
  destination `t`.
- the gap: `I = F` keeps D0's initial/final substitution mutant inert, and
  the exact edge table makes the forward and reversed first steps differ.
- why: the edge is directional and the branch through `3` is dead. The
  correct and reversed implementations produce opposite results on the same
  input. `I = F` makes D0 mutant M1 (using `nfa_initial` as the final
  predicate) inert here, so this case isolates edge direction.

### stdlib/formal-languages/nfa-determinize-does-not-invent-acceptance

- spec: `spec/50-stdlib/61-formal-languages.md` §3.1–§3.3.
- promise class: durable invariant.
- given: the NFA from `nfa-determinize-complete-from-accepting-path`, with
  word `[b]`. The initial state `0` has no `b`-edge; the path list has one
  state per symbol as usual.
- expect: `nfa_accepts ... [b]` is disproved and the determinized DFA's
  acceptance reduces to `False`.
- measured: NFA witness existence and DFA acceptance for a word with no
  outgoing edge from its sole initial state.
- claimed: determinization does not invent an accepting path absent from the
  NFA.
- the gap: this fixture rules out only the named word and NFA; the positive
  `[a, b]` control exercises a real accepting path through the same relation.
- why: this negative control makes the soundness side sensitive to a
  determinization that invents a transition or accepting path. Its positive
  control is the accepted `[a, b]` word above.

## Finite emptiness

### stdlib/formal-languages/nfa-finite-empty-language

- spec: `spec/50-stdlib/61-formal-languages.md` §3.2–§3.3.
- promise class: durable invariant.
- given: `q = Fin 3`, `a = Bool`, `fq = fin_finite 3`,
  `fa = bool_finite`, edge set `{(0, a, 1), (1, b, 2)}`,
  `I = {0}`, and `F = {}`.
  This is the empty-language member of the pair: the nonempty case below uses
  the same carrier, certificates, edges, and initial predicate, changing only
  `F`.
- expect: `nfa_is_empty q a fq fa n` reduces to `True`. For any fixed word
  `w`, the instance of `nfa_is_empty_rejects` proves that an accepted NFA
  proposition would imply `Bottom`; in fact no final state exists, so no
  `nfa_accepts ... w` proof exists.
- measured: the emptiness Boolean and the instantiated rejection law for
  this finite NFA.
- claimed: a `True` emptiness result excludes every accepted word.
- the gap: the final predicate is false for every state, so the no-acceptance
  result is independent of the search implementation; the theorem instance
  separately checks the exported implication.
- why: this is a finite but empty language, not an automaton with no states.
  D0 mutant M1, substituting `nfa_initial` for `nfa_final`, makes the initial
  mask accepting at the empty word and flips the emptiness result to `False`.
  The reversed-edge mutant alone leaves the no-final-state result `True`.

### stdlib/formal-languages/nfa-finite-nonempty-language

- spec: `spec/50-stdlib/61-formal-languages.md` §3.2–§3.3.
- promise class: durable invariant.
- given: the same carrier, certificates, edges, and initial predicate as
  `nfa-finite-empty-language`, but `F = {2}`, word `[a, b]`, and path
  `[1, 2]`. Only the final predicate changes.
- expect: `nfa_accepts ... [a, b]` is proved and
  `nfa_is_empty q a fq fa n` reduces to `False`. Applying
  `determinize_complete` and the §2 finite-state decision gives the same
  nonempty result through the constructed Dfa.
- measured: one explicit accepted word and the finite emptiness Boolean.
- claimed: a finite NFA with an accepting path is not empty.
- the gap: this fixture shares the empty-language case's carrier,
  certificates, edges, and initial predicate; only `nfa_final` changes, and
  the accepting path is independently displayed.
- why: paired with `nfa-finite-empty-language`, this catches both constant
  answers to the emptiness decision. Both certificates are explicit; the
  state enumeration is needed for the mask and the alphabet certificate for
  §2's search.

## Coverage and limits

- Path acceptance covers Nil/Nil finality, both length-mismatch branches,
  finality after a nonempty path, and the separate initial-state predicate.
- `determinize_complete` and `determinize_sound` are each instantiated with
  an accepted word; the finite emptiness pair exercises both Boolean results.
- D0's reverse-edge mutant is exercised with an NFA whose initial and final
  predicates coincide. Its wrong result cannot be attributed to D0's
  initial/final substitution mutant.
- The seed does not test §1's standalone Dfa contract or §2's general finite
  reachability contract. It uses those packages only as §3 requires. It does
  not claim a complexity bound, an NFA witness-returning operation, or an
  ε-transition.
- No case asserts repository text, declaration counts, private mask layout,
  or a particular implementation strategy.
