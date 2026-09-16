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
`let … in` controls below are live on that base and must remain green. The
application-atom boundary cases are not an A0 aggregate: they state
`32 §3`'s contract pin, which is independent of reserved-name admission. The
rows this base does not yet satisfy carry
**RED-UNTIL-LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE**: the `if` row, the
projection row, and the bare-operator negatives. The lambda, `let`, `match`,
and arrow rows conform on this base and carry no gate. Their temporal rows
instead carry **RED-UNTIL-TEMPORAL-EXPRESSION-SURFACE** (`OQ-syntax`; no build
WP is framed in this candidate): landing the reserved-name build cannot clear
a grouped expression form that is not yet available. The reserved-head temporal
row also retains its reserved-name dependency until A0 lands. Executed parser
observations on the authorized re-anchor base
`cb646c784b2dc480bdae703473055481ca6b5e44` show that generic `(<+>)`,
`<+> Zero`, and bare `<+>` all accept, while
`<+> if true then Zero else Zero` still accepts as an operator-headed
application. Some identifier-head rows already have their
target shape, but ungrouped `if` is still accepted as an argument, a bare
projection still attaches inside the argument, and expression-level `temporal`
is not yet accepted even when grouped.

**Disposition (2026-09-16).** The application-atom boundary rows are
**relocated**, not withdrawn and not deferred: they move off
[[LANG-RESERVED-INFIX-NAMES]] onto
[[LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE]], which also owns the catalog
migration. Grounding: the rows state `spec/30-surface/32-grammar.md §3`'s
contract pin (`:362-363` for the operator-name restriction, `:371-376` for the
five leading forms and for `(keep Nat) -> Nat` / `(keep box).value`), so they
are not an A0 over-reach; A0 admits reserved names and cannot satisfy a claim
about sources containing none, which is why the gate was mis-attributed rather
than merely premature. The priority-queue catalog control's failure measures
that catalog source was written against a non-conforming parser, and is not
evidence against the rows. Architect ruling `evt_5sf71fnjxpzmb`; the earlier
WITHDRAW ruling `evt_64avxs9ashqqk` is void.

**Promise class.** The exact six-name inventory and its five alias pairs are a
normative compatibility vector. Ordinary application, resolved-identity fixity,
and absence of a default binding are durable invariants. The grouped-argument
boundary, across both application productions and both operator-name token
classes, and its intentional ungrouped parse trees are durable grammar
invariants. A future additive alias requires a separate contract change; it is
not a snapshot update.

**Independent oracle.** The expected inventory, application trees, grouping,
and literal results below are stated directly from the amended spec. No expected
value is obtained from the parser, resolver, fixity table, or elaborated output
under test. The isolated units prevent one earlier dead-end from hiding a later
omitted name. A separate simultaneous unit gives the six definitions distinct
literal bodies, so an accidental cross-row identity collapse cannot hide behind
six internally consistent isolated runs. The application-atom tables state each
grouped and ungrouped tree or rejection independently rather than deriving one
from the parser's result for the other.

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

## Operator values and the application-atom boundary

The tree observations in this section are fixed before name resolution or
elaboration. The operator-value case separately observes the grouped name's
resolved identity. Write `A(f, x)` for one application node, `N(<+>)` for the
generic operator-name node, and `N(Le)` for the canonical reserved-name node.
Use `Lam`, `Let`, `If`, `Match`, `Temporal`, `Arrow`, and `Proj` for the
corresponding chapter-32 expression productions. Parentheses group but do not
add a tree node.

### surface/operators/bare-operator-value-requires-grouping

- spec: `32 §1`/§3
- given: instantiate two isolated common fixtures, first with generic
  `OP = <+>` and then with reserved `OP = ≤`. Add these two lines to each unit:

  ```ken ignore
  const grouped_op : Nat -> Nat -> Nat = (OP)
  const applied_op : Nat -> Nat = OP Zero
  ```

  For the negative twin, replace only `(OP)` in `grouped_op` with bare `OP`.
- expect-positive: **RED-UNTIL-LANG-RESERVED-INFIX-NAMES** — for both literal
  instantiations, `(OP)` parses as the grouped `operator_name` atom and resolves
  to that fixture's exact `G(OP)`, while `OP Zero` parses as
  `A(G(OP), Zero)`. The generic grouped and applied arms are live before A0 and
  must remain so; the reserved arms turn green when A0 admits `Le` to the same
  grammar.
- expect-negative: **RED-UNTIL-LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE** —
  bare `<+>` and bare `≤` each reject syntactically before resolution: neither
  is an `application_atom` or a complete `operator_prefix`, because each has
  zero following atoms. Neither may produce a global-value tree for its
  `G(OP)`. This is `32 §3`'s restriction, which `:362-363` states applies
  equally to generic and reserved operator names — so the row's target is the
  spec's, not A0's. The generic refusal is a required behavior change, not a
  pre-existing token dead end; the reserved refusal must occur after `Le`
  reaches `operator_name`, so the reserved arm additionally presupposes A0.
  **MEASURED on this base:** bare `<+>` parses as an ordinary global-value
  reference rather than rejecting, so the generic arm is red against the
  successor; bare `≤` rejects at the `Le` token, which is the pre-A0 dead end
  and not yet this restriction being enforced.
- why: rejecting every ungrouped operator would make both negatives pass for the
  wrong reason, while special-casing only one token class would make one triple
  pass. The adjacent grouped and nonzero-application positives keep each route
  live. **MEASURED:** in both token classes the grouped zero-application value
  resolves, the ungrouped zero-application value rejects, and an ungrouped
  nonzero application accepts. **CLAIMED:** grouping, not token identity,
  distinguishes an operator value from the head-only prefix form. **THE GAP:**
  these parse-and-resolution triples do not establish infix reassociation; the
  common fixtures' separate structural trees do.

### surface/operators/identifier-headed-non-atom-arguments-require-grouping

- spec: `32 §3`, `expr application_atom`
- given: feed each table cell independently to the full-expression parser. The
  names are deliberately unresolved because the observation ends at parsing;
  no type or evaluation result is used as an oracle.

  | class | grouped source | exact grouped tree | ungrouped source | exact ungrouped outcome |
  |---|---|---|---|---|
  | lambda | `keep (λx. x)` | `A(keep, Lam(x, x))` | `keep λx. x` | reject at the leading `λ`; no complete tree |
  | let | `keep (let x = Zero in x)` | `A(keep, Let(x, Zero, x))` | `keep let x = Zero in x` | reject at the leading `let`; no complete tree |
  | if | `keep (if true then Zero else Zero)` | `A(keep, If(true, Zero, Zero))` | `keep if true then Zero else Zero` | reject at the leading `if`; no complete tree |
  | match | `keep (match flag { true ↦ Zero; false ↦ Zero })` | `A(keep, Match(flag, {true ↦ Zero; false ↦ Zero}))` | `keep match flag { true ↦ Zero; false ↦ Zero }` | reject at the leading `match`; no complete tree |
  | temporal | `keep (temporal { Top })` | **RED-UNTIL-TEMPORAL-EXPRESSION-SURFACE** — `A(keep, Temporal(Top))` | `keep temporal { Top }` | reject at the leading `temporal`; no complete tree |
  | arrow | `keep (Nat -> Nat)` | `A(keep, Arrow(Nat, Nat))` | `keep Nat -> Nat` | accept as `Arrow(A(keep, Nat), Nat)`, not as the grouped tree |
  | projection | `keep (box.value)` | `A(keep, Proj(box, value))` | `keep box.value` | accept as `Proj(A(keep, box), value)`, not as the grouped tree |

- expect-grouped: every grouped non-temporal source has exactly the stated
  application tree. Live on this base for all six non-temporal classes.
- expect-leading-conforming: the ungrouped `lambda`, `let`, and `match` rows
  reject with a primary span covering exactly the named leading token. Live on
  this base; these three conform to `32 §3` today and carry no gate.
- expect-leading-if: **RED-UNTIL-LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE** —
  `32 §3` names `if` among the five leading forms that must reject at their
  leading token when ungrouped after an application head. **MEASURED on this
  base:** `keep if true then Zero else Zero` is accepted and yields exactly the
  grouped tree, so the parser diverges from the pin here. The row states the
  spec and is red until the successor closes the divergence.
- expect-arrow: the ungrouped arrow row accepts with exactly its stated outer
  tree and does not silently acquire the grouped interpretation. Live on this
  base; conforms to `32 §3`'s `(keep Nat) -> Nat` and carries no gate.
- expect-projection: **RED-UNTIL-LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE** —
  `32 §3` fixes the ungrouped projection as `(keep box).value`, the application
  nested in the projection's left `expr`. **MEASURED on this base:** `keep
  box.value` yields the grouped reading instead, so the parser diverges from
  the pin here. The row states the spec and is red until the successor closes
  the divergence.
- expect-temporal: **RED-UNTIL-TEMPORAL-EXPRESSION-SURFACE** — once the
  separately deferred grouped expression form exists, it has the tabled tree
  and its ungrouped twin rejects at `temporal`. This row is not an A0 exit
  condition and does not turn green merely because reserved infix names land.
- why: each row varies only grouping around one named non-atom class. Restoring
  unrestricted `expr expr` makes the three currently enforced rejection rows,
  and eventually the temporal row, accept as applications; treating arrow or
  projection as an argument atom changes its named outer tree. **MEASURED on
  this base:** seven independent grouped/ungrouped pairs expose three enforced
  reject boundaries (`lambda`, `let`, `match`), one spec-required reject
  boundary this base does not enforce (`if`), one deferred reject boundary
  (`temporal`), one conforming precedence boundary (`arrow`), and one
  spec-required precedence boundary this base does not enforce (`projection`).
  **CLAIMED:** the `expr application_atom` production admits only atoms in bare
  identifier-headed argument position. **THE GAP:** these are parser
  observations only; the ordinary-application elaboration path is covered
  independently by the common operator fixture and existing surface
  elaboration cases.

### surface/operators/operator-prefix-arguments-are-atoms

- spec: `32 §1`/§3, `operator_prefix ::= operator_name application_atom+`
- given: parse every table row independently as shown. The generic `<+>` head
  is the live `operator` control; the reserved `≤` head is the independent `Le`
  discriminator. As in the identifier-head table, names remain unresolved and
  the observation ends at parsing.

  | head/class | grouped source | exact grouped tree | ungrouped source | exact ungrouped outcome |
  |---|---|---|---|---|
  | generic/lambda | `<+> (λx. x)` | `A(N(<+>), Lam(x, x))` | `<+> λx. x` | reject at `λ` after consuming `<+>`; no complete tree |
  | reserved/lambda | `≤ (λx. x)` | `A(N(Le), Lam(x, x))` | `≤ λx. x` | reject at `λ` after consuming `Le`; no complete tree |
  | generic/let | `<+> (let x = Zero in x)` | `A(N(<+>), Let(x, Zero, x))` | `<+> let x = Zero in x` | reject at `let` after consuming `<+>`; no complete tree |
  | reserved/let | `≤ (let x = Zero in x)` | `A(N(Le), Let(x, Zero, x))` | `≤ let x = Zero in x` | reject at `let` after consuming `Le`; no complete tree |
  | generic/if | `<+> (if true then Zero else Zero)` | `A(N(<+>), If(true, Zero, Zero))` | `<+> if true then Zero else Zero` | reject at `if` after consuming `<+>`; no complete tree |
  | reserved/if | `≤ (if true then Zero else Zero)` | `A(N(Le), If(true, Zero, Zero))` | `≤ if true then Zero else Zero` | reject at `if` after consuming `Le`; no complete tree |
  | generic/match | `<+> (match flag { true ↦ Zero; false ↦ Zero })` | `A(N(<+>), Match(flag, {true ↦ Zero; false ↦ Zero}))` | `<+> match flag { true ↦ Zero; false ↦ Zero }` | reject at `match` after consuming `<+>`; no complete tree |
  | reserved/match | `≤ (match flag { true ↦ Zero; false ↦ Zero })` | `A(N(Le), Match(flag, {true ↦ Zero; false ↦ Zero}))` | `≤ match flag { true ↦ Zero; false ↦ Zero }` | reject at `match` after consuming `Le`; no complete tree |
  | generic/temporal | `<+> (temporal { Top })` | **RED-UNTIL-TEMPORAL-EXPRESSION-SURFACE** — `A(N(<+>), Temporal(Top))` | `<+> temporal { Top }` | reject at `temporal` after consuming `<+>`; no complete tree |
  | reserved/temporal | `≤ (temporal { Top })` | **RED-UNTIL-LANG-RESERVED-INFIX-NAMES** and **RED-UNTIL-TEMPORAL-EXPRESSION-SURFACE** — `A(N(Le), Temporal(Top))` | `≤ temporal { Top }` | reject at `temporal` after consuming `Le`; no complete tree |
  | generic/arrow | `<+> (Nat -> Nat)` | `A(N(<+>), Arrow(Nat, Nat))` | `<+> Nat -> Nat` | accept as `Arrow(A(N(<+>), Nat), Nat)`, not as the grouped tree |
  | reserved/arrow | `≤ (Nat -> Nat)` | `A(N(Le), Arrow(Nat, Nat))` | `≤ Nat -> Nat` | accept as `Arrow(A(N(Le), Nat), Nat)`, not as the grouped tree |
  | generic/projection | `<+> (box.value)` | `A(N(<+>), Proj(box, value))` | `<+> box.value` | accept as `Proj(A(N(<+>), box), value)`, not as the grouped tree |
  | reserved/projection | `≤ (box.value)` | `A(N(Le), Proj(box, value))` | `≤ box.value` | accept as `Proj(A(N(Le), box), value)`, not as the grouped tree |

- expect-nontemporal: **RED-UNTIL-LANG-RESERVED-INFIX-NAMES** — all twelve
  grouped sources have exactly the stated `operator_prefix` trees. The eight
  ungrouped leading-form sources reject with their primary span on the argument
  token named in the table, after the operator head has been consumed. The four
  arrow/projection sources accept with exactly the stated outer trees.
- expect-temporal: the generic grouped source is
  **RED-UNTIL-TEMPORAL-EXPRESSION-SURFACE**. The reserved grouped source carries
  that marker and **RED-UNTIL-LANG-RESERVED-INFIX-NAMES**; A0 can clear only its
  head, not the unavailable grouped argument. Both ungrouped sources reject at
  the `temporal` argument token once their head is admitted.
- why: the live generic matrix rejects an implementation that leaves generic
  operator prefixes on an arbitrary-`expr` argument path. The parallel reserved
  matrix rejects a generic-only repair or an old `Le` token dead end: a reserved
  negative that stops at `≤` rather than the tabled argument token fails.
  Grouped positives prevent a reject-all repair, while the arrow/projection
  trees distinguish atom consumption from precedence reparsing. **MEASURED:**
  fourteen paired rows independently observe both operator-name token classes
  at all seven argument classes. **CLAIMED:** the `application_atom+` tail, not
  an arbitrary expression parser, is the only argument route from
  `operator_prefix`. **THE GAP:** these are parse observations; the common
  fixtures and the two operator-value triples separately pin resolution and
  nonzero application for both heads.

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

### surface/operators/simultaneous-reserved-names-stay-distinct

- spec: `31 §1c`; `32 §1`/§3; `33 §1`/§3.3
- given: one compilation unit defines all six canonical names at once with
  distinct integer-literal bodies, then calls every admitted source spelling:

  ```ken ignore
  fn ≤ (x : Int) (y : Int) : Int = 11
  fn ≥ (x : Int) (y : Int) : Int = 22
  fn ≠ (x : Int) (y : Int) : Int = 33
  fn ∧ (x : Int) (y : Int) : Int = 44
  fn ∨ (x : Int) (y : Int) : Int = 55
  fn ∈ (x : Int) (y : Int) : Int = 66

  const le_glyph : Int = ≤ 0 0
  const le_ascii : Int = <= 0 0
  const ge_glyph : Int = ≥ 0 0
  const ge_ascii : Int = >= 0 0
  const ne_glyph : Int = ≠ 0 0
  const ne_ascii : Int = /= 0 0
  const and_glyph : Int = ∧ 0 0
  const and_ascii : Int = /\ 0 0
  const or_glyph : Int = ∨ 0 0
  const or_ascii : Int = \/ 0 0
  const member_glyph : Int = ∈ 0 0
  ```

- expect: **RED-UNTIL-LANG-RESERVED-INFIX-NAMES** — the unit accepts and its
  declaration environment contains six pairwise-distinct identities
  `G(Le)`, `G(Ge)`, `G(Ne)`, `G(And)`, `G(Or)`, and `G(Member)`. The following
  table is the independently fixed observation map:

  | constants | resolved callee | normal form |
  |---|---|---|
  | `le_glyph`, `le_ascii` | `G(Le)` | `11` |
  | `ge_glyph`, `ge_ascii` | `G(Ge)` | `22` |
  | `ne_glyph`, `ne_ascii` | `G(Ne)` | `33` |
  | `and_glyph`, `and_ascii` | `G(And)` | `44` |
  | `or_glyph`, `or_ascii` | `G(Or)` | `55` |
  | `member_glyph` | `G(Member)` | `66` |

- why: isolated fixtures establish acceptance but can remain internally
  consistent if every dedicated token is mapped to one shared key. Here such a
  collapse either creates a duplicate declaration or changes at least one
  independently named callee/result row. The paired rows simultaneously prove
  equality within each alias pair and inequality across the six identities.

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

### surface/operators/reserved-names-do-not-widen-excluded-grammars

- spec: `32 §1`
- given: compile every matrix row independently with the well-typed surroundings
  its fragment needs. The six local-binder rows use the full canonical-token
  roster. Each paired ASCII spelling lexes to the same token class before this
  grammar boundary; the simultaneous fixture above exercises those spellings.
  The remaining rows use one roster member per excluded grammar. Each negative
  changes only the marked identifier from its adjacent positive:

  | excluded grammar | negative fragment | ordinary-name positive | exact diagnostic at marked token |
  |---|---|---|---|
  | local binder — `Le` | `const p : Nat = let ≤ = Zero in Zero` | `const p : Nat = let x = Zero in Zero` | `expected identifier, found Le` |
  | local binder — `Ge` | `const p : Nat = let ≥ = Zero in Zero` | `const p : Nat = let x = Zero in Zero` | `expected identifier, found Ge` |
  | local binder — `Ne` | `const p : Nat = let ≠ = Zero in Zero` | `const p : Nat = let x = Zero in Zero` | `expected identifier, found Ne` |
  | local binder — `And` | `const p : Nat = let ∧ = Zero in Zero` | `const p : Nat = let x = Zero in Zero` | `expected identifier, found And` |
  | local binder — `Or` | `const p : Nat = let ∨ = Zero in Zero` | `const p : Nat = let x = Zero in Zero` | `expected identifier, found Or` |
  | local binder — `Member` | `const p : Nat = let ∈ = Zero in Zero` | `const p : Nat = let x = Zero in Zero` | `expected identifier, found Member` |
  | type name | `def ≤ = Nat` | `def Alias = Nat` | `expected uppercase constructor name, found Le` |
  | constructor name | `data Marker = Only | ≥` | `data Marker = Only | Another` | `expected uppercase constructor name, found Ge` |
  | module path | `module ≠ {}` | `module Provider {}` | `expected uppercase constructor name, found Ne` |
  | module alias | `import Provider as ∧` | `import Provider as Alias` | `expected identifier, found And` |
  | record field | `record Box { ∨ : Nat }` | `record Box { value : Nat }` | `expected identifier, found Or` |
  | attached-proof name | `proof ∈ for id (x : Nat) : Equal Nat (id x) x = Refl` | `proof id_self for id (x : Nat) : Equal Nat (id x) x = Refl` | `expected identifier, found Member` |

- expect-positive: every ordinary `ident`/`ConId` control accepts in the same
  parser-and-elaboration harness. The import rows have an existing `Provider`;
  the proof rows have `fn id (x : Nat) : Nat = x`, so neither positive depends
  on an unresolved surrounding name.
- expect-negative: **RED-UNTIL-LANG-RESERVED-INFIX-NAMES** — each negative
  lexes the marked spelling to its dedicated token class, enters the construct
  containing the excluded identifier position, and produces the exact tabled
  diagnostic. Its primary span is exactly the marked reserved token. In
  particular, a local row must enter `let_binding` before refusing its name,
  and the constructor row must consume `Only |` before refusing the second
  constructor. A rejection at the surrounding declaration head or at an
  unrelated later token does not conform.
- why: the admitted shared view is `operator_name`, used only by global value
  and selection names. Reusing it as a general identifier parser would make at
  least one matrix negative accept. The same-grammar positive rows distinguish
  that over-widening refusal from a broken declaration parser, while the live
  simultaneous global-name fixture distinguishes it from the old reserved-token
  dead end.

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
