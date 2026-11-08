//! Option append-split selectors: spec/50-stdlib/72-optimization.md §§1.8, 3.8.
//! The laws lift the §§1.6, 3.6 non-empty append-split selectors.
//! Promise class: durable invariant. Open one-premise laws and closed branch
//! controls check the public behavior without replacing the conformance seed.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, KernelError};

const PACKAGE: &str = "Algorithm.Optimization.Argmin";
const LAWS: [&str; 4] = [
    "argmin_option_append_keeps_left",
    "argmin_option_append_takes_right",
    "argmax_option_append_keeps_left",
    "argmax_option_append_takes_right",
];

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_package() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("Argmin and its providers root-load");
    env
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.Optimization.Argmin
  (argmin, argmax, argmin_option, argmax_option,
   argmin_option_append_keeps_left, argmin_option_append_takes_right,
   argmax_option_append_keeps_left, argmax_option_append_takes_right)
import Data.Collections.Derived (list_append)
import Data.Collections.NonEmpty (nonempty_cons)
import Data.Numeric.Nat.Order (lt_nat)

theorem min_left (a : Type) (cost : a → Nat) (x : a) (xs : List a) (y : a) (ys : List a)
    (h : Equal Bool (lt_nat (cost (argmin a cost (nonempty_cons a y ys))) (cost (argmin a cost (nonempty_cons a x xs)))) False)
  : Equal (Option a) (argmin_option a cost (list_append a (Cons a x xs) (Cons a y ys))) (argmin_option a cost (Cons a x xs)) =
  argmin_option_append_keeps_left a cost x xs y ys h

theorem min_right (a : Type) (cost : a → Nat) (x : a) (xs : List a) (y : a) (ys : List a)
    (h : Equal Bool (lt_nat (cost (argmin a cost (nonempty_cons a y ys))) (cost (argmin a cost (nonempty_cons a x xs)))) True)
  : Equal (Option a) (argmin_option a cost (list_append a (Cons a x xs) (Cons a y ys))) (argmin_option a cost (Cons a y ys)) =
  argmin_option_append_takes_right a cost x xs y ys h

theorem max_left (a : Type) (cost : a → Nat) (x : a) (xs : List a) (y : a) (ys : List a)
    (h : Equal Bool (lt_nat (cost (argmax a cost (nonempty_cons a x xs))) (cost (argmax a cost (nonempty_cons a y ys)))) False)
  : Equal (Option a) (argmax_option a cost (list_append a (Cons a x xs) (Cons a y ys))) (argmax_option a cost (Cons a x xs)) =
  argmax_option_append_keeps_left a cost x xs y ys h

theorem max_right (a : Type) (cost : a → Nat) (x : a) (xs : List a) (y : a) (ys : List a)
    (h : Equal Bool (lt_nat (cost (argmax a cost (nonempty_cons a x xs))) (cost (argmax a cost (nonempty_cons a y ys)))) True)
  : Equal (Option a) (argmax_option a cost (list_append a (Cons a x xs) (Cons a y ys))) (argmax_option a cost (Cons a y ys)) =
  argmax_option_append_takes_right a cost x xs y ys h
"#;

/// MEASURED: the nine-provider closure's trusted GlobalId sets agree before
/// and after Argmin; all four new laws belong to Argmin with Transparent
/// checked bodies; a selective-import open client consumes each exact type.
/// CLAIMED: all four laws are public generic one-premise zero-new-trust proofs.
/// THE GAP: inherited provider trust remains; closed examples below cannot
/// establish the universal propositions on their own.
#[test]
fn option_append_split_laws_are_generic_public_and_add_no_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Classes.LawfulClasses",
        "Core.Function.Combinators",
        "Core.Logic.Or",
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Collections.List",
        "Data.Collections.Derived",
        "Data.Collections.NonEmpty",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} root-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("Option append-split proofs root-load");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "Option append-split trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "Option laws add no trusted GlobalId");
    for law in LAWS {
        let id = env.globals[&format!("{PACKAGE}.{law}")];
        assert!(owned.contains(&id), "Argmin owns {law}");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("selective-import client consumes all four exact generic laws");
}

const PREFIX: &str = r#"
import Algorithm.Optimization.Argmin
  (argmin_option, argmax_option,
   argmin_option_append_keeps_left, argmin_option_append_takes_right,
   argmax_option_append_keeps_left, argmax_option_append_takes_right)
import Data.Collections.Derived (list_append)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const four : Nat = Suc three
fn id_cost (n : Nat) : Nat = n
fn tied_cost (n : Nat) : Nat = one

const l31 : List Nat = Cons Nat three (Cons Nat one (Nil Nat))
const r24 : List Nat = Cons Nat two (Cons Nat four (Nil Nat))
const r14 : List Nat = Cons Nat one (Cons Nat four (Nil Nat))
const r04 : List Nat = Cons Nat Zero (Cons Nat four (Nil Nat))
const l13 : List Nat = Cons Nat one (Cons Nat three (Nil Nat))
const r20 : List Nat = Cons Nat two (Cons Nat Zero (Nil Nat))
const r30 : List Nat = Cons Nat three (Cons Nat Zero (Nil Nat))
const r40 : List Nat = Cons Nat four (Cons Nat Zero (Nil Nat))

theorem min_keeps_left :
  Equal (Option Nat) (argmin_option Nat id_cost (list_append Nat l31 r24)) (argmin_option Nat id_cost l31) =
  argmin_option_append_keeps_left Nat id_cost three (Cons Nat one (Nil Nat)) two (Cons Nat four (Nil Nat)) Proved

theorem min_keeps_tied_left :
  Equal (Option Nat) (argmin_option Nat tied_cost (list_append Nat l31 r24)) (argmin_option Nat tied_cost l31) =
  argmin_option_append_keeps_left Nat tied_cost three (Cons Nat one (Nil Nat)) two (Cons Nat four (Nil Nat)) Proved

theorem min_takes_right :
  Equal (Option Nat) (argmin_option Nat id_cost (list_append Nat l31 r04)) (argmin_option Nat id_cost r04) =
  argmin_option_append_takes_right Nat id_cost three (Cons Nat one (Nil Nat)) Zero (Cons Nat four (Nil Nat)) Proved

theorem max_keeps_left :
  Equal (Option Nat) (argmax_option Nat id_cost (list_append Nat l13 r20)) (argmax_option Nat id_cost l13) =
  argmax_option_append_keeps_left Nat id_cost one (Cons Nat three (Nil Nat)) two (Cons Nat Zero (Nil Nat)) Proved

theorem max_keeps_tied_left :
  Equal (Option Nat) (argmax_option Nat tied_cost (list_append Nat l13 r20)) (argmax_option Nat tied_cost l13) =
  argmax_option_append_keeps_left Nat tied_cost one (Cons Nat three (Nil Nat)) two (Cons Nat Zero (Nil Nat)) Proved

theorem max_takes_right :
  Equal (Option Nat) (argmax_option Nat id_cost (list_append Nat l13 r40)) (argmax_option Nat id_cost r40) =
  argmax_option_append_takes_right Nat id_cost one (Cons Nat three (Nil Nat)) four (Cons Nat Zero (Nil Nat)) Proved

theorem min_left_winner : Equal (Option Nat) (argmin_option Nat id_cost (list_append Nat l31 r24)) (Some Nat one) = Proved
theorem min_tied_left_winner : Equal (Option Nat) (argmin_option Nat tied_cost (list_append Nat l31 r24)) (Some Nat three) = Proved
theorem min_right_winner : Equal (Option Nat) (argmin_option Nat id_cost (list_append Nat l31 r04)) (Some Nat Zero) = Proved
theorem max_left_winner : Equal (Option Nat) (argmax_option Nat id_cost (list_append Nat l13 r20)) (Some Nat three) = Proved
theorem max_tied_left_winner : Equal (Option Nat) (argmax_option Nat tied_cost (list_append Nat l13 r20)) (Some Nat one) = Proved
theorem max_right_winner : Equal (Option Nat) (argmax_option Nat id_cost (list_append Nat l13 r40)) (Some Nat four) = Proved
"#;

const REFUSALS: [(&str, &str); 4] = [
    (
        "min_left_wrongly_keeps_left",
        "theorem min_left_wrongly_keeps_left : Equal (Option Nat) (argmin_option Nat id_cost (list_append Nat l31 r04)) (argmin_option Nat id_cost l31) = argmin_option_append_keeps_left Nat id_cost three (Cons Nat one (Nil Nat)) Zero (Cons Nat four (Nil Nat)) Proved",
    ),
    (
        "min_right_wrongly_takes_tie",
        "theorem min_right_wrongly_takes_tie : Equal (Option Nat) (argmin_option Nat id_cost (list_append Nat l31 r14)) (argmin_option Nat id_cost r14) = argmin_option_append_takes_right Nat id_cost three (Cons Nat one (Nil Nat)) one (Cons Nat four (Nil Nat)) Proved",
    ),
    (
        "max_left_wrongly_keeps_left",
        "theorem max_left_wrongly_keeps_left : Equal (Option Nat) (argmax_option Nat id_cost (list_append Nat l13 r40)) (argmax_option Nat id_cost l13) = argmax_option_append_keeps_left Nat id_cost one (Cons Nat three (Nil Nat)) four (Cons Nat Zero (Nil Nat)) Proved",
    ),
    (
        "max_right_wrongly_takes_tie",
        "theorem max_right_wrongly_takes_tie : Equal (Option Nat) (argmax_option Nat id_cost (list_append Nat l13 r30)) (argmax_option Nat id_cost r30) = argmax_option_append_takes_right Nat id_cost one (Cons Nat three (Nil Nat)) three (Cons Nat Zero (Nil Nat)) Proved",
    ),
];

/// MEASURED: six checked same-form law applications and independent reduced
/// winner equalities cover both directions and tied-left distinct labels.
/// A fresh fully accepted prefix precedes each of four false-premise law
/// applications, each rejected with kernel TypeMismatch at its own theorem.
/// CLAIMED: the split chooses the left part on a tie and the strictly better
/// right part otherwise; a false comparison proof cannot invoke the law.
/// THE GAP: these closed examples complement, rather than replace, the
/// generic public laws; they are not official conformance-runner execution.
#[test]
fn closed_option_split_choices_accept_and_wrong_premises_refuse() {
    let mut accepted = loaded_package();
    accepted
        .elaborate_file(PREFIX)
        .expect("all four laws, both tied-left labels, and six direct winners check");

    for (name, claim) in REFUSALS {
        let parsed = parse_decls(claim).expect("negative client parses");
        let [SourceDecl::TheoremDecl {
            name: actual,
            params,
            span: own_span,
            ..
        }] = parsed.as_slice()
        else {
            panic!("{name} must be exactly one closed theorem");
        };
        assert_eq!(actual, name);
        assert!(params.is_empty(), "{name} must be closed");
        let mut probe = loaded_package();
        probe
            .elaborate_file(PREFIX)
            .expect("fresh fully accepted prefix precedes each refusal");
        match probe.elaborate_file(claim) {
            Err(ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                span,
            }) if span.start >= own_span.start
                && span.start < span.end
                && span.end <= own_span.end => {}
            other => panic!("{name} must fail at its own false premise: {other:?}"),
        }
    }
}
