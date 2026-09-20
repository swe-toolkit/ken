---
id: LANG-IMPORT-IDENTITY-ESCAPE
title: "give bind_import's already-a-local arm the same-identity escape its already-imported arm already has, keyed on resolved GlobalId rather than on the surface spelling -- a module may not currently import a name it also reaches ambiently, even when both routes resolve to one declaration"
status: merged
owner: language
size: S
gate: none
tier: T1
depends_on: []
blocks: [CAT-PARSING-CURSOR-LAWS]
github: null
origin: "Architect mechanism ruling evt_56ngqsbcmtavc, on a hard stop reached by CAT-PARSING-CURSOR-LAWS (foundation-leader evt_5aaega9rz7zpa). The Architect rejected the source-side workaround (a local alias plus thirteen proof-reference rewrites) as the wrong layer and named the resolver defect exactly. Steward-framed as a precursor rather than a fold, per that ruling: the fix is in crates/ken-elaborator/src/modules.rs, which is not a catalog proof WP's lane."
---

# Import of an ambiently-reachable name is refused, even at one identity

**`bind_import` treats "already imported the identical thing" as fine and
"already present ambiently as the identical thing" as an ambiguity.** That
asymmetry is the whole defect.

## Settled inputs — measured. Do not re-derive.

**1. The two "sources" are one declaration reached two ways.** Architect,
`evt_56ngqsbcmtavc`: `IsTrue` is declared **exactly once in the corpus** —
`catalog/packages/Core/Classes/LawfulClasses.ken.md:54`,
`pub fn IsTrue (b : Bool) : Prop = Equal Bool b True`. There is **no base or
kernel `IsTrue`**; grepping `crates/ken-elaborator/src` and
`crates/ken-kernel/src` returns nothing. The ambient binding arrives through
`Data.Numeric.Nat.Order`'s own
`export Core.Classes.LawfulClasses (Ord, IsTrue, bool_or, leq_nat)`.

**2. The defect is one clause,** `crates/ken-elaborator/src/modules.rs:256`.
Arm 1 fires on **mere membership in `locals`** and compares nothing; arm 2
already carries the idempotence escape:

    if self.locals.contains(bare) { ... return Err(AmbiguousReference) }   ARM 1
    Some(existing) if existing == qualified => {}                          ARM 2

The reported `local` is the bare string `"IsTrue"` precisely because
`bindings.get("IsTrue")` was `None` and it fell back to `bare.to_string()`.

**3. A string comparison cannot be the fix.** The two spellings differ by
construction — `"IsTrue"` against `"Core.Classes.LawfulClasses.IsTrue"` — so a
string-keyed escape could never fire. **Key on the resolved `GlobalId`.**

## Deliverable

**Arm 1 gains the escape arm 2 has, keyed on resolved identity:** if `bare` is
already a local **and** the local's resolved id equals the id `qualified`
resolves to, the import is a **no-op** — bind nothing, raise nothing.
Otherwise the ambiguity stands and is reported as today.

**No new trust, no representation change, no diagnostic-text rewrite beyond what
the clause requires.**

## Acceptance criteria

**`AC-1` — the same-identity import is accepted and binds nothing.** *Control:*
a module that reaches a name ambiently **and** imports it explicitly elaborates,
and the binding table afterwards is what it would have been without the import.
"It compiles" is not the control; assert on the resolved binding.

**`AC-2` — genuinely distinct spellings STILL REFUSE, and this control must go
RED if the escape is written too wide.** *Control:* two declarations with
different `GlobalId`s reachable under one bare name still raise
`AmbiguousReference`, naming both sources. **This is the falsifier for the whole
node.** An escape keyed on the string, or on presence rather than identity,
passes `AC-1` and fails this — which is exactly the failure mode to catch.

**`AC-3` — the reported `sources` stop lying when the escape does not fire.**
*Control:* in the surviving ambiguity, the first entry is the local's resolved
identity rather than the `unwrap_or_else(|| bare.to_string())` fallback, **or**
state in the handback that the fallback is still reachable and why. Do not
silently keep a diagnostic that prints a bare name as if it were a source.

**`AC-4` — the motivating case actually clears.** *Control:* with this landed,
`catalog/packages/Capability/Parsing/Cursor.ken.md` can import `IsTrue`
explicitly from `Core.Classes.LawfulClasses`, and
`catalog_ambient_passthrough_migration_census` shows `Capability.Parsing.Cursor`
**without** `IsTrue`. **Report the census delta; do not edit the sentinel.**

**`AC-5` — no-regression, in CI** (`COORDINATION §12`). Targeted locally:
`-p ken-elaborator`.

## Stop condition

**If the escape cannot be keyed on resolved identity at that point** — because
the id is not available where `bind_import` runs — **stop and report the
shape**, do not fall back to a string or path comparison. A spelling-keyed
escape is the defect this node exists to remove, one layer along.

## Not this node

- **Re-baselining the ambient census.** It is measuring a true thing. The debt
  stays outstanding until this node discharges it.
- **The alias workaround** — rejected at `evt_56ngqsbcmtavc`, and the thirteen
  `Cursor.ken.md` proof references are not to be rewritten.
- **A general audit of name-keyed structures.** The Architect notes this is the
  third instance of one predicate tonight, with `InstanceHeadSpellingsShareAn`
  `Identity` and `match_instance_head_core` as the other two. **That is a
  finding, not a scope** — if you see a fourth, report it and stop.
