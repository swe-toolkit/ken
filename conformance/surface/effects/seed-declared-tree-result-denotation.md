# Declared tree result denotations

Format: `../../README.md`. These cases pin the result translation for
`proc ... visits ρ` in `spec/30-surface/36-effects.md §2.5`, including the
Architect's rulings `evt_7jtkd0pad55ke`, `evt_rywsrnwtq0rt`,
`evt_3c1f42fq76n8p`, and `evt_1rtqn5ns08yp1`. They are input-to-behavior
expectations, not reports of execution. The surface spelling follows existing
effect examples; no official conformance runner or semantic Ken tests were run
for this seed.

The effects and tree types below are the existing public families. `IO R`
stands for the Console tree with return type `R`; `HostIO a R` is the host
signature indexed by `Auth`. A row remains a declared upper bound, but every
label in it must inject into the declared tree signature. Host-operation labels
are members of the host-operation alphabet and inject wherever their family
does. Examples name `FsOpen`; they do not replace the full alphabet rule.

## Declared result with an ITree head

### surface/effects/tree-result-row-injection-family
- spec: `36 §2.3`, `§2.5`
- given: the same tree-returning body in two declarations:

  ```ken
  proc p : IO Unit visits [Console] = print_line "x"
  proc q : IO Unit visits [Console, Clock] = print_line "x"
  ```

- expect: `p` accepts and denotes to its declared `IO Unit` tree. `q` is
  refused because `Clock` has no summand in `IO`, even though the body uses only
  `Console`. The pair differs only by the extra row label.
- why: a declared tree is its own denotation, and every declared row label
  must inject into its signature. The correct rule separates acceptance from
  refusal; an implementation that ignores row injection accepts both.

### surface/effects/tree-result-host-operation-injection
- spec: `36 §2.3`, `§2.5` (host-operation alphabet)
- given: a tree-result declaration with `HostIO APartial Unit`, row
  `[FS, FsOpen]`, and a tree-valued body. Its control has the same declared
  tree and row `[FS]`. Also test the same operation label in an `IO Unit`
  result with row `[Console, FsOpen]`, against the same-form `[Console]`
  control.
- expect: the HostIO declaration accepts: `FsOpen` injects through its `FS`
  family. The `IO` declaration refuses because `FsOpen` has no `FS` summand
  there; its `[Console]` control accepts. For each host-operation label `o`,
  the same relation holds between `o` and its family: it injects exactly
  where that family injects. In particular, FS and Process operation labels
  inject into `FSOp a` and `HostIO a` wherever their family does.
- why: host operations are part of the injection population, not labels to
  ignore or treat as independent families. The same `FsOpen` across the two
  signatures pins both sides of the family mapping. The parameterized
  expectation is over the host-operation alphabet, not a frozen list of
  today's names.

### surface/effects/tree-result-transparent-head
- spec: `36 §2.5`; `33 §1` (transparent definitions)
- given: a transparent result alias and a tree-returning procedure:

  ```ken
  def ConsoleTree = IO Unit
  proc p : ConsoleTree visits [Console] = print_line "x"
  ```

- expect: after weak-head normalization, the result is the declared tree
  `IO Unit`; the procedure does not return an outer `IO (IO Unit)` tree.
- why: classification uses the declared result's weak-head-normal form,
  not only a syntactically exposed `ITree` head.

## Function-valued declared results

### surface/effects/function-result-post-arrow-tree
- spec: `36 §2.5`
- given: a function-valued result whose final result is not a tree:

  ```ken
  proc g (n : Int) : Int -> Int visits [Console] =
    \m. let w = print_line "x" in n
  ```

- expect: accepts under the ruled post-arrow behavior. The denotation is
  `Int -> Int -> IO Int`: the ordinary function binders remain, and the
  row's tree wraps the final result after the last arrow. The Console action
  runs when that returned function is applied.
- why: this distinguishes the selected behavior from refusing every
  non-tree-final arrow result. The equivalent parameter form
  `proc g (n : Int) (m : Int) : Int visits [Console]` has the same denotation.
  Expected acceptance is a contract target, not a measurement of the current
  implementation.

### surface/effects/function-result-outer-action-refused
- spec: `36 §2.5` (post-arrow rule and outer-action refusal)
- given: the existing `outer` shape, in a valid effect fixture:

  ```ken
  proc outer : Unit -> IO Unit visits [Console] =
    let w = print_line "x" in
    \_. ITree.Ret ConsoleOp console_resp Unit MkUnit
  ```

- expect: refuses with E0038. The action is before the last arrow, where the
  body returns a function rather than a tree. Moving it under the binder would
  change how many times it runs. The same action inside the returned lambda
  is accepted and sequences into that lambda's `IO Unit` result.
- why: pins the post-arrow placement against both a tree-before-arrows
  interpretation and moving an outer action beneath a binder. Before the
  last arrow, the outer level returns a function rather than a tree, so there
  is no tree to sequence the action into. The Architect reported this refusal
  as measured in `crates/ken-cli/tests/lang_proc_non_tree_let.rs`; CV did not
  run it.

### surface/effects/function-result-tree-final-control
- spec: `36 §2.5`
- given: a function-valued result `Unit -> HostIO APartial ExitCode`, whose
  final result after the arrow is a declared tree.
- expect: its final `HostIO APartial ExitCode` is its own denotation after the
  arrow. The result is a function returning that tree, not an outer tree of
  functions. No existing declared-tree core type changes.
- why: the tree-final case is the same rule read after all arrows. This
  control prevents the non-tree-final case from being implemented by a
  special exception that changes a tree-final function.

## Filesystem authority index

### surface/effects/non-tree-fs-index-from-written-cap
- spec: `36 §2.5`; `62-authority.md §1`
- given: an FS producer with the accepted closed index:

  ```ken
  proc read (cap : Cap APartial) (path : Bytes)
      : Result FileError Bytes visits [FS] =
    read_bytes APartial cap path
  ```

  Compare with the same declaration and body after removing the `cap` binder.
- expect: the first accepts and denotes to `Cap APartial -> Bytes ->
  FS APartial (Result FileError Bytes)` (with the declared argument order).
  The second refuses: the call has no written capability value in scope. The
  row does not insert a capability, and a caller's index supplies none.
- why: the `FS` tree's index comes from the written `Cap a` parameter.
  Varying only that binder gives an accepted/refused pair. The public
  `read_bytes` signature is the one shown in
  `catalog/packages/Capability/Filesystem/Authority.ken.md`; the error kind
  for the missing name is not part of this case.

### surface/effects/non-tree-fs-bound-index
- spec: `36 §2.5`
- given: a non-tree `[FS]` procedure whose telescope binds
  `(a : Auth)` before `(cap : Cap a)` and whose body returns an FS action.
- expect: its denotation has the ordinary written Pi telescope and one tree
  index: `(a : Auth) -> Cap a -> ... -> FS a B`. A closed `Cap APartial`
  or `Cap AFull` supplies the corresponding closed index. If two FS actions
  in one result require different written Auth indices, the procedure is
  refused because one result tree cannot carry both indices; the same-form
  control with matching indices accepts.
- why: neither a hidden Auth binder nor caller-index inference supplies the
  tree index. The matching/mismatching pair pins the one-index constraint.

### surface/effects/free-fs-row-collapses
- spec: `36 §2.4`, `§2.5`
- given: a free, non-tree procedure with no `Cap` parameter and an otherwise
  effect-free body:

  ```ken
  proc identity (x : Int) : Int visits [FS] = x
  proc ambient (x : Int) : Int visits [FS, Console] = x
  ```

- expect: `identity` has denotation `Int -> Int`; `[FS]` contributes no
  summand and the empty signature collapses. `ambient` has an `IO Int`
  result: only the remaining ambient `Console` label contributes a tree.
  Both rows remain declared upper bounds. A free `[FS]` procedure that
  returns or sequences an FS-family action without a written `Cap` refuses.
- why: this distinguishes a declared `[FS]` bound from a source of FS
  authority, while preserving a tree for any remaining ambient effect.

## Ensures on a declared-tree result

### surface/effects/tree-result-ensures-all-ret
- spec: `36 §2.5`, `§4.3`
- given: a declared-tree procedure returning
  `t = Vis (Console.Write "tick") (\_. Ret False)` at result type
  `IO Bool`, with `ensures Equal Bool result False`. Compare with the same
  tree and `ensures Equal Bool result True`.
- expect: the first emits a separate obligation
  `AllRet (\r. Equal Bool r False) t`, which reduces through the `Vis`
  response and `Ret False` to a proved equality. The second obligation
  reduces to the false equality `Equal Bool False True` and is not
  discharged. The procedure's result remains `IO Bool`; the payload is not
  refined to a proof-carrying pair and the tree is not run twice.
- why: `ensures` is a relational post on every return leaf, not a change to
  the declared result type. The same tree with the opposite post is the
  discriminating refusal control.

### surface/effects/non-tree-result-ensures-keeps-payload
- spec: `36 §2.5` (non-tree `B` with a nonempty row)
- given: the same caller with two versions of a non-tree-result proc:

  ```ken
  proc one : Int ensures Equal Int result 1 visits [Console] = 1
  proc caller : Int visits [Console] =
    let n : Int = one in n
  ```

  The control changes only `one`'s postcondition to
  `ensures Equal Int result 2`.
- expect: the first denotes to `IO Int`, with obligation
  `AllRet (λ r. Equal Int r 1) (Ret 1)`, which is discharged. `caller`
  binds `n : Int`. The control's `AllRet (λ r. Equal Int r 2) (Ret 1)`
  reduces to `Equal Int 1 2` and is not discharged; the caller's binding
  still has type `Int`.
- why: this distinguishes a tree whose leaf has type `⟦B⟧` from
  `Σ ⟦B⟧ ψ`: a caller binds the result at its declared `Int`, not a
  proof-carrying pair. Architect reports a prior payload-refinement build
  refused this accepted caller with E0038 (`evt_7x0p31v0tkkvj`); that
  evidence was not rerun for this seed. Architect reports the current-main
  sentinel `ensured_changed_proc_refuses_until_tree_post_is_specified` refuses
  both rows by name; CV did not rerun it. The expected outputs are contract
  targets, not CV measurements.

### surface/effects/tree-result-ensures-bind-cut
- spec: `36 §2.5`, `§4.3` (`all_ret_bind`)
- given: `t : ITree E X`, a checked premise `AllRet P t`, a continuation
  `k : X -> ITree E Y`, and a checked function
  `(x : X) -> P x -> AllRet Q (k x)`.
- expect: the checked `all_ret_bind` declaration derives
  `AllRet Q (bind t k)`. A caller still receives `x : X`; `P x` is a
  separate postcondition fact, not part of the tree's return payload.
- why: pins the cut used to sequence tree-result procedures and prevents
  introducing a second postcondition encoding.

### surface/effects/all-ret-is-checked-prelude-fold
- spec: `36 §2.5`, `§4.3`; `14 §3` (`elim_ITree`)
- given: the prelude declarations `AllRet` and `all_ret_bind`, a client using
  them on a `Ret` and a `Vis` tree, and this direct surface-recursive copy:

  ```ken
  fn has_post (e : Type) (r : e → Type) (a : Type) (p : a → Prop) (t : ITree e r a) : Prop =
    match t {
      ITree.Ret x ↦ p x;
      ITree.Vis op k ↦ (resp : r op) → has_post e r a p (k resp)
    }
  ```
- expect: both prelude declarations and the client kernel-check as ordinary
  definitions over the delivered `ITree` eliminator. Loading them adds no
  `trusted_base()` entry. The surface-recursive copy is rejected by the
  termination check, not accepted as a replacement.
- why: the fold and cut are checked prelude declarations, not surface
  recursion, primitives, or postulates. Architect reports the direct
  surface-recursive form rejects with K0014 NotTerminating; CV did not run
  that probe. No kernel change, new trusted declaration, official runner
  coverage, or candidate approval is claimed here.

## Space-operation ensures

### surface/effects/space-ensures-state-transformer
- spec: `36 §2.5` (scope boundary), `§4.3`
- given: a space operation and a control that changes only its postcondition:

  ```ken
  space Counter {
    mut n : Int = 0
    proc bump () : Unit visits [Counter]
        ensures n == old(n) + 1 = n becomes n + 1
  }
  ```

  In the control, replace the postcondition with `ensures n == old(n)`.
- expect: the first obligation checks through §4.3's outer
  state-transformer pair, reduces to `s.n + 1 == s.n + 1`, and is discharged
  by reflexivity. The control reduces to `s.n + 1 == s.n` and is not
  discharged. The declared result remains `Unit`; ψ observes the pre- and
  post-state components of the `R × S` pair.
- why: the row names a space label, so §4.3 governs its `ensures` contract.
  §2.5's separate `AllRet` post governs declarations whose row names no
  space label. The existing
  `surface/effects/tree-result-ensures-all-ret` case pins that no-space side.
  This case does not settle a space operation whose declared result is itself
  a tree.
