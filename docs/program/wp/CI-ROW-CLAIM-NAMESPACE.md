# CI-ROW-CLAIM-NAMESPACE

Owner: **verify**. Size **S**. Gate: none. Depends on: nothing.
**Sequence after `CI-DOCTEST-UNEXECUTED`** — same ring, one node at a time.

## 1. Objective

`verify-row-claims` hardcodes `surface/` in both its claim and heading
patterns, so eight of nine `conformance/` namespaces are invisible to it.
Widen it to the namespaces that actually exist, without re-admitting the
file-path citations a previous census excluded.

## 2. Fixed inputs, measured at `origin/main = 376d495d`

**2a. The two patterns**, `scripts/ci-ignored-sweep.py:36-37`:

```python
ROW_CLAIM_RE   = re.compile(r"^\s*//(?:/)?\s+(?P<row>surface/\S+)")
ROW_HEADING_RE = re.compile(r"^###\s+(?P<row>surface/\S+)(?:\s.*)?$")
```

**2b. `conformance/` top-level namespaces, nine of them**: `behavioral`,
`challenge`, `fs`, `kernel`, `runtime`, `security`, `stdlib`, `surface`,
`verify`. The checker governs `surface` alone.

**2c. The reproduction, from the conformance-validator.** Four
`/// runtime/evaluation/<id>` claims attached to real `#[test]` functions,
measured on an untouched detached base worktree **and** on the edited tree:
both report exactly **30**, expected **34**. The attachment form was correct;
the namespace was excluded. **That A/B is why the cause is grounded rather than
inferred** — a single reading of 30 is also consistent with mis-attachment.

**2d. The population that must stay excluded.** File-path citations in doc
comments (`spec/30-surface/35-numbers.md`,
`conformance/surface/numbers/seed-numbers.md`) are **not** row ids, and under a
pattern loose enough to span namespaces they can be mistaken for them.

> **Corrected 2026-08-10, Steward error.** This section previously asserted that
> `CI-ROW-CLAIM-COMMENT-FORM`'s census found **29** such `//!` citations. **That
> attribution was wrong** — at `376d495d` the 29 describes attached
> `/// surface/...` row claims, and the `//!` claim-form count there is **0**.
> Verify measured 83 `//!` lines / 86 path tokens / 27 body-begins-with-path on
> the rebased tree, none of which is 29. **Carry no count from this frame**; see
> the replacement `AC-3`.

**Two measured facts that separate the populations** (Steward, on `379bc0f4`):

- **Structural.** A filesystem-derived namespace set holds the directories under
  `conformance/` — `behavioral`, `challenge`, `fs`, `kernel`, `runtime`,
  `security`, `stdlib`, `surface`, `verify`. It does **not** hold `conformance`
  or `spec`, so any citation written with its leading directory fails to match
  **by construction**.
- **Suffix.** For a citation written namespace-relative (a bare
  `surface/numbers/seed-numbers.md`), **zero of the 825 `### ` row headings
  under `conformance/` end in `.md`, and every file-path citation does.**

**2e. Baseline resolved count: 30**, on `main` at `376d495d`. **Do not carry
this number** — see `AC-4`.

## 3. Deliverables

**D1 — Derive the namespace set from the filesystem.** The claim and heading
patterns must accept any namespace that exists as a directory under
`conformance/`. **Do not hardcode a list of nine** — that is the same defect
with a longer literal and it drifts the first time a namespace is added.
**Do not use a permissive `\w+/` pattern** — see `AC-3`.

**D2 — Report the new resolved count**, measured on the delivered tree. **No
predicted number from this frame.**

**D3 — Hold and report anything the widening surfaces.** Eight namespaces have
never been checked. If widening reveals unresolved claims, **report them with
`file:line` and stop**; do not fix them, do not author `conformance/` rows, and
do not narrow the population to get green. That disposition is what produced
this node's two predecessors and it is the standing rule.

## 4. Acceptance criteria

**AC-1 — The gap is closed in the informative direction.** A fabricated id in a
**non-`surface/` namespace**, attached to a real `#[test]`, must **red**, naming
the test and the id. Restore byte-identically, re-run green. **A `surface/`
control proves nothing here** — that path already worked.

**AC-2 — Both directions, on a real non-`surface/` claim.** A non-`surface/`
claim that **resolves** must pass. Without it, the widening is satisfiable by a
checker that rejects every non-`surface/` claim. If
`CONF-EVAL-COMPUTED-BOOL-ELIM`'s `runtime/evaluation/` rows have landed, they
are the live positive control; if not, construct one and restore it.

**AC-3 — No file-path citation is ever counted as a row claim.** On the
delivered tree, enumerate the set of tokens the checker's claim pattern matches,
and assert that **every member is a row id and none is a file path**. Discharge
it against both facts in §2d: show that `conformance` and `spec` are not members
of the derived namespace set, and that **no matched claim token ends in `.md`**.
Report the set size before and after — **those numbers are outputs, not
targets, and no count is carried into this criterion.**

**This is the criterion most likely to fail silently**, because admitting
citations raises the total and looks like success. If a `### ` heading ending in
`.md` exists, the suffix discriminator is unsound — **report that rather than
working around it.**

**AC-4 — No count carried.** The only number here is 30, at `376d495d`.
Re-measure and report what you get.

**AC-5 — The `surface/` population is unchanged.** All 30 existing resolved
claims still resolve. A widening that perturbs the working namespace is a
regression regardless of what it adds.

## 5. Scope

**In:** `scripts/ci-ignored-sweep.py`, `scripts/test-ci-ignored-sweep.py`.

**Out, and these are bans:**

- **No `conformance/` authoring.** Ownership boundary.
- **No production code changes.**
- **No adequacy creep.** Resolution only — a claimed id resolves to exactly one
  `### <id>` heading. Whether the test *covers* the row is human judgment and is
  not this node's.
- **No narrowing to reach green.**

## 6. Contention

`scripts/ci-ignored-sweep.py` — **`CI-DOCTEST-UNEXECUTED` does not touch it**
(that node writes `.github/workflows/ci.yml`, `crates/ken-runtime/src/values.rs`,
and four comment sites), so the files are disjoint. The sequencing constraint is
the ring, not the paths.

`CONF-EVAL-COMPUTED-BOOL-ELIM` is authoring the `runtime/evaluation/` rows this
node's `AC-2` wants as a positive control. **Re-derive at pickup whether they
have landed** and say which control you used — do not assume either way.

> **Framing note.** Three nodes have now widened this one checker along three
> different axes — executing cover, comment marker, namespace — and each hole was
> invisible until someone wrote the first artifact that fell in it. **The
> recurring question is not "is the pattern right?" but "what population has
> never been exercised?"** If a fourth axis occurs to you while building this,
> report it rather than widening speculatively.
