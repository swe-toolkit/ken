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

use ken_elaborator::checked_core::{semantic_fingerprint, CheckedCorePackage, StableSymbol, SymbolNamespace};
use ken_elaborator::compiler_driver::{
    checked_runtime_symbols_v1_key, compile_ken_package_sources, CompilerManifest,
    CompilerTargetKind, CompilerSource, TargetSelector,
};

/// A package whose `Nat` is the one the prelude `GlobalId`s denote — the same
/// situation as the real D5 package.
///
/// ⚠ **An earlier revision of this fixture declared its own `data Nat`, and that
/// was the wrong witness.** A shadowing declaration creates a *second* `Nat`, so
/// the prelude ids correctly denote the prelude's one and the record spells
/// that — the control went red against a producer that was right. Measured on
/// the real `nested_inductive_pkg` path, the produced pair is
/// `ctor:nested_inductive_pkg::Nat::{Zero,Suc}`: the ids' prelude **origin** and
/// their package-qualified **spelling** are different axes.
const SOURCE: &str = r#"const two : Nat = Suc (Suc Zero)
"#;

const PACKAGE: &str = "d1b_role_a_pkg";

/// The pair `legacy_prelude()` carries. The record must NOT spell these.
const LEGACY_ZERO: &str = "ctor:prelude::Nat::Zero";
const LEGACY_SUC: &str = "ctor:prelude::Nat::Suc";

/// The one real generic package emission both controls drive.
fn emit_package() -> CheckedCorePackage {
    compile_ken_package_sources(
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
    .expect("the generic package path compiles this source")
    .package
}

#[test]
fn d1b_role_a_generic_package_emission_produces_the_package_qualified_nat_pair() {
    let package = emit_package();
    let metadata = &package.artifact.semantic.metadata;

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

/// Control 1b — the record's bytes are covered by the semantic core hash.
///
/// **Why this exists as its own test.** The implementation stores the record in
/// `semantic.metadata` and the commit said that lane participates in
/// `core_semantic_hash`. That was a claim in prose with nothing executing it:
/// a later move to a non-hashed lane, or exclusion from the fingerprint, would
/// have left every other assertion in this file green. A mechanism claim in a
/// comment is exempt from execution, so it is pinned here instead.
///
/// **Established from the implementation before it was pinned:**
/// `core_semantic_hash` is `semantic_fingerprint(&artifact.semantic)`
/// (`checked_core.rs:1379`), and `canonical_semantic_bytes` encodes
/// `inputs.metadata` (`:1317`). The control below discriminates on that, rather
/// than being tuned until it agreed with the storage choice.
#[test]
fn d1b_role_a_the_record_is_covered_by_the_semantic_core_hash() {
    let baseline = emit_package();
    let repeat = emit_package();

    // STABILITY HALF. Without it, "the hash moved" is consistent with a hash
    // that moves on every compile, which would discriminate nothing.
    assert_eq!(
        baseline.core_semantic_hash, repeat.core_semantic_hash,
        "two identical generic package emissions must produce the same semantic core hash"
    );
    assert_eq!(
        baseline.core_semantic_hash,
        semantic_fingerprint(&baseline.artifact.semantic),
        "the package's recorded hash must be the fingerprint of its own semantic inputs, or the \
         mutation below is being applied to something the hash does not read"
    );

    let key = checked_runtime_symbols_v1_key();

    // MUTATION 1 — change the record's bytes. The hash must move.
    let mut mutated = baseline.clone();
    let record = mutated
        .artifact
        .semantic
        .metadata
        .get_mut(&key)
        .expect("the record is present to mutate");
    record.push(0xFF);
    assert_ne!(
        semantic_fingerprint(&mutated.artifact.semantic),
        baseline.core_semantic_hash,
        "mutating the CheckedRuntimeSymbolsV1 bytes did not change the semantic core hash -- the \
         record is NOT hash-covered, so a corrupted or substituted role table would be invisible"
    );

    // MUTATION 2 — remove the record entirely. The hash must move.
    let mut removed = baseline.clone();
    removed
        .artifact
        .semantic
        .metadata
        .remove(&key)
        .expect("the record is present to remove");
    assert_ne!(
        semantic_fingerprint(&removed.artifact.semantic),
        baseline.core_semantic_hash,
        "removing the record did not change the semantic core hash -- its presence is not covered"
    );

    // And the untouched clone still hashes to the baseline, so the two reds
    // above are attributable to the mutations rather than to cloning.
    assert_eq!(
        semantic_fingerprint(&baseline.artifact.semantic),
        baseline.core_semantic_hash,
        "the unmutated package must still hash to its own recorded value"
    );
}
