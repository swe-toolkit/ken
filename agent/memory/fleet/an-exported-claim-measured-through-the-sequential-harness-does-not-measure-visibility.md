---
name: an-exported-claim-measured-through-the-sequential-harness-does-not-measure-visibility
description: The sequential package harness binds private names too, so a test that asserts "exported" through it passes for private declarations. Ground a visibility claim with a no-build `ken check` probe placed under a git-archived catalog root.
metadata:
  type: feedback
---

# An "exported" claim needs the real loader's export table

**Measured 2026-09-25 on CAT-DERIVED-SORT-LAWS.** Squash
`c1c93f293b1fe502f4cc67980be1b8a297175649`, reported at `evt_xqj8sfpk31hq`.

## The shape

Several sources treat Derived's sort surface as public:

- Its Public API list names `Perm`, `insert`, `sort` and `sort_bool*`.
- `cat3_collections_package.rs` asserts that each "should be exported by
  Derived.ken".

None of these names is declared `pub`. The cat3 test resolves names through
`load_derived_fixture`, the sequential harness, which binds private names
too. So the test is green for a property the real loader refutes. The new
laws attached to `sort`/`insert` therefore have no consumer outside Derived.

## How to apply

1. When a package's prose or a test says "public" or "exported", check the
   declaration for `pub`. Then check which loader the test used: an
   `env.globals` lookup after a sequential load does not measure visibility.
   The real measure is `env.module_state.exports[module]`; see the
   PriorityQueue export-table test in `modules.rs`.
2. **A no-build probe works when the build lock is contended.**
   - Take a copy of the landed catalog: `git archive <sha> catalog | tar -x
     -C <scratch>`.
   - Put the probe modules under `<scratch>/catalog/packages/Probe/X.ken`.
     The CLI uses the roots loader only when the path contains a derivable
     `catalog/packages` module address. A file at the scratch root fails
     with `UnboundName` on the module itself.
   - Run any seat's prebuilt `target/debug/ken check` on the probe files.
   - Always pair the probe with a control import of a known-`pub` name.
3. State the carry: say whose binary you used and when it was built. Also
   say that the squash touched no `crates/*/src`, if that is true.

The same error once served as evidence in a pub-flip clearance: a qualified
`env.globals` lookup in `cat3_collections_package.rs` was read as proof that a
private name resolved cross-package. See
[[a-catalog-pub-flip-is-inert-to-consumers-so-hunt-its-signature-closure-trust-and-prose]].

Related: CHECKS.md check 4,
[[my-own-tracker-capability-landed-line-can-be-stale]].
