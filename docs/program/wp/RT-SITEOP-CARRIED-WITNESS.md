# RT-SITEOP-CARRIED-WITNESS — a witness for a carried site-bound operand

**A synthesized `SiteOperand` demands a compile-time `Lowered` template from
the same effect seat that byte-span activation wants carried. The two demands
are in direct structural conflict, so 29 quarantined rows and four seats cannot
be discharged by any byte-span mechanism. This node resolves the conflict.**

**Owner:** Team Runtime. **Branch:** `wp/RT-SITEOP-CARRIED-WITNESS`.
**Size:** L — provisional, and it is **not** a sizing you should trust yet; see
§3, the mechanism is unruled and the mechanism sets the size.
**Risk:** medium-high — the touched typing exists to make an unsound
substitution unstateable.

**Status `ready` as of 2026-08-17. §3's fork is RULED** (Architect
`evt_559gymspqap8w`, pasted verbatim below). The gap was always measured; what
was open was the mechanism, and it is now answered.

---

## 1. Fixed inputs

**RE-PINNED at `origin/main = dd7301fc4` (2026-08-17).** The original anchors
were measured at `11bc4c4a`; `RT-CARRIER-BYTESPAN-OBSERVE` `D5` and then
[[RT-BACKEND-PRIMITIVE-LOWERING-SPLIT]] both landed in this region, moving every
one of them by roughly 2,200 lines. The `D5` candidate
`4244d082915bbd6fe154a5e727c6a23c879f1f37` is merged, not held.

| anchor at `dd7301fc4` | what it is | was |
|---|---|---|
| `lowering/mod.rs:13568-13576` | `site_operand_argument` — the sole template projection | `:11354-11362` |
| `lowering/mod.rs:13872-13888` | the `SiteOperand` reconciliation arm and its refusal | `:11640-11667` |
| `lowering/mod.rs:13520-13530` | `SiteOperandWitness` and `site_operand_witness` — what a witness may be | `:11316-11333` |
| `lowering/mod.rs:13517` | why `None` is a refusal rather than a fallback | `:11300-11304` |

**Verified individually by the Steward at `dd7301fc4`, by content and not by
offset** — each was located by grepping its own text, so a further shift
invalidates the number without invalidating the anchor. **Re-pin again at
pickup** and check the derivation against what moved; the line numbers are
recorded to be checked, not trusted.

## 2. The conflict, exactly

`site_operand_argument` calls
`seats.specialized(EffectSeatSlot::Argument(index))?`. That is a hard demand
for the compile-time template. A seat activated to `EITHER_PHASE` may deliver
its value as a boundary word instead, and then `specialized` errors — which is
the refusal, propagated unchanged.

The reconciliation arm is deliberate about this, and its own comment is the
clearest statement of the design intent this node must not break:

> A declared `SiteOperand` whose claimed operand is CARRIED refuses at that
> exact seat, propagated from `specialized`. It does not reconstruct a
> template, widen the carrier, borrow a sibling, or fall back — reconciliation
> needs a compile-time witness, and there is none.

⇒ **The refusal is not a bug and this node must not "fix" it by weakening it.**
The comment at `:11300-11304` says a permissive fallback *"would reopen the
substitution for exactly the variants nobody thought about."* Any candidate
that makes the refusal quieter, rather than making the witness available, is
the wrong shape and should be rejected on sight.

## 3. THE FORK IS RULED — Architect `evt_559gymspqap8w`, 2026-08-17

**The question was:** how does a site-bound operand obtain a witness when its
seat's value is carried?

**The answer is the first of the three sketches, in a specific form: project the
carried word to runtime `(pointer, len)` through an emitted helper and admit
that as the site operand's value** — §2g's ruled emitted-helper route.

> ### THIS SECTION IS THE AUTHORITY FOR THE RULING. Do not paste it anywhere else.
>
> The Architect asked that the ruling be pasted here verbatim rather than cited
> by event id, so the ring does not have to fetch an id to act on it. **That
> makes this frame the single place it lives.** Do not copy it into a code
> comment: `lowering/mod.rs:3506-3509` records that an earlier revision pasted a
> ruling into a comment and thereby created a **second authority** — *"the exact
> defect this chain keeps paying for."* Cite this section, do not restate it.
>
> **Where this frame and `RT-FNSPLIT-C1-operational-carrier.md §2g` disagree,
> §2g governs** — it carries Architect Decision `dec_4te25repm33ph` verbatim.
> The ruling below says the same of itself: *"the frame governs over any
> restatement — including this post."*

### 3a. Coordinate corrections — the ruling's line numbers are 20-25 low

**The ruling's prose is verified correct at `dd7301fc4`; its line numbers are
not.** Every cited site was located by its own text and every substantive claim
checks out. Use this table, not the numbers inside the quotation.

| the ruling cites | actual at `dd7301fc4` |
|---|---|
| `planning/static_transition.rs:5380-5386` | **`:5403-5408`** |
| `lowering/mod.rs:3633`, `:4066` (the ban) | **`:3963-3967`** (refuses-not-converts), **`:6481-6484`** (the no-inverse block), **`:3345`** |
| `lowering/mod.rs:13555` `site_operand_witness` | **`:13530`** (enum at `:13520`) |
| `lowering/mod.rs:3602` `CarriedBoundaryWord` | **`:3499`**, its governing comment at **`:3494-3497`** |
| `lowering/core/tests/effects.rs:3551` | **`:3548`** |
| `specialized_join_arm` (no line given) | **`lowering/mod.rs:4029`** |
| `SynthesizedArgument::into_lowered` (no line given) | **`lowering/mod.rs:13497`** |

### 3b. The ruling, verbatim

> **CAPABILITY RULING — `NATIVE-HANDLE-CARRIER` effect-seat gap.** Grounded in
> the mechanism; every claim below cites the site I read it at. **The hard stop
> was correct and so was refusing to add `int_to_uint64_raw`.**
>
> ## Q1: both offered locations are the wrong one
>
> **`CarriedWord` is the correct representation and §38 stays closed. The
> seat-observation side is ALSO already correct** — this is not a gap on either
> side of the fork as posed.
>
> `D5` (`RT-CARRIER-BYTESPAN-OBSERVE`) **measured the byte-span observation
> succeeding at all four** of `FsReadFile/FsWriteFile/FsChangeMode/FsOpen`
> `Argument(0)`. They were left `Avail::SPECIALIZED_ONLY` **deliberately, and
> explicitly not because the observer fails them** —
> `planning/static_transition.rs:5380-5386`:
>
> > *"LEFT SPECIALIZED_ONLY, and NOT because the observer fails them — `D5`
> > measured it succeeding at all four. Each is the `SiteOperand(0)` of its
> > operation's synthesized `FileError`, so the same seat is read a SECOND time
> > as a compile-time `Lowered` template. Supplying one from a boundary word is
> > the `Carried -> Lowered` inverse this node bans. **Flipping them makes the
> > refusal later and less legible, not absent.**"*
>
> ⇒ **The binding constraint is a second, different consumer of the same
> operand**, and the refusal text names the membership test because that is
> where the check sits — not because that is what must change.
>
> **This is the same trap the ring just stepped around, one layer in.** Widening
> this seat's `Avail` to `EITHER_PHASE` would be a green, well-tested change
> that fixes nothing: the refusal relocates into `FileError` synthesis, later
> and harder to read. It is not even quietly available —
> `ac_4_byte_span_seats_are_activated_exactly_where_d5_proved_them`
> (`lowering/core/tests/effects.rs:3551`) asserts the **whole partition** as a
> normative compatibility vector, so a flip reds there and forces a per-seat
> evidence decision. That control is doing exactly its job.
>
> ## Q2: the mechanism, and what it is not
>
> **It is NOT the banned `Carried -> Lowered` inverse.** The ban's content is
> precise (`lowering/mod.rs:3633`, `:4066`): it forbids **answering a
> compile-time question about a runtime value** — recovering a *template*. Two
> in-tree precedents bound it: `specialized_join_arm` **refuses** rather than
> converts, and the carried-join dual moves `Lowered -> CarriedBoundaryWord`,
> the sanctioned direction.
>
> **The `FileError` site operand does not evidently ask a compile-time
> question.** `site_operand_witness` (`mod.rs:13555`) maps a byte span to
> `Values(pointer, len)` — **runtime CLIF values** — and `Lowered` already has
> runtime-valued inhabitants (`ResponseBytes`, `ResourceToken`,
> `CapabilityToken`, `BorrowedNativeValue`).
>
> **The decisive fact is the carrier's shape.** `CarriedBoundaryWord { word:
> ir::Value }` is a **single** value (`mod.rs:3602`), and its own governing
> comment says field counts and children are *"answered by calling an emitted
> helper at runtime, never by reading a field of this struct — the struct having
> room for a compile-time answer is exactly how the wall would grow back."*
>
> ⇒ **The available direction: project the carried word to runtime `(pointer,
> len)` through an emitted helper and admit that as the site operand's value.**
> That is §2g's **ruled emitted-helper route**, which is the sanctioned
> mechanism by construction — not the inverse. A byte span's pointer and length
> cannot come from reading the word; they were always going to come from an
> emitted call.
>
> ## What I am NOT ruling — and it is the successor's first deliverable
>
> **Two premises I did not walk, stated so nobody treats this as settled:**
>
> 1. **Whether the synthesized `FileError`'s child is read as a template
>    anywhere downstream** — erasure, or the checked-core body view.
>    `SynthesizedArgument::into_lowered` hands it on as a real `Lowered` child,
>    so it is not merely witnessed and discarded. **If any downstream reader
>    takes a template from it, the gap is genuinely representational and my
>    direction above is wrong.** I enumerated the two accessors and the two
>    `site_operand_witness` call sites; I did not walk the child's downstream
>    consumers.
> 2. **Whether minting a runtime-valued `Lowered` from a carrier is itself
>    within §2g/§2h.** That is the frame's to answer, and **the frame governs
>    over any restatement** — including this post.
>
> **Answer (1) before sizing anything.** The two answers differ by a lot, and
> this is precisely the diagnostic-first discipline that just paid for itself.
>
> ## Q3: scope
>
> **This does not belong in `NATIVE-HANDLE-CARRIER`.** The component that must
> change is **synthesized error-value construction and site-operand
> provenance**, not the handle carrier. That node narrows to what its name says;
> this gets a successor.
>
> Agreed the `S` sizing is dead — but **do not re-size the successor until
> deliverable 1 answers premise (1)**, because a plumbing answer and a
> representational answer are not the same node. The cut is yours; I am not
> proposing one.

### 3c. What the ruling does to §2, and it does not soften it

**§2 stands unchanged.** The refusal is still not a bug, and a candidate that
makes it quieter is still the wrong shape. The ruling *sharpens* why: the
refusal text names the membership test **because that is where the check sits,
not because that is what must change**. ⇒ Editing `specialized`, the `Avail`
partition, or `site_operand_witness`'s `None` arm to make the error go away is
now doubly banned — §8 already forbade it, and the ruling shows it merely
relocates the refusal into `FileError` synthesis.

**The third sketch in the original fork — making the conflict unstateable by
constraining the two readers — is NOT the ruled answer.** Do not build it.

## 4. Deliverables

**`D1`/`D1a`/`D1b` are firm and dispatchable now. `D2` onward are cut only
after `D1b` reports** — see the sizing hold below.

> ### THE SIZING HOLD IS THE ARCHITECT'S, AND IT BINDS ME AS WELL AS YOU
>
> *"Do not re-size the successor until deliverable 1 answers premise (1),
> because a plumbing answer and a representational answer are not the same
> node."* **The `size: L` in the node's frontmatter is the pre-ruling
> provisional and is not evidence of anything.** `D1b` is a diagnostic and is
> sized for about an hour; the rest of this node is unsized on purpose.
>
> **Report `D1b` and stop there.** Do not roll into `D2` in the same turn, even
> if the answer looks obvious — the recut is the Steward's and it depends on
> which answer you got.

> ### `D1b` IS ANSWERED: PLUMBING AVAILABLE. `D2` IS CUT IN §4a. Read that, not this.
>
> **Corrected `D1b` at exact `02f255fc1`, reported `evt_5bz715jje5p8s`.** The
> discriminator was decisive: `px7m`'s `Some bytes |-> bytes` returns the bound
> bytes **unchanged**, and the error arm hands them to `write_bytes_then_line`,
> which passes them straight into `Console.write` — **no literal, equality,
> decode, length or other content-sensitive operation.** The disposable
> interpreter control passed with stdout `missing.binnot-found` and exactly
> `[FsReadFile, ConsoleWrite, ConsoleWrite]`.
>
> **Verified at the tree by the Steward**, this time checking the classification
> and not only the coordinate: `write_bytes_then_line (bytes : Bytes)` passes
> `bytes` directly to `write Stdout bytes`.
>
> ⇒ **A runtime-valued `Lowered` suffices for the fixture's downstream use, so
> the emitted-helper projection is NOT refuted.** `D1b` deliberately **selects no
> implementation**; that is `D2`.
>
> **The block below is the superseded question. It is retained because the
> correction is the point** — the first `D1b` traced a correct chain and
> misclassified its terminal step, and both `D1b` runs together are what
> established where the template demand actually sits.

> ### `D1b` RAN 2026-08-17 AND RE-RUNS AGAINST A SHARPENED QUESTION — SUPERSEDED
>
> **The chain below was walked correctly and its terminal step was
> misclassified** (report `evt_2vj52hacadmab`, Architect ruling
> `evt_6f3exyz6we97n`). `constructor_field_bindings` clones a `Lowered` and wraps
> it in the specialized phase — **it never inspects the value**, so it demands a
> `Lowered`, not a template. `Lowered` has runtime-valued inhabitants, so that
> demand is satisfiable by a runtime span.
>
> **The genuine template demand is UPSTREAM and is the refusal itself:**
> `ClaimedEffectSeats::specialized` (`lowering/mod.rs:13434`), *"Read one seat's
> compile-time template."* Its own doc contemplates *"a carried route being
> written for it"* — the opposite of a settled wall.
>
> **THE QUESTION IS NOW PREMISE (2), AND NO WALK CAN ANSWER IT:**
>
> > **Is a runtime-valued `Lowered` — a `ResponseBytes`-shaped span — a
> > legitimate site-operand value, or does `Lowered` in this position mean
> > compile-time-KNOWN content?**
>
> **Tracing shows what is passed; it cannot show what is permitted.** Do not
> answer this by walking the chain again.
>
> **The discriminating test is one fixture.** `px7m_hostresult_computational_
> match.rs` is the right witness, previously read for the wrong thing: **binding
> the path bytes is not the question — what the program then DOES with them is.**
> Consumed in a way that needs their content at compile time (a structural match
> on a literal) ⇒ representational. Only passed on ⇒ a port.
>
> **The original `D1b` text is retained below unchanged**, because the walk it
> produced is sound as a chain and the re-run should not redo it.

- **`D1b` — THE FIRST DELIVERABLE. Answer the Architect's premise (1): is the
  synthesized `FileError`'s child read as a TEMPLATE anywhere downstream?**

  Walk the child's downstream consumers — **erasure and the checked-core body
  view are the two the Architect named**, and the walk is not limited to them.
  The starting point is `SynthesizedArgument::into_lowered`
  (`lowering/mod.rs:13497`), which hands the child on as a real `Lowered`, so it
  is **not merely witnessed and discarded**.

  **The Architect enumerated the two accessors and the two `site_operand_witness`
  call sites and explicitly did NOT walk the consumers. That is exactly the
  work.** Do not re-do the enumeration; start where it stopped.

  **The two outcomes are different nodes, so report which one you found:**

  | if | then |
  |---|---|
  | **no downstream reader takes a template** from the child | the ruled emitted-helper projection is available, and the remaining work is **plumbing** |
  | **any downstream reader takes a template** | the gap is **genuinely representational**, the Architect's direction in §3b is **wrong by its own terms**, and this returns to the Architect rather than to a fix |

  **A negative result here is a real deliverable, not a null one.** State the
  route you walked and what you read, so the negative is falsifiable — a
  zero-hit grep is evidence about a name, and this claim needs to be about the
  mechanism.

- **`D1` — carry the measurement in, do not re-derive it.**
  [[RT-CARRIER-BYTESPAN-OBSERVE]]'s `D5` established the blocker on two
  independent routes, and its candidate is
  reproducible green evidence. Confirm it still holds at your base and report
  what moved; **do not spend a turn re-establishing a settled fact.**
- **`D1a` — the exact population.** Name every one of the 29 rows and the four
  seats, and confirm each one's *measured* cause is this blocker and not
  something that has since diverged. **A row whose cause changed is a finding
  and comes back to the Steward**, not something to absorb.

  **`D1a` ALSO REPAIRS THE ROWS' OWN POINTERS, and they are already stale.**
  Found by the Adversary on merged `1b877875`: the rows say *"awaiting Steward
  recut"*, **but the recut has already happened — it is this node.**
  `rt_parity_native.rs` contains **zero** occurrences of
  `RT-SITEOP-CARRIED-WITNESS`, so a reader who lands on one of these rows
  **cannot reach its live owner from the row.** Make every row name this node.

  **The reasons are otherwise good and must not be rewritten wholesale** — they
  name the measured cause and explicitly disclaim `D5` as the blocker, which is
  better than what they replaced. **Repair the pointer, keep the diagnosis.**
- **`D2` onward — the mechanism.** Cut in §4a below, against the plumbing result.

## 4a. THE RECUT — `D2` against `PLUMBING AVAILABLE` (Steward, 2026-08-17)

**Sizing, now that `D1b` is in: `M`.** The Architect held sizing because *"a
plumbing answer and a representational answer are not the same node."* The answer
is plumbing, so this is a **port**, not a representation change: one projection
route plus the population pass. **The node's `size: L` predates the ruling and is
superseded by this line.**

> ### PREMISE (2) IS DISCHARGED — Architect `evt_tmctzqr3858p`. `D2` may dispatch.
>
> **The hard stop does not fire, and the emitted-helper direction is not
> refuted. It is also NOT endorsed** — non-refutation is not selection, and the
> Architect says so explicitly. `D2` still owns choosing the mechanism.
>
> **The confirmation rests on a CENSUS, not on the fixture, and that difference
> binds what `D2` may assume.** *"A fixture is EXISTENTIAL and the hard stop's
> condition is UNIVERSAL"* — one program consuming bytes by value cannot close
> *"any reader."* So the Architect censused **every** `ConstructorField::
> Specialized` read site at the base: **there are eight, not one.**
>
> | site | what it does | template demand |
> |---|---|---|
> | `:3134` `specialized_at` | returns the payload opaquely; refuses only `StaticWorker` | no |
> | `:3144` `into_specialized_at` | by-value twin | no |
> | `:4840` `constructor_field_bindings` | clones into `LoweringOperand::Specialized` | no — phase rebinding |
> | `:5582` `d9_collect` | pushes CLIF values onto `words` | no — a runtime-word walk by construction |
> | `:7612` unit-boundary rewrite | matches `Lowered::Closure`; non-`Closure` returns unchanged | no |
> | `:10826` `first_boundary_closure_path` | `#[cfg(test)]` diagnostic | no |
> | `:18066` `unwrap_terminal_ret` | unwraps `ITree::Ret` opaquely | no |
>
> **`:7612` is the one that could have refuted this** — it pattern-matches *into*
> the `Lowered`, demanding a specific inhabitant. It does not, because every
> non-`Closure` field falls straight through. **It is also the site a census
> stopping at "the sole constructor-field reader" would have missed. There was
> never one reader, and that phrasing was wrong.**
>
> ### STATE THE RESULT DENOTATIONALLY. "Not required during lowering" is unsound.
>
> **The instrument cannot say that** — an interpreter-only control never lowers;
> it observes denotation. **Carry these words into `D2`:**
>
> > The program's **meaning** never requires the content — the bytes are produced
> > at runtime, selected structurally, and consumed by the host as a runtime
> > value. Therefore a lowering that demands compile-time content **is asking for
> > more than the semantics requires.**
>
> ⇒ That is the bookkeeping side of the denotation/bookkeeping split: **a missing
> port, not correct semantics.** *"Not required during lowering"* invites a
> reader to think lowering was measured. It was not.

- **`D2` — the emitted-helper projection at the refusal site.** Project the
  carried word to runtime `(pointer, len)` through an emitted helper and admit
  that as the site operand's value, at
  `site_operand_argument`/`ClaimedEffectSeats::specialized` — **the upstream
  demand, which is where the template is actually required.** Not at
  `constructor_field_bindings`, which only rebinds a phase and was never the
  constraint.

  **A byte span's pointer and length were always going to come from an emitted
  call** — they cannot come from reading the word. §2g is the sanctioned route
  and §8's bans are unchanged.

  **The refusal this replaces is at `lowering/mod.rs:9639`**, produced by
  `specialized` (`:13434`, doc `:13427`) via `site_operand_argument` (`:13568`).
  **That is a PRODUCER demand, not a consumer one** — the half the original
  falsifier's "downstream" wording excluded.

- **`D2` ALSO FIXES A VOCABULARY DEFECT IN THE SAME FAMILY, and this is a carry
  the Architect asked to be paid by whoever next edits here.**

  **`specialized_at`'s own doc calls the payload a "template"** — *"readers that
  consume the template rather than borrow it"* — while `Lowered` demonstrably has
  runtime-valued inhabitants (`ResponseBytes`, `ResourceToken`, `CapabilityToken`,
  `BorrowedNativeValue`). **The vocabulary asserts a property the type does not
  have.**

  **That wording is what produced this misreading twice in one day, in opposite
  directions.** The `#[ignore]` reason at
  `px7m_hostresult_computational_match.rs:178` inherits it verbatim.

  **No rename is asked for.** Fix the doc wording while you are in this family,
  so the third reader does not pay for it.

- **`D1a` — unchanged from §4, and the obligation I briefly added is WITHDRAWN.**

  > **Withdrawn, stated so nobody reinstates it.** On the fixture result alone I
  > required `D1a` to classify all 29 rows for **content-sensitivity**, reasoning
  > that `px7m` being content-insensitive is a fact about `px7m` and not about
  > the other 28.
  >
  > **The Architect's eight-site census subsumes that.** The compile-time demand
  > comes from the **seat** (`specialized`), not from what a Ken source does with
  > the bytes — a source that compares bytes to a literal lowers to a *runtime*
  > comparison and introduces no compile-time demand. **No reader in the family
  > requires compile-time-known content, universally.** So a per-row
  > content-sensitivity pass would cost a turn and find nothing.
  >
  > **What survives is narrower and was already `D1a`'s job:** confirm each row's
  > *measured cause* is this blocker. **A row whose site-operand value reaches a
  > consumer OUTSIDE the censused `ConstructorField::Specialized` family is a
  > finding** and returns to the Steward — that is the residual the census does
  > not cover, and it is a cause question, not a content question.

## 5. Acceptance criteria

**`AC-0` is the only one `D1b` must meet. `AC-1` and `AC-2` hold whatever the
mechanism; the rest are cut with the recut.**

- **`AC-0` — premise (1) is answered on a walked route, and the answer names
  which of the two outcomes in §4 obtains.** The report cites the consumers it
  walked, by file and symbol, and says explicitly whether any takes a template
  from the synthesized `FileError`'s child.

  **This AC is discharged by a report, not by a green suite**, and it is the
  one deliverable here that cannot be satisfied by a passing test. A candidate
  that changes code has overrun `D1b`.

- **`AC-1` — the refusal still refuses.** The substitution the `SiteOperand`
  typing exists to prevent is still unstateable, demonstrated by a control that
  is **seen to fail** before it passes. **A candidate that only makes the error
  go away has removed a soundness net, not supplied a witness** — this AC is the
  one that discriminates those two outcomes, so it is not optional and it is not
  discharged by a green suite.
- **`AC-2` — the residue is attributed, per row.** Every row this node
  un-skips is green and named; every row it does **not** un-skip carries its
  measured cause. Report **ignored separately from passed**, per file — a bare
  `passed / failed` pair reads green while nothing has been un-skipped.
- **`AC-3` (no-regression).** Workspace green **in CI** — never a local
  `--workspace` run (`COORDINATION §12`).
- Further ACs land with the `D1b` recut.

## 5a. What "done" unblocks — this node is now on the critical path

**It was `blocks: []` and that was wrong from 2026-08-17.**
[[NATIVE-HANDLE-CARRIER]] hard-stopped on this exact gap
(`evt_4eynen6drs79x`), and the Architect ruled the fix does not belong in that
node. So the edge is real and now recorded: this node **blocks
[[NATIVE-HANDLE-CARRIER]]**, which in turn heads **19 transitive dependents** —
`PX8-F-CAP-41` → `PX8` → {`ABI-R3`, `PX9`} → Tracks A/M/S/T.

**Concretely, this is what stands between the tree and a complete Linux ABI.**
That is a reason to get `D1b`'s answer right, not a reason to hurry past it.

## 6. Inherited: the `D6` activation-gate discharge pass

**Moved here from [[RT-CARRIER-BYTESPAN-OBSERVE]] `D6`**, because its premise is
"the activation" and this node is where the activation completes. Its
specification moves verbatim; read it there.

**What is already measured and must NOT be re-derived:** the family-2a sentinel
asserts zero applications and the `ken-runtime` lib suite is green at the `D5`
candidate, so the partial activation did not make the carried source-Match route
executable — **the dormancy premise is intact.** Start from that.

The split-phase rig remains the named producer for the outcome-1 propagation
witness.

## 7. Inherited obligation — a seat activated with no end-to-end row

**`(FsWriteFile, Argument(2))` was activated by `D5` on per-seat evidence:
measured reach and measured observation, with no committed row exercising it
end-to-end**, because its sibling path seat blocks every program that reaches
it. That satisfied `RT-CARRIER-BYTESPAN-OBSERVE.AC-4` as written, which asks for
per-seat evidence.

⇒ **This node is the first that can exercise that seat end-to-end, and the
hazard is one of attribution.** If the activation is subtly wrong, the failure
surfaces inside *this* node's candidate and reads as this node's regression.

**It is not.** A failure at `(FsWriteFile, Argument(2))` traceable to the
activation itself belongs to `RT-CARRIER-BYTESPAN-OBSERVE` — report it as such
and return it to the Steward rather than absorbing it. **Reverting the
activation is a one-line change plus its pin**, and that remains available.

## 8. Banned scope

- **Weakening the `SiteOperand` refusal or `site_operand_witness`'s `None`
  arm** to make the error disappear. See §2 — that is the failure mode, not the
  fix.
- **Rolling `D2` into the same turn as `D1b`.** The recut is the Steward's and
  it depends on which answer `D1b` returns. See §4's sizing hold.
- **Widening the four `Fs*` `Argument(0)` seats to `EITHER_PHASE`.** §3b rules
  it: a green change that relocates the refusal into `FileError` synthesis
  rather than removing it. `ac_4_byte_span_seats_are_activated_exactly_where_
  d5_proved_them` (`lowering/core/tests/effects.rs:3548`) reds on it by design.
- **Building the third fork sketch** — making the conflict unstateable by
  constraining the two readers. It was not the ruled answer.
- **Absorbing a row whose measured cause turns out not to be this blocker.**
  That is a finding and a Steward recut.
- **Re-deriving the `D5` measurement** instead of carrying it in.

## 9. Hard stop

Stop and return the seam if the ruled mechanism turns out to require changing
what a `Lowered` witness *is* for readers other than `SiteOperand`, or if the
29 rows split across more than one cause after `D1a`.

**Added with the ruling: stop if `D1b` finds a downstream template reader.**
That makes the gap representational, which the Architect stated would make its
own §3b direction wrong. **Return it — do not try to route around the reader**,
and do not treat "only one reader, and it looks harmless" as a negative result.
The Architect asked for this premise precisely because it did not walk it.
