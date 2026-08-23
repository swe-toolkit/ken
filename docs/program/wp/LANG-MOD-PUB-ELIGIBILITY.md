# WP — LANG-MOD-PUB-ELIGIBILITY (module/import WP-3): reject `pub` on ineligible placements with a surface diagnostic

Language lane (module/import campaign [[LANG-MODULE-IMPORT-SYSTEM]]). One WP, one
branch `wp/LANG-MOD-PUB-ELIGIBILITY`, one PR. Owner: language. Size: M. Gate:
none. Depends on: none (independent; the language ring takes it after WP-1 per
the single-threaded ring order WP-1 -> WP-3). Blocks:
LANG-MOD-CATALOG-REALIZATION (WP-4).

Source: Architect component framing `evt_hpnhqy1ex286` (WP-3). The eligible/
ineligible placement list is normative in the merged spec surface (`def16ecf4`);
this WP implements the behavior it describes.

Fixed inputs measured at `origin/main be321d40b` (WP-1 fully closed; code sites
unchanged since the doc-only M9 close).

## Objective

Make `pub` on an unsupported placement a surface error, as the merged grammar
normatively requires. Today the parser wraps any declaration in `Decl::Pub`
with no eligibility check, so `pub import` / `pub export` / `pub` on a module
header / instance / fixity / program-or-package form is silently accepted and
ignored. The spec says such a prefix "is never accepted and ignored" — it must
reject with a surface diagnostic. This WP adds exactly that gate; it changes no
resolution or visibility semantics beyond turning silent acceptance into a
surface rejection.

## Fixed inputs (SETTLED, at `origin/main 503817dcc`)

1. The normative rule: `spec/30-surface/32-grammar.md:92-99` — "The optional
   `visibility` prefix is grammar factoring, not blanket `pub` eligibility.
   `pub` is well formed only where `33 §4` and the declaration's own rule allow
   it; ... the prefix is never accepted and ignored. In particular, `pub` is not
   an alternate spelling for an `import` [/export/module] declaration. An
   attached `pub proof` also retains `33 §8.2`'s requirement that its subject be
   public." Grammar production: `visibility ::= "pub"` (`:28`); `pub import` is
   not a valid declaration (`:89`).
2. The eligibility basis: `spec/30-surface/33-declarations.md` §4.1 "Visibility —
   private by default, `pub` to export" (`:274-286`) — `pub` exports a
   top-level name-introducing declaration into the module's public interface;
   the proof rule §8.2 (subject must be public).
3. The bypassing seam: `crates/ken-elaborator/src/parser.rs` — `parse_pub_decl`
   (`:1286-1289`) parses the inner declaration and returns
   `Decl::Pub(Box::new(inner))` with NO eligibility check; dispatched from
   `Token::KwPub => self.parse_pub_decl(start)` (`:214`). It already holds the
   fully-parsed inner `Decl`, so the inner kind is available at this seam.
4. The scope-time handling that currently silently inerts (NOT a rejection):
   `crates/ken-elaborator/src/modules.rs` — `expand_scope` (`:1458`), the
   `decl.is_pub()` placement branch (`:1473`), and the admission-run pub gating
   (`:1764-1768`). These publish/qualify/inert by kind and nesting; none rejects
   an ineligible `pub`.
5. The cross-cutting invariant + pin: flat-Σ / zero `trusted_base()` delta;
   `crates/ken-elaborator/tests/es3_modules_acceptance.rs`
   `module_elaborates_to_identical_flat_sigma` (`:28`). This gate is a surface
   rejection only — it allocates no kernel decl and mints no `GlobalId`.

## Design judgment (front-loaded)

- **Reject, do not silently inert.** The spec is explicit that an ineligible
  `pub` is a surface error, not a prefix that is accepted and dropped. The gate
  must produce a diagnostic, not fall through to `expand_scope`'s inert path.
- **Freeze the CATEGORY, not a roster.** The eligible set is the top-level,
  name-introducing definitional declarations per 33 §4 (plus `pub proof` under
  §8.2). Encode eligibility as a predicate over the `Decl` kind — a match whose
  ineligible arms are the structural/non-name-introducing forms
  (import, export, module/header, instance, fixity, program/package) — so a
  newly-added decl kind must be classified explicitly rather than defaulting to
  "eligible." A hand-listed roster of ineligible names is the anti-goal (it goes
  stale as decl kinds are added); the discriminating AC pins the category.
- **Earliest seam that has the inner kind.** `parse_pub_decl` already parses the
  inner `Decl`, so it is the natural surface seam; the Architect may instead
  site the predicate at the earliest elaboration point that both has the inner
  kind and can emit a surface diagnostic. Either is acceptable provided it
  rejects (not inerts) and the diagnostic is a surface error. Do not scatter the
  predicate across both the parser and `expand_scope`.
- **`pub proof` keeps §8.2.** A `pub proof` is eligible, but the existing rule
  that its subject must be public is retained unchanged — this WP neither
  removes nor weakens it.

## Deliverables

1. A `pub`-eligibility predicate over the `Decl` kind (eligible = the top-level
   name-introducing definitional decls per 33 §4, including `proof`;
   ineligible = the enumerated structural forms), encoded so an unclassified new
   kind is a compile-time obligation, not a silent "eligible."
2. The `pub` seam rejects an ineligible placement with a surface diagnostic;
   eligible placements are accepted unchanged.
3. The `pub proof` §8.2 subject-must-be-public rule retained.
4. Tests: one eligible-`pub` positive; each enumerated ineligible placement
   rejected with a surface diagnostic; the §8.2 rule still enforced.

## Acceptance criteria (property + closure axis + loud failure)

- **AC-1 (eligible accepted).** `pub` on an eligible interface declaration
  elaborates as it does today (accepted, exported).
- **AC-2 (ineligible rejected — the discriminator pair).** `pub` on each
  enumerated unsupported placement (import, export, module/header, instance,
  fixity, program/package) rejects with a surface diagnostic. Freeze the
  CATEGORY of eligible placement (a predicate over decl kind), not a hand-listed
  roster — CV group-1 discriminator (`evt_2wejn8hekr4qw`). CONTROL: a positive
  eligible-`pub` case that stays accepted, paired with the ineligible cases, so
  the gate is shown to discriminate rather than reject-all.
- **AC-3 (§8.2 retained).** `pub proof` remains subject to the
  subject-must-be-public rule — a `pub proof` whose subject is private still
  rejects, exactly as before this WP.
- **AC-4 (cross-cutting invariant).** Zero `trusted_base()` delta; the flat-Σ
  pin `module_elaborates_to_identical_flat_sigma`
  (`es3_modules_acceptance.rs:28`) stays green.
- **AC-NO-REGRESSION.** Whole-suite green in CI (COORDINATION §12). Local builds
  targeted `-p ken-elaborator` (and `ken-cli` if the diagnostic surfaces
  through it), never `--workspace`.

This WP does NOT change resolution/strictness (WP-2), does NOT change the module
loader entry (WP-1, landed), and does NOT rewrite any catalog file (WP-4).

## Contention

Clear. Writes land in `crates/ken-elaborator/src/parser.rs` (and possibly
`modules.rs`) plus tests. WP-1 (which also touched `ken-elaborator`) is LANDED
(`503817dcc`), so there is no live language contention — WP-3 builds on the
landed tree. The runtime priority-1 lane's in-flight slice
(RT-CHECKED-IH-CAPTURED-ENV-SCHEMA tier 2) is `crates/ken-runtime` only — no
overlap. WP-2/WP-4 are the same single-threaded ring, sequenced after. The
Steward routes both language and runtime merges to sequence any window.

## Capability tier (§4h)

T2 (cheap coder). A bounded semantic gate at a known parser/scope seam;
the review is a discriminator pair (eligible accepted vs each ineligible
rejected), not an argument. No novel design, no soundness-bearing invention.
Size M.

## Reviewers

- Architect — component fit (the gate rejects at the right surface seam; the
  eligibility predicate is category-frozen; §8.2 retained; no semantics drift).
- conformance-validator — the `pub`-eligibility rejections are CV's
  discriminator-pair territory (group 1, `evt_2wejn8hekr4qw`): an eligible-`pub`
  positive against the enumerated ineligible placements.

## Do-not guards

- Do NOT silently accept-and-ignore an ineligible `pub` (the current behavior) —
  reject with a surface diagnostic.
- Do NOT freeze a hand-listed roster of ineligible names; freeze the eligible
  category as a predicate so a new decl kind must be classified.
- Do NOT remove or weaken the `pub proof` §8.2 subject-must-be-public rule.
- Do NOT touch resolution/strictness (WP-2) or `resolve_ref`; do NOT weaken
  `module_elaborates_to_identical_flat_sigma`.

## Sequencing (Steward-owned)

Released after WP-1's merge (landed `503817dcc`); the single-threaded language
ring takes WP-3 next. On WP-3's merge, WP-2 (LANG-MOD-STRICT-RESOLUTION, the
soundness core) is next, then WP-4 (gated additionally on the Or/Inl/Inr fork).
