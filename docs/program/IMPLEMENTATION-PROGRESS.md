# Implementation progress — the build backbone

**Owned by the Steward** (`agent/playbooks/federation/steward.md §2a`). This
file tracks execution **against the implementation DAG**
(`05-implementation-dag.md`), the build's analog of `spec/SPEC-PROGRESS.md`.
It **survives compaction**: on a cold start or after a compact, read this
first, then continue from the frontier (below). Update it **every synthesis
pass and on every WP state change**. The plan lives in `05`; this file
tracks *progress against it*. Run until complete, blocked, or instructed
(§2b).

**This file holds CURRENT STATE ONLY, and it is GENERATED** — edit
`docs/program/issues/*.md` and re-run `scripts/gen-progress.sh`; hand edits
here are overwritten. The full chronicle — every prior "live state"
snapshot, the detailed evidence trail for every merged WP, and the
day-by-day session logs back to project start — lives in
[`diary/`](diary/INDEX.md). If you need *why* a past call was made, or the
mechanism detail behind a closed WP, start there;
[`diary/CURRENT-BRIEFING.md`](diary/CURRENT-BRIEFING.md) carries the live
operator briefing and the Steward's resume state.

**Status legend:** `draft` (not framed / deps unmet) · `ready` (deps met,
unassigned) · `active` (a team is building) · `in-review` (PR open / QA / CI)
· `merged` (landed + retro in) · `closed` (resolved without landing, e.g. a
superseded or withdrawn item). Gates: see `05-implementation-dag.md`.

**★ GENERATED FILE — do not hand-edit.** This file is regenerated from the
frontmatter of every `docs/program/issues/*.md` work-item file by
`scripts/gen-progress.sh`. To change tracked status, edit the relevant
`docs/program/issues/<ID>.md` file and re-run the generator. CI checks that
the committed file matches the generator's output.

## Last generated

2026-08-14 15:48:10Z — from 273 issue file(s) in `docs/program/issues/`.

## Work-item status

| ID | Title | Status | Owner | Size | Gate | GitHub |
|---|---|---|---|---|---|---|
| `A3` | catalog-coverage walker | draft | TBD | TBD | none | — |
| `ABI-A1` | promote ConsoleRead and ClockWallNow to NativeTested with differential evidence | draft | runtime | M | none | — |
| `ABI-A2` | promote FsAppendFile, FsMetadata, FsRename to NativeTested | draft | runtime | M | none | — |
| `ABI-A3` | promote FsReadDirectory, FsCreateDirectory, FsRemoveFile, FsRemoveDirectory to NativeTested | draft | runtime | M | none | — |
| `ABI-M1` | manifest v2 — family-scoped, versioned, generated from family schemas | draft | runtime | L | none | — |
| `ABI-M2` | runtime facility/operation probes, distinct from build-time facts | draft | runtime | M | none | — |
| `ABI-R1` | correct stale filesystem capability prose — scoped roots, rights, symlink policy and no-follow resolution have landed | closed | foundation | S | none | — |
| `ABI-R3` | generated operation inventory derived from catalog structure — a new operation must be a build break | draft | runtime | M | none | — |
| `ABI-REVOKE` | runtime revocation membrane — the deferred runtime face of 62 §4 | draft | runtime | TBD | none | — |
| `ABI-S1` | descriptor completion — seek, truncate, sync/data-sync, flags, duplication under explicit inheritance policy | draft | runtime | M | none | — |
| `ABI-S2` | directory streaming — supersedes whole-directory read where streaming is the honest shape | draft | runtime | M | none | — |
| `ABI-S3` | monotonic clocks, sleep/deadlines, and secure kernel entropy | merged | runtime | L | none | — |
| `ABI-S4` | statx-shaped metadata with field-availability bits | draft | runtime | M | none | — |
| `ABI-S5` | terminal basics and process signal disposition at the executable edge | draft | runtime | M | none | — |
| `ABI-S6` | ordinary anonymous and file-backed mappings as opaque runtime-owned regions and bounded byte views | draft | runtime | L | none | — |
| `BUDGET-EFF` | TransferCount.remaining must be bounded by the effective request | merged | verify | M | none | — |
| `BUDGET-EXHAUST` | transfer-budget bound checks are fail-open on variant extension | merged | verify | S | none | — |
| `CAT-C2` | Localized Map/Set key-interface split: a non-canonical carrier becomes a lawful Map/Set key under a weaker key-order dictionary while staying an unlawful Ord key wherever antisym concludes kernel Equal | draft | spec-enclave | M | none | — |
| `CAT-CAPEX` | catalog exhibits no checked capability/authority exemplar — write one against the landed Cap/Auth surface | merged | ergo | M | none | — |
| `CB-HYGIENE` | cranelift_backend facade: strip WP-token narration, separate test material from implementation | merged | runtime | S | none | — |
| `CI-ASSERTIONLESS-L1` | Four registered conformance claims whose only cover does not check them — l1_acceptance.rs, three ignored and one live, green, and counted as cover | merged | verify | S | none | — |
| `CI-DOCTEST-UNEXECUTED` | CI runs no --doc step on a premise that is false -- doctests are collected but never executed, and the positive control for a 20-block compile_fail set is among the dead ones | merged | verify | S | none | — |
| `CI-IGNORED-SWEEP` | nothing in the repo ever re-runs an ignored row, so every skip is write-only and a landed repair ships with its own regression cover switched off | merged | verify | S | none | — |
| `CI-L1-EXECUTING-COVER` | Three executing, green l1_acceptance rows certify conformance cases they cannot check -- sec62 never issues the conversion query its soundness row turns on, sec61 names a row id that does not exist, and ac5_no_implicit_cross_type_coercion is satisfied by an elaboration limitation rather than by the coercion refusal it claims | merged | verify | M | none | — |
| `CI-OLD-PRESTATE-ROW-CURRENCY` | The `old`-capture flip pair still asserts pre-state elaboration is unavailable, which LANG-SPACE-PRESTATE-BIND made false -- and the soundness row's stated relation, `reject/reject at distinct gates, not a verdict flip`, is now the opposite of what the code does | merged | verify | S | none | https://github.com/swe-toolkit/ken/pull/1854 |
| `CI-ROW-CLAIM-COMMENT-FORM` | verify-row-claims extracts only from /// doc comments, so a row claim written with // is invisible to it -- two false soundness certificates survive on main in exactly that form | merged | verify | S | none | — |
| `CI-ROW-CLAIM-NAMESPACE` | verify-row-claims hardcodes surface/ in both its claim and heading patterns, so eight of the nine conformance namespaces are structurally invisible to it -- a claim it cannot see is indistinguishable from a claim that does not exist | merged | verify | S | none | — |
| `CI-SKIPPED-NATIVE-TESTS` | Restore rt_parity_native — dedicated CI job, outlier not fixed | merged | verify | S | none | — |
| `CI-TRACKER-GATE` | Wire the issue-tracker schema + regeneration gate into CI | closed | operator | S | none | 804 |
| `CONF-EVAL-COMPUTED-BOOL-ELIM` | The conformance matrix does not state that a closed computed Bool consumed by the Bool eliminator selects the same method as the corresponding constructor -- the two runtime representations reach the eliminator by independent index derivations and nothing ties them together | merged | spec-enclave | S | none | — |
| `CONF-FMT8-LEVELTOK` | FMT8's fixture is unproducible: the row demands a 'genuine level-token fixture' but the lexer has no Level/Label token kind and never will under endpoint (b) | draft | spec-enclave | S | none | — |
| `CONF-SEC4-REFL-PAIR` | Sec4's C1/C2 refl pair is stale against ADR-0013: the true arm is unreachable and the false arm is green for the wrong reason | draft | spec-enclave | S | none | — |
| `CONF-VERIFY-OLD-ROW-UNSATISFIABLE` | The seed's only unclaimed row states expect: accepts against a landed elaborator that rejects unconditionally, and the Coverage map rolls it up as a satisfied family | merged | spec-enclave | S | none | — |
| `CONF-VERIFY-SPEC-SYNTAX-PHANTOM-CLAIMS` | Four v1_acceptance tests claim verify/spec-syntax conformance rows that were never authored -- invisible until the row-claim checker's namespace widening, and now a mechanical merge blocker for CI-ROW-CLAIM-NAMESPACE | merged | spec-enclave | S | none | — |
| `DOC-AGENT-CITE` | agent core modules name normative authorities as a reading list rather than binding them to claim classes, so seven of seven cold runs made material claims without citing the sources D2 requires | merged | doc | M | none | — |
| `DOC-ASBUILT-AGENTS` | As-built slice 6 — reconcile the thirteen-page agents corpus against its 7 shared drifted sources; it is instructions machines follow, not prose people skim | merged | doc | M | none | — |
| `DOC-ASBUILT-AUDIT` | As-built reconciliation — 28 cited sources have drifted from their attestations, so the library's currency claim is unbacked corpus-wide | merged | doc | L | none | — |
| `DOC-ASBUILT-CHAPTERS` | As-built slice 4 — reconcile the four remaining reading-ken chapters together; they share 9 sources and their claim classes cross page boundaries | merged | doc | L | none | — |
| `DOC-ASBUILT-EXECUTION` | As-built slice 2 — reconcile 06-execution.md against its 16 drifted cited sources, the largest phase-A population | merged | doc | L | none | — |
| `DOC-ASBUILT-FRAGMENTS` | As-built slice 1 — reconcile fragments.md against its 9 drifted cited sources; it is the keystone because 7 other documents cite it | merged | doc | M | none | — |
| `DOC-ASBUILT-LEDGER` | As-built phase B — the terminal re-stamp: install the reviewed attestation ledger for all 28 drifted rows at once and regenerate library/STATUS.md | merged | doc | S | none | — |
| `DOC-ASBUILT-READER` | As-built slice 5 — reconcile the four reader-facing entry pages against their 6 shared drifted sources | merged | doc | M | none | — |
| `DOC-ASBUILT-SOLUTIONS` | As-built slice 3 — reconcile the exercise/solution PAIR against its 11 drifted cited sources; a stale claim here is a broken answer under a retired question | merged | doc | L | none | — |
| `DOC-ATTEST-LIVING` | attesting living tracker files makes every routine WP status flip redden the currency gate | closed | doc | S | none | — |
| `DOC-CAP-ASBUILT` | The capability chapter tells readers the catalog has no checked authority exemplar; CAT-CAPEX adds one, falsifying that claim in two places | merged | doc | S | none | — |
| `DOC-CATALOG-CONTENTS` | Catalog entry format: rename the `## Index` heading to `## Contents` in 19 entries and remove the 16 reading-path sections | merged | doc | M | none | — |
| `DOC-CURRENCY-ANCHOR` | library/REVISION certifies nothing about the corpus — currency is unchecked | closed | doc | S | none | — |
| `DOC-GATE-CONTROL-BINDING` | validation-gate registry: make the two DOC-GATE-RECORD-AXIS checks orphan-proof by lifting them to pure detectors with committed controls | merged | verify | S | none | https://github.com/swe-toolkit/ken/pull/928 |
| `DOC-GATE-NEEDLE` | schema-gate controls assert on a needle the test itself supplied, so one constraint class is fully vacuous | merged | verify | S | none | — |
| `DOC-GATE-RECORD-AXIS` | validation-gate registry: bind token→runner COVERAGE on the record axis, and close the `kind` vocabulary | merged | verify | S | none | https://github.com/swe-toolkit/ken/pull/922 |
| `DOC-GATE-WIRE-BINDING` | validation-gate registry: bind the kind-vocabulary RULE to its GATE by registering it as a VALIDATION_GATES row | merged | verify | XS | none | https://github.com/swe-toolkit/ken/pull/933 |
| `DOC-PROGRAM-SELF-REFUTE` | three sites of current program law assert assurances the same corpus has already measured as absent, and 12-documentation-program.md now carries both a drift-gate claim and the measurement refuting it | merged | doc | M | none | — |
| `DOC-PROGRAM-WAVE-RECONCILE` | Reconcile the documentation program's wave status against the landed corpus — the status line, the wave table, and the section 4b headers all say map only over bodies that measured otherwise, and produce the residual register that says what the doc ring owes next | merged | doc | M | none | — |
| `DOC-VALIDATION-BINDING` | validation vocabulary claims a 1:1 binding to the gates; nothing binds it | merged | verify | S | none | — |
| `DOC-W0` | documentation Wave 0 — library/ charter and currency substrate | closed | doc | M | none | 830 |
| `DOC-W1` | documentation Wave 1 — the read-Ken spine, taught from checked fragments | closed | doc | L | none | — |
| `DOC-W2` | documentation Wave 2 — agent core modules, task packs, and cold-context evals | merged | doc | L | none | 936 |
| `DOC-W3-DEPDATA` | Wave 3 slice 3 — the dependent-data guide page, the one guide subject of seven with no explanatory coverage anywhere in library/ | merged | doc | S | none | — |
| `DOC-W3-GUIDE` | Wave 3 slice 1 — migrate catalog/guide/ into library/guide/ under migration-local fence verification, conserving all 40 checked fences through the move | merged | doc | M | none | — |
| `DOC-W3-HOWTO` | Wave 3 slice 2 — library/how-to/ recipes scoped by the CLI's seven-subcommand task surface, each grounded in a real diagnostic or checked artifact | merged | doc | M | none | — |
| `DOC-W4-LANGUAGE` | Wave 4 slice 2 — the language reference, scoped to whatever survives a residual measurement against the 625-line page already named `surface-reference` | merged | doc | S | none | — |
| `DOC-W4-RESIDUAL` | Wave 4 slice 3 — the terminal residual measurement across the four remaining reference surfaces and the four indexes, authoring only what survives it | merged | doc | S | none | — |
| `DOC-W4-TOOLCHAIN` | Wave 4 slice 1 — the toolchain reference, plus the D0 report on which of Wave 4's generated facts the toolchain can actually produce today | merged | doc | M | none | — |
| `DOC-W5-CAPABILITY` | Wave 5 precondition — the Librarian's format-capability report: which of Wave 5's nine fact classes the checked artifact format can express today, and therefore whether Wave 5 is authorable or blocked on a generator | merged | doc | S | none | — |
| `DOC-W5A-CARD-FORMAT` | Wave 5 slice 1 — the reference card format, the generated subject index for all 39 packages, and six proving cards across Core and Tooling | merged | doc | M | none | — |
| `DOC-W5B-CARDS-APP-DATA` | Wave 5 slice 2 — apply the settled card format to Application (3) and Data (11): fourteen complete cards | merged | doc | M | none | — |
| `DOC-W5C-CARDS-CAPABILITY` | Wave 5 slice 3 — apply the settled card format to Capability (19): nineteen complete cards, closing the 39-package set | merged | doc | M | none | — |
| `DOC-W5D-INDEXES` | Wave 5 closeout — build the four cross-package indexes the cards can support (declaration/type, law, effect/capability, assurance) and record why the four held-class indexes cannot be built | merged | doc | M | none | — |
| `DOC-W6-AGENT-EVAL` | Wave 6 residual — the cold-context agent evaluation certifies agent_core_ready against a corpus 3.4x smaller than today's, and three of the four pack-selected core modules have changed since | merged | doc | M | none | — |
| `DS-9` | lawful JSON codec — the data-structures tier's acceptance test: a Json value type, encode/decode, and the proved round-trip law, assembled entirely from the landed Core/Data sections | draft | foundation | L | none | — |
| `EFF-SPACE-ENSURES-PRESTATE` | `old` is transparent, so a space operation's `ensures` cannot express the pre/post distinction `36 §4.3` is built on | closed | language | M | none | — |
| `F1-37` | F1 [task-list #37] — bignum Int soundness review for K3 trusted-base promotion | draft | runtime | TBD | none | — |
| `F3-39` | F3 [task-list #39] — reducer: degrade-not-wrap + retire legacy arms | draft | runtime | TBD | none | — |
| `F4` | content-addressing + value-model design (aka PX8-F-PROOF) | draft | foundation+spec-enclave | M | none | — |
| `KERNEL-NESTED-IND` | admit nested strictly-positive inductives in the kernel — structural positivity through declared parameter positions, generated and checked dependent eliminators with one lifted IH per contained recursive occurrence, iota, and surface consumability | active | kernel | L | none | — |
| `KERNEL-RECURSIVE-RESULT-SURFACE` | A source term that denotes the kernel-supplied recursive method result for a lifted recursive field -- the missing surface capability that makes an unbounded residual-All fold expressible | merged | spec-enclave | M | none | — |
| `KERNEL-SUBST-OUTER-INDEX-SCOPE` | Rule whether kernel subst_outer should bound its parameter index -- it panics on an out-of-range params[p_idx] and is defended only by a reachability argument enumerating one of its 29 call sites | draft | spec-enclave | S | none | — |
| `KW-ORACLE-CLOSURE` | close the KW-THEOREM source oracle structurally — the occurrence sweep is never applied, and the file population is a five-arm hand enumeration | merged | language | S | none | 986 |
| `KW-ORACLE-REMOVE` | Delete the whole-tree source-text oracle: it asserts facts about repository text, which is now a prohibited test subject | merged | language | S | none | 1035 |
| `KW-THEOREM` | rename the surface keyword `lemma` to `theorem` | merged | language | M | none | — |
| `LANG-COMMENT-CLASSIFIER-SHARED` | The lexer and the lossless layer each carry their own copy of the block-comment classification -- the `{--`-before-`{-` ordering twice and both end-scanners twice -- so their agreement is held by a comment saying they mirror each other `exactly` and by tests, with nothing failing to compile when they diverge; and the divergence they can reach disagrees about comment KIND rather than acceptance, which the `is_ok()`-comparing net cannot see and round-trip cannot see either | merged | language | S | none | — |
| `LANG-COMMENT-POPULATION-PARITY` | The B1 round-trip helper counts a comment population that production stopped using -- `assert_round_trip` filters `TriviaKind::LineComment` while `attach_comments` filters `is_comment()`, so the whole-`catalog/` walk is green only because no catalog source contains a block or doc comment, and the first author who writes one gets a red in a different crate accusing the attachment mechanism of losing a home | merged | language | S | none | — |
| `LANG-DECEQ-CHAR-LAWFUL-INSTANCES` | `37 §2.5` defers the proof-carrying `DecEq String` / `Ord String` instances as a `tracked follow-on` because the transport needs a lawful `DecEq Char` that is not landed -- and the follow-on was never filed, so the second unowned obligation in this chapter sits in spec prose with no tracker row | draft | language | unsized | operator | — |
| `LANG-EXHAUSTIVENESS-WITNESS-PAYLOAD` | 34 §4.1 requires naming the unmatched PATTERN WITNESS, and ExhaustivenessError's payload is a single String documented as a constructor NAME -- so no change at any emission site can discharge the obligation, and it reads as satisfied today only because every landed omission test uses a zero-arity constructor where name and most-general pattern coincide | merged | language | M | none | — |
| `LANG-FOREIGN-NAME-CONTROL-CHARS` | Escape decoding made `foreign` symbol and library names able to carry an embedded NUL, where the source text `\\0` previously reached the compiler as two harmless characters -- a NUL in a name that will cross a C-ABI boundary is the classic truncation vector, the declared and effective names silently differ, and there is no consumer today only because the loader path has not landed yet | merged | language | XS | none | 2128 |
| `LANG-FOREIGN-NAME-FORMAT-CHARS` | Unicode Cf format characters -- bidi overrides, zero-width joiners, U+FEFF -- are a visual-spoofing vector at the same `foreign`-name trust boundary the Cc control-character check just closed, and they are a DIFFERENT vector: not truncation but two distinct declarations rendering identically to the reviewer doing the check | draft | language | XS | operator | — |
| `LANG-GADT-SEQUENCE-TRACKER-GAP` | `34 §8` names four `SURF-gadt-*` build WPs and all four have frames in `docs/program/wp/` -- none has a tracker node, so `gen-progress.sh` shows the whole dependent-constructor area as absent, while the code has in fact moved past every one of the four frames' stated baselines | merged | language | S | none | — |
| `LANG-LEX-HEX-FLOAT` | Both spec literal tables give `0x1p-3` as a `Float` form, but the lexer has no hex-float path at all -- and unlike every other numeric form in this arc it cannot be reached by handing a string to `parse::<f64>()`, because Rust's float parser rejects hex-float syntax, so the value must be assembled and correctly rounded by hand | merged | language | M | none | https://github.com/swe-toolkit/ken/pull/1885 |
| `LANG-LEX-NUMERIC-FORMS` | The lexer implements none of the numeric literal forms 31-lexical and 35-numbers list besides bare decimal -- no `1_000` separators, no `0xFF`/`0b1010`/`0o17` radix integers, no `0x1p-3` hex float -- and `1e-9`, which both spec tables give as the canonical Float example, does not lex as a float at all because the exponent branch is gated on having seen a dot | merged | language | M | none | https://github.com/swe-toolkit/ken/pull/1881 |
| `LANG-LEX-PROJECTION-ADJACENCY` | The positional-projection lexer guard tests raw character adjacency, so exactly one of four spacing variants fails -- `p.1.2`, `p.1 .2` and `p. 1 .2` all lex as two projections while `p. 1.2` lexes as `Dot, FloatLit(1.2)` -- and the refusal comes from the number scanner rather than from any grammar rule | merged | language | S | none | https://github.com/swe-toolkit/ken/pull/1864 |
| `LANG-LOSSLESS-COUNT-ASSERTION-RETIRE` | `assert_round_trip`'s comment-count assertion cannot fire -- production reconciles the same two sets and refuses first, so `kenfmt_b1_lossless.rs:27` states a theorem while its message reads as a live check, and the `pub fn is_comment` it is the sole external caller of exists only to feed it | merged | language | XS | none | — |
| `LANG-MATCH-DIAGNOSTIC-PROSE` | The match checker's two error variants now SAY things that are false -- the exhaustiveness message calls an applied pattern a constructor, the reachability doc cites 34 §5 (Refinement types) for an obligation in §4.2, and a test file's header still advertises a gap the same file's own regression test proves closed | active | language | S | none | — |
| `LANG-NATIVE-PRODUCTION-STACK-FOOTPRINT` | `ken-cli` native production runs `px4b_native_production` at effectively zero stack margin -- base passes with a few hundred bytes to spare, so any candidate adding a few hundred bytes aborts it, and `98e6ac51` is the trigger that exposed this rather than its cause | merged | language | M | none | — |
| `LANG-NESTED-MATCH-LIFT-ALIGNMENT` | the generated-All aligned check path is lost when the lifted match is nested under an outer contribution, so a residual-Bag fold cannot type-check | closed | language | M | none | — |
| `LANG-POW10-CASCADE-LITERAL-CLAUSE` | The pow10 generator's own doc comment says every branch is a concrete literal, which its own recursion refutes in the same way the elab.rs copy did -- but its conclusion rests on a DIFFERENT and TRUE property (no saturating/min/clamp anywhere in the generated cascade), so this is a wording repair on a sound argument, not a second false justification | merged | language | XS | none | — |
| `LANG-PRELUDE-COLLECTIONS` | `37 §9` requires the List combinators delivered in the prelude and they are declared inside a test file instead -- `prelude.rs` supplies `data List` and no operation over it, so a program that imports the prelude has a list type and no way to map over it; and `filter` was deferred on the ground that `Bool` is an opaque non-matchable primitive, which is no longer true, with the promised follow-on never filed | merged | language | S-M | none | 2144 |
| `LANG-PRELUDE-COMBINATOR-BLOCK-DELTA` | `AC-6`'s doc says a live differential is impossible because `ElabEnv::new()` has no 'before' env -- true of an ENV-level bracket and false of a BLOCK-level one, which is the established idiom four sites away in the same crate, so the instrument the comment says does not exist is writable, id-keyed, and fails in production | merged | language | S | none | 2189 |
| `LANG-PRELUDE-ELABORATION-DEPTH` | Elaboration has an unstated stack requirement that exceeds Rust's 2 MiB spawned-thread default: every compilation elaborates the whole prelude, `elab.rs:997` measures ~115 KiB of headroom out of 2 MiB, and thirteen sites across four crates independently bumped their thread to 256 MiB without any stated rule -- so the rest of `37 §9` is a queue of prelude additions spending a margin nobody measures and no site states | merged | language | S | none | — |
| `LANG-RECORD-STACK-OVERFLOW` | The record-literal surface work aborts a real `ken-cli` native compilation with a stack overflow -- `mrc_4a_cross_crate_census_and_its_controls` SIGABRTs at every SHA of the arc including the one carrying the 143-line stack rework, so the rework is not the repair; the arc's own depth fixture never detected it because it builds match arms with `=>`, which is not a Ken token | merged | language | M | none | — |
| `LANG-REFINED-FALLBACK-COLDNESS-CLAIM` | The doc comment justifying LANG-NATIVE-PRODUCTION-STACK-FOOTPRINT's -3120 saving says the pow10 cascade's arms are all bare literals, which the generator refutes -- only the True arms are, every False arm is the next nested match, and the innermost is an application; the conclusion survives, the recorded reason does not, and it misleads in both directions | merged | language | S | none | 2195 |
| `LANG-SELECTOR-CLASSIFIER-RESIDUAL-DIAGNOSTIC` | The selector's non-universe classifier arm reports the elaborator's own refusal as KernelRejected and fabricates a Type(?0) expectation that Omega would equally satisfy | merged | language | S | none | — |
| `LANG-SELECTOR-SORT-SPLIT-ELAB` | Implement the sort-split recursive-result selector in the elaborator -- parse `recursive result for x` and `induction hypothesis for x`, classify the selected hidden result by sort, and remove `structural result of x` from the crates | merged | language | L | none | — |
| `LANG-SORT-META-CAPABILITY` | Rule whether a term/sort metavariable representation is authorized -- the elaborator cannot today leave a selected result undecided between Type and Omega, so the spec's conditional ambiguity clause has an unreachable antecedent | draft | spec-enclave | S | none | — |
| `LANG-SPACE-PRESTATE-BIND` | `old` in a block-space operation's `ensures` still fails closed, though the cell environment it was waiting for now exists -- bind s_pre/s_post and elaborate the Hoare pair against the state transformer | merged | language | M | none | https://github.com/swe-toolkit/ken/pull/1848 |
| `LANG-STACK-ARC-EVIDENCE-USABILITY` | The trusted-base guard now localizes the bracket but reports a bare GlobalId, so it names no offender; and both frame-size figures this arc produced cite objdump without naming the artifact, so neither is reproducible by the next reader -- three repairs that make the arc's own evidence usable | merged | language | S | none | — |
| `LANG-STRUCTURAL-RESULT-ELAB` | Implement the structural-result selector in the elaborator -- derive the field/evidence/result association from the kernel method telescope and elaborate `structural result of x` to the hidden recursive method result | merged | language | L | none | — |
| `LANG-SURFACE-BLOCK-COMMENTS` | `31-lexical.md:562-567` specifies nestable block comments `{- ... -}` and doc comments `--- ...` / `{-- ... --}` attaching to the following declaration, and neither exists -- the semantic lexer's skip_ws_comments knows only whitespace and `--`, and TriviaKind carries only Whitespace and LineComment, so the two independent scanners that must agree about comments have only ever been exercised on the one form that cannot nest and cannot fail to terminate | merged | language | M | none | — |
| `LANG-SURFACE-DECIMAL-PRECISION` | `Decimal` is specified with an arbitrary-precision coefficient and the spec explicitly forecloses a fixed-width one, but the surface caps it at `i64` across three carriers -- `Token::DecimalLit(i64, i32)`, `NumLit::Decimal(i64, i32)`, and `NumericLitVal::Decimal { coeff: i64 }` -- and the lexer refuses a wider coefficient outright | merged | language | M | none | https://github.com/swe-toolkit/ken/pull/1876 |
| `LANG-SURFACE-IF` | `if e then t else f` is required by 32-grammar §3 and is wholly absent -- no token, no keyword-map entry, no parser arm, no AST node -- while its stated elaboration target (real matchable `data Bool = True | False`) has been pre-registered since ES2 | merged | language | M | none | https://github.com/swe-toolkit/ken/pull/1819 |
| `LANG-SURFACE-INT-PRECISION` | `Int` is specified arbitrary-precision and the kernel already carries `Term::IntLit(num_bigint::BigInt)`, but the surface truncates to `NumLit::Int(i128)` through a lossy `n as i128` cast, and the lexer implements none of the `0x`/`0b`/`0o`/`_` forms that 31-lexical lists | merged | language | M | none | https://github.com/swe-toolkit/ken/pull/1871 |
| `LANG-SURFACE-LITERAL-ESCAPES` | The lexer performs NO escape processing -- its single string form pushes every character verbatim, so the escape repertoire that SPEC-LITERAL-ESCAPE-PIN just closed is entirely unimplemented, and Char literals, byte literals, byte strings and raw triple strings do not exist at all despite String, Char and Bytes all being built prelude targets | merged | language | M | none | 2119 |
| `LANG-SURFACE-PAIR` | Pair literals, positional projections, and the Sigma type production are required by 32-grammar and wholly absent from the surface -- `Token::Times` is lexed for `×` and consumed by nothing, `(a, b)` is a parse error, and `.1`/`.2` fall outside the projection guard -- while the kernel's Sigma/Pair/Proj1/Proj2 are complete and already exercised by records | merged | language | M | none | https://github.com/swe-toolkit/ken/pull/1859 |
| `LANG-SURFACE-RECORD-DECL` | `33 §2` specifies `record Point { x : Int, y : Int }` and `record` is already a reserved keyword, but the lexer emits no token for it and the parser has no declaration form -- while the elaboration target is complete and already exercised, since `class` elaborates to exactly the right-nested Sigma chain a record needs and `p.x` already parses and resolves, refusing only at `infer_proj` because that lookup scans the class registry | merged | language | M | none | — |
| `LANG-SURFACE-RECORD-LITERAL` | `33 §2` names record literals `{ x = 1, y = 2 }`, punning `{ x, y }` and functional update `{ p | y = 3 }` as having their expected definitional behaviour, and none of the three parses -- expression-position `{` has no arm in `parse_atom_expr_base` at all, so the brace fork the sibling frame warned about does not exist here: refinement braces live in `parse_type`, which is a separate parser | merged | language | M | none | — |
| `LANG-TRIVIA-KIND-MAPPING-PIN` | `LANG-COMMENT-CLASSIFIER-SHARED` made scanner divergence unrepresentable and moved the surface one hop to `From<CommentKind> for TriviaKind`, which is now the sole place a classification becomes a behaviour -- the completeness axis is closed by the compiler but the per-arm mapping is asserted nowhere, and the one fixture that covers the block form is a configuration where the doc rule and the positional heuristic return the same answer, so a Block/DocBlock transposition compiles and reds nothing | merged | language | XS | none | — |
| `LANG-TRUSTED-BASE-LABEL-KIND-TAG` | The `AC-6` trusted-base enumeration is blind to the one movement it exists to catch -- `trusted_base_labels` flattens kernel-declaration and surface names into one untagged `Vec<String>`, so a postulate becoming a primitive under the same spelling renders identically across all 107 entries, and the injectivity the fallback depends on is measured rather than enforced | merged | language | XS | none | — |
| `LANG-VIEW-RETIRE` | Operator ruling SURF-1 retired the single definition keyword `view` and split it into `const`/`fn`/`proc`, but the landed elaborator still accepts it -- and `view` is not an alias: it takes an EARLY RETURN out of the bidirectional purity check that `33 §1` calls a hard error, so every definition still spelled `view` has never been checked for the effect discipline the spec requires | merged | language | M | none | — |
| `LIB-GATE-DECOUPLE` | main is red on two library documentation-census gates: the currency gate the operator decoupled from merges still fires from inside CI, and a doc-only merge invalidated the ledger unreported | merged | verify | S | none | 1039 |
| `LOADER-CITE-ANCHOR` | LOADER-STALE-PREMISE cites the spec by line number (:147-158) — rots silently in the one catalog file outside the currency gate | merged | doc | XS | none | — |
| `LOADER-STALE-PREMISE` | \"no disk loader yet\" is stale in 9 places — including already-landed library/ content | merged | doc | S | none | — |
| `MAP-TRANSPORT-CODEC` | If Map/Set need a portable canonical serialization, it is ordinary package Ken — not a runtime primitive: settle whether a codec is required at all, and if so place it out of trusted_base() | closed | ergo | TBD | none | — |
| `MODELS-TIER` | agent/MODELS.md — the Runtime seating is the fleet-wide norm, not an exception | draft | steward | S | none | — |
| `NATIVE-HANDLE-CARRIER` | Native build-pipeline completeness — a constructor-private resource-carrying handle fails checked-core body-view lowering (MissingClosureMetadata) when it crosses the higher-order withBuffer normalization boundary | draft | runtime | M | none | — |
| `ORACLE-VIS-CHECK` | replace the text-pin oracle in px4b_native_production.rs with a real visibility check | merged | runtime | S | none | — |
| `ORACLE-VIS-PACKAGING` | replace the text-pin visibility oracle on build_process_starter_executable_artifact | merged | runtime | XS | none | — |
| `PROG-TRACKER-MERGE-DRIVER` | Two docs candidates in flight ALWAYS conflict on generated IMPLEMENTATION-PROGRESS.md and nowhere else -- and the recorded reason merge=union was rejected is FALSE at the current generator, so D0 re-derives the warrant before anything is built | ready | steward | S | none | — |
| `PUB-VERIFY` | scripted-pr-automerge.sh exits 0 on a failed push | closed | steward | S | none | — |
| `PX10` | processes — declarative spawn plan, deny-by-default inheritance, pidfd identity, typed child-exit observation | draft | runtime | L | none | — |
| `PX11` | sockets — typed addresses, bounded send/receive, explicit option families, injected resolver capability | draft | runtime | L | none | — |
| `PX12` | readiness — nonblocking transitions, epoll/eventfd/timerfd/signalfd, cancellation and timeout IN THE OPERATION TYPE | draft | runtime | L | none | — |
| `PX8-ERRID-ALLOC` | ResourceErrorV1 has no allocation-failure identity and buffer allocation is infallible, so PX8's allocation-distinct-from-BufferLimit row cannot be produced at all | merged | foundation | M | none | — |
| `PX8-ERRID-SCOPE` | PX8 clause-(a) A2b — five PR-C error identities have no independent production-reaching evidence; Architect ruled all five inside the closure | merged | verify | L | none | — |
| `PX8-F-CAP-41` | PX8 clause-(a) behavior blocker — closed buffer endpoint (start==capacity) must derive zero-effective ReadEof, not host-reject | draft | foundation | M | none | 41 |
| `PX8-SPAN-PROV` | PX8 clause-(b) gap — BufferSpan carries no originating-buffer identity; freeze accepts a same-shape span from a different buffer | merged | spec-enclave | M | none | 914 |
| `PX8-WROTE-ABS` | PX8 clause-(a) evidence gap — interpreter capped-short Wrote lacks an absolute oracle; PR-C error identities unreached | merged | verify | S | none | — |
| `PX8` | partial/positioned IO — the completion program's root; closure condition | draft | runtime | L | none | — |
| `PX9` | cross-domain System.Error — semantic identity, raw errno, operation, resource, safe context, and honest retry classification | draft | foundation | L | none | — |
| `Q-CLAIM-CLOSURE` | Q-RESIDUE adversary findings — claim-loss in multi-claim test blocks, plus R1/R2/R3 | merged | runtime | S | none | — |
| `Q-CLAIM-COMPARE-ORD` | claim-loss in list_instance_routes... (compare_ord) — both routing claims dropped, replacement only instantiates Bool | merged | runtime | XS | none | — |
| `Q-RESIDUE` | the Track Q rework residue — 10 tests, folded from Q3-Q7 | closed | runtime | S | none | 818 |
| `RT-4B-C2-REACHABILITY` | Establish whether a witness driving through `ken-elaborator` can reach the `D2f` observation at all, and at what cost -- the question 4b opened with, never answered, and the reason every 4b measurement so far has been taken on in-crate D2j fixtures instead of the real C2 source; the answer decides whether the reach node re-points or 4b's honest status becomes blocked-on-cross-crate-expressibility rather than awaiting-a-count | closed | runtime | S | none | — |
| `RT-4B-ENUMERATION-INPUT-SIZE` | Gate 4b's observer records whether an oriented plan ARRIVED but not how large the population it walks is, so `keys = []` cannot be read as either lawful absence or a missing producer relation -- and the scalar that would settle it is the admitted-discovery ledger's length, not the oriented plan's, because that ledger is what candidate enumeration actually iterates | merged | runtime | S | none | 2109 |
| `RT-4B-OBSERVATION-FEATURE-GATE` | Re-gate the existing D2f observation behind an off-by-default Runtime feature with a doc-hidden feature-scoped accessor, and prove the feature inert by comparing artifacts from TWO COMPILATIONS -- not a runtime toggle inside one, which is what the landed switch already proves and is a different claim; this is the increment the C2 answer made conditional, and it is what unblocks re-pointing the reach node at the real witness | merged | runtime | M | none | 2123 |
| `RT-4B-UNIQUENESS-GATE-ATTRIBUTION` | Gate 4b's observation route is exhausted -- four admitted discoveries enter enumeration and zero candidates leave, and none of the fourteen elimination routes is attributed; the Architect named `fusion_unique_static_body_triple` as the cheapest and most informative place to recover attribution, but instrumenting one gate is only decisive if the eliminations actually happen there, and nothing measured says they do | draft | runtime | S | none | — |
| `RT-4B-UNIQUENESS-GATE-REACH` | Count whether any candidate reaches the twelfth of thirteen elimination exits before building anything that classifies what happens there -- a call-site counter at `fusion_unique_static_body_triple`, changing no signature, no control flow and no plan, which decides whether the attribution increment has a subject at all | ready | runtime | S | none | — |
| `RT-4B-WALKED-CONSTANCY` | Two assertions landed in one candidate compose into a result neither states -- the five input populations read `(4, 2, 0, 2, 1)` identically whether fusion forms or is perturbed so it cannot, so `walked` discriminates input size and nothing downstream of it; the observation's own doc calls this a gap in attribution, which is a weaker claim than what was measured, and the next reader of a non-zero walked count is one node away | merged | runtime | XS | none | 2116 |
| `RT-AGG-COMPOSE` | escaping two Resources into one aggregate (Prod (Resource _) (Resource _)) fails at erasure — checked endpoints do not compose | draft | runtime | TBD | none | — |
| `RT-BACKEND-MODULE-SPLIT` | Split the oversized ken-runtime backend files into modules — the follow-on to the recursive-descent retirement, not an interlude in it | draft | runtime | M | none | — |
| `RT-BACKEND-PRIMITIVE-LOWERING-SPLIT` | Move the primitive-lowering family to its own module — the first production slice of the backend split, and the architectural release point for NATIVE-HANDLE-CARRIER | draft | runtime | M | none | — |
| `RT-BACKEND-SPLIT-CENSUS` | Stage A of the backend module split — five inventories over the post-retirement tree, before any code moves | draft | runtime | M | none | — |
| `RT-BODY-OCCURRENCE-PROVENANCE` | Non-root function seeds alias the scheduling entry as the body origin, so the source traversal enters the entry and never reaches the real body occurrence or its join subtree | merged | runtime | M | none | — |
| `RT-C2-DRIVER-STAGE-ATTRIBUTION` | The D5 observation identity driver reports every non-zero nested exit as `nested {} compilation failed`, so the one message AC-2 itself produces names the wrong stage -- plus one clause recording why the compiled-feature const must stay adjacent to the gate it mirrors | ready | runtime | XS | none | — |
| `RT-C2-OBSERVATION-ARTIFACT-IDENTITY` | The always-on `dasm-c2-observation` feature has no artifact-identity control, and the always-on choice is what makes the off-configuration unreachable from the crate the controls live in -- the sibling's nested-cargo A/B needs a carrier feature before it can be reused | merged | runtime | S | none | — |
| `RT-C2-OBSERVATION-SELFCHECK-CRATE-MISMATCH` | The artifact-identity control's anti-vacuity self-check reads `ken-cli`'s `dasm-c2-observation` feature while the property that decides whether the two artifacts differ is `ken-runtime`'s -- they agree today and nothing holds them together, so a future `ken-cli` dev-dependency enabling the runtime feature would make the off side ON with the assertion still reading `disabled` and no signal at all | merged | runtime | S | none | 2196 |
| `RT-CALL-EDGE-EXECUTABILITY-AXIS` | executable_call_edges probes a body-axis set with an entry-axis key, so a template-only callee whose axes differ survives the filter and fails later as a forward-declaration error | ready | runtime | S | none | — |
| `RT-CANDIDATE-LEDGER-RESIDUALS` | Two named population questions on the merged candidate/disposition ledger were never reached, and the node that could have covered them is closed | ready | runtime | S | none | — |
| `RT-CARRIED-CONTINUATION-RESUME` | A carried scrutinee reaching a continuation frame has no resume path — the carried elimination does not implement the Carried x {PendingLet, Active} arm | merged | runtime | M | none | — |
| `RT-CARRIED-ORDINARY-COMPOSITION` | Carried ordinary elimination consumes exactly one frame — a composed suffix behind an ordinary carried eliminator is refused rather than continued | merged | runtime | M | none | — |
| `RT-CARRIED-RESOURCE-SCALAR` | A carried word cannot satisfy a ResourceScalar effect seat -- same Need-not-in-Avail shape as the byte-span gap, different need, different seats | draft | runtime | TBD | none | — |
| `RT-CARRIER-BYTESPAN-OBSERVE` | Carrier byte-span observation capability — every BytesPointerLength seat is SPECIALIZED_ONLY and the carrier has no total emitted byte-span observer, so a carried host result cannot satisfy a byte-span effect seat | merged | runtime | L | none | — |
| `RT-CARRIER-PRODUCER-OCCURRENCE` | a source aggregate reaches the carrier with no planner-issued producer occurrence, so the C2 edge refuses to emit and the nested-payload selection row never exercises its property | ready | runtime | M | none | — |
| `RT-CENSUS-CAVEAT-GUARD` | The identifier-census caveat's staleness guard is an existence check standing in for a count check, so it cannot detect the drift it was written to catch | ready | runtime | S | none | — |
| `RT-CHECKED-IH-REALIZATION-AUTHORITY` | Mint the checked-IH realization authority -- pending marker, oriented plan, call template, slot and parent -- so the ComputationalRecursorClosure capsule is realizable IN PLACE at the source-machine Match seat, without widening the ordinary-Match selector and without any terminal-All licensing | ready | runtime | M | none | — |
| `RT-CLOSURE-BOUNDARY-LANE` | A closure cannot cross the durable boundary -- runtime-local and live-domain only, with no durable lane | draft | runtime | TBD | none | — |
| `RT-COMPMATCH-TREE-SCRUTINEE` | ComputationalMatch refuses a tree-producing scrutinee that is not Bool or a constructor (rt_span_prov) | draft | runtime | TBD | none | — |
| `RT-CONTINUATION-CALL-DISCHARGE` | A planned continuation call is neither directly emitted nor compositionally consumed once the Active resume path goes live — attribution, not repair | merged | runtime | S | none | — |
| `RT-CONTINUATION-EDGE-DISPOSITION` | One planner edge carries both binding projection and a causal call obligation — split the representation so a binding candidate can be settled InlineNoCall without ever entering the call-discharge partition | merged | runtime | M | none | — |
| `RT-CONTKEY-CONSUMING-OCCURRENCE` | The continuation specialization key names the owner of the continuation's own occurrence and has nowhere to name the occurrence that CONSUMES its answer; the enclosing eliminator is measurably not in hand at the interning site, so the fact must be seeded at the outer-match walk and threaded there -- a plan-construction change, not a field addition | merged | runtime | M | none | — |
| `RT-CONTKEY-ELIMINATOR-ORIGIN-UNFIRED` | consuming_occurrence carries two fields and only one of them is checked -- eliminator_origin is copied into the re-derivation before the comparison, so AC-1's assert is x == x on that field, and AC-2's mutation perturbs only the field that was already independent, leaving step 1 never fired | active | runtime | XS | none | — |
| `RT-CONTSPEC-ABI` | ContinuationSpecialization slice 2 — land the explicit unit/descriptor projection and the ABI, owner/lifetime/affinity and zero-allocation negative gates, still DORMANT | merged | runtime | M | none | — |
| `RT-CONTSPEC-ACTIVATE` | ContinuationSpecialization seam 2 — lowering activation and exact-use consumption: direct call before the identity-erasing join, active emitted owner, affine call occurrence, JoinArm consumption, gating the 37-row lower-owned population | merged | runtime | L | none | — |
| `RT-CONTSPEC-ASSEMBLY` | ContinuationSpecialization seam 1 — the lawful assembly: extract the accepted branch-scope helper and its feature-gated harness onto the landed slice 0-2 blobs, unactivated, and prove the prior-slice surfaces are untouched | merged | runtime | M | none | — |
| `RT-CONTSPEC-LEDGER` | ContinuationSpecialization seam 3 — retire the boundary-use schema: the four BoundaryUse axes are compile-time constants that no lowering, ABI, selection, lifetime, or emission consumer reads, so they are deleted from the continuation-specialization contract | merged | runtime | S | none | — |
| `RT-CONTSPEC-LOWER` | ContinuationSpecialization slice 3 — attach the token at each producer alternative, emit the direct call before the identity-erasing join, close nested recursion and the ledgers, then ACTIVATE | closed | runtime | L | none | — |
| `RT-CONTSPEC-PLANNER` | ContinuationSpecialization slice 1 — land the planner closure DORMANT: exact ordered projection, full-key interning before discovery, exact causal edge tokens, finite recursion | merged | runtime | M | none | — |
| `RT-CONTSPEC-SUBSTRATE` | ContinuationSpecialization slice 0 — re-derive and independently gate the DORMANT D7 substrate: closed case-emission reachability, exact occurrence/owner/lifetime authority, pre-allocation closure | merged | runtime | M | none | — |
| `RT-CONTSPEC-WITNESS` | ContinuationSpecialization seam 4 — integrated witness and closeout: the native population, the six formerly shadowed rows reclassified, the two host rows rerun, and the campaign closeout record | merged | runtime | M | none | — |
| `RT-CONTSRC-CALLABLE-CONTRACT` | Closed callable-contract arm for continuation sources — a recursive IH is a compiler-only static worker with no value carrier, and the enclosing slot authority is unconditionally a value contract, so its environment sits outside the domain RT-CONTSRC-PRODUCER-LOCAL owns | ready | runtime | M | none | — |
| `RT-CONTSRC-FRAME-FINALIZE` | The continuation availability lifecycle stops one stage short -- Stage 1 interns a `ContinuationFrameRequirement` before the context ids that would resolve it are minted, so Stage 2's `ContinuationFrameIdentity` never runs for the five governed rows and the publication gate correctly refuses to publish an unfinalized claim; the storage-independent view both consumers read is designed, typed and landed, it is simply never finalized | closed | runtime | M | none | — |
| `RT-CONTSRC-PRODUCER-LOCAL` | Producer-local continuation source coordinate — a mid-body value is a third availability class with no ABI seat, so continuation specialization cannot name its environment | merged | runtime | L | none | — |
| `RT-DECL-CLOSURE-PORT` | Transparent-declaration-closure emission port — a retained TransparentDeclarationClosure residual forces the whole object onto the monolithic RecursiveDescent root, which exceeds Cranelift's per-function ceiling | merged | runtime | L | none | — |
| `RT-DESCENT-RETIRE` | Retire RecursiveDescent — delete the migration selector, the residual enum, the authority variant, and the recursive-descent emission lane | draft | runtime | M | none | — |
| `RT-DYNAMIC-ARM-SCALAR-MERGE` | A carried Match arm carrying a nested-IH result cannot satisfy merge_scalar_operand -- measure what the arm actually produces before bounding the repair | merged | runtime | M | none | — |
| `RT-EFFECT-DIFF` | One reusable rich differential boundary over EffectObservation — interpreter vs native, first-divergence reporting, so backend-local tests can observe what only the CLI suites currently can | ready | runtime | L | none | — |
| `RT-ENTRY-TRAP-254` | public_source_observes_raw_argv_environment_cwd_bytes_in_field_order exits 1 with an explicit entry trap where it expects 254 — branch-introduced, and the only tip failure that is not the byte-span gap | closed | runtime | M | none | — |
| `RT-ENTRY-TRAP-PX7O` | px7o heterogeneous eliminator frames: native traps at the explicit entry (RuntimeTrap(4), exit 1) where the interpreter returns exit 7 -- the entry-trap family the de Bruijn repair did NOT clear | closed | runtime | TBD | none | — |
| `RT-ESCAPE` | escaping a second Resource through a bracket fails native lowering | merged | runtime | M | none | PR #911 @ 238a5c5d (origin/main 4ac9141e, CI green) |
| `RT-FNSPLIT-B1R` | RT-NATIVE-FNSPLIT Boundary B1R — encode the occurrence-local semantic material B1 counted but never stored (repair of landed B1) | merged | runtime | L | none | 937 |
| `RT-FNSPLIT-B2A-C` | plan↔lowering occurrence correspondence — transport the preallocated StaticOriginId to the site where it is out of scope | merged | runtime | L | none | 940 |
| `RT-FNSPLIT-B2A-S` | defunctionalize retained body selection — static-origin tag plus one closed consumer, replacing cloned-RuntimeExpr identity | merged | runtime | M | none | 944 |
| `RT-FNSPLIT-B2A` | RT-NATIVE-FNSPLIT Boundary B2a — make the semantic plane load-bearing for emission (behaviour-preserving port) | closed | runtime | L | none | — |
| `RT-FNSPLIT-B2B` | RT-NATIVE-FNSPLIT Boundary B2b — full emission census, finite differences, and the explicit growth verdict | closed | runtime | M | none | — |
| `RT-FNSPLIT-B2E` | semantic boundary-value elimination — an opaque boundary inhabitant plus a mechanically closed operation-by-class disposition ledger over every reachable Lowered consumer, inert | closed | runtime | L | none | — |
| `RT-FNSPLIT-B2F` | functionization and authority switch — per-static-origin Cranelift target functions, atomic with switch-over, equivalence evidence, and old-path removal | merged | runtime | L | none | 1192 |
| `RT-FNSPLIT-B2O-CHECK` | the B2O checking layer advertises more than it enforces — structural closure for the item enumerator and reachability for the validator arms | ready | runtime | M | none | — |
| `RT-FNSPLIT-B2O` | static body ownership — a total, validated occurrence → PredeclaredFunction mapping in the semantic plane, inert | merged | runtime | M | none | 963 |
| `RT-FNSPLIT-B2R` | representation and call-ABI contract — a stable executable contract for every value that crosses a generated-function boundary, inert | merged | runtime | L | none | 967 |
| `RT-FNSPLIT-B2V` | executable boundary-value ABI — one closed 64-bit tagged word for ValueWord/ResultWord plus the emitted-code interface to construct, discriminate and project it | merged | runtime | L | none | — |
| `RT-FNSPLIT-C1` | operational carrier + three executable eliminators — a runtime-general carrier at the Lowered/lowering boundary with a real producer -> validator -> eliminator edge, grounded on artifact-static semantic identity | merged | runtime | L | none | https://github.com/swe-toolkit/ken/pull/1156 |
| `RT-FNSPLIT-C2-SYNTH-ID` | closed synthesized-constructor-role identity capability, with the DynamicConstructor producer that consumes it — the identity source compiler-synthesized effect payloads have no occurrence to ask for | merged | runtime | M | none | 1186 |
| `RT-FNSPLIT-C3-ACTIVATION` | the opaque activation owner — one Rust representation authority in ken-runtime that constructs, publishes and tears down per-invocation boundary storage, with the deployment-supplied capacity profile and the one-argument public adapter seam | merged | runtime | L | none | 1181 |
| `RT-FNSPLIT-RECUR-PORT` | emission-port completion — the governed nested-bracket family (recursive ComputationalMatch + trap arms) must select FunctionizedUnits, so RT-SCALE-B can measure the completed population | merged | runtime | XL | none | — |
| `RT-FNUNIT-RESULT-TOKEN` | Broad starter shapes fail the result-token table on the FunctionizedUnits lane — pre-existing, unmasked by retiring SeedClosureCall | merged | runtime | M | none | https://github.com/swe-toolkit/ken/pull/1892 |
| `RT-FRAME-MARKER-ONCE` | Checked Runtime frame marker is consumed more than once under a nested computational eliminator | draft | runtime | TBD | none | — |
| `RT-GROUNDVALUE-RECURSIVE-DROP` | `RuntimeGroundValue` is a recursive type, so a decoder that is carefully iterative still cannot honour \"deep valid data uses no recursive host stack\" end to end -- a deeply nested value overflows the stack in its own `drop`, reproducible without the decoder, and the depth at which that happens is UNMEASURED: the two numbers in the source report are an observed abort and a deliberately-safe control, not a bisected threshold | draft | runtime | unknown | none | — |
| `RT-JOIN-DISPOSITION` | Join-disposition phase repair — the landed RECUR-PORT `consumed XOR statically-unselected` invariant conflates structural materialization with semantic reachability and false-rejects a join materialized before its enclosing match selects | merged | runtime | M | none | — |
| `RT-JOIN-ORIGIN-ATTRIBUTION` | A planner-required join origin is neither traversal-consumed nor structurally dispositioned, and the set difference does not say which of three authorities is wrong | merged | runtime | S | none | — |
| `RT-LEXICAL-R3-FUSION-EMITTER` | Row 5's before-hole expression is the one member of the eight-expression lexical-recursor population whose lawful repair requires static-continuation fusion -- it is carved out of RT-LEXICAL-RECURSOR-CONSUMERS together with its repair and discriminating-control obligations, because leaving the expression in the parent while moving the machinery would give the parent an AC it cannot discharge | merged | runtime | M | none | — |
| `RT-LEXICAL-RECURSOR-CONSUMERS` | Repair the LexicalCallArgumentRecursor consumer population on the functionized lane, activated by B-only exclusion before the retirement removes the seam | active | runtime | M | none | — |
| `RT-LEXICAL-ROW2-MISSING-MINT` | Row 2 of the lexical-recursor population fails post-compile with a missing Mint rather than at a lowering boundary, so it is not repairable by RT-LEXICAL-RECURSOR-CONSUMERS' D2 | merged | runtime | S | none | — |
| `RT-MATCH-FRAME-FP` | match-frame fingerprints must hash a dedicated closure-free header carrier, not a Debug rendering of closure-capable cases | merged | runtime | M | none | https://github.com/swe-toolkit/ken/pull/1108 |
| `RT-MATCH-RECURSOR-CONSUMERS` | Complete the MatchScrutineeRecursor consumer repair in Position A — the D2 increment closed one witness, not the population | merged | runtime | M | none | — |
| `RT-NATIVE-FNSPLIT` | Native backend: bound per-function lowering growth to O(n) — helper identity is a variable-width whole-configuration key (orig. single-Function VReg::MAX, since fixed) | merged | runtime | TBD | none | — |
| `RT-NESTED-IH-NATIVE-REALIZATION` | Native realization of the nested-IH recursive computation beyond scalar admission -- emitted definition, ABI/owner wiring, and execution that survives the Cranelift verifier and agrees with the interpreter at Nat 3 | active | runtime | L | none | — |
| `RT-PARITY` | interpreter/native parity erratum (adversary F5 + F6) | closed | runtime | M | none | — |
| `RT-PLANNER-ATTRIB-K` | Boundary A planner: fixed K is a design invariant — move the K-exceeded rejection off the capacity channel | merged | runtime | XS | none | https://github.com/swe-toolkit/ken/pull/935 |
| `RT-PLANNER-DIAGNOSTIC-K` | Boundary A planner: report planner-invariant failures as planner defects, and assert fixed_k CONSTANT rather than merely affine | merged | runtime | S | none | https://github.com/swe-toolkit/ken/pull/929 |
| `RT-PROCESS-EXIT-STATUS` | ProcessExitStatus refusal in the escape lane (rt_escape r2_cross_buffer_freeze_fails_closed_with_invalid_bounds) | draft | runtime | TBD | none | — |
| `RT-PRODUCER-MATCH-PORT` | Producer-match call port — an ordinary Match whose scrutinee is directly a Call routes the whole object to RecursiveDescent | merged | runtime | M | none | — |
| `RT-RECURSOR-TRANSPORT` | Retire the two live recursor residual classes — MatchScrutineeRecursor and LexicalCallArgumentRecursor — off the RecursiveDescent lane | draft | runtime | M | none | — |
| `RT-SCALE-A` | Boundary A — re-derive the planner census for n=3..7 against the COMPLETED factored representation, superseding the provisional outer-planner numbers | merged | runtime | M | none | — |
| `RT-SCALE-B` | Boundary B — the full n=3..7 emission measurement, the research-grounded analytical model, and the operator scaling verdict that gates RT-NATIVE-FNSPLIT's merge | merged | runtime | L | none | — |
| `RT-SCRATCH-LIFETIME-REMAINING-CRATES` | `RT-TEST-SCRATCH-RAII` fixed the scratch-directory leak in the two directories its census declared, and the defect is not confined to them -- unguarded `temp_dir()` sites remain in `ken-interp`, `ken-host` and `ken-verify`, including one that reproduces the original node's defect statement verbatim and generates the second half of a prefix `scripts/ken-cargo`'s reaper already names | ready | runtime | M | none | — |
| `RT-SEED-CALL-PORT` | Seed-closure call port — a Call whose callee is the retained non-lexical closure form routes the whole object to RecursiveDescent | merged | runtime | M | none | — |
| `RT-SITEOP-CARRIED-WITNESS` | Site-bound operand reader cannot witness a carried value — a synthesized SiteOperand demands a compile-time Lowered template from the same seat byte-span activation wants carried | draft | runtime | L | none | — |
| `RT-SPECIALIZED-ACTIVE-RESUME` | A live specialized value with an Active frame is refused by a constructor-only destructure — Active resume does not require constructor shape | merged | runtime | S | none | — |
| `RT-SPECIALIZED-MATCH-ATTRIBUTION` | A Match scrutinee arriving as a Specialized operand falls to the remainder arm, and neither the stage nor the seat says which Lowered class | merged | runtime | S | none | — |
| `RT-SPLIT` | decompose cranelift_backend.rs | merged | runtime | L | none | — |
| `RT-SRC-DISPATCH-COVER` | close the source-machine scrutinee-dispatch coverage tier surfaced by RT-SPLIT slice 4 | draft | runtime | TBD | none | — |
| `RT-SRCBODY-BIND-ORDER` | Functionized source-body units install the parameter run in ABI order where the body reads de Bruijn-nearest-first, so every multi-parameter source body binds its parameters permuted | merged | runtime | M | none | — |
| `RT-SYMLINK-LANE` | SymlinkPolicy is honoured by the interpreter lane and unreachable in the native lane — FollowWithinScope has no native behaviour | draft | runtime | TBD | none | — |
| `RT-TERMINAL-ALL-ELIM-AUTHORITY` | Issue the typed terminal-All structured-IH elimination authority upstream in checked erasure/planning, and let only that issued relation license the source-machine Match seat to consume a ComputationalRecursorClosure | draft | runtime | M | none | — |
| `RT-TEST-SCRATCH-RAII` | Runtime and CLI test fixtures mint a nanosecond-suffixed scratch directory per run and never remove it -- `temp_output_dir` returns a bare `PathBuf`, `tempfile` is not a dependency, and the resulting ~1200 leaked directories per hour under load have filled `/workspaces/ken` to 100% seven times, where the failure presents as a broad regression in the linker-invoking suites rather than as a disk condition | merged | runtime | M | none | — |
| `RT-UNIT-CLOSURE-CONVERT` | Activate function-unit closure conversion for predeclared units — a retained nested body's free de Bruijn references become declared typed capture slots, reconstructed at unit entry from exact caller operands | closed | runtime | TBD | none | — |
| `RT-VALUE-TOTALITY` | Make every total traversal of Value non-recursive in the host stack, and remove the closure capabilities the landed closure boundary forbids | merged | runtime | L | none | — |
| `RT-WORKER-BIND` | compiler-only static-worker binding and transport substrate — lowering cannot bind a worker's carried capture operands into a selected semantic body, and continuation specialization cannot emit a target without it | merged | runtime | L | none | — |
| `RT-WORKER-FIXTURE-DECODE` | AC-5's target-redirect detector is dark — its expression dies at the run step with Backend NativeResultDecode token 9, before any of its three comparisons, while the fixture helper's other caller passes | ready | runtime | M | none | — |
| `SEAL-2` | carrier producer closure, over a derived enumeration | merged | foundation | M | none | PR #912 @ 4ac9141e (origin/main, CI green) |
| `SEC1-IFC-R3` | [Sec1-reduce] cannot be reified yet: NO production path can return Verdict::Disproved, so the verdict D5 requires is unreachable and every Disproved in sec1_acceptance is hand-rigged | draft | verify | M | G-Sec | — |
| `SEC1-IFC` | Reify the three named Sec1 stubs — two of them are the SOLE NETS for Sec1's two trusted surfaces, and both are placeholders under a green suite | merged | verify | M | G-Sec | https://github.com/swe-toolkit/ken/pull/1094 |
| `SEC1-R3-MINIMAL-ROUTE` | SEC1-IFC-R3 was escalated as needing an SMT backend, and that named the one component the program has already deferred by policy while leaving the binding one unidentified -- prover.rs:317 lists FOUR deferred components and nobody has measured which is minimally sufficient for AC-R3c; also re-derive the recorded claim that widening decidable equality is vacuous, which is true of one registry and false of a second that already holds Char | merged | verify | S | none | 2124 |
| `SEC4-TCB` | Sec4's trust-model conformance seed is fully authored and nothing executes it — Sec1/Sec1ct/Sec2 each have an acceptance suite bound to their seed, Sec4 has none | merged | verify | M | G5 | — |
| `SPAN-SEAL` | seal the BufferSpan producer surface | merged | foundation | M | none | — |
| `SPEC-31-WIDTH-ERRATUM` | spec 31-lexical mandates a 96-column canonical width while the formatting conformance suite asserts 88 in 18 places and cites 31 §1d as its source — rule the exact value and reconcile | closed | spec | S | none | https://github.com/swe-toolkit/ken/pull/1054 |
| `SPEC-38-ERRATUM` | spec 38-ffi-io self-contradicts on the transfer bound — rule and reconcile | closed | spec | S | none | 827 |
| `SPEC-ALIGN-A1` | Scope the landed-code authority convention out of the normative status blocks, and census every private-mechanism constraint against its conformance consumers before relaxing any of them | merged | spec | M | none | 1028 |
| `SPEC-ALIGN-B1` | Split the frozen interoperability and provenance schemas into versioned protocol profiles, under a per-edge threat audit rather than a field count | draft | spec | L | none | — |
| `SPEC-AUTH-EX` | 62-authority §7 worked examples are written in a retired surface — retired `view` keyword, retired `Cap_FS` spelling, and `write_at` for the landed `write_file` | draft | spec-enclave | S | none | — |
| `SPEC-CLOSURE-BOUNDARY` | Revise the runtime value spec to remove the closure-identity inconsistency and state the closure/value boundary with minimum constraints on the implementation | merged | spec | M | none | — |
| `SPEC-ERRATUM-39-2-3-CITATION` | Erratum: 34-data-match.md:625 still cites `39 §2.3` for higher-order pattern abstraction, a coordinate the structural-result merge reassigned to Structural-result association | merged | spec-enclave | S | none | — |
| `SPEC-IDENT-BLESSED` | Settle the identifier character set: 31-lexical promises a bounded blessed-Unicode-letter table that does not exist, cites a security chapter that carries no such claim, and states a confusable gate the landed lexer does not implement | merged | spec-enclave | M | none | https://github.com/swe-toolkit/ken/pull/1147 |
| `SPEC-MISSION-GROUNDING` | Ground the spec as a whole against the mission — audit every retained constraint for which mission property fails without it, and relax the ones where nothing does | draft | spec | L | none | — |
| `SPEC-NESTED-IND` | un-defer nested strictly-positive inductives in 14 §8.5 — state structural positivity through declared strictly-positive type-parameter positions, the lifted induction hypotheses, and the iota rules, WITHOUT mutual families | merged | spec-enclave | M | none | — |
| `SPEC-SELECTOR-SORT-SPLIT` | split the recursive-result selector by motive sort -- `recursive result for x` when Type-classified, `induction hypothesis for x` when Omega-classified -- and remove `structural result of x` | merged | spec | M | none | — |
| `SPEC-STATUS-RECONCILE` | the spec's two status vocabularies do not correspond — define the correspondence (or replace the ladder), then apply it | merged | spec-enclave | M | none | — |
| `SPEC-STORE-SPLIT` | Split durable canonical bytes from in-process maximal sharing: demote the store mechanism to private, retarget the conformance rows that assert it, and re-cut the runtime program against the relaxed contract | merged | spec-enclave | L | none | — |
| `SRC-ATTEST` | squash-stable whole-source attestation + fresh merge-result authorization | merged | doc | M | none | — |
| `STR-BIJ-TEST-CARRIER` | The AC2 reverse-direction test claims a universal inverse and its sole operand is an NFC fixed point — it is green under the correct law AND under the false one it pins | merged | language | S | none | https://github.com/swe-toolkit/ken/pull/1102 |
| `STR-BIJ` | the String/List Char 'bijection' over-claim (adversary A1 + A2) | merged | spec-enclave | S | none | https://github.com/swe-toolkit/ken/pull/1096 |
| `STR-NFC-CONSTRUCTION` | NFC-at-construction is normative and unimplemented: all three `EvalVal::Str` ingresses store the raw string, so `char_length`/`byte_length`/`s2l`/`==` observe unnormalized values and the interp carrier disagrees with the runtime carrier | merged | language | L | none | https://github.com/swe-toolkit/ken/pull/1109 |
| `SURF-IDENT-TR39` | The lexer's confusable-resistance is satisfied VACUOUSLY by an ASCII-only identifier rule — spec 31 §2's blessed Unicode letters are unimplemented, and the test that looks like the TR39 gate cannot see the difference | merged | ergo | S–M | none | — |
| `SURF-SPACE-CELLS` | The `space` block surface — cells and `becomes` — is unbuilt, while its entire desugaring target (the `State` effect: Get/Put/run_state) is built and live | merged | language | M–L | none | https://github.com/swe-toolkit/ken/pull/1152 |
| `SURF-gadt-coverage-diagnostics` | Tracker node for the landed dependent-constructor coverage/diagnostics slice (`34 §4.3`/`§8`): points at `docs/program/wp/SURF-gadt-coverage-diagnostics.md`, and carries the tracker-gap node's two D3 findings -- the AC5 reject half EXISTS and is cited, and the exhaustiveness diagnostic names the missing CONSTRUCTOR, not `§4.1`'s most-general PATTERN witness, for any constructor with arguments | merged | language | S | none | — |
| `SURF-gadt-elaboration` | Tracker node for the landed dependent-constructor elaboration slice (`34 §8`): points at `docs/program/wp/SURF-gadt-elaboration.md`, filed retroactively for the same tracker-generation reason as its parser/AST predecessor | merged | language | M | none | — |
| `SURF-gadt-field-sugar` | Tracker node for the landed dependent-constructor field-sugar slice (`34 §8`): points at `docs/program/wp/SURF-gadt-field-sugar.md`; the frame's own D0 audit bounded the slice to declaration-only labels, which is why AC3's unknown/missing/extra-field checks have no corresponding test -- there is no named-argument constructor expression or pattern syntax to check them against | merged | language | S | none | — |
| `SURF-gadt-parser-ast` | Tracker node for the landed dependent-constructor parser/AST slice (`34 §8`): points at `docs/program/wp/SURF-gadt-parser-ast.md`, filed retroactively because `gen-progress.sh` generates the tracker from `docs/program/issues/` and this slice landed with a frame but no issue node | merged | language | S | none | — |
| `TEST-NATIVE-STACK-PROVISIONING-STANDARD` | Record the stated-stack standard where a candidate author will read it -- the governing property is that a test's stack is STATED, not that it is large, and the tree already derives both halves including the RUST_MIN_STACK / stack_size split | merged | doc | S | none | — |
| `TEST-STATED-STACK-SITE-RECONCILE` | Reconcile the 15 stated-stack sites to the ruling -- and the first deliverable is CLASSIFYING each into one of the three acts, because the twelve 256 MiB sites need a measured peak that nobody has ever taken | ready | runtime | M | none | — |
| `V3-KRIPKE-DECOMPOSITION` | The FO Kripke embedding is the DAG's V3 headline and has never had a tracker node -- only V3-RESIDUAL and V4-RESIDUAL exist, both merged, and what they produced is the single Int-literal refutation arm; establish what the embedding requires and how it decomposes into one-hour increments, because an L-sized node cannot be released and the adequacy lemma is kernel-facing rather than prover-facing | merged | verify | M | none | — |
| `V3-RESIDUAL` | V3's suite has FOUR assertion-free placeholder tests carrying ordinary names — `disproved_carries_countermodel` asserts nothing, passes, and reads in cargo output exactly like a real pin | merged | verify | L | G2-G3 | https://github.com/swe-toolkit/ken/pull/1103 |
| `V3-VERDICT-CENSUS` | Every obligation the prover cannot close is registered as a postulate in trusted_base(), so weak proof search is not a convenience gap but a trusted-base gap -- and nobody has measured how large it is; census the verdict distribution over the existing obligation corpus, and for each Unknown record the fragment it routed to and the syntactic shape that defeated the search | merged | verify | S | none | 2120 |
| `V4-RESIDUAL` | The Kripke countermodel is an inert shell: it is never related to `φ` at all — no interpretation of the formula, no recursive forcing evaluator — and V3's prose `description` is stuffed into `FormRef`, a slot meant for a structural subformula reference | merged | verify | L | G2-G3 | 1117 |
| `VIS-BR-LITERAL` | visibility walk: raw-string prefixes br and cr are unrecognized by the literal scanner | merged | runtime | XS | none | — |

## Releasable frontier

Items whose status is `ready` and whose every `depends_on` entry is
itself `merged` or `closed` (i.e. nothing left blocking a kickoff):

- `PROG-TRACKER-MERGE-DRIVER` — Two docs candidates in flight ALWAYS conflict on generated IMPLEMENTATION-PROGRESS.md and nowhere else -- and the recorded reason merge=union was rejected is FALSE at the current generator, so D0 re-derives the warrant before anything is built
- `RT-4B-UNIQUENESS-GATE-REACH` — Count whether any candidate reaches the twelfth of thirteen elimination exits before building anything that classifies what happens there -- a call-site counter at `fusion_unique_static_body_triple`, changing no signature, no control flow and no plan, which decides whether the attribution increment has a subject at all
- `RT-C2-DRIVER-STAGE-ATTRIBUTION` — The D5 observation identity driver reports every non-zero nested exit as `nested {} compilation failed`, so the one message AC-2 itself produces names the wrong stage -- plus one clause recording why the compiled-feature const must stay adjacent to the gate it mirrors
- `RT-CALL-EDGE-EXECUTABILITY-AXIS` — executable_call_edges probes a body-axis set with an entry-axis key, so a template-only callee whose axes differ survives the filter and fails later as a forward-declaration error
- `RT-CANDIDATE-LEDGER-RESIDUALS` — Two named population questions on the merged candidate/disposition ledger were never reached, and the node that could have covered them is closed
- `RT-CARRIER-PRODUCER-OCCURRENCE` — a source aggregate reaches the carrier with no planner-issued producer occurrence, so the C2 edge refuses to emit and the nested-payload selection row never exercises its property
- `RT-CENSUS-CAVEAT-GUARD` — The identifier-census caveat's staleness guard is an existence check standing in for a count check, so it cannot detect the drift it was written to catch
- `RT-CHECKED-IH-REALIZATION-AUTHORITY` — Mint the checked-IH realization authority -- pending marker, oriented plan, call template, slot and parent -- so the ComputationalRecursorClosure capsule is realizable IN PLACE at the source-machine Match seat, without widening the ordinary-Match selector and without any terminal-All licensing
- `RT-CONTSRC-CALLABLE-CONTRACT` — Closed callable-contract arm for continuation sources — a recursive IH is a compiler-only static worker with no value carrier, and the enclosing slot authority is unconditionally a value contract, so its environment sits outside the domain RT-CONTSRC-PRODUCER-LOCAL owns
- `RT-EFFECT-DIFF` — One reusable rich differential boundary over EffectObservation — interpreter vs native, first-divergence reporting, so backend-local tests can observe what only the CLI suites currently can
- `RT-FNSPLIT-B2O-CHECK` — the B2O checking layer advertises more than it enforces — structural closure for the item enumerator and reachability for the validator arms
- `RT-SCRATCH-LIFETIME-REMAINING-CRATES` — `RT-TEST-SCRATCH-RAII` fixed the scratch-directory leak in the two directories its census declared, and the defect is not confined to them -- unguarded `temp_dir()` sites remain in `ken-interp`, `ken-host` and `ken-verify`, including one that reproduces the original node's defect statement verbatim and generates the second half of a prefix `scripts/ken-cargo`'s reaper already names
- `RT-WORKER-FIXTURE-DECODE` — AC-5's target-redirect detector is dark — its expression dies at the run step with Backend NativeResultDecode token 9, before any of its three comparisons, while the fixture helper's other caller passes
- `TEST-STATED-STACK-SITE-RECONCILE` — Reconcile the 15 stated-stack sites to the ruling -- and the first deliverable is CLASSIFYING each into one of the three acts, because the twelve 256 MiB sites need a measured peak that nobody has ever taken

## Blockers

Items not yet `merged`/`closed` whose `depends_on` names an id that
is itself not yet `merged`/`closed`:

- `ABI-A1` blocked by `ABI-REVOKE` (status: draft)
- `ABI-A2` blocked by `ABI-REVOKE` (status: draft)
- `ABI-A3` blocked by `ABI-REVOKE` (status: draft)
- `ABI-A3` blocked by `ABI-R3` (status: draft)
- `ABI-M1` blocked by `ABI-R3` (status: draft)
- `ABI-M2` blocked by `ABI-M1` (status: draft)
- `ABI-R3` blocked by `PX8` (status: draft)
- `ABI-REVOKE` blocked by `ABI-R3` (status: draft)
- `ABI-S1` blocked by `PX9` (status: draft)
- `ABI-S2` blocked by `ABI-A3` (status: draft)
- `ABI-S4` blocked by `ABI-M1` (status: draft)
- `ABI-S5` blocked by `PX9` (status: draft)
- `ABI-S6` blocked by `ABI-S1` (status: draft)
- `DS-9` blocked by `KERNEL-NESTED-IND` (status: active)
- `F4` blocked by `A3` (status: draft)
- `KERNEL-NESTED-IND` blocked by `RT-NESTED-IH-NATIVE-REALIZATION` (status: active)
- `NATIVE-HANDLE-CARRIER` blocked by `RT-BACKEND-PRIMITIVE-LOWERING-SPLIT` (status: draft)
- `PX10` blocked by `PX9` (status: draft)
- `PX10` blocked by `ABI-M1` (status: draft)
- `PX10` blocked by `ABI-S5` (status: draft)
- `PX11` blocked by `PX9` (status: draft)
- `PX11` blocked by `ABI-M1` (status: draft)
- `PX12` blocked by `PX10` (status: draft)
- `PX12` blocked by `PX11` (status: draft)
- `PX8-F-CAP-41` blocked by `NATIVE-HANDLE-CARRIER` (status: draft)
- `PX8` blocked by `PX8-F-CAP-41` (status: draft)
- `PX9` blocked by `PX8` (status: draft)
- `PX9` blocked by `ABI-REVOKE` (status: draft)
- `RT-4B-UNIQUENESS-GATE-ATTRIBUTION` blocked by `RT-4B-UNIQUENESS-GATE-REACH` (status: ready)
- `RT-BACKEND-MODULE-SPLIT` blocked by `RT-DESCENT-RETIRE` (status: draft)
- `RT-BACKEND-PRIMITIVE-LOWERING-SPLIT` blocked by `RT-BACKEND-SPLIT-CENSUS` (status: draft)
- `RT-BACKEND-SPLIT-CENSUS` blocked by `RT-DESCENT-RETIRE` (status: draft)
- `RT-DESCENT-RETIRE` blocked by `RT-RECURSOR-TRANSPORT` (status: draft)
- `RT-RECURSOR-TRANSPORT` blocked by `RT-LEXICAL-RECURSOR-CONSUMERS` (status: active)
- `RT-TERMINAL-ALL-ELIM-AUTHORITY` blocked by `KERNEL-NESTED-IND` (status: active)

## Gate progress

Work items grouped by the gate (`05-implementation-dag.md`) they
feed; `none`/`TBD` gates are omitted here (see the status table above
for every item, gated or not):

- **G-Sec**: `SEC1-IFC-R3` (draft) `SEC1-IFC` (merged)
- **G2-G3**: `V3-RESIDUAL` (merged) `V4-RESIDUAL` (merged)
- **G5**: `SEC4-TCB` (merged)
- **operator**: `LANG-DECEQ-CHAR-LAWFUL-INSTANCES` (draft) `LANG-FOREIGN-NAME-FORMAT-CHARS` (draft)

## Archive & diary

- The complete build chronicle — every prior live-state snapshot, the full
  evidence trail behind every merged WP back to project start — and the
  day-to-day session narrative both live in [`diary/`](diary/INDEX.md), one
  file per day under `diary/YYYY/Mon/DD.md`. See
  [`diary/CURRENT-BRIEFING.md`](diary/CURRENT-BRIEFING.md) for the live
  operator briefing and Steward resume state.
- Per-item briefs, where they exist, live under
  [`wp/`](wp/) and are linked from the corresponding
  `docs/program/issues/<ID>.md` file.
