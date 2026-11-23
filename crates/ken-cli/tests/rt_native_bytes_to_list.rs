//! Program I parity and governed allocation for bytes_to_list (spec 18a, 37 §2.6).
#![cfg(target_os = "linux")]

use std::ffi::OsStr;
use std::os::unix::{ffi::OsStrExt, process::CommandExt};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use ken_host::{CapacityExhaustedV1, CapacityResourceV1, CapacityScopeV1, TerminalErrorV1};
use ken_runtime::boundary_resource_profile::{
    starter_smoke_profile, BoundaryRegionLimitsV1, BoundaryResourceProfileV3,
};

fn source(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/bytes_to_list")
        .join(format!("{name}.ken"))
}

fn build(name: &str, root: &Path, label: &str, profile: BoundaryResourceProfileV3) -> PathBuf {
    let region = |r: BoundaryRegionLimitsV1| {
        serde_json::json!({
            "nodes": r.nodes, "words": r.words, "data_bytes": r.data_bytes,
            "native_int_limbs": r.native_int_limbs,
        })
    };
    let profile = serde_json::json!({
        "runtime": { "invocation_epochs": profile.runtime.invocation_epochs },
        "call_events": {
            "event_generations": profile.call_events.event_generations,
            "live_pending_slots": profile.call_events.live_pending_slots,
        },
        "invocation": region(profile.invocation),
        "persistent": region(profile.persistent),
    });
    let out = root.join(label);
    std::fs::create_dir(&out).unwrap();
    let profile_path = out.join("profile.json");
    std::fs::write(&profile_path, profile.to_string()).unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_ken"))
        .arg("native-build")
        .arg(source(name))
        .arg(&out)
        .arg(profile_path)
        .env_remove("RUST_MIN_STACK")
        .output()
        .expect("invoke native-build");
    assert!(
        result.status.success(),
        "{name}: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    out.join("ken-starter")
}

fn check_exit(out: Output, expected: i32, lane: &str, name: &str) {
    assert_eq!(
        out.status.code(),
        Some(expected),
        "{name} {lane}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        out.stdout.is_empty(),
        "{name} {lane} emitted unexpected output"
    );
}

fn run(exe: &Path, root: &Path, argv: &[&[u8]]) -> Command {
    let (first, rest) = argv.split_first().expect("explicit argv[0]");
    let mut cmd = Command::new(exe);
    cmd.current_dir(root)
        .env_remove("KEN_HOST_OBSERVATION_PATH")
        .arg0(OsStr::from_bytes(first))
        .args(rest.iter().map(|arg| OsStr::from_bytes(arg)));
    cmd
}

/// Durable invariant: compare each real lane separately to an input-derived
/// answer. The arm named by a fixture is established by arm-drop controls,
/// not by examining its surface syntax.
fn parity(name: &str, rows: &[(&[&[u8]], i32)]) {
    let root = tempfile::tempdir().unwrap();
    std::fs::write(root.path().join("input.bin"), b"a\xff").unwrap();
    let exe = build(name, root.path(), "native", starter_smoke_profile());
    for (argv, expected) in rows {
        let interp = Command::new(env!("CARGO_BIN_EXE_ken"))
            .arg("run")
            .arg(source(name))
            .arg("--")
            .args(argv.iter().map(|arg| OsStr::from_bytes(arg)))
            .current_dir(root.path())
            .env_remove("RUST_MIN_STACK")
            .output()
            .expect("invoke interpreter");
        check_exit(interp, *expected, "interpreter", name);
        check_exit(
            run(&exe, root.path(), argv).output().unwrap(),
            *expected,
            "native",
            name,
        );
    }
}

#[test]
fn p5_literal_list_uint8_projects_to_int() {
    parity("p5", &[(&[b""], 75), (&[b"ab"], 0)]);
}
#[test]
fn p9_recursive_list_int_projects_to_eq_int() {
    parity("p9", &[(&[b"ab", b""], 75), (&[b"ab", b"c"], 0)]);
}
#[test]
fn p10_recursive_list_of_options_matches_the_element() {
    parity("p10", &[(&[b"ab"], 0)]);
}
#[test]
fn p7_byte_pattern_discriminates_content() {
    parity("p7", &[(&[b"ab"], 0), (&[b"ac"], 75)]);
}
#[test]
fn p6_borrowed_argument_is_converted_in_main() {
    parity("p6", &[(&[b""], 75), (&[b"ab"], 0)]);
}
#[test]
fn response_span_is_read_in_its_reply_body() {
    parity("response", &[(&[b"program"], 0)]);
}
/// Durable invariant: both lanes must return success only for the three
/// UTF-8 octets of `aß` in order, followed immediately by Nil.
#[test]
fn static_bytes_preserve_exact_utf8_order_and_nil() {
    parity("static", &[(&[b"program", b"unused"], 0)]);
}
#[test]
fn carried_borrowed_argument_crosses_a_recursive_unit() {
    parity("borrowed-carried", &[(&[b"program", b"ab"], 0)]);
}
#[test]
fn carried_host_reply_crosses_a_recursive_unit() {
    parity("persistent", &[(&[b"program"], 0)]);
}

fn observed(exe: &Path, root: &Path, octets: &[u8]) -> ken_runtime::LinkedEffectTrace {
    let trace = root.join("trace");
    let _ = std::fs::remove_file(&trace);
    let out = run(exe, root, &[b"program", octets])
        .env("KEN_HOST_OBSERVATION_PATH", &trace)
        .output()
        .unwrap();
    ken_runtime::decode_linked_effect_trace(
        &std::fs::read(trace).unwrap_or_else(|e| panic!("no observation: {e}; {out:?}")),
    )
    .unwrap()
}

/// Durable invariant: compare the live end counts and the exact exhaustion
/// witness, not an allotment large enough to hide one extra allocation.
#[test]
fn materialized_list_has_exact_persistent_node_threshold() {
    let root = tempfile::tempdir().unwrap();
    let mut profile = starter_smoke_profile();
    let exe = build("cost", root.path(), "baseline", profile);
    let empty = observed(&exe, root.path(), b"").region_end.unwrap();
    let octets = b"abcde";
    let n = octets.len() as u64;
    let full = observed(&exe, root.path(), octets).region_end.unwrap();
    // Measured with this fixture and the starter profile: empty uses one
    // persistent node (N1 Nil), leaving a zero-node non-list baseline.
    // Five bytes use six nodes; limit 6 passes and limit 5 requests 6.
    assert_eq!(empty.persistent.nodes, 1, "zero-length argument is one Nil");
    let baseline = 0;
    let limit = baseline + n + 1;
    assert_eq!(full.invocation, empty.invocation);
    assert_eq!(full.persistent.nodes, limit);
    assert_eq!(full.persistent.words, empty.persistent.words + 2 * n);
    assert_eq!(full.persistent.data_bytes, empty.persistent.data_bytes);
    assert_eq!(
        full.persistent.native_int_limbs,
        empty.persistent.native_int_limbs
    );
    profile.persistent.nodes = limit as usize;
    let exe = build("cost", root.path(), "at-limit", profile);
    check_exit(
        run(&exe, root.path(), &[b"program", octets])
            .output()
            .unwrap(),
        0,
        "native",
        "cost",
    );
    profile.persistent.nodes -= 1;
    let exe = build("cost", root.path(), "below-limit", profile);
    let failed = observed(&exe, root.path(), octets);
    assert_eq!(
        failed.terminal_error,
        Some(TerminalErrorV1::CapacityExhausted(CapacityExhaustedV1 {
            scope: CapacityScopeV1::Persistent,
            resource: CapacityResourceV1::Nodes,
            limit: u128::from(limit - 1),
            requested: u128::from(limit),
        }))
    );
}
