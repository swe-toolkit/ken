//! Per-source Strict resolution for native preparation (spec 33 §3.3).

use std::path::PathBuf;

use ken_elaborator::compiler_driver::{
    prepare_native_program_sources, CompilerDriverError, CompilerSource, NativeProgramBuildError,
};
use ken_elaborator::ElabError;

fn catalog() -> Vec<PathBuf> {
    vec![PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")]
}

fn source(name: &str, text: &str, roots: &[PathBuf]) -> CompilerSource {
    CompilerSource::new(name, text).with_roots(roots.to_vec())
}

// Native preparation uses a large worker stack for its checked-core emission;
// this provisions that baseline rather than masking an elaboration refusal.
fn on_native_stack(f: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .stack_size(256 * 1024 * 1024)
        .spawn(f)
        .expect("spawn native preparation worker")
        .join()
        .expect("native preparation worker panicked");
}

fn assert_unbound(result: Result<impl std::fmt::Debug, NativeProgramBuildError>, missing: &str) {
    let error = result.expect_err("later source must not inherit an import");
    match error {
        NativeProgramBuildError::Driver(CompilerDriverError::Elaboration(
            ElabError::UnboundName { name, .. },
        )) => assert_eq!(name, missing),
        other => panic!("expected UnboundName {missing}, got {other:?}"),
    }
}

const EXIT_MAIN: &str = "program capabilities FS APartial\n\
    fn main (_input : ProcessInput) (_caps : ProgramCaps APartial)\n\
    : HostIO APartial ExitCode = host_pure APartial ExitCode ExitCode.Success\n";

/// MEASURED: the second source cannot use host_pure imported only by the
/// first. Importing it in the second source makes the same program prepare.
/// CLAIMED: selective import bindings belong to the source that declares
/// them, not to the shared native preparation run. THE GAP: a distinct
/// Console/Exit pair and changed-roots refusal are covered below.
#[test]
fn ac1_second_source_requires_its_own_exit_import() {
    on_native_stack(|| {
        let roots = catalog();
        let dependency = source(
            "dependency.ken",
            "import Capability.Process.Exit (host_pure)\nconst dep : Bool = True\n",
            &roots,
        );
        let without = source("main.ken", EXIT_MAIN, &roots);
        assert!(
            !without
                .text
                .lines()
                .any(|line| line.starts_with("import Capability.Process.Exit")),
            "negative main must not already import Exit"
        );
        assert_unbound(
            prepare_native_program_sources(
                "msis_ac1_missing_exit",
                vec![dependency.clone(), without],
            ),
            "host_pure",
        );
        let own_import = EXIT_MAIN.replacen(
            "program capabilities FS APartial\n",
            "program capabilities FS APartial\nimport Capability.Process.Exit (host_pure)\n",
            1,
        );
        assert_ne!(
            own_import, EXIT_MAIN,
            "positive source must carry its own import"
        );
        let with = source("main.ken", &own_import, &roots);
        prepare_native_program_sources("msis_ac1_imported_exit", vec![dependency, with])
            .expect("main's own Exit import must prepare");
    });
}

/// MEASURED: with identical roots, Console imported only in the earlier
/// source does not supply print_line to a later main that itself imports
/// Process.Exit. Its Console-imported twin prepares. CLAIMED: the root set
/// is shared, but each source's import scope is not. THE GAP: the roots
/// mismatch is an independent refusal below, not another import failure.
#[test]
fn ac2_same_roots_do_not_share_console_import() {
    on_native_stack(|| {
        let roots = catalog();
        let dependency = source(
            "dependency.ken",
            "import Capability.Console.Text (print_line)\nconst dep : Bool = True\n",
            &roots,
        );
        let main = "program capabilities FS APartial\n\
            import Capability.Process.Exit (host_program)\n\
            proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)\n\
            : HostIO APartial ExitCode visits [Console] =\n\
            host_program APartial (print_line \"isolated\")\n";
        assert!(
            !main
                .lines()
                .any(|line| line.starts_with("import Capability.Console.Text")),
            "negative main must not already import Console"
        );
        assert_unbound(
            prepare_native_program_sources(
                "msis_ac2_missing_console",
                vec![dependency.clone(), source("main.ken", main, &roots)],
            ),
            "print_line",
        );
        let own_import = main.replacen(
            "import Capability.Process.Exit (host_program)\n",
            "import Capability.Process.Exit (host_program)\nimport Capability.Console.Text (print_line)\n",
            1,
        );
        assert_ne!(
            own_import, main,
            "positive source must carry its own import"
        );
        prepare_native_program_sources(
            "msis_ac2_imported_console",
            vec![dependency, source("main.ken", &own_import, &roots)],
        )
        .expect("main's own Console import must prepare");
    });
}

/// MEASURED: a second source with different roots refuses before it can
/// resolve imports. CLAIMED: one elaboration run retains exactly one root set;
/// changing roots cannot reuse a unit cached by module path alone.
#[test]
fn ac2_different_roots_refuse_before_second_source() {
    on_native_stack(|| {
        let roots = catalog();
        let second_roots = vec![roots[0].join("..")];
        assert_ne!(roots, second_roots);
        let error = prepare_native_program_sources(
            "msis_ac2_roots",
            vec![
                source("dependency.ken", "const dep : Bool = True\n", &roots),
                source("main.ken", EXIT_MAIN, &second_roots),
            ],
        )
        .expect_err("roots cannot change during one elaboration run");
        match error {
            NativeProgramBuildError::Driver(CompilerDriverError::Elaboration(
                ElabError::ParseError { msg, .. },
            )) => assert_eq!(
                msg,
                "catalog roots cannot change during one elaboration run"
            ),
            other => panic!("expected exact roots refusal, got {other:?}"),
        }
    });
}

/// MEASURED: two distinct sources declaring the same top-level name refuse
/// with DuplicateDefinition despite each having its own Strict import scope.
/// CLAIMED: resetting resolution scope does not erase the package's shared
/// duplicate-declaration guard.
#[test]
fn duplicate_top_level_declarations_still_refuse() {
    on_native_stack(|| {
        let roots = catalog();
        let declaration = "const base : Bool = True\n";
        let error = prepare_native_program_sources(
            "msis_duplicate",
            vec![
                source("first.ken", declaration, &roots),
                source("second.ken", declaration, &roots),
            ],
        )
        .expect_err("duplicate top-level base must refuse");
        match error {
            NativeProgramBuildError::Driver(CompilerDriverError::Elaboration(
                ElabError::DuplicateDefinition { name, .. },
            )) => assert_eq!(name, "base"),
            other => panic!("expected duplicate base, got {other:?}"),
        }
    });
}
