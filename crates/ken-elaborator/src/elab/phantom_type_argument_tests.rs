//! AC-2: each retained argument position contributes its checked Console row
//! on both the checked binder and written annotation planes. Promise class:
//! durable invariant. MEASURED: both row computations on checked declarations;
//! CLAIMED: no held argument loses its row; GAP: these selected shapes cover
//! the named occurrence forms, not arbitrary future syntax or declarations.

use super::{type_carried_row_in, written_type_row, CheckedEffectRows};
use crate::effects::RowType;
use crate::error::Span;
use crate::resolve::RType;
use crate::ElabEnv;
use ken_kernel::{declare_postulate, Decl, Level, Term};

const PREFIX: &str = "data Box = MkBox (Unit ->[Console] HostIO APartial Unit)\n\
    data PBox = MkPBox Unit\n\
    data Tag (a : Type) : Type where { MkTag : PBox -> Tag a }\n";

// P1/P2: durable name-plane soundness invariants. MEASURED: the pre-source
// type-former spellings are charge-all, and merging a same-spelling phantom
// mask keeps that entry charge-all. CLAIMED: an RCon written annotation cannot
// borrow a different checked family's mask. GAP: neither file elaboration nor
// session elaboration admits a same-spelling Coproduct declaration here; P2
// exercises the registration merge, not an end-to-end source shadow.
#[test]
fn prelude_type_former_spellings_seed_charge_all_masks() {
    let env = ElabEnv::new().expect("complete prelude");
    for name in ["ITree", "Coproduct", "Pair"] {
        let id = env.globals[name];
        assert!(
            super::super::is_globals_type_former(&env.env, id),
            "{name} is a checked type former"
        );
        assert_eq!(
            env.type_charged_args.get(name),
            Some(&None),
            "{name}: unseeded spelling"
        );
    }
}

#[test]
fn seeded_coproduct_spelling_survives_phantom_name_merge() {
    let mut env = ElabEnv::new().expect("complete prelude");
    env.elaborate_file(PREFIX).expect("Box, PBox, Tag controls");
    // Same mask as `data Coproduct a b = MkC Unit`, supplied directly because
    // both file and session resolution refuse this shadow before registration.
    let joined = crate::modules::join_charged_name_mask(
        env.type_charged_args.get("Coproduct"),
        Some(vec![false, false]),
    );
    assert_eq!(
        env.type_charged_args.get("Coproduct"),
        Some(&None),
        "P2 pre-source entry"
    );
    assert_eq!(
        joined, None,
        "phantom collision lowered the pre-source mask"
    );
    env.type_charged_args.insert("Coproduct".into(), joined);
    let rows = CheckedEffectRows::new(
        &env.effect_rows,
        &env.effect_rows_by_id,
        &env.type_carried_rows,
        &env.type_carried_rows_by_id,
        &env.type_charged_args,
        &env.type_charged_args_by_id,
    );
    let written = RType::RApp(
        Box::new(RType::RApp(
            Box::new(RType::RCon("Coproduct".into(), Span::zero())),
            Box::new(RType::RCon("Box".into(), Span::zero())),
            Span::zero(),
        )),
        Box::new(RType::RCon("Unit".into(), Span::zero())),
        Span::zero(),
    );
    assert!(
        written_type_row(&written, &rows)
            .concrete_effects()
            .contains("Console"),
        "ambiguous prelude Coproduct annotation must retain Box's Console"
    );
}

fn check_position(decl: &str, head: &str, args: &[&str], expect_console: bool) {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(&format!("{PREFIX}{decl}"))
        .unwrap_or_else(|err| panic!("{head}: checked declaration required: {err:?}"));
    if head == "OpaqueHead" {
        // A checked opaque function head with no family declaration mask.
        // The row test is about applying its type, not its postulate body.
        let ty = Term::pi(Term::Type(Level::zero()), Term::Type(Level::zero()));
        let id = declare_postulate(&mut env.env, "OpaqueHead".into(), vec![], ty)
            .expect("well-formed checked opaque type constructor");
        env.globals.insert(head.into(), id);
    }
    let rows = CheckedEffectRows::new(
        &env.effect_rows,
        &env.effect_rows_by_id,
        &env.type_carried_rows,
        &env.type_carried_rows_by_id,
        &env.type_charged_args,
        &env.type_charged_args_by_id,
    );
    let head_id = *env.globals.get(head).expect("registered head");
    let is_family = matches!(env.env.lookup(head_id), Some(Decl::Inductive(_)));
    let checked_head = if is_family {
        Term::indformer(head_id, vec![])
    } else {
        Term::const_(head_id, vec![])
    };
    let checked = args.iter().fold(checked_head, |term, name| {
        let id = env.globals[*name];
        Term::app(term, Term::indformer(id, vec![]))
    });
    // Compute every observation before asserting any, so a mutation reports
    // retained-position and phantom-control results independently even when
    // both are housed in one test function.
    let mut observations = Vec::new();
    observations.push((
        "retained checked binder",
        type_carried_row_in(checked, &rows)
            .concrete_effects()
            .contains("Console"),
        expect_console,
    ));
    for checked_identity in [false, true] {
        let written_head = if checked_identity {
            RType::RCheckedGlobal {
                name: head.into(),
                id: head_id,
                span: Span::zero(),
            }
        } else {
            RType::RCon(head.into(), Span::zero())
        };
        let written = args.iter().fold(written_head, |ty, name| {
            RType::RApp(
                Box::new(ty),
                Box::new(RType::RCon((*name).into(), Span::zero())),
                Span::zero(),
            )
        });
        observations.push((
            if checked_identity {
                "retained written ID"
            } else {
                "retained written name"
            },
            written_type_row(&written, &rows)
                .concrete_effects()
                .contains("Console"),
            expect_console,
        ));
    }
    let tag = Term::app(
        Term::indformer(env.globals["Tag"], vec![]),
        Term::indformer(env.globals["Box"], vec![]),
    );
    observations.push((
        "phantom checked",
        type_carried_row_in(tag, &rows) == RowType::empty(),
        true,
    ));
    let tag_ty = RType::RApp(
        Box::new(RType::RCon("Tag".into(), Span::zero())),
        Box::new(RType::RCon("Box".into(), Span::zero())),
        Span::zero(),
    );
    observations.push((
        "phantom written",
        written_type_row(&tag_ty, &rows) == RowType::empty(),
        true,
    ));
    let deviations: Vec<_> = observations
        .iter()
        .copied()
        .filter(|(_, actual, expected)| actual != expected)
        .collect();
    assert!(
        deviations.is_empty(),
        "{head} {args:?}: {deviations:?}; all {observations:?}"
    );
}

#[test]
fn direct_field_keeps_parameter_charged() {
    check_position(
        "data Hold (a : Type) : Type where { MkHold : a -> Hold a }",
        "Hold",
        &["Box"],
        true,
    );
}

#[test]
fn nested_list_field_keeps_parameter_charged() {
    check_position(
        "data Nest (a : Type) : Type where { MkNest : List a -> Nest a }",
        "Nest",
        &["Box"],
        true,
    );
}

#[test]
fn arrow_domain_keeps_parameter_charged() {
    check_position(
        "data In (a : Type) : Type where { MkIn : (a -> Unit) -> In a }",
        "In",
        &["Box"],
        true,
    );
}

#[test]
fn arrow_codomain_keeps_parameter_charged() {
    check_position(
        "data Out (a : Type) : Type where { MkOut : (Unit -> a) -> Out a }",
        "Out",
        &["Box"],
        true,
    );
}

#[test]
fn field_of_phantom_family_still_keeps_outer_parameter_charged() {
    check_position(
        "data Wrap (a : Type) : Type where { MkWrap : Tag a -> Wrap a }",
        "Wrap",
        &["Box"],
        true,
    );
}

#[test]
fn index_argument_keeps_row_charged() {
    check_position(
        "data Indexed (a : Type) : Type -> Type where { MkIndexed : Indexed a Unit }",
        "Indexed",
        &["Unit", "Box"],
        true,
    );
}

#[test]
fn result_index_occurrence_keeps_parameter_charged() {
    check_position(
        "data Ix (a : Type) : Type -> Type where { MkIx : Ix a a }",
        "Ix",
        &["Box", "Unit"],
        true,
    );
}

#[test]
fn opaque_head_keeps_every_argument_charged() {
    check_position("", "OpaqueHead", &["Box"], true);
}

#[test]
fn alias_head_keeps_every_argument_charged() {
    check_position(
        "const AliasHead : Type -> Type = List",
        "AliasHead",
        &["Box"],
        true,
    );
}
