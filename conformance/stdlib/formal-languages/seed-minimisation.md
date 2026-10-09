# Dfa equivalence and minimisation

Format: `../../README.md`.
Spec: `spec/50-stdlib/61-formal-languages.md` §5.

These authored behavioral oracles follow Architect D0 `evt_30hjxcgryy66c`.
That checked development measured feasibility, not execution of this seed
or delivery of `Algorithm.FormalLanguages.Minimisation`. No Ken check,
Cargo test, CI result, catalog implementation, or publication is claimed
for these cases. Every case has promise class **durable invariant**: it
pins accepted words, equivalence decisions, future-language agreement,
or equality of reached or canonical states, never which representative
`canonical` chooses.

## Shared closed fixtures

The common alphabet is `Bool` with explicit `fa = bool_finite`. For
state carrier `Bool`, supply `fq = bool_finite`; for state carrier
`Unit`, supply `fr = unit_finite`. The four closed automata are:

```ken
accept_all : Dfa Unit Bool =
  MkDfa Unit Bool (λs. λx. s) MkUnit (λs. True)
reject_all : Dfa Unit Bool =
  MkDfa Unit Bool (λs. λx. s) MkUnit (λs. False)
twin : Dfa Bool Bool =
  MkDfa Bool Bool (λs. λx. bool_not s) True (λs. True)
only_empty : Dfa Bool Bool =
  MkDfa Bool Bool (λs. λx. False) True (λs. s)
```

Both one-state machines ignore symbols; they vary only in their final
predicate. `twin` toggles its state for every symbol but accepts every
word from either state. `only_empty` accepts just `[]` from initial
state `True`; it rejects all nonempty words and accepts none from state
`False`. `[]` is `Nil Bool` and `[False, True]` is the ordered two-symbol
list. For each case, both state certificates and `fa` are supplied to
the public decision or `minimise`; the premise of `same_future` itself
does not need a certificate. No fixture supplies `DecEq Bool` as an
argument to any §5 operation.

## Whole-machine equivalence

### stdlib/formal-languages/equivalent-different-carriers

- spec: `spec/50-stdlib/61-formal-languages.md` §5.1.
- promise class: durable invariant.
- given: `d = twin : Dfa Bool Bool`, `e = accept_all : Dfa Unit Bool`,
  certificates `bool_finite`, `unit_finite`, `bool_finite`; words `[]`,
  `[False]`, and `[False, True]`.
- expect: `equivalent Bool Unit Bool fq fr fa d e = True`.
  On each listed word, both `accepts` return `True`. More strongly,
  both final predicates return `True` on **every** state, so the
  all-words agreement premise of `equivalent_complete` is inhabited;
  the decision's `True` result supplies the antecedent of
  `equivalent_sound`. Both theorem directions have positive premises,
  independently of the three example words.
- measured: cross-carrier Boolean equivalence and three paired
  acceptance results, plus nonvacuous theorem antecedents.
- claimed: language equivalence is about accepted words, not equal
  state representations or transition graphs.
- the gap: a finite word sample alone cannot prove equality on all
  words. The constant-final argument supplies the universal premise,
  while the package must still deliver the checked theorems.
- why: a state-shape comparison would refuse these different carriers;
  a product disagreement check must return none for every word.

### stdlib/formal-languages/equivalent-empty-word-final-flip

- spec: `spec/50-stdlib/61-formal-languages.md` §5.1.
- promise class: durable invariant.
- given: compare `equivalent` of `accept_all` with itself against
  `equivalent` of `accept_all` with `reject_all`; all have carrier
  `Unit`, alphabet `Bool`, `fq = fr = unit_finite`, and
  `fa = bool_finite`. The only changed automaton field is the second
  machine's final predicate.
- expect: the self-pair returns `True`; the opposite-final pair
  returns `False`. At `[]`, the first machine accepts and the second
  rejects, so the product's disagreement final is already `True` at
  its start; the self-pair has no disagreement on any word.
- measured: opposite decisions and opposite accepted-word results
  on one fixed input with the transition/start/certificates held fixed.
- claimed: the decision returns `False` when a disagreement word
  exists and `True` when no such word exists.
- the gap: the empty-word witness proves the negative decision for
  this pair, not the general soundness/completeness laws. The positive
  self-pair controls against a constant-`False` decision.
- why: inverting `disagree`'s equal/unequal classification flips both
  decisions; examining only the negative pair would miss a constant
  negative implementation.

### stdlib/formal-languages/equivalent-disagreement-after-transition

- spec: `spec/50-stdlib/61-formal-languages.md` §5.1.
- promise class: durable invariant.
- given: `d = only_empty : Dfa Bool Bool`, `e = accept_all`, with
  `fq = bool_finite`, `fr = unit_finite`, `fa = bool_finite`.
  Compare words `[]` and `[False]` on the **same** pair of machines.
- expect: both machines accept `[]`. On `[False]`, `only_empty`
  rejects and `accept_all` accepts; therefore
  `equivalent Bool Unit Bool fq fr fa d e = False`. The finite search
  need not return `[False]` as its *chosen* witness; existence of this
  disagreement suffices.
- measured: equal outputs at the start and unequal outputs after a
  one-symbol transition, plus the Boolean equivalence decision.
- claimed: the product decision searches reachable disagreement, not
  only the two initial final predicates.
- the gap: the fixed word witnesses failure of universal agreement;
  no shortest witness, search order, or `find_word` result is pinned.
- why: an implementation that checks only initial final values returns
  `True` here, while the preceding empty-word case already detects
  wrong final comparison at the start.

## State-specific future decisions

### stdlib/formal-languages/equivalent-twin-states

- spec: `spec/50-stdlib/61-formal-languages.md` §5.1.
- promise class: durable invariant.
- given: `d = e = twin`, `s = True`, `t = False`,
  `fq = fr = fa = bool_finite`. The states are different Bool values.
- expect: `equivalent_states Bool Bool Bool fq fr fa d e s t = True`.
  `same_future Bool Bool Bool d e s t` is inhabited because both
  final predicates always return `True`, including after any word.
  Thus the independently grounded proposition is a positive premise
  for `equivalent_states_complete`, and the positive Boolean result
  is a premise for `equivalent_states_sound`. Both directions yield
  the specified agreement.
- measured: agreement of two distinct states for all suffixes and
  a positive state-equivalence decision with both law antecedents live.
- claimed: the decision classifies future languages, not `Equal Bool s t`.
- the gap: the proof of all-words agreement follows the fixture's
  constant final predicate, not merely two sample words; the general
  laws remain checked package obligations, not seed test verdicts.
- why: substituting state equality for language equality incorrectly
  returns `False` even though both states accept every continuation.

### stdlib/formal-languages/inequivalent-only-empty-states

- spec: `spec/50-stdlib/61-formal-languages.md` §5.1.
- promise class: durable invariant.
- given: `d = e = only_empty`, `s = True`, `t = False`,
  `fq = fr = fa = bool_finite`; input suffix `[]`.
- expect: `equivalent_states Bool Bool Bool fq fr fa d e s t = False`.
  `final d (run d True []) = True` but
  `final e (run e False []) = False`, so `same_future` is not
  inhabited on these states. The empty word is already a product
  disagreement witness.
- measured: negative state-equivalence decision and opposite
  empty-continuation acceptance from the two supplied states.
- claimed: a search from specified states detects inequivalent
  futures even when the two Dfa arguments are the same machine.
- the gap: `accepts` alone starts at `True` and would not exercise the
  `False` argument; the two `run`/`final` results do.
- why: treating `equivalent_states` as the whole-machine `equivalent`
  decision, which ignores its `s`/`t` arguments, returns `True` here.

## Preserved acceptance and reduced reached states

### stdlib/formal-languages/minimise-preserves-two-languages

- spec: `spec/50-stdlib/61-formal-languages.md` §5.2.
- promise class: durable invariant.
- given: construct `minimise Bool Bool bool_finite bool_finite d`
  separately for `d = twin` and `d = only_empty`; compare each with
  its own original on `[]`, `[False]`, and `[False, True]`.
- expect: `twin` and its minimised machine accept all three words.
  `only_empty` and its minimised machine accept `[]` but reject the
  two nonempty words. For each fixed word, the corresponding instance
  of `accepts_minimise` proves equality of the paired Boolean values.
- measured: accepted-word results for six original/minimised pairs,
  with both accepting and rejecting outcomes.
- claimed: minimisation preserves the original language across every
  finite word, rather than only the two representatives sampled here.
- the gap: six values cannot discharge the universal checked law; the
  theorem must be proved against the exported `minimise` definition.
  Neither these results nor that law chooses a representative value.
- why: merging the inequivalent states of `only_empty` would make
  one of its positive/negative results wrong; rejecting everything
  would fail the `twin` and empty-word controls.

### stdlib/formal-languages/minimise-reduced-twin-reached

- spec: `spec/50-stdlib/61-formal-languages.md` §5.2.
- promise class: durable invariant.
- given: `d = twin`, `fq = fa = bool_finite`,
  `m = minimise Bool Bool fq fa d`; let `u = []` and `v = [False]`.
  Let `s0 = start Bool Bool m` and compare `run m s0 u` and
  `run m s0 v`. From the original initial `True`, `twin` toggles
  state on `v` but not `u`.
- expect: both prefixes have identical acceptance for **every**
  continuation `w` (all words are accepted), so the premise of
  `minimise_reduced` is inhabited. Its conclusion is
  `Equal Bool (run m s0 []) (run m s0 [False])`. The equality is the
  oracle; no assertion assigns `True` or `False` to either side.
- measured: equality of the two reached states of the minimised
  machine under a nonvacuous same-future premise.
- claimed: redundant states collapse **after reachability** even
  though the original twin has two different reachable states.
- the gap: this case covers one pair of prefixes, not arbitrary
  equal-future prefixes. The general theorem is checked by Ken, and
  no state-count minimality or representative enumeration is claimed.
- why: `canonical := identity` leaves the two original toggling
  states distinct, failing this equality while still preserving
  every accepted word in the preceding case.

### stdlib/formal-languages/minimise-does-not-collapse-different-futures

- spec: `spec/50-stdlib/61-formal-languages.md` §5.2.
- promise class: durable invariant.
- given: change only `d` in the preceding prefix comparison to
  `only_empty`, keeping `q = a = Bool`, `fq = fa = bool_finite`,
  `u = []` and `v = [False]`; let
  `m = minimise Bool Bool fq fa only_empty`.
- expect: `accepts m [] = True` and `accepts m [False] = False`.
  Their empty-continuation results differ, so the all-continuations
  premise of `minimise_reduced` is **not** inhabited for this pair;
  the theorem is not applied vacuously to claim equality. Equality
  of the two reached states would force equal final results, so
  these values also forbid collapsing them to one state.
- measured: a positive/negative acceptance pair under the same
  prefixes that collapsed for `twin`.
- claimed: reduced reached states do not merge futures that differ.
- the gap: this is an opposite-premise control, not a claim that
  `minimise_reduced` proves distinctness or cardinality minimality.
- why: a construction that canonicalises all states to one chosen
  representative passes the twin-equality oracle but violates these
  contrasting acceptance results.

## Canonicalisation laws without a chosen representative

### stdlib/formal-languages/canonical-same-future-only-empty

- spec: `spec/50-stdlib/61-formal-languages.md` §5.2.
- promise class: durable invariant.
- given: `d = only_empty`, `q = a = Bool`, `fq = fa = bool_finite`,
  `s = True`, and `c = canonical Bool Bool fq fa d s`. Compare the
  future acceptance from `c` and `s` on suffixes `[]`, `[False]`, and
  `[False, True]` without assigning a Bool value to `c`.
- expect: `same_future Bool Bool Bool d d c s` is inhabited, as
  `canonical_same_future` requires. In particular, the final results
  after `[]` agree and are `True`; the results after `[False]` and
  `[False, True]` agree and are `False`. The original future from
  `s` is independently known from `only_empty`'s transition and final
  test, not from the chosen canonical value.
- measured: equality of the canonical and original future results on
  one accepting and two rejecting suffixes, with the public all-words
  Ω guarantee as the law's expected conclusion.
- claimed: canonicalising a state preserves its entire future
  language, not only acceptance from the Dfa's initial state.
- the gap: three suffixes do not prove the universal statement;
  `canonical_same_future` must be checked for all words and states.
  This fixture asserts no particular representative, only its
  observable equality with the original state's future.
- why: mapping the accepting empty-word state to a state that rejects
  `[]` changes the first result even if later nonempty words still
  agree. The accepting and rejecting suffixes prevent a vacuous
  positive claim.

### stdlib/formal-languages/canonical-unique-twin-states

- spec: `spec/50-stdlib/61-formal-languages.md` §5.2.
- promise class: durable invariant.
- given: `d = twin`, `q = a = Bool`, `fq = fa = bool_finite`,
  `s = True`, `t = False`,
  `c_s = canonical Bool Bool fq fa d s`, and
  `c_t = canonical Bool Bool fq fa d t`. Both original states accept
  every suffix because `twin`'s final predicate is constantly `True`.
- expect: the independently grounded proposition
  `same_future Bool Bool Bool d d s t` is inhabited, including at
  `[]` and `[False]`. The positive instance of `canonical_unique`
  then concludes `Equal Bool c_s c_t`. No expectation says whether
  either canonical result is `True` or `False`.
- measured: a nonvacuous all-words premise and equality of the two
  canonical results for distinct original states.
- claimed: equal future languages normalise to one equal state in
  the same carrier, without deciding state equality as an input.
- the gap: a single equal-future state pair does not prove the
  universally quantified checked theorem or state-count minimality.
  Its expected equality is about the two results, not a selected
  enumeration entry.
- why: `canonical := identity` leaves `c_s = True` and `c_t = False`
  on this pair, violating their equality while preserving each
  state's accepted language; the fixture separates uniqueness from
  mere future preservation.

## Coverage boundary

The seed distinguishes constant acceptance, empty-word disagreement,
nonempty disagreement, same and different state futures, preserved
acceptance, and collapsing **only** redundant reached states. A positive
premise for each of the four equivalence laws and for
`minimise_reduced` is explicitly given. `canonical_same_future` now has
an accepting/rejecting-suffix instance, and `canonical_unique` has an
independently inhabited same-future premise on distinct twin states.
All eight laws have positive seed instances; none selects a fixed
canonical element.

There is no `DecEq q` argument, shortest-word promise, chosen state
representative, quotient carrier, state-count claim, exhaustive local
check, Nfa/Regex link, or complexity assertion. All eight §5 theorems
remain package obligations; closed examples are not their proofs.
