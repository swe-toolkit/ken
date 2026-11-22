# Appended successor lists build the graph union

Format: `../../README.md`.
Spec: `spec/50-stdlib/62-graphs.md` §20.
Frame: `docs/program/issues/SPEC-GRAPHS-SUCCESSOR-APPEND-UNION.md`.

Status: authored; this seed has not been executed. Architect's AC-0
checked the generic proof on the released base in scratch, with no
`Axiom` and equal `trusted_base()` identity sets (114 → 114). That is
not seed execution, official conformance, CI, publication, or landing.

The open client exercises the generic public law. The closed `Bool`
fixture distinguishes the union from either operand alone at the edge
`True → False`. The swapped control changes which operand contributes
that edge.

## Accepted client

Accept the complete client below before testing either isolated
refusal. Each refusal starts from a fresh copy of this accepted client,
then appends only its own refused declaration.

```ken
import Core.Classes.LawfulClasses (DecEq, DecEq_instance_Bool)
import Algorithm.Graphs.Dependency
  (edge, graph_from_successors, graph_union,
   successors_append, graph_from_successors_append)

fn none_out (v : Bool) : List Bool = Nil Bool

fn true_to_false (v : Bool) : List Bool =
  match v {
    True ↦ Cons Bool False (Nil Bool);
    False ↦ Nil Bool
  }

theorem open_append
      (q : Type) (d : DecEq q) (f : q → List q) (g : q → List q) (s : q) (t : q)
    : Equal Bool
        (edge q (graph_from_successors q d (successors_append q f g)) s t)
        (edge
          q
          (graph_union
            q
            (graph_from_successors q d f)
            (graph_from_successors q d g))
          s
          t) =
  graph_from_successors_append q d f g s t

theorem seed_union
    : Equal Bool
        (edge Bool
          (graph_from_successors Bool DecEq_instance_Bool
            (successors_append Bool none_out true_to_false))
          True
          False)
        (edge Bool
          (graph_union
            Bool
            (graph_from_successors Bool DecEq_instance_Bool none_out)
            (graph_from_successors Bool DecEq_instance_Bool true_to_false))
          True
          False) =
  graph_from_successors_append
    Bool DecEq_instance_Bool none_out true_to_false True False

theorem appended_edge_true
    : Equal Bool
        (edge Bool
          (graph_from_successors Bool DecEq_instance_Bool
            (successors_append Bool none_out true_to_false))
          True
          False)
        True =
  Proved

theorem left_edge_false
    : Equal Bool
        (edge Bool
          (graph_from_successors Bool DecEq_instance_Bool none_out)
          True
          False)
        False =
  Proved

theorem right_edge_true
    : Equal Bool
        (edge Bool
          (graph_from_successors Bool DecEq_instance_Bool true_to_false)
          True
          False)
        True =
  Proved

theorem seed_union_swapped
    : Equal Bool
        (edge Bool
          (graph_from_successors Bool DecEq_instance_Bool
            (successors_append Bool true_to_false none_out))
          True
          False)
        (edge Bool
          (graph_union
            Bool
            (graph_from_successors Bool DecEq_instance_Bool true_to_false)
            (graph_from_successors Bool DecEq_instance_Bool none_out))
          True
          False) =
  graph_from_successors_append
    Bool DecEq_instance_Bool true_to_false none_out True False
```

### `stdlib/graphs/graph-successor-append-union-open-client`

- spec: §20, `graph_from_successors_append`.
- promise class: durable invariant.
- given: the generic `open_append` client in the accepted block.
- expect: the client is accepted for arbitrary `q`, `d`, `f`, `g`, `s`,
  and `t` by applying the public theorem.
- claimed: the public generic law is available to ordinary client code.
- gap: this client use does not replace checking the theorem's proof.
- why: the declaration applies the public theorem to generic endpoint
  arguments; the closed `Bool` fixture does not replace that proof.

### `stdlib/graphs/graph-successor-append-union-bool`

- spec: §20, the pointwise edge equality.
- promise class: durable invariant.
- given: `seed_union` and the three `Proved` value controls in the
  accepted client, at source `True` and target `False`.
- expect: `seed_union` is accepted. The appended-adaptor edge is `True`,
  the `none_out` edge is `False`, and the `true_to_false` edge is `True`.
- controls: the values are checked independently of `seed_union`'s law
  application.
- claimed: the appended adaptor and union have the same edge at the
  selected endpoints.
- gap: one closed `Bool` witness does not establish the generic law.
- why: `f = none_out` contributes no edge while `g = true_to_false`
  contributes `True → False`.

### `stdlib/graphs/graph-successor-append-union-swapped`

- spec: §20, `graph_from_successors_append`.
- promise class: durable invariant.
- given: `seed_union_swapped` in the accepted client, with `f` and `g`
  exchanged and the same `True → False` endpoints.
- expect: the swapped law application is accepted; its appended-adaptor
  edge is `True`.
- controls: the same value controls show that `true_to_false` supplies
  the edge and `none_out` does not.
- claimed: exchanging the two successor-list inputs preserves the law.
- gap: this one swapped instance does not establish the law for all
  carriers and successor functions.
- why: this accepted sibling positions the contributing list first and
  `none_out` second for the right-only refusal below.

## Isolated refused claims

For each refusal, start from a fresh copy of the complete accepted
client. Append only that refusal's theorem. The accepted same-form law
application is already in the prefix; do not include either refused
theorem in the other refusal's prefix.

### `stdlib/graphs/graph-successor-append-union-left-only-refused`

- spec: §20, `graph_from_successors_append`.
- promise class: durable invariant.
- given: the accepted prefix and `seed_left_only`, which keeps
  `seed_union`'s inputs and proof term, but uses a distinct name and
  replaces the right-hand union with the `f` graph alone.
- expect: `KernelRejected(TypeMismatch)` for the refused declaration.
  Its reported source span is contained within that declaration; do not
  pin a finer span.
- controls: `seed_union` accepts in the same prefix. The independent
  values show an appended edge of `True` and an `f`-only edge of
  `False`.
- claimed: the union edge cannot be replaced by the left-input edge.
- gap: this one Bool counterexample does not establish the generic law.
- why: the union contains the edge contributed by `g`, so it cannot
  equal the graph from `f` alone.

Append only this declaration to a fresh accepted prefix:

```ken
theorem seed_left_only
    : Equal Bool
        (edge Bool
          (graph_from_successors Bool DecEq_instance_Bool
            (successors_append Bool none_out true_to_false))
          True
          False)
        (edge Bool
          (graph_from_successors Bool DecEq_instance_Bool none_out)
          True
          False) =
  graph_from_successors_append
    Bool DecEq_instance_Bool none_out true_to_false True False
```

### `stdlib/graphs/graph-successor-append-union-right-only-refused`

- spec: §20, `graph_from_successors_append`.
- promise class: durable invariant.
- given: the accepted prefix and `seed_right_only`, which keeps
  `seed_union_swapped`'s inputs and proof term, but uses a distinct name
  and replaces the right-hand union with the second-input `g` graph alone.
- expect: `KernelRejected(TypeMismatch)` for the refused declaration.
  Its reported source span is contained within that declaration; do not
  pin a finer span.
- controls: `seed_union_swapped` accepts in the same prefix. The
  independent values show an appended edge of `True` and a second-input
  `none_out` edge of `False`.
- claimed: the union edge cannot be replaced by the second-input edge.
- gap: this swapped Bool counterexample does not establish the generic
  law.
- why: after swapping the inputs, the second operand contributes no
  edge; the appended graph still contains the edge from the first.

Append only this declaration to a fresh accepted prefix:

```ken
theorem seed_right_only
    : Equal Bool
        (edge Bool
          (graph_from_successors Bool DecEq_instance_Bool
            (successors_append Bool true_to_false none_out))
          True
          False)
        (edge Bool
          (graph_from_successors Bool DecEq_instance_Bool none_out)
          True
          False) =
  graph_from_successors_append
    Bool DecEq_instance_Bool true_to_false none_out True False
```

## Evidence boundary

Architect's AC-0 reports a checked generic proof on scratch at the
released base, no added `Axiom`, and equal `trusted_base()` identity
sets (114 → 114). It also reports both refusals as whole-declaration
`KernelRejected(TypeMismatch)`. These are D0 scratch results, not an
execution of this seed. This file makes no claim about package tests,
official conformance, CI, publication, landing, or measured landed
trust.
