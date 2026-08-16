---
id: V3-FO-SUBST-DEPTH-CONTROL
title: "Give subst_form_at a control that can see its binder-depth discipline, and name the call-site correspondence the shift oracle silently depends on"
status: ready
owner: language
size: S
gate: none
depends_on: [V3-FO-GUARD-SHIFT-DIFFERENTIAL]
blocks: []
github: null
origin: "Steward, 2026-08-16, dispositioning Adversary hunt evt_d1wy8d6kytpw on the merged range 790c16ea6..197374712. The hunt answered a question the Steward handed it on V3-FO-GUARD-SHIFT-DIFFERENTIAL's merge -- whether the depth<=1 refutation shape generalizes -- and it does. Every coordinate re-verified by symbol against origin/main b03c1084b before filing. Steward-filed per COORDINATION section 2."
---

## The finding, measured, and it is not the same size as its predecessor

**`subst_form_at` raises `depth + 1` at `ForallWorld` and `ForallObj`
(`fo_kripke.rs:921-922`). Mutating both to a literal `1` leaves 19 tests green
across three targets** — `--lib fo_kripke` (8), `v3_fo_kripke_slice_acceptance`
(6), `v3_fo_obligation_signature_discovery_acceptance` (5). Adversary
`evt_d1wy8d6kytpw`, probe reverted, worktree clean.

**The code is correct. The controls cannot tell that it is.** The mutant
diverges from correct **only at the second nested quantifier below a
substitution point** — `subst_form_at(form, 0, ..)` passes `1` at the first
binder either way — so a corpus that never nests two deep below a `ForallRight`
is **blind by construction.**

> ### THIS IS WORSE-PLACED THAN `mentions_var0`, AND THAT IS THE REASON TO FILE IT
>
> `mentions_var0` is a **capture guard whose failure costs a refusal.**
> `subst_form_at` is on the **certificate-CHECKING** path: `check_tree`'s
> `Rule::ForallRight` arm calls `subst0_form(body, eigen)` at `:856` to
> instantiate a quantifier body before the child sequent is checked. **Its
> depth discipline decides which bound occurrences the eigenparameter replaces
> — that is, whether a certificate is ACCEPTED.**

## Why 19 tests are blind. There are TWO reasons and only one is depth.

**Reason 1, the Adversary's: the rows are shallow.** Same boundedness that got
`14204d028` refused, one function over.

**Reason 2, found on re-check and it is the one that changes the deliverable:
the producer and the checker share the function.** `subst0_form` has exactly two
callers:

| site | role |
|---|---|
| `fo_kripke.rs:856` | `check_tree`'s `ForallRight` — **validates** a certificate |
| `fo_kripke.rs:993` | `search`'s `ForallRight` — **constructs** the certificate |

⇒ **A depth error is applied identically on both sides, so the certificate the
search builds is exactly the one the checker expects.** They cannot disagree.

> **Any test shaped "discovery succeeds, `find_certificate` returns `Some`,
> `check_cert` accepts" is STRUCTURALLY incapable of detecting this**, at any
> depth, forever. **Adding deeper end-to-end rows would not have caught it** —
> and end-to-end rows are what the three acceptance targets are made of.
>
> ⇒ **The control must be a DIRECT oracle on `subst_form_at`, not a route
> through search-then-check.** Deepening the corpus is the repair that looks
> right and does nothing.

## SIZING. This is LATENT, and I am stating it so nobody over-escalates.

**The FO route withholds `Proved`.** `attempt_fo_with_signature` returns
`emit_unknown_hole_fo_withheld` on quote-find-check success
(`prover.rs:597`, definition at `:800`) rather than reaching `attempt_ipc`. **An
accepted-but-wrong certificate cannot produce a `Proved` today.**

⇒ **The gap goes live exactly when the withhold is lifted** — when
`embedding_adequacy` and `checker_soundness` get a kernel-checked home. **That
is the same trigger as [[V3-FO-KEN-LEVEL-CHECKER-AUTHORING]]**, and this node is
cheap insurance bought before it, not a live soundness hole.

**Do not import urgency this sizing does not support, and do not use the
latency to defer it either** — the control costs a handful of rows now and
costs an investigation later.

## `D0` — the direct oracle, with the row that discriminates

**Add direct unit rows for `subst_form_at` with hand-written expected output**,
in `fo_kripke.rs`'s own `mod tests` (the function is private to the file, so
that file's tests are its complete population — the same closure that made the
predecessor's hole invisible).

**The discriminating row, derived rather than left to be found:**

```
subst_form_at( ForallObj(ForallObj(Bound(2))), 0, Parameter(p) )
```

| | inner call receives | `Bound(2)` resolves to |
|---|---|---|
| correct (`depth + 1`) | depth `2` | `Parameter(p)` — the replacement |
| mutant (literal `1`) | depth `1` | `Bound(1)` — `2 > 1`, so decremented |

**Two binders deep is the minimum that discriminates**, and the shallow row
`ForallObj(Bound(1))` at depth 0 gives `Parameter(p)` under both — which is why
every existing row passes. **Cover `ForallWorld`, `ForallObj`, and a mixed
nesting**, and include the shallow row explicitly so the boundary between what
does and does not discriminate is on the record.

**The expected `Form` is written by hand.** An expectation computed by calling
the function, or by routing through `search`, reintroduces reason 2.

## `D1` — name the call-site correspondence in the `mentions_var0` oracle's doc

**`V3-FO-GUARD-SHIFT-DIFFERENTIAL`'s `D2` names two mechanisms — the underflow
guard for depth-0 rows and ordinary cutoff arithmetic for binder rows. Both are
properties of `shift`. Neither is a property of the CALLER.**

The oracle pins `mentions_var0` to `shift`. **Its relevance rests on
`quote_iform` performing exactly `shift(codomain, -1, 0)` (`:585`).** Change
that call site — a different cutoff, a `subst_var`, a local helper — and **the
differential keeps passing while the guard no longer describes the operation
performed.**

> ### THAT ASYMMETRY IS THE WHOLE POINT
>
> **Break either named mechanism and the differential REDS. Break the call-site
> correspondence and nothing moves.** `D2` exists so that a future break in
> `shift` is legible rather than a regression to chase, **and the one illegible
> break is the one it does not name.**

**A second production site already has to agree:** `denote` at `:482` does
`shift(&denote(env, sig, q), 1, 0)`, documented as the inverse of `quote_fo`'s
`shift(codomain, -1, 0)`. **Two production sites hold one convention and the
doc describes one.**

**Deliverable: one clause in that doc comment naming the call-site
correspondence as a premise of the oracle.** A test could assert it; a stated
premise is enough, because legibility is what `D2` is for.

> **A third `shift(&px, 1, 0)` exists at `:1023` in `positive_control_term`.
> That one is a FIXTURE builder, not the quote/denote convention** — a break
> there changes what the control tests rather than what the guard describes.
> **Do not fold it into the clause**; naming it would make the premise vaguer,
> not stronger.

## Acceptance criteria

**`AC-1`.** **The `D0` rows discriminate, demonstrated by mutation.** Apply the
Adversary's exact mutation — `depth + 1` to a literal `1` at both
`fo_kripke.rs:921-922` — and show the new rows **red**, then revert. **A count
of zero is worth exactly what the demonstration that the instrument would have
seen one is worth.**

**`AC-2`.** **The oracle is direct.** No `D0` row establishes its expectation by
routing through `search`, `find_certificate`, or `check_cert`. **Reason 2 above
is the finding; a control that inherits it is not a control.**

**`AC-3`.** **The shallow row is present and passes under both arms**, recorded
as the boundary rather than omitted as uninteresting.

**`AC-4`.** **`D1` is a doc clause only.** No change to `mentions_var0`,
`quote_iform`, `denote`, `shift`, `check_tree`, `check_cert`, or conjunct 3.

**`AC-5`.** **No FO `Proved`, no slice widening, no primitive, no trusted
axiom.** The withhold stays exactly as it is — **this node does not lift it and
must not be read as preparation for lifting it.**

**`AC-6`.** No-regression, in CI (`COORDINATION §12`). Local validation is
targeted only — `-p ken-elaborator`, never `--workspace`.

## Banned scope

- **Deepening the end-to-end acceptance corpus as the repair.** It cannot
  detect this defect at any depth. Add rows there if they are wanted for their
  own sake, **not as a discharge of `AC-1`.**
- **Changing `subst_form_at`.** It is correct. **This node builds the control
  that can see that it is.**
- **Auditing every other predicate in the crate for bounded populations.** That
  is a real question and it is the Adversary's standing hunt, not this node's
  deliverable.
- **Lifting or narrowing the FO withhold.**

## Sequencing

**Lane 2 (operator priority), queued behind `V3-FO-DISCOVERY-BOTTOM-OVERCOLLECT`
rather than concurrent with it** — both touch `fo_kripke.rs`, and that node's
candidate `a92364e2e` is QA-approved and awaiting Architect review at filing
time. **Release this one after it lands**, or accept the contention knowingly.

**Nothing blocks on this node**, and nothing gates on its landing.

## Do not re-run these

**Checked by the Adversary and holding:** `Term` derives `PartialEq`
structurally with no manual impl, and `Term::pi`/`lam`/`app` are pure
constructors doing no normalization. **Both would fail loudly if they stopped
holding**, which is why they were not named as mechanisms in `D1`'s clause.
