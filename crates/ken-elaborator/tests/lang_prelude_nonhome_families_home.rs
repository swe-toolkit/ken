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
        Err(ElabError::UnboundName { name: missing, span }) => {
            assert_eq!(missing, name);
            let start = source.find(name).expect("the missing spelling is in source");
            // `export name` reports the entire export clause as its span;
            // a declaration's unresolved use reports the name token itself.
            let expected_start = if source == format!("export {name}\n") { 0 } else { start };
            assert_eq!((span.start, span.end), (expected_start, start + name.len()),
                "refusal must land on the intended spelling or export clause");
        },
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
/// the stored now_action result and lift_clock action-binder type each have
/// a Const head at ClockIO's pre-source id; dropping only its import refuses
/// at ClockIO. Even after selection, ClockOp and clock_resp refuse. CLAIMED:
/// homing the alias leaves the representation private and makes typed clock
/// actions writable. THE GAP: checking does not observe wall time.
#[test]
fn clock_io_alias_homes_only_its_existing_checked_type() {
    let bootstrap = ElabEnv::new().expect("clock alias before selection");
    let owner = "Capability.Time.WallClock";
    let action = "proc now_action : ClockIO Instant visits [Clock] = wall_now\n";
    let source = format!("import {owner} (ClockIO, wall_now)\n{action}");
    let imported = strict(&source).expect("selected ClockIO and wall_now in an effectful proc");
    let owners = PRELUDE_PACKAGE_HOMES.iter()
        .filter_map(|(home, member)| (*member == "ClockIO").then_some(*home))
        .collect::<Vec<_>>();
    assert_eq!(owners, [owner]);
    assert!(!PRELUDE_FLOOR_NAMES.contains(&"ClockIO"));
    assert_eq!(imported.globals["ClockIO"], bootstrap.globals["ClockIO"]);
    let Some(Decl::Transparent { ty, .. }) = imported.env.lookup(imported.globals["now_action"]) else {
        panic!("now_action must be a checked declaration")
    };
    let clock_id = bootstrap.globals["ClockIO"];
    assert!(
        matches!(ty, Term::App(head, _)
        if matches!(head.as_ref(), Term::Const { id, .. } if *id == clock_id)),
        "now_action result must retain the pre-source ClockIO head; stored type: {ty:?}"
    );
    unbound(&format!("import {owner} (wall_now)\n{action}"), "ClockIO");
    for representation in ["ClockOp", "clock_resp"] {
        unbound(&format!("{source}const escaped = {representation}\n"), representation);
    }

    let lifted = "proc lift_clock (r : Type) (action : ClockIO r) \
                  : HostIO AFull r visits [Clock] = host_clock AFull r action\n";
    let lift_source = format!("import {owner} (ClockIO)\n\
        import Capability.Process.Exit (host_clock)\n{lifted}");
    let lifted_env = strict(&lift_source).expect("selected ClockIO in host_clock argument");
    assert_eq!(lifted_env.globals["ClockIO"], clock_id);
    let Some(Decl::Transparent { ty, .. }) = lifted_env.env.lookup(lifted_env.globals["lift_clock"]) else {
        panic!("lift_clock must be a checked declaration")
    };
    let Term::Pi(_, action_binder) = ty else { panic!("missing r binder: {ty:?}") };
    let Term::Pi(action_type, _) = action_binder.as_ref() else {
        panic!("missing action binder: {action_binder:?}")
    };
    assert!(matches!(action_type.as_ref(), Term::App(head, _)
        if matches!(head.as_ref(), Term::Const { id, .. } if *id == clock_id)),
        "lift_clock action binder must retain the pre-source ClockIO head; stored type: {action_type:?}");
    unbound(&format!("import Capability.Process.Exit (host_clock)\n{lifted}"), "ClockIO");
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

/// Promise class: durable Strict-boundary invariant. MEASURED: each ruled
/// internal pre-source name has a registered checked identity, yet a fresh
/// Strict client with no import refuses at that exact spelling. CLAIMED:
/// internal actions, representations and helpers do not leak through the
/// ambient source-name route. THE GAP: the refusal does not test imported
/// homes or the internal checked-ID consumers; those are separate gates.
#[test]
fn ruled_internal_prelude_names_are_registered_but_not_strictly_nameable() {
    let bootstrap = ElabEnv::new().expect("checked pre-source identities");
    for name in [
        // G1: entropy remains unavailable in the reference host.
        "EntropyIO",
        "random_bytes",
        "host_entropy",
        "ambient_entropy",
        "EntropyOp",
        "RandomBytes",
        "entropy_resp",
        // G2: monotonic clock operations remain unavailable.
        "MonotonicInstant",
        "MkMonotonicInstant",
        "Deadline",
        "MkDeadline",
        "monotonic_now",
        "sleep_until",
        "ClockOp",
        "WallNow",
        "MonotonicNow",
        "SleepUntil",
        "clock_resp",
        // G3: the ambient injection is checked representation, not source API.
        "ambient_console",
        "ambient_clock",
        "AmbientTailOp",
        "ambient_tail_resp",
        "AmbientOp",
        "ambient_resp",
        // G4: the source-facing exit route is host_pure, not host_exit.
        "host_exit",
        // G5: This bootstrap roster only lists retained global bindings.
        // Private directory constructors are removed even from bootstrap
        // names and exercised by the qualified-constructor privacy tests.
        "FSOp",
        "ReadFile",
        "WriteFile",
        "AppendFile",
        "Metadata",
        "CreateDirectory",
        "RemoveFile",
        "RemoveDirectory",
        "Rename",
        "ChangeMode",
        "fs_resp",
        // G6: Decimal's public operations are infix, not bare representation.
        "DecimalPair",
        "MkDecimalPair",
        "decimalPow10",
        "decimalAdd",
        "decimalSub",
        "decimalMul",
        "decimalEq",
        // G7: scalar internals do not inherit a catalog namesake's route.
        "IsTrue",
        "inLowerScalarBool",
        "inUpperScalarBool",
        // G8: these pre-source names have separate catalog identities or
        // explicit retirement/migration carries in I4.
        "zip",
        "is_sorted",
        "Perm",
        "And",
        "and_intro",
        "and_fst",
        "and_snd",
        "Not",
        "Empty",
        "Dec",
        "Yes",
        "No",
        "decide",
        // G9: the catalog owns separate checked buffer/transfer helpers.
        "buffer_nat_add",
        "buffer_nat_to_int",
        "buffer_window_start",
        "buffer_window_length",
        "buffer_span_start",
        "buffer_suc_cong",
        "transfer_count_request_budget",
        "write_all_call_bound",
        "write_all_complete",
        "write_all_first_error",
        "write_all_all_success",
        // G10: internal resource settlement and mapping representation.
        "ResourceTraceIdentity",
        "MappingExtent",
        "MkMappingExtent",
        "mapping_extent_length",
        "resource_settle_ok_error",
        "resource_settle_body_error",
        "resource_settle_ok",
        "resource_settle_err",
        "resource_settle_result",
    ] {
        assert!(
            bootstrap.globals.contains_key(name),
            "internal `{name}` must have a registered pre-source identity"
        );
        unbound(&format!("const escaped = {name}\n"), name);
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
