//! Strict homes for existing pre-source operations and the Decimal floor.
//! Normative identities: 30 §4, 35 §2.3 and §5, 38 §1.12.

use std::path::PathBuf;

use ken_elaborator::modules::{PRELUDE_FLOOR_NAMES, PRELUDE_PACKAGE_HOMES};
use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, Term};

fn strict(source: &str) -> Result<ElabEnv, ElabError> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages");
    let mut env = ElabEnv::new()?;
    env.elaborate_isolated_file_strict_v1(&[root], source)?;
    Ok(env)
}

fn unbound(source: &str, name: &str) {
    match strict(source) {
        Err(ElabError::UnboundName { name: missing, .. }) => assert_eq!(missing, name),
        Err(error) => panic!("{name}: wrong refusal: {error:?}"),
        Ok(_) => panic!("{name}: missing import was accepted"),
    }
}

/// Promise class: durable checked-identity invariant. MEASURED: every member
/// of the eight-width arithmetic family and both scalar/ABI families imports
/// from its one owner at the pre-source ID; removing that same import from an
/// otherwise identical Strict client refuses at the used name. CLAIMED: H
/// publishes old identities, not new implementations or ambient aliases.
/// THE GAP: this is source binding, not an execution or overflow oracle; the
/// CV's independent seed drives the arithmetic results.
#[test]
fn fixed_width_and_char_homes_use_old_ids_and_require_imports() {
    let bootstrap = ElabEnv::new().expect("checked pre-source IDs");
    let mut names = Vec::new();
    for width in ["Int8", "Int16", "Int32", "Int64", "UInt8", "UInt16", "UInt32", "UInt64"] {
        for operation in ["Add", "Sub", "Mul"] {
            for behavior in ["checked", "saturating"] {
                names.push(("Data.Numeric.FixedWidth", format!("{behavior}{operation}{width}")));
            }
        }
    }
    for name in ["RangeError", "MkRangeError", "intToUSize", "intToISize", "intToCInt"] {
        names.push(("Data.Numeric.FixedWidth", name.to_owned()));
    }
    for name in ["isScalar", "inRangeBool"] {
        names.push(("Core.Classes.LawfulClasses", name.to_owned()));
    }
    assert_eq!(names.len(), 55, "the ruled H operation inventory");
    for (owner, name) in names {
        let owners = PRELUDE_PACKAGE_HOMES.iter()
            .filter_map(|(home, member)| (*member == name).then_some(*home))
            .collect::<Vec<_>>();
        assert_eq!(owners, [owner], "{name}: one checked home");
        assert!(!PRELUDE_FLOOR_NAMES.contains(&name.as_str()), "{name}: not floor");
        let client = format!("export {name}\n");
        let imported = strict(&format!("import {owner} ({name})\n{client}"))
            .unwrap_or_else(|error| panic!("{name}: home refused: {error:?}"));
        assert_eq!(imported.globals[name.as_str()], bootstrap.globals[name.as_str()],
            "{name}: original checked identity");
        unbound(&client, &name);
    }
}

/// Promise class: durable checked-identity and privacy invariant. MEASURED:
/// ClockIO's declared type head is the pre-source checked alias; without its
/// import it refuses. Even after selecting ClockIO, spelling its ClockOp and
/// clock_resp representation refuses. CLAIMED: homing the alias does not
/// accidentally make its body or constructors writable. THE GAP: client
/// typing does not measure a clock's wall-time observation.
#[test]
fn clock_io_alias_homes_only_its_existing_checked_type() {
    let bootstrap = ElabEnv::new().expect("clock alias before selection");
    let owner = "Capability.Time.WallClock";
    let client = "const now_action : ClockIO Instant = wall_now\n";
    let source = format!("import {owner} (ClockIO, wall_now)\n{client}");
    let imported = strict(&source).expect("selected ClockIO and wall_now");
    let owners = PRELUDE_PACKAGE_HOMES.iter()
        .filter_map(|(home, member)| (*member == "ClockIO").then_some(*home))
        .collect::<Vec<_>>();
    assert_eq!(owners, [owner]);
    assert!(!PRELUDE_FLOOR_NAMES.contains(&"ClockIO"));
    assert_eq!(imported.globals["ClockIO"], bootstrap.globals["ClockIO"]);
    let Some(Decl::Transparent { ty, .. }) = imported.env.lookup(imported.globals["now_action"]) else {
        panic!("now_action must be a checked declaration")
    };
    assert!(matches!(ty, Term::App(head, _)
        if matches!(head.as_ref(), Term::Const { id, .. } if *id == bootstrap.globals["ClockIO"])),
        "declared type must keep the exact alias head: {ty:?}");
    unbound(&format!("import {owner} (wall_now)\n{client}"), "ClockIO");
    for representation in ["ClockOp", "clock_resp"] {
        unbound(&format!("{source}const escaped = {representation}\n"), representation);
    }
}

/// Promise class: durable representation-privacy invariant. MEASURED: a
/// Strict client imports every package whose export reaches FSOp in its
/// checked type, yet naming FSOp and AmbientOp independently still refuses
/// at each name. CLAIMED: a public HostIO surface does not grant its
/// representation. THE GAP: this pins the named public reachers, not an
/// unenumerated new provider; the checked-type census measures those.
#[test]
fn all_hostio_reachers_leave_fs_and_ambient_representation_private() {
    let imports = [
        "Capability.Process.Child", "Capability.Process.Exit",
        "Capability.Filesystem.Authority", "Capability.System.Buffer",
        "Capability.System.IO", "Capability.System.Mapping",
        "Capability.System.Resource", "Capability.Time.WallClock",
    ].into_iter().map(|owner| format!("import {owner}\n")).collect::<String>();
    let positive = format!("{imports}const writable : Type = HostIO APartial Unit\n");
    strict(&positive).expect("all public reachers load while HostIO remains writable");
    for representation in ["FSOp", "AmbientOp"] {
        unbound(&format!("{positive}const escaped = {representation}\n"), representation);
    }
}

fn binary_head(body: &Term) -> &Term {
    let Term::Lam(_, first) = body else { panic!("missing first argument") };
    let Term::Lam(_, second) = first.as_ref() else { panic!("missing second argument") };
    let Term::App(head, right) = second.as_ref() else { panic!("missing right operand") };
    assert_eq!(right.as_ref(), &Term::var(0), "right operand");
    let Term::App(head, left) = head.as_ref() else { panic!("missing left operand") };
    assert_eq!(left.as_ref(), &Term::var(1), "left operand");
    head.as_ref()
}

/// Promise class: durable exact-identity operator invariant. MEASURED: each
/// Decimal infix form lowers to precisely the original checked operation
/// applied to the two open parameters, and Decimal's floor type accepts a
/// literal without import; a same-named source type is refused. CLAIMED: the
/// B and R routes retain the original IDs and do not expose DecimalPair.
/// THE GAP: only the operator head is pinned here; value and overflow
/// behavior remain independent acceptance/seed observations.
#[test]
fn decimal_floor_and_all_four_infix_operations_keep_pre_source_ids() {
    let bootstrap = ElabEnv::new().expect("original Decimal operations");
    assert!(PRELUDE_FLOOR_NAMES.contains(&"Decimal"));
    let floor = strict("const c : Decimal = 2.0d\n").expect("Decimal literal without import");
    assert_eq!(floor.globals["Decimal"], bootstrap.numeric_env.decimal_id);
    let Some(Decl::Transparent { ty, .. }) = floor.env.lookup(floor.globals["c"]) else {
        panic!("literal must elaborate to a checked definition")
    };
    assert!(matches!(ty, Term::Const { id, .. } if *id == bootstrap.numeric_env.decimal_id));
    match strict("data Decimal = OwnDecimal\n") {
        Err(ElabError::AmbiguousReference { name, .. }) => assert_eq!(name, "Decimal"),
        Err(error) => panic!("source must not redeclare floor Decimal: {error:?}"),
        Ok(_) => panic!("source redeclared the floor Decimal"),
    }
    for (op, original, output) in [
        ("+", "decimalAdd", "Decimal"),
        ("-", "decimalSub", "Decimal"),
        ("*", "decimalMul", "Decimal"),
        ("==", "decimalEq", "Bool"),
    ] {
        let source = format!("fn candidate (x : Decimal) (y : Decimal) : {output} = x {op} y\n");
        let checked = strict(&source).unwrap_or_else(|error| panic!("Decimal {op}: {error:?}"));
        let Some(Decl::Transparent { body, .. }) = checked.env.lookup(checked.globals["candidate"]) else {
            panic!("Decimal {op} must be checked")
        };
        assert_eq!(binary_head(body), &Term::const_(bootstrap.globals[original], vec![]),
            "Decimal {op}: must directly call the original operator");
    }
}
