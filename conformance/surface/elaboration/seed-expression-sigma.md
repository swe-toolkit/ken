# Expression-position dependent Σ — conformance pair

## surface/elaboration/expression-sigma-and-converts (durable invariant)

- spec: `spec/30-surface/32-grammar.md §2–3`; `spec/10-kernel/13-pi-sigma.md §4`
- given: in the `SigmaCatalog` module, define a local `And` with
  expression-position Σ and check conversion both ways against type-position Σ:

  ```ken
  module SigmaCatalog {
    pub fn And (a : Omega) (b : Omega) : Omega = (x : a) × b

    theorem expression_sigma_to_type_sigma
      (a : Omega) (b : Omega) (p : And a b)
      : (x : a) × b = p

    theorem type_sigma_to_expression_sigma
      (a : Omega) (b : Omega) (p : (x : a) × b)
      : And a b = p
  }
  ```

- expect: the local definition and both identity theorems check by conversion.
  A value of `And a b` checks at `(x : a) × b`, and a value of the latter
  checks at `And a b`.
- why: the module declares its own `And`; no prelude member or unlanded
  catalog provider is assumed. Type-position Σ supplies the existing
  comparison form, so the two identity theorems exercise conversion in both
  directions.

## surface/elaboration/expression-sigma-relevant-first-stays-type (durable invariant)

- spec: `spec/30-surface/32-grammar.md §3`; `spec/10-kernel/13-pi-sigma.md §4`
- given: `fn RelevantAtType (p : Int -> Omega) : Type = (x : Int) × p x`
  followed by `fn RelevantAtOmega (p : Int -> Omega) : Omega = (x : Int) × p x`.
- expect: the first declaration checks, with `x` bound in the codomain; the
  second rejects the Ω ascription with a sort mismatch (found `Type`, not Ω),
  rather than rejecting the expression for syntax or scope.
- why: `sort_sigma` keys on both component sorts. A relevant first component
  cannot become a proof-irrelevant proposition merely because its codomain
  is Ω-sorted.
