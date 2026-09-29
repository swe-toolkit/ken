//! F2/F3 acceptance tests (`docs/program/wp/F2F3-reducer-degrade.md`,
//! `spec/10-kernel/18a-primitive-registry.md §5`,
//! `conformance/surface/numbers/seed-numbers.md` AC3/AC4).
//!
//! F2: bare fixed-width `add/sub/mul_intN`/`add_uintN` must degrade
//! (`EvalVal::Neutral`) on overflow, never wrap — the runtime face of the
//! no-overflow obligation. The sanctioned modular class (`wrapping_*_intN`)
//! is the only path permitted to wrap.
//! F3: the legacy unregistered `add`/`sub`/`mul` (wrapping i64) arms are
//! retired — unregistered and unreduced.

use ken_elaborator::{ElabEnv, ElabError};
use ken_interp::eval::{prim_reduce, EvalVal};
use ken_kernel::{GlobalId, Term};

// ── AC3/AC4 — bare op degrades, `+%` on the SAME operands still wraps ──────

/// surface/numbers/bare-overflow-never-silently-wraps (soundness, hard-AC).
/// `(100 : Int8) + (100 : Int8)` sums to 200 in ℤ, out of `Int8` range
/// (max 127). Must NOT silently produce the wrapped value `-56`.
#[test]
fn ac4_bare_add_int8_overflow_degrades_not_wraps() {
    let a = EvalVal::Int(100);
    let b = EvalVal::Int(100);
    let result = prim_reduce("add_int8", &[a, b]);
    assert_eq!(
        result,
        EvalVal::Neutral,
        "bare add_int8 on overflowing operands must degrade to Neutral, never yield -56"
    );
}

/// The discriminating partner: `+%` (`wrapping_add_int8`) on the identical
/// overflowing operands still wraps to the modular value -56. If this test
/// and the one above were both green under a broken conversion (e.g. the
/// bare arm still secretly wrapping), the pair would fail to flip — this
/// pins the flip.
#[test]
fn ac4_wrapping_add_int8_same_operands_still_wraps() {
    let a = EvalVal::Int(100);
    let b = EvalVal::Int(100);
    let result = prim_reduce("wrapping_add_int8", &[a, b]);
    assert_eq!(
        result,
        EvalVal::Int(-56),
        "wrapping_add_int8 (+%) must still wrap on the same overflowing operands"
    );
}

/// Non-overflowing bare arithmetic is unaffected (non-regression on the
/// total case): the checked path computes the same value as before.
#[test]
fn bare_add_int8_no_overflow_still_computes() {
    let result = prim_reduce("add_int8", &[EvalVal::Int(10), EvalVal::Int(20)]);
    assert_eq!(result, EvalVal::Int(30));
}

/// Every in-scope bare fixed-width op/width degrades on its own overflow
/// boundary — completeness across the whole F2 arm set, not just Int8/add.
#[test]
fn ac3_all_bare_fixed_width_arms_degrade_on_overflow() {
    let cases: &[(&str, i64, i64)] = &[
        ("add_int8", 127, 1),
        ("sub_int8", -128, 1),
        ("mul_int8", 127, 2),
        ("add_int16", i16::MAX as i64, 1),
        ("sub_int16", i16::MIN as i64, 1),
        ("mul_int16", i16::MAX as i64, 2),
        ("add_int32", i32::MAX as i64, 1),
        ("sub_int32", i32::MIN as i64, 1),
        ("mul_int32", i32::MAX as i64, 2),
        ("add_int64", i64::MAX, 1),
        ("sub_int64", i64::MIN, 1),
        ("mul_int64", i64::MAX, 2),
        ("add_uint8", 255, 1),
        ("add_uint16", 65535, 1),
        ("add_uint32", (u32::MAX) as i64, 1),
    ];
    for (op, a, b) in cases {
        let result = prim_reduce(op, &[EvalVal::Int(*a), EvalVal::Int(*b)]);
        assert_eq!(
            result,
            EvalVal::Neutral,
            "{op}({a}, {b}) must degrade to Neutral on overflow, never wrap"
        );
    }
}

// ── F3 — legacy `add`/`sub`/`mul` retired: unregistered AND unreduced ──────

/// Legacy `add`/`sub`/`mul` retirement guard: after deletion,
/// `prim_reduce` no longer recognizes the legacy wrapping-i64 symbols —
/// they fall through to the generic stuck arm.
#[test]
fn f3_legacy_add_sub_mul_unreduced() {
    for op in ["add", "sub", "mul"] {
        let result = prim_reduce(op, &[EvalVal::Int(1), EvalVal::Int(2)]);
        assert_eq!(
            result,
            EvalVal::Neutral,
            "legacy '{op}' must be unreduced (stuck) post-retirement"
        );
    }
}

fn application_head_id(term: &Term) -> Option<GlobalId> {
    let mut cursor = term;
    loop {
        match cursor {
            Term::App(function, _) => cursor = function,
            Term::Const { id, .. } => return Some(*id),
            _ => return None,
        }
    }
}

fn elaborated_operation_head(
    env: &mut ElabEnv,
    declaration: &str,
) -> Result<Option<GlobalId>, ElabError> {
    let result = env.elaborate_decl_v1(declaration)?;
    let (_, body) = env
        .env
        .transparent_body(result.def_id)
        .expect("checked constant has a transparent body");
    Ok(application_head_id(&body))
}

fn legacy_name_is_refused_or_canonical(
    env: &mut ElabEnv,
    legacy: &str,
    canonical_id: GlobalId,
) -> bool {
    let declaration = format!("const legacy_probe_{legacy} = {legacy} 1 2");
    match elaborated_operation_head(env, &declaration) {
        Err(ElabError::UnboundName { name, .. })
        | Err(ElabError::UnresolvedCon { name, .. }) => name == legacy,
        Ok(head) => head == Some(canonical_id),
        Err(error) => panic!("{legacy} failed for an unrelated reason: {error:?}"),
    }
}

/// Replaces the source-text oracle in
/// `docs/program/issues/TEST-SOURCE-TEXT-ORACLE-RETIRE.md`, item 21. This
/// measures whether each surface name is refused or resolves to the canonical
/// non-legacy identity through real elaboration, not a registration scan.
///
/// Promise class: durable invariant. MEASURED: modern operations resolve to
/// their canonical IDs; legacy names are either refused or resolve to those
/// IDs. CLAIMED: no legacy name exposes a separate identity. THE GAP: this
/// measures the default prelude, not every possible imported namespace.
#[test]
fn f3_legacy_names_are_refused_or_use_canonical_int_operations() {
    let mut env = ElabEnv::new().expect("prelude init");
    for (legacy, canonical) in [("add", "add_int"), ("sub", "sub_int"), ("mul", "mul_int")] {
        let canonical_id = env.globals[canonical];
        let modern = format!("const canonical_probe_{legacy} = {canonical} 1 2");
        assert_eq!(
            elaborated_operation_head(&mut env, &modern).expect("canonical op elaborates"),
            Some(canonical_id),
            "{canonical} must resolve to its registered identity"
        );
        assert!(
            legacy_name_is_refused_or_canonical(&mut env, legacy, canonical_id),
            "{legacy} must be unbound or resolve to {canonical}'s identity"
        );
    }

    // Population control at the resolver's real global-identity input: an
    // opaque function under a legacy spelling is not the canonical operation.
    let mut wrong = ElabEnv::new().expect("prelude init");
    let int = Term::const_(wrong.globals["Int"], vec![]);
    let wrong_type = Term::pi(int.clone(), Term::pi(int.clone(), int));
    let wrong_id = wrong
        .declare_postulate_raw("WrongLegacyAdd", wrong_type)
        .expect("wrong legacy fixture admits");
    wrong.globals.insert("add".into(), wrong_id);
    assert!(wrong.env.trusted_base().contains(&wrong_id));
    let wrong_canonical_id = wrong.globals["add_int"];
    assert!(!legacy_name_is_refused_or_canonical(
        &mut wrong,
        "add",
        wrong_canonical_id,
    ));
}
