---
id: CAT-FMT-GATE-CORPUS-ENUMERATION
title: "The frozen-corpus formatter gate enumerates catalog by the `.ken.md` suffix plus one hand-named `.ken` path, and the catalog holds two `.ken` files -- so `FoKripke.ken` is outside the gate and nothing reds. Collect by a predicate over the suffix set instead, and assert the collected population EQUALS the matching file set rather than merely being non-empty."
status: draft
owner: verify
size: S
gate: none
tier: T2
depends_on: []
blocks: []
github: null
origin: "Architect finding evt_6vknv4sv5dh5z, filed by the Steward per COORDINATION section 2. Surfaced while reconciling AC-7's corpus measurement on LANG-ATOM-START-CLOSURE-POSITION-COVERAGE: the same .ken.md-suffix blind spot appeared in three instruments within one hour -- the implementer's census, the Architect's confirming check, and this CI gate. The first two produced a wrong sentence in a channel post and were caught inside the hour; this one is load-bearing in CI and has been silently wrong for as long as FoKripke.ken has existed."
---

## The defect, measured at `b53dd9fcf`

**The gate collects its catalog population by the `.ken.md` suffix and then
patches in one `.ken` file by explicit path. The catalog contains two `.ken`
files. One of them is not in the gate.**

`crates/ken-cli/tests/ken_fmt.rs`, inside `strict_frozen_corpus_gate_is_green`:

```rust
fn strict_frozen_corpus_gate_is_green() {
    let repository = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut catalog = Vec::new();
    collect(&repository.join("catalog"), ".ken.md", &mut catalog);
    let mut rosetta = Vec::new();
    collect(&repository.join("examples/rosetta"), ".ken", &mut rosetta);
    catalog.sort();
    rosetta.sort();
    assert!(!catalog.is_empty(), "catalog corpus must not be empty");
    assert!(!rosetta.is_empty(), "Rosetta corpus must not be empty");

    let boundary =
        repository.join("catalog/packages/Tooling/Verification/ProofErasureBoundaryChecker.ken");
    assert!(boundary.is_file());
```

The catalog's `.ken` sources, enumerated at `b53dd9fcf`:

    catalog/packages/Tooling/Verification/
        FoKripke.ken                        NOT IN THE GATE
        ProofErasureBoundaryChecker.ken     named explicitly, by path

**The sharpest part is three lines apart, inside one function body.** The gate
already collects `.ken` **by suffix** — for `examples/rosetta`. For `catalog` it
uses `.ken.md` and then names a single `.ken` file by path. **The capability is
not missing; it is used immediately above and not used below.** This is an
inconsistency within one test body, not an absent facility.

`.ken.md` does not end in `.ken`, so the two suffixes are disjoint and one
`collect` call cannot serve both. The fix is a second call, exactly as the
function already does across two directories.

## Why it fails silently, and in the admitting direction

⇒ **A ROSTER MAINTAINED BY EXPLICIT NAME BESIDE A RULE THAT ALMOST COVERS IT
FAILS SILENTLY AND IN THE ADMITTING DIRECTION.** The population is a suffix rule
plus a hand-maintained exception list, **and the exception list is already one
short.** The next `.ken` file added under `catalog/` is outside the formatter
gate by default, and nothing reds.

**The existing controls cannot catch it.** `!catalog.is_empty()` and
`!rosetta.is_empty()` are **presence** oracles. Neither is a completeness
oracle, and completeness is the property at issue — a gate that collects 53 of
55 sources passes both assertions.

## Scope — stated narrowly

**`FoKripke.ken` is NOT unparsed.** It is covered by elaboration tests
(`crates/ken-elaborator/tests/lang_truncation_surface_syntax.rs:454`,
`env.elaborate_file(FOK_SOURCE)`, plus further consumers). **This is a
formatter-gate hole, not a parse hole.**

**Consequence for the node this was found from:** the structural argument now
carrying AC-7 on `LANG-ATOM-START-CLOSURE-POSITION-COVERAGE` — *every catalog
source passes a green parse gate, so a corpus that parses contains zero
constructs the parser rejects* — is UNAFFECTED, because `FoKripke.ken`'s parse
coverage comes from the elaboration tests rather than from this gate. Do not
read this node as reopening that argument.

## Remedy direction

**Collect by a predicate over the suffix set `{.ken.md, .ken}` rather than
suffix-plus-exceptions**, mirroring the rosetta call. The explicit `boundary`
path then becomes redundant, **and its redundancy is the acceptance signal** —
if removing that line leaves the gate green with both `.ken` files in the
corpus, the enumeration defect is closed by construction rather than by a longer
list.

## Acceptance criteria, with their controls

- **AC-1.** The gate's collected catalog population is asserted **equal to** the
  set of files under `catalog/` matching either suffix — not merely non-empty.
- **AC-2.** The explicit `boundary` path is REMOVED, and the gate stays green
  with both `.ken` files in the corpus. Removal is the acceptance signal; a
  candidate that keeps the line and adds `FoKripke.ken` beside it has lengthened
  the list rather than closed the defect.
- **AC-3 (control, REQUIRED).** Add a throwaway `.ken` file under `catalog/` and
  confirm AC-1's assertion **REDS**; then remove it. **An enumeration bug is
  invisible to any test that only checks the list it already has**, so a control
  that does not add a file cannot exercise this property.
- **AC-4.** State whether `FoKripke.ken` was already `ken fmt --check`-clean when
  it entered the gate. If it was not, that is a finding to report, not a licence
  to exclude it — and the repair is its own increment.

## Why this is a node and not a note

**Three instruments shared this exact blind spot within one hour:** the
implementer's AC-7 census, the Architect's independent confirming check, and
this gate. The first two produced a wrong sentence in a channel post, cost one
review round, and were caught the same hour. **The third is load-bearing in CI
and has been wrong silently for as long as `FoKripke.ken` has existed.**

That asymmetry — same defect, three hosts, one of them a gate — is the argument.
**The two cheap instances are the evidence that the expensive one is not a
fluke.**

## Ownership note (Steward)

Filed to **verify**: the defect is a CI gate's completeness property and the
remedy touches `crates/ken-cli/tests/`, not the formatter itself. The `CAT-`
prefix reflects the corpus under measurement, not the owning ring. If the verify
ring reads this as formatter-surface work, say so and it re-homes to language —
that is a routing call, not a re-scope.
