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

**2d. The population that must stay excluded.**
`CI-ROW-CLAIM-COMMENT-FORM`'s census found **29** `//!` file-path citations
(`spec/30-surface/35-numbers.md`,
`conformance/surface/numbers/seed-numbers.md`) that are **not** row ids. They
are indistinguishable from ids under any pattern loose enough to span
namespaces.

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

**AC-3 — The excluded population stays excluded.** After the widening, the 29
file-path citations of §2d must **still not** be counted as claims. State the
measured citation count before and after. **This is the criterion most likely to
fail silently**, because admitting them raises the total and looks like success.

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
