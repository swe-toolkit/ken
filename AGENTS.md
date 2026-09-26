# Working in `swe-toolkit/ken`

Guidance for any Claude Code session (and any agent) operating in this repo.

## Load your role playbook first (every agent, every session)

You are **one role** in a multi-agent federation, and your operating
instructions live in a role-specific **skill**. Before doing any work — and
again **after every context compaction** — orient yourself:

1. Call **`orientation()`** (convo MCP) to learn your **role** (e.g. `steward`,
   `kernel-leader`, `spec-author`) and focus space.
2. Read **`agent/COORDINATION.md`** (federation law) and **`agent/MODELS.md`**
   (model tiers) — binding on every role.
3. **Invoke the skill for your role** (the Skill tool) and follow it as your
   standing playbook — route from your `orientation()` role:

| Your role (from `orientation()`) | Skill to load |
|---|---|
| `steward` | `ken-steward` |
| `lieutenant` | `ken-merge-lieutenant` |
| `architect` | `ken-architect` |
| `librarian` | `ken-librarian` |
| `research` | `ken-research` |
| `adversary` | `ken-adversary` |
| `spec-leader` | `ken-spec-leader` |
| `spec-author` | `ken-spec-author` |
| `conformance-validator` | `ken-conformance-validator` |
| `<team>-leader` — kernel/verify/language/runtime/ergo/foundation/**doc** | `ken-build-leader` |
| `<team>-implementer` | `ken-build-implementer` |
| `<team>-qa` | `ken-build-qa` |

** The `doc` team has no `doc-qa` seat — the `librarian` is its QA.** Its
ring is `doc-leader` + `doc-author` + `librarian`, and the Librarian reviews
the ring's WPs *and* holds a standing as-built mandate no build QA has. So
`librarian` routes to `ken-librarian` (above), **not** to `ken-build-qa`, and
there is no `agent/teams/doc/qa.md`. The doc track is also the one standing
exception to the fleet's single-threaded posture: it **runs concurrently**
with build work (operator, 2026-07-21), on the basis that it touches
`library/` and `agent/` rather than `crates/` — the exception is
contention-free-ness, not priority.

Build-team roles share the `ken-build-*` archetype skills — your team is the
prefix on your role name (`kernel-leader` → `ken-build-leader`). The skills are
the `agent/playbooks/` corpus, surfaced as skills via `.claude/skills/` (Claude
Code) and `.agents/skills/` (Codex) — both symlink into `agent/playbooks/`;
editing a playbook edits its skill. If a team-specific overlay
exists (`agent/teams/<team>/<role>.md`), load it after the archetype skill. The
Steward owns this corpus and its routing.

**If the `Skill` tool reports your skill is unknown:** the skill registry loads
at **session start**, so a skill added or updated mid-session (e.g. you just
`git rebase`d onto a `main` that added it) is not registered for the `Skill`
tool until your next restart. Don't block on it — **`Read` the file directly at
`.claude/skills/<skill>/SKILL.md`** (or its `agent/playbooks/` target) and follow
it; it is the identical playbook. A fresh session start registers every skill
for the `Skill` tool. This makes playbook updates self-heal on rebase without a
forced restart.

## Read the decision checks (every agent, every session)

After loading your playbook, **read `agent/memory/CHECKS.md`**: at most twelve
checks, one per recurring cause of hard stops. That is the whole startup read.
**Do not bulk-read the scope directories**, at startup or after compaction.

**Apply a check when its trigger fires**: before you post a ruling, release a
frame, approve a candidate, or build on a premise. Recalling a lesson is not
applying it. The 2026-09-25 audit found that ten of the last twenty hard stops
had a lesson naming their cause in the responsible seat's required reading,
and none of the ten was prevented.

**The rest of `agent/memory/` is reference, searched on demand.** When a
decision touches an area where the fleet has been burned before, grep your
scopes (the table below) for the mechanism term, not only the subject name,
and read the hits. Nothing requires you to read a lesson you did not search
for.

| Your role | Scopes to search (under `agent/memory/`) |
|---|---|
| _any role_ | `fleet/` |
| `steward` | `fleet` + `enclave/` + `roles/steward/` |
| `lieutenant` | `fleet` + `build/` |
| `architect` | `fleet` + `enclave` + `roles/architect/` |
| `spec-leader` / `spec-author` / `conformance-validator` | `fleet` + `enclave` + `roles/<role>/` |
| `librarian` | `fleet` + `roles/librarian/` + `teams/doc/` |
| `research` | `fleet` + `enclave` + `roles/research/` |
| `adversary` | `fleet` + `enclave` + `roles/adversary/` |
| `<team>-leader` | `fleet` + `build/` + `build/leaders/` + `teams/<team>/` |
| `<team>-implementer` | `fleet` + `build/` + `build/implementers/` + `teams/<team>/` |
| `<team>-qa` | `fleet` + `build/` + `build/qa/` + `teams/<team>/` |

These are **lessons, not law**: recall aids that reflect what was true when
written. Verify that a named file, flag or function still exists before
acting on one. This corpus is the source of truth. Codex's generated
`~/.codex/memories/` and any harness's private memory are supplemental only.

**Adding to the corpus** (rules in `agent/memory/README.md`): search first and
extend an existing lesson rather than adding a near-duplicate. A cause that
has recurred across hard stops belongs in `CHECKS.md` as a merged or replaced
check, not as one more file. Directory placement is the only routing, and a
`scope:` frontmatter key confers nothing.

## Reference material is off-limits to code authors

`local/refs/` (gitignored) holds reference implementations. **Do not read them
to write Ken's code.** Per `CLEAN-ROOM.md`:

- **The AGPLv3 prototype (`yon`) is NOT mounted in this environment.** It
  is the *excluded inspiration* — Ken's design is its own; `yon` is not a
  consultable reference. There is zero AGPLv3 contact, which is strictly
  cleaner. **No agent should go looking for it.**
- **The permissive references** (Lean, Agda, cooltt, smalltt, cctt, …) may be
  **read to understand** by the Architect / Spec enclave, **the research agent,
  and the adversary agent** (the adversary for known prior-art failure modes) to
  sharpen the spec / hunt flaws, but **not copied** into the repo. Implementer
  agents build from `/spec`, never from `local/refs/`.
- **Copyleft references** (GPL/AGPL/CeCILL — e.g. smtcoq, spot, jif) are for the
  **Spec enclave, the research agent, and the adversary agent only**, for
  approach and behavior only, under the leakage recheck. Never consulted by
  implementer agents, never vendored.

When unsure whether you may look at something under `local/refs/`, the answer
is no — ask the operator or the Spec enclave.

## Enforced prohibitions

Never call `mcp__convo__get_transcript`, and read full message bodies with
`get_recent_context` or `get_mentions` at `detail: "standard"`. This rule and
the others in `agent/COORDINATION.md §12a` (stash, workspace builds,
`gh run rerun`, `moot.toml`, held refs) are refused mechanically.

## Conventions

- **Read `docs/PRINCIPLES.md`** — the project's reasoning charter (agents-write/
  humans-read, decide on intrinsic merits not effort, small auditable TCB,
  reflect-don't-extend, subsume-don't-proliferate, honesty about the boundary).
  When the spec does not settle a choice, reason from it.
- **Local builds and tests are targeted only:** use `scripts/ken-cargo`
  scoped to the crate or suite you touched, never `--workspace`. The whole
  workspace runs in CI, so "no-regression" in a frame means green in CI
  (`agent/COORDINATION.md §12`).
- **Write in plain text. No decorative icons** (operator, 2026-08-01). Do not
  open lines, headings, table cells, or emphasis with symbols like star, warning
  sign, no-entry, check mark, or any emoji. **The operator finds them
  distracting rather than helpful, and they are not a substitute for saying the
  thing.** Convey emphasis with **bold**, with sentence structure, and with
  where you put the point — not with a glyph.
  - Applies to every artifact you author: docs, WP frames, tracker nodes, commit
    messages, PR bodies, playbooks, memory lessons, and **convo posts**.
  - This is a rule about *decoration*, not about characters. Symbols that carry
    information stay: arrows in derivations (`⇒`, `→`), math and Ken notation
    (`Ω`, `≠`), and literal terminal glyphs quoted as data (a pane's `❯`
    prompt, a spinner). If removing it would lose information, it is not
    decoration.
  - **If you are copying the shape of a nearby artifact, copy the current
    rule, not the old formatting.** Much of the corpus predates this and was
    only cleaned in the instruction material; an existing WP or issue that
    still carries icons is not a licence to add more.
- **Wrap markdown at 80 columns** — target 80 *display* columns / codepoints (a
  multi-byte `—`, `→`, `Ω` is one column); lines of 81–85 are acceptable slack,
  so only reflow what exceeds **85**. Don't spend your own tokens hand-reflowing
  prose: after you finish writing or editing a Markdown file, **delegate the wrap
  to a cheap Haiku subagent** driven by the `wrap-md-80` skill. Spawn it with the
  Agent tool (`model: haiku`), telling it to read
  `agent/playbooks/tools/wrap-md-80.md` and apply it to your file(s). The skill
  is a pure whitespace-only reflow (it never changes a word, and leaves code
  fences, tables, and front matter alone); verify its output is safe with
  `git diff -w --stat` showing **no** content change. This keeps authoring on
  your model and formatting on the cheapest tier.
- **Use Mermaid for diagrams and charts** — dependency graphs, flows, state
  machines, sequence diagrams — in fenced ` ```mermaid ` blocks, **not** ASCII
  art (it renders, diffs, and edits better). Mermaid/code fences are **exempt**
  from the 80-column rule. Keep node labels plain (avoid parentheses inside
  labels; spell out symbols like `Omega` if a renderer is finicky).
- The spec is in `spec/` (`spec/SPEC-PROGRESS.md` is the status backbone); open
  design decisions are in `spec/90-open-decisions.md`; architecture decisions in
  `docs/adr/`; the clean-room policy in `CLEAN-ROOM.md`.
- Agent-team coordination law: `agent/COORDINATION.md`. Git/merge model:
  `docs/program/04-git-and-integration.md`.
