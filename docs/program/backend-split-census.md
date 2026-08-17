# Backend split census

Measurement SHA: `4de48651434dd6340f81ec9b1b7a5ac2ec8c0199`

This is the durable Stage A result for `RT-BACKEND-SPLIT-CENSUS`. All five
inventories were derived from the one SHA above, after
`RT-DESCENT-RETIRE` and the three census-control repairs landed. No source
file in the measured boundary was changed while taking the census.

## Inventories

1. [Type ownership](backend-split-census-type-ownership.md) records 278
   non-private declarations, including all 199 declarations spelled
   `pub(in crate::cranelift_backend)`, their declaring owners, lexical
   mint-shape files, and crate-local reference consumers.
2. [Lifecycle, evidence, and closeout](backend-split-census-lifecycles.md)
   records the authority-bearing planner and lowering lifecycles, their
   production evidence, consumers, and terminal checks.
3. [Re-export surface](backend-split-census-reexports.md) records 57 explicit
   re-export statements independently for default library, default test,
   feature-only, and test-or-feature profiles, plus the crate-root glob edge.
4. [Test properties](backend-split-census-tests.md) records all 716 literal
   backend tests and the fixture, mutation, static observation, denominator,
   and source-oracle populations. It records the inline caveat guard as fixed
   but partial: 322 bare `cfg(test)` regions versus the 340-region rationale
   domain that also contains 18 `any(test, ...)` regions.
5. [Co-change baseline](backend-split-census-cochange.md) records the
   post-retirement four-file churn and pairwise co-change matrix over 156
   commits since 2026-07-01.

Each inventory states its lexical domain, exact selector, and blind spots.
Counts are complete only over those declared selectors; none is presented as
a semantic reachability proof.

## Primitive-lowering fail-closed gate

The call graph selected by `RT-BACKEND-PRIMITIVE-LOWERING-SPLIT` remains
acyclic at the measurement SHA and has no new shared owner. The verdict is
**proceed**.

### Moving owner

`Lowering::lower_expr` has one `RuntimeExpr::PrimitiveCall` dispatch to
`Lowering::lower_primitive_call`. Identifier-boundary search over the measured
backend finds one definition and that one caller. The dispatcher still owns
argument occurrence lookup and evaluation, partiality, carried scalar
projection, representation checks, symbol dispatch, and the final specialized
result.

The selected twelve methods are still called only from
`lower_primitive_call`:

- `lower_int_binop`;
- `lower_int_cmp`;
- `lower_bool_not`;
- `lower_bool_binop`;
- `lower_bytes_length`;
- `lower_bytes_at`;
- `lower_bytes_slice`;
- `lower_bytes_concat`;
- `lower_bytes_encode`;
- `lower_bytes_decode`;
- `lower_string_byte_length`; and
- `lower_string_char_length`.

For each name, the exact selector was an identifier-boundary `rg` over
`cranelift_backend/` and `boundary_value_clif.rs`. Each result contains one
definition and only dispatcher call sites. That selector cannot see calls
through macros, aliases, or function pointers; these private inherent methods
are not passed as values in the measured source.

The free recursive `lowered_char_list` has exactly its definition, one
self-recursive call, and one dispatcher call. The free `expect_two_args`
helper has five calls, all from the selected primitive methods. It is an
acyclic arity seam, not a second owner; the next move may keep it in parent
support or move it without changing the selected ownership boundary.

### Shared services that stay

The surrounding methods confirm why the child module remains nested below the
current core/source-machine owner:

| service | non-primitive evidence |
|---|---|
| `child_occurrence` | Many constructor, call, closure, match, and continuation paths use the same source-child authority. |
| `lower_expr` | It is the general expression dispatcher and the sole caller of the primitive dispatcher. |
| `specialized_operands_at` | Deferred constructors, closure captures, constructor templates, and lexical environments use it in addition to primitive arguments. |
| `emit_carrier_scalar` | Unit emission, boundary transfer, dynamic matches, effects, and primitive projection all call it. |
| `lower_dynamic_small_int` | Host-result/resource paths and primitive scalar projection both call it. |
| `native_int_tag` | Boundary/result emission and native-Int operations call it; it is adjacent to the primitive family but not owned by it. |
| `lower_unsigned_u64_int` | Resource and host-effect lowering call it as well as byte-length primitive paths. |

The dependency direction is therefore unchanged:
`lower_expr -> lower_primitive_call -> selected primitive helpers`, while the
primitive family calls parent-owned shared value and carrier services. No
selected helper calls back into `lower_primitive_call`, and no shared service
acquired a primitive-only owner edge. The anticipated nested child-module seam
still represents this graph without a second evaluator or a facade cycle.

### Gate selector limitation

The D6 read used identifier-boundary source references, not a Rust call-graph
engine. It is complete over direct spelled calls at the named SHA and cannot
see macro expansion, dynamic dispatch, or generated code. The selected methods
are private and have no address-taking or aliasing hits, which closes those
blind spots for this source tree without turning the lexical count into a
general mechanism claim.
