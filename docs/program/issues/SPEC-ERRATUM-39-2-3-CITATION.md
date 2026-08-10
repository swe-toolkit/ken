---
id: SPEC-ERRATUM-39-2-3-CITATION
title: "Erratum: 34-data-match.md:625 still cites `39 §2.3` for higher-order pattern abstraction, a coordinate the structural-result merge reassigned to Structural-result association"
status: ready
owner: spec-enclave
size: S
gate: none
depends_on: []
blocks: []
github: null
origin: Librarian as-built finding evt_4vfczx1j6b5z5 on merged f9572c27 (PR #1800), measured at 57d4507e. Steward-verified independently against the object store before filing. The same candidate corrected this exact citation class at 34-data-match.md:370 and missed the later occurrence in the same normative document.
---

> # ONE LINE OF NORMATIVE SPEC. DO NOT WIDEN IT.

## The defect

`spec/30-surface/34-data-match.md:625` cites `39 §2.3` as the authority for
**higher-order pattern abstraction**:

```
   `v : T` (higher-order pattern abstraction, `§3.2`, `39 §2.3`), and form the
```

At `57d4507e`, `39 §2.3` is **`### 2.3 Structural-result association`** — the
section `KERNEL-RECURSIVE-RESULT-SURFACE` `D0` introduced. The intended
authority is `39 §2` item 3, *"Type inference & unification — a bidirectional,
Hindley-Milner + ..."*.

**The citation still resolves.** It points at real, adjacent, plausible content
that is about something else entirely, which is why no checker and no reader
notices.

## Why it survived a careful review

`D0` did not overlook the hazard — it fixed the same citation, in the same
file, at `:370`, and the handoff called that deletion out explicitly as
*"preventing the new section from laundering that citation."*

⇒ **The defect is not that the class was missed, but that fixing one member of
a class reads as having fixed the class.** The reviewer sees a deliberate,
correctly-reasoned correction and has no signal that a second instance exists
84 lines further down.

## The fix

Replace `` `39 §2.3` `` with `` `39 §2` item 3 `` at `:625`, matching the
spelling `D0` already used at `:370` verbatim.

## Acceptance

- **`AC-1`** `git grep -n '39 §2\.3' -- spec` returns **exactly two** matches,
  both of which are genuine structural-result references:
  `32-grammar.md:279` and `34-data-match.md:336`. **State the count, not
  "clean"** — the failure direction here is a sweep that reports success
  because it fixed the occurrence it looked at.
- **`AC-2`** The corrected coordinate resolves to the unification item, checked
  by reading `39 §2` item 3, not by assuming.

## Scope

`spec/30-surface/34-data-match.md` only. **One line.**

⛔ Not in scope: renumbering `39`'s sections, adding anchors, introducing a
citation convention, or a repo-wide citation audit. The two `docs/program/wp/`
occurrences of the same class are **already fixed by the Steward** in the
corpus batch that publishes this node — do not re-fix them and do not count
them in `AC-1`, which is scoped to `spec`.
