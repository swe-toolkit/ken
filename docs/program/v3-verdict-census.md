# V3 verdict census

Measured on `wp/V3-VERDICT-CENSUS` from exact base `006786c6`.

This is a census of the named test corpus, not of the shipped language. It
reports what those tests produced and licenses no conclusion about prover
adequacy or solver benefit.

## Method

The census ran the existing obligation producers. It did not derive outcomes
from the classifier or prover source. A temporary `cfg(v3_census)` observation
inside `attempt_obligation` recorded, for each actual call:

- obligation id, provenance, route, and `phi`;
- the returned verdict;
- the result of calling the existing `ipc_search` and kernel `check` immediately
  before the real attempt; and
- `trusted_base()` before and after, including whether an `Unknown` hole was new
  and present afterward.

The observation used the existing search and kernel check; it did not reproduce
their rules. The relevant signatures are
`ipc_search(ctx: &Context, phi: &Term, depth: usize) -> Option<Term>`
(`crates/ken-elaborator/src/prover.rs:434`) and
`check(env: &GlobalEnv, ctx: &Context, t: &Term, ty: &Term) -> KernelResult<()>`
(`crates/ken-kernel/src/check.rs:386`): both receive only shared references. A
source search under `crates/ken-kernel/src` found zero uses of `RefCell`,
`Cell`, `Mutex`, or `OnceCell`, so the kernel check has no interior mutability
hidden behind those shared references. Thus the observation did not change the
following real attempt. The temporary observation was then removed. The final
diff under `crates/ken-elaborator/src/prover.rs` is empty.

These targeted commands ran, each with `KEN_VERDICT_CENSUS=1`,
`RUSTFLAGS='--cfg v3_census -Aunexpected_cfgs'`, `--nocapture`, and
`--test-threads=1`:

```text
scripts/ken-cargo test -p ken-elaborator --test v3_acceptance
scripts/ken-cargo test -p ken-elaborator --test v4_acceptance
scripts/ken-cargo test -p ken-elaborator --test sec1_acceptance
scripts/ken-cargo test -p ken-elaborator --test t1_acceptance
scripts/ken-cargo test -p ken-cli --test t2_acceptance
scripts/ken-cargo test -p ken-elaborator --lib ifc::
```

All six commands passed. The four Elaborator integration drivers ran 66 tests,
the CLI driver ran 14 tests, and the `ifc::` filter ran zero tests with 123
filtered out. No workspace command ran.

## Distribution

| Driver | Proved | Disproved | Unknown | Attempts |
|---|---:|---:|---:|---:|
| `v3_acceptance` | 3 | 1 | 4 | 8 |
| `v4_acceptance` | 5 | 1 | 9 | 15 |
| `sec1_acceptance` | 3 | 0 | 2 | 5 |
| `t1_acceptance` | 3 | 0 | 5 | 8 |
| `t2_acceptance` | 4 | 0 | 2 | 6 |
| `ifc::` | — | — | — | **not run: filter selected zero tests** |
| **Total** | **18** | **2** | **22** | **42** |

The route and verdict cross-tabulation was:

| Route | Proved | Disproved | Unknown | Total |
|---|---:|---:|---:|---:|
| D | 2 | 2 | 20 | 24 |
| FO | 16 | 0 | 1 | 17 |
| HO | 0 | 0 | 1 | 1 |

## Unknown inventory

Every one of the 22 `Unknown` outcomes recorded `ipc_search = None`. Zero
recorded a candidate that the kernel rejected.

| Route | Syntactic `phi` shape | Count | Search observation |
|---|---|---:|---|
| D | closed bare `Const` atom | 20 | `None` |
| FO | nested `Pi`, the corpus's encoded excluded-middle goal | 1 | `None` |
| HO | `Eq Omega0 (Const _) (Const _)` with free variables | 1 | `None` |

The twenty D atoms comprise sixteen rendered as `g3`, two as `g4`, and two as
`g570`; those renderings are environment-local ids, not three semantic classes.
The constructor floor is the same in all twenty: `Term::Const`.

The distinction required by AC-3 is therefore measured, not collapsed:
search incompleteness accounts for 22 of 22 Unknowns, while kernel rejection of
a proposed IPC certificate accounts for 0 of 22.

## Trusted-base cost

The verdict count and postulate count were measured separately:

- `Unknown` verdicts: 22;
- new hole ids attributed to the corresponding attempt: 22; and
- new hole ids present in `trusted_base()` after the attempt: 22.

They agree in this corpus. The agreement was not assumed from the current
one-hole implementation.

The tests do not share one `GlobalEnv`, so there is no truthful single shipped
session denominator. Using the actual unit available here, the 42 post-attempt
environment snapshots contain 1,058 trusted-base entries in total. The 22
newly attributed hole entries are therefore **22/1,058 = 2.08%** of that
execution-weighted snapshot total. This fraction counts an environment again
when a test performs another obligation attempt; it is a corpus-execution
measure, not a claim about a persistent Ken session.

The unaggregated measurements make that boundary visible. Immediately after
each Unknown, the new hole was:

- 1 of 3 trusted-base entries in nineteen environments;
- 1 of 2 in one environment; and
- 1 of 110 in two CLI environments.

## Structurally unreachable observed shapes

The observed free-variable equality confirms the first pre-stated shape. It
routes to HO, and the current IPC search has no equality rule; its actual search
result was `None`.

The twenty closed bare atoms also expose a structural limit in the measured
contexts: they have no matching hypothesis, and the current search can only
introduce `Pi` or `Sigma`, find an equal hypothesis, or project a `Sigma`
hypothesis. Their actual search results were all `None`.

The one nested-`Pi` excluded-middle goal likewise returned `None`. The current
search introduces its binders but has no rule that can use the resulting
higher-order hypothesis to construct the remaining atom.

The corpus did **not** produce either of the other two pre-stated shapes:

- no observed `phi` contained a `Constructor` or `Elim`, so this run does not
  measure the cost of case analysis; and
- no observed goal required using a quantified hypothesis at a different
  instance, so this run does not measure quantifier instantiation.

The current engine has no `Constructor` or `Elim` rule and hypothesis lookup is
exact-type equality, so those shapes remain structurally outside its rules.
Their cost is unmeasured here, not zero.

## Coverage boundary

All 42 attempted triples carried `ProvKind::Prove`. The shipped elaborator can
also generate `Ensures`, `LawField`, `PartialPrim`, and the call-precondition
classification used for FFI runtime checks. This corpus exercised none of those
four provenance classes.

The `ifc::` unit-test filter passed but selected zero tests and produced zero
attempts. The Sec1 driver directly exercised five prover calls, but this run did
not observe a call through `RelationalClaim::check` in `src/ifc.rs`.

Not run: other Elaborator or CLI integration suites, other workspace crates,
ignored tests, doctests, examples, catalog programs, or a full shipped-language
session. No obligations were added to fill those gaps. Consequently the
distribution above is only the named corpus distribution and cannot establish
how often the shipped language produces any route, verdict, or trusted-base
fraction.
