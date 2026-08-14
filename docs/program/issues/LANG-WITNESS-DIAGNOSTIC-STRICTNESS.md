---
id: LANG-WITNESS-DIAGNOSTIC-STRICTNESS
title: "missing_pattern_witness's two lookups read DIFFERENT tables and the strict one runs first, so ctor_name's fallback is dead code on this path -- two incompatible beliefs about one id with the stricter winning by line order -- and ind.constructors[ordinal] is a second, unmessaged panic source on the same data, all on a function that runs only while the elaborator is already reporting an error"
status: ready
owner: language
size: S
gate: none
depends_on: [LANG-REACHABILITY-SUBSUMING-ARMS]
blocks: []
github: null
origin: "Architect non-blocking carry on the LANG-WITNESS-ARITY-DERIVED merge (dec_2hprs9v6a3ds8, exact e6db3456): the new expect has an error-policy asymmetry with ctor_name and no silent arity fallback is authorized. Sharpened by the Adversary hunt evt_1fybxqm29839b on the same SHA, which measured the mechanism -- the two lookups consult different maps that are pruned independently by design -- and added the unmessaged slice index as a sibling. Steward-filed per COORDINATION §2 because a carry recorded only in an approval verdict and a PR body evaporates; the same failure forced RT-CONTKEY-REFUSAL-PROFILE-SPLIT and LANG-REACHABILITY-SUBSUMING-ARMS to be filed the same day."
---

> ## THE `depends_on` EDGE IS CONTENTION, NOT PREMISE. Added 2026-08-14.
>
> **This node's premise is `e6db3456`, which is MERGED.** Nothing it measures
> waits on [[LANG-REACHABILITY-SUBSUMING-ARMS]].
>
> The edge exists because both nodes edit
> `crates/ken-elaborator/src/elab.rs` and Language runs one node at a time.
> **It moved out of the "Not this node" prose and into the frontmatter because
> `gen-progress.sh` reads `depends_on` and cannot see prose** — this lane has
> twice produced an edge that was real in prose and absent from the generator's
> field, and both times it hid a stall.
>
> ⇒ **`ready` here means shovel-ready and sequenced**, not pullable today.

## What this is

**Not a live crash, and nobody is claiming one.** The Adversary looked for a
reachable panic and did not find one; all four call sites pass ids taken from
constructor lists, so the `expect`'s justification looks sound.

**What changed is the failure mode.** The shape on `main`:

```rust
fn missing_pattern_witness(cx: &ElabCtx, id: GlobalId) -> MissingPatternWitness {
    let (ind, ordinal) = cx.env.constructor(id).expect("…always names a constructor…");
    MissingPatternWitness {
        constructor: ctor_name(cx, id),          // same id, degrades to "<ctor_…>"
        arity: ind.constructors[ordinal].args.len(),
    }
}
```

## `H1` -- the two lookups do not consult the same map

| call | table | on miss |
|---|---|---|
| `ctor_name(cx, id)` | **`cx.globals`**, the elaborator's surface-name map | `format!("<ctor_{:?}>", id)` |
| `cx.env.constructor(id)` | **`env.ctor_index`**, the kernel's constructor index | `None` ⇒ **panic** |

**These are populated independently, and they are known to disagree by design:**
`globals` is deliberately pruned in seven places -- `prelude.rs:2498`,
`conversions.rs:361`, and five sites in `elab.rs` -- while the kernel
declaration remains.

⇒ **The direction that actually happens is the safe one:** a de-registered
constructor resolves in `ctor_index` and falls back in `ctor_name`, so the arity
is right and the name renders `<ctor_…>`. **The direction that panics is an id
in nobody's constructor index** -- `env.constructor` returns `None` for a
non-inductive declaration through its `_ => None` arm.

## `H2` -- the fallback is now unreachable, so its author's intent is defeated by position

`ctor_name`'s `unwrap_or_else` exists **because someone anticipated an id that
does not resolve.** The `expect` asserts the opposite about **the same id**,
three lines above it. Whatever id would have triggered that fallback **has
already panicked.**

⇒ **The precise statement of the Architect's asymmetry is not "two policies
exist" but "the lenient policy can no longer fire where it was written to
fire."** Two incompatible beliefs about one value, with the stricter winning by
line order.

## `H3` -- a second panic source, and it says nothing at all

`ind.constructors[ordinal]` is a **slice index, not a lookup.** `ordinal` comes
from `ctor_index`; if that index and `ind.constructors` ever disagreed in length
this is an out-of-bounds panic **with no message**. The `expect` at least states
its invariant; this states nothing, on the same line's data.

## `H4` -- the venue is what makes this worth a disposition

**This function runs only while the elaborator is already reporting an error** --
every call site is inside an `ExhaustivenessError` construction. So the failure
introduced converts a clean user-facing *"non-exhaustive match"* diagnostic into
a panic.

⇒ **A hardening change made the failure mode strictly worse on the one path
whose job is to report failures**, in exchange for making a mismatched
`(constructor, arity)` pair unrepresentable. **That is a good trade and it is the
merged node's whole point** -- but it should be on the record explicitly rather
than implicit, which is most of why this node exists.

## The remedy space is BOUNDED, and one option is excluded by ruling

**Architect: no silent arity fallback is authorized.** The reason is the
predecessor's: an arity fallback of `0` renders `C` for an arity-3 constructor,
**which is exactly the defect `LANG-EXHAUSTIVENESS-WITNESS-PAYLOAD` closed.**
Reintroducing it would undo the node this one is filed against.

**The admissible remedies:**

1. **Keep the panic** and make `ctor_name` **equally strict** on this path, so
   the two lines encode one belief.
2. **Thread a `Result`** through the diagnostic path so neither lookup panics.
3. **Keep the panic and document the invariant**, and give `H3` the same
   treatment -- `.get(ordinal).expect(…)` with its own sentence, or a note that
   `ctor_index`'s ordinal is a `constructors` index by construction.

**Choosing among these is a design call.** If the choice is not obvious from the
code, **route it to the Architect rather than picking one** -- he registered the
asymmetry and did not prescribe the fix.

## Acceptance criteria

**`AC-1` -- after the change, the two lookups encode ONE belief about the id**,
whichever remedy is chosen. A reviewer must be able to say in one sentence what
happens when the id does not resolve.

**`AC-2` -- `H3` is dispositioned, not left.** Either it gains a message or it
gains a written reason why it cannot fire. **Silence is the one outcome this
node exists to remove.**

**`AC-3` -- no silent arity fallback.** Ruled, not negotiable.

**`AC-4` -- the diagnostic output is unchanged for every currently-passing
case.** This node changes failure behaviour, never the rendered witness. The
predecessor's controls stay green on the same derivation.

**`AC-5` -- no-regression, in CI.** `COORDINATION §12`; build and test targeted,
`-p ken-elaborator`.

## Sizing

**`S`.** One function, two lookups, one slice index. **If remedy 2 (`Result`
threading) is selected, stop and report before building it** -- that crosses the
diagnostic path's signature and is a different size, which is a re-cut rather
than an overrun.

## Not this node

- **Not a reopening of [[LANG-WITNESS-ARITY-DERIVED]].** That merge stands, its
  trade is endorsed above, and no value it produces is known to be wrong.
- **Not [[LANG-REACHABILITY-SUBSUMING-ARMS]]**, which is the **reachability**
  payload at `elab.rs:1737`/`:2427`/`:8446`. This is the **exhaustiveness** path.
  Same file, different sites. **Sequence this after it**, not concurrently.
- **Not a general `ElabError` panic audit.** Four call sites, one function.
