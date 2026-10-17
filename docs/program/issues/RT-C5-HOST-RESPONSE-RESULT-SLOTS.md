---
id: RT-C5-HOST-RESPONSE-RESULT-SLOTS
title: "C5 regularity for host replies: make_result fills the type slots of every host-built Result e a with Unknown, so a closed, checked, reflexive cast whose index is a host reply evaluates to Unknown. Derive the slots by evaluating the checked response function at the dispatched op value, never by a per-op table"
status: merged
owner: runtime
size: M
gate: architect
tier: T1
depends_on: [RT-C5-PRODUCER-TYPE-SLOTS]
blocks: []
github: null
origin: "Architect AC-0 ruling on RT-C5-PRODUCER-TYPE-SLOTS (evt_7rtbs5j2730es item 5): host make_result is cut to its own node because its authority is beyond S. Spec 40-runtime/42-evaluation.md §3.6. Steward-filed per COORDINATION section 2."
---

# C5 on host-reply Result slots

## Objective

A host-built `Result e a` carries in its type slots the values a
source-built `Result` of the same checked type evaluates to. A closed,
checked, reflexive cast whose index is a host reply then reduces, as spec 42
§3.6 requires.

## Settled inputs (Architect `evt_7rtbs5j2730es`, on `5386dc871`)

- **The writer.** Host `make_result` (`crates/ken-interp/src/eval.rs`
  ~:5518) fills both `Result` slots with `Unknown`. It has about 25 callers.
- **The authority varies by op.** `Result e a` is given by `fs_resp a op`
  (`prelude.rs` ~:2467; 21 arms across `FileError` and `ResourceError`) and
  `ambient_resp op` (~:2600). The only single authority is evaluating that
  checked response function at the dispatched op value.
- **A per-op table is ruled out.** It would be a second derivation of
  `fs_resp`.
- **The mechanism to reuse.** RT-C5-PRODUCER-TYPE-SLOTS lands
  `checked_type_value` and `type_value_slots` beside
  `primitive_result_type`, with the existing filler as fallback.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The dispatched op's `EvalVal` is threaded through `fs_dispatch` and
`ambient_dispatch` to reify and the `make_result` callers. The `Result`
slots are the evaluated checked response type's parameters, and an
unavailable authority keeps `Unknown`.

## Acceptance

- **AC-0 (probe, then ruling).** At the landed base, show one fs reply and
  one ambient reply whose reflexive cast evaluates to `Unknown`. Name each
  distinct `(e, a)` pair and the threading path. The Architect rules before
  any build.
- **AC-1.** One CaptureHost row per distinct `(e, a)` reaches its value, and
  each has a differing-content negative. A row with the same payload and a
  different error type is false.
- **AC-2 (control).** Restoring the `Unknown` filler returns those rows to
  `Unknown`. `neutral_inductive_type_app_index_does_not_cast` stays green
  and unedited.

## AC-0 ruling (Architect `evt_4tde2bd6a2t4m`, on `dc4f53614`)

The authority is the Vis node's own checked response family, `args[1]`,
applied to the **unpeeled** op. The new `result_reply_slots` computes it in
`run_io_with_effect_recorder` before `peel_coproduct`, and the computed slots
are threaded to `fs_dispatch`, `reify_host_reply_v1` and `ambient_dispatch`,
not the op. Applied to the peeled op the family gives a closure. AC-1 is 14
`run_io` CaptureHost rows: the 12 `fs_resp` pairs, plus (IOError,
ReadResult) and (IOError, Unit). They come with plain-versus-coproduct axis
rows, a preflight Err row, and a differing-content negative each.
(IOError, Bytes) is unreachable at base, and its refusal is measured. The
ruling's code and added stops are operative. The `ambient_dispatch` Unit-arm
wrapping is carried to RT-AMBIENT-UNIT-REPLY-SHAPE.

## Stop conditions

- Any edit to `eq_type_eq` or `cast_reduce`, or any name or spelling lookup.
- Any kernel, `trusted_base()` or spec change (an operator question).
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`.
