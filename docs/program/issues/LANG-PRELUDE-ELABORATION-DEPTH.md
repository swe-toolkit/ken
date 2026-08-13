---
id: LANG-PRELUDE-ELABORATION-DEPTH
title: "Elaboration has an unstated stack requirement that exceeds Rust's 2 MiB spawned-thread default: every compilation elaborates the whole prelude, `elab.rs:997` measures ~115 KiB of headroom out of 2 MiB, and thirteen sites across four crates independently bumped their thread to 256 MiB without any stated rule -- so the rest of `37 §9` is a queue of prelude additions spending a margin nobody measures and no site states"
status: ready
owner: language
size: S
gate: none
depends_on: [LANG-PRELUDE-COLLECTIONS]
blocks: []
github: null
origin: "PR #2144 CI failure (run 31752261297, job 94620375276): `LANG-PRELUDE-COLLECTIONS` added four recursive prelude declarations and a nested-compile worker died with `fatal runtime error: stack overflow` on a source that calls none of them. Architect correction at evt_54y1jadrfk9eq established the mechanism is elaboration depth at prelude registration, not the runtime recursion the Steward first hypothesised. Steward census at c1b9a1e8 established there is no production stack_size site anywhere in crates/."
---

## READY as of 2026-08-13: the flip condition was met, and the result strengthens the node

Filed `draft` pending the `#2144` measurement. **That measurement landed and
flipped this node to `ready`.** `LANG-PRELUDE-COLLECTIONS` merged at
`60b78c95` (PR #2144, superseding tip `52ffffbe`), and the discriminator was run
both ways:

| configuration | result |
|---|---|
| unmodified worker, at the candidate | **red** |
| unmodified worker, at `main` | **green** |
| candidate plus the 256 MiB worker repair | **green** |

**Read the middle row.** The failure is attributable to the candidate, not to a
test fragile on any base — and *candidate red, `main` green* means **`main` was
already at the edge.** That worker was one prelude declaration away from
failing on **any** future addition, whatever its shape. Architect at
`evt_649r77rhx56vj`, and it is measured rather than inferred.

⇒ **D3 is the live question, not the optional one.** The alarm is being switched
off, and we now know exactly how close it was when it last rang.

**The node never depended on CI staying red.** A green CI says the worker's
thread was too small. It says nothing about how much margin the prelude has
left, which is the whole question here.

## What this is

Every compilation registers the prelude, so **every prelude declaration is
elaborated on every compile of every program, including programs that reference
none of them.** `elab.rs:997` carries a measured frame budget from
`LANG-RECORD-STACK-OVERFLOW` (`b4d38b8a`):

> *"A wide, fixed-depth recursion elsewhere (`register_decimal_char`'s 31-level
> match cascade, unrelated to any arm here) sits close to the guard page as a
> result: ~115 KiB of headroom out of a 2 MiB thread stack remained at the
> deepest call after that node's repair -- cleared by inches, not a mile."*

It also records **why the margin erodes without anyone touching the deep path**:
in an unoptimized build a new arm's locals in `check` are paid by every call
regardless of which arm runs.

`LANG-PRELUDE-COLLECTIONS` then added four recursive declarations, two of them
(`zip`, `filter`) with nested matches, and a nested-compile worker overflowed on
a four-line source that calls none of them.

## What is established, and what is only expected

Kept separate deliberately: the first list is the frame's fixed input, the
second is what this node exists to convert into measurement.

**Established at `c1b9a1e8`:**

- `elab.rs:997`, the ~115 KiB-of-2 MiB figure and the every-call locals cost.
- The failing object is a **debug** nested `cargo test` worker
  (`.../debug/deps/...`, `r3_c2_source_mixed_branch.rs:501-505` compiles a
  four-line source with no list and no combinator call) on a default 2 MiB
  harness thread.
- **Thirteen `stack_size` sites across `crates/`, and none of them is
  production.** Six in `ken-cli/tests`, three in `ken-elaborator/tests`, one in
  `ken-verify/tests`, two in `ken-runtime` under `#[test]`
  (`static_transition.rs:23720` is inside the `#[test]` beginning at `:23582`),
  one in `ken-runtime/tests`. Nine of the thirteen use 256 MiB.

**Expected but unmeasured — this is the gap:**

- That the real driver's compile has meaningfully more headroom than the failing
  worker, whether from an 8 MiB main thread, an optimized build, or both. **No
  production `stack_size` exists**, so nothing in the tree asserts it and nobody
  has taken the measurement.
- Whether the cost is **shape-dependent** (nested matches like `zip`/`filter`
  are what tip it) or **cumulative** (any four additions would have). The
  Architect's probe at `evt_54y1jadrfk9eq` separates these: add the four
  declarations one at a time and see whether the tipping one is the deepest or
  merely the fourth.

## The thirteen sites are fossils, not a convention

Architect at `evt_3tp62zf6c7q6d`, and it is the sharpest reading of the census:

**Ken does not choose the stack its compilations run on. It inherits it from
whoever calls in.** `ken-cli`'s main thread today; any thread a caller spawns
otherwise. **Rust's default for a spawned thread is 2 MiB — exactly what the
`r3_4b` worker had when it aborted.** So the failing configuration is not an
exotic harness artifact. It is what any caller gets by invoking
`ken-elaborator` from a normally-spawned thread.

⇒ **Nobody chose 256 MiB as policy.** Thirteen separate places discovered
independently that a full compile does not fit and bumped it, and three crates
converged on the same number with no stated rule anywhere. That is not a
convention being followed; it is **thirteen rediscoveries of a requirement the
code cannot express.** The `r3_4b` worker is not the outlier because it is
wrong — it is the one site that had not yet hit the wall.

**So the deliverable is a stated minimum, not only a headroom number.** A
headroom figure answers the question for one caller and leaves the requirement
implicit, which is what produced thirteen fossils. A stated minimum — *elaboration
requires at least N of stack; a caller on a spawned thread must provide it* —
measured on the product path and recorded where the next site's author will
read it, makes the fourteenth site a compliance detail instead of a fourteenth
rediscovery.

**Scoped to the present cost, deliberately.** The Architect framed this partly as
an interface contract for a future embedder. Ken has no users and no embedders,
and publishing an external contract for an absent consumer is the kind of
ceremony this program declines. **The grounded cost is internal and already
paid thirteen times.** State the number where this repo's next author finds it;
if an embedder ever exists, the number is already measured and publishing it is
a documentation act, not a new measurement.

## The fix removes the only thing that noticed

Architect, same event, and it is the argument for D3. The `r3_4b` worker
overflowed because it was the one full-compile site still running on a default
thread — **which is exactly what made it a canary.** Raising it to 256 MiB is
the right fix and it also retires the only site that would have told us the
margin was gone. After the fix, **nothing watches the elaboration budget on the
test side and nothing states it on the product side**, and the next prelude
addition lands with neither.

That is the gap D3 closes, and it is why D3 is contingent on the measurement
rather than optional: if the margin turns out to be wide, no watch is needed; if
it is inches, retiring the canary without replacing it is a regression in what
we can see.

## Why this is a node and not a rider

**The constraint is grounded in a measured number in the tree**, not in a
preference for a tidier margin: `elab.rs:997` is an existing comment recording an
existing repair, and `#2144` is it being spent.

**It binds required future work.** `spec/30-surface/37-strings-collections.md
§9` still owes `Array`/`Map`/`Set` (abstract types, `§1`), the lawful
`DecEq`/`Ord` instances, and the combinator laws as propositions. Every one is a
prelude addition, and on the current mechanism each pays on every compilation of
every program. **If the margin is thin, the constraint is global and applies to
additions that are not recursive at all** — which is a different and much wider
rule than "watch recursive definitions."

**It is not folded into `LANG-PRELUDE-COLLECTIONS`** because that node is
approved at `a977445c` and held only on CI. Growing an approved candidate to
carry a question it merely surfaced is what makes a Decision stale for a reason
unrelated to what was reviewed.

## Not this node

- **Removing or reshaping any of the four combinators.** They are the spec's
  required floor (`37 §9`), the Architect verified their definitions sound, and
  none of them executes in the failing case.
- **`RUST_MIN_STACK` or any global thread-stack change.** It is invisible and
  changes every thread in the process; the Architect refused it once already
  today and conceded at `evt_26jk8jqrgxb13` that a local explicit change to one
  named worker is a different object. Keep that distinction; do not relitigate.
- **Fixing the `r3_4b` worker.** That belongs to `LANG-PRELUDE-COLLECTIONS`'s
  superseding tip, not here.
- **Adding `Array`, `Map`/`Set`, `DecEq`/`Ord`, or the laws.** This node measures
  what they will cost; it does not deliver them.
