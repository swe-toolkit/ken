//! Spec 38 §1.13, PX11 stream sockets. Each case reaches the checked Ken
//! surface, interpreter, host dispatcher and Linux socket leaf.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const PROFILE: &str = r#"{"runtime":{"invocation_epochs":18446744073709551615},"call_events":{"event_generations":18446744073709551615,"live_pending_slots":64},"invocation":{"nodes":64,"words":256,"data_bytes":512,"native_int_limbs":64},"persistent":{"nodes":64,"words":256,"data_bytes":512,"native_int_limbs":64}}"#;

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn run(name: &str, cwd: &Path, arguments: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_ken"))
        .current_dir(cwd)
        .arg("run")
        .arg(fixture(name))
        .arg("--")
        .args(arguments)
        .output()
        .expect("run checked Ken source")
}

fn assert_success(name: &str, output: Output) {
    assert_eq!(
        output.status.code(),
        Some(0),
        "{name}: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

/// Promise class: durable invariant (the seven Net ops stay interpreter-live).
/// MEASURED: real Ken listener, ephemeral-port readback, connect/accept,
/// full-byte sends and reads in both directions, shutdown then EOF across
/// IPv4, IPv6 and Unix. CLAIMED: the three address families' stream contracts
/// are executable. THE GAP: the Linux host is required to support all three
/// address families; this case does not establish native support (AC-4 refuses).
#[test]
fn px11_all_three_families_exchange_exact_bytes_and_observe_eof() {
    for family in ["4", "6", "unix"] {
        let dir = tempfile::Builder::new()
            .prefix("ken-px11-")
            .tempdir_in("/tmp")
            .expect("isolated socket fixture directory");
        std::fs::write(dir.path().join("px11-payload.bin"), b"PX11ok!")
            .expect("known-answer payload");
        let unix_path = dir.path().join("listener.sock");
        let unix = unix_path.to_str().expect("UTF-8 temporary path");
        assert_success(
            family,
            run("px11_stream_exchange.ken", dir.path(), &[family, unix]),
        );
        if family == "unix" {
            assert!(
                unix_path.exists(),
                "socket pathname survives bracket release"
            );
        } else {
            assert!(
                !unix_path.exists(),
                "IP traffic does not create a Unix socket"
            );
        }
    }
}

/// Promise class: durable invariant. MEASURED: a valid absolute Unix address
/// without a listener returns the canonical NotFound cause through the
/// SocketOp ConnectSocket and SocketResource Some envelope. CLAIMED: the
/// address-aware System.Error conversion preserves typed failure identity.
/// THE GAP: the checked theorem in the fixture pins the address itself;
/// the runtime match observes the envelope's constructor and cause.
#[test]
fn px11_missing_unix_listener_has_typed_socket_error() {
    let dir = tempfile::Builder::new()
        .prefix("ken-px11-")
        .tempdir_in("/tmp")
        .unwrap();
    let unix_path = dir.path().join("missing.sock");
    assert!(!unix_path.exists(), "negative case requires no listener");
    let path = unix_path.to_str().unwrap();
    assert_success(
        "missing listener",
        run("px11_missing_listener.ken", dir.path(), &[path]),
    );
}

/// Promise class: durable invariant. MEASURED: a Unix address under only a
/// Loopback Net grant produces CapabilityDenied in the typed socket envelope.
/// CLAIMED: a grant for a different address class cannot authorize connect.
/// THE GAP: the host-level backend-call counter independently checks that
/// denial precedes the socket syscall; an eventual NotFound must not suffice.
#[test]
fn px11_wrong_grant_refuses_unix_connect() {
    let dir = tempfile::Builder::new()
        .prefix("ken-px11-")
        .tempdir_in("/tmp")
        .unwrap();
    let path = dir.path().join("absent.sock");
    assert_success(
        "wrong grant",
        run(
            "px11_wrong_grant.ken",
            dir.path(),
            &[path.to_str().unwrap()],
        ),
    );
}

/// Promise class: transition sentinel, retired by the native-parity successor.
/// MEASURED: native-build of the same checked Net program returns the exact
/// pre-emission unavailable diagnostic and an empty output directory.
/// CLAIMED: no Net-declared program falls through to native object emission.
/// THE GAP: the gate applies to the declared Net plan; the test does not
/// enumerate every possible Net syntax spelling.
#[test]
fn px11_declared_net_native_build_refuses_before_emission() {
    let dir = tempfile::tempdir().unwrap();
    let output_dir = dir.path().join("native");
    std::fs::create_dir(&output_dir).unwrap();
    let profile = dir.path().join("profile.json");
    std::fs::write(&profile, PROFILE).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_ken"))
        .arg("native-build")
        .arg(fixture("px11_stream_exchange.ken"))
        .arg(&output_dir)
        .arg(profile)
        .output()
        .expect("attempt native build through checked source");
    assert_eq!(output.status.code(), Some(1), "{output:?}");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("host_effect_lowering_unavailable"),
        "{stderr}"
    );
    assert!(stderr.contains("NetListen"), "{stderr}");
    assert!(stderr.contains("RepresentedUnavailable"), "{stderr}");
    assert_eq!(std::fs::read_dir(&output_dir).unwrap().count(), 0);
}
