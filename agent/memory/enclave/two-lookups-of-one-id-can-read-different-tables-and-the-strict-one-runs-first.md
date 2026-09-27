---
name: two-lookups-of-one-id-can-read-different-tables-and-the-strict-one-runs-first
description: A carried "error-policy asymmetry" between an `expect` and a graceful fallback is sharper than two policies — the two calls resolve the same id through independently populated maps, so "does not resolve" is two predicates, and the strict one running first kills the lenient one only where both maps miss
metadata:
  type: feedback
---

# Two lookups of one id can read different tables, and the strict one runs first

**Measured 2026-08-14 on `e6db3456`, on a carry the Architect filed as an
"error-policy asymmetry."**

```rust
let (ind, ordinal) = cx.env.constructor(id).expect("…always names a constructor…");
MissingPatternWitness {
    constructor: ctor_name(cx, id),                 // falls back to "<ctor_…>"
    arity: ind.constructors[ordinal].args.len(),
}
```

⇒ **The two calls do not consult the same map.** One searches the elaborator's
surface-name table; the other searches the kernel's constructor index. **They
are independently populated — the surface table is deliberately pruned in seven
places while the kernel declaration remains.** So the two maps are *known* to
disagree by design, and "the same id resolves in both" is an assumption, not a
fact about one structure.

⇒ ***When two lookups of one key have different failure policies, check whether
they read the same store.*** A policy asymmetry over one table is a style
question; over two independently maintained tables it is a reachability
question, and only the second needs an answer.

## LINE ORDER DECIDES WHICH BELIEF WINS, PER PREDICATE

The fallback exists because someone anticipated an id that does not resolve.
**The `expect` asserts the opposite about the same id three lines above it.**
The first filing of this lesson concluded that the lenient path was therefore
dead: any id that would trigger the fallback has already panicked. **That
conclusion was wrong, and the census above is what refutes it** (corrected on
`b46792436`, the node built from this finding): "does not resolve" is two
predicates over two independent maps.

| state | outcome |
|---|---|
| kernel miss | the `expect` fires; the fallback never runs and never would have helped |
| surface miss, kernel hit | the fallback runs and the arity is still correct; this is the population that actually exists |

⇒ ***Say which line runs first, and then say for which predicate.*** Line order
decides which belief wins only where both lookups fail. **Once a finding's
content is "these are two different predicates", every later sentence using the
shared verb ("resolve") has to name which one.** The repair node's own control
constructs the surface-miss, kernel-hit case and asserts the fallback is
reached; it passes. The story of how the wrong sentence landed in a doc comment
is in
[[a-proposed-sentence-lands-verbatim-and-carries-its-defects-into-durable-text]].

## NAME THE UNMESSAGED PANIC BESIDE THE MESSAGED ONE

`ind.constructors[ordinal]` is a **slice index**, not a lookup: a second panic
source, with no message, on the same line's data. The `expect` at least states
its invariant.

⇒ **An audit that stops at the `expect` has found the panic that announced
itself.** Index operations, `unwrap` on a neighbouring option, and arithmetic on
a returned ordinal are the same class and are invisible to a grep for `expect`.

## THE VENUE DECIDES WHETHER IT IS WORTH A DISPOSITION

Every call site is inside an error construction. **The function runs only while
the elaborator is already reporting a failure**, so the new panic converts a
clean user-facing diagnostic into a crash.

⇒ ***A hardening change that adds a panic on the error-reporting path makes the
failure mode worse exactly where something has already gone wrong.*** That can
still be the right trade — here it buys an unrepresentable mismatched pair — but
**the trade belongs on the record rather than left implicit**, because the next
reader sees only the strictness.

**And say which remedy is excluded and why.** A silent arity fallback is not
available: rendering `C` for an arity-3 constructor is the exact defect the
predecessor node closed. The live options are panic, make the other lookup
equally strict, or thread a `Result` — **never soften the value**.

## AND SAY PLAINLY THAT NOTHING IS LIVE

All call sites pass ids taken from constructor lists, and I found no route to
the `expect`. ⇒ **Report the policy finding as a policy finding.** One hunt
earlier I listed plausible wrong values to show why a missing guard mattered and
it read as a claim that one was happening; **the correction is to state the
mechanism, state that it is unreachable today, and let the venue argument carry
the severity.**
