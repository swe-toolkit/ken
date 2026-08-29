---
id: RT-COMPOSED-RETURN-PRODUCER-ORDER-BUILD
title: "Implement the shape-(a) producer-authority proof: an opaque, compiler-only proof that joins the already-validated governed transport to the existing confluence key AFTER transport selection and BEFORE emission, and licenses the fresh-result mint with it — closing the composed-return wall's Tail route without a store, captured continuation, runtime tag, or recovery. Builds the probably-viable verdict from RT-COMPOSED-RETURN-PRODUCER-ORDER-DISCOVERY."
status: draft
owner: runtime
size: L
gate: none
tier: T1
depends_on: []
blocks: [PX8]
github: null
origin: "Operator decision 2026-08-29: frame and release the shape-(a) build as lane 1's next objective, after the discovery RT-COMPOSED-RETURN-PRODUCER-ORDER-DISCOVERY returned a decisive verdict of PROBABLY VIABLE (merged b7c0c913f, PR #3105). The Architect approved the constructive sketch as genuinely constructive and prohibition-clean (evt_2se624p3t1eym) and was explicit that the paper approval covers the verdict ONLY — any implementation receives a fresh gate and MUST encode the stated unforgeability. The composed-return wall RT-COMPOSED-RETURN-PRODUCED-TRANSFER (D0b=NO) and the D3 chain stay closed; this is the authority/producer-order construction the wall's disposition named, not a Produced-transfer successor. Steward-filed per COORDINATION section 2."
---

> # HELD — HARD STOP at implementation. Awaiting an operator decision. `draft`
> # (un-released; the schema has no `blocked` — `draft` = held/not-dispatchable).
>
> Released `evt_7f5wx652c0bez`, dispatched (`evt_268yzfg2vmysf`, pickup
> `evt_3vagnf38nwexq`); the runtime implementer hit an immediate hard stop and
> returned home with the branch UNCHANGED — no source edit, no candidate. The
> Architect ruling `evt_6bq9q76rmzm90` (base `1045cae6c`, tree `cfd53fe4`)
> SUSTAINS the contradiction: the private post-selection/pre-emission proof can
> only REFUSE earlier. It cannot alter the already-emitted call/`returned` word,
> and the Tail `Ret` consumer receives `scrutinee.word` unchanged, so it CANNOT
> turn the base-red `ResourceBodyResult` `PatternMatchFailure` rows into
> `InvalidOffset`. **Shape (a) is a viable authority join but INSUFFICIENT as the
> semantic Tail repair** — it flips no Tail partition and closes no PX8 witness.
>
> The Architect's required disposition: HOLD this frame; permit NO
> binding/consumer workaround. TWO forward paths, and the choice is the
> OPERATOR's (a producer-authority hardening increment closes no PX8 witness and
> `blocks` nothing — a priority call, not a Steward sequencing call):
> 1. RETAIN this frame recut STRICTLY as a producer-authority HARDENING
>    increment — keep AC-ORDER, AC-NO-FAIL-OPEN, AC-ONE-AUTHORITY,
>    AC-AFFECTED-CLOSURE; ADD byte-inertness preservation; REMOVE
>    AC-TAIL-PRODUCTS-EXACT, the "0/48 -> producing" claim, `blocks: [PX8]`, and
>    every claim of closing the Tail semantic route.
> 2. DROP it; the semantic repair is a fresh operator decision on shape (b) or a
>    fresh T1 semantic object with an explicit producer-result-to-Tail-Ret
>    binding — no reopening of Produced-transfer / D3 / Direct-only salvage /
>    recovery / store / tag / HS15.
>
> Until the operator rules, this frame is HELD and NOT recut — the recut in
> option 1 is applied only if the operator retains it. The discovery verdict
> `RT-COMPOSED-RETURN-PRODUCER-ORDER-DISCOVERY` carries the matching correction.
>
> The native carried-value campaign COMPONENT front is drained (Architect
> `evt_1xndnw1dp1r6v`); `RT-NATIVE-CARRIED-VALUE` stays product-open and blocks
> PX8 via its ignored full-program rows. This build is the shape-(a) mechanism
> that closes the Tail composed-return route those rows exercise. It is a T1
> soundness-bearing lowering build; the runtime ring's standing hard-stop
> protocol applies.

## The mechanism (Architect-approved sketch, `evt_2se624p3t1eym`)

Re-measure every coordinate at the working SHA — they decay. The two executable
authorities were byte-identical on the discovery's base, candidate, and current
main: `lowering/source.rs` blob `88fcc401b0e078f78298a0998d09364b22e64a27`,
planner `aggregates.rs` blob `9eb2c118e227c3a7db2849e03046db02d93a48eb`.

Forward, coherent order — validation, select, JOIN, producer, mint:

1. The governed arrival validates its exact call key and common fresh-result
   projection (`source.rs:3976-4241`).
2. The existing accessor selects exactly one transport or refuses ambiguity —
   unique-or-refuse, not positional (`aggregates.rs:3411-3436`).
3. Before emitting that transport (select `source.rs:4309`, emit `:4369`), an
   exact confluence-key lookup requires projection equality AND membership of
   that selected transport's own `source_call_identity`. The retained confluence
   class already carries the source-identity member set and common projection
   (`aggregates.rs:414-443`), construction already refuses projection
   disagreement (`:6211-6237`), and plan validation already closes governed
   `(coordinate, member)` pairs against certificate pairs and sanitized installed
   keys (`:6638-6684`). The join adds no catalog — it reads the identity from the
   exact transport and verifies membership.
4. Only that private, opaque producer-authority proof licenses the existing
   fresh-result mint (`source.rs:4373`, `RoutedAnswer::checked(returned)`).

## The load-bearing property: unforgeability (Architect-required)

The producer-authority construction is PRIVATE to the exact join, and the
fresh-result mint has NO fail-open path around the proof. Missing class,
projection mismatch, wrong/non-member transport, or a non-governed arrival at
this producer must refuse BEFORE call emission. This is the property a control
must prove causal, not merely present — a proof neutered to always-succeed, or a
mint reachable without it, MUST redden a control (see `AC-NO-FAIL-OPEN`).

## Deliverable

The mechanism above implemented in the native lowering, plus the controls below.
The Tail composed-return route becomes realizable: the 0/48 Tail partition flips
to producing exact results that agree with the interpreter, while Direct stays
3/3.

## Acceptance criteria

- **AC-ORDER** — the fresh-result mint is licensed ONLY by the producer-authority
  proof, formed AFTER transport selection and BEFORE emission. A mutation that
  mints without the proof, or forms the proof before selection, reddens a
  control. The order stays validation -> select -> join -> producer -> mint.
- **AC-NO-FAIL-OPEN** — neutering the proof (make it always-succeed, or remove
  its requirement at the mint) MUST redden a control: the mint has no path around
  the proof. A control that stays green against a neutered proof is manufactured
  and is a HARD STOP, not a landing (the `RT-RESULT-CLOSURE-LIFETIME` lesson:
  fail-closed and exclusive are different — this AC demands the mint is
  EXCLUSIVELY licensed by the proof).
- **AC-TAIL-PRODUCTS-EXACT** — the Tail composed-return source programs
  (fs-read-at-offset, fs-write-at-offset; the discovery's witnesses) produce
  their EXACT results (e.g. `InvalidOffset`, not merely a changed default),
  agreeing with the interpreter. Direct programs stay green.
- **AC-ONE-AUTHORITY** — no second catalog: the proof reads source identity from
  the exact selected transport, never recovered from the quotient, copied into
  installed access, or guessed by position. The confluence member set is not
  duplicated into a parallel structure.
- **AC-AFFECTED-CLOSURE** — cover every target that loads any module whose
  closure the increment changes, diff-touched or not, widened to `ken run`
  consumers. Not a relaxation of the targeted-build hard rule — what changes is
  which targets count as affected, not how many crates build at once.
- **AC-NO-REGRESSION** — whole-suite green in CI; local targeted `-p ken-runtime`
  / `-p ken-cli` only, never `--workspace`.

## Prohibitions (a candidate needing any of these has left shape (a))

No store, captured continuation, runtime tag/discriminator, or recovery; no
backward token move across the emit-then-validate order; no second catalog,
positional selection, quotient-recovered identity, or `answer_route` promotion;
no new word, parameter, ABI field, receipt, side table, lookup, search, capture
write, or fallback. No Direct-only salvage; no revival of
`RT-COMPOSED-RETURN-PRODUCED-TRANSFER` or the D3 chain. The proof is a private
Rust lowering value, not an emitted field or SSA value.

## Reviewers

Architect — the implemented mechanism matches the approved sketch: the proof is
private to the join, formed after selection and before emission, uses one
retained authority, and the mint has no fail-open path; prohibition-clean. Runtime
QA — the controls red/green as specified, and `AC-NO-FAIL-OPEN` genuinely reddens
against a neutered proof (a manufactured-only control is a hard stop), with the
Tail products exact. A design fork HARD-STOPS to the Architect.

## Capability tier

T1 — a soundness-bearing native-lowering construction reviewed on the provenance
and unforgeability argument, not a differential diff. Size L.

## Sequencing

Lane 1 (runtime), the new objective now that the campaign component front is
drained and the shape-(a) verdict is viable. Released 2026-08-29 per operator. No
`depends_on` — the discovery is merged and the ordering is clear. HS15 stays
unspent; it belonged to the closed endpoint series, not to this build.
