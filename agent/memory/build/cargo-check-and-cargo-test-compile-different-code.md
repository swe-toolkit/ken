---
scope: build
audience: (see scope README)
source: private memory `cargo-check-reports-zero-while-cfg-test-sites-remain`,
  `cfg-test-build-asymmetry-is-bidirectional`,
  `a-cfg-test-reexport-is-a-production-only-red-the-test-profile-cannot-see`
  (R4 triage, 2026-09-26)
---

# `cargo check`/`build` and `cargo test` compile different code, and either can be green while the other is red

`cargo check` and `cargo build` never compile `#[cfg(test)]` modules or
`#[cfg(test)]`-gated items — the test target is not part of that build. In a
repo where tests live inside the production file (via `use super::*` or a
`mod tests` in the same file), that is exactly the population a migration or
a refactor most needs to reach, and it is invisible to `check`/`build` by
construction. The gap runs in both directions, so neither profile alone is
the oracle.

**Production green, test red.** A crate-wide type migration
(`ken-runtime` lowering, `RT-WORKER-BIND` D1, 2026-08-02) reached
`scripts/ken-cargo check -p ken-runtime` exit 0 with zero errors while 15
further migration sites remained in `cfg(test)` fixtures and `matches!`
assertions — visible only once `scripts/ken-cargo test -p ken-runtime` ran.
The compiler is the completeness oracle for a compiler-guided migration, so
"the compiler reports zero" reads as "the migration is complete"; it is a
true statement about a narrower question than the one being asked.

**Test green, production red — the inverse.** An item gated with
`#[cfg(test)]` but re-exported through an ungated `pub use` compiles under
`--profile test` (the item exists there, so the import resolves) and fails
production with `E0432 unresolved import` (the item is gone). Measured on
`RT-CONTSRC-PRODUCER-LOCAL` D3b, 2026-08-05: `ken-cargo check -p ken-runtime
--profile test` reported 0 errors with the full lib suite green at 728
passed, while `ken-cargo check -p ken-runtime` (no `--profile test`) reported
the unresolved import. Nothing in the normal test-iteration loop could see
it, because the profile that would catch it is the one you stop running once
you're iterating on tests.

**A build-vs-test warning can also mislead.** `cargo build`'s unused-import
warnings are specifically unreliable here: an import used only under
`cfg(test)` reads as unused to the non-test build, and removing it breaks the
test build (one instance produced 33 `E0433`/`E0425` errors). Never act on an
unused-import warning without deleting the import and re-running the test
build to confirm.

This is a sibling of
[[a-p-scoped-run-and-cis-workspace-run-compile-different-feature-sets]]: that
lesson is about a `-p`-scoped run compiling a different *feature set* than
CI's `--workspace` run; this one is about `check`/`build` compiling a
different *code population* than `test` within the same scoped run. Both are
instances of "a partial build is a partial view, and treating either as the
whole build ships the half nobody looked at."

## How to apply

- Run both **`scripts/ken-cargo build -p <crate>`** and **`scripts/ken-cargo
  test -p <crate>`** before claiming a crate is green. Neither alone is a
  gate, and both stay scoped to one crate per the no-`--workspace` rule
  (COORDINATION §12) — CI runs the workspace build.
- When using the compiler as a completeness oracle for a migration, the
  green that ends the pass is `ken-cargo test -p <crate>`, not `ken-cargo
  check -p <crate>` — `check` is fine for fast worklist iteration while the
  count is falling.
- On any change that adds or moves a re-export near `#[cfg(test)]` items,
  check the two profiles separately: `ken-cargo check -p <crate>` and
  `ken-cargo check -p <crate> --profile test`. Gate the re-export with the
  same `#[cfg(test)]` as the item it re-exports, in its own `use` block
  rather than mixed into a production one.
- Never act on an unused-import warning from a plain `build`/`check` without
  re-running the test build afterward.
- Treat a disagreement between the two profiles as the finding, not as a
  contradiction to explain away.
