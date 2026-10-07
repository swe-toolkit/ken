use ken_elaborator::checked_core::{StableSymbol, SymbolNamespace};
use ken_elaborator::ElabEnv;
use ken_kernel::Decl;
use ken_elaborator::compiler_driver::{compile_ken_package_sources, CompilerManifest, CompilerSource, CompilerTargetKind, TargetSelector};

const PKG: &str = "verify_stable_identity_ac0";
const TARGET: &str = r#"program capabilities FS APartial
const ac0_need : String requires Equal Int 0 0 = "ac0-run"
const ac0_use : String = ac0_need
proc main
      (_input : ProcessInput) (_caps : ProgramCaps APartial)
    : HostIO APartial ExitCode
    visits [Console] =
  host_program APartial (print_line ac0_use)
"#;
const UNRELATED: &str = "const unrelated_stable_shift : Nat = Suc (Suc Zero)\n";

#[test]
fn ac0_obligation_identity_measurement_only() {
    let target = || CompilerSource::new("src/main.ken", TARGET);
    let extra = || CompilerSource::new("src/extra.ken", UNRELATED);
    let cases = [
        ("alone", vec![target()]),
        ("after", vec![extra(), target()]),
        ("before", vec![target(), extra()]),
    ];
    for (label, sources) in cases {
        let output = compile_ken_package_sources(
            &CompilerManifest::new(PKG, Vec::new()),
            sources,
            TargetSelector::StableSymbol {
                package_identity: StableSymbol::new(SymbolNamespace::Module, vec![PKG.to_string()]),
                symbol: StableSymbol::declaration(PKG, &[], "main"),
                kind: CompilerTargetKind::Executable,
            },
        ).unwrap_or_else(|e| panic!("{label}: {e:?}"));
        let sem = &output.package.artifact.semantic;
        eprintln!("AC0 {label} core_semantic_hash={:016x}", output.package.core_semantic_hash);
        eprintln!("AC0 {label} obligations={:?}", sem.obligations.keys().collect::<Vec<_>>());
        eprintln!("AC0 {label} origin={:?}", sem.obligation_metadata);
        eprintln!("AC0 {label} global symbols={:?}", sem.symbols.iter().filter(|s| s.to_string().contains("global_")).collect::<Vec<_>>());
        eprintln!("AC0 {label} delta={:?}", sem.trusted_base_delta);
        eprintln!("AC0 {label} report={:?}", output.report.obligations);
    }

    for (label, prefix) in [("alone", None), ("after", Some(UNRELATED))] {
        let mut elab = ElabEnv::new().expect("prelude environment");
        if let Some(prefix) = prefix {
            elab.elaborate_file_v1(prefix).expect("unrelated declaration");
        }
        let results = elab.elaborate_file_v1(TARGET).expect("diagnostic file elaboration");
        for result in results {
            eprintln!("AC0 elab {label} decl={} def_id={} obligations={:?}", result.name, result.def_id.0,
                result.obligations.iter().map(|o| (o.id, o.hole_id.0, &o.kind)).collect::<Vec<_>>());
        }
        let unnamed = elab.env.decls().filter(|d| !elab.globals.values().any(|i| *i == d.id())).map(|d| {
            let kind = match d {
                Decl::Transparent { .. } => "Transparent",
                Decl::Opaque { .. } => "Opaque",
                Decl::Primitive { .. } => "Primitive",
                Decl::Inductive(_) => "Inductive",
            };
            (d.id().0, kind.to_owned())
        }).collect::<Vec<_>>();
        eprintln!("AC0 unnamed {label} count={} entries={unnamed:?}", unnamed.len());
    }
}
