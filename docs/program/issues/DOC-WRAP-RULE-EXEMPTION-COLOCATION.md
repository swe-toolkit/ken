---
id: DOC-WRAP-RULE-EXEMPTION-COLOCATION
title: "the 80-column rule and its exemption list are stated at different levels in five documents, three of which state the rule with no exemption at all and two of which carry lists that disagree, so two seats applied the rule to exempt front matter and one was about to degrade a file that held the refuting measurement"
status: ready
owner: doc
size: S
gate: none
depends_on: []
blocks: []
github: null
origin: "Measured 2026-09-16 during review of a fleet-memory candidate. The Architect raised the colocation defect (evt_139rybd8n3c31) and widened it to a second document with a divergent list (evt_65sxwwmbyy668); the Steward measured the full sweep and cut this node at the Architect's recommendation, since the fix now carries a canonical-list decision rather than being a one-line edit."
tier: T2
---

# The defect

The repo's 80-column markdown rule has an exemption list — front matter,
tables, fenced code, and more. **In every document that states the rule to a
reader, the exemption is either absent or arrives as a property of something
else.** A reviewer applying the rule by eye never reaches it.

## Measured across `origin/main` `10046c38e`

| site | states the rule to | exemption | list |
|---|---|---|---|
| `AGENTS.md:212-222` | every agent, every session | as a property of the Haiku skill | code fences, tables, front matter |
| `agent/playbooks/federation/architect.md:318-330` | the Architect | as a property of the Haiku skill | code fences, tables, **Mermaid**, front matter |
| `agent/playbooks/federation/steward/merge-procedure.md:686` | the Steward, **at the routing gate** | **none** | — |
| `docs/PRINCIPLES.md:409` | every reader of the charter | **none** | — |
| `agent/playbooks/tools/wrap-md-80.md` | the wrap tool | yes, in its own right | fenced blocks of any kind including mermaid, indented code, inline code spans, HTML comments, YAML front matter, tables, unbreakable tokens |

Plus **four** `docs/program/` frames carrying `Wrap markdown at 80 columns` as
a bare instruction line with no exemption within six lines (a fifth has one).

## Three findings, and the second and third are the ones that change the fix

**1. Three of the five rule-statement sites carry no exemption at all.** The
worst-placed is `merge-procedure.md:686` — *"Width-check markdown at 80 display
columns (codepoints, not bytes) before routing."* That is the sentence the
Steward acts under when gating a candidate, and it is stated bare. **The
routing gate as written flags exempt front matter**, which is exactly the
failure that produced this node.

**2. The two lists that exist disagree, and neither is the canonical one.**
`architect.md` names Mermaid blocks; `AGENTS.md` does not. **But both are
abbreviations of `wrap-md-80.md`'s list, which is larger than either** — it
also exempts indented code, inline code spans, HTML comments, and unbreakable
tokens. So "make it one list" cannot mean picking one of the two playbook
lists: **lifting either abbreviated list into its rule sentence would silently
drop four more exemption classes.** The canonical list is the tool spec's, and
it is the only one written to be complete.

**3. The rule is stated to readers who will never invoke the tool.** The
colocation defect is usually described as the exemption hiding in a sentence
about the Haiku pass. That is true, and it understates it: a reviewer applying
the rule by eye, a Steward width-checking at the routing gate, and an
implementer reading a frame's instruction line **are not going to run the wrap
tool at all.** An exemption stated as a property of a delegated tool does not
reach any of them, so moving it closer is not enough — it has to be stated as a
property of *the rule*.

# Why this is worth a node rather than a message

**Measured cost, not a tidiness argument.** The Steward raised a wrap finding
against a 493-column line that was a front-matter `description:` — exempt. It
reached a runtime implementer as a should-fix. That implementer **had already
measured the corpus** (23 of 23 fleet lessons exceed 85 columns in front
matter; longest 696; zero wrapped) and **was still going to reflow it**, which
would have made their file the only deviant in its corpus.

So the failure is not that someone skimmed. **Two seats who both cite this rule
when enforcing it read past the exemption in the same document, and the one
holding the refuting measurement moved to comply anyway.**

Per `steward.md §4c`, the constraint is grounded: this is a measured
misapplication with a named artifact that was about to be degraded, not an
aesthetic preference for tidier documents.

# Scope

**In scope.** The five rule-statement sites above, and the bare-instruction
lines in `docs/program/`. The deliverable is that **wherever the rule is stated
to a reader, its exemption is readable at that point** — not one bullet down,
not as a property of a tool the reader will not invoke.

**The canonical-list decision is the doc team's**, and it is the reason this is
a node. The recommendation from the measurement: canonicalize on
`agent/playbooks/tools/wrap-md-80.md`'s list, since it is the only one written
to be complete and both others are lossy abbreviations of it. Whatever is
chosen, **it must be one list** — the current divergence is how a fix that
edits one file drops Mermaid, or four other classes, from the exemption.

**Out of scope.** Changing the rule itself — the 80-column target, the 81-85
slack, the reflow-what-exceeds-85 threshold. None of those is in question and
none should move. Also out of scope: reflowing any existing file. **No
`.md` body content is edited by this node** beyond the rule statements
themselves.

**Note for whoever executes it.** `CLAUDE.md` is a symlink (mode 120000) to
`AGENTS.md` on `origin/main`. The text is in `AGENTS.md`; editing `CLAUDE.md`
as a regular file would replace the symlink.

# Acceptance

- **AC-1, colocation.** Every site in the table above that states the rule to a
  reader states its exemption in the same breath. Control: a reader who stops
  at the end of the rule's own sentence knows front matter is exempt.
- **AC-2, one list.** All sites naming an exemption list name the *same* list.
  Control: `diff` the lists pairwise; the count of distinct lists is 1.
- **AC-3, the sweep is a predicate, not this table.** The exit criterion is
  "every site that states the rule", not "the five rows above". Control: state
  how a newly-added rule statement would be caught, and re-run the census key
  after the edit rather than trusting the seed list. The table is a seed.
- **AC-4, no rule drift.** The 80 target, the 81-85 slack, and the >85 reflow
  threshold are byte-identical to their current values at every site.
  Control: grep each number before and after; any change is a failure.
- **AC-5, no body reflow.** `git diff -w --stat` shows changes confined to the
  rule statements. No file is reflowed as a side effect of this node.

# Not blocked, and blocking nothing

This competes with no lane. File it when the doc track has room.
