# Standard infix binding conformance seed

Format: `../../README.md`. These cases pin the standard **meanings** of
`∧ ∨ ≤ ≥ ≠`, their standard fixities, and the use-site call-completion
contract, against `spec/30-surface/33-declarations.md §6.1`/`§6.2` and
`spec/30-surface/39-elaboration.md §6.9`. They assign no meaning to `∈`.

**Status and reachability.** The cases that observe a **completed** two-operand
occurrence are **RED-UNTIL-LANG-STANDARD-INFIX-CALL-COMPLETION**: completion is
A1's to build and no elaborator code ships with this contract. The cases that
observe today's artifacts — the extant bindings and their identities, the `Ord`
field set, and the comparator registry — are **live on this base and must
remain green**. A row is tagged only where the observation genuinely needs A1;
tagging a live row would hide a regression behind a gate.

**Promise class.** The five standard meanings, their carrier-directedness, and
the identity-keyed completion policy are durable invariants. The comparator
**inventory** is a recorded census, not a promise that it never grows: a future
registration widens it and the spec moves with it.

### surface/operators/standard-meanings-are-extant-bindings

- spec: `33 §6.1`
- given: resolve the standard meanings of `∧`, `∨` and `≤` to canonical
  identities, and compare each against the identity of the already-declared
  standard-package function of the same name.
- expect: the identities are **equal**. The standard-operator home reaches
  `bool_and`, `bool_or` and `ord_leq_at` by re-export, so by `33 §4.3` it
  republishes the existing `GlobalId` rather than minting another.
- why: this is the discriminating case against a **redefinition**. A second
  same-shaped declaration would satisfy every behavioural test — the two
  functions agree by construction — while producing two identities for one
  meaning, at which point `39 §6.9`'s policy holds between an occurrence and
  only one of them and the live catalog callers keep using the other.
  **MEASURED on this base:** `bool_and`, `bool_or` and `ord_leq_at` are
  declared in the standard package and are called from live catalog sources.
  **CLAIMED:** re-export, not redefinition. **THE GAP:** identity equality is
  the only observation that separates them; a value-level comparison cannot.

### surface/operators/geq-is-derived-not-an-ord-field

- spec: `33 §6.1`, `33 §5.2`
- given: enumerate the declared field names of `class Ord a`, and separately
  resolve the standard meaning of `≥`.
- expect: the field set is exactly `leq`, `refl`, `antisym`, `trans`, `total`
  and contains **no** `geq`; `≥` resolves to a top-level binding, not to a
  field projection.
- why: a reader who assumes `≥` is a class method looks for a member that was
  never declared, and an implementation that adds one widens a lawful class to
  serve an operator. **MEASURED on this base:** the five field names above, and
  no `geq` anywhere in the class. **CLAIMED:** `≥` is derived over `leq`.
  **THE GAP:** this pins the *shape*; the evaluation-order property is the
  separate row below.

### surface/operators/geq-reverses-values-not-operand-asts

- spec: `33 §6.1`
- given: an occurrence `x ≥ y` whose operands are two distinct observable
  expressions, ordered so that evaluating the right operand first is
  detectable.
- expect: **RED-UNTIL-LANG-STANDARD-INFIX-CALL-COMPLETION** — the left operand
  is evaluated first. The reversal happens on the two already-evaluated values
  inside the binding's body.
- why: the implementation this row excludes is a surface rewrite of `x ≥ y`
  into `y ≤ x`, which is the obvious way to build `≥` and silently swaps
  evaluation order. A row asserting only that `≥` "agrees with `≤` flipped"
  passes under that rewrite and pins nothing. **MEASURED:** not yet — the
  observation needs completion. **CLAIMED:** left-to-right operand evaluation
  is unchanged by `≥`. **THE GAP:** stated as an order observation rather than
  a result comparison, because the results agree under both implementations.

### surface/operators/completion-is-keyed-on-identity-not-glyph

- spec: `39 §6.9`
- given: three occurrences — (a) a standard binding reached under an import
  alias, (b) a user-declared local operator spelled `≤` with its own
  declaration, (c) the explicit application `ord_leq_at Nat d`.
- expect: **RED-UNTIL-LANG-STANDARD-INFIX-CALL-COMPLETION** — (a) completes;
  (b) does **not** complete and elaborates per its own declaration; (c) stays a
  valid partial application yielding a function, and is **not** reinterpreted
  as a four-argument call awaiting a prefix.
- why: the three arms fail under three different wrong implementations, which
  is why they are one case and not three loose assertions. Keying on glyph text
  breaks (a) and (b) in opposite directions — (a) fails to complete under an
  alias, (b) wrongly captures a user's operator. Treating every two-argument
  application of a standard binding as needing completion breaks (c).
  **MEASURED:** not yet. **CLAIMED:** the defining `GlobalId` plus checked
  telescope is the key. **THE GAP:** an implementation keyed on glyph text
  passes a single-arm test built from (a) alone.

### surface/operators/neq-carrier-registry-is-the-closed-inventory

- spec: `33 §6.2`
- given: enumerate the registered `==` comparator rows, and independently
  enumerate the mutators of the registry.
- expect: exactly five carriers — `Int`, `Float`, `Float32`, `Decimal` keyed on
  its registered representation, and `Char` — and exactly one mutator. `Nat`,
  `Bool` and `String` are absent.
- why: the inventory is load-bearing for `≠` and is asserted as **closed**, so
  the case must observe the closure and not merely the rows: the registry field
  is private and has a single mutator, which bounds the population by
  construction rather than by search. **MEASURED on this base:** five carrier
  registrations and one mutator. **CLAIMED:** the census is complete.
  **THE GAP:** counting raw matches of the census command yields more lines
  than carriers, because the mutator's own definition and body match it; the
  agreement asserted here is **by carrier name in both directions**, not by
  line count.

### surface/operators/neq-decimal-is-keyed-on-its-representation

- spec: `33 §6.2`, `18a §5.6.1`
- given: the registered key for the `Decimal` row, and the identity that
  normalisation delivers for a `Decimal`-typed operand.
- expect: both are the **representation's** identity. A lookup keyed on a
  distinct `Decimal` alias identity is never performed.
- why: `Decimal` is a transparent alias, so a rule naming only "Decimal"
  describes a key nothing looks up, and the error is invisible in prose — the
  sentence reads correctly and the row is unreachable. **MEASURED on this
  base:** the registration is keyed on the representation. **CLAIMED:** the
  spec's `Decimal` row names the key actually used. **THE GAP:** a
  value-level `≠` test at `Decimal` passes whichever identity the spec names,
  because the elaborator uses the real one regardless of the prose.

### surface/operators/neq-two-refusals-are-distinct

- spec: `33 §6.2`
- given: two refused occurrences — `x ≠ y` at `Nat`, and `x ≠ y` at a bound
  type variable.
- expect: **RED-UNTIL-LANG-STANDARD-INFIX-CALL-COMPLETION** — both are refused,
  and the refusals are **distinguishable**. The `Nat` occurrence is a registry
  miss on a head that *is* a constant or inductive former; the type-variable
  occurrence is refused **before any lookup**, because the head is neither.
- why: this row exists precisely because both arms reject, so a case asserting
  only rejection is green under an implementation that collapses them. The two
  states differ in what would change them: registering a comparator for `Nat`
  closes the first and closes nothing in the second, which is a statement about
  what `≠` **is** rather than a gap. **MEASURED on this base:** the selection
  path matches only a constant or an inductive former and returns before any
  lookup for every other head. **CLAIMED:** the two refusals are separately
  stated and separately closable. **THE GAP:** a single "unsupported carrier"
  outcome satisfies a rejection-only assertion and fails this one.

### surface/operators/standard-fixities-travel-with-identity

- spec: `33 §6.1`, `33 §6`
- given: parse `∧`, `∨`, `≤`, `≥` and `≠` occurrences reached under an import
  alias and under a re-exported path.
- expect: `∧` is `infixr 3`, `∨` is `infixr 2`, and `≤ ≥ ≠` are `infix 4` on
  every path. Fixity is declared of the binding's canonical identity, so it is
  not re-scopable at a use site.
- why: an implementation that attaches fixity to the surface spelling parses
  the same program differently under an alias. **MEASURED:** the fixity rule
  keyed on canonical identity is already normative in `33 §6`. **CLAIMED:** the
  five standard operators take the stated precedences on every path.
  **THE GAP:** a single-path parse test cannot see path-dependence; the aliased
  and re-exported arms are what make it discriminating.
