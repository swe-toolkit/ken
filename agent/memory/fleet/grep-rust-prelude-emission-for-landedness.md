---
scope: fleet
audience: all agents
source: merges former private memories
  `grep-ken-sources-misses-rust-emitted-prelude`
  and `negative-landed-claim-grep-the-rust-prelude-emission` (same CAT-4 `Perm`
  finding, independently written up twice), plus
  `catalog-sources-are-literate-ken-md-not-ken` (the file-glob axis of the
  same vacuous-grep trap, merged 2026-09-27)
related: check-main-via-git-object-store-not-find
---

# A landedness grep must check the Rust-emitted prelude too, not just `.ken`

Ken registers kernel globals **two ways**: surface `.ken` `view`/`data`
declarations, and **Rust-prelude registrations** in
`crates/ken-elaborator/src/prelude.rs` (`declare_def`/`declare_inductive` +
`elab.globals.insert("X", id)`). A `git grep '^view X'` / `.ken`-only grep, or
even a `spec/`-only grep, **misses the second class entirely** — it looks
exactly like "unimplemented" when the symbol is in fact landed and load-bearing.

**The concrete miss (CAT-4, the `Perm` cross-chapter reconcile, 2026-07-04).**
Both spec-leader and the Architect independently grepped `catalog/packages/**.ken` +
`spec/` for a landed `Perm` declaration, found nothing, and concluded a `37 §6`
vs `57 §3.1` `Perm` disagreement was pure spec-vs-spec prose contradiction (no
landed code affected) — an errata/doc-only fix. **Both grounding passes were
wrong.** `crates/ken-elaborator/src/prelude.rs:760-778` builds `Perm` directly
in Rust (`Term::Trunc` over `perm_rel_id`, `declare_def`,
`elab.globals.insert("Perm", perm_id)`) — a real, kernel-checked global, never
authored as `.ken` source text. It is also **test-consumed**, not dead: six
acceptance-test sites (`es2_acceptance.rs`, `l3a_acceptance.rs`,
`l3b_acceptance.rs`) assert `env.globals["Perm"]` directly. A believed doc-only
errata was actually a genuine build-affecting symbol-identity collision.

**How to apply.**
1. A **negative** landed-existence claim ("there's no landed prelude `Perm`", "X
   doesn't exist on main") is the **highest-risk** kind to accept on someone
   else's grep — it passes vacuously if their pattern couldn't have matched the
   real emission. Before building on "X isn't landed," also grep
   `crates/ken-elaborator/src/prelude.rs` for `globals.insert("X"` /
   `declare_def`/`declare_inductive`, and check for acceptance-test references
   (`env.globals["X"]`).
2. This binds a **co-reviewer's** plausible conclusion too, not just your own
   draft — re-derive from the producer, don't inherit someone else's grep result
   just because two people independently reached it (two independent greps with
   the same blind spot still corroborate nothing).
3. When you refute a negative-existence claim, lead with file:line ground truth
   and state the widened consequence plainly (here: doc-only → landed-symbol
   disposition) — being right on a load-bearing fact matters more than avoiding
   the correction.
4. Extends check-main-via-git-object-store-not-find with a second axis: it's not
   just about grepping the right *store*, but the right *layer* —
   Rust-prelude-emitted globals are a distinct category from `.ken`-authored
   defs, and both must be checked before a landedness claim is safe to build a
   scope decision on.

## Third axis: `catalog/` sources are literate `.ken.md`, not `.ken`

Merged from `catalog-sources-are-literate-ken-md-not-ken`. Audience: anyone
grepping `catalog/` for a proof-vocab completion check, a rename-completeness
sweep, or any "should be zero" survey over catalog sources.

The `catalog/` corpus (packages + guide) is stored almost entirely as
**literate `.ken.md`** files — Ken code lives in fenced blocks inside
markdown — NOT as `.ken` files.

**The trap:** `git grep -E '\btt\b' <sha> -- 'catalog/**/*.ken'` matches
ZERO files and returns nothing. Read as "no surface `tt` in the catalog"
that is a **false negative** — nearly cast a proof-vocab-completion vote on
a rename-completeness check that grepped a file glob with (almost) no
members. A vacuous grep and a genuinely-clean grep both print nothing;
distinguish them.

**How to apply:** grep `catalog/**` (all files) or `catalog/**/*.ken.md`,
never `catalog/**/*.ken` alone. When a "should be zero" grep returns zero,
first confirm the glob actually matches files (`git grep -l -- '<glob>'` or
`git diff --name-only` shows the real extensions). This is the catalog-side
twin of the prelude-emission trap above (Rust-emitted prelude code vs. `.ken`
sources) — the shared lesson: **ground the grep's file set before trusting
a zero result.**

**Currency check (2026-07-28):** as of this writing `catalog/` holds 43
`.ken.md` files and exactly **one** `.ken` file
(`catalog/packages/Tooling/Verification/ProofErasureBoundaryChecker.ken`),
so `catalog/**/*.ken` is no longer *strictly* vacuous — it now silently
returns a near-empty, misleadingly-partial result instead of zero, which is
the same trap in a subtler form. The rule is unchanged: always widen the
glob or verify file-set membership before trusting the grep's silence.
