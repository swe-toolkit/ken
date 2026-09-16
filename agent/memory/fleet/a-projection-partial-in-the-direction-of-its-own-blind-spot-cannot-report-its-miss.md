---
name: a-projection-partial-in-the-direction-of-its-own-blind-spot-cannot-report-its-miss
description: "A classifier keyed on a token cannot see the cluster spelled another way - and because the key and the blind spot are the same object, what it misses lands in the residual bucket looking classified rather than missed. The residual caught it; the classifier could not. Size the residual and read a large one as a verdict on the KEY, never on the population."
metadata:
  type: feedback
---

# A projection partial in the direction of its own blind spot cannot report its miss

**Measured 2026-09-16 while classifying a large diff for `RT-D5B`.** The
classifier keyed on the literal token `MappingAcquireFile` and reported **33
hunks as "neither"** — neither cluster A nor cluster B. The population was not
unclassifiable. Cluster B is mostly spelled

    try_new_mapped_file · resource_map_file · mapping_acquire_file_source_rights

none of which contain the key. **The residual bucket caught it. The classifier
could not.**

## Why this shape is worse than an ordinary miss

A classifier keyed on `K` partitions on the presence of `K`. Anything spelled
without `K` is invisible **to the same faculty that would have to report it
missing** — so it does not surface as an error, a gap, or a zero. It lands in
*"neither"*, and a residual bucket **reads as a classification result rather
than as a failure**. The output looks complete and is shaped like an answer.

⇒ **The key and the blind spot are the same object.** No amount of care applied
to the classifier's logic helps, because the logic is working correctly; it is
the projection that is partial, and it is partial in exactly the direction that
would be needed to notice.

## The detector, and it is cheap

- **Size the residual, and treat a large one as a reading on the KEY rather
  than on the population.** "A third of the hunks are unclassifiable" is almost
  never a fact about the hunks.
- **Ask what else the concept is SPELLED as** before keying anything on one
  token. A Rust variant, a snake_case helper and a Ken-source identifier are
  routinely three different strings for one idea, and no rule derives one from
  another.
- **Prefer a key the producer already emits** over one you invent — an intern
  table, a declared enum, a generated catalog. A key you chose is a key whose
  coverage you also have to prove.
- **Keep a residual bucket at all.** The only reason this was caught is that
  the classifier had somewhere to put what it could not name. A partition with
  no residual reports 100% coverage by construction.

## The counting form of the same failure

The identical shape appears in counts, where the selector is the key: a census
keyed on a spelling the subject does not use returns a clean, confident,
**wrong** number — and offers nothing to suggest it is short. See
[[repairing-a-census-completeness-does-not-re-aim-its-subject]] for the case
where the census was complete and answering a different question, and
[[a-pattern-match-is-evidence-about-what-encloses-it]] for the case where the
hit was real and its scope unresolved. This file is the third: the hit never
happened, and the instrument reported that as a category.

Related: [[an-enumeration-needs-a-proven-closure-not-a-better-grep]] — the
answer is a proven closure over the producers, never a better key.
