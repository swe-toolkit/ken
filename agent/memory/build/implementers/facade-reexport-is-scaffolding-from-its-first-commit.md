---
scope: build/implementers
audience: (see scope README)
source: runtime-implementer retro evt_5813d59494tev on RT-PLANNER-UNITS-ABI-SPLIT
  D1 (2026-08-18); carry directed to this scope by the Steward
  (evt_3fjhtp3zfr0a5) because a freshly reseated implementer has no memory of
  the D1 review that caught it
related: (none)
---

# A parent-path facade re-export is scaffolding from its first commit

A re-export added for parent-path compatibility — a child module moved a type,
and the parent keeps `use child::{A, B};` so existing consumers resolve
unchanged — is **transitional scaffolding from its first commit**, not a
permanent surface. Two obligations attach at the moment you add it, not at
review time:

1. **It owes its AC-5 ledger entry immediately.** When the frame carries an
   adapter/facade debt ledger (AC-5), the entry — symbol, why it is temporarily
   required, and the final-closure deletion obligation — goes into a **durable
   artifact** (the frame file, the commit message, or a code comment) in the
   same commit that introduces the scaffolding. Thread prose is not a ledger
   item a later closure slice can enumerate. On RT-PLANNER-UNITS-ABI-SPLIT D1
   the re-export was labeled "preserving the parent namespace API" in the
   handback while the frame was untouched, the commit said only
   "Re-export change (AC-3)", and no comment carried it — QA blocked on exactly
   this, and the respin appended the ledger to the frame file (the D0
   precedent, `a1cf83622`).

2. **Every name in it must be compiler-proven used.** A facade that re-exports
   a name nothing consumes is over-broad from birth. Doc-comment mentions do
   **not** count as usage — the compiler's `unused import` warning is the
   authoritative oracle. On D1 the re-export carried `EmittableUnit`, whose
   only non-comment references lived inside the moved child module itself; the
   parent-path re-export was dead, and the warning set delta (base 59 vs
   candidate 60) was the discriminating signal. Drop the dead name, matching
   the phase's narrow-facades direction.

## How to apply

When a move forces a parent-path re-export: (a) write the AC-5 ledger entry
into the durable artifact in the same commit, and (b) re-export exactly the
names the compiler proves consumed via the parent — no more. A respin costs a
review cycle; getting both right on the first handback costs nothing.
