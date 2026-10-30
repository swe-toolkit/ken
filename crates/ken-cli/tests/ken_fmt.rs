use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn ken_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_ken"))
}

fn fixture(name: &str, contents: &str) -> PathBuf {
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(name);
    fs::write(&path, contents).unwrap();
    path
}

#[test]
fn fmt_rewrites_plain_and_literate_sources_through_landed_entry_points() {
    let plain = fixture(
        "fmt_rewrite.ken",
        "fn id (f : Nat -> Nat) (x : Nat) : Nat = f   (x)\nconst staged : Nat = let first = Zero in let second = first in second\n",
    );
    let literate = fixture(
        "fmt_rewrite.ken.md",
        "Prose -> unchanged.\n```ken\nfn id (f : Nat -> Nat) (x : Nat) : Nat = f   (x)\nconst staged : Nat = let first = Zero in let second = first in second\n```\n",
    );
    let output = Command::new(ken_bin())
        .arg("fmt")
        .arg(&plain)
        .arg(&literate)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        fs::read_to_string(plain).unwrap(),
        "fn id (f : Nat → Nat) (x : Nat) : Nat = f (x)\n\nconst staged : Nat = let first = Zero; second = first in second\n"
    );
    assert_eq!(
        fs::read_to_string(literate).unwrap(),
        "Prose -> unchanged.\n```ken\nfn id (f : Nat → Nat) (x : Nat) : Nat = f (x)\n\nconst staged : Nat = let first = Zero; second = first in second\n```\n"
    );
}

#[test]
fn fmt_check_names_every_offender_and_never_writes() {
    let first = fixture("fmt_check_first.ken", "const first   : Nat = Zero\n");
    let second = fixture("fmt_check_second.ken", "const second   : Nat = Zero\n");
    let before_first = fs::read(&first).unwrap();
    let before_second = fs::read(&second).unwrap();
    let output = Command::new(ken_bin())
        .arg("fmt")
        .arg("--check")
        .arg(&first)
        .arg(&second)
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains(first.to_str().unwrap()), "{stderr}");
    assert!(stderr.contains(second.to_str().unwrap()), "{stderr}");
    assert_eq!(fs::read(&first).unwrap(), before_first);
    assert_eq!(fs::read(&second).unwrap(), before_second);
}

#[test]
fn fmt_fails_loudly_for_unsupported_paths_and_must_parse_bodies() {
    let unsupported = fixture("fmt_unsupported.txt", "not Ken\n");
    let invalid = fixture(
        "fmt_invalid.ken.md",
        "```ken\nconst unfinished : Nat =\n```\n",
    );
    for (path, needle) in [
        (unsupported, "unsupported path"),
        (invalid, "formatting error"),
    ] {
        let output = Command::new(ken_bin())
            .arg("fmt")
            .arg(path)
            .output()
            .unwrap();
        assert!(!output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr).contains(needle));
    }
}

#[test]
fn strict_frozen_corpus_gate_is_green() {
    let repository = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut catalog = Vec::new();
    collect(&repository.join("catalog"), ".ken.md", &mut catalog);
    let mut rosetta = Vec::new();
    collect(&repository.join("examples/rosetta"), ".ken", &mut rosetta);
    catalog.sort();
    rosetta.sort();
    assert!(!catalog.is_empty(), "catalog corpus must not be empty");
    assert!(!rosetta.is_empty(), "Rosetta corpus must not be empty");

    let boundary =
        repository.join("catalog/packages/Tooling/Verification/ProofErasureBoundaryChecker.ken");
    assert!(boundary.is_file());
    let output = Command::new(ken_bin())
        .arg("fmt")
        .arg("--check")
        .args(&catalog)
        .arg(boundary)
        .args(&rosetta)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "frozen corpus is not canonical: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// Promise class: durable invariant. MEASURED: a Process-granted checked
/// program is admitted with its declared FS authority and rejected when that
/// authority is removed. CLAIMED: a Process grant does not replace the FS
/// authority needed by its checked ProgramCaps AFull. THE GAP: this exercises
/// one Process entrypoint and does not enumerate all runnable programs.
#[test]
fn every_checked_runnable_root_declares_its_fs_authority() {
    let repository = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut roots = Vec::new();
    for directory in ["catalog", "examples", "conformance", "crates"] {
        let directory = repository.join(directory);
        collect(&directory, ".ken", &mut roots);
        collect(&directory, ".ken.md", &mut roots);
    }
    roots.sort();
    roots.dedup();

    let mut runnable = Vec::new();
    for path in &roots {
        let source = fs::read_to_string(path).unwrap();
        if !source.contains("ProgramCaps") && !source.contains("HostIO") {
            continue;
        }
        runnable.push(path);
        assert!(
            source.lines().any(|line| {
                let Some(capabilities) = line.trim().strip_prefix("program capabilities ") else {
                    return false;
                };
                let mut fs_count = 0;
                for member in capabilities.split(',') {
                    let mut words = member.split_ascii_whitespace();
                    if words.next() == Some("FS") {
                        let authority = words.next();
                        if !matches!(authority, Some("ANone" | "APartial" | "AFull"))
                            || words.next().is_some()
                        {
                            return false;
                        }
                        fs_count += 1;
                    }
                }
                fs_count == 1
            }),
            "runnable Ken root lacks exactly one declared FS authority: {}",
            path.display()
        );
    }
    assert!(!runnable.is_empty(), "runnable corpus must not be empty");
}

#[test]
fn process_program_requires_its_declared_fs_authority() {
    let source = include_str!("fixtures/px10_child_wait_smoke.ken");
    let admitted = fixture("px10_process_with_fs.ken", source);
    let output = Command::new(ken_bin())
        .arg("run")
        .arg(&admitted)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let no_fs = source.replacen(
        "program capabilities FS AFull, Process",
        "program capabilities Process",
        1,
    );
    assert_ne!(
        no_fs, source,
        "control must remove the declared FS authority"
    );
    let rejected = fixture("px10_process_without_fs.ken", &no_fs);
    let output = Command::new(ken_bin())
        .arg("run")
        .arg(&rejected)
        .output()
        .unwrap();
    assert!(
        !output.status.success(),
        "removing required FS authority was admitted"
    );
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("MissingCapability { effect = FS }"),
        "the admission failure must name the missing FS grant: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn collect(directory: &Path, suffix: &str, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(directory).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            collect(&path, suffix, out);
        } else if path.to_string_lossy().ends_with(suffix) {
            out.push(path);
        }
    }
}
