//! `RT-DYNAMIC-ARM-SCALAR-MERGE` `D1b-role-b`, control 2 — the role record is
//! decoded and validated at erasure, on the real `D5` package.
//!
//! **Three things asserted together, and only one of them is new.**
//!
//! 1. the executable declaration set is still exactly `liftAdd` and `liftSize`;
//! 2. `checked_core.data_metadata` still carries the exact Nat family and pair
//!    with their audit facts;
//! 3. the **typed** `checked_core.runtime_symbols` record decodes to exactly
//!    `nested_inductive_pkg::Nat::{Zero,Suc}` and differs from the legacy pair.
//!
//! ⚠ **Halves 1 and 2 pass today, before this slice exists**, because both
//! facts were already carried — the Steward measured that before confirming the
//! cut. They are here because the deliverable is *metadata preservation without
//! closure widening* and both halves are required to state it. But they carry
//! **no discriminating power for `D1b-role-b`**, and this file must not read as
//! though they did.
//!
//! ⇒ **All of this control's discriminating power is in half 3**, which is why
//! half 3 asserts **exact package-qualified identity** rather than presence. ⛔
//! A record that merely decoded, or a field that was merely `Some`, or a role
//! count that was merely right, would pass for reasons having nothing to do
//! with the change. The mutation that proves it: swap **only** Nat `Zero` to the
//! legacy symbol and the assertion must red.
//!
//! This is the same discrimination shape control 1 already required of the
//! producer, applied to the consumer — and it is the shape this node has twice
//! shipped without: something true, asserted where it could not have detected
//! being false.

use ken_elaborator::checked_core::{StableSymbol, SymbolNamespace};
use ken_elaborator::compiler_driver::{
    compile_ken_package_sources, CompilerManifest, CompilerSource, CompilerTargetKind,
    TargetSelector,
};
use ken_elaborator::erasure::erase_checked_core_package_for_target;
use ken_runtime::RuntimeProgram;

const PACKAGE: &str = "nested_inductive_pkg";

/// The real `D5` source, character-for-character as `nc14_data_match_lowering`
/// drives it. ⛔ Not a reduced stand-in: the control's whole point is that it
/// runs on the package whose Nat the native fold must eventually recognize.
const NESTED_LIFT_NAT_THREE_SOURCE: &str = "data Bag (a : Type) : Type where { \
      Empty : Bag a ; One : a -> Bag a ; Join : a -> a -> Bag a \
    }\n\
    data LiftRose = LiftLeaf | LiftNode (Bag LiftRose)\n\
    fn liftAdd (x : Nat) (y : Nat) : Nat = match x { \
      Zero |-> y ; Suc x2 |-> Suc (liftAdd x2 y) \
    }\n\
    fn liftSize (r : LiftRose) : Nat = match r { \
      LiftLeaf |-> Suc Zero ; \
      LiftNode b |-> match b { \
        Empty |-> Suc Zero ; \
        One x |-> Suc (liftSize x) ; \
        Join x y |-> Suc (liftAdd (liftSize x) (liftSize y)) \
      } \
    }\n\
    const liftSizeResult : Nat = liftSize \
      (LiftNode (Join LiftRose LiftLeaf (LiftNode (Empty LiftRose))))";

/// The pair `legacy_prelude()` carries. The decoded record must NOT spell these.
const LEGACY_ZERO: &str = "ctor:prelude::Nat::Zero";
const LEGACY_SUC: &str = "ctor:prelude::Nat::Suc";

fn decl_symbol(package: &str, name: &str) -> StableSymbol {
    StableSymbol::declaration(package, &[], name)
}

/// Erase the real `D5` package for its `liftSize` target.
fn erased_d5_program() -> RuntimeProgram {
    let target = decl_symbol(PACKAGE, "liftSize");
    let out = compile_ken_package_sources(
        &CompilerManifest::new(PACKAGE, Vec::new()),
        vec![CompilerSource::new(
            "src/main.ken",
            NESTED_LIFT_NAT_THREE_SOURCE,
        )],
        TargetSelector::StableSymbol {
            package_identity: StableSymbol::new(
                SymbolNamespace::Module,
                vec![PACKAGE.to_string()],
            ),
            symbol: target,
            kind: CompilerTargetKind::Executable,
        },
    )
    .expect("the real D5 source emits a checked-core package");
    let closure = out.closures.first().expect("selected target closure");
    erase_checked_core_package_for_target(&out.package, closure.reachable_declarations.iter())
        .expect("the D5 package erases, and its role record decodes and validates")
}

#[test]
fn d1b_role_b_erasure_carries_the_exact_nat_role_identity_without_widening_the_closure() {
    let program = erased_d5_program();

    // ── HALF 1: the executable closure is unchanged ──────────────────────────
    // Pre-existing behaviour. Present because "metadata preservation WITHOUT
    // closure widening" needs both halves stated; it discriminates nothing here.
    let executable: Vec<&str> = program
        .declarations
        .iter()
        .map(|declaration| declaration.symbol.as_str())
        .collect();
    let expected_add = decl_symbol(PACKAGE, "liftAdd").to_string();
    let expected_size = decl_symbol(PACKAGE, "liftSize").to_string();
    let mut sorted = executable.clone();
    sorted.sort_unstable();
    let mut expected = vec![expected_add.as_str(), expected_size.as_str()];
    expected.sort_unstable();
    assert_eq!(
        sorted, expected,
        "the executable declaration set must remain exactly liftAdd and liftSize -- carrying the \
         role record must not add Data declarations to the executable closure"
    );

    // ── HALF 2: checked-core data metadata still carries the Nat facts ───────
    // Also pre-existing. The Steward measured all of it as already carried at
    // dba42b0a, which is precisely why it cannot discriminate this slice.
    let nat_family = format!("decl:{PACKAGE}::Nat");
    let data = program
        .erased_core.metadata
        .checked_core
        .data_metadata
        .get(&nat_family)
        .unwrap_or_else(|| {
            panic!(
                "checked-core data metadata lost the Nat family {nat_family}; present: {:?}",
                program
                    .erased_core.metadata
                    .checked_core
                    .data_metadata
                    .keys()
                    .collect::<Vec<_>>()
            )
        });
    let zero_symbol = format!("ctor:{PACKAGE}::Nat::Zero");
    let suc_symbol = format!("ctor:{PACKAGE}::Nat::Suc");
    let zero = data
        .constructors
        .iter()
        .find(|ctor| ctor.symbol == zero_symbol)
        .expect("Nat::Zero is recorded in the data metadata");
    let suc = data
        .constructors
        .iter()
        .find(|ctor| ctor.symbol == suc_symbol)
        .expect("Nat::Suc is recorded in the data metadata");
    assert_eq!(zero.argument_count, 0, "Zero must stay nullary");
    assert_eq!(suc.argument_count, 1, "Suc must stay unary");
    assert_eq!(
        suc.recursive_positions,
        vec![0],
        "Suc must retain its recorded recursive position"
    );

    // ── HALF 3: the typed record, and the ONLY discriminating half ───────────
    let record = program
        .erased_core.metadata
        .checked_core
        .runtime_symbols
        .as_ref()
        .expect(
            "the real D5 package carries no decoded runtime-symbol record -- item 4 is not \
             running on the path a package-backed compile takes",
        );

    // EXACT IDENTITY, not presence. A decoded-but-legacy record would satisfy
    // every structural check above and still leave the native Peano fold unable
    // to match a package-qualified Nat, which is the whole defect D1b repairs.
    assert_eq!(
        record.spine.nat_zero, zero_symbol,
        "the decoded Nat zero role is {} but the package's own Nat::Zero is {zero_symbol}",
        record.spine.nat_zero
    );
    assert_eq!(
        record.spine.nat_suc, suc_symbol,
        "the decoded Nat successor role is {} but the package's own Nat::Suc is {suc_symbol}",
        record.spine.nat_suc
    );

    // DIFFERS FROM LEGACY on BOTH roles. Asserted separately from the equality
    // above so that a future package literally named `prelude` could not make
    // both assertions agree by coincidence.
    assert_ne!(
        record.spine.nat_zero, LEGACY_ZERO,
        "the decoded Nat zero role is the LEGACY symbol; a package-qualified Nat never matches it"
    );
    assert_ne!(
        record.spine.nat_suc, LEGACY_SUC,
        "the decoded Nat successor role is the LEGACY symbol; see above"
    );

    // The record is the package's throughout, not only in its Nat pair. A
    // record assembled from two different packages would pass the Nat halves.
    for (role, symbol) in record.roles() {
        assert!(
            symbol.contains(PACKAGE),
            "decoded role {role} is {symbol}, which is not qualified by this package -- the \
             record was not built from this package's stable-symbol table"
        );
    }
}

/// The decoded record's roles are validated against the package's own facts.
///
/// Separate from the identity control above because it pins a different
/// property: that erasure *checks* what it decoded, rather than copying bytes
/// into a typed field and calling that validation.
#[test]
fn d1b_role_b_every_decoded_role_resolves_in_the_packages_own_checked_facts() {
    let program = erased_d5_program();
    let checked = &program.erased_core.metadata.checked_core;
    let record = checked
        .runtime_symbols
        .as_ref()
        .expect("the D5 package carries a decoded runtime-symbol record");

    // Every constructor-namespace role the record carries must be findable in
    // exactly one recorded data family. This mirrors the validation erasure
    // performs, against the same metadata a consumer would read -- so if the
    // validation were removed, an unresolvable role would reach here.
    let mut constructor_roles = 0usize;
    for (role, symbol) in record.roles() {
        if !symbol.starts_with("ctor:") {
            continue;
        }
        constructor_roles += 1;
        let families: Vec<&String> = checked
            .data_metadata
            .iter()
            .filter(|(_, data)| data.constructors.iter().any(|ctor| &ctor.symbol == symbol))
            .map(|(family, _)| family)
            .collect();
        assert_eq!(
            families.len(),
            1,
            "decoded role {role} ({symbol}) resolves to {} families {families:?}; a role must \
             have exactly one constructor identity",
            families.len()
        );
    }

    // POSITIVE CONTROL ON THE LOOP. Without it, "every constructor role \
    // resolved" would also hold if the record carried no constructor roles at \
    // all, or if `roles()` returned nothing.
    assert!(
        constructor_roles > 0,
        "the record carried no constructor-namespace roles, so the loop above asserted nothing"
    );
}
