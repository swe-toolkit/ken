---
id: SPEC-DIAGNOSTIC-CODE-CONTRACT
title: "The Architect's DIAG-STABLE-ERROR-CODES D0 rules that a CLI failure code is a pinned identifier, a conformance surface like the verification protocol's, but no spec section states it. Author the normative section: presence, code grammar and domains, stability, the human line, the native cause families, and the machine format"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Architect D0 evt_1yhr30fekfyyv, item 5 ('Normative: yes, and it needs a /spec section'), grounded in spec/20-verification/25-protocol.md sections 1 and 6. Serves DIAG-STABLE-ERROR-CODES (operator 2026-10-12), whose rendering increment waits on it. Steward-filed per COORDINATION section 2."
---

# A normative contract for CLI failure codes

## Objective

The spec states what every CLI-returned failure carries, so that a code
can be relied on by tools and conformance tests rather than only by the
implementation's own ledger.

## Settled inputs (D0 `evt_1yhr30fekfyyv`, on `473ccf9b4`)

- **The ground.** spec/20-verification/25-protocol.md section 1 says
  "Human-readable error text is not enough ... the machine view is
  primary", and section 6 makes the protocol's stable identifiers a
  conformance surface.
- **The D0's provisional design:**
  - codes are one domain letter and four digits, dense per domain;
  - the domains are C (CLI), E (elaboration), K (kernel, projected),
    X (execution), N (native) and H (host, projected);
  - an append-only ledger in which a code is never reused or re-meant,
    and a retired code stays reserved;
  - a human line `error[<code>]: <message>`;
  - seven native cause families N0001-N0007, defined by meaning.

  The spec may respell the codes once, before rendering lands.
- **Unsettled, and this section's to decide or defer:** a JSON form
  (for example mirroring `ken.verify/v1`), more than one diagnostic per
  run, warnings, and any mapping from code to exit status.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. **D0 (Architect, short).** Confirm or adjust the grammar and domains,
   and rule which of the unsettled items the section fixes now and which
   it defers.
2. **The section**, stating:
   - presence: every CLI-returned failure carries exactly one code;
   - the grammar and domains;
   - the stability rule;
   - the human line shape;
   - the native families by meaning;
   - whether the ledger is incorporated by reference;
   - the machine format as ruled.

   Add a conformance seed of closed cases.

## Acceptance

- **AC-1.** The section states exactly the D0's rulings. Each deferred
  item is named as deferred.
- **AC-2.** The seed has a case per stated rule, including the
  `error[<code>]:` line shape and a refused malformed code.
- **AC-3.** `SPEC-PROGRESS.md` records the section.

## Stop conditions

- A rule would need a kernel or check-rule change.
- The section would fix a CLI flag or output format the D0 did not rule.
