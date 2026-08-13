# LANG-VIEW-RETIRE — execute SURF-1's `view` retirement in the elaborator

**Owner: language. Size: M. Gate: none.**

**Base: re-derive `origin/main` at cut time. Do not take a SHA from this
frame.** Fixed inputs below were measured at **`cbe58725`**; re-measure the
counts at your base and **report both numbers** if they differ — a moved count
is information, not a nuisance.

## Fixed inputs

| fact | site, at `cbe58725` |
|---|---|
| `view` bypasses effect-row verification | `crates/ken-elaborator/src/elab.rs:4687` — `if keyword == DefKeyword::View { return Ok(()); }` |
| `view` bypasses the purity-mismatch diagnostics | `elab.rs:4772` — `DefKeyword::View => {}` |
| `view` is treated as pure for row purposes | `elab.rs:4384` |
| `view` may be **named** `const`/`fn`/`proc` | `parser.rs:83` `expect_legacy_view_name` |
| one shared parser for all four keywords | `parser.rs:457` `parse_view_decl`, dispatched at `:195-198` |
| lexer sites | `lexer.rs:19` `KwView`, `lexer.rs:468` `"view" => Token::KwView` |
| target mapping (ratified) | `spec/30-surface/36-effects.md:410` §1.6.4 |
| spec's own record of the gap | `spec/30-surface/31-lexical.md:656` |
| population | 116 keyword-position occurrences across 28 files under `crates/` |

## D1 — census before you edit anything

Two counts, each reported as a number with its measurement SHA:

1. **Every `view` definition under `crates/`, classified by the `36 §1.6.1`
   rule** into the `const` / `fn` / `proc` it must become. Report the three
   subtotals. A definition you cannot classify is a finding — name it, do not
   guess.
2. **Every `view` definition whose NAME is `const`, `fn` or `proc`**, which
   `expect_legacy_view_name` permits and the ordinary path does not. Report the
   list, even if empty. **"There were none" is a result; "I did not look" is
   not.**

## D2 — the purity truth, measured before the migration lands

This is the deliverable that makes the node worth doing, and it must be
measured **separately from** and **before** the mechanical migration, because
afterwards the two are indistinguishable.

**Migrate the population, then report how many definitions newly fail the
bidirectional purity check** at `elab.rs` — the check `view` currently early-
returns out of.

- **Each failure is a latent purity violation the legacy keyword was hiding.**
  Report it with `file:line`, the keyword it was assigned, and the effect the
  checker inferred.
- **Do NOT silence a failure by choosing a weaker keyword than §1.6.1 dictates.**
  Assigning `proc` to something the rule makes an `fn`, in order to get past the
  check, converts a real finding into a permanent lie in the source. If a
  definition genuinely performs an effect, `proc` **is** its correct keyword —
  the test is whether §1.6.1 gives that answer independently of whether the
  check passes.
- **If the count is zero, say so plainly.** That is a clean result: it means the
  exemption was never load-bearing.

## D3 — remove the keyword, and say what the spelling becomes

Delete the `KwView` dispatch arm, the `DefKeyword::View` variant, the
`expect_legacy_view_name` path, and both `elab.rs` special cases. The shared
`parse_view_decl` body stays.

**Then settle the end state for the token, and cite the spec line you settled
it from.** `31-lexical.md:591` records the sibling precedent — `use` *"remains
reserved and produces a migration diagnostic rather than becoming a free
identifier."* State in the handback which of these `view` becomes and on what
authority:

- reserved, with a migration diagnostic naming the §1.6.1 replacement; or
- a free identifier.

**If the spec does not settle it, stop and ask — do not pick.** That is a
surface-vocabulary question and it belongs to the enclave, not to this node.

## D4 — the diagnostic is the migration's user interface

If D3 lands the reserved-with-diagnostic disposition, the message must name the
**specific** replacement keyword the §1.6.1 rule gives for the definition in
front of it — not the three-way table. A diagnostic that says "use
`const`/`fn`/`proc`" makes every reader redo D1's classification by hand.

## Acceptance criteria

- **AC-1 — no `view` definition remains under `crates/`, and `git grep -nE
  '\bview [a-zA-Z_]' -- crates` returns only prose.** Read the residue; an
  English "view of the kernel" in a comment is not a violation and must not be
  edited to satisfy a grep.
- **AC-2 — the elaborator rejects `view` as a definition keyword**, with a
  control that asserts the *rejection*, not merely that the old form is absent.
- **AC-3 — D2's failure count is reported as a number with its evidence**, one
  `file:line` per failure. **An AC that only asserts "the suite is green" after
  the migration cannot distinguish "no violations existed" from "each was
  papered over with a weaker keyword"** — so the number is the deliverable and
  the green is not.
- **AC-4 — for every definition that changed keyword, the assigned keyword is
  the one §1.6.1 gives**, and the handback states the three subtotals from D1
  next to the three actual counts. If they differ, explain each difference.
- **AC-5 — the `view fn` / `view const` / `view proc` naming census from D1-2 is
  reported**, with the disposition of any hit.
- **AC-6 — no change to `trusted_base()`, no new kernel term, and no change to
  what any surviving definition elaborates to.** This migration is a keyword
  discipline change; the elaborated terms are invariant except where D2's check
  legitimately rejects a program that was previously accepted.

## Banned scope

- **Editing `spec/`, `conformance/`, `docs/` or `library/`.** Those populations
  are prose, they do not execute, and they belong to the enclave, CV, the doc
  ring and the librarian. Report their counts; do not touch them.
- **Weakening or deleting the purity check** to make the migration pass.
- **Renaming `parse_view_decl` or `ViewDecl`.** Cosmetic and it would bury the
  semantic diff.
- **Top-level `let`**, which `36 §1.6.4` removes in the same clause and which
  has its own population.

## Hard stops

- **A `view` definition you cannot classify under §1.6.1** — stop and report
  it. That is a genuine spec-coverage gap and it routes to the enclave.
- **The spec does not settle what the `view` spelling becomes after removal**
  (D3) — stop and ask.
- **D2's failure count is large enough that fixing the violations is a bigger
  job than the migration.** Land the census and the classification, report the
  number, and let the Steward cut the repair as its own node. **Do not let a
  pile of hidden purity violations turn this into an open-ended node** — the
  measurement is complete work on its own.

## Sizing and sequencing

The mechanical half is one pass over 28 files. The unknown is D2's failure
count, which is exactly why D2 is measured and reported rather than absorbed.
**A releasable increment is D1 + D2 with the migration staged**, even if D3 has
not landed — the purity number is the finding, and it should not wait behind
keyword removal.

## Contention

`crates/ken-elaborator/src/{lexer,parser,elab}.rs`. **`parser.rs` is also
touched by `LANG-PARSER-DEPTH-HONEST`** — sequence them, do not run both at
once. No Runtime contention: nothing here is on the R3 path.
