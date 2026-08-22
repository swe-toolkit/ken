---
id: V3-FO-EMBEDDING-ADEQUACY
title: "Author the embedding and prove embedding_adequacy (classically_valid of the translated form implies the source form) — the SECOND of the two theorems 23 section 4.4 requires before route FO may return proved"
status: draft
owner: language
size: L
gate: none
depends_on: [V3-FO-KEN-LEVEL-CHECKER-AUTHORING]
blocks: []
github: null
origin: "Steward, 2026-08-22, discharging the framing debt surfaced when the FO D0 fork was routed to the spec enclave. V3-FO-CHECKER-SOUNDNESS is the FIRST of the two 23 section 4.4 theorems; this node is the SECOND, previously unfiled. The enclave D0 ruling (spec-leader evt_2enqgkgqwd2g5, from spec-author evt_3kefqcayzajq9) directed that this node be cut AFTER D0 landed, on the structural assumption, so it does not race ahead and silently assume a (b)/(c) kernel premise. Steward-filed per COORDINATION section 2."
---

> # FILED 2026-08-22 — the second route-FO `proved` theorem, on the structural arm

Route FO may return `proved` only once BOTH `23 section 4.4` theorems hold. The
first is `checker_soundness` ([[V3-FO-CHECKER-SOUNDNESS]]); this node is the
second, `embedding_adequacy`.

## Objective

Author the embedding (the translation of a source formula into its object form)
and prove `embedding_adequacy`: that classical validity of the translated form
implies the source form.

## Why it is the cleaner of the two, and forces no kernel arm

The spec enclave's D0 analysis (spec-author, evt_3kefqcayzajq9) established that
adequacy is structural induction on the source formula over the translation
clauses — no certificate, and no rotation-prone second argument like the one that
made `checker_soundness` the SCT-expressibility crux. It needs no SCT and forces
no kernel `size_rel` change. Under the arm-(a) ruling
([[V3-FO-SOUNDNESS-SCT-EXPRESSIBILITY]]) each of the two theorems resolves the
same way — structural elaboration — so this node does NOT depend on the SCT
rotation fix and does not gate on a kernel arm.

## Sequencing

Filed at `draft`; not on the current lane-2 frontier. It sits behind the FO
re-elaboration ([[V3-FO-SOUNDNESS-SCT-EXPRESSIBILITY]]) and `checker_soundness`
([[V3-FO-CHECKER-SOUNDNESS]]) in the language ring's queue, and is framed when the
ring reaches it. Its dependency `V3-FO-KEN-LEVEL-CHECKER-AUTHORING` (PR #2421) is
merged, so the premise is real; it is `draft` because it is unframed, not because
a dependency is unmet.

## Return condition (binding, from the enclave)

If authoring the embedding reaches a named sub-structure whose termination is
genuinely non-structural — not a subterm descent of any derived inductive, which
would contradict `23 section 4` fixing the reflective checker as structural — that
returns to the spec enclave + Architect as a new fork, never a silent fall into a
kernel measure change.
