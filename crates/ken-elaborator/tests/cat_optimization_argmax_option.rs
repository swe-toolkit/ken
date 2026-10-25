//! Option-valued greatest-cost selection: spec/50-stdlib/72-optimization.md §3.4.
//! Source: conformance/stdlib/optimization/seed-argmax-option.md.
//! Promise class: durable invariant. The seed covers Nil, singleton and an
//! earliest maximum that is neither head nor last tie; generic laws are used.

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
    "/../../conformance/stdlib/optimization/seed-argmax-option.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_argmin() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], ARGMIN)
        .expect("Argmin, Argmax Option and their provider closure root-load");
    env
}

// The compiler base already binds IsTrue. Its redundant unaliased import in
// the seed cannot elaborate; keep every other seed declaration and import item.
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
        .expect("§3.4 seed fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
        .map(|fence| omit_redundant_is_true(&SEED[fence.body_range]))
        .collect()
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.Optimization.Argmin
  (argmax, argmax_option, argmax_option_nil, argmax_option_cons,
   argmax_option_member, argmax_option_greatest)
import Core.Classes.LawfulClasses (leq_nat)
import Data.Collections.Derived (list_elem)
import Data.Collections.NonEmpty (nonempty_cons)

theorem client_nil (a : Type) (cost : a → Nat) :
  Equal (Option a) (argmax_option a cost (Nil a)) (None a) =
  argmax_option_nil a cost

theorem client_cons (a : Type) (cost : a → Nat) (x : a) (rest : List a) :
  Equal (Option a) (argmax_option a cost (Cons a x rest))
    (Some a (argmax a cost (nonempty_cons a x rest))) =
  argmax_option_cons a cost x rest

theorem client_member
    (a : Type) (cost : a → Nat) (xs : List a) (y : a)
    (found : Equal (Option a) (argmax_option a cost xs) (Some a y)) :
  list_elem a y xs =
  argmax_option_member a cost xs y found

theorem client_greatest
    (a : Type) (cost : a → Nat) (xs : List a) (y : a)
    (found : Equal (Option a) (argmax_option a cost xs) (Some a y))
    (z : a) (member : list_elem a z xs) :
  IsTrue (leq_nat (cost z) (cost y)) =
  argmax_option_greatest a cost xs y found z member
"#;

/// SOURCE: spec/50-stdlib/72-optimization.md §3.4.
/// PROMISE CLASS: durable invariant.
/// MEASURED: five named globals are transparent and package-owned; a generic
/// client applies four exact law types and consumes both Some corollaries;
/// three helpers refuse selective import. Trusted GlobalId sets are equal.
/// CLAIMED: the specified §3.4 operations and laws add no new trust.
/// THE GAP: this does not enumerate all public module exports and does not
/// erase assumptions inherited from the provider closure.
#[test]
fn generic_argmax_option_exports_check_without_new_trust_or_public_helpers() {
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
        .expect("Argmax Option exports and proof bodies kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "§3.4 trusted_base() count: before={}, after={}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "Argmax Option adds no trusted GlobalId");
    for name in [
        "argmax_option",
        "argmax_option_nil",
        "argmax_option_cons",
        "argmax_option_member",
        "argmax_option_greatest",
    ] {
        let id = env.globals[&format!("{ARGMIN}.{name}")];
        assert!(owned.contains(&id), "{name} must be Argmin-owned");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("generic client applies all four §3.4 laws");
    for private in [
        "option_greatest",
        "argmax_option_member_base",
        "argmax_option_greatest_base",
    ] {
        match env.elaborate_file(&format!("import {ARGMIN} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{ARGMIN}.{private}"));
            }
            other => panic!("{private} must stay private: {other:?}"),
        }
    }
}

/// SOURCE: conformance/stdlib/optimization/seed-argmax-option.md positives.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the complete seed positive fence elaborates, including Nil,
/// singleton, non-head earliest greatest-cost tie and distinct minimum,
/// plus all four law instances and membership/greatest-cost certificates.
/// CLAIMED: None on empty, Some on nonempty and earliest tied maximum wins.
/// THE GAP: three closed inputs supplement rather than replace generic laws;
/// neither runtime cost nor arbitrary-value equality is proved here.
#[test]
fn shipped_argmax_option_seed_positive_client_checks() {
    let programs = seed_programs();
    assert_eq!(
        programs.len(),
        6,
        "one positive and five refusal Ken fences"
    );
    let mut env = loaded_argmin();
    env.elaborate_file(&programs[0])
        .expect("entire shipped §3.4 positive client kernel-checks");
    for name in [
        "empty_returns_none",
        "singleton_returns_its_only_value",
        "earliest_tied_maximum_is_selected",
        "same_input_minimum_is_distinct",
        "nil_equation_instance",
        "cons_equation_instance",
        "two_is_a_member_of_tie_input",
        "selected_value_has_input_membership",
        "selected_cost_is_greatest_against_later_tie",
    ] {
        let id = env.globals[name];
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
}

fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) => format!("constructor:{name}"),
        SourceType::TVar(name, _) => format!("variable:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unsupported closed Argmax Option proposition: {other:?}"),
    }
}

/// SOURCE: conformance/stdlib/optimization/seed-argmax-option.md refusals.
/// PROMISE CLASS: durable invariant.
/// MEASURED: for each refusal, the entire accepted positive client precedes
/// exactly one parsed closed false theorem. Its independently specified
/// proposition is well-typed with a witness, but its Proved body is rejected
/// by the kernel with TypeMismatch at that theorem, not at a missing export.
/// CLAIMED: Nil cannot return Some; a nonempty tie returns Some two, neither
/// None, the head, the later tie nor the minimum on the same list.
/// THE GAP: these five exact closed counterexamples do not enumerate all
/// possible lists; QA owns independent AC-3 production mutations.
#[test]
fn shipped_argmax_option_false_claims_refuse_after_full_positive_prefix() {
    let programs = seed_programs();
    assert_eq!(
        programs.len(),
        6,
        "one positive and five refusal Ken fences"
    );
    let expected_claims = [
        ("refuse_some_on_nil", "empty_input", "Some Nat two"),
        ("refuse_none_on_nonempty", "tie_input", "None Nat"),
        ("refuse_head", "tie_input", "Some Nat three"),
        ("refuse_last_tie", "tie_input", "Some Nat one"),
        ("refuse_least", "tie_input", "Some Nat Zero"),
    ];
    for ((name, input, claimed_result), refused) in
        expected_claims.into_iter().zip(programs.iter().skip(1))
    {
        let expected = parse_decls(&format!(
            "theorem expected_false_result : Equal (Option Nat) \
             (argmax_option Nat seed_cost {input}) ({claimed_result}) = Proved"
        ))
        .expect("independently specified refusal parses");
        let [SourceDecl::TheoremDecl {
            theorem: expected_goal,
            ..
        }] = expected.as_slice()
        else {
            panic!("expected proposition has one theorem");
        };
        let negative = parse_decls(refused).expect("isolated refusal Ken fence parses");
        let [SourceDecl::TheoremDecl {
            name: actual_name,
            params,
            theorem,
            body: ken_elaborator::Expr::ECon(proof, _),
            span,
            ..
        }] = negative.as_slice()
        else {
            panic!("each refusal contains one attempted closed theorem");
        };
        assert_eq!(actual_name, name);
        assert!(params.is_empty(), "the refused proposition is closed");
        assert_eq!(proof, "Proved", "the refusal attempts a checked proof");
        assert_eq!(
            type_shape(theorem),
            type_shape(expected_goal),
            "{name} must assert its intended same-input false result"
        );
        let claim_span = theorem.span();
        let claim = &refused[claim_span.start..claim_span.end];
        let well_typed_claim =
            format!("theorem refusal_claim_typechecks (h : {claim}) : {claim} = h");
        let mut checked_type = loaded_argmin();
        checked_type
            .elaborate_file(&programs[0])
            .expect("full positive client checks before the type control");
        checked_type
            .elaborate_file(&well_typed_claim)
            .expect("refused proposition is well-typed under a witness");

        let mut env = loaded_argmin();
        env.elaborate_file(&programs[0])
            .expect("entire positive client checks before each fresh refusal");
        match env.elaborate_file(refused) {
            Err(ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                span: failure,
            }) if failure.start >= span.start
                && failure.end <= span.end
                && failure.start < failure.end => {}
            other => panic!("{name} must kernel-reject at its own theorem: {other:?}"),
        }
    }
}
