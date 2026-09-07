---
id: LANG-FIXITY-DECL-SURFACE
title: "`infixl N op` / `infixr N op` / `infix N op` populate a fixity table the parser consults -- the third and last part of user-defined operators, and the only one carrying a real design call: declaration-before-use versus whole-module collection, and scoping across imports"
status: active
owner: language
size: M
gate: none
depends_on: [LANG-INFIX-APPLICATION-DEFAULT]
blocks: []
github: null
origin: "Filed 2026-08-15 by the Steward from CONF-FMT8-LEVELTOK's census, originally scoped to fixity alone and left `draft`/unsized pending a scope ruling. Architect ruling evt_1s7mqjg4tyxx1 answered it: declared fixity is OWED, not an endpoint -- 32-grammar.md:392-393 names its existence as explicitly NOT an open question. The same ruling established the node was scoped to a third of the feature and decomposed it; this node is now part (iii). Import-scoping measured and costed by the Architect at evt_33rw7w8xkdya2, re-verified by the Steward. Steward re-cut per COORDINATION section 2."
---

> # THE ID IS DELIBERATELY UNCHANGED. DO NOT RENAME THIS NODE.
>
> The Architect's ruling observed that the *feature* is user-defined operators,
> not fixity, and initially instructed a rename to match. **He then withdrew
> that instruction** (`evt_33rw7w8xkdya2`), and the reason is worth keeping:
> **his rename was conditioned on this staying ONE node covering all three
> parts.** Once it was cut into three, **(iii) genuinely is just fixity** — the
> id is the accurate name for what this node now contains, not a legacy
> compromise.
>
> There is a second, independent reason. `seed-canonical-format.md` links
> `[[LANG-FIXITY-DECL-SURFACE]]` at three live sites (`:27`, `:58`, `:424`),
> landed at `e6d2716cf`. **Renaming would dangle every one of them, recreating
> the exact defect [[CONF-BLOCKER-MARKER-RECONCILE]] existed to remove**, hours
> after it merged.
>
> The feature as a whole is [[LANG-SYMBOLIC-OPERATOR-NAMES]] +
> [[LANG-INFIX-APPLICATION-DEFAULT]] + this node. **No node is named
> `LANG-USER-DEFINED-OPERATORS`** — that would be a fourth name for work three
> nodes already cover.

## RELEASED. Predecessors landed; the import-scoping ruling is folded in.

Both predecessors have landed — [[LANG-SYMBOLIC-OPERATOR-NAMES]] and
[[LANG-INFIX-APPLICATION-DEFAULT]] — so `a <+> b` parses and a fixity table now
has something to bind. The one live design call this node carried, scoping
across imports, was ruled by the Architect at [[LANG-INFIX-APPLICATION-DEFAULT]]'s
merge Decision (`evt_7gpavatfrehyq`) and is now **normative spec** at
`spec/30-surface/33-declarations.md §6`. That ruling is folded into `D1` and
`D2` below, and this frame is now shovel-ready.

**What `D1` still owes is a report, not a decision.** Sub-questions 2 and 3 are
ruled below; sub-question 1 is guided to its natural reading. The ring reports
`D1` with citations before building `D2` (`AC-1`) — that gate stands.

## The scope question is ANSWERED. Recorded so it is not re-litigated.

This node was `draft` and unsized because the Steward would not paraphrase
`spec/30-surface/32-grammar.md:393` into a ruling. The Architect ruled on the
verbatim sentence:

> *"The remaining spellings and the levels of non-arithmetic user operators
> stay `OQ-syntax`; the existence of declared fixity, and this arithmetic
> ordering, are **not**."*

`OQ-syntax` is the open-question marker; the sentence names the **existence of
declared fixity** as explicitly not open. ⇒ **Not an endpoint. It is owed.**

**The V0 objection is a category error and is closed.** `32-grammar.md:410`
excludes *"operators or fixity"* — and in the same list excludes **literals**
and **`match`**, both landed. It is a bootstrap staging subset, not a statement
about what the L-level owes. Do not re-raise it.

## Deliverables

**`D1` — report the design fork with citations, before any table is built.**

**1. Declaration-before-use, or whole-module collection?** Must `infixl 5 <+>`
appear textually before the first use of `<+>`, or does the parser collect
every fixity declaration in the module first? **These differ observably** on a
module that uses an operator above its declaration. **Report what `§6` and the
module chapter settle.** Where the spec leaves it open, **whole-module
collection is the natural reading** and the guided default: the Architect
confirmed `D2`'s post-resolution reassociation supports it at zero extra cost
(sweep a unit's `fixity_decl`s into the table before reassociating its bodies,
so a use textually preceding its operator's `fixity_decl` still sees the
fixity), whereas declaration-before-use would need an added ordering check. He
explicitly did not decide q1 — it stays the ring's/spec's call. State it as a
choice if the spec does not force it.

**2. Scoping across imports — MEASURED AND COSTED. A two-option fork, not an
open question.** Measured by the Architect (`evt_33rw7w8xkdya2`) and
re-verified by the Steward at `e6d2716cf`:

```rust
exports: HashMap<String, HashMap<String, String>>    // modules.rs:54
```

**Module → local name → resolved name. Names only, no attributes.** ⇒ **A
fixity declaration on an exported operator has nowhere to live in the current
representation.** The two options and their prices:

| option | meaning | price |
|---|---|---|
| **module-local fixity** | an operator's fixity does not cross an `import` | **zero change to `modules.rs`** |
| **propagating fixity** | fixity travels with the operator's canonical identity across `import`/re-export | **realized as the `D2` `GlobalId`-keyed side-table (`num_values` pattern), NOT the export widening** — export leaf untouched |

> The "export leaf widens from `String` to a record" realization that this
> table originally priced (`evt_33rw7w8xkdya2`; 10 sites, grown to 16 by
> `9ca7c4ce5`) is **superseded** by the Architect's `D2` ruling below: fixity
> lives in a program-threaded side-table keyed by `GlobalId`, so the export map
> is untouched and that site count is moot. The **answer** (propagating) is
> unchanged; only the realization is cheaper.

**RULED — propagating fixity, now normative spec.** The Architect ruled
question 2 at [[LANG-INFIX-APPLICATION-DEFAULT]]'s merge Decision
(`evt_7gpavatfrehyq`), and it landed as `spec/30-surface/33-declarations.md §6`:
declared fixity is a property of the operator's **canonical identity**, not of
any surface path or alias, so it **travels with import and re-export** — a
client, an aliased import, or a re-exported path sees the declaring module's
fixity, because both republish the same `GlobalId` rather than minting another
(`§3.3`, `§4.3`). Fixity is **not re-scopable at a use site**. ⇒ **The
propagating option is the answer**, not module-local fixity. Its realization is
the `GlobalId`-keyed side-table ruled in `D2` (not an export-map widening), so
the `modules.rs` threading-site count that priced the widening is moot.

**The conflicting-fixity-on-two-imports sub-question is DROPPED.** Per `§6`, two
**distinct** declarations that share an operator spelling, imported unqualified,
are an ordinary identity clash resolved by the existing **`AmbiguousReference`**
rule (`§3.3`) — there is no fixity-specific merge, and qualified, aliased, or
selective import disambiguates exactly as for any other name. Nothing to design
here.

**3. Redeclaration within a module — RULED: ERROR.** A fixity declaration binds
precedence and associativity to the operator's one canonical identity (`§4.3`,
`§6`); two conflicting fixity declarations for the same in-module identity are
not last-wins. Reject with a diagnostic that names both declaration sites. (Two
declarations that agree are idempotent, mirroring the `§3.3` same-identity
rule.)

**`D2` — RULED: a post-resolution reassociation pass over a `GlobalId`-keyed
fixity table.** Ruled by the Architect (`evt_4b66p3qb88wnn`). `infixl N op` /
`infixr N op` / `infix N op` populate the table; a single pass reassociates the
operator spine after name resolution. (Anchors below are perishable — verified
at `9ca7c4ce5`; the Architect's own citations were line-shifted, these are the
measured lines.)

**Consultation is POST name-resolution, in a distinct pre-body pass — not
in-parser.** Because fixity binds to the resolved canonical identity (`§6`), an
operator's fixity is unknown until `§3.3` resolution has mapped its spelling to
a `GlobalId`. So:

- **Resolution keeps the operator run FLAT.** Name resolution does not
  reassociate (local ids do not exist there yet); it carries the run as a flat
  `RInfixSpine` (ordered operands + resolved operator heads) and passes it
  through untouched.
- **A single post-resolution pass is the sole reassociator.** It consumes the
  neutral spine and emits ordinary `RApp`, so the existing `check`/body path is
  UNCHANGED and only ever sees `RApp`. Reassociation is **not** type-directed —
  association must be fixed before the body is typed (`a <+> b <+> c` has no
  type until you know it is `(a<+>b)<+>c`), so a lazy-in-elaboration placement
  is rejected: it would smear a purely syntactic restructuring across the
  type-directed elaborator (reflect-don't-extend, and far harder to audit).
- **Exact site — the predeclare→check boundary in `elaborate_mutual_group`**
  (`crates/ken-elaborator/src/elab.rs:10931`). The group pre-admits all members,
  binding every name in `globals` (`id = env.fresh_id(); globals.insert(name,
  id)`) BEFORE any body is elaborated, then checks bodies. Run reassociation at
  that boundary (and the analogous point on the singleton path). There `globals`
  already holds imported + earlier-elaborated + this-group ids — exactly and
  only what a spine in this group's body can reference under dependency-ordered
  elaboration — so every head resolves to a `GlobalId` with no forward-reference
  gap.

**Carrier — a program-threaded `HashMap<GlobalId, Fixity>`, NOT an export-map
widening.** Use the exact shape and lifecycle of the existing
`num_values: HashMap<GlobalId, NumericLitVal>` (`elab.rs:323`, threaded `&mut`
through the elaborator; populated at declaration via `cx.num_values.insert`
`elab.rs:7898`/`:7974`; read program-wide via `env.num_values.get`
`compiler_driver.rs:3847`/`:3904`). Populate the fixity table when a
`fixity_decl` is elaborated: resolve its named operator to its DEFINING
`GlobalId` and insert `Fixity { assoc, prec }`. This propagates across `import`
with **zero interface change**: `§4.3` republishes the same `GlobalId`, and the
table is program-accumulated, so an imported operator's fixity — recorded when
its defining unit was elaborated, dependency-first, before the importer — is
already present when the importer's spines reassociate.

**Consultation and the default.** Reassociation maps head-name → `GlobalId` (via
the unit's resolved `globals`, which already honors qualified/aliased/selective
import) → `fixity_table[id]` → **`infixl 9` on a table miss**. Store the
DECLARED fixity only; the undeclared default is applied AT CONSULTATION, never
pre-seeded — the declared/undeclared distinction is load-bearing for `D3`
(non-associative) and the `D1`-q3 redeclaration-as-error rule. This is where
[[LANG-INFIX-APPLICATION-DEFAULT]]'s hard-wired `infixl 9` **relocates**, and it
keeps INFIX's default-path fixtures green (`AC-3`).

**`D3` — `infix N` (non-associative) rejects `a <+> b <+> c`.** With a
diagnostic that says so. **This is the arm most likely to be silently omitted**,
because both associative arms have obvious behaviour and this one only shows up
as an error path.

**`D4` — precedence level bounds.** State what range of `N` is accepted and
what happens outside it. `32-grammar.md:373` gives the default as `9`; if the
spec bounds the range, cite it — **if it does not, say so and pick, stating the
choice as a choice.**

> ## VOID + REPAIR 2026-09-07 — first candidate regressed the compile-stack peak (Architect `evt_7rfr3n4867xjr`)
>
> Candidate `65f8a93e` was **void** (§2; the z3930 approval does not carry). The
> lieutenant's pre-publish A/B was decisive: `map_build_acceptance.rs`'s
> `cat4_union_intersection_difference_execute_over_nat` +
> `local_prebinding_preserves_legacy_map_union_stack_budget` are green on
> merge-base `2b85460c2` and deterministically SIGABRT together on the candidate
> — a file OUTSIDE the declared 10-path scope, not a concurrency artifact.
>
> **Mechanism (Architect, grounded at `65f8a93e`):** the candidate added a NEW
> unconditional full-body recursive elaboration pass (`reassociate_rdecl`
> `elab.rs:9471` → `reassociate_rexpr` `elab.rs:9206`, no guard), and arithmetic
> `+`/`*`/`+%` was routed through the SAME spine (`parser.rs:2152-2172` emits
> `EInfixSpine`), so even arithmetic-bearing bodies enter the deep pass. Its peak
> exceeds the legacy compile-stack budget the `map_build` guard calibrates.
>
> **Invariant violated:** zero-cost-when-unused on the shared elaboration path
> (now `AC-7`). **Sanctioned structural levers** (the ring measures + combines;
> the Architect reviews the result):
> - **(A) skip guard — primary.** Do not run the reassociation traversal over a
>   body/decl that contains no `RInfixSpine` — the pass is provably identity on a
>   spine-free tree, so skipping is behavior-preserving. Restores the merge-base
>   peak for all pre-fixity code, including the `map_build` suite.
> - **(B) take arithmetic off the spine (reflect-don't-extend).** Return `+`/`*`/
>   `+%` to direct `EBinOp` via the merge-base cascade so arithmetic-bearing
>   bodies are spine-free; keep
>   `declared_precedence_compares_with_arithmetic_levels` + the arithmetic-order
>   fixtures byte-identical.
> - Frame-discipline (`#[inline(never)]` per-arm split / iterative spine
>   reduction) on `reassociate_rexpr` itself **only if** a genuine USER-operator
>   body still overflows after (A)+(B).
>
> **Prohibited** (Architect + language-leader concur): no `RUST_MIN_STACK`, no
> larger-stack thread wrapping the pass, no relaxing/serializing the `map_build`
> stack-budget test. That guard caught a real regression; it stays as-is and
> must go GREEN.

## Acceptance criteria

**`AC-1`.** `D1` is reported with citations before `D2` is implemented. **A
candidate that implements the table without first reporting the fork has
skipped the deliverable**, even if the table is right.

**`AC-2`.** Declared fixity changes parse shape. **Control:** the same source
text parses to **different** trees under `infixl 5` and `infixr 5`, asserted
structurally. **A value assertion fails this** — a symmetric operator evaluates
identically under both.

**`AC-3`.** An operator with no fixity declaration still parses at `infixl 9`.
**Control:** [[LANG-INFIX-APPLICATION-DEFAULT]]'s fixtures stay green
unchanged. **This node adds a table; it must not make the default path
conditional on one existing.**

**`AC-4`.** `D3`'s non-associative rejection has its own fixture and its own
diagnostic assertion.

**`AC-5` — direction stated.** Declared fixity **changes the parse** of
programs that declare it. Programs that declare nothing are unaffected. **If a
program with no fixity declaration changes meaning, stop** — that is `AC-3`
failing by another route.

**`AC-6`.** No-regression, in CI (`COORDINATION §12`). Targeted locally:
`-p ken-elaborator`. **There is no `ken-parser` crate** — parsing lives in
`crates/ken-elaborator/src/parser.rs`, and name resolution and the
reassociation this node adds live in the same crate. **The affected-closure / QA
scope for this node MUST include
`crates/ken-elaborator/tests/map_build_acceptance.rs`** (the deep-expression /
compile-stack-budget suite): the reassociation pass is a shared full-body
elaboration traversal, so its affected closure extends to deep-expression
acceptance tests OUTSIDE the declared diff scope. Running only the focused
fixity tests is the too-narrow QA scope that let the first candidate reach
publish (`evt_7rfr3n4867xjr`).

**AC-6b — the FULL-CRATE test COMPILE is the mandatory pre-git_request census
(the add-surface exhaustiveness axis).** This node adds surface — a new
`ElabEnv` field, `Expr::EInfixSpine`, `Decl::FixityDecl`. Every **exhaustive
consumer** of that surface (struct destructures, `match` arms in
`lossless.rs`/`resolve.rs`/`modules.rs` and in **test-support modules** such as
`tests/seal2_support/mod.rs`) must be updated or the workspace fails to
**compile** — an `E0027`/non-exhaustive-match error, not a test failure. A
filtered `--test lang_fixity` / `--test map_build` run **never compiles those
other test binaries**, so it cannot see the break; the second candidate
(`eb8303c2`, CI `cargo test --no-run --workspace --locked` exit 101) reached the
publisher on exactly this gap. **The ring's pre-git_request gate MUST run the
crate's full test compile — `cargo test -p ken-elaborator --no-run` with NO
`--test` filter** (single-crate, compile-only, within the targeted-only rule of
`COORDINATION §12`). That command compiles every `ken-elaborator` test binary and
is the **sound census** of exhaustive consumers; repairing only the individually
named sites inherits the blind spot of whoever enumerated them. The full-workspace
`--locked` verdict still runs in CI, but this cheap local compile catches the
class before a git_request spends a Decision.

**`AC-7` — zero-cost when the feature is unused (the load-bearing repair AC).** A
body/declaration that uses **no** fixity/infix surface must pay **no** new
compile-stack cost from the reassociation pass. **Control:** the two
`map_build_acceptance.rs` guards
(`cat4_union_intersection_difference_execute_over_nat` +
`local_prebinding_preserves_legacy_map_union_stack_budget`) — which use no user
operators and predate this feature — compile at the **merge-base** compile-stack
profile and pass, run together, on the candidate. A candidate whose new
traversal raises the legacy stack peak (a SIGABRT or a budget-guard red) fails
this AC. The invariant is zero-cost-when-unused on the shared elaboration path;
the VOID + REPAIR banner above carries the sanctioned levers (spine-free skip;
arithmetic off the spine).

## What this unblocks, and the follow-through that is NOT yours

`conformance/surface/formatting/seed-canonical-format.md` carries
`BLOCKED-ON-USER-FIXITY-SURFACE ([[LANG-FIXITY-DECL-SURFACE]])`, and FMT1's
aggregate names it. **When this lands, that fixture becomes producible and FMT1
loses one of its three blockers.**

**Do not edit `conformance/`.** Say in the handback that the row is now
producible; the spec enclave owns the flip.

## Banned scope

- **Do not re-open whether fixity is owed.** Ruled, `evt_1s7mqjg4tyxx1`, on the
  spec's own words.
- **Do not decide the `D1` fork unilaterally** where the spec leaves it open.
- **Do not change the arithmetic ordering.** Landed, normative, and settled by
  the same sentence that settles this node's existence.
- **Do not touch the kernel or any `trusted_base()` path.** Per `§6`, fixity is
  surface-and-elaboration only: it guides parsing into the same core term the
  kernel re-checks regardless of which path named the operator, so it adds
  nothing to `trusted_base()`. Zero-TCB.

## AT THIS NODE'S MERGE DECISION: the Architect's exact-SHA review returns

He ruled the `D2` mechanism (`evt_4b66p3qb88wnn`) and called it shovel-ready;
its exact-SHA review returns to him. **Steward: he is a required reviewer on
this node's candidate** — route it to him alongside language QA before the merge
Decision. What he will check (fold-recorded so the ring builds to it):

- the reassociation pass emits ordinary `RApp` and the `check`/body path is
  UNCHANGED (elaborator not extended);
- the table is keyed by the canonical `GlobalId`, never a string or alias;
- `infixl 9` on a table miss (default applied at consultation, not pre-seeded);
- `D3` non-associative rejects `a <+> b <+> c` with its own diagnostic;
- `AC-3` holds — undeclared operators still parse at `infixl 9` via the
  relocated default.

**Re-review of the fresh (post-void) candidate additionally checks** (Architect
`evt_7rfr3n4867xjr`): (a) spine-free bodies skip the reassociation pass —
zero-cost-when-unused restored (`AC-7`); (b) BOTH `map_build` guards green AND
the fixity feature intact (14/14 + propagation / conflict / `D3` / `D4` / SCC);
(c) if lever (B) is taken, the arithmetic ordering is byte-identical. **QA's
affected closure MUST include `map_build_acceptance.rs`**, not only the focused
fixity tests. The void approval does not carry (`§2`); a fresh exact-SHA review
is required.
