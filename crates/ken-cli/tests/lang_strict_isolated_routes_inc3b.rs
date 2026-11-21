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

fn invoke(route: &str, source: &Path, dir: &Path) -> Output {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_ken"));
    cmd.arg(route).arg(source);
    if route == "native-build" {
        cmd.arg(dir.join("artifact")).arg(dir.join("profile.json"));
    }
    cmd.output().expect("execute CLI binary")
}

/// Promise class: transition sentinel for the frozen post-RNSPK file cohort.
/// MEASURED: each parent Legacy-clean physical source other than the private
/// relay exists and passes the real `ken check` route without a source rewrite.
/// CLAIMED: the one scoped route handles every file in this frozen cohort.
/// THE GAP: Rust-held source variants and the private relay remain owned by
/// their separate binaries; this list was measured on parent 15e57bf39.
#[test]
fn post_rnspk_file_cohort_checks_through_cli_route() {
    const FILES: [&str; 73] = [
        "conformance/challenge/C2-proof-relevant-omega/sound-perm-count.ken",
        "conformance/challenge/C3-codata-totality/sound-fuel-unfold.ken",
        "crates/ken-cli/tests/fixtures/abi_s5_block_on_stdin.ken",
        "crates/ken-cli/tests/fixtures/abi_s6_mapping_file_backed.ken",
        "crates/ken-cli/tests/fixtures/abi_s6_mapping_file_backed_cow.ken",
        "crates/ken-cli/tests/fixtures/d0_branched_scrutinee_source.ken",
        "crates/ken-cli/tests/fixtures/lang_effect_row_local_latent/qa_higher_order_let.ken",
        "crates/ken-cli/tests/fixtures/px10_argv_env_pipe.ken",
        "crates/ken-cli/tests/fixtures/px10_child_wait_smoke.ken",
        "crates/ken-cli/tests/fixtures/px10_live_child.ken",
        "crates/ken-cli/tests/fixtures/px10_native_tested_pending_overlap.ken",
        "crates/ken-cli/tests/fixtures/px10_recursive_performed_p10.ken",
        "crates/ken-cli/tests/fixtures/px10_recursive_performed_p2.ken",
        "crates/ken-cli/tests/fixtures/px10_recursive_performed_p5.ken",
        "crates/ken-cli/tests/fixtures/px10_recursive_performed_p7.ken",
        "crates/ken-cli/tests/fixtures/px10_recursive_performed_p8.ken",
        "crates/ken-cli/tests/fixtures/px10_recursive_performed_vlet2.ken",
        "crates/ken-cli/tests/fixtures/px10_recursive_performed_vmatch2.ken",
        "crates/ken-cli/tests/fixtures/px10_recursive_performed_vmatch_mixed.ken",
        "crates/ken-cli/tests/fixtures/resolution_report/accepted.ken",
        "crates/ken-cli/tests/fixtures/resolution_report/shadow.ken",
        "crates/ken-cli/tests/fixtures/rnspk_v_argv_stdout_null.ken",
        "crates/ken-cli/tests/fixtures/rt_bracket_body_once_buffer.ken",
        "crates/ken-cli/tests/fixtures/rt_bracket_body_once_lookalike.ken",
        "crates/ken-cli/tests/fixtures/rt_bracket_body_once_no_bracket.ken",
        "crates/ken-cli/tests/fixtures/rt_bracket_body_once_resource.ken",
        "crates/ken-cli/tests/fixtures/rt_bracket_body_once_three.ken",
        "crates/ken-cli/tests/fixtures/rt_native_erased_omega_call_explicit.ken",
        "crates/ken-cli/tests/fixtures/rt_native_erased_omega_call_plain.ken",
        "crates/ken-cli/tests/fixtures/rt_native_erased_omega_call_refined.ken",
        "crates/ken-cli/tests/fixtures/rt_native_erased_omega_call_refnonrec.ken",
        "crates/ken-cli/tests/fixtures/rt_native_erased_omega_call_requires.ken",
        "crates/ken-cli/tests/fixtures/rt_native_erased_omega_call_requires_dropped.ken",
        "crates/ken-cli/tests/fixtures/rt_native_then_chain_source_slot_transfer.ken",
        "crates/ken-cli/tests/fixtures/rt_nested_read_at_b2.ken",
        "crates/ken-cli/tests/fixtures/rt_nested_read_at_more.ken",
        "crates/ken-cli/tests/fixtures/rt_nested_read_captured_read.ken",
        "crates/ken-cli/tests/fixtures/rt_nested_read_length_more.ken",
        "crates/ken-cli/tests/fixtures/rt_nested_read_nested_control.ken",
        "crates/ken-cli/tests/fixtures/rt_nested_read_single_read_control.ken",
        "crates/ken-cli/tests/fixtures/rt_performing_underived_acmain.ken",
        "crates/ken-cli/tests/rt_ignored_two_buffer_witness.ken",
        "crates/ken-cli/tests/rt_nat_fanout_escaped_resource.ken",
        "crates/ken-cli/tests/rt_nat_fanout_reached_live_resource.ken",
        "crates/ken-cli/tests/rt_one_buffer_residual_layout.ken",
        "crates/ken-cli/tests/rt_per_emitter_one_bracket.ken",
        "crates/ken-cli/tests/rt_plain_match_layout.ken",
        "crates/ken-cli/tests/rt_two_bracket_distinguishable.ken",
        "crates/ken-elaborator/tests/fixtures/check_refl_closed_step_fold.ken",
        "crates/ken-elaborator/tests/fixtures/lang_generated_j/e1.ken.md",
        "crates/ken-elaborator/tests/fixtures/lang_generated_j/e4.ken.md",
        "crates/ken-elaborator/tests/fixtures/lang_generated_j/e5.ken.md",
        "crates/ken-elaborator/tests/fixtures/obs_type_eq_structural/f4.ken.md",
        "crates/ken-elaborator/tests/fixtures/obs_type_eq_structural/f7.ken.md",
        "crates/ken-elaborator/tests/fixtures/rt_bracket_region_early_release.ken",
        "crates/ken-elaborator/tests/fixtures/rt_bracket_transport_file_backed.ken",
        "crates/ken-elaborator/tests/fixtures/rt_bracket_transport_mapping.ken",
        "crates/ken-elaborator/tests/fixtures/sibling_goal_refinement/f5_reconstructed.ken.md",
        "crates/ken-elaborator/tests/fixtures/sibling_goal_refinement/f5_reconstructed_ascribed.ken.md",
        "crates/ken-elaborator/tests/fixtures/sibling_goal_refinement/f6_reconstructed.ken.md",
        "crates/ken-elaborator/tests/fixtures/sibling_goal_refinement/f6_reconstructed_no_witness.ken.md",
        "examples/rosetta/accumulator-factory/accumulator-factory.ken",
        "examples/rosetta/ackermann/ackermann.ken",
        "examples/rosetta/factorial/factorial.ken",
        "examples/rosetta/fibonacci/fibonacci.ken",
        "examples/rosetta/fizzbuzz/fizzbuzz.ken",
        "examples/rosetta/gcd/gcd.ken",
        "examples/rosetta/hailstone/hailstone.ken",
        "examples/rosetta/hello-world/hello-world.ken",
        "examples/rosetta/letter-frequency/letter-frequency.ken",
        "examples/rosetta/mutual-recursion/mutual-recursion.ken",
        "examples/rosetta/read-file-lines/read-file-lines.ken",
        "examples/rosetta/rpn-calculator/rpn-calculator.ken",
    ];
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repository root");
    for relative in FILES {
        let path = repo.join(relative);
        assert!(path.is_file(), "frozen source missing: {}", path.display());
        let checked = Command::new(env!("CARGO_BIN_EXE_ken"))
            .arg("check")
            .arg(&path)
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
        assert!(
            !refused.status.success(),
            "{route} must refuse no-import root"
        );
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
