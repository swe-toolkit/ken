//! VAL2 Rosetta pangram — the differential runner (Deliverable 1,
//! `docs/program/wp/VAL2-rosetta-pangram.md`).
//!
//! Globs `examples/rosetta/*/`, runs each `<slug>.ken` through the real
//! `ken` CLI binary (subprocess — soundness-inert, drives the compiler as
//! a black box and diffs text), and checks its declared oracle:
//!
//! - `expected` present  -> must-match: stdout must equal the file exactly,
//!   a mismatch fails this test (reds CI).
//! - `KNOWN-GAP.md` present -> recorded non-blocker: the runner does not
//!   assert anything about `ken run`'s outcome, just notes the dir in the
//!   findings summary printed at the end.
//! - Neither present -> hard error (never a silent skip).
//!
//! `ken run` still elaborates one flat source unit rather than loading a
//! catalog roots closure. Examples that reuse
//! `catalog/packages/Data/Collections/Derived.ken.md` (per the frame's DRY
//! rule) therefore need its provider closure concatenated ahead of their own
//! source. This legacy runner extracts the canonical closure, removes every
//! now-redundant import edge from the flattened provider sources with exact
//! cardinality checks, and orders Transport, Or, OrdResult, Compare, the
//! canonical Nat operations and function combinators, then base List and
//! Derived.
//!
//! **This concatenation is NOT applied blanket to every example.**
//! Empirically, unconditionally prepending declarations that a given
//! example doesn't need can catastrophically amplify an unrelated
//! `ken-interp` performance characteristic (VAL2 findings routed to
//! Runtime/Architect, `wp/RTP1-interp-sharing`) — the runner only
//! concatenates for the specific dirs confirmed (via a real `ken run`,
//! not just in-process) to both need and tolerate it. See `NEEDS_COLLECTIONS`
//! below; every other dir runs its own `.ken` file as-is.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

use ken_elaborator::lexer::{Lexer, Token};
use ken_elaborator::{ElabEnv, ElabError};

const PER_EXAMPLE_TIMEOUT: Duration = Duration::from_secs(90);

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn ken_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_ken"))
}

fn rosetta_dir() -> PathBuf {
    workspace_root().join("examples/rosetta")
}

/// Dirs confirmed (in-process AND via a real `ken run`) to need
/// `catalog/packages/Data/Collections/Derived.ken.md` concatenated ahead of their own source, and to
/// stay within the per-example timeout with it prepended. Do not add a
/// slug here without measuring it first (see the module doc).
const NEEDS_COLLECTIONS: &[&str] = &["palindrome", "closures", "merge-sort", "tree-traversal"];

fn catalog_source(path: &str) -> String {
    let markdown = fs::read_to_string(workspace_root().join(path))
        .unwrap_or_else(|error| panic!("{path} must be readable: {error}"));
    ken_elaborator::literate::extract_ken_md(&markdown)
        .unwrap_or_else(|error| panic!("{path} must extract: {error:?}"))
        .source
}

fn remove_flattened_directive(source: &mut String, owner: &str, directive: &str) {
    // Ken's lexer skips comments and keeps string literals as single tokens.
    // Match active syntax, not a textual occurrence in either of those places.
    let expected = Lexer::lex(directive)
        .unwrap_or_else(|error| panic!("invalid {owner} directive `{directive}`: {error:?}"));
    let expected = &expected[..expected.len() - 1]; // omit Eof
    assert!(
        matches!(
            expected.first(),
            Some((Token::KwImport | Token::KwExport, _))
        ),
        "{owner} flattened directive must be an import or export"
    );
    let tokens =
        Lexer::lex(source).unwrap_or_else(|error| panic!("invalid {owner} source: {error:?}"));
    let occurrences = tokens
        .windows(expected.len())
        .filter(|window| {
            window
                .iter()
                .zip(expected)
                .all(|((token, _), (wanted, _))| token == wanted)
        })
        .map(|window| window[0].1.start..window.last().unwrap().1.end)
        .collect::<Vec<_>>();
    let [range] = occurrences.as_slice() else {
        panic!(
            "{owner} must carry exactly one active directive `{directive}`, got {}",
            occurrences.len()
        );
    };
    source.replace_range(range.clone(), "");
}

/// Promise class: durable invariant (CAT-LIST-LENGTH-TRUST-FREE-BASE AC-2).
///
/// MEASURED: Ken's lexer distinguishes active import tokens from comments and
/// strings, and the flattener refuses zero or two active copies. CLAIMED: a
/// behavior-neutral source comment cannot block Rosetta while a real closure
/// edge change fails closed. THE GAP: these synthetic cases isolate the token
/// discriminator; the full Rosetta test exercises the real catalog closure.
#[test]
fn flattened_directives_ignore_comments_and_refuse_real_omissions_or_duplicates() {
    let directive = "import Data.Collections.List (length)";
    let mut source = format!(
        "import Data.Collections.List -- inline comment\n  (length)\n\
         -- {directive}\n\
         const note : String = \"{directive}\"\n"
    );
    remove_flattened_directive(&mut source, "fixture", directive);
    assert!(source.contains(&format!("-- {directive}")));
    assert!(source.contains(&format!("\"{directive}\"")));
    assert!(!source.contains("\n  (length)"));

    for (input, count) in [
        (format!("-- {directive}\n"), 0),
        (format!("{directive}\n{directive}\n"), 2),
    ] {
        let failure = std::panic::catch_unwind(|| {
            let mut input = input;
            remove_flattened_directive(&mut input, "fixture", directive);
        })
        .expect_err("missing or duplicate active import must fail closed");
        let message = failure
            .downcast_ref::<String>()
            .expect("refusal must carry its diagnostic");
        assert!(
            message.contains(&format!(
                "exactly one active directive `{directive}`, got {count}"
            )),
            "unexpected refusal: {message}"
        );
    }
}

/// Promise class: durable invariant (SPEC-TIME-WINDOW-LENGTH-BOUND respin).
///
/// MEASURED: the declaration flattener matches its head as active tokens, so
/// a comment, a string or a longer name spelling the head is not a
/// declaration, and zero or two active heads fail closed. CLAIMED: a
/// behavior-neutral catalog comment cannot block Rosetta. THE GAP: the end is
/// still the head's first closed brace group; the full Rosetta test
/// exercises the real catalog declarations.
#[test]
fn flattened_declarations_ignore_comments_and_refuse_real_omissions_or_duplicates() {
    let head = "pub fn weaken";
    let declaration = "pub fn weaken (n : Nat) : Nat = match n { Zero ↦ Zero ; Suc m ↦ m }";
    let source = format!(
        "-- {head}\n\
         const note : String = \"{head}\"\n\
         pub fn weaken_more (n : Nat) : Nat = match n {{ Zero ↦ Zero ; Suc m ↦ m }}\n\
         {declaration}\n"
    );
    let range = flattened_braced_declaration(&source, "fixture", head);
    assert_eq!(&source[range], declaration);

    for (input, count) in [
        (format!("-- {declaration}\n"), 0),
        (format!("{declaration}\n{declaration}\n"), 2),
    ] {
        let failure =
            std::panic::catch_unwind(|| flattened_braced_declaration(&input, "fixture", head))
                .expect_err("missing or duplicate active declaration must fail closed");
        let message = failure
            .downcast_ref::<String>()
            .expect("refusal must carry its diagnostic");
        assert!(
            message.contains(&format!("exactly one declaration `{head}`, got {count}")),
            "unexpected refusal: {message}"
        );
    }
}

fn flattened_exact_line_declaration(
    source: &str,
    owner: &str,
    declaration: &str,
) -> std::ops::Range<usize> {
    assert_eq!(
        declaration.split('\n').count(),
        1,
        "{owner} exact line declaration must not contain a newline"
    );

    let mut offset = 0usize;
    let mut declaration_range = None;
    for line in source.split_inclusive('\n') {
        let line_without_newline = line.strip_suffix('\n').unwrap_or(line);
        if line_without_newline == declaration {
            assert!(
                declaration_range.is_none(),
                "{owner} must carry exactly one line declaration `{declaration}`"
            );
            declaration_range = Some(offset..offset + declaration.len());
        }
        offset += line.len();
    }

    declaration_range
        .unwrap_or_else(|| panic!("{owner} must carry line declaration `{declaration}`"))
}

fn flattened_braced_declaration(source: &str, owner: &str, start: &str) -> std::ops::Range<usize> {
    // Match the declaration head as active tokens, as the directive flattener
    // does: a comment, a string or a longer name that spells it is not a head.
    let expected = Lexer::lex(start)
        .unwrap_or_else(|error| panic!("invalid {owner} declaration head `{start}`: {error:?}"));
    let expected = &expected[..expected.len() - 1]; // omit Eof
    let tokens =
        Lexer::lex(source).unwrap_or_else(|error| panic!("invalid {owner} source: {error:?}"));
    let heads = tokens
        .windows(expected.len())
        .enumerate()
        .filter(|(_, window)| {
            window
                .iter()
                .zip(expected)
                .all(|((token, _), (wanted, _))| token == wanted)
        })
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    let [head] = heads.as_slice() else {
        panic!(
            "{owner} must carry exactly one declaration `{start}`, got {}",
            heads.len()
        );
    };
    let mut depth = 0usize;
    for (token, span) in &tokens[*head..] {
        match token {
            Token::LBrace => depth += 1,
            Token::RBrace if depth > 0 => {
                depth -= 1;
                if depth == 0 {
                    return tokens[*head].1.start..span.end;
                }
            }
            _ => {}
        }
    }
    panic!("{owner} declaration `{start}` must close");
}

fn collections_prelude() -> String {
    let transport = catalog_source("catalog/packages/Core/Logic/Transport.ken.md");
    let or_source = catalog_source("catalog/packages/Core/Logic/Or.ken.md");
    let ord_result = catalog_source("catalog/packages/Core/Logic/OrdResult.ken.md");
    let mut compare = catalog_source("catalog/packages/Core/Logic/Compare.ken.md");
    let lawful_classes = catalog_source("catalog/packages/Core/Classes/LawfulClasses.ken.md");
    // `IsTrue` is a braceless transparent alias in the canonical provider.
    // Extract its exact complete line instead of letting the braced scanner
    // consume the next unrelated declaration that happens to contain `{`.
    let is_true = flattened_exact_line_declaration(
        &lawful_classes,
        "LawfulClasses",
        "pub fn IsTrue (b : Bool) : Prop = Equal Bool b True",
    );
    let canonical_lawful_ops = std::iter::once(&lawful_classes[is_true])
        .chain(
            [
                "pub fn bool_leq",
                "pub fn bool_and",
                "pub fn bool_or",
                "pub fn leq_nat",
            ]
            .into_iter()
            .map(|start| {
                let declaration =
                    flattened_braced_declaration(&lawful_classes, "LawfulClasses", start);
                &lawful_classes[declaration]
            }),
        )
        .collect::<Vec<_>>()
        .join("\n");
    let nat_order_source = catalog_source("catalog/packages/Data/Numeric/Nat/Order.ken.md");
    // This hand-maintained allowlist selects the consumer's operation inventory.
    // New Order exports are excluded instead of entering flat source without dependencies.
    let nat_order = [
        "pub fn min",
        "pub fn sub",
        "pub theorem leq_nat_weaken_right",
    ]
    .into_iter()
    .map(|start| {
        let declaration = flattened_braced_declaration(&nat_order_source, "Nat.Order", start);
        &nat_order_source[declaration]
    })
    .collect::<Vec<_>>()
    .join("\n");
    let arithmetic_source = catalog_source("catalog/packages/Data/Numeric/Nat/Arithmetic.ken.md");
    let nat_arithmetic = [
        "pub fn add",
        "pub proof zero_l for add",
        "pub proof suc_l for add",
    ]
    .into_iter()
    .map(|start| {
        let declaration = flattened_braced_declaration(&arithmetic_source, "Nat.Arithmetic", start);
        &arithmetic_source[declaration]
    })
    .collect::<Vec<_>>()
    .join("\n");
    let combinators = catalog_source("catalog/packages/Core/Function/Combinators.ken.md");
    let list = catalog_source("catalog/packages/Data/Collections/List.ken.md");
    let mut collections = catalog_source("catalog/packages/Data/Collections/Derived.ken.md");

    // `ken run` consumes one flat source unit here. Remove imports whose
    // providers this runner flattened above, plus Derived's facade-only export.
    // Fail closed if an edge changes shape or cardinality.
    for import in [
        // Flat `ken run` has the convertible Ω conjunction in its prelude.
        "import Core.Logic.Conjunction (And, and_intro)",
        "import Core.Logic.Or (Or, Inl, Inr)",
        "import Core.Logic.OrdResult (OrdResult, Lt, Eq, Gt, ord_eq, ord_lt, ord_gt)",
        "import Core.Logic.Transport (sym)",
    ] {
        remove_flattened_directive(&mut compare, "Compare", import);
    }
    // The legacy flat unit uses its convertible prelude Ω conjunction; it
    // cannot also import the package's same-spelled declarations by module.
    for import in [
        "import Core.Function.Combinators (comp, idf)",
        "import Core.Classes.LawfulClasses (IsTrue, bool_and, bool_or, bool_leq, leq_nat, leqChar)",
        "import Core.Logic.Compare (list_compare, list_eq)",
        "import Core.Logic.Conjunction (And, and_intro, and_fst, and_snd)",
        "import Core.Logic.Or (Or, Inl, Inr)",
        "import Core.Logic.OrdResult\n  (OrdResult,\n    Lt,\n    Eq,\n    Gt,\n    ord_eq,\n    ord_lt,\n    ord_gt,\n    ord_result_leq,\n    ord_result_dispatch2,\n    ord_result_elim,\n    ord_result_elim2)",
        "import Core.Logic.And (And as Conj)",
        "import Core.Logic.Transport (cong, sym, trans)",
        "import Data.Numeric.Nat.Order (leq_nat_weaken_right, min, sub)",
        "import Data.Numeric.Nat.Arithmetic (add)",
        "import Data.Collections.List (length)",
        "export length",
    ] {
        remove_flattened_directive(&mut collections, "Derived", import);
    }
    // `list_all` is the one Derived export whose provider this runner does not
    // flatten: `Core.Logic.And`'s `And` would shadow the prelude Ω `And` that
    // Derived's own proofs use. No example uses it, so it is excluded like the
    // Nat.Order exports outside the allowlist above.
    let list_all = flattened_braced_declaration(&collections, "Derived", "pub fn list_all");
    collections.replace_range(list_all, "");

    format!(
        "{transport}\n{or_source}\n{ord_result}\n{compare}\n{canonical_lawful_ops}\n{nat_order}\n{nat_arithmetic}\n{combinators}\n{list}\n{collections}"
    )
}

enum Oracle {
    Expected(String),
    KnownGap,
}

fn oracle_for(dir: &Path) -> Oracle {
    let expected_path = dir.join("expected");
    let gap_path = dir.join("KNOWN-GAP.md");
    match (expected_path.exists(), gap_path.exists()) {
        (true, false) => {
            Oracle::Expected(fs::read_to_string(&expected_path).expect("expected must be readable"))
        }
        (false, true) => Oracle::KnownGap,
        (true, true) => panic!(
            "{}: has BOTH `expected` and `KNOWN-GAP.md` — a dir must declare exactly one oracle",
            dir.display()
        ),
        (false, false) => panic!(
            "{}: has NEITHER `expected` nor `KNOWN-GAP.md` — every rosetta dir must declare its \
             oracle (never a silent skip)",
            dir.display()
        ),
    }
}

/// Runs `<slug>.ken` (with the package prelude prepended) through the real
/// `ken` binary as a subprocess, with a generous timeout. Returns
/// `Some((stdout, exit_success))` on completion within the timeout, `None`
/// on timeout (killed).
fn rosetta_program_source(slug: &str, ken_src_path: &Path) -> String {
    let own_src = fs::read_to_string(ken_src_path)
        .unwrap_or_else(|e| panic!("{}: cannot read {}: {}", slug, ken_src_path.display(), e));
    if !NEEDS_COLLECTIONS.contains(&slug) {
        return own_src;
    }
    let mut body = own_src
        .strip_prefix("program capabilities FS APartial\n")
        .expect("runnable Rosetta source starts with its program header")
        .to_owned();
    if slug == "merge-sort" {
        // The flat provider needs leqChar before its declarations; the
        // isolated file's later import is redundant in this composed source.
        remove_flattened_directive(
            &mut body,
            "merge-sort",
            "import Core.Classes.LawfulClasses (leqChar)",
        );
    }
    // Each runnable Rosetta file owns its Console/Process imports. The
    // flattened provider still needs leqChar before its declarations.
    let other_imports = "import Core.Classes.LawfulClasses (leqChar)\n";
    format!(
        "program capabilities FS APartial\n\
         import Core.Logic.Conjunction (And, and_intro, and_fst, and_snd)\n\
         {other_imports}{}\n{body}",
        collections_prelude()
    )
}

fn run_example(slug: &str, ken_src_path: &Path, tmp_dir: &Path) -> Option<(String, bool)> {
    let full_src = rosetta_program_source(slug, ken_src_path);
    let source_dir = workspace_root()
        .join("target/tmp/rosetta-strict-sources")
        .join(
            tmp_dir
                .file_name()
                .expect("named example scratch directory"),
        );
    fs::create_dir_all(&source_dir).expect("catalog-rooted example scratch directory");
    let concat_path = source_dir.join(format!("{slug}.ken"));
    fs::write(&concat_path, full_src)
        .unwrap_or_else(|e| panic!("{}: cannot write concatenated source: {}", slug, e));

    let mut child = Command::new(ken_bin())
        .arg("run")
        .arg(&concat_path)
        // fs-read-file-lines-flip: `read-file-lines.ken` names its fixture
        // as a workspace-root-relative path (`conformance/fs/fixtures/...`).
        // Cargo runs a test binary with CWD = the crate's own manifest dir
        // (`crates/ken-cli`), not the workspace root, so the spawned `ken`
        // binary must be pinned to the workspace root explicitly — every
        // other example does no filesystem access and is unaffected.
        .current_dir(workspace_root())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("{}: failed to spawn `ken run`: {}", slug, e));

    let start = Instant::now();
    loop {
        if let Some(status) = child.try_wait().expect("try_wait failed") {
            let output = child.wait_with_output().expect("wait_with_output failed");
            let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
            return Some((stdout, status.success()));
        }
        if start.elapsed() > PER_EXAMPLE_TIMEOUT {
            let _ = child.kill();
            let _ = child.wait();
            return None;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}

/// Promise class: durable invariant (spec 36 §1.2, LATENT AC-4).
/// MEASURED: the real Collections-prefixed Closures example checks and runs
/// to its output oracle through the existing flat-source CLI route.
/// CLAIMED: a pure callback carried by a catalog data field is not falsely
/// charged for an undeclared effect. THE GAP: this is one representative
/// example; the independent D0 paired census covers other affected sources.
#[test]
fn closures_carries_pure_callback_through_catalog_prelude() {
    let root = rosetta_dir().join("closures");
    let tmp = std::env::temp_dir().join(format!("ken-rosetta-closures-{}", std::process::id()));
    fs::create_dir_all(&tmp).expect("create isolated example output directory");
    let (output, successful) = run_example("closures", &root.join("closures.ken"), &tmp)
        .expect("the Collections-prefixed Closures example must complete");
    assert!(successful, "the checked example must be accepted");
    let Oracle::Expected(expected) = oracle_for(&root) else {
        panic!("the Closures example needs a checked output oracle");
    };
    assert_eq!(
        output, expected,
        "the accepted example must preserve its output"
    );
}

/// Promise class: durable invariant. MEASURED: the rewritten FizzBuzz source
/// accepts under Strict only with its explicit checked package imports, and
/// its unchanged CLI oracle records the same twenty output lines. CLAIMED:
/// changing a hand-built Console tree to a changed-result proc preserves
/// the application behavior without exposing ConsoleOp or Write as floor.
/// THE GAP: this checks the concrete program and its output, not every
/// recursive Console tail call in arbitrary Ken source.
#[test]
fn fizzbuzz_changed_proc_keeps_output_and_strict_import_boundary() {
    let root = rosetta_dir().join("fizzbuzz");
    let original = fs::read_to_string(root.join("fizzbuzz.ken")).expect("FizzBuzz source");
    let catalog = workspace_root()
        .join("catalog/packages")
        .canonicalize()
        .expect("catalog root");
    let mut selected = ElabEnv::new().expect("prelude");
    selected
        .elaborate_isolated_file_strict_v1(&[catalog.clone()], &original)
        .expect("changed-result FizzBuzz with persisted imports");
    let without_write = original.replacen("import Capability.Console.Text (write)\n", "", 1);
    assert_ne!(
        without_write, original,
        "negative must remove Console write import"
    );
    assert!(
        !without_write.contains("Capability.Console.Text (write)"),
        "negative must retain no Console write import"
    );
    let mut negative = ElabEnv::new().expect("prelude");
    match negative.elaborate_isolated_file_strict_v1(&[catalog], &without_write) {
        Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, "write"),
        Err(error) => panic!("FizzBuzz missing write: wrong refusal {error:?}"),
        Ok(_) => panic!("FizzBuzz accepted without the write import"),
    }
    let tmp = tempfile::tempdir().expect("FizzBuzz output directory");
    let (output, successful) = run_example("fizzbuzz", &root.join("fizzbuzz.ken"), tmp.path())
        .expect("FizzBuzz CLI run completes");
    assert!(successful, "the rewritten FizzBuzz program must run");
    let Oracle::Expected(expected) = oracle_for(&root) else {
        panic!("FizzBuzz requires its output oracle");
    };
    assert_eq!(output, expected, "FizzBuzz output after the R rewrite");
}

/// Promise class: durable invariant. MEASURED: the same read-file-lines
/// program checks with explicit FS/Console/HostIO imports under Strict and
/// still prints its exact CLI oracle on the current route. CLAIMED: the
/// checked HostIO wrapper substitution preserves the composed effects.
/// THE GAP: interpreter/native agreement beyond this example is exercised
/// by its dedicated differential tests, not by this one CLI output.
#[test]
fn read_file_lines_rewrite_keeps_oracle_and_strict_import_boundary() {
    let example = rosetta_dir().join("read-file-lines");
    let original =
        fs::read_to_string(example.join("read-file-lines.ken")).expect("read-file-lines source");
    let catalog = workspace_root()
        .join("catalog/packages")
        .canonicalize()
        .expect("catalog root");
    let mut strict = ElabEnv::new().expect("prelude");
    strict
        .elaborate_isolated_file_strict_v1(&[catalog.clone()], &original)
        .expect("read-file-lines with persisted checked imports");
    let without_wrappers = original.replacen(
        "import Capability.Process.Exit (host_bind, host_console, host_fs, host_program_then, host_pure)\n",
        "",
        1,
    );
    assert_ne!(
        without_wrappers, original,
        "negative must remove HostIO imports"
    );
    assert!(
        !without_wrappers.contains("import Capability.Process.Exit"),
        "negative must retain no HostIO import"
    );
    let mut negative = ElabEnv::new().expect("prelude");
    match negative.elaborate_isolated_file_strict_v1(&[catalog], &without_wrappers) {
        Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, "host_pure"),
        Err(error) => panic!("wrong no-import refusal: {error:?}"),
        Ok(_) => panic!("read-file-lines accepted without HostIO wrapper imports"),
    }
    let tmp = tempfile::tempdir().expect("read-file-lines output directory");
    let (output, successful) = run_example(
        "read-file-lines",
        &example.join("read-file-lines.ken"),
        tmp.path(),
    )
    .expect("read-file-lines CLI run completes");
    assert!(successful, "rewritten read-file-lines program must run");
    let Oracle::Expected(expected) = oracle_for(&example) else {
        panic!("read-file-lines requires its output oracle");
    };
    assert_eq!(output, expected, "read-file-lines output after R rewrite");
}

/// Promise class: durable invariant. MEASURED: the same accumulator program
/// checks at the strict entry with its package imports but no generic-bind
/// import, and its current CLI route keeps the output oracle. CLAIMED:
/// `itree_bind` and `resp_coproduct` are floor identities usable before
/// run_state, without changing the hidden-state program's result.
/// THE GAP: this output does not establish every State handler equation.
#[test]
fn accumulator_factory_uses_generic_floor_bind_and_keeps_its_oracle() {
    let example = rosetta_dir().join("accumulator-factory");
    let source =
        fs::read_to_string(example.join("accumulator-factory.ken")).expect("accumulator source");
    let catalog = workspace_root()
        .join("catalog/packages")
        .canonicalize()
        .expect("catalog root");
    let mut strict = ElabEnv::new().expect("prelude");
    strict
        .elaborate_isolated_file_strict_v1(&[catalog], &source)
        .expect("direct State tree names the generic floor without an import");
    let tmp = tempfile::tempdir().expect("accumulator output directory");
    let (output, successful) = run_example(
        "accumulator-factory",
        &example.join("accumulator-factory.ken"),
        tmp.path(),
    )
    .expect("accumulator CLI run completes");
    assert!(successful, "the rewritten State tree must run");
    let Oracle::Expected(expected) = oracle_for(&example) else {
        panic!("accumulator requires its output oracle");
    };
    assert_eq!(
        output, expected,
        "accumulator output after the floor spelling"
    );
}

/// Promise class: durable invariant. MEASURED: each of the four existing
/// Collections-prefixed users elaborates under the strict entry with the
/// explicit Conjunction import, while the same source without it refuses an
/// unbound conjunction name. CLAIMED: the imported checked conjunction is
/// sufficient for these users, rather than adding the pre-source And to the
/// floor. THE GAP: their output parity is independently checked by the
/// existing CLI oracle test, not by this strict elaboration pin.
#[test]
fn collections_users_recheck_with_explicit_conjunction_import_under_strict() {
    let root = workspace_root()
        .join("catalog/packages")
        .canonicalize()
        .expect("catalog root");
    let imports = "import Core.Logic.Conjunction (And, and_intro, and_fst, and_snd)\n";
    for slug in NEEDS_COLLECTIONS {
        let source_path = rosetta_dir().join(slug).join(format!("{slug}.ken"));
        let composed = rosetta_program_source(slug, &source_path);
        let mut env = ElabEnv::new().expect("prelude");
        env.elaborate_isolated_file_strict_v1(&[root.clone()], &composed)
            .unwrap_or_else(|error| panic!("{slug}: strict conjunction import: {error:?}"));
        let without_conjunction = composed.replacen(imports, "", 1);
        assert_ne!(
            without_conjunction, composed,
            "{slug}: negative must remove the conjunction import"
        );
        assert!(
            !without_conjunction.contains(imports),
            "{slug}: negative must retain no conjunction import"
        );
        let mut negative = ElabEnv::new().expect("prelude");
        match negative.elaborate_isolated_file_strict_v1(&[root.clone()], &without_conjunction) {
            Err(ElabError::UnboundName { name, .. }) => assert!(
                ["And", "and_intro", "and_fst", "and_snd"].contains(&name.as_str()),
                "{slug}: no-import first missing name {name}"
            ),
            Err(error) => panic!("{slug}: wrong no-import refusal: {error:?}"),
            Ok(_) => panic!("{slug}: strict source accepted without its conjunction import"),
        }
    }
}

#[test]
fn rosetta_examples_match_their_oracles() {
    let root = rosetta_dir();
    let tmp_dir = std::env::temp_dir().join("ken-rosetta-runner");
    fs::create_dir_all(&tmp_dir).expect("create tmp dir");

    let mut entries: Vec<PathBuf> = fs::read_dir(&root)
        .unwrap_or_else(|e| panic!("cannot read {}: {}", root.display(), e))
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    entries.sort();

    assert!(
        !entries.is_empty(),
        "expected at least one dir under {}",
        root.display()
    );

    let mut failures: Vec<String> = Vec::new();
    let mut known_gaps: Vec<String> = Vec::new();
    let mut passed: Vec<String> = Vec::new();

    for dir in &entries {
        let slug = dir.file_name().unwrap().to_string_lossy().into_owned();
        let ken_path = dir.join(format!("{slug}.ken"));
        if !ken_path.exists() {
            failures.push(format!("{slug}: missing {slug}.ken"));
            continue;
        }

        match oracle_for(dir) {
            Oracle::KnownGap => {
                known_gaps.push(slug);
            }
            Oracle::Expected(expected) => match run_example(&slug, &ken_path, &tmp_dir) {
                None => failures.push(format!("{slug}: timed out after {PER_EXAMPLE_TIMEOUT:?}")),
                Some((stdout, success)) => {
                    if !success {
                        failures.push(format!(
                            "{slug}: `ken run` exited non-zero, stdout: {stdout:?}"
                        ));
                    } else if stdout != expected {
                        failures.push(format!(
                            "{slug}: stdout mismatch — expected {expected:?}, got {stdout:?}"
                        ));
                    } else {
                        passed.push(slug);
                    }
                }
            },
        }
    }

    println!("\n=== Rosetta pangram findings summary ===");
    println!("PASS ({}): {:?}", passed.len(), passed);
    println!("KNOWN-GAP ({}): {:?}", known_gaps.len(), known_gaps);
    println!("FAIL ({}): {:?}", failures.len(), failures);

    assert!(
        failures.is_empty(),
        "rosetta examples failed:\n{}",
        failures.join("\n")
    );
}
