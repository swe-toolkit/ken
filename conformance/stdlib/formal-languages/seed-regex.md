# Regex denotation and derivative matching

Format: `../../README.md`.
Spec: `spec/50-stdlib/61-formal-languages.md` §4.

These authored oracles pin the public contract from
`SPEC-FORMAL-LANGUAGES-REGEX-CONTRACT` and Architect D0
`evt_3bf35x1gz9evy`. D0 checked a separate development for feasibility;
no Regex catalog implementation, seed execution, Ken check, Cargo test, or
CI run is claimed here. Cases describe program behavior, not source text.

## Fixture convention

All cases use `a = Bool` and the explicitly supplied canonical
`d : DecEq Bool` (`DecEq_instance_Bool`). `T` abbreviates `True`, `F`
abbreviates `False`, and `[]` is `Nil Bool`. The listed words and regular
expressions are closed fixture values. A positive `regex_lang` expectation
means its Ω proposition is inhabited by the stated public evidence; a
negative one means no such evidence exists. `regex_matches` is the Boolean
result, not the proposition. Literal words are inputs, not private
implementation shapes. Every case has promise class **durable invariant**.

## Base languages and alphabet discrimination

### stdlib/formal-languages/regex-fail-and-eps

- spec: `spec/50-stdlib/61-formal-languages.md` §4.1–§4.3.
- promise class: durable invariant.
- given: compare `Fail Bool` and `Eps Bool` on words `[]` and `[T]`,
  with the fixture's explicit `d`.
- expect: `regex_lang Bool (Fail Bool) []` and its `[T]` instance reduce
  to `Bottom`; `regex_matches` returns `False` for both. `regex_lang` for
  `Eps Bool` has a proof at `[]` and no proof at `[T]`, and the matcher
  returns `True` and `False`, respectively. `nullable Fail = False` and
  `nullable Eps = True`. At `Eps`/`[]`, the inhabited denotation is an
  antecedent for `nullable_complete`; its `True` nullability is an
  antecedent for `nullable_sound`.
- measured: each predicate's witness status and Boolean result on the
  same two words, plus a nonvacuous instance of both nullability laws.
- claimed: `Fail` denotes the empty language, while `Eps` denotes exactly
  the empty word; nullability agrees on a positive empty-word instance.
- the gap: these fixed inputs check those constructors, not all regexes;
  neither theorem is counted merely by applying it with a false premise.
- why: the two expressions have opposite results on the empty word, and
  `Eps` rejects a nonempty one.

### stdlib/formal-languages/regex-symbol-deceq-direction

- spec: `spec/50-stdlib/61-formal-languages.md` §4.1–§4.3.
- promise class: durable invariant.
- given: `r = Sym Bool T`, `d = DecEq_instance_Bool`, and words `[T]`,
  `[F]`, `[]`, and `[T, T]`. Compare `deriv Bool d T r` with
  `deriv Bool d F r` on the empty residual word.
- expect: `d.eq T T = True` and `d.eq F T = False`. The first derivative
  has the language of `Eps`, the second the language of `Fail`.
  `regex_lang Bool r [T]` is inhabited and
  `regex_matches Bool d r [T] = True`; the other three words have no
  denotation witness and return `False`. `deriv_sound` is instantiated
  with the first derivative's proof at `[]`; `deriv_complete` is
  instantiated with the independently constructed `Sym T` proof at
  `[T]`. Both antecedents are inhabited.
- measured: the two distinct comparator results, both residual languages
  at `[]`, four whole-word results, and both derivative-law directions.
- claimed: `Sym c` accepts one occurrence of precisely `c`, using the
  explicit `DecEq` dictionary; derivatives describe left quotients.
- the gap: equality is independently fixed by the Bool carrier and its
  public dictionary, not computed from the matcher under test. These
  concrete instances are not proofs of the laws' universal quantifiers.
- why: replacing `d.eq x c` by constant `True` makes `[F]` accepted;
  ignoring the result or the singleton length also changes a listed word.

## Constructors and composition

### stdlib/formal-languages/regex-alt-both-arms

- spec: `spec/50-stdlib/61-formal-languages.md` §4.1–§4.3.
- promise class: durable invariant.
- given: `r = Alt Bool (Sym Bool T) (Sym Bool F)`; compare `[T]`, `[F]`,
  `[]`, and `[T, T]`, holding `r` and `d` fixed.
- expect: the public `Or` evidence supplies a `regex_lang` witness for
  `[T]` through the left arm and `[F]` through the right arm; both
  matcher results are `True`. Neither `[]` nor `[T, T]` has a witness,
  and their matcher results are `False`. `nullable r = False`.
- measured: two accepted arms and two rejected shapes for one expression.
- claimed: `Alt` denotes union and its derivative accepts either branch.
- the gap: left and right have disjoint one-symbol witnesses; neither
  positive result is inferred by copying the matcher's own computation.
- why: dropping either alternation arm changes its corresponding accepted
  word, while accepting every word changes the negative controls.

### stdlib/formal-languages/regex-cat-split-order-and-empty-suffix

- spec: `spec/50-stdlib/61-formal-languages.md` §4.1–§4.3.
- promise class: durable invariant.
- given: `r = Cat Bool (Sym Bool F)
  (Alt Bool (Eps Bool) (Sym Bool T))`; compare `[F]`, `[F, T]`,
  `[T, F]`, `[T]`, and `[]` with the same `d`.
- expect: `Split` witnesses are `([F], [])` for `[F]` and
  `([F], [T])` for `[F, T]`, in that order. Both have inhabited
  `regex_lang` and matcher `True`. No valid split exists for
  `[T, F]`, `[T]`, or `[]`; their matcher results are `False`.
  `nullable r = False` despite its nullable right operand.
- measured: the factorisations and matcher results for two positive and
  three negative words, including the reversed word.
- claimed: `Cat` respects left-to-right concatenation and permits an
  empty **right** word without losing the left symbol.
- the gap: each positive witness is constructed from public `Split` data
  and the operand predicates, not reverse-engineered from the matcher.
- why: reversing the split accepts `[T, F]`; refusing an empty right side
  rejects `[F]`. Neither fault is hidden by the other positive fixture.

### stdlib/formal-languages/regex-cat-nullable-left-derivative

- spec: `spec/50-stdlib/61-formal-languages.md` §4.1–§4.3.
- promise class: durable invariant.
- given: `r = Cat Bool (Alt Bool (Eps Bool) (Sym Bool F))
  (Sym Bool T)`, word `[T]`, and its empty residual word. The left
  operand accepts `[]` but not `[T]`; the right accepts `[T]`.
- expect: `Split` with left `[]` and right `[T]` proves
  `regex_lang Bool r [T]`; `regex_matches Bool d r [T] = True`.
  `deriv Bool d T r` accepts `[]` through its
  `guard (nullable left) (deriv d T right)` alternative. Consequently
  both `deriv_complete` on the displayed split and `deriv_sound` on
  the derivative's independently computed positive residual have
  inhabited premises and the specified conclusions.
- measured: an empty-left factorisation, the nonempty word's matcher
  result, and the derivative's empty-residual membership.
- claimed: the concatenation derivative includes the right derivative
  exactly when the left expression is nullable.
- the gap: the left operand cannot consume the first `T` through its
  nonempty branch, so the other derivative alternative cannot rescue an
  omitted nullable-left branch on this word.
- why: deleting the guarded right-derivative alternative flips the
  positive result to `False`; a nonnullable left operand is independently
  exercised by the preceding case.

## Repetition and both matcher laws

### stdlib/formal-languages/regex-star-zero-and-repeat

- spec: `spec/50-stdlib/61-formal-languages.md` §4.1–§4.3.
- promise class: durable invariant.
- given: compare `Star Bool (Fail Bool)` on `[]` and `[T]`, and
  `Star Bool (Sym Bool T)` on `[]`, `[T, T]`, and `[T, F]`.
- expect: both stars accept `[]` through the public `MkPieces` witness
  with `ws = []`; both matchers return `True` and both nullabilities are
  `True`. `Star Fail` rejects `[T]` and its matcher returns `False`.
  `Star (Sym T)` accepts `[T, T]` via `ws = [[T], [T]]` and matcher
  `True`, but rejects `[T, F]` and returns `False`.
- measured: zero-piece and two-piece witnesses, and the Boolean results
  for positive and negative repetition inputs.
- claimed: star contains zero or more finite concatenated member words,
  never an arbitrary nonmember word.
- the gap: the two-piece witness uses `list_concat` and `list_all` over
  independently denoted `Sym T` pieces; it is not matcher-generated.
- why: `nullable Star = False` breaks the empty words, a derivative that
  forgets the remaining star breaks `[T, T]`, and a permissive repetition
  breaks `[T, F]`.

### stdlib/formal-languages/regex-star-leading-empty-piece

- spec: `spec/50-stdlib/61-formal-languages.md` §4.1–§4.3.
- promise class: durable invariant.
- given: `r = Star Bool (Alt Bool (Eps Bool) (Sym Bool T))`, word `[T]`,
  and public `Pieces` evidence with `ws = [[], [T]]`.
- expect: `list_concat Bool ws = [T]`; `list_all` holds because the
  first piece satisfies `Eps` and the second `Sym T`. Thus
  `regex_lang Bool r [T]` is inhabited and
  `regex_matches Bool d r [T] = True`. An instance of
  `regex_matches_complete` takes this explicit Ω witness to the
  Boolean `True`; independently evaluating the Boolean and applying
  `regex_matches_sound` returns an Ω proof on the same input.
- measured: the empty-prefix piece evidence and **both** matcher laws
  applied with inhabited antecedents on one positive word.
- claimed: star's matcher agrees in both directions with finite-piece
  denotation, including decompositions with an empty piece.
- the gap: a single fixture does not prove general soundness or
  completeness; the expected pieces and their membership proofs are
  constructed without using matcher output as their oracle.
- why: a `Pieces` constructor that incorrectly forbids empty pieces
  rejects this displayed witness. The Boolean matcher could still accept
  `[T]` via `ws = [[T]]`; its result alone does **not** distinguish the
  two decompositions. The negative `[T, F]` control separately checks
  that the remaining symbol is not accepted without a witness.

## Coverage and limits

The cases distinguish `Fail`/`Eps`, `Sym` and explicit `DecEq`, both
alternation arms, ordered and empty-sided concatenation, zero/multiple
star repetitions, and a valid leading empty piece. Both directions of
each derivative and matcher law have positive antecedents in named
fixtures. Their expected results come from the independent Ω denotation
and closed Bool inputs, not from a second matcher algorithm.

This seed does not claim to run the general proofs, decide `Regex`
similarity, produce an Nfa or Dfa, pin private derivative syntax, constrain
complexity, or test repository declarations/counts. The six §4 theorems
are part of the package contract and must be checked at catalog delivery;
these concrete cases alone are not replacements for them.
