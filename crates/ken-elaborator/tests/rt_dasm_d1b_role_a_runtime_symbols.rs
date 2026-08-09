//! `RT-DYNAMIC-ARM-SCALAR-MERGE` `D1b-role-a`, control 1.
//!
//! **What this pins.** On the *real generic package-emission path* — the one a
//! plain `ken` package takes, which builds no native entrypoint plan — the
//! produced `CheckedRuntimeSymbolsV1` record carries the package's **own**
//! `Nat` constructor identities, and those differ from the explicit legacy pair.
//!
//! **Why it must fire on that path specifically.** The defect this deliverable
//! repairs is that the role record was only ever produced inside the
//! process-starter transaction, so a generic package materialized none and
//! Runtime silently fell back to `legacy_prelude()` — whose `Nat` spells
//! `ctor:prelude::Nat::{Zero,Suc}` and therefore never matches a
//! package-qualified `Nat`. A control that produced the record by calling the
//! builder directly would pass while the generic path stayed broken.
//!
//! **The probe is unconditional, and that is deliberate.** An earlier
//! measurement on this node used a probe that only reported when it found
//! `Data` declarations; its zero could not be told apart from "this code was
//! never reached", and it produced a false conclusion that cost a re-cut. So
//! the first assertion here is that the record is *present at all* — a missing
//! record fails loudly rather than reading as a clean absence.

use ken_elaborator::checked_core::{StableSymbol, SymbolNamespace};
use ken_elaborator::compiler_driver::{
    checked_runtime_symbols_v1_key, compile_ken_package_sources, CompilerManifest,
    CompilerTargetKind, CompilerSource, TargetSelector,
};

/// A package that declares its **own** `Nat`, so its constructor identities are
/// package-qualified and cannot coincide with the prelude's.
const SOURCE: &str = r#"data Nat = Zero | Suc Nat

const two : Nat = Suc (Suc Zero)
"#;

const PACKAGE: &str = "d1b_role_a_pkg";

/// The pair `legacy_prelude()` carries. The record must NOT spell these.
const LEGACY_ZERO: &str = "ctor:prelude::Nat::Zero";
const LEGACY_SUC: &str = "ctor:prelude::Nat::Suc";

#[test]
fn d1b_role_a_generic_package_emission_produces_the_package_qualified_nat_pair() {
    let output = compile_ken_package_sources(
        &CompilerManifest::new(PACKAGE, Vec::new()),
        vec![CompilerSource::new("src/main.ken", SOURCE)],
        TargetSelector::StableSymbol {
            package_identity: StableSymbol::new(
                SymbolNamespace::Module,
                vec![PACKAGE.to_string()],
            ),
            symbol: StableSymbol::new(
                SymbolNamespace::Declaration,
                vec![PACKAGE.to_string(), "two".to_string()],
            ),
            kind: CompilerTargetKind::Executable,
        },
    )
    .expect("the generic package path compiles this source");

    let metadata = &output.package.artifact.semantic.metadata;

    // UNCONDITIONAL: the record must EXIST on this path. A missing key is the
    // exact defect under repair, and it must fail here rather than fall through
    // to a vacuously-satisfied comparison below.
    let record = metadata.get(&checked_runtime_symbols_v1_key()).unwrap_or_else(|| {
        panic!(
            "no CheckedRuntimeSymbolsV1 in the generic package's semantic metadata -- the record \
             is not being produced on the path a plain package takes, which is the defect this \
             deliverable repairs. Keys present: {:?}",
            metadata.keys().collect::<Vec<_>>()
        )
    });

    let text = String::from_utf8_lossy(record);

    // POSITIVE CONTROL ON THE INSTRUMENT: a marker every record carries. Without
    // it, "the legacy spellings are absent" is equally consistent with a correct
    // record and with a reader that cannot see anything at all.
    assert!(
        text.contains("CheckedRuntimeSymbolsV1"),
        "the stored bytes do not carry the version header, so nothing below is reading the record"
    );

    let expected_zero = format!("ctor:{PACKAGE}::Nat::Zero");
    let expected_suc = format!("ctor:{PACKAGE}::Nat::Suc");

    assert!(
        text.contains(&expected_zero),
        "the record does not carry the package's own Nat::Zero ({expected_zero})"
    );
    assert!(
        text.contains(&expected_suc),
        "the record does not carry the package's own Nat::Suc ({expected_suc})"
    );

    // DIFFERS FROM LEGACY. This is the half that makes the control
    // discriminating: a record that merely existed but still spelled the
    // prelude pair would leave the Peano fold missing exactly as before.
    assert!(
        !text.contains(LEGACY_ZERO),
        "the record spells the LEGACY {LEGACY_ZERO}; a package-qualified Nat would never match it, \
         so the transported authority would be inert"
    );
    assert!(
        !text.contains(LEGACY_SUC),
        "the record spells the LEGACY {LEGACY_SUC}; see above"
    );

    // The two halves of the population are both present: the six roles the
    // starter path takes from the entrypoint plan are resolved here through the
    // same table, so a generic package carries them too. A spine-only record
    // would reproduce this defect at the first of them.
    for role in ["MkProcessInput", "Nil", "Cons", "MkProd", "Success", "Failure"] {
        assert!(
            text.contains(role),
            "the record is missing the entry-plan role {role}; a spine-only record reproduces this \
             defect at the next special constructor"
        );
    }
}
