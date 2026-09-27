---
scope: fleet
audience: (see scope README)
source: private memory `correcting-scope-must-sweep-whole-doc`
---

# Correcting a false claim in a doc must sweep the whole document

This file also carries two sharper forms of the same sweep: amending a frame
mid-flight (sweep the guardrails section first), and folding a review
correction (edit the operative deliverable in place; a later note does not
replace it).

Fixing an over-claim in one section of a doc (e.g. a "SCOPE UPDATE" note added
at the top) does not remove the SAME claim if it also appears verbatim or
paraphrased elsewhere in the same document — a narrow, section-scoped edit reads
as complete but leaves the original wording live for a reader who starts
elsewhere in the file.

**Why:** on `wp/lawful-classes-lane` I added a correct "SCOPE UPDATE" section
noting `DecEq Decimal` re-defers (a real soundness hole), but left the ORIGINAL
frame bullet and a "Deliverables" list item — both further down the same doc —
still asserting the exact over-claim ("real structural proof... bottoms out at
`DecEq Int`/`Num Int` Axiom leaves") the update was supposed to correct.
Architect caught it post-gate: the doc was internally contradictory, and the
un-struck claim was the identical over-claim family just corrected on `main` in
a parallel erratum thread that same hour. Spec-author and conformance-validator
banked the same lesson independently on that parallel thread
("grep-the-region-fold-all... my initial remediation pattern was too narrow and
missed a coupled occurrence").

**How to apply:** after fixing/striking a false claim anywhere in a doc, `grep`
the WHOLE file (not just the section you touched) for the claim's distinctive
terms before considering the edit done. A narrow patch to the symptom you were
told about, leaving sibling restatements untouched, is a half-fix that reads as
complete — worse than an unfixed doc because it looks resolved.

**Sharpening — sweep for the same CLASS of claim, including ones the correction
ITSELF introduces (L3-strings post-close erratum, 2026-07-03).** The whole-doc
sweep for the *same claim* is necessary but not sufficient: a
deliverability-honesty *correction* can introduce a **sibling over-claim on the
same axis** in the very text you're writing. Fixing ADR 0010's "`DecEq Char`
instance landed" overclaim (swept both sites correctly), I wrote "`Ord String`
transports now" — itself an unbuilt lawful instance stated as available, the
identical axis. Architect's independent re-derivation caught it during his
soundness review; folded pre-vote. **Why it happens:** the fixer is primed to
see the ONE instance being corrected and blind to the adjacent claim of the same
kind, especially one they're newly authoring in the fix. **Apply:** after
drafting an honesty correction, re-run the honesty check over the *corrected
text itself* — every instance-claim in it (`X landed` / `Y available now` /
`Z deliverable`), not just the flagged token. This is why the multi-gate
conjunction exists: the fixer's blind spot is exactly what an independent
reviewer nets. (Deliverability-honesty axis: value-level FUNCTIONS shipping now
≠ lawful INSTANCES with law proofs, which are separate follow-ons; keep them
distinct — see trusted by typing guarantee is not kernel proved Q.)

**Sharpening — sweep the whole REPO, and treat a coordinator's named-site count
as a FLOOR not the scope (Map-container retirement, 2026-07-03).** A
cross-cutting retirement/rename/supersession usually contradicts sites across
MANY chapters, not one doc — and the number of sites the coordinator *names* can
undercount the real scope. Steward mandated the `Map`/`Set`-primitive-retirement
sweep but named **3** prose sites (`37`, `50-stdlib/README`, `30-taxonomy`); a
whole-REPO `git grep` of the distinctive tokens (`declare_primitive.*Map`,
`0x07`/`0x08`, `DecEq.-keyed`, the type name) across ALL of `spec/` surfaced **2
more** — and the deeper ones were the *authoritative registries* (kernel
`18a-primitive-registry`, runtime `41-values` kind-tags), i.e. the sites most
load-bearing to leave contradicting, not catalog prose. **Apply:** for any
retirement/rename, grep the WHOLE repo for the claim's distinctive tokens and
reconcile every hit; a named site-list is where to start, not the perimeter.
Reconciling spec *prose* in another chapter (kernel/runtime) to match a ruled
decision is **in-lane authoring** (the soundness owner reviews it at the gate) —
flag the cross-chapter reach, don't silently expand *and* don't leave the
half-fix. (Runtime/Architect independently confirmed both beyond-named-set
reconciles sound + in-WP.)

Sibling of laundered citation authority and named floor must be grepped not
assumed — all three are instances of "a claim's *name*/wording travels
independently of its *truth*, so fixing one occurrence doesn't fix the claim."

## Amending a frame mid-flight? Sweep the GUARDRAILS section — it is the one you forget, and it reads as law

Merged from `amending-a-frame-mid-flight-must-sweep-its-guardrails-section`
(source: CC8 frame, 2026-07-14 — foundation-qa caught it at kickoff; the Steward
wrote the frame, wrote the amendment, and still left two sections contradicting
it).

**CC8 was held for hours while the byte substrate landed underneath it.** When
SUB-1b (lawful `DecEq Bytes`) went in flight, the Steward **amended §3.2** of the
CC8 frame — *"an environment key is a plain `Bytes`, compared with `DecEq
Bytes`"* — and kicked the WP.

**Two other sections still said the opposite.** foundation-qa found one at
preflight; sweeping the doc found a second the QA had missed:

| § | Stale text | Reality at kickoff |
|---|---|---|
| **§6 Guardrails** | *" Do NOT settle `Bytes → Nat`. **No `DecEq Bytes`**… key matching goes through `ArgBytes` + `bytes_at` + `uint8_to_int` + `eq_int`."* | Pat **ruled** it; SUB-1b **landed** `DecEq Bytes`; SUB-2 **deleted** `ArgBytes`. **Every clause false.** |
| **§3.2 item 2** | *"Where CC3's `Cursor` ABI already demands `ArgBytes`, pass it — consume it as-is."* | **`ArgBytes` does not exist.** SUB-2 deleted the class, its cached fields, and its proof obligation outright. **There is nothing to pass.** |

### Why the guardrails section is the dangerous one

**A frame's §"what to build" gets re-read and re-reasoned. Its §"do not do X" gets
OBEYED.** A guardrail is phrased as a prohibition — imperative, absolute, no
argument invited — so a build agent **complies without re-deriving it**. That is
exactly what makes it useful, and exactly what makes a *stale* one poison:

**⇒ A stale guardrail does not merely fail to help. It actively forbids the
correct implementation, in the frame author's own voice, with the frame author's
authority.** CC8's §6 would have sent Foundation to `ArgBytes` — **a class that no
longer exists** — while forbidding the `DecEq Bytes` the WP was un-held *in order
to use*.

### The rule

**When the substrate moves under a held WP, do not patch the section you were
thinking about. Grep the WHOLE frame for every name the change touched** — here:
`DecEq Bytes`, `ArgBytes`, `bytes_at`, `cached-Nat`, `Int → Nat` — **and
adjudicate every hit.** The section you amended is the one you had in mind; the
ones that bite are the ones you didn't.

Same instrument as the whole-document sweep above, but with a sharper
target: **the guardrails/do-not-reopen section is the highest-authority, least-
re-examined prose in the document.** Sweep it first, not last.

### And the corollary for the READER

**foundation-qa caught this at preflight by comparing the frame against the tree,
before writing a line.** That is the whole discipline:

> **Frame text is an INPUT TO AUDIT, not scripture.**

Every frame should carry the perishability clause (*"treat every anchor as
perishable; if a fixed input is FALSE, say so with exact tree anchors and
ESCALATE — do not build around it"*) — **and CC8's did.** It worked. The clause
caught the frame author's own error. **Keep writing it, and keep meaning it.**
Sibling of [[wp-frame-stale-vs-landed-kernel]] (which now also carries the
frame-pseudocode-diverges-from-landed-mechanism lesson).

### Mechanics footnote

If the WP branch is **already checked out in the implementer's worktree**, do
**not** commit the amendment over it from a second worktree — that desyncs them
([[plumbing-commit-onto-held-branch-and-its-desync-risk]]). **Hand the implementer
the verbatim replacement text and have it ride their commit.** Frame authority
stays with the author; the mechanical edit rides the branch holder.

## A later note saying a deliverable is false does not replace the deliverable

Merged from
`a-later-note-saying-a-deliverable-is-false-does-not-replace-the-deliverable`
(source: RT-FNSPLIT-B2V Architect blocks, 2026-07-25; confirmed again
RT-FNSPLIT-B2F, 2026-07-28).

When a reviewer corrects a frame, **edit the operative deliverable in
place**. Appending a clarification that says the earlier text is false does
**not** replace it: both readings now live in the document, and the
**superseded one is the one positioned to be obeyed**, because construction
authority (the deliverable's own table, the AC set) is what an implementer
reads *first* and a clarification hundreds of lines below is read *second,
if at all*.

**Why:** two of the Architect's three blocks on the `RT-FNSPLIT-B2V` recut
(`docs/program/wp/RT-FNSPLIT-B2V-executable-value-abi.md`, 2026-07-25) had
this single cause. `D4`'s table still required four dispositions and
defined *represented immediate* as "payload fits the tagged word directly"
— a definition the recut's own clarification block declared false for
`RepresentedImmediate { spill: Some(..) }`. Separately, RETAIN still froze
the "64/112 layout change" that the recut's promoted wide-`Int` obligation
necessarily changes. Architect, exactly: *"A later note saying the earlier
deliverable is false does not replace the deliverable."* Appending **feels
like faithful transcription of the reviewer's words** and is in fact leaving
the defect operative — that is why it repeats.

### Confirmed again 2026-07-28 — self-authored, correct, and old

`RT-FNSPLIT-B2F`'s correction block at the very TOP of the frame recorded
that `lower_expr` has **61** call sites, not 59. Hours later the
implementer read `AC-5`'s own heading — *"all **59** calls"* — and reported
the frame as defective. **Both numbers were mine; the banner was right;
the reader still got the stale number.**

⇒ **Position does not save a correction.** Even a banner *above
everything* loses to the operative line, because **a reader who goes to an
AC to learn what the AC requires reads the AC** — not the document's
preamble. The banner is a supplement, never a substitute, and *"I already
corrected that at the top"* is not a defence.

**The fix that generalizes: delete the count from the requirement.** The
AC now says *every* call **at whatever count the tokenized derivation
returns on your own base**, and names the derivation as the only pin. ⇒ A
number in a requirement rots on the next merge and each fix mints a fresh
one to rot; **an obligation phrased over a derivation cannot go stale.**

**How to apply:** on every fold, (1) edit the requirement's own text —
replace the table, the AC sentence, the RETAIN bullet; (2) demote the
clarification to *explaining* that text rather than contradicting it; (3)
run a **whole-frame reconcile as part of the fold, not a step after it** —
re-read every deliverable, AC, and RETAIN list and confirm none still
states the superseded contract, then grep the old wording to prove it; (4)
leave the old phrasing only where it is explicitly marked superseded.
Sibling of the
guardrails-section sweep above — same
shape: the header changed, the body did not. The pattern recurs wherever a
frame's status, retain list, or pinned number is edited without sweeping the
prose that restates it.
