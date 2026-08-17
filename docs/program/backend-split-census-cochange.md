# Backend split census: co-change baseline

Measurement SHA: `4de48651434dd6340f81ec9b1b7a5ac2ec8c0199`

This is the post-retirement four-file churn matrix required by
`research/compiler-refactoring-program.md` section 1.3.

## Boundary and selector

The commit domain is the ancestry of the measurement SHA from 2026-07-01
through that SHA, inclusive. The exact command family is:

```text
MEASUREMENT_SHA=<the SHA stated above>
git log "$MEASUREMENT_SHA" \
  --since=2026-07-01 --format='COMMIT %H' --numstat -- <four paths>
```

The parser counts one touch per distinct commit/path, sums numeric insertion
and deletion columns, and counts pairwise intersections of the resulting
commit sets. It cannot see work discarded before commit, semantic coupling
between commits that touched only one file, or history outside the named date
and ancestry. Binary `-` numstat cells would contribute zero; none of these
four Rust files produced one.

The range contains 156 distinct commits touching at least one of the four
paths.

## Per-file churn

| file | commits | insertions | deletions | net | lines at measurement SHA |
|---|---:|---:|---:|---:|---:|
| `planning/static_transition.rs` | 64 | 46,100 | 11,265 | 34,835 | 34,835 |
| `lowering/mod.rs` | 61 | 26,695 | 5,051 | 21,644 | 21,644 |
| `lowering/core.rs` | 79 | 26,180 | 5,807 | 20,373 | 20,373 |
| `lowering/core/tests/control.rs` | 107 | 39,399 | 5,426 | 33,973 | 33,973 |

## Pairwise co-change commits

| | static transition | lowering support | lowering core | control tests |
|---|---:|---:|---:|---:|
| static transition | 64 | 30 | 28 | 35 |
| lowering support | 30 | 61 | 47 | 49 |
| lowering core | 28 | 47 | 79 | 57 |
| control tests | 35 | 49 | 57 | 107 |

The diagonal is the per-file touch count. Off-diagonal cells are symmetric
distinct-commit intersections, not percentages and not causal edges.
