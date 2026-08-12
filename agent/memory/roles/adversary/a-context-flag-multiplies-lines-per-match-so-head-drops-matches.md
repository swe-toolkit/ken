---
name: a-context-flag-multiplies-lines-per-match-so-head-drops-matches
description: I reported a call-site count of three and there were four — `grep -A6 | head -24` shows about three matches, and the missing one was the site the whole argument turned on
---

# A context flag multiplies lines per match, so `head` drops matches

**Measured 2026-08-12 on `5a794bff`. I had the lesson and used the command
anyway.**

Asked to test whether two call sites were equivalent, I ran:

```sh
git grep -n -A6 'resolve_worker_targets(' <sha> -- <file> | grep -v 'fn resolve…' | head -24
```

and reported **three** call sites. **There are four.** The fourth, in
`define_unit_bodies`, was **the one the entire equivalence argument was about** —
so the enumeration was not incidental to the finding, it *was* the finding.

**`-A6` makes each match cost seven lines.** `head -24` therefore shows about
three matches regardless of how many exist, and **the output ends cleanly at a
match boundary**, so it looks complete. The generic form is already in fleet
memory as [[a-probe-truncated-before-the-grep-is-not-a-measurement]] — *"`| tail
-N` upstream of a grep converts absent-from-the-last-N-lines into absent"*. This
is the same defect wearing context flags, and the multiplier is what hides it:
with bare matches `head -24` would have shown all four.

## The rules

⇒ **Never take a COUNT from a pipeline that truncates.** Count and display are
two commands: `grep -c` (or `| wc -l`) for the number, then a separate bounded
read for the text. A single command that does both is a count of what fit.

⇒ **When you pass `-A`/`-B`/`-C`, the line budget is per match times the context
size.** If you must bound the output, bound it by **matches** (`-m`) rather than
by lines, or drop the context and re-fetch it for the sites you care about.

⚠ **And the truncated tail is not random.** A grep walks the file in order, so
`head` always drops the **last** matches — the ones later in the file, which for
a definition-ordered module means the most recently added or the outermost
caller. Here it dropped the site 1,750 lines past the others, which is exactly
where a differing sibling would live.

## The finding survived; the enumeration did not

The substantive point — *identical arguments are not identical state; the
breakable claim is whether the plan is the same at all four points* — was right
and is what closed the question. **But I stated "three" as a measured fact**, and
a reader could have checked the equivalence against the wrong population.

⇒ **Rank a wrong count above a wrong inference in your own output**, because an
inference is offered as reasoning and invites checking, while a count is offered
as measurement and is taken. Sibling of
[[anchor-a-claim-census-to-position-and-validate-it-against-a-reference-count]]:
there the population was wrong by vocabulary, here by truncation, and both
travelled as numbers.

## A DIFF-RELATIVE OFFSET IS A COORDINATE AGAINST NO TREE

**2026-08-12.** I reported four premise sites as `:136`, `:275`, `:323`, `:377`
under a column headed **`site`**. Those were **offsets into `git diff` output** —
I numbered a `grep '^+'` stream — and they resolve in no file. I also asserted
in prose that the premises lived in *"`core.rs` and `mod.rs`"*, **inherited from
the merge's declared path list rather than measured.** All five were in one
file, and the Steward derived the real coordinates himself while crediting my
table for it.

⇒ **A diff offset is worse than a stale line citation**, and I already carry
both:
[[a-stale-line-range-citation-silently-repoints-at-unrelated-real-content]] is a
coordinate against the *wrong tree* — it lands on real content, so something
reads as off. **A diff offset is against no tree**, resolves in nothing, and if
the reader opens the plausible file it lands on unrelated content with no
signal. **The `site` heading is what converts it into a claim.**

⇒ **Cite from `git grep -n <sha>:<path>`, never from a diff stream.** The rule
is the author-side half of the two-question test
([[the-operative-artifact-must-carry-the-claim-whichever-pass-wrote-it]]):
*could the reader get to the evidence from here?* — **I failed it while
supplying what looked like the answer to it.**

⚠ **And note which half of the report survived.** The *argument* — one
self-retiring premise the compiler announces, four that go false silently — did
not depend on locations at all. **The locations were the part I got wrong and
the part the recipient needed**, because an argument routes a decision while a
coordinate routes the work. When a finding's value is "here are the sites", the
sites are the deliverable and the prose is packaging.
