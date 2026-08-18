---
id: RT-C2-DRIVER-STAGE-ATTRIBUTION
title: "The D5 observation identity driver reports every non-zero nested exit as `nested {} compilation failed`, so the one message AC-2 itself produces names the wrong stage -- plus one clause recording why the compiled-feature const must stay adjacent to the gate it mirrors"
status: merged
owner: runtime
size: XS
gate: none
depends_on: []
blocks: []
github: null
origin: "Adversary hunt evt_6phjvzckqw52q on squash 1200edf0, triaged by the Steward and accepted. The hunt re-ran AC-2's own mutation rather than reading it; the driver message is what firing the control produced. Both sites re-checked against main 99869bb7 before framing: crates/ken-cli/tests/dasm_c2_observation_artifact_identity.rs:141-142 and crates/ken-runtime/src/cranelift_backend.rs:104-112."
---

## What this is

**Two clauses in two files.** Neither changes behaviour; both change what a
later reader is told. Sized `XS` deliberately.

**This node needs no ring turn of its own.** Ride it with the next candidate
that touches `ken-runtime` or `ken-cli`. If none arrives before Runtime's
current node closes, it is a single short turn.

## `D1` — the driver blames the wrong stage on the path it exists to exercise

`crates/ken-cli/tests/dasm_c2_observation_artifact_identity.rs:141-142`:

```rust
assert!(
    output.status.success(),
    "nested {} compilation failed:\nstdout:\n{}\nstderr:\n{}",
```

The command it guards is a **`cargo test` run**, not a build. So a **test
failure inside the nested run** — which is precisely how this control fires
when it catches the defect it was written for — surfaces to the author as
*"compilation failed"*. Nothing failed to compile.

**Measured, not inferred.** The Adversary re-ran `AC-2`'s own mutation (adding
`dasm-c2-observation` to `ken-cli/Cargo.toml:28`'s dev-dependency feature
list). The control reddened correctly with *"ken-runtime and ken-cli disagree
on the D5 observation configuration"* at `:60` — and that true cause reached
the author **embedded inside a headline naming the wrong stage**.

⇒ **Restate the message so it names the run, not the build** — *"nested
feature-off run failed"*. The real cause is already in the embedded stdout; the
defect is that a reader who stops at the first line goes looking at build
configuration.

**Same class as the bracket-message repair in
[[LANG-REFINED-FALLBACK-COLDNESS-CLAIM]] `D5`**, and worth naming as such: a
diagnostic that is accurate about a failure it was not written for.

## `D2` — record WHY the const sits where it sits

`crates/ken-runtime/src/cranelift_backend.rs:110` is
`DASM_C2_OBSERVATION_COMPILED`; `:112` is the `#[cfg(feature = ...)]`
re-export it mirrors. **Adjacency is the mechanism, and nothing says so.**

**This closes an attack rather than opening one, and the reasoning is the
deliverable.** The obvious concern about the landed fix is that it creates a
second pair of gates that must stay in step — the same shape as the original
defect. **It does not**, and the census says why:

| family | sites |
|---|---|
| `feature` only | `cranelift_backend.rs:110` (the const), `:112` (the re-export), `mod.rs:16004/16011/16031/16040` |
| `any(test, feature)` | `mod.rs:15971/15981/15990/15997`, `:17768`, `:17844` |

**One feature resolution per `ken-runtime` compilation feeds both the `cfg!` and
the `#[cfg]`.** They cannot disagree by feature-graph drift; desynchronizing
them requires editing one of two adjacent predicates in one file.

**The original defect had exactly the route this pair lacks:** its two gates sat
in *different crates*, coupled only through Cargo resolution, so a third crate's
dev-dependency desynchronized them **with no file changing.** That is what made
it silent.

⇒ **One clause at `:104-109` saying the const is deliberately adjacent to the
gate it mirrors, and that moving it somewhere more logical is what would
reintroduce the drift route.** The risk this guards against is a later reader
tidying the const into a more natural home.

## Recorded and NOT a deliverable — the name/doc width nit

`DASM_C2_OBSERVATION_COMPILED`'s **doc** is precise (*"includes the D5
observation entry point"*); its **name** is wider than the fact, because in
`ken-runtime`'s own test builds the `any(test, feature)` family is compiled
while the const reads `false`.

**Ruled benign, and the reason is recorded here so the shape is not re-filed.**
The facade is `feature`-only, so an in-crate reader seeing `false` could not
have used the scope anyway; and for the cross-crate control `ken-runtime` is a
dependency, where `cfg(test)` is false. **The const is correct for everything
anyone can actually do with it.** Do not rename it.

## Acceptance criteria

**`AC-1` — `D1` is verified by FIRING the control, not by reading it.** Apply
`AC-2`'s dev-dependency mutation, confirm the headline now names the run rather
than the build, **report the verbatim text**, and restore. The message is only
observable on the failing path, which is why reading it does not discharge this.

**`AC-2` — the embedded cause survives the rewording.** The real assertion text
must still reach the author. A tidier headline that drops the stdout is worse
than the misattribution it replaces.

**`AC-3` — `D2` changes a comment only.** No change to the const, the
re-export, the facade's gating, or any `cfg` predicate. If the clause appears
to require a code change, **stop and report** — that would mean the adjacency
argument is not what this node says it is.

**`AC-4` — no-regression, in CI.** `COORDINATION §12` — the venue is CI, never
a local `--workspace` run. Build targeted.

## Not this node

- **Not a rename of `DASM_C2_OBSERVATION_COMPILED`.** See the nit above.
- **Not a change to the `any(test, feature)` family.** The split between the
  two families is an Architect requirement with a stated reason: mirroring
  `any(test, feature)` would read `true` in every test build and reintroduce the
  vacuous pass inside the fix.
- **Not a general sweep of `assert!` messages** in the test corpus. This is the
  one message `AC-2` itself produces.
