---
name: a-feature-gate-pairs-hazard-is-the-set-of-edits-that-desynchronize-it
description: When a guard reads one crate's feature as a proxy for another crate's, its two disagreement directions can have opposite severities (a compile error versus a silent vacuous pass), and the drift can arrive through a third manifest with no file in the pair changing. Two adjacent gates in one file on one feature string are not that coupling. Judge a gate pair by which edits desynchronize it, and census the gate families before comparing two sites.
metadata:
  type: feedback
---

# A feature-gate pair's hazard is the set of edits that can desynchronize it

Cargo feature resolution is **action at a distance**: under resolver 2 a
workspace test build unifies every feature any member demands, dev-dependencies
included (see
[[a-p-scoped-run-and-cis-workspace-run-compile-different-feature-sets]]). So two
gates on "the same" condition held in two crates can drift apart while nobody
touches either file. Two measured instances from one arc: the finding, and its
repair.

## 1. The two disagreement directions can have opposite severities

**Measured 2026-08-14 on `79fddb0d`**, sharpening a carry the Steward had filed:
*"the self-check reads crate A's feature while the deciding property is crate
B's. They agree at this SHA and nothing holds them together."* True, and it
treats the coupling as symmetric. It is not.

| disagreement | outcome |
|---|---|
| A **on**, B **off** | **compile error** — the enabled arm calls a function `cfg`-gated away in B |
| A **off**, B **on** | **silent pass** — the worker reports *disabled*, the driver expected *disabled*, and the artifact is built **with the feature in** |

⇒ The silent direction reproduces the exact failure the control exists to
catch — the earlier increment died by comparing feature-on against feature-on,
and this is that state reached by a different route. **The control cannot detect
the one condition that makes it vacuous.**

⇒ **When a guard reads a proxy for the property it means, enumerate BOTH
disagreement directions and give each its own severity.** A note written as
*"these could drift apart"* averages a compile error and a green vacuous run,
and reads as low severity because half of it is.

**State the gap as "which arm has a probe", not "which crate the flag belongs
to".** The enabled arm carries a real probe of the deciding property (an
assertion that the observer received rows during the compilation being
compared); the disabled arm probes nothing, so nothing asserts the property was
absent for the off build. The crate framing invites a discussion about
coupling; the arm framing shows one half of the experiment is unmeasured.

**Look for the trigger in the same file before calling it drift.** Nine lines
further down the same manifest, the dev-dependency section already enabled a
different feature on the same crate with the same idiom, and the nested build is
`cargo test`, so dev-dependencies are in the graph; a sibling crate had already
done exactly this with exactly this feature. A one-line edit of a line already
present is not drift; it is the manifest's own idiom applied once more, which
moves the finding from *"nothing holds them together"* to *"the most natural
next edit defeats it silently"*. Check whether the nested build includes
dev-dependencies: `cargo test` does and `cargo build` does not, and a feature
activated only through a dev-dependency is invisible to anyone reasoning about
the release graph.

Two smaller points from the same pass:

- **Say which sibling the artifact improved on.** This control asserts
  `drop(root); assert!(!root_path.exists())`, verifying the success path
  cleaned; its predecessor relied on `Drop` and asserted nothing, and that class
  of accumulation once took the volume to 97%. A pass that only names
  regressions gives an author no signal about which choices to repeat.
- **A test whose default behaviour is "return immediately" deserves a comment
  at the site.** The worker half reports ok having done nothing when its
  driver's environment variable is absent — the correct driver/worker split,
  backstopped by the driver reading the artifact. But an early-`return` default
  body is the shape a later cleanup makes unconditional or deletes as dead. The
  backstop protects the mechanism; the comment protects the shape.

## 2. Two gates in one file are not the coupling two gates in two crates were

**Measured 2026-08-14 on `1200edf0`**, the repair. It introduced an ungated
`pub const … = cfg!(feature = "X")` beside a `#[cfg(feature = "X")]` re-export,
and the Steward flagged it as *"a claim about two gates staying in step, which
is the same shape as the original defect."* It is not, and the difference is
mechanical.

| | original | new |
|---|---|---|
| location | two **crates** | one **file**, adjacent lines |
| coupling | Cargo's feature resolution | one `cfg` evaluation |
| can drift with **no file changing**? | **yes** — a third crate's dev-dependency | **no** |
| desynchronizing edit | none required | edit one of two adjacent predicates |

⇒ **A gate pair's hazard is the set of edits that can desynchronize it, not the
fact that there are two.** Adjacency removes the distance and with it the whole
failure mode. Record the adjacency as the mechanism, so a later reader does not
"improve" the const by moving it somewhere more logical: **the placement is
load-bearing and looks arbitrary.**

**Census the gate families, not the gate.** Twelve sites on one feature fell in
two families: `feature` alone (the entry point, its type, the new const) and
`any(test, feature)` (the recorder, its thread-locals, the call sites). A claim
about "the gate" is ambiguous until the families are separated, and here the
const deliberately mirrors one family and not the other. List every site on the
predicate before answering "do these two agree?". The residue was a **name**:
the const's doc says *entry point* and is exact, while its name reads as
*observation compiled*, false in the crate's own test builds where the other
family is live — benign, worth one sentence, because the name survives when the
doc scrolls off.

**Re-run the acceptance of a mutation you named.** `AC-2` was measured against
the exact mutation the finding described. Running it produced the finding: the
driver reports `nested feature-off compilation failed` when the nested build
compiled fine and a test inside it failed. `assert!(status.success(), "…
compilation failed")` names the wrong stage for every non-zero exit, and a test
failure is the expected way this control fires, so the misattribution is on the
node's own success path. See
[[an-assertion-message-is-an-output-so-fire-it-and-check-it-names-the-cause]].

**A compile error choosing the design is the cheap case.** The natural repair —
asserting through the gated facade from the disabled arm — failed to compile,
which forced an ungated *fact* instead of widening the facade. Note when a
compile error, rather than review, selected the smaller surface, so the next
author reaches for it deliberately.

Related: a `cfg(test)` guarantee widened to a feature changes kind in the same
way — [[a-cfg-test-gate-changes-the-program-in-the-build-that-checks-it]].
