"""Exact nextest identities of the two optional formatter whole-corpus rows."""

# This is an allowlist of skips, not a substring or binary-wide exclusion.
# An additional test is scheduled by default, regardless of its name.
FORMATTER_CORPUS_TESTS = frozenset({
    ("ken-elaborator", "r_layer_tests::kenfmt_b3_layout::ac7_whole_catalog_is_parse_preserved_idempotent_and_width_bounded"),
    ("ken-elaborator", "r_layer_tests::kenfmt_b4_splicing::ac6_whole_literate_corpus_preserves_prose_ast_and_idempotence_read_only"),
})
