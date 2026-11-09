//! The CLI routes use the same Strict isolated-source import boundary.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const WITH_IMPORT: &str = "program capabilities FS ANone\n\
import Capability.Process.Exit (host_pure)\n\
fn main (_input : ProcessInput) (_caps : ProgramCaps ANone)\n\
  : HostIO ANone ExitCode = host_pure ANone ExitCode ExitCode.Success\n";
const WITHOUT_IMPORT: &str = "program capabilities FS ANone\n\
fn main (_input : ProcessInput) (_caps : ProgramCaps ANone)\n\
  : HostIO ANone ExitCode = host_pure ANone ExitCode ExitCode.Success\n";
const PROFILE: &str = r#"{"runtime":{"invocation_epochs":18446744073709551615},"call_events":{"event_generations":18446744073709551615,"live_pending_slots":64},"invocation":{"nodes":64,"words":256,"data_bytes":512,"native_int_limbs":64},"persistent":{"nodes":64,"words":256,"data_bytes":512,"native_int_limbs":64}}"#;

fn collect_sources(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).expect("source tree exists") {
        let path = entry.expect("source entry").path();
        if path.is_dir() {
            collect_sources(&path, out);
        } else if path.to_string_lossy().ends_with(".ken")
            || path.to_string_lossy().ends_with(".ken.md")
        {
            out.push(path);
        }
    }
}

fn invoke(route: &str, source: &Path, dir: &Path) -> Output {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_ken"));
    cmd.arg(route).arg(source);
    if route == "native-build" {
        cmd.arg(dir.join("artifact")).arg(dir.join("profile.json"));
    }
    cmd.output().expect("execute CLI binary")
}

/// Promise class: transition sentinel, retired when the cut-base cohort or
/// its one compiler-internal relay exemption changes. The AC-0 census found
/// 98 isolated sources and 64 Legacy-clean at the cut; the relay keeps its
/// separate Legacy elaboration as a core-term fixture (30 §4, I row). MEASURED:
/// every presently Strict-clean physical root runs through the real CLI check
/// binary, not just the library's direct entry. CLAIMED: every cut-base
/// Legacy-clean isolated file except that single internal fixture checks on
/// the switched route. THE GAP: Rust-held source variants are separately
/// provisioned and tested by their owning binaries, not by this file cohort.
#[test]
fn cut_base_legacy_clean_file_cohort_checks_through_switched_cli_route() {
    const CUT_LEGACY_CLEAN: usize = 64;
    const INTERNAL_CORE_ONLY: usize = 1;
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let relay = repo.join("crates/ken-cli/tests/fixtures/rt_performing_mixed_owner_relay.ken");
    let relay_source = std::fs::read_to_string(&relay).expect("internal relay fixture");
    let mut legacy = ken_elaborator::ElabEnv::new().expect("prelude");
    legacy
        .elaborate_file_v1(&relay_source)
        .expect("internal core fixture retains its Legacy elaboration");
    let mut strict = ken_elaborator::ElabEnv::new().expect("prelude");
    match strict.elaborate_isolated_file_strict_v1(&[], &relay_source) {
        Err(ken_elaborator::ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, "BufferHandle")
        }
        other => panic!("internal fixture must not gain a Strict home: {other:?}"),
    }
    let mut files = Vec::new();
    for tree in ["examples", "conformance", "crates"] {
        collect_sources(&repo.join(tree), &mut files);
    }
    files.retain(|path| {
        !path.components().any(|part| part.as_os_str() == "catalog")
            && (!path.components().any(|part| part.as_os_str() == "crates")
                || path.components().any(|part| part.as_os_str() == "tests"))
    });
    files.sort();
    let mut accepted = Vec::new();
    for path in &files {
        let source = std::fs::read_to_string(path).expect("physical Ken source");
        let roots: Vec<_> = ken_elaborator::modules::isolated_catalog_root(path)
            .into_iter()
            .collect();
        let mut env = ken_elaborator::ElabEnv::new().expect("prelude");
        let result = if path.to_string_lossy().ends_with(".ken.md") {
            env.elaborate_isolated_ken_md_file_strict_v1(&roots, &source)
        } else {
            env.elaborate_isolated_file_strict_v1(&roots, &source)
        };
        if result.is_ok() {
            accepted.push(path);
        }
    }
    assert_eq!(files.len(), 98, "cut-base isolated file population changed");
    assert_eq!(
        accepted.len(),
        CUT_LEGACY_CLEAN - INTERNAL_CORE_ONLY,
        "cut-base Legacy-clean files, except the internal relay, need imports"
    );
    for path in accepted {
        let checked = Command::new(env!("CARGO_BIN_EXE_ken"))
            .arg("check")
            .arg(path)
            .output()
            .expect("real checked CLI root");
        assert!(
            checked.status.success(),
            "{}: {}",
            path.display(),
            String::from_utf8_lossy(&checked.stderr)
        );
    }
}

/// Promise class: durable invariant (spec/30-surface/33 §3.3 and 70 §1.4).
/// MEASURED: each shipped CLI route refuses the exact unimported non-floor
/// reference with UnboundName and accepts the same program with its home.
/// CLAIMED: all three paths use Strict, not Legacy, for isolated roots.
/// THE GAP: the imported and unimported sources must differ only by the
/// import line; the compared program and its discovered catalog root are fixed.
/// A compile-preserving evasion routing any one path to Legacy makes its
/// unimported case accept and fails that route's exact diagnostic assertion.
#[test]
fn three_cli_routes_refuse_unimported_name_and_accept_imported_twin() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let dir = repo.join("target/tmp/strict-isolated-routes-inc3b-cli");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("profile.json"), PROFILE).unwrap();
    let source = dir.join("root.ken");
    for route in ["check", "run", "native-build"] {
        std::fs::write(&source, WITHOUT_IMPORT).unwrap();
        let refused = invoke(route, &source, &dir);
        assert!(!refused.status.success(), "{route} must refuse no-import root");
        let stderr = String::from_utf8_lossy(&refused.stderr);
        assert!(
            stderr.contains("UnboundName { name: \"host_pure\""),
            "{route} must use Strict's unbound-name diagnostic: {stderr}"
        );
        std::fs::write(&source, WITH_IMPORT).unwrap();
        let accepted = invoke(route, &source, &dir);
        assert!(
            accepted.status.success(),
            "{route} imported twin: {}",
            String::from_utf8_lossy(&accepted.stderr)
        );
    }
}

/// Promise class: durable invariant (33 §3.2). MEASURED: the real check CLI
/// resolves a relative root and its `..`-spelled twin from the source's
/// normalized parent, while a closer decoy root selects a different checked
/// type. CLAIMED: root discovery follows the source file, not process cwd or
/// an unnormalized lexical `nested` component. THE GAP: the decoy must be a
/// reachable root that would reject this same source; the third invocation
/// observes that rejection rather than relying on path text.
#[test]
fn relative_and_parent_spelled_source_choose_the_same_nearest_catalog_root() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let dir = repo.join("target/tmp/strict-isolated-routes-inc3b-root-paths");
    let _ = std::fs::remove_dir_all(&dir);
    let packages = dir.join("catalog/packages");
    let sub = dir.join("sub");
    let nested = sub.join("nested");
    let decoy = nested.join("catalog/packages");
    std::fs::create_dir_all(&packages).unwrap();
    std::fs::create_dir_all(&decoy).unwrap();
    std::fs::write(
        packages.join("Dependency.ken.md"),
        "# Dependency\n\n```ken\npub const value : Nat = Zero\n```\n",
    )
    .unwrap();
    std::fs::write(
        decoy.join("Dependency.ken.md"),
        "# Decoy\n\n```ken\npub const value : Bool = True\n```\n",
    )
    .unwrap();
    let program = "import Dependency (value)\nconst answer : Nat = value\n";
    std::fs::write(sub.join("entry.ken"), program).unwrap();
    std::fs::write(nested.join("entry.ken"), program).unwrap();
    let invoke_from_sub = |spelling: &str| {
        Command::new(env!("CARGO_BIN_EXE_ken"))
            .current_dir(&sub)
            .args(["check", spelling])
            .output()
            .expect("check from source's sibling directory")
    };
    for spelling in ["entry.ken", "nested/../entry.ken"] {
        let result = invoke_from_sub(spelling);
        assert!(
            result.status.success(),
            "{spelling} must select the source's root: {}",
            String::from_utf8_lossy(&result.stderr)
        );
    }
    let decoy_result = invoke_from_sub("nested/entry.ken");
    assert!(
        !decoy_result.status.success(),
        "the closer nested root must not look like the source's root"
    );
    assert!(
        String::from_utf8_lossy(&decoy_result.stderr).contains("TypeMismatch"),
        "the decoy must be reached: {}",
        String::from_utf8_lossy(&decoy_result.stderr)
    );
}
