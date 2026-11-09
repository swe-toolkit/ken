//! Increment 3a negative-source check: preserve the intended missing name
//! while imports and checked vocabulary become available under Strict.

use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};

fn missing_observation(
    result: Result<(), ElabError>,
) -> Result<(String, ken_elaborator::Span, String), String> {
    match result {
        Ok(()) => Err("source accepted".into()),
        Err(error) => {
            let code = error.code().as_str().to_string();
            match error {
                ElabError::UnresolvedCon { name, span } | ElabError::UnboundName { name, span } => {
                    Ok((name, span, code))
                }
                other => Err(format!("unexpected diagnostic {other:?}")),
            }
        }
    }
}

/// Promise class: transition sentinel, retired with the 3b route switch
/// that removes the Legacy side of the comparison. MEASURED: each static
/// invalid-source fixture reaches Legacy E0027 on its own missing name and
/// Strict E0012 on that same name after only its required imports. Report raw
/// spans and compare the normalized Strict span to the Legacy span: an import
/// changes absolute byte positions but not the error's source location. CLAIMED: vocabulary changes
/// do not mask or shift these intentional refusals. THE GAP: generated Rust
/// sources and the six FSOp template cases are separate populations, not
/// claimed by this file-source subset.
#[test]
fn baseline_unresolved_sources_keep_the_same_missing_name_in_strict() {
    const CASES: [(&str, &str); 15] = [
        ("conformance/challenge/C1-deceq-noncanonical/sound-deceq-char.ken", "DecEq"),
        ("conformance/challenge/C5-verified-sort/sound-verified-sort.ken", "isSorted"),
        ("conformance/challenge/C5-verified-sort/unsound-const-nil.ken", "isSorted"),
        ("conformance/challenge/C6-lawful-ord-vs-stub/sound-ord-proved.ken", "Ord"),
        ("conformance/challenge/C6-lawful-ord-vs-stub/stub-ord-axiom.ken", "Ord"),
        ("crates/ken-elaborator/tests/fixtures/kernel_conv_cast_consumer.ken", "ConvCastExpectedOf"),
        ("crates/ken-elaborator/tests/fixtures/kernel_conv_omega_consumer.ken", "OmegaExpectedOf"),
        ("crates/ken-elaborator/tests/fixtures/kernel_conv_quot_class_consumer.ken", "ClassExpectedOf"),
        ("crates/ken-elaborator/tests/fixtures/kernel_conv_quot_consumer.ken", "QuotEquivOf"),
        ("crates/ken-elaborator/tests/fixtures/kernel_conv_quot_elim_consumer.ken", "ConvBoolQuotient"),
        ("crates/ken-elaborator/tests/fixtures/sibling_goal_refinement/lookup_zip_with.ken.md", "Vec"),
        ("crates/ken-elaborator/tests/fixtures/sibling_goal_refinement/lookup_zip_with_user_local.ken.md", "Vec"),
        ("examples/rosetta/closures/closures.ken", "concat"),
        ("examples/rosetta/palindrome/palindrome.ken", "list_append"),
        ("examples/rosetta/tree-traversal/tree-traversal.ken", "list_append"),
    ];
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root");
    let catalog = root.join("catalog/packages");
    let mut mismatches = Vec::new();
    for (path, expected) in CASES {
        let original = std::fs::read_to_string(root.join(path))
            .unwrap_or_else(|error| panic!("{path}: cannot read fixture: {error}"));
        let c5 = path.starts_with("conformance/challenge/C5-verified-sort/");
        let strict_source = if c5 {
            format!("import Core.Logic.Conjunction (And)\n{original}")
        } else if path == "conformance/challenge/C6-lawful-ord-vs-stub/sound-ord-proved.ken" {
            format!("import Core.Classes.LawfulClasses (IsTrue)\n{original}")
        } else {
            original.clone()
        };
        let shift = strict_source.len() - original.len();
        let literate = path.ends_with(".ken.md");
        let mut legacy = ElabEnv::new().expect("checked prelude");
        let legacy = if literate {
            legacy.elaborate_ken_md_file_v1(&original).map(|_| ())
        } else {
            legacy.elaborate_file_v1(&original).map(|_| ())
        };
        let old = match missing_observation(legacy) {
            Ok(observation) => observation,
            Err(detail) => {
                println!("ISOL3A_SAME_NAME {path} legacy={detail}");
                mismatches.push(format!("{path}: {detail}"));
                continue;
            }
        };
        let mut selected = ElabEnv::new().expect("checked prelude");
        let strict = if literate {
            selected
                .elaborate_isolated_ken_md_file_strict_v1(&[catalog.clone()], &strict_source)
                .map(|_| ())
        } else {
            selected
                .elaborate_isolated_file_strict_v1(&[catalog.clone()], &strict_source)
                .map(|_| ())
        };
        let new = match missing_observation(strict) {
            Ok(observation) => observation,
            Err(detail) => {
                println!("ISOL3A_SAME_NAME {path} legacy={old:?} strict={detail}");
                mismatches.push(format!("{path}: {detail}"));
                continue;
            }
        };
        let normalized = (new.1.start.checked_sub(shift), new.1.end.checked_sub(shift));
        println!(
            "ISOL3A_SAME_NAME {path} legacy={}@{}..{} code={} strict={}@{}..{} code={} normalized={normalized:?} import_bytes={shift}",
            old.0, old.1.start, old.1.end, old.2,
            new.0, new.1.start, new.1.end, new.2,
        );
        if old.0 != expected
            || new.0 != old.0
            || normalized != (Some(old.1.start), Some(old.1.end))
            || old.2 != "E0027"
            || new.2 != "E0012"
        {
            mismatches.push(format!(
                "{path}: expected {expected}, observed {old:?} -> {new:?}"
            ));
        }
    }
    assert!(
        mismatches.is_empty(),
        "same-name/position divergence: {mismatches:?}"
    );
}

/// Promise class: transition sentinel, retired when merge-sort's currently
/// unsupported `eq` is given a real checked definition. MEASURED: the
/// rewritten example imports its checked ordering while the first remaining
/// missing name is `eq`. CLAIMED: the route switch does not mask this known
/// negative as a missing ordering home. THE GAP: this negative says nothing
/// about the other (independently checked) examples.
#[test]
fn merge_sort_still_refuses_on_its_unhomed_eq_after_ordering_import() {
    let source_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/rosetta/merge-sort/merge-sort.ken");
    let source = std::fs::read_to_string(&source_path).expect("example source");
    let roots: Vec<_> = ken_elaborator::modules::isolated_catalog_root(&source_path)
        .into_iter()
        .collect();
    let mut env = ElabEnv::new().expect("prelude");
    match env.elaborate_isolated_file_strict_v1(&roots, &source) {
        Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, "eq"),
        other => panic!("merge-sort must preserve its known missing name: {other:?}"),
    }
}

/// Promise class: durable invariant. MEASURED: five invalid effect-row
/// fixtures retain E0038 through the switched Strict entry with their own
/// persisted checked H imports. CLAIMED: no missing-import error masks their
/// effect-row rejection. THE GAP: generated Rust strings are separate inputs.
#[test]
fn local_latent_invalid_sources_keep_their_type_mismatch_kind() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root");
    let catalog = root.join("catalog/packages");
    for name in [
        "fn_calls_console_cb",
        "p6_global_result_arrow",
        "p6b_forward_to_pure_result",
        "p8_subsumption",
        "proc_cb_undeclared",
    ] {
        let path = root
            .join("crates/ken-cli/tests/fixtures/lang_effect_row_local_latent")
            .join(format!("{name}.ken"));
        let source = std::fs::read_to_string(&path).expect("negative source");
        let mut selected = ElabEnv::new().expect("prelude");
        let error = selected
            .elaborate_isolated_file_strict_v1(&[catalog.clone()], &source)
            .expect_err("strict effect escape");
        match error {
            ElabError::TypeMismatch { reason, .. } => {
                assert!(reason.contains("EffectEscapes"), "{name}: {reason}");
            }
            other => panic!("{name}: wrong Strict refusal {other:?}"),
        }
    }
}

/// Promise class: transition sentinel until a valid quotient surface and
/// conformance verdict replace these unconsumed prepared sketches. MEASURED:
/// the exact C7 pair has the known Legacy type error at unsupported `/` and
/// the Strict missing `elim_` name (the source writes `elim_/`). CLAIMED: the two reporting orders differ
/// over the same two invalid constructs, without becoming a conformance pass.
/// THE GAP: this fixed two-row inventory is not a classifier for new sketches.
#[test]
fn known_diverge_c7_pair_keeps_both_specific_refusal_codes() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root");
    let catalog = root.join("catalog/packages");
    for path in [
        "conformance/challenge/C7-quotient-respect/sound-quotient-respect.ken",
        "conformance/challenge/C7-quotient-respect/unsound-nonrespecting.ken",
    ] {
        let source = std::fs::read_to_string(root.join(path)).expect("C7 sketch");
        let mut legacy = ElabEnv::new().expect("prelude");
        let old = legacy
            .elaborate_file_v1(&source)
            .expect_err("unsupported quotient");
        match old {
            ElabError::TypeMismatch { reason, .. } => {
                assert!(
                    reason.contains("'/' not supported on this type"),
                    "{path}: {reason}"
                )
            }
            error => panic!("{path}: expected Legacy TypeMismatch, got {error:?}"),
        }
        let mut selected = ElabEnv::new().expect("prelude");
        match selected.elaborate_isolated_file_strict_v1(&[catalog.clone()], &source) {
            Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, "elim_", "{path}"),
            Err(error) => panic!("{path}: wrong Strict missing-name code: {error:?}"),
            Ok(_) => panic!("{path}: an invalid C7 sketch was admitted"),
        }
    }
}
