---
id: LANG-TRUSTED-BASE-LABEL-KIND-TAG
title: "The `AC-6` trusted-base enumeration is blind to the one movement it exists to catch -- `trusted_base_labels` flattens kernel-declaration and surface names into one untagged `Vec<String>`, so a postulate becoming a primitive under the same spelling renders identically across all 107 entries, and the injectivity the fallback depends on is measured rather than enforced"
status: ready
owner: language
size: XS
gate: none
depends_on: []
blocks: []
github: null
origin: "Two residuals parked on LANG-PRELUDE-ELABORATION-DEPTH -- the Architect's env.globals injectivity finding (evt_4bdcm6fer2570) and the Adversary's Finding 3 (evt_31dc9xfdp2ny), both non-blocking. Cut as a node by the Steward 2026-08-14 for the reason given below. Both residuals re-verified against the landed code at main 246019b9 before filing."
---

## Why this is a node now, when both residuals say it should not be

**Both residuals end with the same disposition: *"not filed as a node; it rides
the next Language candidate that enters this file."* That disposition was
correct and its premise has expired.**

The premise was that a node for a one-function edit is overhead **when a
candidate is going to pass through anyway.** No such candidate exists.
`LANG-LOSSLESS-COUNT-ASSERTION-RETIRE` is `XS` and does not touch this file;
Language's only other two nodes — `LANG-DECEQ-CHAR-LAWFUL-INSTANCES` and
`LANG-FOREIGN-NAME-FORMAT-CHARS` — are both `draft` behind **operator**
decisions, not behind framing. So the ride-along would wait on an operator
answer that has nothing to do with it.

**This is `steward.md §4c` applied in the direction that creates work, so it is
stated rather than assumed.** The constraint is grounded: the defect is a
**trusted-base audit surface that cannot see a change in trust class**, which is
`docs/PRINCIPLES.md`'s small-auditable-TCB commitment, not a tidier-graph
preference. What changed is only the scheduling premise, and re-deriving it at
the point of use is exactly what `§4c` asks for.

## The defect

`crates/ken-elaborator/tests/lang_prelude_collections.rs:190-215`, measured at
`main` `246019b9`:

```rust
match decl {
    Decl::Opaque { name, .. } => name.clone(),
    _ => by_global_name
        .get(&id.0)
        .map(|s| s.to_string())
        .unwrap_or_else(|| "<unregistered>".to_string()),
}
```

**Two arms, two different namespaces, one untagged `Vec<String>`.** The first
takes the **kernel declaration** name from the audit surface; the second takes
the **surface** name from `env.globals`. Nothing in the emitted label records
which arm produced it.

⇒ **A trusted-base entry that changes kind while keeping its spelling renders
identically**, and the 107-entry `AC-6` enumeration cannot see it. **An opaque
is an axiom; a primitive is a thing with a reduction.** They are not the same
trust claim, and a postulate quietly becoming a primitive under the same name is
precisely the movement an enumeration of the trusted base exists to catch.

**Second defect, same function.** `trusted_base_labels` depends on
`env.globals` being injective, and the doc comment above it measures that —
*"452 globals, 452 distinct ids, zero aliased (measured at this WP's tip)."*
**Nothing enforces it.** A later prelude addition registering a second name for
an existing id does not fail anything; it makes the label
**order-dependent**, so the test becomes intermittently-changing with a diff
that does not explain itself. A measured fact with a maintained consequence
needs to be a maintained fact.

**They travel together and always did** — both are inside `trusted_base_labels`,
both are one-function edits, and splitting them leaves a second pass over the
same twenty lines.

## Deliverables

**`D1` — tag every label with the decl's kind.** `Opaque(Bytes)`,
`Primitive(add_int)`, and so on, so the namespace each name came from is on the
face of the label. **Take the kind from `ken_kernel::Decl`'s own variants — do
not invent a taxonomy** and do not collapse two variants into one tag because
they look similar from the test.

**`D2` — `<unregistered>` carries its kind too.** This is free and the finding
scoped itself out of it: `decl` is in hand on **both** arms, so the three
deliberately-dropped `conversions.rs` entries can be tagged even when no
surface name resolves. A bare `<unregistered>` should not survive `D1`.

**`D3` — enforce the injectivity the fallback rests on.** The closing line
already exists in the function that depends on it:

```rust
assert_eq!(by_global_name.len(), env.globals.len(),
    "env.globals is not injective; labels would be order-dependent");
```

**`D4` — update the `AC-6` expected list in the same edit.** The 107-entry
enumeration changes shape with `D1`; the residual is explicit that this is *"the
same edit, not a second one."* **The count must stay 107** — `D1` retags
entries, it does not add or drop any. A changed count is a finding, not a
list to bless.

**`D5` — retire both residuals on `LANG-PRELUDE-ELABORATION-DEPTH`**, naming
this node and which deliverable discharged each. Both currently read as open
obligations. Leave the merge record's history intact; only the residuals' live
status changes.

## Acceptance criteria

**`AC-1` — the blindness is demonstrated closed, by mutation, not by
argument.** Force a trusted-base entry down the **other** arm — the shape a
kind change produces — and confirm the `AC-6` enumeration **reds**, naming the
entry. **Report the failing text.** Then restore. Before `D1` this mutation is
invisible, and that is the entire claim of this node; an unmutated green run
does not discharge it.

**`AC-2` — `D3`'s assertion is shown to fire.** Introduce a second name for an
existing `GlobalId`, confirm the injectivity assertion reds with its message,
and restore. A line that has never failed is not known to be a control — this
arc has now found that shape three times.

**`AC-3` — no bare `<unregistered>` survives.** `git grep -n '<unregistered>'
-- crates/` and report the output. Every occurrence must carry a kind.

**`AC-4` — the count is unchanged at 107**, and the diff of the expected list
is a **retagging** only. Report the count and confirm no entry was added or
removed. If the count moved, stop and report it rather than updating the
expected list to match.

**`AC-5` — `-p ken-elaborator` green**, and no file outside
`tests/lang_prelude_collections.rs` and the two residual sections is touched.

**`AC-6` — no-regression, in CI.** `COORDINATION §12` — the venue is CI, not a
local `--workspace` run.

## Contention

**None.** `LANG-LOSSLESS-COUNT-ASSERTION-RETIRE` is Language's only active node
and its scope is `src/lossless.rs` and `tests/kenfmt_b1_lossless.rs`;
intersection with this file is empty. Runtime is on
`RT-DYNAMIC-ARM-SCALAR-MERGE` `c3`, scoped to
`crates/ken-runtime/src/cranelift_backend/lowering/mod.rs` and two
`Cargo.toml`s — also empty.

**If `LANG-LOSSLESS-COUNT-ASSERTION-RETIRE` has not merged when this is picked
up, that is fine** — different files, and no ordering between them.

## Not this node

- **No change to `trusted_base()` itself, to the prelude, or to any kernel
  declaration.** This node changes how the test *labels* the trusted base, not
  what is in it. A change to the membership is a different node and a much
  larger claim.
- **No new assertion about what the trusted base *should* contain.** `AC-6` is
  an enumeration deliberately, and `D5b` already ruled that its size is a
  finding to report rather than a reason to weaken it to a per-name check.
- **No repair of the `<unregistered>` entries' registration.** Tagging them is
  `D2`; making them resolve is a different question and is not asked here.
