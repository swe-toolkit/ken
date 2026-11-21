# Reachability absorbs an included graph

Format: `../../README.md`.
Spec: `spec/50-stdlib/62-graphs.md` §19, child
`4f1bb70b0c88e7794e3faf4820ae446da91c8757`.
Ruling: Architect AC-0 D0, `evt_1b4cgkw5bezx4`.

Status: authored behavioral oracle. The accepted and X-ADDS source
blocks reproduce the D0 inputs below. The edge-inclusion pair is an
additional oracle derived from §19. D0 checks were scratch measurements
on base `15e57bf397ebbbb8b1b7186381ed6bf9b1bb6c47`; they do not execute
this seed or check the Spec child or an assembled candidate. This seed
has not been run by the official runner. No CI, publication, landing, or
landed-trust measurement is claimed.

The fixture is `Vertex3`: `g_base` has V0 → V1 and V2 → V0;
`h_sub` has only V0 → V1; `h_adds` has V1 → V2. From V0,
`g_base` does not reach V2, while its union with `h_adds` does. The
accepted `h_sub` case uses an edge-inclusion proof keyed to `g_base`.

The promise class is **durable invariant**. These rows apply public
checked laws and assert closed reachability observations, not roster
counts or repository spellings. The finite fixture is only a witness:
it neither proves the generic theorem nor decomposes union reachability.

## Accepted client

The D0 accepted input is `arch-absorb-accepted.ken.md`, SHA-256
`4a26718aa59f7bba38b13039b50445e8f49dbb42d05a1018495f23435834a426`.
It contains open-argument applications, closed value controls, an
independent `h_sub` equality, and closed law applications for idempotence
and absorption.

```ken
import Data.Finite.Finite (Finite, MkFinite)

import Data.Collections.Derived (list_elem_head, list_elem_later)

import Algorithm.Graphs.Dependency
  (Graph,
   MkGraph,
   edge,
   graph_union,
   graph_reachable,
   graph_union_reachable_absorb,
   graph_union_reachable_idem)

data Vertex3 : Type where {
  V0 : Vertex3;
  V1 : Vertex3;
  V2 : Vertex3
}

const vertex3_finite : Finite Vertex3 =
  MkFinite
    Vertex3
    (Cons Vertex3 V0 (Cons Vertex3 V1 (Cons Vertex3 V2 (Nil Vertex3))))
    (λv.
      match v {
        V0 ↦ list_elem_head Vertex3 V0 (Cons Vertex3 V1 (Cons Vertex3 V2 (Nil Vertex3)));
        V1 ↦
          list_elem_later
            Vertex3
            V1
            V0
            (Cons Vertex3 V1 (Cons Vertex3 V2 (Nil Vertex3)))
            (list_elem_head Vertex3 V1 (Cons Vertex3 V2 (Nil Vertex3)));
        V2 ↦
          list_elem_later
            Vertex3
            V2
            V0
            (Cons Vertex3 V1 (Cons Vertex3 V2 (Nil Vertex3)))
            (list_elem_later
              Vertex3
              V2
              V1
              (Cons Vertex3 V2 (Nil Vertex3))
              (list_elem_head Vertex3 V2 (Nil Vertex3)))
      })

const g_base : Graph Vertex3 =
  MkGraph
    Vertex3
    (λu.
      λv.
        match u {
          V0 ↦
            match v {
              V0 ↦ False;
              V1 ↦ True;
              V2 ↦ False
            };
          V1 ↦
            match v {
              V0 ↦ False;
              V1 ↦ False;
              V2 ↦ False
            };
          V2 ↦
            match v {
              V0 ↦ True;
              V1 ↦ False;
              V2 ↦ False
            }
        })

const h_sub : Graph Vertex3 =
  MkGraph
    Vertex3
    (λu.
      λv.
        match u {
          V0 ↦
            match v {
              V0 ↦ False;
              V1 ↦ True;
              V2 ↦ False
            };
          V1 ↦
            match v {
              V0 ↦ False;
              V1 ↦ False;
              V2 ↦ False
            };
          V2 ↦
            match v {
              V0 ↦ False;
              V1 ↦ False;
              V2 ↦ False
            }
        })

const h_adds : Graph Vertex3 =
  MkGraph
    Vertex3
    (λu.
      λv.
        match u {
          V0 ↦
            match v {
              V0 ↦ False;
              V1 ↦ False;
              V2 ↦ False
            };
          V1 ↦
            match v {
              V0 ↦ False;
              V1 ↦ False;
              V2 ↦ True
            };
          V2 ↦
            match v {
              V0 ↦ False;
              V1 ↦ False;
              V2 ↦ False
            }
        })

fn is_v2 (v : Vertex3) : Bool =
  match v {
    V0 ↦ False;
    V1 ↦ False;
    V2 ↦ True
  }

theorem base_misses_v2 : Equal Bool (graph_reachable Vertex3 vertex3_finite g_base V0 is_v2) False =
  Proved

theorem adds_reaches_v2
    : Equal Bool
        (graph_reachable Vertex3 vertex3_finite (graph_union Vertex3 g_base h_adds) V0 is_v2)
        True =
  Proved

theorem gen_absorb
      (q : Type)
      (fq : Finite q)
      (g : Graph q)
      (h : Graph q)
      (s : q)
      (target : q → Bool)
      (included : (u : q)
        → (v : q)
        → Equal Bool (edge q h u v) True
        → Equal Bool (edge q g u v) True)
    : Equal Bool
        (graph_reachable q fq (graph_union q g h) s target)
        (graph_reachable q fq g s target) =
  graph_union_reachable_absorb q fq g h s target included

theorem gen_idem
      (q : Type) (fq : Finite q) (g : Graph q) (s : q) (target : q → Bool)
    : Equal Bool
        (graph_reachable q fq (graph_union q g g) s target)
        (graph_reachable q fq g s target) =
  graph_union_reachable_idem q fq g s target

theorem sub_same
    : Equal Bool
        (graph_reachable Vertex3 vertex3_finite (graph_union Vertex3 g_base h_sub) V0 is_v2)
        (graph_reachable Vertex3 vertex3_finite g_base V0 is_v2) =
  Proved

theorem law_idem
    : Equal Bool
        (graph_reachable Vertex3 vertex3_finite (graph_union Vertex3 g_base g_base) V0 is_v2)
        (graph_reachable Vertex3 vertex3_finite g_base V0 is_v2) =
  graph_union_reachable_idem Vertex3 vertex3_finite g_base V0 is_v2

theorem law_absorb
    : Equal Bool
        (graph_reachable Vertex3 vertex3_finite (graph_union Vertex3 g_base h_sub) V0 is_v2)
        (graph_reachable Vertex3 vertex3_finite g_base V0 is_v2) =
  graph_union_reachable_absorb
    Vertex3
    vertex3_finite
    g_base
    h_sub
    V0
    is_v2
    (λu.
      λv.
        match u {
          V0 ↦
            match v {
              V0 ↦ λe. absurd e;
              V1 ↦ λe. Proved;
              V2 ↦ λe. absurd e
            };
          V1 ↦
            match v {
              V0 ↦ λe. absurd e;
              V1 ↦ λe. absurd e;
              V2 ↦ λe. absurd e
            };
          V2 ↦
            match v {
              V0 ↦ λe. Proved;
              V1 ↦ λe. absurd e;
              V2 ↦ λe. absurd e
            }
        })
```

### `stdlib/graphs/R-GEN-ABSORB` and `R-GEN-IDEM`

- spec: §19, the public absorption and idempotence laws
- promise class: durable invariant
- given: arbitrary `q`, `fq`, `g`, `h`, `s`, `target`, and `included`
  for absorption; arbitrary `q`, `fq`, `g`, `s`, and `target` for
  idempotence
- expect: both declarations are accepted as direct applications of the
  public checked laws at open arguments
- why: these rows establish client usability at the published binder
  order; they do not prove either generic theorem body

### `stdlib/graphs/R-CLOSED-VALUE-CONTROLS`

- spec: §19 and §1.2 reachability semantics
- promise class: durable invariant
- given: the closed `g_base`, `h_adds`, V0 start, and `is_v2` target
- expect: `base_misses_v2` proves False and `adds_reaches_v2` proves
  True, each independently with `Proved`
- why: these distinct values make the X-ADDS conclusion false; they are
  controls, not a decomposition of generic reachability

### `stdlib/graphs/R-SUB-SAME`

- spec: §19, absorption with `h_sub` included in `g_base`
- promise class: durable invariant
- given: the closed reachability observations for
  `graph_union g_base h_sub` and `g_base`
- expect: `sub_same` proves their equality independently with `Proved`
- why: this same-form control does not apply the new public law

### `stdlib/graphs/R-LAW-IDEM`

- spec: §19, the premise-free idempotence law
- promise class: durable invariant
- given: `g_base` as both operands, with V0 and `is_v2`
- expect: `law_idem` is accepted by direct application of
  `graph_union_reachable_idem`

### `stdlib/graphs/R-LAW-ABSORB`

- spec: §19, h-to-g inclusion and union-left/g-right orientation
- promise class: durable invariant
- given: `h_sub`'s V0 → V1 edge is included in `g_base`; the inclusion
  term proves the two `g_base` edges with `Proved` and the other seven
  vertex pairs by `absurd`
- expect: `law_absorb` is accepted by direct application of
  `graph_union_reachable_absorb`
- why: the closed witness checks the public premise direction and
  argument order without purporting to check the catalog's generic proof

## Isolated edge-inclusion premise

This pair checks the `included` type without a reachability equality
conclusion. Both inputs use a separate `Bool` fixture: `g_base` has
`False → False` and `False → True`; `h_sub` keeps only `False → True`;
`h_adds` has only `True → False`, absent from `g_base`. The proof term
is byte-identical; only the h operand in its inclusion type changes.
These names are local to this fixture, not the earlier `Vertex3` graph.

### `stdlib/graphs/R-EDGE-INCLUSION`

- spec: §19, `included` requires edge inclusion from `h` to `g`
- promise class: durable invariant
- given: the direct edge-inclusion type for `h_sub` and `g_base`, with the
  same pointwise proof used by X-EDGE-INCLUSION
- expect: `sub_edges_included` is accepted
- control: the proof's `Proved` arm is required at `False → True`; the other
  arms eliminate impossible `h_sub` edges
- why: this checks the premise independently of reachability equality

```ken
import Algorithm.Graphs.Dependency (Graph, MkGraph, edge)

const g_base : Graph Bool =
  MkGraph
    Bool
    (λu.
      λv.
        match u {
          False ↦
            match v {
              False ↦ True;
              True ↦ True
            };
          True ↦
            match v {
              False ↦ False;
              True ↦ False
            }
        })

const h_sub : Graph Bool =
  MkGraph
    Bool
    (λu.
      λv.
        match u {
          False ↦
            match v {
              False ↦ False;
              True ↦ True
            };
          True ↦
            match v {
              False ↦ False;
              True ↦ False
            }
        })

const h_adds : Graph Bool =
  MkGraph
    Bool
    (λu.
      λv.
        match u {
          False ↦
            match v {
              False ↦ False;
              True ↦ False
            };
          True ↦
            match v {
              False ↦ True;
              True ↦ False
            }
        })

theorem sub_edges_included
    : (u : Bool)
      → (v : Bool)
      → Equal Bool (edge Bool h_sub u v) True
      → Equal Bool (edge Bool g_base u v) True
  = λu.
      λv.
        match u {
          False ↦
            match v {
              False ↦ λe. absurd e;
              True ↦ λe. Proved
            };
          True ↦
            match v {
              False ↦ λe. absurd e;
              True ↦ λe. absurd e
            }
        }
```

### `stdlib/graphs/X-EDGE-INCLUSION`

- spec: §19, `included` is the sole graph premise
- promise class: durable invariant
- given: the same graph definitions and proof as R-EDGE-INCLUSION, with
  only the inclusion target changed from `h_sub` to `h_adds`
- expect: `KernelRejected` at `adds_edges_not_included`; do not pin the
  diagnostic message
- control: R-EDGE-INCLUSION accepts the same proof term for `h_sub`
- why: at `True → False`, `h_adds` has an edge and `g_base` does not, so
  the unchanged `absurd` arm cannot prove the inclusion result

```ken
import Algorithm.Graphs.Dependency (Graph, MkGraph, edge)

const g_base : Graph Bool =
  MkGraph
    Bool
    (λu.
      λv.
        match u {
          False ↦
            match v {
              False ↦ True;
              True ↦ True
            };
          True ↦
            match v {
              False ↦ False;
              True ↦ False
            }
        })

const h_sub : Graph Bool =
  MkGraph
    Bool
    (λu.
      λv.
        match u {
          False ↦
            match v {
              False ↦ False;
              True ↦ True
            };
          True ↦
            match v {
              False ↦ False;
              True ↦ False
            }
        })

const h_adds : Graph Bool =
  MkGraph
    Bool
    (λu.
      λv.
        match u {
          False ↦
            match v {
              False ↦ False;
              True ↦ False
            };
          True ↦
            match v {
              False ↦ True;
              True ↦ False
            }
        })

theorem adds_edges_not_included
    : (u : Bool)
      → (v : Bool)
      → Equal Bool (edge Bool h_adds u v) True
      → Equal Bool (edge Bool g_base u v) True
  = λu.
      λv.
        match u {
          False ↦
            match v {
              False ↦ λe. absurd e;
              True ↦ λe. Proved
            };
          True ↦
            match v {
              False ↦ λe. absurd e;
              True ↦ λe. absurd e
            }
        }
```

## Isolated refused claim

The D0 refused input is `arch-absorb-refused.ken.md`, SHA-256
`ded208ec3fa9f15a9c18c75b0e0f32a86c7b5f9c8a74f99de9ace73dc0ac1dcb`.
It repeats the same finite fixture and the two closed value controls,
then contains only `law_absorb`. Relative to the accepted row, only the
union conclusion and fourth law argument change from `h_sub` to
`h_adds`; the inclusion term is byte-identical.

```ken
import Data.Finite.Finite (Finite, MkFinite)

import Data.Collections.Derived (list_elem_head, list_elem_later)

import Algorithm.Graphs.Dependency
  (Graph,
   MkGraph,
   edge,
   graph_union,
   graph_reachable,
   graph_union_reachable_absorb,
   graph_union_reachable_idem)

data Vertex3 : Type where {
  V0 : Vertex3;
  V1 : Vertex3;
  V2 : Vertex3
}

const vertex3_finite : Finite Vertex3 =
  MkFinite
    Vertex3
    (Cons Vertex3 V0 (Cons Vertex3 V1 (Cons Vertex3 V2 (Nil Vertex3))))
    (λv.
      match v {
        V0 ↦ list_elem_head Vertex3 V0 (Cons Vertex3 V1 (Cons Vertex3 V2 (Nil Vertex3)));
        V1 ↦
          list_elem_later
            Vertex3
            V1
            V0
            (Cons Vertex3 V1 (Cons Vertex3 V2 (Nil Vertex3)))
            (list_elem_head Vertex3 V1 (Cons Vertex3 V2 (Nil Vertex3)));
        V2 ↦
          list_elem_later
            Vertex3
            V2
            V0
            (Cons Vertex3 V1 (Cons Vertex3 V2 (Nil Vertex3)))
            (list_elem_later
              Vertex3
              V2
              V1
              (Cons Vertex3 V2 (Nil Vertex3))
              (list_elem_head Vertex3 V2 (Nil Vertex3)))
      })

const g_base : Graph Vertex3 =
  MkGraph
    Vertex3
    (λu.
      λv.
        match u {
          V0 ↦
            match v {
              V0 ↦ False;
              V1 ↦ True;
              V2 ↦ False
            };
          V1 ↦
            match v {
              V0 ↦ False;
              V1 ↦ False;
              V2 ↦ False
            };
          V2 ↦
            match v {
              V0 ↦ True;
              V1 ↦ False;
              V2 ↦ False
            }
        })

const h_sub : Graph Vertex3 =
  MkGraph
    Vertex3
    (λu.
      λv.
        match u {
          V0 ↦
            match v {
              V0 ↦ False;
              V1 ↦ True;
              V2 ↦ False
            };
          V1 ↦
            match v {
              V0 ↦ False;
              V1 ↦ False;
              V2 ↦ False
            };
          V2 ↦
            match v {
              V0 ↦ False;
              V1 ↦ False;
              V2 ↦ False
            }
        })

const h_adds : Graph Vertex3 =
  MkGraph
    Vertex3
    (λu.
      λv.
        match u {
          V0 ↦
            match v {
              V0 ↦ False;
              V1 ↦ False;
              V2 ↦ False
            };
          V1 ↦
            match v {
              V0 ↦ False;
              V1 ↦ False;
              V2 ↦ True
            };
          V2 ↦
            match v {
              V0 ↦ False;
              V1 ↦ False;
              V2 ↦ False
            }
        })

fn is_v2 (v : Vertex3) : Bool =
  match v {
    V0 ↦ False;
    V1 ↦ False;
    V2 ↦ True
  }

theorem base_misses_v2 : Equal Bool (graph_reachable Vertex3 vertex3_finite g_base V0 is_v2) False =
  Proved

theorem adds_reaches_v2
    : Equal Bool
        (graph_reachable Vertex3 vertex3_finite (graph_union Vertex3 g_base h_adds) V0 is_v2)
        True =
  Proved

theorem law_absorb
    : Equal Bool
        (graph_reachable Vertex3 vertex3_finite (graph_union Vertex3 g_base h_adds) V0 is_v2)
        (graph_reachable Vertex3 vertex3_finite g_base V0 is_v2) =
  graph_union_reachable_absorb
    Vertex3
    vertex3_finite
    g_base
    h_adds
    V0
    is_v2
    (λu.
      λv.
        match u {
          V0 ↦
            match v {
              V0 ↦ λe. absurd e;
              V1 ↦ λe. Proved;
              V2 ↦ λe. absurd e
            };
          V1 ↦
            match v {
              V0 ↦ λe. absurd e;
              V1 ↦ λe. absurd e;
              V2 ↦ λe. absurd e
            };
          V2 ↦
            match v {
              V0 ↦ λe. Proved;
              V1 ↦ λe. absurd e;
              V2 ↦ λe. absurd e
            }
        })
```

### `stdlib/graphs/X-ADDS`

- spec: §19, the edge-inclusion premise from `h` to `g`
- promise class: durable invariant
- given: the byte-identical `g_base`-keyed inclusion term from
  R-LAW-ABSORB, with `h_adds` replacing `h_sub` in the union conclusion
  and fourth law argument
- expect: `KernelRejected` at the `law_absorb` declaration; do not pin
  the diagnostic message
- control: `base_misses_v2` and `adds_reaches_v2` remain independent
  accepted controls in this input
- why: `h_adds` contributes V1 → V2, absent from `g_base`. At that
  pair the unchanged inclusion term's `absurd` branch receives a proof
  of `Equal Bool True True`, so the premise does not hold. The rejection
  is attributable to this law application, not its imports or fixture.

## Coverage boundary

The open-argument rows check that clients can apply the published laws;
the premise-only pair isolates edge inclusion without reachability, and
the closed rows exercise the finite boundary and its controls. They do
not prove or decompose the generic theorem, characterize all finite
graphs, or add a public edge-level law.

## Evidence boundary

Architect's AC-0 D0 checked the accepted and X-ADDS inputs on scratch
built from `15e57bf397ebbbb8b1b7186381ed6bf9b1bb6c47`. The accepted
input checked at rc 0; the X-ADDS input was rejected at its own
`law_absorb` declaration. The premise-only pair is an added oracle from
§19, not a D0 measurement. These are D0 scratch measurements, not an
execution of this seed or the official conformance runner. They do not
establish the assembled candidate's AC-T, CI, publication, landing, or
landed trust.
