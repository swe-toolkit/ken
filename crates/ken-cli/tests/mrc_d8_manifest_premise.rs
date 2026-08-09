//! `RT-MATCH-RECURSOR-CONSUMERS` `D8`, frame section 4a.2 — the pin on the
//! premise that keeps the census transport out of the shipped `ken` binary.
//!
//! `with_child_match_recursor_census` is an unconditional `pub` item called from
//! production `ken-cli/src/main.rs`. The entire safety argument for that is that
//! **the feature never reaches the shipped binary**, and that argument rests on
//! exactly three facts about three manifests:
//!
//! 1. the workspace root declares `resolver = "2"`;
//! 2. `ken-runtime` does not make `px8-ds-test-support` a default feature;
//! 3. `ken-cli` takes `ken-runtime` **unfeatured** in `[dependencies]`, with the
//!    featured edge only in `[dev-dependencies]`.
//!
//! **Fact 1 is one line and nothing else in the repository checks it.** A
//! virtual workspace manifest with no `resolver` key defaults to **resolver 1**
//! even when every member is edition 2021 — the workspace does not inherit it
//! from members. Under resolver 1 dev-dependency features unify into normal
//! builds, so `cargo build --release --bin ken` would compile the transport into
//! the shipped binary, where three environment variables any caller can set turn
//! a production `ken` into a file writer against a caller-chosen sink.
//!
//! This pin lives in `ken-cli` because `ken-cli` is the crate whose production
//! binary is at risk.
//!
//! # This is not a rule about how to spell a manifest
//!
//! If a future change makes the transport unreachable from production by
//! construction, `D8` is discharged by deleting this file and saying so.
//!
//! # It reads three exact paths, so no prose can fire it
//!
//! Every check below is a read of a named `Cargo.toml`. Nothing here greps the
//! repository, so Markdown that merely *discusses* `resolver` or
//! `px8-ds-test-support` — including the frame section that specifies this pin,
//! and this comment — cannot satisfy or trip any assertion.

use std::path::{Path, PathBuf};

// ---- the modelled subset of TOML ----------------------------------------
//
// Deliberately small: enough for three facts and no more. What is modelled is
// table headers, `key = value` entries, values that continue across lines while
// `[]`/`{}` are unbalanced, inline-table keys, and string arrays.
//
// # What is NOT modelled, and which way each one fails
//
// A hand-written reader is clean about exactly the shapes its author
// remembered, so the residual is stated by DIRECTION rather than by adjective.
// The direction that matters is a fact reported HELD when it is not; a fact
// reported LOST when it holds is a loud, attributable red.
//
// | unmodelled shape | present in the three manifests | direction if it appeared |
// |---|---|---|
// | dotted keys (`workspace.resolver = "2"`) | no | **safe** — no table context, so fact 1 reads NOT DECLARED and reds. Controlled below |
// | arrays of tables (`[[bin]]`) | yes, in `ken-cli` | **safe** — treated as an unrelated table, so it cannot supply a key to a wanted one. Controlled below |
// | multi-line basic strings (`"""…"""`) | no | **UNCONTROLLED.** A `[table]`-shaped or `key = value`-shaped line inside one would be read as structure |
// | literal strings (`'…'`) containing brackets | no | **UNCONTROLLED.** Bracket depth is counted for double-quoted strings only, so one could unbalance a continuation |
//
// The last two are a real residual, not a covered case: if a manifest ever uses
// them, this reader needs a fresh look rather than a patch. They are called out
// here because the earlier revision of this comment claimed the controls kept
// every unmodelled shape "from being an assumption", which was false — the
// controls cover comments, neighbouring tables, multi-line arrays, and the two
// shapes marked controlled above, and nothing else.

/// Strip a `#` comment, respecting double-quoted strings so a `#` inside a
/// value is not treated as a comment introducer.
fn strip_comment(line: &str) -> &str {
    let bytes = line.as_bytes();
    let mut in_string = false;
    for (index, byte) in bytes.iter().enumerate() {
        match byte {
            b'"' => in_string = !in_string,
            b'#' if !in_string => return &line[..index],
            _ => {}
        }
    }
    line
}

/// Depth change contributed by one line's brackets and braces, ignoring those
/// inside double-quoted strings.
fn depth_delta(text: &str) -> i32 {
    let mut in_string = false;
    let mut delta = 0;
    for byte in text.bytes() {
        match byte {
            b'"' => in_string = !in_string,
            b'[' | b'{' if !in_string => delta += 1,
            b']' | b'}' if !in_string => delta -= 1,
            _ => {}
        }
    }
    delta
}

/// Every `key = value` entry of one table, with values joined across
/// continuation lines. Returns `None` when the table is absent, which is a
/// distinct answer from an empty table.
fn table_entries(manifest: &str, table: &str) -> Option<Vec<(String, String)>> {
    let header = format!("[{table}]");
    let mut in_table = false;
    let mut seen = false;
    let mut entries: Vec<(String, String)> = Vec::new();
    let mut pending: Option<(String, String, i32)> = None;

    for raw in manifest.lines() {
        let line = strip_comment(raw).trim_end();
        let trimmed = line.trim();

        if let Some((key, value, depth)) = pending.take() {
            let depth = depth + depth_delta(line);
            let value = format!("{value} {trimmed}");
            if depth > 0 {
                pending = Some((key, value, depth));
            } else {
                entries.push((key, value.trim().to_string()));
            }
            continue;
        }

        if trimmed.starts_with('[') && trimmed.ends_with(']') && !trimmed.contains('=') {
            in_table = trimmed == header;
            seen |= in_table;
            continue;
        }
        if !in_table {
            continue;
        }
        let Some((key, value)) = trimmed.split_once('=') else {
            continue;
        };
        let key = key.trim().to_string();
        let depth = depth_delta(value);
        if depth > 0 {
            pending = Some((key, value.trim().to_string(), depth));
        } else {
            entries.push((key, value.trim().to_string()));
        }
    }

    if let Some((key, value, _)) = pending {
        entries.push((key, value.trim().to_string()));
    }
    seen.then_some(entries)
}

fn entry<'a>(entries: &'a [(String, String)], key: &str) -> Option<&'a str> {
    entries
        .iter()
        .find(|(found, _)| found == key)
        .map(|(_, value)| value.as_str())
}

/// The top-level keys of an inline table `{ a = 1, b = 2 }`.
fn inline_keys(value: &str) -> Vec<String> {
    let Some(body) = value.trim().strip_prefix('{').and_then(|v| v.strip_suffix('}')) else {
        return Vec::new();
    };
    let mut keys = Vec::new();
    let mut depth = 0;
    let mut in_string = false;
    let mut field = String::new();
    for character in body.chars() {
        match character {
            '"' => {
                in_string = !in_string;
                field.push(character);
            }
            '[' | '{' if !in_string => {
                depth += 1;
                field.push(character);
            }
            ']' | '}' if !in_string => {
                depth -= 1;
                field.push(character);
            }
            ',' if !in_string && depth == 0 => {
                if let Some((key, _)) = field.split_once('=') {
                    keys.push(key.trim().to_string());
                }
                field.clear();
            }
            _ => field.push(character),
        }
    }
    if let Some((key, _)) = field.split_once('=') {
        keys.push(key.trim().to_string());
    }
    keys
}

/// The string items of an array value `["a", "b"]`.
fn list_items(value: &str) -> Vec<String> {
    let Some(body) = value.trim().strip_prefix('[').and_then(|v| v.strip_suffix(']')) else {
        return Vec::new();
    };
    body.split(',')
        .map(|item| item.trim().trim_matches('"').to_string())
        .filter(|item| !item.is_empty())
        .collect()
}

// ---- the three facts, as predicates over manifest TEXT -------------------
//
// Each takes text rather than a path, so the pin below can apply it to the real
// manifest and the controls can apply the SAME code to synthetic text. A
// control that exercised a different predicate would be testing itself.

/// FACT 1 — the workspace root declares `resolver = "2"`.
fn workspace_declares_resolver_two(manifest: &str) -> bool {
    table_entries(manifest, "workspace")
        .as_deref()
        .and_then(|entries| entry(entries, "resolver"))
        .is_some_and(|value| value.trim().trim_matches('"') == "2")
}

/// FACT 2 — `px8-ds-test-support` is not reachable from `default`.
///
/// Transitive, not just direct: `default = ["x"]` with `x = ["px8-ds-test-
/// support"]` makes it a default feature just as surely as naming it outright,
/// and a check that only looked at `default`'s own items would miss that.
fn feature_is_not_default(manifest: &str, feature: &str) -> bool {
    let Some(entries) = table_entries(manifest, "features") else {
        // No `[features]` table at all means nothing can be defaulted on.
        return true;
    };
    let mut frontier = list_items(entry(&entries, "default").unwrap_or("[]"));
    let mut seen: Vec<String> = Vec::new();
    while let Some(name) = frontier.pop() {
        if name == feature {
            return false;
        }
        if seen.contains(&name) {
            continue;
        }
        seen.push(name.clone());
        if let Some(value) = entry(&entries, &name) {
            frontier.extend(list_items(value));
        }
    }
    true
}

/// FACT 3 — the named dependency is unfeatured in `[dependencies]` and carries
/// the feature only on the `[dev-dependencies]` edge.
fn dependency_is_featured_only_for_dev(manifest: &str, dependency: &str, feature: &str) -> bool {
    let normal = table_entries(manifest, "dependencies").unwrap_or_default();
    let Some(normal_edge) = entry(&normal, dependency) else {
        // The production edge must EXIST; a missing one would make this pass
        // for the wrong reason.
        return false;
    };
    if inline_keys(normal_edge).iter().any(|key| key == "features") {
        return false;
    }

    let dev = table_entries(manifest, "dev-dependencies").unwrap_or_default();
    let Some(dev_edge) = entry(&dev, dependency) else {
        return false;
    };
    // Split the inline table's `features = [...]` back out by locating the key
    // and taking its bracketed value.
    let Some(rest) = dev_edge.split_once("features").map(|(_, rest)| rest) else {
        return false;
    };
    let Some(open) = rest.find('[') else {
        return false;
    };
    let Some(close) = rest[open..].find(']') else {
        return false;
    };
    list_items(&rest[open..=open + close])
        .iter()
        .any(|item| item == feature)
}

// ---- the pin -------------------------------------------------------------

fn workspace_root() -> PathBuf {
    // `ken-cli`'s manifest dir is `<root>/crates/ken-cli`.
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("ken-cli sits two levels below the workspace root")
        .to_path_buf()
}

/// Read a manifest, failing closed. A missing or unreadable manifest is a red,
/// never a silent skip: "the file was not there" and "the fact holds" are the
/// two readings this must never conflate.
fn read_manifest(path: &Path) -> String {
    std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("manifest {} unreadable: {error}", path.display()))
}

const FEATURE: &str = "px8-ds-test-support";
const DEPENDENCY: &str = "ken-runtime";

/// **The pin.** All three facts, against the real manifests on disk.
#[test]
fn mrc_d8_the_transport_cannot_reach_the_shipped_binary() {
    let root = workspace_root();

    let workspace = read_manifest(&root.join("Cargo.toml"));
    let runtime = read_manifest(&root.join("crates/ken-runtime/Cargo.toml"));
    let cli = read_manifest(&root.join("crates/ken-cli/Cargo.toml"));

    // POSITIVE CONTROL ON THE INSTRUMENT'S INPUTS, before any fact is read.
    //
    // `read_manifest` panics on a missing file, so a wrong path cannot pass
    // silently — but a wrong path that happens to resolve to SOME manifest
    // could, and a fact asserted against the wrong file is green for a reason
    // that has nothing to do with the premise. These identify each file by its
    // own content.
    assert!(
        table_entries(&workspace, "workspace")
            .as_deref()
            .and_then(|entries| entry(entries, "members"))
            .map(list_items)
            .is_some_and(|members| members.iter().any(|m| m == "crates/ken-cli")),
        "the file read as the workspace root is not the workspace root"
    );
    for (label, text, expected) in [
        ("ken-runtime", &runtime, "ken-runtime"),
        ("ken-cli", &cli, "ken-cli"),
    ] {
        let name = table_entries(text, "package")
            .as_deref()
            .and_then(|entries| entry(entries, "name"))
            .map(|value| value.trim().trim_matches('"').to_string());
        assert_eq!(
            name.as_deref(),
            Some(expected),
            "the file read as {label}'s manifest names a different package"
        );
    }

    assert!(
        workspace_declares_resolver_two(&workspace),
        "FACT 1 LOST: the workspace manifest no longer declares `resolver = \"2\"`. A virtual \
         workspace with no resolver key defaults to resolver 1, under which `ken-cli`'s \
         dev-dependency feature unifies into normal builds and `cargo build --release --bin ken` \
         compiles the census transport into the SHIPPED binary"
    );

    let runtime = read_manifest(&root.join("crates/ken-runtime/Cargo.toml"));
    assert!(
        feature_is_not_default(&runtime, FEATURE),
        "FACT 2 LOST: `{FEATURE}` is reachable from `ken-runtime`'s `default` feature, so every \
         consumer gets the transport whatever the resolver does"
    );

    let cli = read_manifest(&root.join("crates/ken-cli/Cargo.toml"));
    assert!(
        dependency_is_featured_only_for_dev(&cli, DEPENDENCY, FEATURE),
        "FACT 3 LOST: `ken-cli` no longer takes `{DEPENDENCY}` unfeatured in `[dependencies]` \
         with the featured edge confined to `[dev-dependencies]`"
    );
}

// ---- the controls --------------------------------------------------------
//
// Three mutations, three reds, PROVED SEPARATELY — a pin that only catches the
// first is a third of a pin, so each fact gets its own test rather than sharing
// one that could pass on any single red.
//
// Each control is a NON-DEGENERATE PAIR on a shared input: the same synthetic
// manifest with and without the mutation. The unmutated side must be ACCEPTED.
// Without that half a rejection proves nothing — a malformed fixture is
// rejected for free, and the mutation would get the credit.

const WORKSPACE_FIXTURE: &str = r#"
[workspace]
resolver = "2"
members = [
    "crates/ken-cli",
]

[workspace.package]
edition = "2021"
"#;

#[test]
fn mrc_d8_control_removing_the_resolver_reds_fact_one() {
    assert!(
        workspace_declares_resolver_two(WORKSPACE_FIXTURE),
        "positive control: the unmutated workspace fixture must be accepted, or the red below is \
         free"
    );

    let mutated = WORKSPACE_FIXTURE.replace("resolver = \"2\"\n", "");
    assert!(
        !workspace_declares_resolver_two(&mutated),
        "MUTATION 1 NOT CAUGHT: removing `resolver = \"2\"` left fact 1 green"
    );

    // And the neighbouring resolver-1 spelling, which is what a downgrade would
    // actually look like.
    let downgraded = WORKSPACE_FIXTURE.replace("resolver = \"2\"", "resolver = \"1\"");
    assert!(
        !workspace_declares_resolver_two(&downgraded),
        "MUTATION 1 NOT CAUGHT: `resolver = \"1\"` was accepted as if it were 2"
    );
}

const RUNTIME_FIXTURE: &str = r#"
[package]
name = "ken-runtime"

[features]
px8-ds-test-support = []

[dependencies]
ken-host = { path = "../ken-host" }
"#;

#[test]
fn mrc_d8_control_defaulting_the_feature_reds_fact_two() {
    assert!(
        feature_is_not_default(RUNTIME_FIXTURE, FEATURE),
        "positive control: the unmutated runtime fixture must be accepted"
    );

    let direct = RUNTIME_FIXTURE.replace(
        "[features]\n",
        "[features]\ndefault = [\"px8-ds-test-support\"]\n",
    );
    assert!(
        !feature_is_not_default(&direct, FEATURE),
        "MUTATION 2 NOT CAUGHT: `default = [\"{FEATURE}\"]` left fact 2 green"
    );

    // The same fact reached one hop away. A check that only read `default`'s own
    // items would call this clean while every consumer got the feature.
    let transitive = RUNTIME_FIXTURE.replace(
        "[features]\n",
        "[features]\ndefault = [\"bundle\"]\nbundle = [\"px8-ds-test-support\"]\n",
    );
    assert!(
        !feature_is_not_default(&transitive, FEATURE),
        "MUTATION 2 NOT CAUGHT: `{FEATURE}` reachable from `default` through another feature left \
         fact 2 green"
    );

    // A `default` list that does NOT reach the feature must stay accepted, or
    // the two reds above would be consistent with rejecting every `default`.
    let unrelated = RUNTIME_FIXTURE.replace("[features]\n", "[features]\ndefault = [\"other\"]\n");
    assert!(
        feature_is_not_default(&unrelated, FEATURE),
        "fact 2 rejected a `default` list that never reaches the feature, so its reds are not \
         about this feature"
    );
}

const CLI_FIXTURE: &str = r#"
[package]
name = "ken-cli"

[dependencies]
ken-kernel = { path = "../ken-kernel" }
ken-runtime = { path = "../ken-runtime" }

[dev-dependencies]
ken-runtime = { path = "../ken-runtime", features = ["px8-ds-test-support"] }
serde_json = "1"
"#;

#[test]
fn mrc_d8_control_moving_the_featured_edge_reds_fact_three() {
    assert!(
        dependency_is_featured_only_for_dev(CLI_FIXTURE, DEPENDENCY, FEATURE),
        "positive control: the unmutated cli fixture must be accepted"
    );

    // The frame's mutation: move the featured edge into `[dependencies]`.
    let moved = CLI_FIXTURE.replace(
        "ken-runtime = { path = \"../ken-runtime\" }",
        "ken-runtime = { path = \"../ken-runtime\", features = [\"px8-ds-test-support\"] }",
    );
    assert!(
        !dependency_is_featured_only_for_dev(&moved, DEPENDENCY, FEATURE),
        "MUTATION 3 NOT CAUGHT: a featured `{DEPENDENCY}` edge in `[dependencies]` left fact 3 \
         green"
    );

    // Losing the dev edge entirely must also red. Otherwise fact 3 would be
    // satisfied by a repository that simply stopped testing the transport, and
    // the pin would be reporting on an absence rather than on the split.
    let no_dev = CLI_FIXTURE.replace(
        "ken-runtime = { path = \"../ken-runtime\", features = [\"px8-ds-test-support\"] }\n",
        "",
    );
    assert!(
        !dependency_is_featured_only_for_dev(&no_dev, DEPENDENCY, FEATURE),
        "MUTATION 3 NOT CAUGHT: removing the dev-dependency edge left fact 3 green"
    );

    // And a dropped production edge must red rather than pass vacuously.
    let no_normal = CLI_FIXTURE.replace("ken-runtime = { path = \"../ken-runtime\" }\n", "");
    assert!(
        !dependency_is_featured_only_for_dev(&no_normal, DEPENDENCY, FEATURE),
        "MUTATION 3 NOT CAUGHT: removing the production edge passed vacuously"
    );
}

// ---- controls on the READER, not on the facts ----------------------------
//
// The three facts above are only as good as the hand-written reader underneath
// them, and a hand-written reader is clean about exactly the shapes its author
// remembered. These are the shapes that would make it report a fact as HELD
// when it is not — the direction that matters.

#[test]
fn mrc_d8_control_the_reader_is_not_satisfied_by_a_comment_or_a_neighbouring_table() {
    // A commented-out resolver must not satisfy fact 1.
    let commented = WORKSPACE_FIXTURE.replace("resolver = \"2\"", "# resolver = \"2\"");
    assert!(
        !workspace_declares_resolver_two(&commented),
        "the reader accepted a COMMENTED-OUT resolver key"
    );

    // A resolver key in a different table must not satisfy fact 1 either.
    let neighbour = WORKSPACE_FIXTURE
        .replace("resolver = \"2\"\n", "")
        .replace("edition = \"2021\"", "edition = \"2021\"\nresolver = \"2\"");
    assert!(
        !workspace_declares_resolver_two(&neighbour),
        "the reader read `resolver` out of `[workspace.package]` as if it were `[workspace]`"
    );

    // A `features` key on a DIFFERENT dependency must not be attributed to
    // `ken-runtime`.
    let other_featured = CLI_FIXTURE.replace(
        "ken-kernel = { path = \"../ken-kernel\" }",
        "ken-kernel = { path = \"../ken-kernel\", features = [\"px8-ds-test-support\"] }",
    );
    assert!(
        dependency_is_featured_only_for_dev(&other_featured, DEPENDENCY, FEATURE),
        "the reader attributed another dependency's `features` key to `{DEPENDENCY}`"
    );

    // An absent table is not an empty one: fact 3 must red when `[dependencies]`
    // is gone entirely.
    let no_table = CLI_FIXTURE.replace("[dependencies]\n", "");
    assert!(
        !dependency_is_featured_only_for_dev(&no_table, DEPENDENCY, FEATURE),
        "the reader treated a missing `[dependencies]` table as satisfying fact 3"
    );
}

#[test]
fn mrc_d8_control_the_reader_survives_a_value_spanning_lines() {
    // The real workspace manifest's `members` array spans lines, and a reader
    // that lost table context inside it would answer about the wrong table.
    let multiline = r#"
[workspace]
members = [
    "crates/ken-cli",
    "crates/ken-runtime",
]
resolver = "2"
"#;
    assert!(
        workspace_declares_resolver_two(multiline),
        "the reader lost a key that follows a multi-line array"
    );

    // Same shape for a feature list written across lines.
    let multiline_features = r#"
[features]
default = [
    "bundle",
]
bundle = [
    "px8-ds-test-support",
]
"#;
    assert!(
        !feature_is_not_default(multiline_features, FEATURE),
        "the reader missed a transitive default written across lines"
    );
}

// ---- controls that mutate the REAL manifests -----------------------------
//
// The synthetic controls above prove the predicates discriminate. They cannot
// prove the predicates discriminate on *the shipped files*, because a fixture
// can drift from the manifest it stands for and nothing would say so. These
// close that: each reads the real manifest, asserts it green, applies the exact
// mutation frame section 4a.2 names, and asserts it reds.
//
// Each is a non-degenerate pair on a SHARED input, and that input is the actual
// file. Each also asserts the mutation CHANGED the text: a mutation that
// silently no-ops because the manifest was respelled would otherwise leave the
// unmutated text in place, and the control would be measuring nothing while
// reading as a result.
//
// Three mutations, three reds, in three separate tests, so a single red names
// which fact lost its control rather than collapsing them into one verdict.

fn real(path: &str) -> String {
    read_manifest(&workspace_root().join(path))
}

/// Apply a mutation and refuse to proceed if it did not bite the real text.
fn mutated(original: &str, applied: String, what: &str) -> String {
    assert_ne!(
        applied, original,
        "the {what} mutation did not change the shipped manifest -- the manifest has been \
         respelled, so this control is measuring nothing. Re-derive the mutation against the \
         current text rather than deleting this assertion"
    );
    applied
}

#[test]
fn mrc_d8_real_control_fact_one_reds_on_the_shipped_workspace_manifest() {
    let text = real("Cargo.toml");
    assert!(
        workspace_declares_resolver_two(&text),
        "the shipped workspace manifest must be green, or the reds below are free"
    );

    let removed = mutated(
        &text,
        text.lines()
            .filter(|line| line.trim() != "resolver = \"2\"")
            .collect::<Vec<_>>()
            .join("\n"),
        "resolver-removal",
    );
    assert!(
        !workspace_declares_resolver_two(&removed),
        "FACT 1 CONTROL LOST: removing `resolver = \"2\"` from the shipped workspace manifest did \
         not red the pin"
    );

    let downgraded = mutated(
        &text,
        text.replace("resolver = \"2\"", "resolver = \"1\""),
        "resolver-downgrade",
    );
    assert!(
        !workspace_declares_resolver_two(&downgraded),
        "FACT 1 CONTROL LOST: `resolver = \"1\"` in the shipped workspace manifest was accepted as \
         if it were 2"
    );
}

#[test]
fn mrc_d8_real_control_fact_two_reds_on_the_shipped_runtime_manifest() {
    let text = real("crates/ken-runtime/Cargo.toml");
    assert!(
        feature_is_not_default(&text, FEATURE),
        "the shipped ken-runtime manifest must be green, or the reds below are free"
    );

    let direct = mutated(
        &text,
        text.replace(
            "[features]\n",
            "[features]\ndefault = [\"px8-ds-test-support\"]\n",
        ),
        "default-feature",
    );
    assert!(
        !feature_is_not_default(&direct, FEATURE),
        "FACT 2 CONTROL LOST: defaulting `{FEATURE}` in the shipped ken-runtime manifest did not \
         red the pin"
    );

    let transitive = mutated(
        &text,
        text.replace(
            "[features]\n",
            "[features]\ndefault = [\"bundle\"]\nbundle = [\"px8-ds-test-support\"]\n",
        ),
        "transitive-default-feature",
    );
    assert!(
        !feature_is_not_default(&transitive, FEATURE),
        "FACT 2 CONTROL LOST: reaching `{FEATURE}` from `default` through another feature did not \
         red the pin on the shipped manifest"
    );
}

#[test]
fn mrc_d8_real_control_fact_three_reds_on_the_shipped_cli_manifest() {
    let text = real("crates/ken-cli/Cargo.toml");
    assert!(
        dependency_is_featured_only_for_dev(&text, DEPENDENCY, FEATURE),
        "the shipped ken-cli manifest must be green, or the reds below are free"
    );

    let moved = mutated(
        &text,
        text.replace(
            "ken-runtime = { path = \"../ken-runtime\" }",
            "ken-runtime = { path = \"../ken-runtime\", features = [\"px8-ds-test-support\"] }",
        ),
        "featured-production-edge",
    );
    assert!(
        !dependency_is_featured_only_for_dev(&moved, DEPENDENCY, FEATURE),
        "FACT 3 CONTROL LOST: a featured `{DEPENDENCY}` edge in the shipped `[dependencies]` did \
         not red the pin"
    );

    let no_dev = mutated(
        &text,
        text.replace(
            "ken-runtime = { path = \"../ken-runtime\", features = [\"px8-ds-test-support\"] }\n",
            "",
        ),
        "dev-edge-removal",
    );
    assert!(
        !dependency_is_featured_only_for_dev(&no_dev, DEPENDENCY, FEATURE),
        "FACT 3 CONTROL LOST: removing the shipped dev-dependency edge did not red the pin"
    );
}

// ---- the two unmodelled shapes that ARE controlled -----------------------

#[test]
fn mrc_d8_control_dotted_keys_and_arrays_of_tables_fail_closed() {
    // A dotted key carries no table context for this reader, so fact 1 must
    // report NOT DECLARED rather than reading it as `[workspace]`'s resolver.
    // Over-strict is the safe direction: a loud red, not a silent hold.
    let dotted = "workspace.resolver = \"2\"\n";
    assert!(
        !workspace_declares_resolver_two(dotted),
        "the reader accepted a DOTTED resolver key, which it does not model -- that is the unsafe \
         direction"
    );

    // An array-of-tables header must not be read as the table of the same name.
    // `ken-cli`'s real manifest contains `[[bin]]`, so this shape is live.
    let array_of_tables = r#"
[[dependencies]]
ken-runtime = { path = "../ken-runtime", features = ["px8-ds-test-support"] }

[dev-dependencies]
ken-runtime = { path = "../ken-runtime", features = ["px8-ds-test-support"] }
"#;
    assert!(
        !dependency_is_featured_only_for_dev(array_of_tables, DEPENDENCY, FEATURE),
        "the reader read `[[dependencies]]` as `[dependencies]`"
    );

    // And the real `ken-cli` manifest's own `[[bin]]` must not disturb the
    // tables the facts are read from -- that is measured by the pin passing, so
    // this asserts the shape is actually PRESENT, or the reassurance is vacuous.
    assert!(
        real("crates/ken-cli/Cargo.toml").contains("[[bin]]"),
        "this control assumes the shipped ken-cli manifest still contains an array-of-tables \
         header; it no longer does, so the shape is no longer exercised"
    );
}
