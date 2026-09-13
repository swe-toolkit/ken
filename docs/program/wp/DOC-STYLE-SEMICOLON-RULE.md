# WP frame — `DOC-STYLE-SEMICOLON-RULE`

Owner: doc ring. Implement the operator's semicolon-restraint rule, then apply
only that rule to the compiler guide. Librarian is the sole QA/as-built owner.

## Objective

Add the heuristic to `agent/playbooks/tools/library-style.md`, with its preferred
rewrites and a counterweight for genuinely load-bearing semicolons. Revise prose
semicolons only in the eight `library/guide/compiler/` pages.

## Acceptance

- Re-measure raw semicolons at cut; classify every occurrence as prose or
  protected code/inline code/Ken example.
- For every changed prose semicolon, preserve its relation with a conjunction or
  leading relational phrase. Do not reflow paragraphs, vary technical terms, or
  alter facts, refusals, boundaries, or links.
- Never alter semicolons in code fences, inline code, or checked Ken examples.
- Edit only the guidance, doc-author checklist if its heuristic inventory needs
  the promised pointer, eight compiler pages, and this frame. No catalog, spec,
  crates, or broader library rollout.
- Run targeted documentation validation; Librarian reviews exact SHA.

## Outcome

Report residual semicolons by classification and any retained load-bearing prose
semicolon, so the gated library rollout has a measured input.
