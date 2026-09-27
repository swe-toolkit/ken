# teams/foundation — Foundation-team lessons

Loaded by the Foundation ring — `foundation-leader`, `foundation-implementer`,
`foundation-qa` — in addition to `fleet`, `build/`, and the function scope
(`build/leaders` · `build/implementers` · `build/qa`).

For lessons specific to the foundation layer: the standard-library catalog
packages, lawful classes, and the shipped `.ken` corpus that downstream proofs
ride on.

**The recurring hazard on this team is corpus-wide reach.** A change to a
catalog package is validated by oracles living in crates the WP never touches, so
a targeted per-crate run cannot see them and they surface as red CI at publish —
after review, after the merge Decision, at the most expensive moment. Enumerate
every test that globs the directory you are adding to.

| Lesson | One-line |
|---|---|
| _(none recorded yet)_ | |

**An empty scope is a normal state, not a defect.** Record a lesson at the
broadest scope where every reader must apply it; a genuinely cross-cutting one
gets a `scope:` frontmatter tag rather than a copy.

| [a-catalog-laws-statement-is-pinned-only-by-use-so-weaken-a-private-one-and-unpack-a-public-one](a-catalog-laws-statement-is-pinned-only-by-use-so-weaken-a-private-one-and-unpack-a-public-one.md) | Nothing checks a law's stated type but use: weaken a private law's statement to test its pin; a public law needs a client that unpacks it; settle falsifiers on the public statement. |
| [a-refl-proof-over-a-provider-definition-can-embed-the-providers-private-body-and-a-widened-closure-pin-is-where-it-shows](a-refl-proof-over-a-provider-definition-can-embed-the-providers-private-body-and-a-widened-closure-pin-is-where-it-shows.md) | A Refl witness can embed a provider's unfolded private body; a widened closure pin in the same squash is the tell; measure a cong variant. |
| [a-shared-fixture-facade-preload-voids-every-consumers-facade-avoidance-pin-and-a-fresh-env-sibling-is-the-tell](a-shared-fixture-facade-preload-voids-every-consumers-facade-avoidance-pin-and-a-fresh-env-sibling-is-the-tell.md) | A facade preload added to a shared test env makes consumers' negative facade-avoidance pins false, so they get deleted; a sibling pinning it in a fresh env names the drop. |
| [an-alias-between-two-identities-with-the-same-body-is-invisible-to-the-type-checker-so-swap-the-binding-to-test-the-identity-pin](an-alias-between-two-identities-with-the-same-body-is-invisible-to-the-type-checker-so-swap-the-binding-to-test-the-identity-pin.md) | When two GlobalIds share a body, elaboration cannot tell which an alias bound; test the identity pin by rebinding in a scratch catalog and counting mentions. |
| [an-exported-claim-measured-through-the-sequential-harness-does-not-measure-visibility](an-exported-claim-measured-through-the-sequential-harness-does-not-measure-visibility.md) | The sequential harness binds private names, so "exported" asserted through it measures nothing; use the loader's export table or a no-build ken check probe. |
