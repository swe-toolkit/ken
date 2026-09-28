//! RT-C5-PRIMITIVE-TYPE-ARGUMENTS: the interpreter is the C5 oracle; the
//! native backend currently refuses this indexed-family convoy before emission.

use ken_elaborator::checked_core::{CheckedCoreBodyViewError, StableSymbol, SymbolNamespace};
use ken_elaborator::compiler_driver::{CompilerDriverError, NativeProgramBuildError};

const INDEXED_INT_CONVOY: &str = r#"program capabilities FS APartial

data Vec (A : Type) : Nat -> Type where {
  VNil : Vec A 0;
  VCons : (n : Nat) -> A -> Vec A n -> Vec A (n+1)
}

fn zipPrimitive (n : Nat) (v : Vec Int n) (w : Vec Int n) : Vec Int n =
  match v {
    VNil |-> VNil Int;
    VCons m a xs |-> match w {
      VCons _ b ys |-> VCons Int m a (zipPrimitive m xs ys)
    }
  }

const checkResult : Bool =
  match zipPrimitive (Suc Zero)
    (VCons Int Zero 7 (VNil Int))
    (VCons Int Zero 8 (VNil Int)) {
    VCons _ head _ |-> eq_int head 7
  }

fn main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode =
  match checkResult {
    True |-> host_exit APartial Success;
    False |-> host_exit APartial (Failure 1)
  }
"#;

/// Promise class: transition sentinel. The WP that lifts the native
/// indexed-motive gate must replace this refusal pin with native Int AND
/// String convoy rows compared against `ken_cli::run_program` on the same
/// checked sources. This sentinel measures refusal, not native C5 success.
#[test]
fn indexed_int_convoy_native_refusal_waits_for_compared_native_rows() {
    let dir = tempfile::Builder::new()
        .prefix("ken-rt-c5-indexed-")
        .tempdir()
        .expect("isolated artifact directory");
    let err = ken_cli::build_native_program(
        INDEXED_INT_CONVOY,
        ken_cli::SourceFormat::Ken,
        "rt-c5-indexed",
        dir.path(),
        ken_runtime::boundary_resource_profile::starter_smoke_profile(),
    )
    .expect_err("an indexed Vec convoy must not silently reach a native artifact");
    match err {
        NativeProgramBuildError::Driver(CompilerDriverError::CheckedCoreBodyView {
            error: CheckedCoreBodyViewError::UnsupportedDependentMotive { symbol, family },
            ..
        }) => {
            assert_eq!(family, StableSymbol::declaration("rt-c5-indexed", &[], "Vec"));
            assert_eq!(symbol.namespace, SymbolNamespace::Declaration);
        }
        other => panic!("expected the named indexed-motive refusal, got {other:?}"),
    }
}
