//! Black-box identity observation at each actual CLI route.

use std::ffi::OsString;
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
        let mut option = OsString::from("--resolution-report=");
        option.push(path.as_os_str());
        cmd.arg(option);
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

#[cfg(unix)]
#[test]
fn non_utf8_report_path_is_preserved_by_all_three_route_parsers() {
    use std::os::unix::ffi::OsStringExt;

    let root = base("resolution-report-non-utf8-path");
    for route in ["check", "run", "native-build"] {
        let dir = root.join(route);
        std::fs::create_dir_all(&dir).unwrap();
        let source = dir.join("root.ken");
        std::fs::write(&source, ACCEPTED).unwrap();
        std::fs::write(dir.join("profile.json"), PROFILE).unwrap();
        let report = dir.join(OsString::from_vec(b"resolution-\xff.json".to_vec()));
        assert!(!report.exists());
        let output = invoke(route, &source, &dir, Some(&report));
        assert!(
            output.status.success(),
            "{route}: non-UTF-8 report path must be accepted: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let json: serde_json::Value = serde_json::from_slice(
            &std::fs::read(&report).expect("report created at the exact non-UTF-8 path"),
        )
        .unwrap();
        assert_eq!(json["schema"], "ken.resolution/v1");
        assert_eq!(json["route"], route);
    }
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

/// Promise class: durable invariant. A single Axiom result is the isolated
/// discriminator for the root-postulate ownership mutation. The independent
/// Proved/prove control below remains green when this ownership is removed.
#[test]
fn axiom_sugar_reports_its_own_checked_root_identity() {
    let dir = base("resolution-report-axiom-discriminator");
    let (doc, _) = document(
        &dir,
        "check",
        "theorem assumed : Equal Nat Zero (Suc Zero) = Axiom\n",
    );
    assert!(has(
        refs(&doc, "assumed"),
        "source-declared",
        "declaration",
        "assumed"
    ));
}

/// Promise class: durable invariant. Root-minted opaque identities in an
/// Axiom theorem, an axiom declaration and an ensures obligation belong to
/// their checked source result on every unchanged Legacy CLI route. A theorem
/// proved by Proved and a prove hole are controls against reclassifying all
/// postulates or changing their already-owned report bytes.
#[test]
fn root_postulates_have_checked_source_owners_on_all_three_routes() {
    let source = format!(
        "{ACCEPTED}\n\
         theorem assumed : Equal Nat Zero (Suc Zero) = Axiom\n\
         axiom assumed_named : Equal Nat Zero (Suc Zero)\n\
         fn constrained (n : Nat) : Nat ensures Equal Nat result Zero = n\n\
         theorem proved : Equal Nat Zero Zero = Proved\n\
         prove deferred : Equal Nat (Suc Zero) Zero\n"
    );
    let root = base("resolution-report-postulate-owners");
    for route in ["check", "run", "native-build"] {
        let dir = root.join(route);
        std::fs::create_dir_all(&dir).unwrap();
        let (doc, _) = document(&dir, route, &source);
        for declaration in ["assumed", "assumed_named", "constrained"] {
            assert!(
                has(
                    refs(&doc, declaration),
                    "source-declared",
                    "declaration",
                    declaration
                ),
                "{route}: root {declaration} must own its own checked opaque identity"
            );
        }
        let proved = refs(&doc, "proved");
        let deferred = refs(&doc, "deferred");
        assert!(!proved.iter().any(|r| r["class"] == "source-declared"));
        assert!(!deferred.iter().any(|r| r["class"] == "source-declared"));
        assert_eq!(
            serde_json::to_string(proved).unwrap(),
            r#"[{"class":"pre-source","name":"Equal"},{"class":"pre-source","name":"Nat"},{"class":"pre-source","name":"Proved"},{"class":"pre-source","name":"Zero"}]"#,
            "{route}: Proved control's already-established report bytes"
        );
        assert_eq!(
            serde_json::to_string(deferred).unwrap(),
            r#"[{"class":"pre-source","name":"Equal"},{"class":"pre-source","name":"Nat"},{"class":"pre-source","name":"Suc"},{"class":"pre-source","name":"Zero"}]"#,
            "{route}: prove control's already-established report bytes"
        );
    }
}

/// Promise class: durable invariant. This independent control has no Axiom
/// result; deleting the additional ownership attribution must leave it green.
#[test]
fn proved_and_prove_controls_keep_their_preexisting_report_bytes() {
    let dir = base("resolution-report-postulate-control");
    let source = "theorem proved : Equal Nat Zero Zero = Proved\n\
                  prove deferred : Equal Nat (Suc Zero) Zero\n";
    let (doc, text) = document(&dir, "check", source);
    assert_eq!(
        text,
        "{\"schema\":\"ken.resolution/v1\",\"route\":\"check\",\"module\":null,\"declarations\":[{\"declaration\":\"proved\",\"references\":[{\"class\":\"pre-source\",\"name\":\"Equal\"},{\"class\":\"pre-source\",\"name\":\"Nat\"},{\"class\":\"pre-source\",\"name\":\"Proved\"},{\"class\":\"pre-source\",\"name\":\"Zero\"}]},{\"declaration\":\"deferred\",\"references\":[{\"class\":\"pre-source\",\"name\":\"Equal\"},{\"class\":\"pre-source\",\"name\":\"Nat\"},{\"class\":\"pre-source\",\"name\":\"Suc\"},{\"class\":\"pre-source\",\"name\":\"Zero\"}]}]}\n"
    );
    assert_eq!(refs(&doc, "proved").len(), 4);
}

/// Promise class: durable invariant. Real catalog Axiom roots report through
/// ken check without treating checked package-owned holes as unattributed.
#[test]
fn three_catalog_axiom_roots_check_and_emit_complete_reports() {
    let dir = base("resolution-report-catalog-axioms");
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages");
    for (module, file, axiom) in [
        (
            "Core.Classes.LawfulClasses",
            "Core/Classes/LawfulClasses.ken.md",
            "ord_int_refl",
        ),
        (
            "Data.Binary.BytesPrimitiveContracts",
            "Data/Binary/BytesPrimitiveContracts.ken.md",
            "bytes_concat_list_view",
        ),
        (
            "Data.Text.StringBijection",
            "Data/Text/StringBijection.ken.md",
            "string_to_list_char_retraction",
        ),
    ] {
        let report = dir.join(format!("{module}.json"));
        let output = invoke("check", &root.join(file), &dir, Some(&report));
        assert!(
            output.status.success(),
            "{module}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let doc: serde_json::Value =
            serde_json::from_slice(&std::fs::read(report).unwrap()).unwrap();
        assert_eq!(doc["module"], module);
        assert!(
            has(refs(&doc, axiom), "source-declared", "declaration", axiom),
            "{module}: {axiom}"
        );
    }
}
