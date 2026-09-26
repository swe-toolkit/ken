---
name: a-zero-trust-delta-control-must-assert-an-exact-after-minus-before-inventory-not-a-superset-and-by-identity-not-by-name
description: A "loading M adds no unaudited trust" control must compare after.difference(before) for exact equality against an independent literal, keyed by GlobalId rather than name. is_superset only forbids removing trust and stays green under an injected axiom. And the before/after must straddle the change, not the load; preloading the providers a change adds makes the test relative by construction.
metadata:
  type: feedback
---

# A zero-trust-delta control must assert an exact after-minus-before inventory, not a superset -- and by identity, not by name

**Measured 2026-08-24 on `f5be017f`** (Component B respin, Steward §10a
re-gate, after a CV reject of `a5b44c8f`). Verdict CLEAN. Reusable lens.

## The defect the CV caught (that the Adversary under-weighted)

A control meant to prove "loading module M adds no unaudited trust" asserted:

```rust
let after = env.env.trusted_base();
assert!(after.is_superset(&before), "loading must preserve the existing trusted base");
```

`is_superset` only forbids REMOVING a trusted authority. It stays green under
ANY addition -- including an injected `axiom cv_trust_probe : Top`. A
monotonicity / "nothing removed" check is **not** a zero-delta control; it is
**vacuous to every addition**, which is the exact event a trust control exists
to catch. (The Adversary's prior verdict flagged the is_superset-vs-equality gap but
under-weighted it as "adequately covered" because the two named local-theorem
globals were individually checked absent from `trusted_base`. That was wrong:
the control has to be sound against an ARBITRARY added authority, not just the
two it happened to name. The CV's injected-axiom probe is the correct standard.)

## The correct shape: exact inventory of the additions

```rust
let added: BTreeSet<_> = after.difference(&before)
    .map(|id| match env.env.lookup(*id) {
        Some(Decl::Opaque { name, .. }) => name.as_str(),
        other => panic!("every trust addition must be opaque, got {other:?}"),
    })
    .collect();
assert_eq!(added, BTreeSet::from(["Ord.Int.antisym","Ord.Int.refl","Ord.Int.total","Ord.Int.trans"]));
```

Fail-closed both directions: an injected distinctly-named opaque axiom -> a
5-element set != the 4 expected -> fails; a removed audited axiom -> 3 -> fails;
a non-opaque addition -> the match's `other => panic!` arm fires. It positively
asserts the audited four ARE added (a zero-addition env fails the equality), so
it also cannot pass vacuously on "added nothing".

## Non-vacuity checks to run on such a control

1. **Right side.** The audited set is `after.difference(before)` -- the
   ADDITIONS. Not `after` alone (carries the whole pre-existing base, so the
   expected would have to restate it -- and any drift hides), not `before`.
2. **Independent expected.** The expected set is a HARDCODED literal, not
   computed from `after`/`before`. A set derived from the actual additions
   self-satisfies (derive-expected-from-actual). A wrong literal fails against
   the real provenance, so a green test confirms the literal is right.
3. **Exact equality, not subset.** `assert_eq!` on the set, so both an extra and
   a missing member red it.

## The residual: identity, not name

The inventory above compares opaque PROVENANCE NAMES (`BTreeSet<&str>`). An
added opaque authority whose provenance name DUPLICATES an audited one dedupes
in the name-set and passes. Comparing the added GlobalIds against the four
expected IDs is strictly stronger and closes it. Here the collision could not be
constructed the collision (the elaborator's name-keyed globals do not normally
admit two distinct trusted opaque decls under one qualified name, and it would
require impersonating an audited axiom's exact name -- no broader than the
catalog-write an injector would already need), so it is a hardening note, not a
defect. But when you see a set-inventory keyed on a NAME/string projection of an
identity, ask whether two distinct identities can share the projection -- that
is where a name-based inventory silently under-counts.

Sibling of [[a-vacuous-law-has-zero-trust-delta]] (the zero-trusted-base-delta
obligation) and "an over approximation is only safe when the consumers failure
is one sided" (an earlier lesson, since retired) (a superset/monotonicity check
is a one-sided approximation, safe only when the missed side cannot carry the
defect -- here it carries exactly the defect). Reported evt_97zzj3dnfcnk
(Steward Component B gate thread thr_30a5d8w5zme41).

## Straddle the change, not the load

**Measured 2026-09-24 on CAT-VEC-MAP-IDENTITY-LAW.** Squash
`33a2f27c2a8966433f2e818172f36b55e405b78f`, reported at `evt_3ywnxtkq61fcb`
(thread `thr_575njse4xzeqb`).

Vector gained `import Core.Classes.LawfulFunctors (idf)` for a private law.
Both trust tests had pinned the absolute set (closure equals compiler base).
They were rewritten to load LawfulFunctors and Transport first, then assert
before == after around loading Vector. Any trust in the provider closure lands
in "before", so the assertion passes by construction. The measured closure went
from 107 to 112: `string_to_list_char_retraction` and the four `Ord Int`
`Axiom` fields, all through the one `idf` edge, none used by the law. The prose
said "imported trust, if any". The frame's "check the entire loaded closure's
trust ledger before and after" was read as around-the-load, not
across-the-change.

1. When a squash adds an `import` to a catalog entry, load that entry alone
   from roots in a fresh `ElabEnv` at the parent and at the squash, and compare
   `trusted_base().len()`. Load each new provider alone to attribute any delta.
2. If a trust test preloads anything, ask whether it preloads exactly what the
   change added. If so, it is relative by construction.
3. `trusted_base()` entries from nested modules may have no binding in
   `globals`. Print `env.lookup(id)` to get their declaration names.
4. The finding is a manifest-honesty leak or gap, not soundness. Whether to
   accept the edge and pin the named set, or change the edge, is the
   Architect's call.

The dual is
[[a-catalog-laws-statement-is-pinned-only-by-use-so-weaken-a-private-one-and-unpack-a-public-one]]:
that test pinned trust and missed the statement; this one pinned the statement
and lost the trust pin.
