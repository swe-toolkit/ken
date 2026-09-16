# Membership binding conformance seed

Format: `../../README.md`. These cases pin `∈`'s standard meaning, its
carrier-first completion, and the provider discipline of
`spec/50-stdlib/58b-membership.md`, against `33 §6.3` and `39 §6.10`.

**Status and reachability.** Cases that observe a **completed** `q ∈ c`
occurrence are **RED-UNTIL-LANG-MEMBERSHIP-OPERATOR-SURFACE**: completion and
the binding are that node's to build, and the binding's surface form
additionally waits on a projection form in the type grammar, which that node
carries as a prerequisite. Cases that observe today's artifacts — the class
shape, the absence of a `Type`-universe field elsewhere in the catalog, and the
fixity band — are live and must stay green.

**Promise class.** Carrier-first inference, the witness-bound provider
discipline, and the `GlobalId`-keyed completion are durable invariants. The
provider *set* is a recorded census, not a promise it never grows.

### surface/operators/membership-resolves-over-each-standard-view

- spec: `58b §3`, `39 §6.10`
- given: `q ∈ c` at each standard provider — a list view, an ordered-key view,
  the same ordered-key view at `v = Unit` serving set membership, and a
  relation-edge view whose query is a pair.
- expect: **RED-UNTIL-LANG-MEMBERSHIP-OPERATOR-SURFACE** — each resolves to the
  provider for its container's head and yields the `Bool` that container's
  existing explicit worker yields on the same inputs.
- why: four providers over three distinct heads, with the set arm sharing the
  ordered-key head, so an implementation that keys on anything coarser than the
  provider collapses two of them. **MEASURED:** not yet. **CLAIMED:** resolution
  is per-provider. **THE GAP:** agreement with the worker is what makes this
  non-vacuous — a case asserting only "returns a `Bool`" passes on any
  implementation that returns one.

### surface/operators/membership-infers-the-carrier-first

- spec: `39 §6.10`
- given: a query type accepted by **two** providers over different containers,
  used on the left of `∈` with each container on the right.
- expect: **RED-UNTIL-LANG-MEMBERSHIP-OPERATOR-SURFACE** — each occurrence
  resolves by its **right-hand** container, and the two occurrences select
  *different* providers despite identical left-hand types.
- why: this is the discriminating pair for inference order, and it is the one
  arrangement where the orders disagree. Under LHS-first inference the two
  occurrences are indistinguishable at the point of choice, so an implementation
  must either guess or fail; under carrier-first they are decided before the LHS
  is looked at. **THE GAP:** every occurrence with an unambiguous query type
  passes under **both** orders, which is why a single-provider case would prove
  nothing here.

### surface/operators/membership-keeps-the-validity-witness-bound

- spec: `58b §3`
- given: an ordered-key view value validated under one comparator, and a second
  comparator over the same key type that orders differently, both in scope.
- expect: **RED-UNTIL-LANG-MEMBERSHIP-OPERATOR-SURFACE** — `member` uses the
  comparator the view value carries. An implementation that resolves a
  canonical `Ord` at the call site is non-conforming, and on a tree ordered
  under the other comparator it answers wrongly on a well-typed input.
- why: the failure this pins is silent. A raw container binds no comparator in
  its type, so a fresh-`Ord` implementation type-checks everywhere and diverges
  only where the two orders disagree. **THE GAP:** a single-comparator fixture
  is green under both implementations; the second, differently-ordering
  comparator is what makes the case discriminate.

### surface/operators/membership-tree-instance-is-refused

- spec: `58b §3`
- given: a program installing `Membership Tree` directly on the raw container
  head.
- expect: **RED-UNTIL-LANG-MEMBERSHIP-OPERATOR-SURFACE** — refused. Key, set
  and relation-edge membership are three meanings over one head and at most one
  could be canonical.
- why: the prohibition is what forces the nominal views, so a corpus without
  this case would leave the view design looking like a stylistic choice.
  **THE GAP:** refusal must be attributed — a rejection for an unrelated reason
  (a missing field, a malformed instance) passes a bare `expect: reject` and
  pins nothing, so the case names the canonical-one-per-head conflict as the
  reason.

### surface/operators/membership-missing-or-ambiguous-provider-is-an-instance-error

- spec: `39 §6.10`
- given: `q ∈ c` where `c`'s head has no admitted provider, and separately where
  resolution is ambiguous.
- expect: **RED-UNTIL-LANG-MEMBERSHIP-OPERATOR-SURFACE** — both are ordinary
  instance-resolution errors at the occurrence, distinguishable from each other,
  and neither is a fallback to a different meaning nor a silent acceptance.
- why: paired with the refusal above so that "rejects" is not the whole
  observation. **THE GAP:** an implementation that silently declines to complete
  and leaves `∈` as an unresolved name would satisfy a rejection-only assertion.

### surface/operators/membership-is-not-a-prop-elimination

- spec: `58b §2`
- given: the proposition view `member_holds` and the `Bool` result it is defined
  over.
- expect: `member_holds` is a definition over the `Bool`; there is no operation
  recovering a `Bool` from a proof of it. Live on this base as a structural
  observation of the contract.
- why: `Bool`-primary is what keeps the checked computation the primary
  artifact. **THE GAP:** an implementation deriving membership by eliminating an
  `Ω`-valued predicate computes the same answers, so no value-level case
  separates them — the observation is structural, and stated as such rather than
  dressed as a value assertion.

### surface/operators/membership-fixity-is-the-comparison-band

- spec: `33 §6.1`, `33 §6.3`
- given: `∈` occurrences parsed alongside `≤`, `≥` and `≠`, and reached under an
  import alias.
- expect: `∈` is `infix 4`, the same band, on every path. Live on this base.
- why: fixity is declared of the binding's canonical identity (`33 §6`), so a
  path-dependent parse would mean an implementation attached it to the spelling.
  **THE GAP:** a single-path parse cannot see path-dependence.
