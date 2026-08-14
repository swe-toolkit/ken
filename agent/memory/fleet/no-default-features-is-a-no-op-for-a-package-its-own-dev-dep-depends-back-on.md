---
name: no-default-features-is-a-no-op-for-a-package-its-own-dev-dep-depends-back-on
description: "`--no-default-features` cannot suppress a package's own defaults when one of that package's `[dev-dependencies]` depends back on it with default features -- the cycle re-activates them in every build that includes dev-deps, i.e. every `cargo test`. So putting a feature in `default` to keep it on in the ordinary run is the SAME KNOB as making its off-configuration unreachable."
scope: fleet
---

# `--no-default-features` is a no-op for a package its own dev-dep depends back on

**The two properties are one knob, in opposite directions.** If you put a
feature in a package's `default` so it stays on in the ordinary targeted test
run, and that package sits in a dev-dependency cycle, you have simultaneously
made its **off-configuration unreachable from the command line**. No wording of
the feature separates them.

## The mechanism

Cargo **unions** feature activations across the graph. `--no-default-features`
suppresses the defaults *the command line asks for*; it does not remove an
activation that arrives over an edge. So:

```
ken-elaborator [dev-dependencies] -> ken-interp
ken-interp     [dependencies]     -> ken-elaborator   # default features, implicitly
```

⇒ building `ken-elaborator`'s **test** targets pulls `ken-interp`, which
re-activates `ken-elaborator/default`. The flag is honoured and then overridden
by the cycle. Dev-deps are in the graph for every `cargo test`, so this is not
an edge case — it is the normal test build.

## MEASURED / CLAIMED / THE GAP

Measured 2026-08-14 at `ca803dfc`, `RT-C2-OBSERVATION-ARTIFACT-IDENTITY`.

- **MEASURED:** with the carrier in `default`, a package-scoped
  `--no-default-features` build still compiled the gated code. Confirmed by
  `cargo tree -p <pkg> --no-default-features -e features -i <dep>`.
- **CLAIMED (by the Steward, in an AC):** the package has `default = []`, so
  adding a default-on carrier turns `--no-default-features` into "exactly and
  only that feature off".
- **THE GAP:** the premise (`default = []`) was true and the inference was
  false. `default = []` says nothing about whether `default` is *suppressible*.
  A whole WP increment was framed on it.

## Why the sibling feature looked copyable and was not

The same crate carried a second observation feature whose nested-cargo A/B
worked. It works because it is **not in `default`** — so the cycle
re-activating `default` re-activates nothing, and its off-configuration is the
natural state rather than something the flag has to win. **Reading the two
features side by side shows no difference**; the difference is one line away, in
whether `default` names it.

## How to apply

- **Before designing any control around `--no-default-features`, check for a
  dev-dep cycle**: does anything in the package's `[dev-dependencies]` depend
  back on the package? One `cargo tree ... -e features -i <dep>` settles it, and
  it is much cheaper than the increment it saves.
- **Never infer suppressibility from `default = []`.** They are unrelated facts.
- **When a feature must be ON in the ordinary run AND have a reachable
  off-configuration, host the A/B in a different package** — one nothing
  dev-depends back onto. Then the feature need not be default-on anywhere and
  the carrier disappears from the design entirely.
- **Check the candidate host can carry the input** before naming it. Here the
  obvious host (the crate owning the feature) could not: emitting the artifact
  required elaborating a source, and that crate sits *below* the elaborator in
  the dependency direction.
- **An in-tree comment asserting "this is not circular" may mean linking.**
  `ken-interp/Cargo.toml:15-17` says exactly that, and it is true for linking
  and false for feature resolution. Both readings are correct about different
  things.

## The anti-vacuity guard is what made this cheap

The frame required a worker assertion that the nested build **actually compiled
the requested configuration**, checked *before* the artifact comparison. It
fired (`left: true, right: false`) and cost one increment. Without it the A/B
would have compared two identical feature-on artifacts, gone green, and shipped
as a control that had never once discriminated.

Related: [[a-p-scoped-run-and-cis-workspace-run-compile-different-feature-sets]]
(the sibling trap — union across *different* packages, rather than a package's
own defaults returning through a cycle), and
[[an-artifact-level-ab-must-not-share-a-cargo-target-directory]].
