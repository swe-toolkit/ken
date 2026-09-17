---
id: SPEC-TYPE-PROJECTION-SURFACE-NORMATIVE
title: "specify the type-position projection form normatively now that LANG-TYPE-PROJECTION-SURFACE-FORM has landed it: the admitted production is a CHAIN (`ident ('.' (ident | ConId))+`, left-associative, only the OUTERMOST field becoming the type node), the base is a VALUE binder rather than a type so the restriction to a binder-rooted chain is itself normative, and the two rejection cases are ASYMMETRIC -- positional `.N` in type position is a specified refusal with a located diagnostic and accepting it is non-conforming, while expression-position `d.Query` is claimed by no production at all and later admitting it is NOT non-conforming. Also discharges 58b's `unspellable` prerequisite clause at four sites and restates the catalog-zero's REASON without changing its value."
status: active
owner: spec
size: S
gate: none
depends_on: [LANG-TYPE-PROJECTION-SURFACE-FORM]
blocks: [LANG-MEMBERSHIP-OPERATOR-SURFACE]
github: null
tier: T1
origin: "Steward cut 2026-09-17, filing a node for work already in candidate -- `spec-author` flagged its absence at AUTHORING time (evt_2ng4xxdcc62wx) rather than at routing, which is the repeat of the gap caught on SPEC-32-PROJECTION-PRECEDENCE. The node is the Steward's to create and was missing because the work was dispatched in-thread off the landing of LANG-TYPE-PROJECTION-SURFACE-FORM (294cb5e28843dff8e2edeae9945e1cdc20b0318f) without a framing step. Candidate 006c8fb00c689b341065db5ee3edf85f800615c8 on wp/SPEC-TYPE-PROJECTION-SURFACE-NORMATIVE, cut from origin/main 294cb5e28, 3 files +79/-19, zero crates/, under spec-leader review at filing time."
---

> # FILED AFTER THE CANDIDATE EXISTED. That is the defect this node records.
>
> **`spec-author` flagged the missing node while authoring, not at routing**
> (`evt_2ng4xxdcc62wx`), which is the whole improvement: the same gap was caught
> on [[SPEC-32-PROJECTION-PRECEDENCE]] only when the candidate reached the
> router, and at that point the work is finished and the node is a formality
> written backwards from a diff. **A node filed at authoring can still shape the
> work; one filed at routing can only describe it.**
>
> **Creating nodes is the Steward's, so the gap is the Steward's.** The trigger
> that was missed: work dispatched in-thread, off a landing, without a framing
> step in between.

# What the spec must say, and why each of the three was NOT obvious

All three came from grounding against the landed tree rather than from the
brief, and **each would have been a wrong edit taken from the brief alone**
(`spec-author`, same event).

## 1. The admitted form is a CHAIN, not a single step

The brief said `ident '.' (ident | conid)`. **The parser loops.** `d.a.Query` is
admitted, left-associative, and **only the outermost field becomes the type
node** — the inner steps project values.

    ident ('.' (ident | ConId))+

## 2. The two rejections are ASYMMETRIC, and writing them alike forbids an
## extension the implementers deliberately left open

| form | tree behaviour | normative status |
|---|---|---|
| `d.1` in TYPE position | explicit refusal, located diagnostic covering the projection: *"positional projection `.N` is not available in type position; name the field instead"* | **accepting it is NON-CONFORMING** |
| `d.Query` in EXPRESSION position | **no diagnostic at all** — claimed by no production, not ambiguous, not reserved | **later admitting it is NOT non-conforming** |

**"Not built for want of a consumer" and "specified rejection" are different
claims and the language lane described the first.** The tree does the second: it
forbids `d.1` in a type. Writing both cases as *"rejected"* would have closed a
door the implementers want openable when a consumer appears.

## 3. The base is a VALUE binder, and that restriction is normative

`Type::TProj` holds a `Box<Expr>`, not a `Box<Type>` — **the projected object is
a value (`d : Membership c`) and only its field is a type.** The sole producer
emits `EVar` wrapped in `EProj`s, so the base is a **binder-rooted chain**: no
operator spine, no application, no refinement.

⇒ **This is load-bearing, not tidiness.** A structural consumer
(`type_contains_effect_row`) answers from that narrowness **in the permissive
direction**. Recording the restriction as normative means a future `(e).field`
owes those consumers a re-derivation instead of silently inheriting today's
answers.

# The `58b` bookkeeping, stated so it is not re-opened

- **The count of three does NOT change.** Three counts *bindings whose types
  project*, not obstacles; it is still three. What retires is the prerequisite
  clause around it.
- **Four sites in `58b`, not two.**
- **`§5`'s "unspellable" is a DISCHARGE, not a retraction.** §5 predicted the gap
  would close and it has. **Recording it as a correction would misreport a
  successful prediction as an error** — the distinction is the point.
- **`§5`'s catalog zero is RETAINED** — re-measured at 0 across all 39 `.ken`
  files, positive control 28 files carrying expression-side projection — **but
  its REASON is restated.** The same zero now means *"no consumer authored yet"*
  where it used to mean *"unspellable"*, and **a reader carrying the old reason
  concludes the form is still missing.** A number whose value survives while its
  meaning inverts is the failure mode being guarded here.

# Related

- [[LANG-TYPE-PROJECTION-SURFACE-FORM]] — the implementation this specifies,
  landed `294cb5e28843dff8e2edeae9945e1cdc20b0318f`. Its own node reads `ready`
  pending M7; that is stale status, not unlanded code.
- [[LANG-MEMBERSHIP-OPERATOR-SURFACE]] — unblocked by the landing. Its banner
  carried the now-false *"UNSPELLABLE"* premise and has been repaired.
- [[SPEC-MEMBERSHIP-CLASS-CONTRACT]] — supplies the three projection-typed
  bindings.
- [[SPEC-32-PROJECTION-PRECEDENCE]] — where the missing-node gap was caught late.
