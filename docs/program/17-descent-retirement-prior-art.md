# Descent retirement: prior-art advisory

## Conclusion

Mature implementations do admit a structurally recursive function whose
result is scrutinized by a match. The surveyed systems do not establish that
this source shape is unreachable, and they do not rely on normalization to
erase every instance of it.

They instead support two established implementation patterns:

1. Lean and Agda impose a value-only case-scrutinee invariant in a compiler
   IR. A preceding phase sequences or rewrites the computation that produces
   the value. The invariant is expressed in the IR datatype and preserved by
   the transformations that construct or rewrite it.
2. GHC permits an expression as a case scrutinee in Core and STG. It lowers the
   scrutinee through the same expression translation as any other computation
   and then constructs the case. Recursion remains ordinary application and
   binding; there is no second whole-function descent lane for this shape.

The prior art therefore supports retiring Ken's exceptional descent lane, but
not on the ground that the source shape is impossible. It supports retirement
only after the ordinary lowering boundary has a stated and enforced invariant
that makes the exceptional classifier input unavailable.

Confidence is **high** in the observations below and **medium-high** in their
applicability to Ken. The remaining uncertainty is local: the fixed input says
Ken's checked path currently supplies a wrapper, while a generic non-plan path
can still emit the bare shape. Prior art cannot prove closure of Ken's own IR
producers.

## Scope and method

This is an approach-and-behaviour survey, not a transcription of another
compiler. It compares three production implementations at pinned revisions.
Admission claims rest on compiler regression inputs, and representation claims
rest on the implementations of the relevant IR constructors and translations,
not on user documentation. No reference source text is reproduced here.

Ken's measured facts are taken as the fixed input from
`RT-MATCH-DIFFERENCE-REACHABILITY`; this advisory does not re-measure them.

## Findings by system

| System | Source shape admitted? | What reaches the relevant compiler IR? | Where the property is enforced |
| --- | --- | --- | --- |
| Lean 4 | Yes. Its elaborator regression suite contains a structural `Nat` function that immediately matches on a recursive call and makes further recursive calls in the alternatives. | Structural elaboration replaces recursive applications with lookups in the course-of-values `below` result dictionary. Later, LCNF represents a case discriminant as a free-variable identifier. The LCNF translation visits an arbitrary discriminant expression first and then constructs the case from the resulting identifier. | The structural elaborator performs the recursive-call replacement. The LCNF datatype makes an expression-valued discriminant unrepresentable, and the LCNF constructor path sequences the expression before creating the case. |
| Agda | Yes. Its `Succeed` suite contains a structurally recursive function whose successor clause uses the recursive result as a `with` scrutinee. | Treeless IR represents a case scrutinee by a variable index. | The Treeless datatype states the value-only shape. Its substitution implementation preserves it: substitution by a variable changes the index, while substitution by a non-variable introduces a `let` and cases on that binding. |
| GHC | Yes in the relevant general sense: Core has recursive bindings, applications, and a case over an arbitrary expression. The representation places no special restriction on a recursive application in scrutinee position. | Core and STG both retain an expression-valued case scrutinee. Core-to-STG recursively translates that expression and then builds an STG case around the translated result. | Uniformity, rather than a value-only grammar, is the invariant. The same expression translator handles the scrutinee; applications use the ordinary application form and recursive functions use ordinary recursive bindings. |

### Lean 4

**Observation.** Lean's official elaborator regression corpus contains the
precise source shape at issue: a structurally recursive definition over
natural numbers matches immediately on a recursive call. This is positive
evidence of admission, not merely the absence of a documented prohibition
([Lean regression case][lean-admission]).

Lean's structural elaborator walks the body, recognizes recursive
applications, obtains their values from the structural recursor's `below`
dictionary, and replaces the applications before later compiler lowering
([Lean result dictionary][lean-below],
[Lean structural replacement][lean-below-replace]). This is a rule-backed
rewrite, but it does **not** normalize the match away: it changes how the
recursive result is obtained.

LCNF then gives `Cases` a free-variable discriminant rather than an arbitrary
expression ([Lean LCNF datatype][lean-lcnf-type]). Its case translation first
visits the discriminant expression and uses the resulting free variable to
construct the case ([Lean LCNF case translation][lean-lcnf-case]). The helper
used by ordinary values introduces an auxiliary `let` when needed
([Lean LCNF naming][lean-lcnf-name]).

**Applicability to Ken.** Lean supplies two relevant precedents: recursion can
be removed from the later compiler's special concerns by elaboration, and a
later value-only case invariant can be enforced by the IR grammar. Neither
precedent establishes Ken's source-unreachability proposition. The transferable
argument is instead that all producers of the classifier's input must cross a
boundary at which computational scrutinees have already been named or framed.

### Agda

**Observation.** Agda's official passing-test corpus contains a recursive
function whose successor clause scrutinizes its recursive result with `with`
([Agda passing case][agda-admission]). Thus Agda also admits the construction.

Agda's Treeless IR makes the case scrutinee a variable index and explicitly
models non-recursive `let` binding ([Agda Treeless datatype][agda-treeless]).
The substitution implementation demonstrates that this is a maintained
invariant rather than a comment about expected inputs: when substitution would
put a non-variable in scrutinee position, it inserts a `let` and keeps the case
on the newly bound variable ([Agda substitution][agda-subst]).

**Applicability to Ken.** This is the closest analogue to the principled
version of `CheckedSubcontinuationFrame`. The important property is not that
one path happens to wrap the computation. It is that the downstream IR cannot
represent the unframed alternative, and ordinary transformations preserve that
fact. Ken could use this argument shape only after it can state and check the
same closure over every producer of the relevant IR.

### GHC

**Observation.** GHC demonstrates that a value-only case grammar is not the
only mature design. Core's `Case` contains an arbitrary expression, alongside
ordinary applications ([GHC Core expression][ghc-core]) and recursive bindings
([GHC Core binding][ghc-bind]).
STG likewise contains an expression-valued case scrutinee
([GHC STG case][ghc-stg]). During Core-to-STG translation, GHC recursively
translates the scrutinee with the ordinary expression translator before
constructing the STG case ([GHC Core-to-STG case][ghc-core-to-stg]).

**Applicability to Ken.** GHC supports the broader retirement claim: admitting
the shape does not require a second whole-function recursive-descent lane. It
does not support pretending the shape is absent. Its transferable invariant is
uniform lowering: the nested computation is an ordinary expression whose
result is consumed by an ordinary case.

## Answers to the three questions

### 1. Do mature implementations admit the shape?

Yes. Lean and Agda have direct passing regression examples. No surveyed system
provided evidence for a source-level refusal, and none supplied a theorem that
normalization always removes the match. Lean rewrites structural recursive
calls to recursor-result lookups, but the surrounding match remains meaningful.

For Ken, this is a negative finding against retirement by source
unreachability. It agrees with the fixed Ken measurement rather than closing
its missing proposition.

### 2. What do they emit?

Lean and Agda use administrative sequencing into a value-only case IR. Lean
also eliminates direct structural recursive calls during elaboration by using
the recursor's result dictionary. GHC retains computation-valued scrutinees but
uses one uniform expression translation. None of the surveyed systems exposes
a special residual case shape that selects an otherwise obsolete
whole-function compiler.

For Ken, these are two possible argument families, not implementation
instructions:

- establish a value-scrutinee or framed-scrutinee invariant before the residual
  selector; or
- show that the functionized lane uniformly lowers the admitted expression
  form without consulting a separate descent classifier.

The fixed input rules out claiming that Ken already has the first argument:
the checked erasure path wraps the match, but generic non-plan erasure can emit
the bare form.

### 3. Is there a principled version of Ken's wrapper?

Yes. Agda Treeless and Lean LCNF make the analogous restriction structural:
case consumes a variable, and computations are sequenced before it. Agda's
substitution operation is especially probative because it repairs a would-be
violation while transforming the IR. This is a compiler-phase invariant, not a
surface-language or kernel theorem.

For Ken, a wrapper is principled only if its presence is required at a named
boundary, checked there, and preserved or reconstructed by every operation that
can produce the downstream match form. A wrapper supplied by one current route
is an implementation fact, not yet the prior-art invariant.

## Recommendation and next actionable step

**Recommendation: retirement remains supported, conditionally.** The Architect
can use the following argument shape:

> Before the residual selector, every admitted computational match scrutinee is
> represented in a form whose immediate node is a named or checked frame.
> Every producer and transformer of that IR is covered by construction or by a
> verifier. Therefore the selector cannot observe the bare nested
> computational-match shape, even though the source language admits it.

The next actionable step is an Architect ruling on whether that statement is
the required pre-classifier IR invariant. If it is, a later build node can make
the invariant structural or checked and demonstrate producer closure. Only
after that evidence exists does the prior-art argument discharge deletion of
the exceptional lane.

If the federation chooses not to establish such an invariant, this survey does
not support retirement on the current ground. The GHC alternative would still
require a separately demonstrated uniform-lowering argument inside Ken; prior
art alone cannot supply it.

## Sources

All implementation links are pinned to the revisions examined on 2026-08-16.

[lean-admission]: https://github.com/leanprover/lean4/blob/57eb1ae3d0d440f29d1f35e9699c6df4d46c2620/tests/elab/structuralRec1.lean#L99-L104
[lean-below]: https://github.com/leanprover/lean4/blob/57eb1ae3d0d440f29d1f35e9699c6df4d46c2620/src/Lean/Elab/PreDefinition/Structural/BRecOn.lean#L110-L123
[lean-below-replace]: https://github.com/leanprover/lean4/blob/57eb1ae3d0d440f29d1f35e9699c6df4d46c2620/src/Lean/Elab/PreDefinition/Structural/BRecOn.lean#L125-L167
[lean-lcnf-type]: https://github.com/leanprover/lean4/blob/57eb1ae3d0d440f29d1f35e9699c6df4d46c2620/src/Lean/Compiler/LCNF/Basic.lean#L367-L377
[lean-lcnf-case]: https://github.com/leanprover/lean4/blob/57eb1ae3d0d440f29d1f35e9699c6df4d46c2620/src/Lean/Compiler/LCNF/ToLCNF.lean#L621-L677
[lean-lcnf-name]: https://github.com/leanprover/lean4/blob/57eb1ae3d0d440f29d1f35e9699c6df4d46c2620/src/Lean/Compiler/LCNF/ToLCNF.lean#L257-L275
[agda-admission]: https://github.com/agda/agda/blob/d8a73ff720197796fb64c7652202d33e7abb3eb6/test/Succeed/WithInParModule.agda#L21-L25
[agda-treeless]: https://github.com/agda/agda/blob/d8a73ff720197796fb64c7652202d33e7abb3eb6/src/full/Agda/Syntax/Treeless.hs#L54-L78
[agda-subst]: https://github.com/agda/agda/blob/d8a73ff720197796fb64c7652202d33e7abb3eb6/src/full/Agda/Compiler/Treeless/Subst.hs#L20-L42
[ghc-core]: https://github.com/ghc/ghc/blob/3915e9827adebe83b7367a8a1cf90c57dc1d7d98/compiler/GHC/Core.hs#L261-L275
[ghc-bind]: https://github.com/ghc/ghc/blob/3915e9827adebe83b7367a8a1cf90c57dc1d7d98/compiler/GHC/Core.hs#L327-L340
[ghc-stg]: https://github.com/ghc/ghc/blob/3915e9827adebe83b7367a8a1cf90c57dc1d7d98/compiler/GHC/Stg/Syntax.hs#L276-L289
[ghc-core-to-stg]: https://github.com/ghc/ghc/blob/3915e9827adebe83b7367a8a1cf90c57dc1d7d98/compiler/GHC/CoreToStg.hs#L466-L486
