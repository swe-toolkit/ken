//! LANG-PRELUDE-FLOOR-FIFTEEN: exact-id floor and compiler-internal Empty.

use std::fs;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{GlobalId, Term};

const COUNTER: &str = "space Counter { mut n : Int = 0 proc get () : Int visits [Counter] = n }";

fn mentions_global(term: &Term, target: GlobalId) -> bool {
    match term {
        Term::Const { id, .. } | Term::IndFormer { id, .. } | Term::Constructor { id, .. }
            if *id == target =>
        {
            true
        }
        Term::Elim { fam, .. } if *fam == target => true,
        _ => term
            .children()
            .iter()
            .any(|child| mentions_global(child, target)),
    }
}

fn apps(term: &Term) -> (&Term, Vec<&Term>) {
    let mut head = term;
    let mut arguments = Vec::new();
    while let Term::App(function, argument) = head {
        arguments.push(argument.as_ref());
        head = function.as_ref();
    }
    arguments.reverse();
    (head, arguments)
}

/// Return precisely the residual family in the checked operation type:
/// ITree (Coproduct (StateOp Counter) Empty) (resp_coproduct ...) Int.
fn checked_residual_id(env: &ElabEnv) -> GlobalId {
    let (_, ty) = env
        .env
        .const_type(env.globals["Counter.get"])
        .expect("checked space operation has a type");
    let (head, args) = apps(&ty);
    assert!(matches!(head, Term::IndFormer { id, .. } if *id == env.prelude_env.itree_id));
    assert_eq!(
        args.len(),
        3,
        "ITree has operation, response, and result arguments"
    );
    let (op_head, op_args) = apps(args[0]);
    assert!(matches!(op_head, Term::IndFormer { id, .. } if *id == env.prelude_env.coproduct_id));
    assert_eq!(
        op_args.len(),
        2,
        "Coproduct has a state arm and residual arm"
    );
    assert!(
        matches!(op_args[0], Term::App(..)),
        "first arm must carry the state"
    );
    match op_args[1] {
        Term::IndFormer { id, .. } => *id,
        other => panic!("residual arm must be an exact inductive identity, got {other:?}"),
    }
}

/// Promise class: durable invariant (30-taxonomy §4; 36 §4.2).
/// MEASURED: after a same-unit ill-kinded Empty declaration, the real space
/// operation checks. CLAIMED: desugaring uses the pre-source Empty identity.
/// THE GAP: the next two controls inspect the residual identity and absence
/// from the public globals map independently.
#[test]
fn ill_kinded_source_empty_cannot_redirect_space_desugaring() {
    let mut env = ElabEnv::new().expect("compiler prelude");
    let captured = env.prelude_env.empty_id;
    env.elaborate_file(&format!(
        "data Empty (a : Type) : Type where {{}}\n{COUNTER}"
    ))
    .expect("a source Empty cannot change the compiler's residual type");
    assert_ne!(
        env.globals["Empty"], captured,
        "source must allocate a new ID"
    );
    assert_eq!(checked_residual_id(&env), captured);
}

/// Promise class: durable invariant (30-taxonomy §4; 36 §4.2).
/// MEASURED: Counter.get's checked residual-position ID equals the pre-source
/// Empty ID, not the inhabited source family's ID. CLAIMED: the compiler's
/// operational identity cannot be redirected by a same-shaped source family.
/// THE GAP: the negative ID contrast requires a genuine distinct source ID.
#[test]
fn inhabited_source_empty_cannot_replace_checked_residual_identity() {
    let mut env = ElabEnv::new().expect("compiler prelude");
    let pre_source = env.prelude_env.empty_id;
    env.elaborate_file(&format!(
        "data Empty : Type where {{ Oops : Empty }}\n{COUNTER}"
    ))
    .expect("a source Empty and space must coexist");
    let source_empty = env.globals["Empty"];
    assert_ne!(
        pre_source, source_empty,
        "fixture must genuinely rebind Empty"
    );
    assert_eq!(checked_residual_id(&env), pre_source);
    assert_ne!(checked_residual_id(&env), source_empty);
}

/// Promise class: durable invariant (30-taxonomy §4; 36 §4.2).
/// MEASURED: removing the surface spelling from globals cannot prevent space
/// elaboration; its checked residual still holds the pre-source exact ID.
/// CLAIMED: source-visible globals are not the desugarer's identity authority.
/// THE GAP: constructor initialization must have captured the same ID before
/// the map entry is removed, not rebuilt one after the fact.
#[test]
fn space_residual_survives_missing_empty_global_spelling() {
    let mut env = ElabEnv::new().expect("compiler prelude");
    let pre_source = env.prelude_env.empty_id;
    assert_eq!(env.globals.remove("Empty"), Some(pre_source));
    env.elaborate_file(COUNTER)
        .expect("space cannot require a source-visible Empty spelling");
    assert_eq!(checked_residual_id(&env), pre_source);
    assert!(!env.globals.contains_key("Empty"));
}

/// Promise class: durable invariant (30-taxonomy §4; 33 §3.3).
/// MEASURED: each new floor name resolves from a strict file to the checked
/// pre-source declaration ID, while a nonmember And remains UnboundName.
/// CLAIMED: the five exact identities join the strict floor, not ambient lookup.
/// THE GAP: one checked occurrence per name does not cover all syntax forms;
/// this is a positive/negative boundary, not a complete binder migration.
#[test]
fn strict_five_proposition_members_resolve_canonically_and_and_does_not() {
    let root = tempfile::tempdir().expect("strict root");
    fs::write(
        root.path().join("Entry.ken"),
        "fn proposition (n : Nat) : Prop = Equal Nat n n\n\
         fn falsehood (n : Nat) : Prop = Bottom\n\
         theorem truth : Top = Proved",
    )
    .expect("write strict source");
    let mut env = ElabEnv::new().expect("compiler prelude");
    let ids = ["Bottom", "Equal", "Prop", "Proved", "Top"].map(|name| (name, env.globals[name]));
    let trust_before = env.env.trusted_base();
    env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "Entry")
        .expect("the five floor names must resolve in strict roots");
    assert_eq!(env.env.trusted_base(), trust_before);
    for (name, id) in ids {
        assert_eq!(
            env.globals[name], id,
            "{name} must preserve its pre-source ID"
        );
    }
    for (declaration, in_type, in_body) in [
        ("Entry.proposition", Some("Prop"), Some("Equal")),
        ("Entry.falsehood", Some("Prop"), Some("Bottom")),
        ("Entry.truth", Some("Top"), Some("Proved")),
    ] {
        let id = env.globals[declaration];
        let (_, ty) = env.env.const_type(id).expect("checked fixture type");
        let (_, body) = env.env.transparent_body(id).expect("checked fixture body");
        if let Some(name) = in_type {
            assert!(
                mentions_global(&ty, env.globals[name]),
                "{declaration} type must use exact {name} identity"
            );
        }
        if let Some(name) = in_body {
            assert!(
                mentions_global(&body, env.globals[name]),
                "{declaration} body must use exact {name} identity"
            );
        }
    }
    assert_eq!(env.globals["Proved"], env.env.tt_id());
    assert_eq!(env.globals["Top"], env.env.top_id());
    assert_eq!(env.globals["Bottom"], env.env.bottom_id());
    fs::write(
        root.path().join("NonFloor.ken"),
        "fn witness (n : Nat) : Prop = And Top Top",
    )
    .expect("write non-floor control");
    let error = ElabEnv::new()
        .expect("compiler prelude")
        .elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "NonFloor")
        .expect_err("And cannot become a strict floor member");
    assert!(
        matches!(error, ElabError::UnboundName { ref name, .. } if name == "And"),
        "wrong reason for non-floor rejection: {error:?}"
    );
}

/// Promise class: durable invariant (30-taxonomy §4; 33 §3.3).
/// MEASURED: a same-name checked source definition is rejected at prebinding
/// for each added floor member before allocating a new ID. CLAIMED: source
/// cannot replace an installed floor identity, regardless of its spelling.
/// THE GAP: caller mutation of the public map bypasses prebinding and is
/// separately checked by the strict-entry control below.
#[test]
fn five_floor_name_redeclarations_fail_before_allocation() {
    for (name, source) in [
        ("Bottom", "fn Bottom : Prop = Bottom"),
        ("Equal", "fn Equal (a : Type) (x : a) (y : a) : Prop = Top"),
        ("Prop", "fn Prop : Type = Nat"),
        ("Proved", "theorem Proved : Top = Proved"),
        ("Top", "fn Top : Prop = Top"),
    ] {
        let root = tempfile::tempdir().expect("strict root");
        fs::write(root.path().join("Entry.ken"), source).expect("write collision source");
        let mut env = ElabEnv::new().expect("compiler prelude");
        let old_id = env.globals[name];
        let next = env.env.next_global_id();
        let error = env
            .elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "Entry")
            .expect_err("a checked source declaration must not rebind a floor name");
        assert!(
            matches!(error,
            ElabError::AmbiguousReference { name: ref rejected, .. } if rejected == name),
            "{name}: wrong reason for source collision: {error:?}"
        );
        assert_eq!(env.globals[name], old_id, "{name}: pre-source ID changed");
        assert_eq!(
            env.env.next_global_id(),
            next,
            "{name}: allocation before rejection"
        );
    }
}

/// Promise class: durable invariant (30-taxonomy §4; 33 §3.3).
/// MEASURED: a checked, same-sort substitution in the live globals map
/// cannot redirect a strict unit's Top identity, even with no source binding.
/// CLAIMED: strict entry revalidates exact pre-source floor IDs before load.
/// THE GAP: the source-declaration route is covered by the previous test;
/// this control reaches the otherwise-bypassing public map mutation.
#[test]
fn strict_entry_rejects_forged_pre_source_floor_identity() {
    let root = tempfile::tempdir().expect("strict root");
    fs::write(
        root.path().join("Entry.ken"),
        "theorem witness : Top = Proved",
    )
    .expect("write floor consumer");
    let mut env = ElabEnv::new().expect("compiler prelude");
    let actual = env.globals["Top"];
    let forged = env.globals["Bottom"];
    assert_ne!(actual, forged);
    assert_eq!(env.globals.insert("Top".to_string(), forged), Some(actual));
    let error = env
        .elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "Entry")
        .expect_err("a strict unit cannot use substituted floor ID");
    match error {
        ElabError::Internal(message) => {
            assert!(message.contains("floor identity mismatch"), "{message}");
            assert!(message.contains("Top"), "{message}");
            assert!(message.contains(&format!("{actual:?}")), "{message}");
            assert!(message.contains(&format!("{forged:?}")), "{message}");
        }
        other => panic!("wrong strict-entry refusal: {other:?}"),
    }
}
