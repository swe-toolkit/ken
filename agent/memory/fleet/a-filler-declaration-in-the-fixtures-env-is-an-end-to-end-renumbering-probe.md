---
name: a-filler-declaration-in-the-fixtures-env-is-an-end-to-end-renumbering-probe
description: Adding one throwaway declaration to a fixture's own env renumbered the ids a converted assertion was built to tolerate, proving the conversion end to end in a single run — and it also separated two id populations that the justifying comment ran together, since a fixture-env edit moves the fixture's ids and leaves base-env ids fixed
metadata:
  type: feedback
---

# A filler declaration in the fixture's env is an end-to-end renumbering probe

**Measured 2026-08-17 on `000d69663` (ds5b).** A remedy replaced a literal
operand assertion with a structural one, on the stated ground that *"an unrelated
prelude/`vec_env()` edit renumbers them without changing the failure this test
pins."* **The claim was argued, never run.**

One throwaway declaration inserted into `vec_env()` ahead of `Vec`:

```
before: found: ((Dg574 Dg67) @8)
after:  found: ((Dg576 Dg67) @8)     9 passed, 1 ignored, 0 failed
```

The head moved by exactly the two ids the filler allocated, the suite stayed
green, and the **deleted** literal `((Dg574 Dg67) @8)` does not occur in the new
message — so the pre-change assertion would have red on this edit.

⇒ ***One declaration buys the counterfactual that no amount of substring
reasoning can.*** It is cheap (test-only, no production rebuild), faithful (the
renumbering is real, not simulated), and it exercises the assertion through the
same producer the shipped test uses.

## IT ALSO SEPARATED TWO POPULATIONS THE COMMENT RAN TOGETHER

The comment named the perturbation class as *"a prelude/`vec_env()` edit"* — one
phrase, two different reaches:

| id | origin | moved by a `vec_env()` edit? |
|---|---|---|
| `Dg574` (`Vec`) | the fixture's own env | **yes** |
| `Dg67` (`Nat`) | the base env (`ElabEnv::new()`) | **no** |

⇒ ***A sibling assertion pinned `Dg67` by a literal and was correct to***, because
its id is stable under the class of edit the comment names; only a base-env
change reaches it, and that reds loudly rather than silently. **Two sites of the
same message got two different remedies, and measuring the perturbation is what
shows the split was principled rather than inconsistent.**

**The general move: when a comment names a perturbation class to justify a
remedy, run one instance of that class and see which operands it actually
reaches.** A class named in prose is almost always wider than the class the
author tested — sibling of
[[a-negative-check-passes-for-any-reason-so-it-needs-a-positive-control]]
(a control's discriminating power is a measurement, never a reading).

## READ THE MERGED BLOB, NOT THE WORKING COPY

My worktree base predated the merge, so the file on disk was the **pre**-merge
version — every line number and every assertion I would have read was the one
being replaced. `git show <sha>:<path>` into scratch, and confirm the crate's
`src/` is byte-identical between the merge and your HEAD
(`git diff --stat <sha> HEAD -- crates/`) before trusting a local run as a
measurement of the merged tree.
