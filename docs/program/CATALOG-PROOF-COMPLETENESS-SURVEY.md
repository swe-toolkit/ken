# Catalog proof-completeness survey

## Measurement

The revised input is `origin/main` at
`aa1f3267d46103563d8b1575d39ff82fc163da90`, which includes PRINCIPLES
#17. Its catalog package tree is identical to the original survey-release
input `9db4e95a77bbee41e39a6c0e1c48bff6c12bf1e6`. The census walked every
`*.ken.md` file recursively under these six roots:

```text
catalog/packages/{Algorithm,Application,Capability,Core,Data,Tooling}
```

The sorted walk contains 49 packages. The tables below contain 49 distinct
package paths: 18 fully-proven, 18 tested-only-with-deferred-proofs, 4
TCB-contract-axiom, and 9 no-proof-obligation.

The unit of classification is a package. A mixed package is tested-only if
any requirement provable over Ken structure lacks a general proof. A
proposition such as `ParserLaws` is a specification, not evidence that an
implementation inhabits it. In contrast, PRINCIPLES #17 says that an `Axiom`
for a tested property of a kernel-`Neutral`, `reg_prim`, kernel certificate,
or FFI export is its proper TCB contract, not an incomplete Ken proof. Those
packages have a distinct state and no `*-LAWS` recommendation unless they
also have a separate structural gap. A computational test is external
evidence only; it never promotes a structurally provable row to fully-proven.
Private proof terms count when their types state the general requirement and
the kernel checks them against the package's actual representation.
Machine-checked complexity is outside this survey, as required by the frame.

## Fully-proven packages

Every requirement named in this table is discharged by a general checked term.
The cited tests are not needed for the classification.

| Package | Semantic requirement | Intrinsic evidence |
|---|---|---|
| `Algorithm/Numeric/Gcd.ken.md` | The computed result divides both inputs and every common divisor divides it. | `GcdSpec`, `gcd_spec`, `gcd_divides_left`, `gcd_divides_right`, and `divides_gcd` in the package form proof-relevant witnesses for arbitrary `Nat` inputs and divisors. |
| `Algorithm/Searching/OrderedSearch.ken.md` | On a sorted list under the supplied lawful order, search decides membership and carries either a membership proof or a refutation. | `search` returns `Dec (Equal Bool (elem ... ) True)` for arbitrary input and uses `elem_step_both_true`, `elem_step_from_tail_after_head`, `elem_step_to_tail_after_head`, and `elem_step_to_tail_before_head`. |
| `Algorithm/Sorting/InsertionSort.ken.md` | Insert and sort preserve sortedness and element multiplicity under the same lawful order. | `insert::sorted`, `sort::sorted`, `insert::count`, `insert::permutation`, and `sort::permutation` are general proofs over the production lists and comparator dictionary. |
| `Capability/Filesystem/Path/Posix.ken.md` | Parsing and rendering preserve valid paths; parsing produces valid paths; normalization is idempotent and removes forbidden dot forms. | `path_parse_render_valid`, `path_parse_valid`, `path_parse_render_parse`, `path_normalize_idempotent`, `path_normalize_has_no_dot`, and `path_normalize_absolute_has_no_dotdot` close the requirements over the production `Path` representation. |
| `Capability/Process/Environment.ken.md` | Replacing and then projecting the raw byte environment returns the replacement without altering its representation. | `process_environment::round_trip` proves the projection/replacement law for every `ProcessInput` and environment list. |
| `Capability/Process/WorkingDirectory.ken.md` | Replacing and then projecting the raw working-directory bytes returns the replacement. | `process_working_directory::round_trip` proves the law for every `ProcessInput` and `Bytes` value. |
| `Core/Classes/EffectfulClasses.ken.md` | The shipped `Option` and `List` Applicative, Monad, and Traversable instances satisfy their class laws. | The exact families are `option_ap_id`, `option_ap_hom`, `option_ap_ich`, `option_ap_cmp`, `option_map_coh`, `option_bind_lid`, `option_bind_rid`, `option_bind::assoc`; their `list_*` counterparts; and `list_traverse_identity_law`, `list_traverse_naturality`, `list_traverse_composition`, `option_traverse_identity_law`, `option_traverse_naturality`, and `option_traverse_composition`. |
| `Core/Classes/LawfulFunctors.ken.md` | The shipped `List` and `Option` functors and foldables obey identity, fusion, traversal, and fold-map coherence; the shipped monoids use checked laws. | `list_map::id`, `list_map::fusion`, `option_map::id`, `option_map::fusion`, `list_foldr_to_list`, `list_fold_map_coherence`, `option_foldr_to_list`, and `option_fold_map_coherence`, with the checked `list_append::{left_unit,assoc,right_unit}` and Boolean monoid proofs used by the instances. |
| `Core/Logic/EmptyDec.ken.md` | Decidability retains proof or refutation evidence and its Boolean observation agrees with the chosen branch. | `dec_eq_decides` constructs the proof-relevant decision; `decide::yes_is_true` and `decide::no_is_false` prove both observations. |
| `Core/Logic/OrdResult.ken.md` | The three comparison results admit complete one- and two-result elimination. | `ord_result_elim` and `ord_result_elim2` quantify over every result and all three or nine branches respectively. |
| `Core/Logic/Transport.ken.md` | Substitution, congruence, cast, symmetry, and transitivity transport equality at their stated families. | `subst`, `cong`, `cast`, `sym`, and `trans` are themselves general `J`-built proof terms; `stuck_of::transport` and `sym_trans_compose` check the nontrivial composition path. |
| `Data/Collections/Deque.ken.md` | Front/back pushes agree with the list view, and popping immediately after the corresponding push returns the inserted value and prior deque. | `toList_pushFront`, `toList_pushBack`, `popFront_pushFront`, and `popBack_pushBack` prove the requirements over the production two-list deque; `PopPreserves` carries the inverse result. |
| `Data/Collections/NonEmpty.ken.md` | Construction structurally guarantees a head, and append is associative. | `NonEmpty` has no empty constructor and `nonempty_append::assoc` proves the semigroup law for arbitrary values. |
| `Data/Collections/PriorityQueue.ken.md` | Merge, insert, and pop preserve multiplicity and validity; minima are global; complete drains conserve entries and are nondecreasing. | `merge_count`, `insert_count`, `pop_min_count`, `merge_valid`, `insert_valid`, `pop_min_valid`, `find_min_global`, `pop_min_global`, `observe_count`, `drain_count`, `drain_valid`, and `drain_nondecreasing` are general private proofs over the actual queue. Complexity remains the separately excluded deferral. |
| `Data/Numeric/Nat/Arithmetic.ken.md` | Natural addition and multiplication satisfy their identities, successor equations, associativity, commutativity, and distributivity. | `add::{zero_r,zero_l,suc_r,suc_l,assoc,comm}`, `mul::{zero_r,zero_l,suc_r,suc_l,one_r,one_l,comm,assoc}`, `mul_add_distrib_r`, and `mul_add_distrib_l` are structural proofs. |
| `Data/Sums/Combinators.ken.md` | Every `Option`, `Result`, and `Either` combinator obeys its constructor equations, and `swap` is involutive. | The exact attached proofs are `get_or_else::{none,some}`, `is_some::{none,some}`, `or_else::{none,some,none_rhs}`, `map_err::{ok,err}`, `and_then::{ok,err}`, `unwrap_or::{ok,err}`, `either::{left,right}`, `map_left::{left,right}`, `map_right::{left,right}`, and `swap::involutive`. |
| `Data/Sums/Validation.ken.md` | Validation accumulates independent errors and its Functor and Applicative instances satisfy all class laws. | `both_errors_accumulate`, `validation_map::id`, `validation_map::fusion`, `validation_ap_id`, `validation_ap_hom`, `validation_ap_ich`, `validation_ap_cmp`, and `validation_map_coh` are general or exact proof terms over `Validation`. |
| `Data/Vector/Vector.ken.md` | Length and bounds are preserved by construction, and the public eliminations compute on their constructor cases. | The indices of `Vec` and `Fin` enforce length/bounds; `head_vcons`, `tail_vcons`, `map_vnil`, `zip_with_vnil`, and `lookup_fzero` prove the advertised computation equations. |

## Tested-only packages and proof recommendations

These 18 packages have a genuine Ken proof gap: the missing requirement ranges
over reducible Ken structure and can be proved generally by elimination or
induction. Each row names the load-bearing behavior, the exact external test
evidence, and the missing proof. The final column is the one-line input for a
follow-on node.
Every recommendation retains the production representation and requires zero
new trust: no `Omega` path carrier, postulate, primitive, `Axiom`, TCB change,
or kernel change.

| Package | Tested-only semantic requirement and exact test | Proof status | Same-representation, no-trust follow-on recommendation |
|---|---|---|---|
| `Application/CommandLine/ArgParse.ken.md` | Flag, raw-value, positional, diagnostic accumulation/location, invalid-byte preservation, and schema-derived help behavior are exercised by `crates/ken-elaborator/tests/cc7_argparse_acceptance.rs::{forge_parses_flags_raw_values_and_positionals_and_renders_derived_help,two_independent_bad_arguments_accumulate_exact_nonzero_locations,invalid_utf8_option_value_survives_byte_identically,adding_one_option_to_the_spec_changes_help_without_a_second_help_edit}`. | The package contains no theorem or attached proof; every behavioral guarantee is external. This is an absent proof suite, not a named deferral. | `CAT-ARGPARSE-LAWS`: prove result characterization, accumulation order, exact provenance, byte preservation, and help derivation over the existing parser/schema values with no new trust. |
| `Application/Configuration/Decoder.ken.md` | Required-field accumulation, raw-byte preservation, and environment/config provenance are exercised by `crates/ken-elaborator/tests/cc8_env_config_decoder_acceptance.rs::{invalid_utf8_environment_value_survives_the_full_pipeline,two_missing_fields_accumulate_exact_environment_origins,config_failures_keep_config_key_origins_distinct_from_environment}`. | No proof term states lookup correctness, accumulation completeness, or provenance faithfulness. The proof suite is absent. | `CAT-CONFIGURATION-DECODER-LAWS`: prove lookup, required-field accumulation, provenance, and raw-`Bytes` preservation over the existing decoder representation with no new trust. |
| `Application/Input/Schema.ken.md` | The traversal visits every field, accumulates every rejection, and derives help from the same schema; `crates/ken-elaborator/tests/cc8_env_config_decoder_acceptance.rs::{schema_help_growth_reaches_both_clients_behaviorally,two_missing_fields_accumulate_exact_environment_origins}` supplies fixture evidence. | `schema_validate_fields`, `schema_validate`, and `schema_help` have no general laws. The proof suite is absent. | `CAT-SCHEMA-LAWS`: prove traversal coverage/order, rejection accumulation, and help coherence over the existing schema and validation values with no new trust. |
| `Capability/Console/Text.ken.md` | The four helpers route UTF-8 bytes to the selected stream, append exactly one newline for line forms, and retain `Result`; `crates/ken-interp/tests/i2_console_floor.rs::package_helpers_route_exact_bytes_and_one_newline` executes the behavior. | The package has no equality or effect-tree proof for any helper. The proof suite is absent. | `CAT-CONSOLE-TEXT-LAWS`: prove each existing helper's exact `write` tree, stream, bytes, newline, and result preservation with no new trust. |
| `Capability/Formatting/Doc.ken.md` | Group/alternative fitting and the `String` boundary are fixture-checked by `crates/ken-elaborator/tests/cc5_pretty_doc_acceptance.rs::{group_and_alt_flip_at_the_exact_fitting_boundary,pretty_doc_loader_surface_and_string_boundary_are_behavioral}`. | `render_content::{preserves_text_tokens,width_independent}` and `render::fixed_point` are real proofs, but no general layout/fitting or `render_string` boundary theorem exists; the source calls the latter proof-free. | `CAT-FORMATTING-DOC-LAWS`: prove fitting/layout and `render_string` coherence for the existing `Doc`, renderer, and structural content projection with no new trust. |
| `Capability/Parsing/Cursor.ken.md` | Successful peek, strict remaining decrease, valid end, and exact argument locations are tested by `crates/ken-elaborator/tests/cc3_parsing_cursor_decoder_acceptance.rs::repetition_progress_and_arg_locations_are_discriminating`. | `CursorPeekHasRemaining`, `CursorAdvanceProgress`, `CursorEndValid`, and `CursorLaws` only define obligations; there is no inhabitant of `CursorLaws ... arg_cursor_ops`. This is an absent proof. | `CAT-PARSING-CURSOR-LAWS`: prove the three `CursorLaws` components for the existing `arg_cursor_ops` and its exact location representation with no new trust. |
| `Capability/Parsing/Decoder.ken.md` | Repetition consumes input, terminates at end, and reports locations; `crates/ken-elaborator/tests/cc3_parsing_cursor_decoder_acceptance.rs::repetition_progress_and_arg_locations_are_discriminating` runs discriminating fixtures. | `DecoderManyConsumesAllLaw` is only a proposition-valued definition; no term proves it, and the combinators have no general semantic equations. The proof suite is absent. | `CAT-PARSING-DECODER-LAWS`: prove the many-consumes-all implication and pure/fail/bind/seq/alt/recursive equations over the existing decoder and derived-fuel implementation with no new trust. |
| `Capability/Parsing/Numeric.ken.md` | Located decimal success/failure and structural digit formatting are exercised by `crates/ken-elaborator/tests/cc2_text_codec_numeric_acceptance.rs::{located_numeric_discriminators_and_codec_boundary_are_checked,verified_roundtrip_stays_structural_and_key_instances_stay_out_of_numeric}`. | `decimal_digit_to_char::valid` and `format_digits_roundtrip` cover digit lists only. `parse_nat_chars` and `parse_int_chars` still lack structural success/failure characterizations. Total `show_int : Int -> String` is a separate primitive-capability deferral, not a Ken proof gap. | `CAT-PARSING-NUMERIC-LAWS`: prove structural parser success/failure and consumption laws over the current character/digit lists; treat any future `show_int` behavior as a separate TCB contract rather than asking Ken induction to prove a primitive. |
| `Capability/Parsing/Parsing.ken.md` | Valid parse spans/source identity and Boolean parser/printer round trip are checked by `crates/ken-elaborator/tests/cat5_parsing_package.rs::{cat5_d1_valid_half_open_bounds_and_zero_width_offsets_check,cat5_d2_success_parser_carries_valid_consumed_span_from_start,cat5_d3_bool_parser_printer_formatter_roundtrip_on_source_bytes}`. | `LessEqNat::{refl,zero_left}` and `valid_zero_width_span` are proofs, but `ParserLaws` is only a predicate and no term proves it for `parse_bool_expr`; no general printer round trip exists. | `CAT-PARSING-LAWS`: prove `ParserLaws` for the existing Boolean parser and its printer/formatter round trip over the same syntax, spans, source, and decoder representation with no new trust. |
| `Capability/Process/Arguments.ken.md` | Byte-preserving projection is proven, but bounded slice-location behavior is only exercised by `crates/ken-elaborator/tests/cc6a_process_arguments_exit_acceptance.rs::structural_slice_location_keeps_nonzero_argument_and_range`. | `process_arguments::round_trip` is intrinsic; `argument_slice_location` has no general existence/rejection and projection theorem. The missing part is absent, not deferred. | `CAT-PROCESS-ARGUMENTS-LAWS`: prove the exact iff between an existing argument plus ordered in-bounds endpoints and the current `ArgLocation` result, including all projected fields, with no new trust. |
| `Capability/System/IO.ken.md` | The actual transparent `writeAll` recursion's short-write continuation, zero-progress rejection, first-error preservation, and exact prefix are run by `crates/ken-verify/tests/px8f_write_partition.rs::checked_write_all_reaches_full_short_zero_progress_flip_and_error_prefixes`. | The five `write_all_*` proofs concern helper observations; none states the result/prefix theorem over the actual `writeAll` effect tree. Exactly-once settlement and host liveness are TCB contracts and are not part of this structural gap. | `CAT-SYSTEM-IO-LAWS`: prove the transparent `writeAll` effect tree's result, prefix, zero-progress, and first-error laws over its `Nat` budget and structural continuations; explicitly exclude host progress and settlement from the Ken proof. |
| `Capability/System/Resource.ken.md` | The transparent bracket's acquire-before-body sequence, settlement on returned outcomes, and body/release error ordering are exercised by `crates/ken-interp/tests/px7f_system_resource.rs::{public_bracket_success_and_early_release_settle_once,caller_control_release_failure_preserves_success_and_body_error_ordering}`. | No proof states the structural sequencing or `ResourceBracketResult` ordering of `withResource`/`release_if_live`. Generation liveness, rights enforcement, stale-copy closure, and controlled traps are TCB contracts and are not Ken proof targets. | `CAT-SYSTEM-RESOURCE-LAWS`: prove source-expressible bracket sequencing, settlement continuations, and returned-result ordering over the existing effect tree and result carrier; exclude runtime generation, rights, liveness, and trap behavior. |
| `Core/Logic/Compare.ken.md` | Three-way pair/list comparison and lexicographic first-difference behavior are executed by `crates/ken-elaborator/tests/compare_ord_lexicographic_acceptance.rs::{raw_compare_discriminates_all_results_and_strict_negatives,pair_and_list_instances_compute_lexicographically}`. | `pair_compare::eq` and `pair_compare::eq_cases` prove only pair equality branches. `list_compare` has no general lexicographic, equality, or order-coherence proof. | `CAT-COMPARE-LAWS`: prove equality soundness/completeness and first-difference lexicographic coherence for the existing pair/list comparison functions and lawful dictionaries with no new trust. |
| `Data/Collections/Derived.ken.md` | Generic list operations, shorter-list zip, range contents/length, generic sort behavior, and String/Bytes views are computationally exercised; exact examples include `crates/ken-elaborator/tests/ds4_list_combinators_acceptance.rs::{ac8_off_by_one_range_length_rejected,ac8_zip_length_is_min_not_left_length,zip_truncates_at_shorter_list_concrete_example}` and `crates/ken-elaborator/tests/cat3_collections_package.rs::slice_width_is_end_minus_start_through_production_slice`. | Many structural and Boolean-sort proofs exist, but filter membership is explicitly held out, generic sort has only a concrete `Bool` proof family, and list/range/zip operations lack general structural laws. Primitive String/Bytes behavior is a separate TCB-contract face. | `CAT-DERIVED-COLLECTIONS-LAWS`: prove filter membership and the missing structural list, range, zip, and generic-sort laws over the current functions; state wrapper-view equations only relative to existing TCB contracts, with no replacement implementation or new trust. |
| `Data/Collections/Map.ken.md` | The five core map laws and later update/set laws are intrinsic, but positive reachability, cycles, domain bounds, and recurrence are fixture-checked by `crates/ken-elaborator/tests/map_build_acceptance.rs::{cat_rel_reachable_plus_has_exact_public_predicate,cat_rel_reachability_obeys_positive_fuel_and_domain_bound,cat_rel_seed_positive_two_edge_cycle_executes,cat_rel_seed_non_root_successor_path_executes}`. | The package states that relation closure faithfulness and saturation proofs remain deferred. The already named target is `CAT-REL-CLOSURE-LAWS`; this is not an absent unnamed gap. | Existing `CAT-REL-CLOSURE-LAWS`: prove positive transitive-closure soundness, completeness/saturation, and domain/fuel correspondence for the existing tree relation and comparator assumptions with no new trust. |
| `Data/Numeric/Nat/Order.ken.md` | The canonical order and concrete `min`/`max`/`sub`/`compare` behavior are exercised by `crates/ken-elaborator/tests/ds2_ord_nat_acceptance.rs::{entry_elaborates_with_every_checked_fence,totality_source_and_public_relation_behavior_survive_the_move}` and production `sub` is exercised by `crates/ken-elaborator/tests/cat3_collections_package.rs::slice_width_is_end_minus_start_through_production_slice`. | Provider order laws and three local zero laws are intrinsic. The source explicitly names `sub n n = Zero` as unproved; broader min/max/sub/compare algebra is absent. | `CAT-NAT-ORDER-LAWS`: prove the complete min/max/sub/compare theory, including self-subtraction, over the current structural definitions and canonical `leq_nat` with no new trust. |
| `Data/Serialization/Json.ken.md` | Mixed recursive array/object size folding and recursive decoder branches are exercised by `crates/ken-elaborator/tests/ds9_json_codec_acceptance.rs::{json_size_consumes_array_and_pair_nested_object_results,decoder_recursive_reaches_array_and_object_many_branches}`. | `char_cursor_*` laws are intrinsic, but `json_size` and the recursive decoder still lack general constructor equations. Total `show_int` is a separate TCB-capability deferral rather than a structural proof obligation. | `CAT-JSON-LAWS`: prove the `json_size` and decoder constructor equations over the same six-constructor `Json`; use a separately stated number-render TCB contract if a future codec round-trip proof needs it. |
| `Tooling/Testing/Property.ken.md` | First-counterexample order and the finite cursor success/mutant failure are executed by `crates/ken-elaborator/tests/cat_property_acceptance.rs::property_finite_sample_witnesses_retain_behavior`. | `cursor_progress_witness` and `cursor_stuck_counterexample_witness` are executable `Bool` constants, explicitly not proof terms; `check` has no soundness/completeness theorem for its sample list. | `CAT-PROPERTY-LAWS`: prove finite-list check soundness, success completeness, and first-counterexample characterization over the existing `Gen` list and `Result` representation with no new trust. |

## TCB-contract-axiom packages

These four packages have no outstanding requirement provable by reducing or
inducting over Ken structure. Their remaining properties characterize opaque
TCB exports. Under PRINCIPLES #17, a correctly quantified contract axiom or
certificate is the right source-level evidence. Tests audit the trusted
implementation; they do not stand in for a possible Ken proof. Axiom placement
is a separate Architect question and creates no `*-LAWS` backfill here.
`TCB-contract-axiom` identifies obligation ownership; it does not claim that
all desirable boundary contracts have already been issued at Ken level.

| Package | TCB export and contract evidence | Classification boundary |
|---|---|---|
| `Capability/System/Buffer.ken.md` | `crates/ken-elaborator/src/prelude.rs` issues `Resource Buffer` plus private allocation/freeze protocol nodes, and `crates/ken-interp/src/eval.rs` owns the generation table that enforces one-current-window, fixed-capacity, and post-settlement invalidation. `crates/ken-interp/tests/px8p_checked_buffer.rs::{positive_capacity_settles_normally_and_body_error_still_settles,escaped_copy_reaches_exact_closed_after_bracket_settlement}` audits them. | `transfer_is_positive` and `transfer_is_bounded` already prove the reducible `TransferCount` facts. Handle generation and settlement state are intentionally opaque to Ken, so their remaining laws are TCB contracts and receive no proof-backfill node. |
| `Core/Classes/LawfulClasses.ken.md` | `leq_int` is kernel-`Neutral`; `Ord Int` states its `refl`, `antisym`, `trans`, and `total` contracts with four `Axiom`s. `eq_int` uses `EqCert` and `DecEqCert` registered in `crates/ken-elaborator/src/numbers.rs`; Char transports those contracts. `crates/ken-elaborator/tests/es4_classes_acceptance.rs::{int_ord_instance_is_audited_delta_not_zero_delta,char_ord_laws_carried_not_stubbed_transport_accepts}` audits the bridge. | These are exactly properties of primitive comparison/certificates, not missing structural proofs. The axioms are proper per #17, so this row receives no proof-backfill node. |
| `Data/Text/Codec.ken.md` | `crates/ken-elaborator/src/bytes.rs` registers opaque `bytes_encode`, `bytes_decode`, and `BytesRoundTripLaw`; `bytes_at` and `uint8_to_int` are also opaque boundary operations. The law is the encode/decode contract consumed by `codec_roundtrip_anchor`. `crates/ken-elaborator/tests/cc2_text_codec_numeric_acceptance.rs::{located_numeric_discriminators_and_codec_boundary_are_checked,verified_roundtrip_stays_structural_and_key_instances_stay_out_of_numeric}` audits fixtures. | `decode_utf8::definition`, `ascii_view_none`, and `ascii_view_some` cover the reducible local equations. General byte-codec behavior belongs to the TCB/FFI contract and receives no proof-backfill node. |
| `Data/Text/StringBijection.ken.md` | `crates/ken-elaborator/src/prelude.rs` registers primitive `string_to_list_char` and `list_char_to_string` as kernel-`Neutral`; `string_to_list_char_retraction` is their explicit contract axiom, and `string_to_list_char_injective` follows from it. `crates/ken-elaborator/tests/cc2_text_codec_numeric_acceptance.rs::bijection_prerequisite_is_the_single_separately_homed_assumption` audits its uniqueness. | No Ken constructor structure exists for the conversion pair to reduce against. The retraction axiom is proper per #17, so this row receives no proof-backfill node. |

## No-proof-obligation packages

These packages do not claim an implementation-independent semantic law of
their own. Their checked definitions, types, or visibility are the interface;
there is no external fixture being used as a substitute for a stated theorem.

| Package | Grounded reason |
|---|---|
| `Capability/Diagnostics/Core.ken.md` | This is vocabulary: data carriers, structural projections, and definitions of `ValidByteRange`, `ValidConfigKeyPath`, `ValidOrigin`, and `ValidDiagnostic`. It does not claim that an arbitrary inhabitant is valid or supply a diagnostic-producing algorithm whose correctness needs proving. |
| `Capability/Diagnostics/Render.ken.md` | `diagnostic_to_doc` is a transparent presentation policy whose exhaustive match/projection body is the contract; the package states no width, injectivity, round-trip, or other correctness law independent of that definition. |
| `Capability/Filesystem/Authority.ken.md` | The two procedures only consume the prelude capability operations. The package adds no capability constructor or management semantics; authority-index separation is in the types and the host-only complement is explicitly outside catalog Ken. |
| `Capability/Filesystem/Errors.ken.md` | `renderIOError` and `renderFileError` are transparent exhaustive naming policy. The surrounding authorization/confinement prose describes the existing host boundary, not a new package algorithm or theorem obligation. |
| `Capability/Process/Exit.ken.md` | The four declarations are transparent aliases, exhaustive `Result` elimination, or direct policy application. Totality follows from ordinary Ken checking; no defaulting, round-trip, or algebraic law is claimed. |
| `Capability/Time/WallClock.ken.md` | The package only projects or replaces the structural `Instant` payload and explicitly refuses an ordering or monotonicity law for `wall_now`. The monotonic-clock design is a separate future capability, not a deferred proof of this package. |
| `Core/Logic/Or.ken.md` | The package declares only the proof-relevant sum carrier `Or` and its `Inl`/`Inr` constructors. It contains no operation or independent law. |
| `Data/Binary/BytesKeys.ken.md` | This is an import/export compatibility package. It declares no local dictionary, equality operation, law, or trust; the actual Bytes laws belong to `Core.Classes.LawfulClasses`. |
| `Data/Text/StringKeys.ken.md` | This is likewise a compatibility consumer/re-export of the canonical String equality and order identities. It declares only checked examples and no local instance or law; the provider's TCB contracts are classified once in the class owner's row. |

## Follow-on summary

There are 18 genuine proof-backfill packages. The Map row already has the
correctly named `CAT-REL-CLOSURE-LAWS` follow-on. The other 17 need nodes framed
from the recommendations above:

- `CAT-ARGPARSE-LAWS`
- `CAT-CONFIGURATION-DECODER-LAWS`
- `CAT-SCHEMA-LAWS`
- `CAT-CONSOLE-TEXT-LAWS`
- `CAT-FORMATTING-DOC-LAWS`
- `CAT-PARSING-CURSOR-LAWS`
- `CAT-PARSING-DECODER-LAWS`
- `CAT-PARSING-NUMERIC-LAWS`
- `CAT-PARSING-LAWS`
- `CAT-PROCESS-ARGUMENTS-LAWS`
- `CAT-SYSTEM-IO-LAWS`
- `CAT-SYSTEM-RESOURCE-LAWS`
- `CAT-COMPARE-LAWS`
- `CAT-DERIVED-COLLECTIONS-LAWS`
- `CAT-NAT-ORDER-LAWS`
- `CAT-JSON-LAWS`
- `CAT-PROPERTY-LAWS`

`CAT-PRIORITY-QUEUE-LAWS` is not in that list: its general semantic proofs are
present in the current provider, so the current PriorityQueue row is
fully-proven. Its separately deferred machine-checked complexity theorem does
not change this survey's semantic classification.

## Candidate-cut remeasurement

The final candidate is the report-only commit named in the exact-SHA handoff;
a commit cannot embed its own hash. Immediately after that commit is created,
the same recursive walk is rerun. Acceptance requires 49 packages again, an
empty package-tree diff from the revised input SHA, and unchanged totals of
18 / 18 / 4 / 9. The handoff records those measured results beside the
candidate SHA.
