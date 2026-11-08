//! Increment 3a negative-source check: preserve the intended missing name
//! while imports and checked vocabulary become available under Strict.

use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};

fn missing_name(error: ElabError) -> String {
    match error {
        ElabError::UnresolvedCon { name, .. } | ElabError::UnboundName { name, .. } => name,
        other => panic!("expected the same missing name, got {other:?}"),
    }
}

/// Promise class: transition sentinel, retired with the 3b route switch
/// that removes the Legacy side of the comparison. MEASURED: each static
/// invalid-source fixture reaches its original missing constructor under
/// Legacy and the same missing name under Strict; the merge-sort fixture's
/// previously homed `leqChar` has its own import. CLAIMED: vocabulary changes
/// do not mask or shift these intentional refusals. THE GAP: generated Rust
/// sources and the six FSOp template cases are separate populations, not
/// claimed by this file-source subset.
#[test]
fn baseline_unresolved_sources_keep_the_same_missing_name_in_strict() {
    const CASES: [(&str, &str); 16] = [
        ("conformance/challenge/C1-deceq-noncanonical/sound-deceq-char.ken", "DecEq"),
        ("conformance/challenge/C5-verified-sort/sound-verified-sort.ken", "And"),
        ("conformance/challenge/C5-verified-sort/unsound-const-nil.ken", "And"),
        ("conformance/challenge/C6-lawful-ord-vs-stub/sound-ord-proved.ken", "IsTrue"),
        ("conformance/challenge/C6-lawful-ord-vs-stub/stub-ord-axiom.ken", "Ord"),
        ("crates/ken-elaborator/tests/fixtures/kernel_conv_cast_consumer.ken", "ConvCastExpectedOf"),
        ("crates/ken-elaborator/tests/fixtures/kernel_conv_omega_consumer.ken", "OmegaExpectedOf"),
        ("crates/ken-elaborator/tests/fixtures/kernel_conv_quot_class_consumer.ken", "ClassExpectedOf"),
        ("crates/ken-elaborator/tests/fixtures/kernel_conv_quot_consumer.ken", "QuotEquivOf"),
        ("crates/ken-elaborator/tests/fixtures/kernel_conv_quot_elim_consumer.ken", "ConvBoolQuotient"),
        ("crates/ken-elaborator/tests/fixtures/sibling_goal_refinement/lookup_zip_with.ken.md", "Vec"),
        ("crates/ken-elaborator/tests/fixtures/sibling_goal_refinement/lookup_zip_with_user_local.ken.md", "Vec"),
        ("examples/rosetta/closures/closures.ken", "concat"),
        ("examples/rosetta/merge-sort/merge-sort.ken", "eq"),
        ("examples/rosetta/palindrome/palindrome.ken", "list_append"),
        ("examples/rosetta/tree-traversal/tree-traversal.ken", "list_append"),
    ];
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root");
    let catalog = root.join("catalog/packages");
    for (path, expected) in CASES {
        let original = std::fs::read_to_string(root.join(path))
            .unwrap_or_else(|error| panic!("{path}: cannot read fixture: {error}"));
        let mut strict_source = original.clone();
        if path == "examples/rosetta/merge-sort/merge-sort.ken" {
            strict_source = strict_source.replacen(
                "program capabilities FS APartial\n",
                "program capabilities FS APartial\nimport Core.Classes.LawfulClasses (leqChar)\n",
                1,
            );
        }
        let literate = path.ends_with(".ken.md");
        let mut legacy = ElabEnv::new().expect("checked prelude");
        let legacy = if literate {
            legacy.elaborate_ken_md_file_v1(&original).map(|_| ())
        } else {
            legacy.elaborate_file_v1(&original).map(|_| ())
        };
        let old_name = missing_name(legacy.expect_err("invalid fixture under Legacy"));
        assert_eq!(old_name, expected, "{path}: original error name");
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
        let new_name = missing_name(strict.expect_err("invalid fixture under Strict"));
        assert_eq!(new_name, old_name, "{path}: first missing name changed");
    }
}

/// Promise class: transition sentinel until 3b removes the Legacy route.
/// MEASURED: five already-invalid effect-row fixtures keep TypeMismatch in
/// both modes. CLAIMED: new vocabulary does not mask their actual error.
/// THE GAP: this is a fixed source cohort, not every generated Rust string.
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
        let mut legacy = ElabEnv::new().expect("prelude");
        let old = legacy
            .elaborate_file_v1(&source)
            .expect_err("legacy type mismatch");
        let mut selected = ElabEnv::new().expect("prelude");
        let new = selected
            .elaborate_isolated_file_strict_v1(&[catalog.clone()], &source)
            .expect_err("strict type mismatch");
        assert!(
            matches!(old, ElabError::TypeMismatch { .. }),
            "{name}: wrong Legacy kind: {old:?}"
        );
        assert!(
            matches!(new, ElabError::TypeMismatch { .. }),
            "{name}: wrong Strict kind: {new:?}"
        );
    }
}

/// Promise class: transition sentinel until the planned source-route switch.
/// MEASURED: each of the four R-rewritten raw templates refuses on its same
/// remaining placeholder constructor in Legacy and Strict after importing
/// every checked H name the real fixture writes. CLAIMED: an internal FSOp
/// refusal did not displace the original negative. THE GAP: substituted
/// executable instances are separately pinned by their consumers.
#[test]
fn raw_fsop_templates_keep_the_same_placeholder_refusal_after_r_rewrite() {
    use std::collections::{BTreeMap, BTreeSet};

    use ken_elaborator::lexer::{Lexer, Token};
    use ken_elaborator::modules::PRELUDE_PACKAGE_HOMES;

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root");
    let catalog = root.join("catalog/packages");
    for path in [
        "crates/ken-cli/tests/fixtures/d0_abi_mapping_source.ken",
        "crates/ken-cli/tests/fixtures/d0_px8ta_nested_source.ken",
        "crates/ken-cli/tests/fixtures/d0_rt_parity_source.ken",
        "crates/ken-elaborator/tests/fixtures/rt_bracket_region_multileaf.ken",
    ] {
        let source = std::fs::read_to_string(root.join(path)).expect("R template");
        let mut legacy = ElabEnv::new().expect("prelude");
        let old = legacy
            .elaborate_file_v1(&source)
            .expect_err("unfilled template");
        let original_missing = missing_name(old);
        let tokens = Lexer::lex(&source).expect("lex actual template");
        let names = tokens
            .iter()
            .filter_map(|(token, _)| match token {
                Token::Ident(name) | Token::ConId(name) => Some(name.as_str()),
                _ => None,
            })
            .collect::<BTreeSet<_>>();
        let mut required: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
        for &(owner, member) in &PRELUDE_PACKAGE_HOMES {
            if names.contains(member) {
                required.entry(owner).or_default().insert(member);
            }
        }
        let (header, body) = source.split_once('\n').expect("template header");
        assert!(header.starts_with("program capabilities FS "), "{path}");
        let imports = required
            .into_iter()
            .map(|(owner, members)| {
                format!(
                    "import {owner} ({})\n",
                    members.into_iter().collect::<Vec<_>>().join(", ")
                )
            })
            .collect::<String>();
        let composed = format!("{header}\n{imports}{body}");
        let mut selected = ElabEnv::new().expect("prelude");
        let new = selected
            .elaborate_isolated_file_strict_v1(&[catalog.clone()], &composed)
            .expect_err("unfilled template under Strict");
        assert_eq!(missing_name(new), original_missing, "{path}: error shifted");
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
