---
name: a-stack-budget-is-prefix-plus-depth-times-frame-and-must-agree-with-every-other-bound-on-the-recursion
description: A recursion's native stack cost is the non-recursive caller prefix plus depth times the per-level frame, and it must fit a physical limit that a fuel/depth cap does not know about. Each review measures one term and misses another -- the caller frame nobody suspects, the iterator-adapter frames a combinator adds per level, and the stack limit sitting below the fuel valve.
metadata:
  type: feedback
---

# A stack budget is prefix + depth x frame, and must agree with every other bound on the recursion

    stack used = caller prefix + depth x per-level frame <= physical limit
    (appears once) (the recursion) (nobody writes it down)

Every stack-budget arc below measured one term correctly and never measured
another. The unmeasured term is where the finding was each time. Before
reasoning about stack margin, name all four quantities, say which you measured,
and say which you assumed.

Related:
[[exact-and-single-threaded-do-not-give-a-test-more-stack-libtest-always-spawns]]
(headroom does not change under isolation; the repair for a depth regression is
a per-level frame reduction, not a provision) and
[[a-total-and-an-increment-are-two-quantities-and-the-ratio-between-them-is-not-a-measurement]]
(do not divide a descent total by a per-frame delta). The `stated-stacks` skill
governs tests that provision their stack.

## Term 1: the limit, and the other number that also bounds the recursion

**Measured 2026-08-15 on `7512b1e8b...40ed5d6e9`.** A depth-56 abort was
reclassified from mechanism defect to **harness limit**: under a 256 MiB test
thread, depths 56 through 1024 pass.

```rust
search(&root, &mut next_param, 200)     // fuel: ~200 levels permitted
if fuel == 0 { return None; }           // the DESIGNED refusal, already correct
```

(`crates/ken-elaborator/src/fo_kripke.rs`.) The measured default-stack limit
was ~56, and the crate's `src/` contained no `stack_size`, `stacker` or `grow(`
-- the big-stack wrapper was test-only.

So the valve exists and is set 3.5x above the physical limit. Between 56 and
200 the outcome is an **abort**, not the refusal the route's design guarantees.
**The repair is a smaller number, not a bigger stack**: no new mechanism, and
the fuel constant is already the right kind of thing.

**When a resource bound (fuel, step or depth cap) and the physical stack both
bound the same recursion, they are two numbers that must agree, and usually
nothing relates them.** Find the other number. A fuel cap is easy to read as
"the" bound and to check on its own terms; the stack is the one nobody writes
down.

### A reclassification can be correct and still move a fact out of view

"The harness stack caused the abort" was true. The reclassification was right
about the TEST and silent about production. **Ask what the reclassified finding
was ALSO evidence of**: here, that the recursion is stack-bound at a depth the
caller's own budget permits -- a production property the test's 256 MiB wrapper
conceals rather than contradicts.

Grant the half that is right, loudly. The paired retraction ("an abort would
take down CI's whole suite" -- false, the runner isolates per process) was
correct and independent. Separating the sound half from the residue is what
keeps a reclassification challenge from reading as a refusal to accept an
answer. (For a reclassification that adds a cause claim, see
[[reclassifying-a-cause-neutral-refusal-asserts-a-cause-you-must-know]].)

## Term 2: the caller prefix, which does not repeat and did not change

**Measured 2026-08-14 on `294fceac`**, attacking a diff sitting on a stack
budget that had just been re-cut. The arc's numbers were all about the
recursion: `check` at 14,744 bytes/call, `check_match_dependent` at 33,352 ->
30,232, the 31-level cascade at ~1010 -> ~915 KiB, and a bisected post-fix
margin of ~96 KiB. One `objdump` of the prologue of the non-recursive function
that *calls* the cascade:

```
49 81 eb 00 10 03 00    sub $0x31000,%r11      <- 200,704 bytes ~ 196 KiB
```

One caller's single frame was more than double the entire margin the repair
bought, and it is live for the whole cascade because it invokes it.

**A stack budget is the sum over the whole chain, and a recursion analysis
enumerates only the repeating part.** The non-recursive prefix is invisible to
per-call arithmetic and to "which frame regrew" reasoning, because it does not
repeat and it did not change. It is nobody's suspect and it can be the largest
single term.

**The tell is an arc whose every number is per-call or per-level.** If no
number in the discussion is a plain frame size for a function that appears
once, the prefix has not been measured.

### The prologue is the measurement, and it costs one command

For a frame larger than a page, the probe loop states the size exactly:
`mov %rsp,%r11; sub $0xN,%r11`. No bisection, no instrumentation, no run:
`objdump -d` on the already-built rlib, one `awk` for the symbol. **Read the
prologues of every function on the chain, not just the ones someone changed.**
A margin found by process-level bisection is a total; the prologues say where
it went.

### Price a candidate by diffing the artifact, not by counting locals

To answer "how much did this candidate consume", remove its hunk, rebuild, and
re-read the same prologue. Here: `0x31000` both ways, zero bytes. **A
candidate's stack cost is a two-build measurement.** Unoptimized frames are
sized for everything declared anywhere in the body, so added locals often land
in existing slack and cost nothing; the intuition "two `BTreeSet`s must add
something" is wrong in the direction that manufactures a finding.

And **a green suite is a point measurement, not a margin measurement.** It says
the total fit once; the prologue says how much room the change took. Report
both and say which is which.

## Term 3: the per-level frame, including frames the source does not show

**Noted 2026-09-11 from the RT-MAPPING-MULTIOP-DISPATCH respin (landed
`1c48b6c5c`).** Held candidate `1874361` went CI-red with two independent
SIGABRT stack overflows (`rt_capture_projection_grow`,
`rt_branched_scrutinee_unit_body_port`). Root cause: `rewrite_rexpr_inner`
(`crates/ken-elaborator/src/modules.rs`) recurses per expression node, and its
`RMatch` arm rewrote the arms with

```
let arms = arms.into_iter()
    .map(|a| Ok(RMatchArm { pat: rewrite_rpattern(..)?, guard: ..transpose()?,
                            body: rewrite_rexpr(..)?, span: a.span }))
    .collect::<Result<Vec<_>, ElabError>>()?;
```

Each `rewrite_rexpr(a.body)` recurses into a nested match inside the `map`
closure inside `collect`'s driver, so every source nesting level carries the
`Map::next` + closure + collect-loop adapter frames on top of the intrinsic
frame. The WP's prelude additions deepened the nesting and the per-level
constant blew the stack. The fix (`modules.rs` +17/-14) replaced the chain
with a plain loop:

```
let mut rewritten_arms = Vec::with_capacity(arms.len());
for arm in arms { rewritten_arms.push(RMatchArm { .. }); }
```

so `rewrite_rexpr` recurses directly from the loop body.

- **Iterator combinator chains read as flat data plumbing, not as stack.** A
  `map`/`collect`/`and_then` in the body of a per-node recursion contributes
  frames per level, invisibly. The tell is an overflow that scales with SOURCE
  NESTING DEPTH, not input size. On a stack-overflow respin or any recursive
  tree rewrite, look for combinator chains in the recursive body first.
- **The loop swap is a legitimate stack fix, not a style regression**, and it
  is the authorized direction of the per-level-frame rule: removing frames
  from the hot descent is allowed; adding them (an IIFE, a closure, fat inline
  locals live across the recursive call) is prohibited.
- **Frame it honestly.** It raises the depth ceiling by lowering the per-level
  constant; it does not make the traversal heap-based. For a finite checked
  program with bounded nesting that is the correct remedy, but do not claim it
  removes the O(depth) recursion.

**A combinator-to-loop swap is behavior-preserving only if it keeps all four**
of: (1) item order (`Vec::with_capacity` + `push` preserves input order, as
`collect` did); (2) within-item field evaluation order; (3) first-error
short-circuit (the loop's `?` returns on the first erroring field exactly as
`collect::<Result<_>>()` did); (4) spans. All four hold means a pure frame
reduction with no resolution, error or order change.

## The emitted output is a carrier too

A phrase sweep for the withdrawn "mechanism defect" framing ran through three
recuts, covering prose and identifiers; QA caught the last survivor in a test
name. The printed report was still headed with the withdrawn framing. See
[[a-new-criterion-is-applied-forward-and-never-swept-backward-over-the-document-that-states-it]]
for the rule (sweep what the artifact emits, not only what it says).
