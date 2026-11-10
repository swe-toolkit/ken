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
    let assert_row = |row: RowType, plane: &str| {
        assert_eq!(
            row.concrete_effects().contains("Console"),
            expect_console,
            "{head} {args:?}: {plane} lost/gained Console: {row:?}",
        );
    };
    assert_row(type_carried_row_in(checked, &rows), "checked binder");
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
        assert_row(
            written_type_row(&written, &rows),
            if checked_identity {
                "written checked ID"
            } else {
                "written name"
            },
        );
    }
    // Every retained-position pin has the same independent phantom control.
    let tag = Term::app(
        Term::indformer(env.globals["Tag"], vec![]),
        Term::indformer(env.globals["Box"], vec![]),
    );
    assert_eq!(
        type_carried_row_in(tag, &rows),
        RowType::empty(),
        "Tag Box checked"
    );
    let tag_ty = RType::RApp(
        Box::new(RType::RCon("Tag".into(), Span::zero())),
        Box::new(RType::RCon("Box".into(), Span::zero())),
        Span::zero(),
    );
    assert_eq!(
        written_type_row(&tag_ty, &rows),
        RowType::empty(),
        "Tag Box written"
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
