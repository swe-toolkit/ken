# Reserved infix names conformance seed

Format: `../../README.md`. These cases pin the bounded name-admission
contract in `spec/30-surface/31-lexical.md §1c` and
`spec/30-surface/32-grammar.md §§1,3,6`. They assign no standard meaning to an
operator.

**Status and reachability.** Every case whose subject is one of the six reserved
notation identities is **RED-UNTIL-LANG-RESERVED-INFIX-NAMES**. On grounding
base `582457ea0b27e608cdb96995494a4b6e3903cd51`, the lexer already produces
six dedicated notation tokens, but the ordinary global-name, prefix, infix, and
fixity consumers accept only the generic symbolic-operator token. The tokens
therefore dead-end before the ordinary name path. The generic-operator and
`let … in` controls below are live on that base and must remain green.

**Promise class.** The exact six-name inventory and its five alias pairs are a
normative compatibility vector. Ordinary application, resolved-identity fixity,
and absence of a default binding are durable invariants. A future additive alias
requires a separate contract change; it is not a snapshot update.

**Independent oracle.** The expected inventory, application trees, and grouping
below are stated directly from the amended spec. No expected value is obtained
from the parser, resolver, fixity table, or elaborated output under test. Each
name is exercised in an isolated compilation unit, so one earlier dead-end
cannot hide a later omitted name.

## Common literal fixture

For a literal spelling `OP`, instantiate this source as one compilation unit:

```ken ignore
fn OP (x : Nat) (y : Nat) : Nat = x
infixr 5 OP
const prefix_bare : Nat = OP Zero (Suc Zero)
const prefix_grouped : Nat = (OP) Zero (Suc Zero)
const infix_once : Nat = Zero OP (Suc Zero)
const infix_chain : Nat = Zero OP (Suc Zero) OP (Suc (Suc Zero))
```

Write `G(OP)` for the single defining global identity introduced by the first
line. The independently stated expected observations for every instantiation
are:

1. the declaration accepts and binds `G(OP)` to the ordinary two-argument
   function with body `x`;
2. `prefix_bare`, `prefix_grouped`, and `infix_once` each lower to
   `App (App (Global G(OP)) Zero) (Suc Zero)`;
3. the `infixr 5` target resolves to `G(OP)`; and
4. `infix_chain` lowers to the right-associated tree
   `App (App (Global G(OP)) Zero)
   (App (App (Global G(OP)) (Suc Zero)) (Suc (Suc Zero)))`.

The tree observations, rather than evaluation of the left-projecting function,
make a wrong associativity visible. For a paired glyph/ASCII row, instantiate
both spellings separately. The two checked terms and defining identities are
identical after alias expansion; the source lexemes are not two declarations.

## The six independent name cases

### surface/operators/reserved-le-name-reaches-all-four-contexts

- spec: `31 §1b`/`§1c`; `32 §1`/§3/§6; `33 §6`
- given: two isolated common fixtures, first with `OP = ≤`, then with
  `OP = <=`
- expect: **RED-UNTIL-LANG-RESERVED-INFIX-NAMES** — both fixtures satisfy all
  four common observations and have the same defining identity and checked
  application trees.
- why: the `Le` notation identity is a declaration name, bare/grouped prefix
  atom, infix name, and fixity target; its two spellings are one name.

### surface/operators/reserved-ge-name-reaches-all-four-contexts

- spec: `31 §1b`/`§1c`; `32 §1`/§3/§6; `33 §6`
- given: two isolated common fixtures, first with `OP = ≥`, then with
  `OP = >=`
- expect: **RED-UNTIL-LANG-RESERVED-INFIX-NAMES** — both fixtures satisfy all
  four common observations and have the same defining identity and checked
  application trees.
- why: the `Ge` notation identity reaches every ordinary operator-name context;
  neither spelling creates a second binding.

### surface/operators/reserved-ne-name-reaches-all-four-contexts

- spec: `31 §1b`/`§1c`; `32 §1`/§3/§6; `33 §6`
- given: two isolated common fixtures, first with `OP = ≠`, then with
  `OP = /=`
- expect: **RED-UNTIL-LANG-RESERVED-INFIX-NAMES** — both fixtures satisfy all
  four common observations and have the same defining identity and checked
  application trees.
- why: `/=` keeps its existing notation-token identity and gains ordinary name
  uses; this case does not add `!=` or a standard disequality meaning.

### surface/operators/reserved-and-name-reaches-all-four-contexts

- spec: `31 §1b`/`§1c`; `32 §1`/§3/§6; `33 §6`
- given: two isolated common fixtures, first with `OP = ∧`, then with
  `OP = /\`
- expect: **RED-UNTIL-LANG-RESERVED-INFIX-NAMES** — both fixtures satisfy all
  four common observations and have the same defining identity and checked
  application trees.
- why: the logical-looking glyph remains an ordinary client-defined function
  name here; the case supplies no Boolean or proposition semantics.

### surface/operators/reserved-or-name-reaches-all-four-contexts

- spec: `31 §1b`/`§1c`; `32 §1`/§3/§6; `33 §6`
- given: two isolated common fixtures, first with `OP = ∨`, then with
  `OP = \/`
- expect: **RED-UNTIL-LANG-RESERVED-INFIX-NAMES** — both fixtures satisfy all
  four common observations and have the same defining identity and checked
  application trees.
- why: the `Or` notation identity is admitted as a name without acquiring a
  standard binding or short-circuit behavior.

### surface/operators/reserved-member-name-reaches-all-four-contexts

- spec: `31 §1a`/§1b/§1c; `32 §1`/§3/§6; `33 §6`
- given: one common fixture with `OP = ∈`
- expect: **RED-UNTIL-LANG-RESERVED-INFIX-NAMES** — the glyph-only fixture
  satisfies all four common observations.
- why: a client-defined ordinary function named `∈` is admitted. This neither
  accepts keyword `in` as an alias nor supplies a standard membership binding,
  class, or carrier.

## Global-name routing

### surface/operators/reserved-global-name-routing-preserves-defining-identity

- spec: `32 §1`/§3; `33 §3.2`/§3.3/§6
- given: for each of the six canonical glyphs, an isolated module exports the
  corresponding common-fixture function as `OP`. Separate consumers name it as
  `Provider.OP`, select it with `import Provider (OP)`, rename that selection to
  the existing generic operator `<+>`, and re-export it from a facade under
  `<+>`. For the five paired names, repeat the source selections with the ASCII
  spelling.
- expect: **RED-UNTIL-LANG-RESERVED-INFIX-NAMES** — every qualified, selected,
  renamed, and re-exported reference resolves to the provider's exact `G(OP)`;
  no route mints a replacement identity. The provider's `infixr 5` declaration
  governs each route, so a three-operand consumer has the common fixture's
  right-associated tree.
- why: `operator_name` is an arm of the existing global-name grammar, not only
  an infix-loop exception. One function declaration accepting while selection
  or a qualified suffix still dead-ends would fail this identity observation.

## Closed roster and omission detector

### surface/operators/reserved-name-roster-is-exact-and-each-row-reaches

- spec: `31 §1c`; `32 §1`
- given: run the six canonical-glyph fixtures above as six isolated compilation
  units and label their results `Le`, `Ge`, `Ne`, `And`, `Or`, and `Member`
- expect: **RED-UNTIL-LANG-RESERVED-INFIX-NAMES** — the exact result map is
  `{Le: four-context-accept, Ge: four-context-accept,
  Ne: four-context-accept, And: four-context-accept,
  Or: four-context-accept, Member: four-context-accept}`. No row may be inferred
  from an aggregate accepted count.
- why: the external six-key map is the omission oracle. Leaving any one
  dedicated token out of the shared operator-name view changes its named cell
  to rejection even when the other five accept.

### surface/operators/paired-aliases-cannot-bind-twice

- spec: `31 §1a`/§1c; `32 §1`; `33 §3`/§6
- given: for each of the five pairs, compare its accepted one-declaration
  fixture with a unit that adds a second otherwise-identical declaration under
  the twin spelling, such as `fn ≤ ...` followed by `fn <= ...`
- expect: **RED-UNTIL-LANG-RESERVED-INFIX-NAMES** — the one-declaration control
  accepts; every two-spelling declaration unit rejects through the ordinary
  duplicate-global rule and identifies the single canonical operator identity.
  A unit with one function and fixity lines under both spellings instead rejects
  through the ordinary duplicate-fixity rule for that same identity.
- why: accepting both source spellings is not permission to create two
  confusable bindings or two fixities.

## No implicit binding and no glyph precedence

### surface/operators/unbound-reserved-names-reach-resolution

- spec: `31 §1c`; `32 §3`; `33 §6`
- given: six isolated units, each containing only
  `const probe : Nat = Zero OP (Suc Zero)`, with `OP` respectively
  `≤`, `≥`, `≠`, `∧`, `∨`, and `∈`
- expect: **RED-UNTIL-LANG-RESERVED-INFIX-NAMES** — each source parses as an
  ordinary infix application and then rejects as an unknown global bearing its
  corresponding canonical operator identity. A lexical or parser dead-end does
  not conform, and no row evaluates through an implicit prelude binding.
- why: name admission and standard meaning are separate. This pairwise result
  simultaneously detects the old dead end and a smuggled-in default binding.

### surface/operators/undeclared-reserved-fixity-is-default-infixl-nine

- spec: `32 §6`; `33 §6`
- given: for each of the six canonical glyphs, remove the `infixr 5` line from
  its otherwise unchanged common fixture and add
  `const mixed : Nat = Zero + Suc Zero OP Suc (Suc Zero)`
- expect: **RED-UNTIL-LANG-RESERVED-INFIX-NAMES** — `infix_chain` lowers to
  `App (App (Global G(OP))
  (App (App (Global G(OP)) Zero) (Suc Zero))) (Suc (Suc Zero))`, and `mixed`
  groups as `Zero + (Suc Zero OP Suc (Suc Zero))`. Every name is left
  associative at level 9 on a fixity-table miss.
- why: the expected trees are fixed independently. A glyph-specific comparison
  or logic precedence, or a precedence derived from the source spelling, changes
  at least one tree.

## Refused aliases with positive controls

### surface/operators/bang-equal-is-not-ne-alias

- spec: `31 §1c`; `32 §1`
- positive: the `reserved-ne-name-reaches-all-four-contexts` fixtures for `≠`
  and `/=`
- negative: an otherwise-identical common fixture with `OP = !=`
- expect-positive: **RED-UNTIL-LANG-RESERVED-INFIX-NAMES** — both specified `Ne`
  spellings accept through the ordinary name path.
- expect-negative: `!=` rejects lexically at `!`; it does not produce `Ne`, a
  generic operator name, or two punctuation tokens that later recombine.
- why: the accepted twin proves the fixture and operator-name path are live;
  the one-token-family change isolates the unadmitted alias.

### surface/operators/in-remains-let-separator-not-member-alias

- spec: `31 §1c`/§4; `32 §1`/§3`
- positive-name: the client-defined `∈` common fixture accepts after
  `LANG-RESERVED-INFIX-NAMES`
- positive-keyword: `const keep : Nat = let x = Zero in x` accepts and returns
  `Zero`
- negative: replace the `∈` declaration name and every use in its common fixture
  with the word `in`
- expect-negative: the replacement rejects at the first global-name position
  because `in` is the distinct local-let keyword. It never reaches a `Member`
  global identity.
- why: the two positive arms prove both intended tokens remain usable. Treating
  `in` as membership breaks the local-let control; rejecting all membership
  spellings breaks the client-defined glyph control.

## Existing-path controls and exclusions

### surface/operators/generic-symbolic-names-remain-live (control)

- spec: `31 §2`; `32 §1`/§3/§6; `33 §6`
- given: four isolated common fixtures with `OP` respectively `<`, `>`, `/`,
  and `%`
- expect: **LIVE — accepted** with the same four common observations for each
  generic symbolic name.
- why: a replacement that recognizes only the six dedicated notation tokens
  regresses the existing generic-operator path. The control also proves the
  common fixture does not require a standard operator meaning.

### surface/operators/fixed-and-protected-token-roles-remain-distinct (control)

- spec: `31 §1b`/§1c/§2/§4; `32 §2`/§3/§6`
- given: independent existing-surface programs containing (a) `a + b * c` and
  `a == b`; (b) `λx. x` and `A → B`; (c) `‖Top‖`; (d) `A @ level`; and
  (e) binding `=`, ascription `:`, attached selector `::`, projection `.`, and
  match arm `↦` in their existing grammatical roles
- expect: every program keeps its pre-amendment token kinds, parse tree, and
  elaborated term. None of the listed fixed operators, delimiters, annotations,
  or punctuation becomes an `operator_name`. In particular, `a + b * c` stays
  `a + (b * c)`, and `==` stays the existing built-in equality operation.
  Unicode lattice `⊔`/`⊓` retain their distinct notation roles; their future
  ASCII spellings are not inferred from the `∨`/`∧` aliases fixed here.
- why: the six-name admission is an additive arm of the ordinary symbolic-name
  view, not a generic reclassification of symbolic source text.

The amendment also does not admit any reserved notation token as a local
binder, type or constructor name, module path or alias, record field, or
attached-proof name. Tests for those grammars should continue to assert their
existing identifier-only behavior; a rejection there is not evidence against
the global-name acceptance cases above.

## Seed placement

This seed lives under `surface/operators/` because every principal case crosses
four surface layers: lexical alias identity, global declaration/name grammar,
prefix/infix expression parsing, and resolved fixity. Placing it under only
`literals/`, `declarations/`, or `formatting/` would make the other reaching
observations look ancillary and would obscure the per-name omission oracle.
The existing owner seeds remain authoritative for arithmetic, modules,
formatting, truncation, annotations, and local binding.

## Clean-room provenance

The cases and expected trees were derived from Ken's amended chapters 31/32,
the landed ordinary operator/fixity contract in chapter 33, the settled work
frame, and first principles. No external reference, `local/refs` implementation,
permissive reference, copyleft reference, or excluded prototype was consulted.
