//! Black-box identity observation at each actual CLI route.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const ACCEPTED: &str = include_str!("fixtures/resolution_report/accepted.ken");
const SHADOW: &str = include_str!("fixtures/resolution_report/shadow.ken");
const PROFILE: &str = r#"{"runtime":{"invocation_epochs":18446744073709551615},"call_events":{"event_generations":18446744073709551615,"live_pending_slots":64},"invocation":{"nodes":64,"words":256,"data_bytes":512,"native_int_limbs":64},"persistent":{"nodes":64,"words":256,"data_bytes":512,"native_int_limbs":64}}"#;

fn base(label: &str) -> PathBuf {
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(label);
    let _ = std::fs::remove_dir_all(&path);
    std::fs::create_dir_all(&path).unwrap();
    path
}

fn invoke(route: &str, root: &Path, dir: &Path, report: Option<&Path>) -> Output {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_ken"));
    cmd.arg(route);
    if let Some(path) = report {
        cmd.arg(format!("--resolution-report={}", path.display()));
    }
    cmd.arg(root);
    if route == "native-build" {
        cmd.arg(dir.join("artifact")).arg(dir.join("profile.json"));
    }
    cmd.output().expect("invoke real ken CLI")
}

fn document(dir: &Path, route: &str, source: &str) -> (serde_json::Value, String) {
    let root = dir.join("root.ken");
    std::fs::write(&root, source).unwrap();
    std::fs::write(dir.join("profile.json"), PROFILE).unwrap();
    let report = dir.join("resolution.json");
    let output = invoke(route, &root, dir, Some(&report));
    assert!(
        output.status.success(),
        "{route}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = std::fs::read_to_string(report).expect("complete report");
    assert!(text.ends_with('\n') && !text[..text.len() - 1].contains('\n'));
    let json: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert_eq!(json["schema"], "ken.resolution/v1");
    assert_eq!(json["module"], serde_json::Value::Null);
    assert_eq!(json["route"], route);
    assert!(
        !text.contains("\"id\""),
        "no numeric checked IDs in the wire document"
    );
    (json, text)
}

fn refs<'a>(doc: &'a serde_json::Value, declaration: &str) -> &'a [serde_json::Value] {
    doc["declarations"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["declaration"] == declaration)
        .unwrap_or_else(|| panic!("missing {declaration}"))["references"]
        .as_array()
        .unwrap()
}

fn has(refs: &[serde_json::Value], class: &str, field: &str, name: &str) -> bool {
    refs.iter().any(|r| r["class"] == class && r[field] == name)
}

#[test]
fn three_routes_report_exact_checked_identity_and_shadow_without_touching_process_channels() {
    let root = base("resolution-report-routes");
    for (label, source) in [("accepted", ACCEPTED), ("shadow", SHADOW)] {
        let mut texts = Vec::new();
        for route in ["check", "run", "native-build"] {
            let dir = root.join(format!("{label}-{route}"));
            std::fs::create_dir_all(&dir).unwrap();
            let (json, text) = document(&dir, route, source);
            for (declaration, name) in [
                ("write_probe", "IO"),
                ("main", "print_line"),
                ("char_code_probe", "charToInt"),
                ("char_order_probe", "leqChar"),
            ] {
                assert!(
                    has(refs(&json, declaration), "pre-source", "name", name),
                    "{route} {label}: {declaration} must reference pre-source {name}"
                );
            }
            let write_refs = refs(&json, "write_probe");
            if label == "accepted" {
                assert!(
                    has(write_refs, "pre-source", "name", "write"),
                    "{route}: accepted write"
                );
                assert!(!has(write_refs, "source-declared", "declaration", "write"));
            } else {
                assert!(
                    has(write_refs, "source-declared", "declaration", "write"),
                    "{route}: the shadow must carry its checked source identity"
                );
                assert!(
                    !has(write_refs, "pre-source", "name", "write"),
                    "{route}: no all-pre-source or spelling-keyed classifier"
                );
            }
            // The flag changes no channel or exit status, not even native's
            // stdout artifact path or check's open-obligation stderr.
            let with_report = invoke(route, &dir.join("root.ken"), &dir, None);
            let with_flag = if route == "native-build" {
                // Re-run the flagged invocation at the same destination to
                // compare its actual stdout, stderr, and exit status.
                std::fs::remove_file(dir.join("resolution.json")).unwrap();
                invoke(
                    route,
                    &dir.join("root.ken"),
                    &dir,
                    Some(&dir.join("resolution.json")),
                )
            } else {
                // Running with a fresh path avoids create-new refusal.
                invoke(
                    route,
                    &dir.join("root.ken"),
                    &dir,
                    Some(&dir.join("extra.json")),
                )
            };
            assert_eq!(
                with_flag.status.code(),
                with_report.status.code(),
                "{label} {route} exit"
            );
            assert_eq!(
                with_flag.stdout, with_report.stdout,
                "{label} {route} stdout"
            );
            assert_eq!(
                with_flag.stderr, with_report.stderr,
                "{label} {route} stderr"
            );
            texts.push(text.replace(&format!("\"route\":\"{route}\""), "\"route\":\"check\""));
        }
        assert_eq!(texts[0], texts[1], "{label}: check/run route-axis parity");
        assert_eq!(
            texts[0], texts[2],
            "{label}: native-driver route-axis parity"
        );
    }
}

#[test]
fn catalog_literate_root_records_imports_and_only_its_checked_root_declarations() {
    let dir = base("resolution-report-catalog-literate");
    let packages = dir.join("catalog/packages");
    std::fs::create_dir_all(&packages).unwrap();
    std::fs::write(
        packages.join("Dependency.ken.md"),
        "# Dependency\n\n```ken\npub const value : Bool = True\n```\n",
    )
    .unwrap();
    let entry = packages.join("Entry.ken.md");
    std::fs::write(&entry,
        "# Entry\n\n```ken\nimport Dependency\nconst answer : Bool = Dependency.value\n```\n\n```ken example\nconst example : Bool = answer\n```\n").unwrap();
    let report = dir.join("resolution.json");
    let output = invoke("check", &entry, &dir, Some(&report));
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let json: serde_json::Value = serde_json::from_slice(&std::fs::read(report).unwrap()).unwrap();
    assert_eq!(json["module"], "Entry");
    let declarations = json["declarations"].as_array().unwrap();
    assert_eq!(
        declarations.len(),
        1,
        "neither imported nor example declarations belong to the root"
    );
    assert_eq!(declarations[0]["declaration"], "answer");
    assert!(has(
        refs(&json, "answer"),
        "imported",
        "module",
        "Dependency"
    ));
    assert!(refs(&json, "answer")
        .iter()
        .any(|r| r["class"] == "imported"
            && r["module"] == "Dependency"
            && r["declaration"] == "value"));
}

#[test]
fn standalone_literate_root_retains_three_route_identity_parity() {
    let root = base("resolution-report-literate-isolated");
    let mut texts = Vec::new();
    for route in ["check", "run", "native-build"] {
        let dir = root.join(route);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("root.ken.md");
        std::fs::write(&path, format!("# Runnable root\n\n```ken\n{ACCEPTED}```\n")).unwrap();
        std::fs::write(dir.join("profile.json"), PROFILE).unwrap();
        let report = dir.join("resolution.json");
        let output = invoke(route, &path, &dir, Some(&report));
        assert!(
            output.status.success(),
            "{route}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let text = std::fs::read_to_string(report).unwrap();
        let json: serde_json::Value = serde_json::from_str(&text).unwrap();
        assert!(has(
            refs(&json, "write_probe"),
            "pre-source",
            "name",
            "write"
        ));
        texts.push(text.replace(&format!("\"route\":\"{route}\""), "\"route\":\"check\""));
    }
    assert_eq!(texts[0], texts[1]);
    assert_eq!(texts[0], texts[2]);
}

#[test]
fn source_constructor_is_attributed_to_its_checked_family() {
    let dir = base("resolution-report-source-constructor");
    let (json, _) = document(
        &dir,
        "check",
        "data LocalChoice = LocalYes | LocalNo\nconst local : LocalChoice = LocalYes\n",
    );
    assert!(refs(&json, "local")
        .iter()
        .any(|r| r["class"] == "source-declared"
            && r["declaration"] == "LocalChoice"
            && r["constructor"] == "LocalYes"));
}

#[test]
fn report_create_new_refuses_existing_path_without_overwriting_it() {
    let dir = base("resolution-report-create");
    let root = dir.join("root.ken");
    std::fs::write(&root, ACCEPTED).unwrap();
    std::fs::write(dir.join("profile.json"), PROFILE).unwrap();
    let path = dir.join("existing.json");
    std::fs::write(&path, "untouched").unwrap();
    for route in ["check", "run", "native-build"] {
        let output = invoke(route, &root, &dir, Some(&path));
        assert_eq!(output.status.code(), Some(1));
        assert!(String::from_utf8_lossy(&output.stderr).contains("C0024"));
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "untouched");
    }
}

#[test]
fn report_is_unlinked_when_elaboration_fails() {
    let dir = base("resolution-report-unlink");
    let root = dir.join("invalid.ken");
    std::fs::write(&root, "const broken : Bool =").unwrap();
    std::fs::write(dir.join("profile.json"), PROFILE).unwrap();
    for route in ["check", "run", "native-build"] {
        let path = dir.join(format!("{route}.json"));
        let output = invoke(route, &root, &dir, Some(&path));
        assert_eq!(
            output.status.code(),
            Some(1),
            "{route}: expected elaboration refusal"
        );
        assert!(
            !path.exists(),
            "{route}: incomplete report must be unlinked"
        );
    }
}
