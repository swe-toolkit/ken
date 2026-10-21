//! Option-valued least-cost selection: spec/50-stdlib/72-optimization.md §1.4.
//! Source: conformance/stdlib/optimization/seed-argmin-option.md.
//! Promise class: durable invariant. Empty, singleton and a non-head,
//! non-last earliest tie are independently observed; generic laws are consumed.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, ImportKind, KenMdFenceRole,
    Type as SourceType,
};
use ken_kernel::{Decl, KernelError};

const ARGMIN: &str = "Algorithm.Optimization.Argmin";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/optimization/seed-argmin-option.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_argmin() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], ARGMIN)
        .expect("Argmin and its provider closure root-load");
    env
}

// The compiler base already binds IsTrue. Omit its redundant unaliased import
// from the executable seed, retaining every other declaration and import item.
fn omit_redundant_is_true(source: &str) -> String {
    let mut edits = Vec::new();
    for declaration in parse_decls(source).expect("seed Ken parses") {
        if let SourceDecl::ImportDecl {
            module,
            kind: ImportKind::Selective(items),
            span,
        } = declaration
        {
            if module != "Core.Classes.LawfulClasses" {
                continue;
            }
            let mut keep = Vec::new();
            let mut removed = false;
            for item in items {
                if item.name == "IsTrue" && item.rename.is_none() {
                    removed = true;
                } else {
                    keep.push(match item.rename {
                        Some(alias) => format!("{} as {alias}", item.name),
                        None => item.name,
                    });
                }
            }
            if removed {
                let replacement = if keep.is_empty() {
                    String::new()
                } else {
                    format!("import {module} ({})", keep.join(", "))
                };
                edits.push((span.start..span.end, replacement));
            }
        }
    }
    let mut client = source.to_owned();
    for (range, replacement) in edits.into_iter().rev() {
        client.replace_range(range, &replacement);
    }
    client
}

fn seed_programs() -> Vec<String> {
    extract_ken_md(SEED)
        .expect("§1.4 seed fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
        .map(|fence| omit_redundant_is_true(&SEED[fence.body_range]))
        .collect()
}

// These clients apply every exported generic §1.4 signature, and feed the
// observed Some result into both direct-to-List corollaries.
const GENERIC_CLIENT: &str = r#"
import Algorithm.Optimization.Argmin
  (argmin, argmin_option, argmin_option_nil, argmin_option_cons,
   argmin_option_member, argmin_option_least)
import Core.Classes.LawfulClasses (leq_nat)
import Data.Collections.Derived (list_elem)
import Data.Collections.NonEmpty (nonempty_cons)

theorem client_nil (a : Type) (cost : a → Nat) :
  Equal (Option a) (argmin_option a cost (Nil a)) (None a) =
  argmin_option_nil a cost

theorem client_cons (a : Type) (cost : a → Nat) (x : a) (rest : List a) :
  Equal (Option a) (argmin_option a cost (Cons a x rest))
    (Some a (argmin a cost (nonempty_cons a x rest))) =
  argmin_option_cons a cost x rest

theorem client_member
    (a : Type) (cost : a → Nat) (xs : List a) (y : a)
    (found : Equal (Option a) (argmin_option a cost xs) (Some a y)) :
  list_elem a y xs =
  argmin_option_member a cost xs y found

theorem client_least
    (a : Type) (cost : a → Nat) (xs : List a) (y : a)
    (found : Equal (Option a) (argmin_option a cost xs) (Some a y))
    (z : a) (member : list_elem a z xs) :
  IsTrue (leq_nat (cost y) (cost z)) =
  argmin_option_least a cost xs y found z member
"#;

/// SOURCE: spec/50-stdlib/72-optimization.md §1.4.
/// PROMISE CLASS: durable invariant.
/// MEASURED: five new globals belong to the roots-loaded package and have
/// transparent bodies; a generic client uses both equations and unpacks both
/// Some corollaries; four helpers refuse selective import. The exact trusted
/// GlobalId set is unchanged after loading the package above its providers.
/// CLAIMED: §1.4 adds zero trust and exposes exactly the intended API.
/// THE GAP: inherited provider assumptions remain; the finite seed below
/// independently observes empty and nonempty selection behavior.
#[test]
fn generic_option_exports_check_without_new_trust_or_public_helpers() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Classes.LawfulClasses",
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
        .elaborate_module_from_roots(&[root()], ARGMIN)
        .expect("Argmin Option definitions and proof bodies kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "§1.4 trusted_base() count: before={}, after={}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "Argmin Option adds no trusted GlobalId");
    for name in [
        "argmin_option",
        "argmin_option_nil",
        "argmin_option_cons",
        "argmin_option_member",
        "argmin_option_least",
    ] {
        let id = env.globals[&format!("{ARGMIN}.{name}")];
        assert!(owned.contains(&id), "{name} must be Argmin-owned");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("§1.4 generic client consumes all four laws");
    for private in [
        "option_member",
        "option_least",
        "argmin_option_member_base",
        "argmin_option_least_base",
    ] {
        match env.elaborate_file(&format!("import {ARGMIN} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{ARGMIN}.{private}"));
            }
            other => panic!("{private} must remain private: {other:?}"),
        }
    }
}

/// SOURCE: conformance/stdlib/optimization/seed-argmin-option.md positives.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the complete shipped positive source fence compiles with the
/// checked Nil, singleton, non-head and non-last tie outputs, and four public
/// law instances, including observed Some membership and least cost.
/// CLAIMED: Nil has no selected value; nonempty selection delegates to argmin
/// and keeps the earliest minimum on a tie.
/// THE GAP: these three closed inputs supplement the generic checked laws;
/// no algorithmic complexity or arbitrary-value equality is inferred.
#[test]
fn shipped_option_seed_positive_client_checks() {
    let programs = seed_programs();
    let [positive, _refusal] = programs.as_slice() else {
        panic!("the §1.4 seed has a positive client and one isolated refusal");
    };
    let mut env = loaded_argmin();
    env.elaborate_file(positive)
        .expect("all shipped §1.4 positive controls check");
    for name in [
        "empty_returns_none",
        "singleton_returns_its_only_value",
        "earliest_tied_minimum_is_selected",
        "nil_equation_instance",
        "cons_equation_instance",
        "two_is_a_member_of_tie_input",
        "selected_value_has_input_membership",
        "selected_cost_is_least_against_the_head",
    ] {
        let id = env.globals[name];
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
}

// The proposition is compared as parsed Ken, not by offsets or whitespace.
// The seed's independent positive control must check before the false theorem.
fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) => format!("constructor:{name}"),
        SourceType::TVar(name, _) => format!("variable:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unsupported closed Option proposition: {other:?}"),
    }
}

/// SOURCE: conformance/stdlib/optimization/seed-argmin-option.md refusal.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the accepted positive client (including the same Nil input)
/// checks before the isolated false theorem; its parsed proposition is the
/// specified Some-two-on-Nil claim, and the kernel rejects at that theorem
/// with TypeMismatch rather than rejecting an unavailable export or fixture.
/// CLAIMED: the empty list cannot yield a Some value.
/// THE GAP: a single Nil witness does not characterize every empty-type case.
#[test]
fn shipped_some_on_nil_claim_refuses_at_the_pinned_theorem() {
    let programs = seed_programs();
    let [positive, refused] = programs.as_slice() else {
        panic!("the §1.4 seed has a positive client and one isolated refusal");
    };
    let expected = parse_decls(
        "theorem expected_false_result : Equal (Option Nat) \
         (argmin_option Nat seed_cost empty_input) (Some Nat two) = Proved",
    )
    .expect("expected Some-on-Nil proposition parses");
    let [SourceDecl::TheoremDecl {
        theorem: expected_goal,
        ..
    }] = expected.as_slice()
    else {
        panic!("independent expected proposition has one theorem");
    };
    let negative_decls = parse_decls(refused).expect("negative Ken fence parses");
    let [SourceDecl::TheoremDecl {
        name,
        theorem,
        body: ken_elaborator::Expr::ECon(proof, _),
        span,
        ..
    }] = negative_decls.as_slice()
    else {
        panic!("isolated refusal contains one attempted proof theorem");
    };
    assert_eq!(name, "seed_neg");
    assert_eq!(proof, "Proved", "refusal must attempt a checked proof");
    assert_eq!(
        type_shape(theorem),
        type_shape(expected_goal),
        "the refusal must assert the shipped Some-on-Nil proposition"
    );
    let mut env = loaded_argmin();
    env.elaborate_file(positive)
        .expect("same-input None control and all positives check first");
    match env.elaborate_file(refused) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span: failure,
        }) if failure.start >= span.start
            && failure.end <= span.end
            && failure.start < failure.end => {}
        other => panic!("false Some-on-Nil theorem must kernel-reject: {other:?}"),
    }
}
