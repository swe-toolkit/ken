//! Canonical natural polynomials: spec/50-stdlib/66-symbolic-algebra.md §§2–3.
//! Source: conformance/stdlib/symbolic-algebra/seed-nat-polynomial-canonical-equality.md.
//! Promise class: durable invariant. Independent equal-pair and unequal-pair
//! observations distinguish canonical comparison from ordered-list comparison
//! and a constant answer; generic clients pin all five law interfaces.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, Expr, KenMdFenceRole,
    Type as SourceType,
};
use ken_kernel::{Decl, KernelError};

const PACKAGE: &str = "Algorithm.SymbolicAlgebra.NatPolynomial";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/symbolic-algebra/seed-nat-polynomial-canonical-equality.md"
));
const SUBSTITUTION_SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/symbolic-algebra/seed-nat-polynomial-substitution.md"
));

fn substitution_seed_sources() -> Vec<&'static str> {
    extract_ken_md(SUBSTITUTION_SEED)
        .expect("shipped substitution seed Ken fences parse")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
        .map(|fence| &SUBSTITUTION_SEED[fence.body_range])
        .collect()
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_package() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("NatPolynomial including canonical equality roots-loads");
    env
}

// The seed's fenced Ken programs are compiler inputs, not source-text facts.
// Two positives run in order before the independent false-theorem input.
fn seed_programs() -> Vec<&'static str> {
    SEED.split("\n```ken\n")
        .skip(1)
        .map(|fence| fence.split_once("\n```").expect("closed Ken fence").0)
        .collect()
}

/// SOURCE: spec/50-stdlib/66-symbolic-algebra.md §2.1–2.2.
/// PROMISE CLASS: durable invariant.
/// MEASURED: after five explicit providers load, the exact trusted GlobalId
/// set is unchanged on loading the extended package, each new operation and
/// law has a package-owned transparent body, and arbitrary-expression clients
/// restate all five theorem interfaces. Private helper imports are refused.
/// CLAIMED: §2 provides all its generic proof bodies without additional trust
/// or exposing its canonicity/cancellation proof scaffolding to clients.
/// THE GAP: the providers' assumptions are inherited, and the independent
/// closed semantic discriminator below supplements these generic interfaces.
#[test]
fn section_two_generic_laws_and_public_operations_add_no_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Collections.Derived",
        "Core.Classes.LawfulClasses",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("NatPolynomial §2 generic proof bodies kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "trusted_base() count: before={}, after={}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "§2 adds no trusted GlobalId");

    for public in [
        "eval_poly",
        "canonicalize",
        "poly_eq",
        "canonicalize_sound",
        "canonicalize_complete",
        "poly_eq_sound",
        "poly_eq_complete",
        "poly_eq_false_refutes",
    ] {
        let id = env.globals[&format!("{PACKAGE}.{public}")];
        assert!(owned.contains(&id), "{public} must belong to NatPolynomial");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{public} must have a checked body"
        );
    }
    assert!(owned.contains(&env.globals[&format!("{PACKAGE}.Poly")]));
    env.elaborate_file(GENERIC_CLIENT)
        .expect("independently restated §2.2 laws elaborate at arbitrary expressions");

    for private in ["add_cancel_right", "canon", "is_canonical", "poly_complete"] {
        match env.elaborate_file(&format!("import {PACKAGE} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{PACKAGE}.{private}"));
            }
            other => panic!("{private} must remain private: {other:?}"),
        }
    }
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, Poly, eval, eval_poly, canonicalize, poly_eq,
   canonicalize_sound, canonicalize_complete, poly_eq_sound,
   poly_eq_complete, poly_eq_false_refutes)

theorem client_canonicalize_sound (env : Nat → Nat) (e : Expr) :
  Equal Nat (eval_poly env (canonicalize e)) (eval env e) =
  canonicalize_sound env e

theorem client_canonicalize_complete (e1 : Expr) (e2 : Expr)
    (h : (env : Nat → Nat) → Equal Nat (eval env e1) (eval env e2)) :
  Equal Poly (canonicalize e1) (canonicalize e2) =
  canonicalize_complete e1 e2 h

theorem client_poly_eq_sound (e1 : Expr) (e2 : Expr)
    (h : IsTrue (poly_eq e1 e2)) (env : Nat → Nat) :
  Equal Nat (eval env e1) (eval env e2) =
  poly_eq_sound e1 e2 h env

theorem client_poly_eq_complete (e1 : Expr) (e2 : Expr)
    (h : (env : Nat → Nat) → Equal Nat (eval env e1) (eval env e2)) :
  IsTrue (poly_eq e1 e2) =
  poly_eq_complete e1 e2 h

theorem client_poly_eq_false_refutes (e1 : Expr) (e2 : Expr)
    (hf : Equal Bool (poly_eq e1 e2) False)
    (h : (env : Nat → Nat) → Equal Nat (eval env e1) (eval env e2)) :
  Bottom =
  poly_eq_false_refutes e1 e2 hf h
"#;

fn run_positive_seed(env: &mut ElabEnv, programs: &[&str]) {
    env.elaborate_file(programs[0])
        .expect("§2 closed operation oracles check through client imports");
    env.elaborate_file(programs[1])
        .expect("§2 closed instances of the five checked laws elaborate");
}

/// SOURCE: conformance/stdlib/symbolic-algebra/
/// seed-nat-polynomial-canonical-equality.md positives.
/// PROMISE CLASS: durable invariant.
/// MEASURED: all sixteen source-shaped closed theorems check through the
/// client import, including the nontrivial equal pair x₀+x₀ versus 2·x₀,
/// unequal x₀ versus x₁, the shifted variable and retained zero second child.
/// CLAIMED: canonicalization and Boolean equality observe the §2 Horner
/// semantics, not §1's ordered lists or a constant Boolean answer.
/// THE GAP: the closed cases cannot establish universality; the five generic
/// theorem bodies are checked independently in the previous test.
#[test]
fn seed_operations_and_all_five_law_instances_check_from_client_import() {
    let programs = seed_programs();
    assert_eq!(programs.len(), 3, "two positive and one false seed input");
    let mut env = loaded_package();
    run_positive_seed(&mut env, &programs);
    for name in [
        "equal_x0_sum_and_two_times_x0",
        "eval_x0_sum_oracle",
        "eval_two_times_x0_oracle",
        "eval_horner_x0_double_oracle",
        "eval_horner_x1_oracle",
        "eval_noncanonical_constant_horner_oracle",
        "canonicalize_constant_oracle",
        "canonicalize_x0_sum_oracle",
        "canonicalize_two_times_x0_oracle",
        "poly_eq_equal_pair_oracle",
        "poly_eq_unequal_variables_oracle",
        "canonicalize_sound_instance",
        "canonicalize_complete_instance",
        "poly_eq_sound_instance",
        "poly_eq_complete_instance",
        "poly_eq_false_refutes_instance",
    ] {
        let id = env.globals[name];
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} is a checked seed assertion"
        );
    }
}

/// SOURCE: conformance/stdlib/symbolic-algebra/
/// seed-nat-polynomial-canonical-equality.md negative.
/// PROMISE CLASS: durable invariant.
/// MEASURED: after the full positive client including the `False` observation
/// on x₀/x₁, a false `IsTrue` theorem on that identical pair is rejected by
/// the kernel as K0004 TypeMismatch at its own theorem rather than by import,
/// parse, or unrelated fixture errors.
/// CLAIMED: distinct variables cannot be represented as universally equal.
/// THE GAP: this seed refusal does not construct a counterexample environment
/// from `poly_eq_false_refutes`, which only contradicts a universal proof.
#[test]
fn unequal_variable_false_theorem_is_kernel_refused_on_same_positive_pair() {
    let programs = seed_programs();
    assert_eq!(programs.len(), 3, "two positive and one false seed input");
    let mut env = loaded_package();
    run_positive_seed(&mut env, &programs);
    let negative = programs[2];
    match env.elaborate_file(negative) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) => {
            let start = negative
                .find("theorem reject_unequal_variables_as_equal")
                .expect("negative theorem declaration");
            assert!(
                span.start >= start && span.end <= negative.len() && span.start < span.end,
                "K0004 must refuse the false theorem's own proof: {span:?}"
            );
        }
        other => panic!("expected kernel TypeMismatch for false IsTrue, got {other:?}"),
    }
}

/// SOURCE: spec/50-stdlib/66-symbolic-algebra.md §3.
/// PROMISE CLASS: durable invariant.
/// MEASURED: after the five explicit §1–2 providers, the exact trusted
/// GlobalId set is unchanged. All four §3 exports are package-owned and
/// transparent; a client consumes both generic laws at arbitrary inputs and
/// cannot import the new private congruence helper.
/// CLAIMED: substitution and its forward equality law add checked public
/// definitions without enlarging the trusted base or public helper surface.
/// THE GAP: finite seed oracles and the refusal are tested separately below.
#[test]
fn section_three_generic_substitution_laws_are_checked_without_new_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Collections.Derived",
        "Core.Classes.LawfulClasses",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("§3 checked bodies and package roots-load");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "§3 trusted_base() count: before={}, after={}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "§3 adds no trusted GlobalId");
    for public in [
        "compose_env",
        "substitute",
        "eval_substitute",
        "poly_eq_substitute",
    ] {
        let id = env.globals[&format!("{PACKAGE}.{public}")];
        assert!(owned.contains(&id), "{public} must belong to NatPolynomial");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{public} must have a checked body"
        );
    }
    env.elaborate_file(SUBSTITUTION_GENERIC_CLIENT)
        .expect("§3 independent generic law clients consume their stated propositions");
    match env.elaborate_file(&format!("import {PACKAGE} (mul_cong)")) {
        Err(ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, format!("{PACKAGE}.mul_cong"));
        }
        other => panic!("congruence helper must remain private: {other:?}"),
    }
}

const SUBSTITUTION_GENERIC_CLIENT: &str = r#"
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, eval, poly_eq, compose_env, substitute,
   eval_substitute, poly_eq_substitute)

theorem client_eval_substitute
    (env : Nat → Nat) (s : Nat → Expr) (e : Expr) :
  Equal Nat (eval env (substitute s e))
    (eval (compose_env env s) e) =
  eval_substitute env s e

theorem client_poly_eq_substitute
    (s : Nat → Expr) (e1 : Expr) (e2 : Expr)
    (h : IsTrue (poly_eq e1 e2)) :
  IsTrue (poly_eq (substitute s e1) (substitute s e2)) =
  poly_eq_substitute s e1 e2 h
"#;

/// SOURCE: conformance/stdlib/symbolic-algebra/
/// seed-nat-polynomial-substitution.md positive fences.
/// PROMISE CLASS: durable invariant.
/// MEASURED: every shipped positive fence checks from a roots-loaded client,
/// including the distinct values 4/3, composed value 4, identity mapping,
/// forward poly equality, and a pair made equal by substitution.
/// CLAIMED: the public operations and generic laws match the closed seed
/// behavior rather than substituting twice or preserving a false decision.
/// THE GAP: generic laws are kernel-checked above; finite oracles are not a
/// proof that every environment and expression gives the claimed result.
#[test]
fn shipped_substitution_seed_positive_cases_check() {
    let mut sources = substitution_seed_sources();
    sources
        .pop()
        .expect("separate uncomposed-env refusal fence");
    assert!(!sources.is_empty(), "seed needs positive source fences");
    let mut env = loaded_package();
    for positive in sources {
        env.elaborate_file(positive)
            .expect("each shipped substitution positive program checks");
    }
}

// Closed false equalities can both reduce to Bottom, so checking convertibility
// of a proposed negative goal is insufficient. Instead compare parsed Ken
// proposition trees, discarding only spans; obtain the two endpoints from the
// checker-accepted positive controls on the SAME substitution and environment.
fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) => format!("constructor:{name}"),
        SourceType::TVar(name, _) => format!("variable:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unsupported shape in closed substitution seed: {other:?}"),
    }
}

fn application<'a>(ty: &'a SourceType, name: &str) -> Option<Vec<&'a SourceType>> {
    let mut args = Vec::new();
    let mut head = ty;
    while let SourceType::TApp(function, argument, _) = head {
        args.push(argument.as_ref());
        head = function;
    }
    args.reverse();
    match head {
        SourceType::TVar(found, _) | SourceType::TCon(found, _) if found == name => Some(args),
        _ => None,
    }
}

fn assert_uncomposed_proposition(shared: &str, refused: &str) {
    let positive_decls = parse_decls(shared).expect("negative fixture's controls parse");
    let equalities = positive_decls
        .iter()
        .filter_map(|decl| match decl {
            SourceDecl::TheoremDecl { theorem, .. } => {
                let args = application(theorem, "Equal")?;
                let [carrier, lhs, rhs] = args.as_slice() else {
                    return None;
                };
                let eval = application(lhs, "eval")?;
                let [env, arg] = eval.as_slice() else {
                    return None;
                };
                Some((*carrier, *lhs, *rhs, *env, *arg))
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    let substituted = equalities
        .iter()
        .find_map(|(carrier, lhs, value, env, arg)| {
            let args = application(arg, "substitute")?;
            let [replacement, original] = args.as_slice() else {
                return None;
            };
            Some((*carrier, *lhs, *value, *env, *replacement, *original))
        })
        .expect("positive substitution value control on original operands");
    let (carrier, substituted_eval, value, env, replacement, original) = substituted;
    let original_eval = equalities
        .iter()
        .find(|(_, _, _, same_env, arg)| {
            type_shape(same_env) == type_shape(env) && type_shape(arg) == type_shape(original)
        })
        .map(|(_, lhs, _, _, _)| *lhs)
        .expect("positive uncomposed value control on original operands");
    let composed = equalities
        .iter()
        .find(|(_, _, same_value, candidate_env, arg)| {
            let Some(args) = application(candidate_env, "compose_env") else {
                return false;
            };
            let [inner_env, s] = args.as_slice() else {
                return false;
            };
            type_shape(inner_env) == type_shape(env)
                && type_shape(s) == type_shape(replacement)
                && type_shape(arg) == type_shape(original)
                && type_shape(same_value) == type_shape(value)
        });
    assert!(
        composed.is_some(),
        "positive composed-environment control checks"
    );
    let negative_decls = parse_decls(refused).expect("refusal Ken parses");
    let [SourceDecl::TheoremDecl {
        theorem: negative_goal,
        body: Expr::ECon(proof, _),
        ..
    }] = negative_decls.as_slice()
    else {
        panic!("refused block must be one theorem with its proof");
    };
    assert_eq!(proof, "Proved", "negative claim must attempt its proof");
    let snippet = |ty: &SourceType| &shared[ty.span().start..ty.span().end];
    let expected = format!(
        "theorem expected_uncomposed : Equal ({}) ({}) ({}) = Proved",
        snippet(carrier),
        snippet(substituted_eval),
        snippet(original_eval)
    );
    let expected_decls = parse_decls(&expected).expect("derived uncomposed equality parses");
    let [SourceDecl::TheoremDecl {
        theorem: expected_goal,
        ..
    }] = expected_decls.as_slice()
    else {
        panic!("expected uncomposed equality is one theorem");
    };
    assert_eq!(
        type_shape(negative_goal),
        type_shape(expected_goal),
        "refused theorem must state uncomposed evaluation on positive control operands"
    );
}

/// SOURCE: conformance/stdlib/symbolic-algebra/
/// seed-nat-polynomial-substitution.md negative fence.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the three closed control theorems check first, and the actual
/// separate refused theorem is structurally the uncomposed evaluation claim
/// built from their positive operands; its own `Proved` is rejected with a
/// kernel TypeMismatch at the final theorem, not at an earlier declaration.
/// CLAIMED: on this fixture evaluation after substitution is not evaluation
/// under the original environment, unlike under the composed environment.
/// THE GAP: one closed counterexample does not decide arbitrary substitutions.
#[test]
fn shipped_uncomposed_env_refuses_on_the_same_positive_operands() {
    let mut sources = substitution_seed_sources();
    let negative = sources
        .pop()
        .expect("separate uncomposed-env refusal fence");
    let declarations = parse_decls(negative).expect("negative fixture Ken parses");
    let Some(SourceDecl::TheoremDecl { span, .. }) = declarations.last() else {
        panic!("negative fixture ends in its uncomposed-env theorem");
    };
    let (shared, refused) = negative.split_at(span.start);
    assert_uncomposed_proposition(shared, refused);
    let mut env = loaded_package();
    env.elaborate_file(shared)
        .expect("all negative-fixture value and composed controls check first");
    match env.elaborate_file(refused) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if span.start < span.end && span.end <= refused.len() => {}
        other => {
            panic!("uncomposed-environment proposition must refuse at its own proof: {other:?}")
        }
    }
}
