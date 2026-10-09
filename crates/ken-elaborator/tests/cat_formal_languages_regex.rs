//! Regular-language denotation, derivative matching and six checked laws.
//!
//! Contract: spec/50-stdlib/61-formal-languages.md §4 and
//! conformance/stdlib/formal-languages/seed-regex.md. Promise class: durable
//! invariants. Closed Bool words are independent fixtures, not derivative
//! syntax or a generated expression census.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_interp::eval::{eval, EvalStore, EvalVal};
use ken_kernel::Decl;

const REGEX: &str = "Algorithm.FormalLanguages.Regex";
const DERIVED: &str = "Data.Collections.Derived";
const CLASSES: &str = "Core.Classes.LawfulClasses";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn checked_bool(env: &ElabEnv, name: &str) -> bool {
    let id = env.globals[name];
    let Some(Decl::Transparent { body, .. }) = env.env.lookup(id) else {
        panic!("{name} must have a kernel-checked transparent body");
    };
    match eval(&[], body, &env.env, &mut EvalStore::new()) {
        EvalVal::Ctor { id, args, .. } if id == env.numeric_env.bool_true_id && args.is_empty() => {
            true
        }
        EvalVal::Ctor { id, args, .. }
            if id == env.numeric_env.bool_false_id && args.is_empty() =>
        {
            false
        }
        other => panic!("{name} must evaluate to a Bool constructor, got {other:?}"),
    }
}

/// MEASURED: exact trusted GlobalIds remain equal across the Regex load;
/// Regex owns its public carriers, operations and six transparent proofs;
/// constructors inhabit their checked families; fresh generic clients apply
/// all six public laws at the §4.3 types. CLAIMED: the exported laws are
/// checked and no local trust was added. THE GAP: generic proof clients pin
/// types and identities, not the finite-word interpreter results below.
#[test]
fn regex_public_laws_generic_clients_and_exact_trust_delta() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        DERIVED,
        CLASSES,
        "Core.Logic.And",
        "Core.Logic.Or",
        "Core.Logic.Transport",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} must roots-load: {error:?}"));
    }
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    let owned = env
        .elaborate_module_from_roots(&[root()], REGEX)
        .expect("fully proved Regex must roots-load");
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(after, before, "Regex must add no trusted identities");

    for name in [
        "Regex",
        "Split",
        "Pieces",
        "regex_lang",
        "nullable",
        "deriv",
        "regex_matches",
        "nullable_sound",
        "nullable_complete",
        "deriv_sound",
        "deriv_complete",
        "regex_matches_sound",
        "regex_matches_complete",
    ] {
        let id = env.globals[&format!("{REGEX}.{name}")];
        assert!(owned.contains(&id), "{name} must be Regex-owned");
        if !matches!(name, "Regex" | "Split" | "Pieces") {
            assert!(
                matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
                "{name} must have a checked transparent body"
            );
        }
    }
    for (family, constructors) in [
        ("Regex", &["Fail", "Eps", "Sym", "Alt", "Cat", "Star"][..]),
        ("Split", &["MkSplit"][..]),
        ("Pieces", &["MkPieces"][..]),
    ] {
        let id = env.globals[&format!("{REGEX}.{family}")];
        let checked = env
            .env
            .inductive(id)
            .expect("public carrier is checked inductive data");
        for constructor in constructors {
            let cid = env.globals[&format!("{REGEX}.{constructor}")];
            assert!(
                checked.constructors.iter().any(|item| item.id == cid),
                "{constructor} must belong to checked {family}"
            );
        }
    }
    for provider in [
        format!("{DERIVED}.list_concat"),
        format!("{DERIVED}.list_all"),
        format!("{CLASSES}.or_left"),
        format!("{CLASSES}.or_right"),
        format!("{CLASSES}.or_cases"),
        format!("{CLASSES}.and_true"),
        format!("{CLASSES}.and_cases"),
    ] {
        let id = env.globals[&provider];
        assert!(
            !owned.contains(&id),
            "{provider} must retain its own identity"
        );
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }

    env.elaborate_file(
        r#"import Algorithm.FormalLanguages.Regex
             (Regex, Fail, Eps, Sym, Alt, Cat, Star, Split, MkSplit,
              Pieces, MkPieces, regex_lang, nullable, deriv, regex_matches,
              nullable_sound, nullable_complete, deriv_sound, deriv_complete,
              regex_matches_sound, regex_matches_complete)
           import Core.Classes.LawfulClasses (DecEq)
           theorem client_nullable_sound (a : Type) (r : Regex a) :
             Equal Bool (nullable a r) True → regex_lang a r (Nil a) =
             nullable_sound a r
           theorem client_nullable_complete (a : Type) (r : Regex a) :
             regex_lang a r (Nil a) → Equal Bool (nullable a r) True =
             nullable_complete a r
           theorem client_deriv_sound
             (a : Type) (d : DecEq a) (x : a) (r : Regex a) :
             (w : List a) →
             regex_lang a (deriv a d x r) w → regex_lang a r (Cons a x w) =
             deriv_sound a d x r
           theorem client_deriv_complete
             (a : Type) (d : DecEq a) (x : a) (r : Regex a) :
             (w : List a) →
             regex_lang a r (Cons a x w) → regex_lang a (deriv a d x r) w =
             deriv_complete a d x r
           theorem client_matches_sound
             (a : Type) (d : DecEq a) (r : Regex a) (w : List a) :
             Equal Bool (regex_matches a d r w) True → regex_lang a r w =
             regex_matches_sound a d r w
           theorem client_matches_complete
             (a : Type) (d : DecEq a) (r : Regex a) (w : List a) :
             regex_lang a r w → Equal Bool (regex_matches a d r w) True =
             regex_matches_complete a d r w"#,
    )
    .expect("each of six public laws must inhabit its independent §4.3 client type");
    for private in [
        "guard",
        "list_head_or",
        "cat_split",
        "star_split",
        "matches_sound_from",
        "matches_complete_from",
    ] {
        match env.elaborate_file(&format!("import {REGEX} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{REGEX}.{private}"));
            }
            Err(other) => panic!("{private} rejected for unrelated reason: {other:?}"),
            Ok(_) => panic!("{private} must remain private"),
        }
    }
}

/// MEASURED: roots-loaded provider operations and shared Boolean lemmas have
/// checked bodies, selective clients call those exact published functions at
/// their general types. CLAIMED: the shared dependencies are usable through
/// their canonical modules, without Regex-private stand-ins. THE GAP: these
/// clients certify the exports; package proofs and seed behavior are separate.
#[test]
fn shared_list_and_bool_providers_are_selectively_usable() {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], REGEX)
        .expect("Regex and its canonical dependencies must roots-load");
    env.elaborate_file(
        r#"import Data.Collections.Derived (list_concat, list_all)
           import Core.Classes.LawfulClasses
             (bool_or, bool_and, or_left, or_right, or_cases, and_true, and_cases)
           theorem client_or_left (b : Bool) (c : Bool) :
             Equal Bool b True → Equal Bool (bool_or b c) True = or_left b c
           theorem client_or_right (b : Bool) (c : Bool) :
             Equal Bool c True → Equal Bool (bool_or b c) True = or_right b c
           theorem client_or_cases (b : Bool) (c : Bool) (g : Omega) :
             Equal Bool (bool_or b c) True →
             (Equal Bool b True → g) → (Equal Bool c True → g) → g =
             or_cases b c g
           theorem client_and_true (b : Bool) (c : Bool) :
             Equal Bool b True → Equal Bool c True →
             Equal Bool (bool_and b c) True = and_true b c
           theorem client_and_cases (b : Bool) (c : Bool) (g : Omega) :
             Equal Bool (bool_and b c) True →
             (Equal Bool b True → Equal Bool c True → g) → g =
             and_cases b c g
           fn client_concat (a : Type) (ws : List (List a)) : List a =
             list_concat a ws
           fn client_all (a : Type) (p : a → Omega) (xs : List a) : Omega =
             list_all a p xs"#,
    )
    .expect("seven shared providers must remain separately publishable");
}

/// MEASURED: the kernel checks §4's seven authored cases as closed
/// Equal Bool proofs reducing the real matcher with explicit DecEq Bool;
/// public Split/Pieces/And/Or evidence inhabits each positive denotation.
/// The interpreter also evaluates the base and dictionary controls. Both
/// directions of all three law pairs consume inhabited concrete premises.
/// CLAIMED: closed words and derivative branches implement the seed contract.
/// THE GAP: these finite fixtures supplement the six universal laws; the
/// interpreter cannot evaluate every recursive derivative to a Bool value.
#[test]
fn seven_regex_seed_cases_match_the_independent_evidence() {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], REGEX)
        .expect("Regex and all transitive providers must roots-load");
    env.elaborate_file(
        r#"import Algorithm.FormalLanguages.Regex
             (Regex, Fail, Eps, Sym, Alt, Cat, Star, Split, MkSplit,
              Pieces, MkPieces, regex_lang, nullable, deriv, regex_matches,
              nullable_sound, nullable_complete, deriv_sound, deriv_complete,
              regex_matches_sound, regex_matches_complete)
           import Data.Collections.Derived (list_append, list_concat, list_all)
           import Core.Logic.And (And, Both)
           import Core.Logic.Or (Or, Inl, Inr)
           import Core.Classes.LawfulClasses (DecEq)

           const z : List Bool = Nil Bool
           const t : List Bool = Cons Bool True z
           const f : List Bool = Cons Bool False z
           const tt : List Bool = Cons Bool True t
           const tf : List Bool = Cons Bool True f
           const ft : List Bool = Cons Bool False t
           const ff : List Bool = Cons Bool False f
           const st : Regex Bool = Sym Bool True
           const sf : Regex Bool = Sym Bool False
           const eps : Regex Bool = Eps Bool
           const alt : Regex Bool = Alt Bool st sf
           const right : Regex Bool = Alt Bool eps st
           const cat_right : Regex Bool = Cat Bool sf right
           const left : Regex Bool = Alt Bool eps sf
           const cat_left : Regex Bool = Cat Bool left st
           const star_fail : Regex Bool = Star Bool (Fail Bool)
           const star_true : Regex Bool = Star Bool st
           const star_empty : Regex Bool = Star Bool right

           theorem fail_z_impossible : regex_lang Bool (Fail Bool) z → Bottom =
             λbad. bad
           theorem fail_t_impossible : regex_lang Bool (Fail Bool) t → Bottom =
             λbad. bad
           theorem eps_z : regex_lang Bool eps z = Proved
           theorem eps_t_impossible : regex_lang Bool eps t → Bottom =
             λbad. absurd bad
           theorem eps_nullable_sound : regex_lang Bool eps z =
             nullable_sound Bool eps Proved
           theorem eps_nullable_complete : Equal Bool (nullable Bool eps) True =
             nullable_complete Bool eps eps_z

           const d_same : Bool = (DecEq_instance_Bool).eq True True
           const d_other : Bool = (DecEq_instance_Bool).eq False True
           theorem sym_t : regex_lang Bool st t =
             trunc_intro
               (Both (Equal Bool True True) (Equal (List Bool) z z) Proved Proved)
           theorem sym_f : regex_lang Bool sf f =
             trunc_intro
               (Both (Equal Bool False False) (Equal (List Bool) z z) Proved Proved)
           theorem deriv_sym_t : regex_lang Bool (deriv Bool DecEq_instance_Bool True st) z =
             Proved
           theorem deriv_sym_sound : regex_lang Bool st t =
             deriv_sound Bool DecEq_instance_Bool True st z deriv_sym_t
           theorem deriv_sym_complete :
             regex_lang Bool (deriv Bool DecEq_instance_Bool True st) z =
             deriv_complete Bool DecEq_instance_Bool True st z sym_t
           theorem deriv_sym_other_impossible :
             regex_lang Bool (deriv Bool DecEq_instance_Bool False st) z → Bottom =
             λbad. bad

           theorem alt_t : regex_lang Bool alt t =
             trunc_intro (Inl (regex_lang Bool st t) (regex_lang Bool sf t) sym_t)
           theorem alt_f : regex_lang Bool alt f =
             trunc_intro (Inr (regex_lang Bool st f) (regex_lang Bool sf f) sym_f)
           theorem right_z : regex_lang Bool right z =
             trunc_intro (Inl (regex_lang Bool eps z) (regex_lang Bool st z) eps_z)
           theorem right_t : regex_lang Bool right t =
             trunc_intro (Inr (regex_lang Bool eps t) (regex_lang Bool st t) sym_t)
           theorem right_nil : regex_lang Bool right (Nil Bool) =
             trunc_intro
               (Inl (regex_lang Bool eps (Nil Bool))
                 (regex_lang Bool st (Nil Bool)) Proved)
           theorem cat_f : regex_lang Bool cat_right f =
             trunc_intro
               (MkSplit Bool (regex_lang Bool sf) (regex_lang Bool right)
                 f f (Nil Bool) (list_append::right_unit Bool f)
                 sym_f right_nil)
           theorem cat_ft : regex_lang Bool cat_right (list_append Bool f t) =
             trunc_intro
               (MkSplit Bool (regex_lang Bool sf) (regex_lang Bool right)
                 (list_append Bool f t) f t Refl sym_f right_t)

           theorem left_z : regex_lang Bool left z =
             trunc_intro (Inl (regex_lang Bool eps z) (regex_lang Bool sf z) eps_z)
           theorem left_nil : regex_lang Bool left (Nil Bool) =
             trunc_intro
               (Inl (regex_lang Bool eps (Nil Bool))
                 (regex_lang Bool sf (Nil Bool)) Proved)
           theorem cat_left_t : regex_lang Bool cat_left t =
             trunc_intro
               (MkSplit Bool (regex_lang Bool left) (regex_lang Bool st)
                 t (Nil Bool) t (list_append::left_unit Bool t)
                 left_nil sym_t)
           theorem cat_left_deriv_complete :
             regex_lang Bool (deriv Bool DecEq_instance_Bool True cat_left) z =
             deriv_complete Bool DecEq_instance_Bool True cat_left z cat_left_t
           theorem cat_left_deriv_direct :
             regex_lang Bool (deriv Bool DecEq_instance_Bool True cat_left) z =
             trunc_intro
               (Inr
                 (regex_lang Bool (Cat Bool (deriv Bool DecEq_instance_Bool True left) st) z)
                 (regex_lang Bool (deriv Bool DecEq_instance_Bool True st) z)
                 deriv_sym_t)
           theorem cat_left_deriv_sound : regex_lang Bool cat_left t =
             deriv_sound Bool DecEq_instance_Bool True cat_left z cat_left_deriv_direct

           theorem star_fail_z : regex_lang Bool star_fail z =
             trunc_intro
               (MkPieces Bool (regex_lang Bool (Fail Bool)) z
                 (Nil (List Bool)) Proved Proved)
           theorem star_true_z : regex_lang Bool star_true z =
             trunc_intro
               (MkPieces Bool (regex_lang Bool st) z
                 (Nil (List Bool)) Proved Proved)
           const two_pieces : List (List Bool) =
             Cons (List Bool) t (Cons (List Bool) t (Nil (List Bool)))
           theorem two_pieces_all :
             list_all (List Bool) (regex_lang Bool st) two_pieces =
             trunc_intro
               (Both (regex_lang Bool st t)
                 (list_all (List Bool) (regex_lang Bool st)
                   (Cons (List Bool) t (Nil (List Bool))))
                 sym_t
                 (trunc_intro
                   (Both (regex_lang Bool st t)
                     (list_all (List Bool) (regex_lang Bool st) (Nil (List Bool)))
                     sym_t Proved)))
           theorem star_true_tt :
             regex_lang Bool star_true (list_concat Bool two_pieces) =
             trunc_intro
               (MkPieces Bool (regex_lang Bool st)
                 (list_concat Bool two_pieces) two_pieces Refl two_pieces_all)

           const empty_first : List (List Bool) =
             Cons (List Bool) z (Cons (List Bool) t (Nil (List Bool)))
           theorem empty_first_all :
             list_all (List Bool) (regex_lang Bool right) empty_first =
             trunc_intro
               (Both (regex_lang Bool right z)
                 (list_all (List Bool) (regex_lang Bool right)
                   (Cons (List Bool) t (Nil (List Bool))))
                 right_z
                 (trunc_intro
                   (Both (regex_lang Bool right t)
                     (list_all (List Bool) (regex_lang Bool right) (Nil (List Bool)))
                     right_t Proved)))
           theorem star_empty_t :
             regex_lang Bool star_empty (list_concat Bool empty_first) =
             trunc_intro
               (MkPieces Bool (regex_lang Bool right)
                 (list_concat Bool empty_first) empty_first Refl empty_first_all)
           theorem star_empty_matches_complete :
             Equal Bool
               (regex_matches Bool DecEq_instance_Bool star_empty
                 (list_concat Bool empty_first)) True =
             regex_matches_complete Bool DecEq_instance_Bool star_empty
               (list_concat Bool empty_first) star_empty_t
           theorem star_empty_matches_sound :
             regex_lang Bool star_empty (list_concat Bool empty_first) =
             regex_matches_sound Bool DecEq_instance_Bool star_empty
               (list_concat Bool empty_first) star_empty_matches_complete

           const fail_z : Bool = regex_matches Bool DecEq_instance_Bool (Fail Bool) z
           const fail_t : Bool = regex_matches Bool DecEq_instance_Bool (Fail Bool) t
           const eps_match_z : Bool = regex_matches Bool DecEq_instance_Bool eps z
           const eps_match_t : Bool = regex_matches Bool DecEq_instance_Bool eps t
           const fail_nullable : Bool = nullable Bool (Fail Bool)
           const eps_nullable : Bool = nullable Bool eps
           const st_z : Bool = regex_matches Bool DecEq_instance_Bool st z
           const st_t : Bool = regex_matches Bool DecEq_instance_Bool st t
           const st_f : Bool = regex_matches Bool DecEq_instance_Bool st f
           const st_tt : Bool = regex_matches Bool DecEq_instance_Bool st tt
           const same_deriv_nullable : Bool =
             nullable Bool (deriv Bool DecEq_instance_Bool True st)
           const other_deriv_nullable : Bool =
             nullable Bool (deriv Bool DecEq_instance_Bool False st)
           const alt_match_z : Bool = regex_matches Bool DecEq_instance_Bool alt z
           const alt_match_t : Bool = regex_matches Bool DecEq_instance_Bool alt t
           const alt_match_f : Bool = regex_matches Bool DecEq_instance_Bool alt f
           const alt_match_tt : Bool = regex_matches Bool DecEq_instance_Bool alt tt
           const alt_nullable : Bool = nullable Bool alt
           const cat_right_z : Bool = regex_matches Bool DecEq_instance_Bool cat_right z
           const cat_right_f : Bool = regex_matches Bool DecEq_instance_Bool cat_right f
           const cat_right_ft : Bool = regex_matches Bool DecEq_instance_Bool cat_right ft
           const cat_right_tf : Bool = regex_matches Bool DecEq_instance_Bool cat_right tf
           const cat_right_t : Bool = regex_matches Bool DecEq_instance_Bool cat_right t
           const cat_right_nullable : Bool = nullable Bool cat_right
           const cat_left_t_match : Bool = regex_matches Bool DecEq_instance_Bool cat_left t
           const cat_left_deriv_nullable : Bool =
             nullable Bool (deriv Bool DecEq_instance_Bool True cat_left)
           const star_fail_z_match : Bool = regex_matches Bool DecEq_instance_Bool star_fail z
           const star_fail_t_match : Bool = regex_matches Bool DecEq_instance_Bool star_fail t
           const star_fail_nullable : Bool = nullable Bool star_fail
           const star_true_z_match : Bool = regex_matches Bool DecEq_instance_Bool star_true z
           const star_true_tt_match : Bool = regex_matches Bool DecEq_instance_Bool star_true tt
           const star_true_pieces_match : Bool =
             regex_matches Bool DecEq_instance_Bool star_true (list_concat Bool two_pieces)
           const star_true_tf_match : Bool = regex_matches Bool DecEq_instance_Bool star_true tf
           const star_true_nullable : Bool = nullable Bool star_true
           const star_empty_t_match : Bool = regex_matches Bool DecEq_instance_Bool star_empty t
           const star_empty_pieces_match : Bool =
             regex_matches Bool DecEq_instance_Bool star_empty (list_concat Bool empty_first)"#,
    )
    .expect("all seven seed cases and independent public witnesses must elaborate");

    for (name, expected) in [
        // Fail and Eps on the empty word and a nonempty word.
        ("fail_z", false),
        ("fail_t", false),
        ("eps_match_z", true),
        ("eps_match_t", false),
        ("fail_nullable", false),
        ("eps_nullable", true),
        // Sym: the explicit dictionary distinguishes same and different symbols.
        ("d_same", true),
        ("d_other", false),
        ("st_z", false),
        ("st_t", true),
        ("st_f", false),
        ("st_tt", false),
        ("same_deriv_nullable", true),
        ("other_deriv_nullable", false),
        // Alt witnesses enter disjoint arms.
        ("alt_match_z", false),
        ("alt_match_t", true),
        ("alt_match_f", true),
        ("alt_match_tt", false),
        ("alt_nullable", false),
        // Cat witnesses preserve order and the empty right piece.
        ("cat_right_z", false),
        ("cat_right_f", true),
        ("cat_right_ft", true),
        ("cat_right_tf", false),
        ("cat_right_t", false),
        ("cat_right_nullable", false),
        // Empty-left Cat derivative must also consider the right operand.
        ("cat_left_t_match", true),
        ("cat_left_deriv_nullable", true),
        // Zero and multiple star pieces, plus a rejected wrong symbol.
        ("star_fail_z_match", true),
        ("star_fail_t_match", false),
        ("star_fail_nullable", true),
        ("star_true_z_match", true),
        ("star_true_tt_match", true),
        ("star_true_pieces_match", true),
        ("star_true_tf_match", false),
        ("star_true_nullable", true),
        // The leading empty piece is a real public witness, not matcher syntax.
        ("star_empty_t_match", true),
        ("star_empty_pieces_match", true),
    ] {
        // The elaborator checks `Proved` only after reducing the original
        // exported matcher at this exact closed input to the expected Bool.
        // The check is independent of ken-interp's partial evaluator.
        let truth = if expected { "True" } else { "False" };
        env.elaborate_file(&format!(
            "theorem regex_case_{name} : Equal Bool {name} {truth} = Proved"
        ))
        .unwrap_or_else(|error| {
            panic!("kernel-reduced seed case {name} must be {truth}: {error:?}")
        });
    }
    for (name, expected) in [
        ("fail_z", false),
        ("fail_t", false),
        ("eps_match_z", true),
        ("eps_match_t", false),
        ("fail_nullable", false),
        ("eps_nullable", true),
        ("d_same", true),
        ("d_other", false),
    ] {
        assert_eq!(checked_bool(&env, name), expected, "base fixture {name}");
    }
}
