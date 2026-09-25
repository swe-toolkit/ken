# Expression-position dependent Σ — conformance pair

## surface/elaboration/expression-sigma-and-converts (durable invariant)

- spec: `spec/30-surface/32-grammar.md §2–3`; `spec/10-kernel/13-pi-sigma.md §4`
- given: a catalog module defines the following term, and `And` is the
  prelude's checked Σ-backed conjunction:

  ```ken
  module SigmaCatalog {
    pub fn And (a : Omega) (b : Omega) : Omega = (x : a) × b
  }
  theorem prelude_to_catalog (a : Omega) (b : Omega) (p : And a b) : SigmaCatalog.And a b = p
  theorem catalog_to_prelude (a : Omega) (b : Omega) (p : SigmaCatalog.And a b) : And a b = p
  ```

- expect: the definition checks, lowers to the existing kernel `Sigma`, and
  both identity proofs check by conversion without an axiom or new trust entry.
- why: the new expression spelling constructs the same Ω-sorted dependent
  pair as the prelude `And`, rather than minting a new connective identity.

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
