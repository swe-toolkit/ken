---
name: library-style
description: >-
  Apply Ken's library/ prose style when authoring, editing, or reviewing
  product documentation, especially library/learn/: compact links, plain
  unnumbered headings, paragraph-led exposition, restrained status language,
  and Python-tutorial teaching structure.
metadata:
  scope: tools
---

# Library Style

Apply this guide to prose under `library/`. Use it when authoring, editing, or
reviewing a page. It governs presentation; the documentation program, the
author's team playbook, and the Librarian's grounding and currency checks still
govern authority and correctness.

Write for a technically capable reader who wants to understand Ken, not the
campaign that produced it. Prefer calm, exact exposition. Let the subject
supply the interest.

## Titles and Structure

Use short, descriptive noun phrases for document titles and section headings.
Prefer `Source File Anatomy`, `Format`, `Effects`, and `Execution` to titles
that promise a journey, ask a rhetorical question, or advertise the material.

Do not put a subtitle after a colon merely to make a title livelier. Put the
qualifying thought in the opening paragraph. For example:

| Avoid | Prefer |
|---|---|
| `Anatomy: orienting in a source file` | `Source File Anatomy` |
| `The shape every entry follows` | `Format` |
| `What actually runs, and what the runtime honestly promises` | `Execution` |

Do not carry filename sequence numbers into titles or headings. A file may be
named `01-anatomy.md` to preserve reading order; its title is `Source File
Anatomy`, not `01 — Source File Anatomy`. Do not number paragraphs or
paragraph-like sections.

Use a heading only for a durable topic boundary that helps navigation. A
heading should normally govern several paragraphs, examples, or both. If a
heading would introduce one short paragraph, remove the heading and connect
the paragraph to the surrounding explanation.

Paragraphs should substantially outnumber headings. As a review heuristic, an
expository page should normally contain at least three prose paragraphs per
heading. Do not split sound paragraphs merely to satisfy the ratio. Short
indexes, reference tables, and exercise answer keys may need a different
shape, but their headings must still be plain and useful.

The prose should remain coherent if the headings are hidden. Use paragraphs to
develop an argument or lesson; do not make headings carry transitions that the
prose itself never states.

## Links

Make link text compact and meaningful in the sentence. When the surrounding
prose already identifies the source, use a small section link such as
`[§1](target)` or `[§4a](target)`. Do not repeat the repository path beside a
link that already targets that path.

Write:

> A required fact belongs in the language rather than in a comment
> ([§1](target)).

Do not write:

> A required fact belongs in the language rather than in a comment
> (`docs/program/07-catalog-style-guide.md` [§1](target)).

Use a short descriptive label when a bare section number would be ambiguous,
such as `[trust model §2](target)`. Do not expose a full repository path in
reader-facing prose unless the path itself is the subject: for example, an
installation location, a command argument, or a file the reader must open.

Keep citations close to the claim they support. A compact label does not relax
the Librarian's evidence rule: the target must establish the claim, not merely
discuss a related topic.

## Status

Describe supported behavior as an ordinary present-tense fact. Do not add
assurances that it has “landed” or is “real.” Avoid “honest” and
“actually” as status emphasis. Its presence in current product documentation
already supplies that context.

Write `The native backend supports X`, not `The real native backend has now
landed X`. Write `This fragment is checked during documentation tests`, not
`This is a real, honestly checked fragment`.

State limitations when they change the meaning of a claim. Name the exact
boundary with `partial`, `planned`, `unavailable`, or a similarly precise
description. For example: `The specification defines X, but the implementation
does not support it`, or `X is planned; current examples must not use it`.

Do not use confidence words in place of a boundary. `Honest`, `real`, and
`current` do not explain which forms work, which fail, or which are specified
but unavailable. Replace the emphasis with that information. Ordinary uses
unrelated to status, such as the mathematical term `real number`, are not
affected.

Keep implementation history, work-package history, review history, and merge
state out of product exposition. Mention provenance only when it helps the
reader assess authority, currency, compatibility, or behavior.

## Prose Patterns

These are revision and authoring heuristics, not an authorship or provenance
test. No external signal reliably establishes who wrote a passage. Apply them
to make the documentation useful to its human reader, consistent with
[PRINCIPLES #2](../../../docs/PRINCIPLES.md#2-the-agent-writes--human-reads-asymmetry-is-a-design-axis).

Lead with the mechanism or reader consequence. Remove an announcement that a
subject is important, central, key, or worth noting unless it supplies a fact
that the next sentence does not. Do not give a one-paragraph thought a heading
or restate the heading as that paragraph's first sentence.

State the positive fact before adding a contrast. Phrases such as “not
merely,” “not just,” and “rather than” do not explain a mechanism by
themselves. Do not manufacture balanced pairs, three-item lists, or a
concluding summary merely for rhetorical shape. End the paragraph when its
evidence has established the point.

Name the component, operation, input, and result. Avoid vague connective
phrases such as “at its core,” “in this landscape,” and “plays a vital role.”
Reuse the exact specification or implementation term rather than varying it for
style. A link identifies where to look; the surrounding prose should say only
why that is the reader's next question.

Do not smooth a pipeline into one broad success claim. Name the handoff and the
stage that consumes or refuses its result. State a limitation once, at the
boundary where it applies, with its source. Do not present an inventory as
complete unless its completeness is established; otherwise bound it or write
“for example.”

Avoid semicolons in prose. Use a conjunction when the clauses have one
continuous relation, or begin a new sentence with a relational phrase when the
second clause qualifies or contrasts with the first. Preserve the relation,
and do not replace a semicolon with a period that leaves the reader to infer it.
This applies to prose only: do not alter code fences, inline code, Ken
examples, literal grammar, or quoted diagnostic text merely to remove a
semicolon.

Precise refusals and trust boundaries are required, not stylistic defects. Do
not remove a negative merely to make prose more affirmative. Every negative
must name the specific operation, condition, and consequence: `check` refuses a
term when conversion fails, rather than “the system cannot handle it.” This is
how the prose keeps
[PRINCIPLES #8](../../../docs/PRINCIPLES.md#8-be-honest-about-the-boundary--over-claiming-is-itself-a-failure)
and
[#14](../../../docs/PRINCIPLES.md#14-nothing-required-lives-in-a-comment--express-it-in-the-language)
visible in the reader-facing text.

## Learning Material

Use the
[Python Tutorial](https://docs.python.org/3/tutorial/index.html) as the main
presentational reference for `library/learn/`. Follow its teaching rhythm, not
its site generator, numbering, or wording.

State the intended reader and the lesson's scope near the beginning. Say what
the page teaches and, where necessary, what it does not yet teach. Keep this
brief enough that the lesson begins promptly.

Introduce one concept at a time in dependency order. Begin with a concrete,
checked example when one is available. Explain what the reader should notice,
add the next concept, and revisit the example with that additional knowledge.

Prefer a continuous lesson to a taxonomy of titled fragments. Use plain
subject headings such as `Numbers`, `Lists`, `Modules`, or their Ken
equivalents. Put motivation, transitions, qualifications, and interpretation
in paragraphs.

Keep examples small enough to read where they appear and complete enough to
support the stated conclusion. Introduce terminology at first use. Put a
caveat beside the concept it limits rather than collecting caveats in a
detached warning section.

Teach the common path in the tutorial and link to reference material for
exhaustive detail. A tutorial should build working understanding; it should not
become a compressed language reference. Do not copy Python's prose or reproduce
its automatic section numbering.

Close with a concise transition to the next lesson or task when the sequence
needs one. Avoid headings such as `Reader can now answer` when a short closing
paragraph can state the outcome directly.

## Review

Read the heading-only outline first. Flag numbered headings, colon subtitles,
rhetorical questions, conversational teasers, and headings that summarize only
one paragraph. Propose the smallest plain replacement.

Compare prose paragraphs with headings. If the outline dominates the page,
combine sections and restore transitions to the prose. Then read the page
without relying on its headings and confirm that the explanation still
progresses.

Inspect reader-visible links for duplicated repository paths. Preserve the
target and source grounding while shortening the label. Do not shorten a label
so far that the sentence becomes ambiguous.

Search status language in context. Flag rhetorical uses of `landed`, `real`,
`honest`, `actually`, and redundant `current` or `now`; do not reject unrelated
technical uses. Require precise implemented, partial, planned, or unavailable
boundaries where the distinction matters.

Read for prose patterns. Flag importance announcements, one-paragraph headings,
empty contrast scaffolding, manufactured symmetry, summary restatement, vague
connective language, unbounded inventories, and links used as explanation.
Preserve a precise refusal: verify that every negative instead names its
operation, condition, and consequence.

For `library/learn/`, verify the audience and scope, dependency order, concrete
examples, incremental explanation, and separation between tutorial and
reference. Treat the Python tutorial as a standard of restraint and sequence,
not as a template to imitate mechanically.

Report style findings separately from factual or currency findings. Cite the
exact title, link, paragraph, or repeated pattern and offer a minimal correction.
Do not turn a sound page into a rewrite merely because another phrasing is also
possible.
