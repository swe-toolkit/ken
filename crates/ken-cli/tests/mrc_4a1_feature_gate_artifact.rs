//! `RT-MATCH-RECURSOR-CONSUMERS` `AC-1`, frame section 4a.1 -- the feature gate,
//! asserted at the ARTIFACT rather than at the source.
//!
//! The sibling suite's feature-on/no-session control proves the mechanism is
//! inert at RUNTIME when no session is opened. This one proves something the
//! runtime cannot: that with the feature off the transport is **not in the
//! binary at all** -- no environment read, no envelope writer, not even the
//! strings they are spelled with.
//!
//! Source inspection cannot discharge this. `with_child_match_recursor_census`
//! is deliberately an unconditional `pub` item whose whole body is gated, because
//! `ken-cli` has no feature of its own to test (it receives
//! `px8-ds-test-support` only through a `[dev-dependencies]` edge, which enables
//! the feature on ken-runtime's unit without defining any `cfg` visible to
//! `ken-cli`'s sources). Whether that gating actually removed anything is a fact
//! about the emitted artifact, so the artifact is what gets read.
//!
//! # Which file to read is the whole problem
//!
//! An artifact-level A/B is only as good as the claim that the two reads are of
//! the two configurations' own binaries. Reading one configuration's file for the
//! other's is the failure that matters, because a feature-on artifact misread as
//! feature-off yields **zero transport strings** -- exactly what success looks
//! like.
//!
//! Sharing one target directory between the two configurations is what makes
//! that hard, and three separate attempts here got it wrong:
//!
//! 1. `<target>/debug/ken` is an **uplift** shared by every configuration built
//!    into that directory, so which build it reflects is a question rather than
//!    a given. A feature-on build once compiled `ken_runtime` with
//!    `--cfg feature="px8-ds-test-support"`, reported `Finished`, and the file
//!    read from that path afterwards had none of the transport strings and
//!    installed no observation when run.
//! 2. That uplift is a **hard link**, so it carries the modification time of the
//!    `deps/` artifact behind it. A correctly cached build therefore looks
//!    "old", and a freshness check built on that mtime reddens on a build that
//!    did nothing wrong -- passing cold and failing warm.
//! 3. Asking cargo which file it produced does not help either: the
//!    `compiler-artifact` message's `executable` field reports that same shared
//!    uplift, so both configurations name one path.
//!
//! So the sharing is removed instead of worked around. **Each configuration gets
//! its own target directory**, which makes the two artifacts physically distinct
//! files that no uplift, link, or timestamp can conflate. It costs a second
//! dependency build; every cheaper arrangement tried above was wrong in the
//! direction that reads as success.
//!
//! What holds the control together is then three assertions: the two artifacts
//! differ byte-for-byte, a needle present in both is actually found, and the
//! transport strings are absent from one and present exactly once in the other.

use std::path::{Path, PathBuf};
use std::process::Command;

/// The transport's own vocabulary. Present exactly once each when the feature is
/// on; entirely absent when it is off.
const TRANSPORT_STRINGS: [&str; 4] = [
    "KEN_MRC_CENSUS_SESSION",
    "KEN_MRC_CENSUS_PARENT",
    "KEN_MRC_CENSUS_SINK",
    "mrc-census-envelope",
];

/// A needle that must appear in BOTH artifacts. Without it, "zero transport
/// strings" is equally consistent with a correct gate and with a reader that
/// cannot find anything at all -- a vacuous pass. This is the positive control
/// on the instrument itself.
const ALWAYS_PRESENT: &str = "native-build";

fn count_occurrences(haystack: &[u8], needle: &str) -> usize {
    let needle = needle.as_bytes();
    if needle.is_empty() || haystack.len() < needle.len() {
        return 0;
    }
    haystack
        .windows(needle.len())
        .filter(|window| *window == needle)
        .count()
}

/// Build the `ken` binary in one feature configuration, in that configuration's
/// OWN target directory, and return the produced path with its bytes.
///
/// The path still comes from cargo's `compiler-artifact` message rather than
/// being spelled by hand, so a layout change surfaces as a missing message
/// instead of as a read of some other file.
fn build_ken(target_dir: &Path, features: Option<&str>) -> (PathBuf, Vec<u8>) {
    std::fs::create_dir_all(target_dir).expect("per-configuration target dir");
    // The cargo running this test, not whatever is on PATH. NOTE: deliberately
    // NOT `scripts/ken-cargo` -- that wrapper takes a machine-wide build lock
    // which the outer invocation already holds, so a nested call through it
    // would deadlock rather than build. A private `--target-dir` is what keeps
    // this clear of the outer build.
    let mut command = Command::new(env!("CARGO"));
    command
        .arg("build")
        .arg("--manifest-path")
        .arg(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"))
        .arg("--bin")
        .arg("ken")
        .arg("--target-dir")
        .arg(target_dir)
        .arg("--message-format")
        .arg("json-render-diagnostics");
    if let Some(features) = features {
        command.arg("--features").arg(features);
    }
    let output = command.output().expect("nested cargo build runs");
    assert!(
        output.status.success(),
        "nested build failed for features {features:?}:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Exactly one executable artifact for the `ken` bin. More than one would mean
    // the invocation produced several candidates and picking any of them would be
    // arbitrary; none means cargo did not report what it built, and the read
    // would fall back to guessing a path.
    let mut executables: Vec<PathBuf> = Vec::new();
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let Ok(message) = serde_json::from_str::<serde_json::Value>(line) else {
            continue;
        };
        if message.get("reason").and_then(|r| r.as_str()) != Some("compiler-artifact") {
            continue;
        }
        if message
            .pointer("/target/name")
            .and_then(|name| name.as_str())
            != Some("ken")
        {
            continue;
        }
        if let Some(executable) = message.get("executable").and_then(|e| e.as_str()) {
            executables.push(PathBuf::from(executable));
        }
    }
    assert_eq!(
        executables.len(),
        1,
        "expected cargo to report exactly one `ken` executable for features {features:?}, got \
         {executables:?}"
    );

    let path = executables.remove(0);
    let bytes = std::fs::read(&path)
        .unwrap_or_else(|error| panic!("artifact {} unreadable: {error}", path.display()));
    (path, bytes)
}

/// The artifact-level feature gate, and the proofs that its two readings are of
/// two different binaries.
#[test]
fn mrc_4a1_feature_gate_holds_at_the_artifact() {
    let target_dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("mrc4a1-feature-gate-target");

    let (off_path, off_bytes) = build_ken(&target_dir.join("off"), None);
    let (on_path, on_bytes) = build_ken(
        &target_dir.join("on"),
        Some("ken-runtime/px8-ds-test-support"),
    );

    // Distinct files by construction -- separate target directories -- so this
    // is the byte comparison doing the work, not a path coincidence.
    assert_ne!(
        off_bytes, on_bytes,
        "the feature-off and feature-on artifacts are byte-identical, so the comparison measures \
         nothing"
    );

    // The instrument works on both inputs. A zero below is then a fact about the
    // binary, not about the reader.
    for (label, bytes) in [("feature-off", &off_bytes), ("feature-on", &on_bytes)] {
        assert!(
            count_occurrences(bytes, ALWAYS_PRESENT) > 0,
            "positive control failed: {label} artifact does not contain {ALWAYS_PRESENT:?}, so \
             this reader cannot be trusted to find anything"
        );
    }

    for needle in TRANSPORT_STRINGS {
        let off = count_occurrences(&off_bytes, needle);
        let on = count_occurrences(&on_bytes, needle);
        println!("MRC-4A1-GATE\t{needle}\toff={off}\ton={on}");
        assert_eq!(
            off, 0,
            "feature-off artifact contains {needle:?}: the transport was compiled into the \
             default build, so the gate is on the item's behaviour in name only"
        );
        assert_eq!(
            on, 1,
            "feature-on artifact contains {needle:?} {on} times, expected exactly 1. Zero means \
             the feature did not reach the linked binary and the sibling suite's zero-row \
             readings would be uninterpretable; more than one means the mechanism was duplicated"
        );
    }

    println!("MRC-4A1-GATE-OFF-PATH\t{}", off_path.display());
    println!("MRC-4A1-GATE-ON-PATH\t{}", on_path.display());
}
