# Expression-position dependent Σ — conformance pair

## surface/elaboration/expression-sigma-and-converts (durable invariant)

- spec: `spec/30-surface/32-grammar.md §1–3`;
  `spec/30-surface/33-declarations.md §3.1–3.3`;
  `spec/10-kernel/13-pi-sigma.md §4`
- given: two compilation units:
  - `SigmaCatalog.ken.md` defines the lowercase conjunction and exports it as
    `And`:
    ```ken
    fn and (a : Omega) (b : Omega) : Omega = (x : a) × b
    export and as And
    ```
  - `SigmaConsumer.ken.md` explicitly imports the exported `And` before
    declaring both conversion directions:
    ```ken
    import SigmaCatalog (And)

    theorem expression_sigma_to_type_sigma
      (a : Omega) (b : Omega) (p : And a b)
      : (x : a) × b = p

    theorem type_sigma_to_expression_sigma
      (a : Omega) (b : Omega) (p : (x : a) × b)
      : And a b = p
    ```

- expect: the provider definition and export are valid, and the consumer's two
  identity theorems check by conversion. Each direction checks `p` at the other
  presentation of the conjunction.
- why: the file paths provide the implicit module identities. `and` is a valid
  lowercase value name; the `And` export is consumed only through an explicit
  selective import in a separate compilation unit. Neither
  the prelude nor an unimported global is assumed. Type-position Σ supplies the
  comparison form, so the theorems exercise conversion in both directions.

## surface/elaboration/expression-sigma-relevant-first-stays-type (durable invariant)

- spec: `spec/30-surface/32-grammar.md §3`; `spec/10-kernel/13-pi-sigma.md §4`
- given: `fn relevantAtType (p : Int -> Omega) : Type = (x : Int) × p x`
  followed by `fn relevantAtOmega (p : Int -> Omega) : Omega = (x : Int) × p x`.
- expect: the first declaration checks, with `x` bound in the codomain; the
  second rejects the Ω ascription with a sort mismatch (found `Type`, not Ω),
  rather than rejecting the expression for syntax or scope.
- why: `sort_sigma` keys on both component sorts. A relevant first component
  cannot become a proof-irrelevant proposition merely because its codomain
  is Ω-sorted.
