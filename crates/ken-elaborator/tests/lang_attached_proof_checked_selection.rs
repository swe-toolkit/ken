//! Promise class: durable Strict checked-identity selection invariants
//! (spec/30-surface/33-declarations.md §§3.1, 8.2).
//! Each refusal has an accepting neighbor; the subject resolves before proof lookup.

use std::fs;
use std::path::{Path, PathBuf};

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{GlobalId, Level, Term};

fn strict(env: &mut ElabEnv, roots: &[std::path::PathBuf], source: &str) -> Result<(), ElabError> {
    env.elaborate_isolated_file_strict_v1(roots, source)?;
    Ok(())
}

fn mentions(term: &Term, id: GlobalId) -> bool {
    match term {
        Term::Const { id: found, .. }
        | Term::IndFormer { id: found, .. }
        | Term::Constructor { id: found, .. }
            if *found == id =>
        {
            true
        }
        Term::Elim { fam, .. } if *fam == id => true,
        _ => term.children().into_iter().any(|child| mentions(child, id)),
    }
}

fn body_mentions(env: &ElabEnv, name: &str, id: GlobalId) {
    let (_, body) = env
        .env
        .transparent_body(env.globals[name])
        .expect("checked body");
    assert!(
        mentions(&body, id),
        "{name} must select checked proof {id:?}: {body:?}"
    );
}

fn catalog() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")
}

fn provider(root: &Path, name: &str) {
    fs::write(
        root.join(format!("{name}.ken")),
        "pub fn f (x : Int) : Int = x\n",
    )
    .expect("write a checked f provider");
}

/// MEASURED: both reference forms in a Strict inline child retain the
/// enclosing proof's checked ID. CLAIMED: the child inherits a prior proof
/// through the resolved subject. THE GAP: a later proof is tested separately
/// so this acceptance cannot silently become flat-global lookup.
#[test]
fn prior_enclosing_proof_selected_by_both_reference_forms_at_checked_id() {
    let mut env = ElabEnv::new().expect("base");
    strict(
        &mut env,
        &[],
        "fn f (x : Int) : Int = x\n\
         proof p for f (x : Int) : Equal Int (f x) x = Refl\n\
         module Child {\n\
           pub theorem by_atom (x : Int) : Equal Int (f x) x = f::p x\n\
           pub theorem by_proof (x : Int) : Equal Int (f x) x = (proof p for f) x\n\
         }\n",
    )
    .expect("both child selectors use the checked enclosing proof");
    let proof = env.globals["f::p"];
    body_mentions(&env, "Child.by_atom", proof);
    body_mentions(&env, "Child.by_proof", proof);
}

/// MEASURED: both forms of a proof declared after the child refuse
/// UnresolvedCon at its canonical spelling. CLAIMED: a snapshot cannot
/// expose a pending declaration as checked. THE GAP: the accepting prior
/// proof above proves that a legitimate earlier proof does cross the layer.
#[test]
fn later_enclosing_proof_refuses_as_pending_in_both_forms() {
    for reference in ["f::q x", "(proof q for f) x"] {
        let source = format!(
            "fn f (x : Int) : Int = x\n\
             module Child {{\n\
               theorem bad (x : Int) : Equal Int (f x) x = {reference}\n\
             }}\n\
             proof q for f (x : Int) : Equal Int (f x) x = Refl\n"
        );
        let mut env = ElabEnv::new().expect("base");
        match strict(&mut env, &[], &source) {
            Err(ElabError::UnresolvedCon { name, .. }) => assert_eq!(name, "f::q"),
            other => panic!("later {reference} must not select a proof: {other:?}"),
        }
    }
}

/// MEASURED: a child import of the same checked f finds its root proof q,
/// while a distinct provider's same-spelled f does not. CLAIMED: attached
/// selection is keyed by subject identity and proof name, not the route or
/// spelling. THE GAP: the provider files mint different checked identities.
#[test]
fn imported_same_subject_selects_root_proof_but_other_identity_refuses() {
    let tmp = tempfile::tempdir().expect("providers");
    provider(tmp.path(), "Provider");
    provider(tmp.path(), "Other");
    let roots = vec![tmp.path().to_path_buf()];
    let root = "import Provider (f)\n\
                proof q for f (x : Int) : Equal Int (f x) x = Refl\n";
    let mut same = ElabEnv::new().expect("base");
    strict(
        &mut same,
        &roots,
        &format!(
            "{root}module Child {{\n\
           import Provider (f)\n\
           pub theorem use_q (x : Int) : Equal Int (f x) x = f::q x\n\
         }}\n"
        ),
    )
    .expect("child's same provider identity selects enclosing proof");
    body_mentions(&same, "Child.use_q", same.globals["Provider.f::q"]);

    let mut different = ElabEnv::new().expect("base");
    let source = format!(
        "{root}module Child {{\n\
           import Other (f)\n\
           theorem bad (x : Int) : Equal Int (f x) x = f::q x\n\
         }}\n"
    );
    match strict(&mut different, &roots, &source) {
        Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, "Other.f::q"),
        other => panic!("another f identity must not select Provider's q: {other:?}"),
    }
}

/// MEASURED: shadowing f with a child local lacking p refuses both reference
/// forms; their exact diagnostics are recorded rather than equated. CLAIMED:
/// an enclosing proof never migrates to a different subject ID merely because
/// the child's subject has the same spelling. THE GAP: the positive preceding
/// child tests prove an unchanged identity still selects the proof.
#[test]
fn child_local_subject_without_proof_does_not_borrow_enclosing_proof() {
    for reference in ["f::p x", "(proof p for f) x"] {
        let source = format!(
            "fn f (x : Int) : Int = x\n\
             proof p for f (x : Int) : Equal Int (f x) x = Refl\n\
             module Child {{\n\
               fn f (x : Int) : Int = x\n\
               theorem bad (x : Int) : Equal Int (f x) x = {reference}\n\
             }}\n"
        );
        let mut env = ElabEnv::new().expect("base");
        let error = strict(&mut env, &[], &source).expect_err("different subject has no proof");
        match reference {
            "f::p x" => match error {
                ElabError::UnresolvedCon { name, .. } => assert_eq!(name, "Child.f::p"),
                other => panic!("child f::p diagnostic: {other:?}"),
            },
            _ => match error {
                ElabError::UnresolvedCon { name, .. } => assert_eq!(name, "Child.f::p"),
                other => panic!("child proof-ref diagnostic: {other:?}"),
            },
        }
    }
}

/// MEASURED: a middle scope's import of f blocks its descendant from the
/// root's f and idp, without granting the middle import to that descendant.
/// CLAIMED: subject lookup precedes proof lookup and import shadows stop the
/// lexical walk. THE GAP: a legitimate prior enclosing proof is selectable
/// when the middle layer does not shadow its subject (AC-1).
#[test]
fn middle_import_shadow_prevents_grandchild_attached_proof_selection() {
    let tmp = tempfile::tempdir().expect("provider");
    provider(tmp.path(), "Provider");
    let roots = vec![tmp.path().to_path_buf()];
    let source = "fn f (x : Int) : Int = x\n\
                  proof idp for f (x : Int) : Equal Int (f x) x = Refl\n\
                  module Mid {\n\
                    import Provider (f)\n\
                    module G {\n\
                      theorem bad (x : Int) : Equal Int x x = (proof idp for f) x\n\
                    }\n\
                  }\n";
    let mut env = ElabEnv::new().expect("base");
    assert!(
        matches!(
            strict(&mut env, &roots, source),
            Err(ElabError::UnboundName { .. })
        ),
        "subject f must refuse before idp is considered"
    );
}

/// MEASURED: the ES3 owner source checks under Strict with cong imported in
/// both using scopes; M's associativity proof names the checked root assoc,
/// its kind is unchanged, and the trusted base stays equal. CLAIMED: an
/// inherited attached proof crosses the inline boundary by checked identity.
/// THE GAP: the Legacy client block uses mixed routes, remains in the Legacy
/// original, and its wholly Strict restatement belongs to LANG-DELETE I4.
#[test]
fn strict_parameterized_abstract_export_preserves_kind_and_owner_proof() {
    let mut env = ElabEnv::new().expect("base env construction failed");
    let trusted_before = env.env.trusted_base();
    strict(
        &mut env,
        &[catalog()],
        r#"
        import Core.Logic.Transport (cong)
        fn list_append (a : Type) (xs : List a) (ys : List a) : List a =
          match xs {
            Nil |-> ys;
            Cons h rest |-> Cons a h (list_append a rest ys)
          }

        proof assoc for list_append
              (a : Type) (xs : List a) (ys : List a) (zs : List a)
            : Equal
                (List a)
                (list_append a (list_append a xs ys) zs)
                (list_append a xs (list_append a ys zs)) =
          match xs {
            Nil |-> Refl;
            Cons h rest |->
              cong
                (List a)
                (List a)
                (list_append a (list_append a rest ys) zs)
                (list_append a rest (list_append a ys zs))
                (Cons a h)
                ((proof assoc for list_append) a rest ys zs)
          }

        module M {
          import Core.Logic.Transport (cong)
          pub data NonEmpty a = NonEmptyCons a (List a)

          pub fn nonempty_singleton (a : Type) (x : a) : NonEmpty a =
            NonEmptyCons a x (Nil a)

          pub fn nonempty_head (a : Type) (xs : NonEmpty a) : a =
            match xs { NonEmptyCons x rest |-> x }

          pub fn nonempty_tail (a : Type) (xs : NonEmpty a) : List a =
            match xs { NonEmptyCons x rest |-> rest }

          pub fn nonempty_append
              (a : Type) (xs : NonEmpty a) (ys : NonEmpty a) : NonEmpty a =
            match xs {
              NonEmptyCons x rest |->
                match ys {
                  NonEmptyCons y more |->
                    NonEmptyCons a x (list_append a rest (Cons a y more))
                }
            }

          pub proof assoc for nonempty_append
                (a : Type) (xs : NonEmpty a) (ys : NonEmpty a) (zs : NonEmpty a)
              : Equal
                  (NonEmpty a)
                  (nonempty_append a (nonempty_append a xs ys) zs)
                  (nonempty_append a xs (nonempty_append a ys zs)) =
            match xs {
              NonEmptyCons x rest |->
                match ys {
                  NonEmptyCons y more |->
                    match zs {
                      NonEmptyCons z last |->
                        cong
                          (List a)
                          (NonEmpty a)
                          (list_append a (list_append a rest (Cons a y more)) (Cons a z last))
                          (list_append a rest (Cons a y (list_append a more (Cons a z last))))
                          (NonEmptyCons a x)
                          (list_append::assoc a rest (Cons a y more) (Cons a z last))
                    }
                }
            }
        }
        "#,
    )
    .expect("the exact NonEmpty owner definition and associativity proof elaborate");

    let root_assoc = env.globals["list_append::assoc"];
    assert!(
        env.globals.contains_key("M.nonempty_append::assoc"),
        "the Strict route must register M's attached proof by its owner"
    );
    body_mentions(&env, "M.nonempty_append::assoc", root_assoc);

    let nonempty_id = env.globals["M.NonEmpty"];
    let (_, exported_kind) = env
        .env
        .const_type(nonempty_id)
        .expect("parameterized abstract type has a kind");
    let expected_kind = Term::pi(Term::ty(Level::Zero), Term::ty(Level::Zero));
    assert_eq!(
        exported_kind, expected_kind,
        "the abstract export must retain the hand-written opaque Pi kind"
    );

    assert_eq!(
        env.env.trusted_base(),
        trusted_before,
        "the catalog import and owner declarations add no trust"
    );
}
