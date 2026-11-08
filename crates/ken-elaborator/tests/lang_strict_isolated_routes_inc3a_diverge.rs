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
