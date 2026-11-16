//! Public prefix composition for take and least-k: spec/50-stdlib/72-optimization.md §4.5.
//! Promise class: durable invariant. Open clients consume the checked public
//! law types; closed tied-cost values and isolated false claims discriminate
//! the prefix, last-two, and violated-bound cases.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

const DERIVED: &str = "Data.Collections.Derived";
const ARGMIN: &str = "Algorithm.Optimization.Argmin";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded() -> (
    ElabEnv,
    BTreeSet<GlobalId>,
    BTreeSet<GlobalId>,
    BTreeSet<GlobalId>,
) {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Classes.LawfulClasses",
        "Core.Function.Combinators",
        "Core.Logic.Or",
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Collections.List",
        "Data.Collections.NonEmpty",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect();
    let derived_owned = env
        .elaborate_module_from_roots(&[root()], DERIVED)
        .expect("Derived roots-loads");
    let after_derived = env.env.trusted_base().into_iter().collect();
    let argmin_owned = env
        .elaborate_module_from_roots(&[root()], ARGMIN)
        .expect("Argmin roots-loads");
    let after_argmin = env.env.trusted_base().into_iter().collect();
    for (module, law, owned) in [
        (DERIVED, "take::prefix", &derived_owned),
        (ARGMIN, "least_k_prefix", &argmin_owned),
    ] {
        let qualified = format!("{module}.{law}");
        let id = env.globals[&qualified];
        assert!(owned.contains(&id), "{qualified} is declared by its owner");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{qualified} has a checked transparent proof"
        );
    }
    (env, before, after_derived, after_argmin)
}

fn has_const(body: &Term, owner: GlobalId) -> bool {
    let mut terms = vec![body];
    while let Some(term) = terms.pop() {
        if matches!(term, Term::Const { id, .. } if *id == owner) {
            return true;
        }
        terms.extend(term.children());
    }
    false
}

const W_GEN: &str = r#"import Algorithm.Optimization.Argmin (least_k, least_k_prefix)
import Core.Classes.LawfulClasses (leq_nat)
import Data.Collections.Derived (take)
import Data.Numeric.Nat.Arithmetic (add)

theorem least_k_prefix_generic
      (a : Type) (cost : a → Nat) (k : Nat) (m : Nat) (xs : List a)
    : Equal (List a) (least_k a cost k xs) (take a k (least_k a cost (add k m) xs)) =
  least_k_prefix a cost k m xs

theorem take_prefix_generic
      (a : Type) (k : Nat) (n : Nat) (xs : List a)
    : Equal Bool (leq_nat k n) True → Equal (List a) (take a k (take a n xs)) (take a k xs) =
  (proof prefix for take) a k n xs
"#;

const ACCEPTED: &str = r#"import Algorithm.Optimization.Argmin (least_k, least_k_prefix)
import Data.Collections.Derived (take, reverse)
import Data.Numeric.Nat.Arithmetic (add)

data Candidate : Type where {
  A : Candidate;
  B : Candidate;
  C : Candidate;
  D : Candidate
}

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two

fn seed_cost (x : Candidate) : Nat =
  match x {
    A ↦ two;
    B ↦ one;
    C ↦ two;
    D ↦ three
  }

const input : List Candidate =
  Cons Candidate A (Cons Candidate B (Cons Candidate C (Cons Candidate D (Nil Candidate))))

fn last_two (xs : List Candidate) : List Candidate =
  reverse Candidate (take Candidate two (reverse Candidate xs))

theorem v_least_two
    : Equal (List Candidate) (least_k Candidate seed_cost two input)
        (Cons Candidate B (Cons Candidate A (Nil Candidate))) =
  Refl

theorem v_least_three
    : Equal (List Candidate) (least_k Candidate seed_cost (add two one) input)
        (Cons Candidate B (Cons Candidate A (Cons Candidate C (Nil Candidate)))) =
  Refl

theorem v_last_two_of_three
    : Equal (List Candidate) (last_two (least_k Candidate seed_cost (add two one) input))
        (Cons Candidate A (Cons Candidate C (Nil Candidate))) =
  Refl

theorem c_closed_prefix
    : Equal (List Candidate) (least_k Candidate seed_cost two input)
        (take Candidate two (least_k Candidate seed_cost (add two one) input)) =
  Refl

theorem c_law_prefix
    : Equal (List Candidate) (least_k Candidate seed_cost two input)
        (take Candidate two (least_k Candidate seed_cost (add two one) input)) =
  least_k_prefix Candidate seed_cost two one input

theorem v_take_three_of_two : Equal (List Candidate) (take Candidate three (take Candidate two input))
    (Cons Candidate A (Cons Candidate B (Nil Candidate))) =
  Refl

theorem v_take_three
    : Equal (List Candidate) (take Candidate three input)
        (Cons Candidate A (Cons Candidate B (Cons Candidate C (Nil Candidate)))) =
  Refl

theorem t_closed_take
    : Equal (List Candidate) (take Candidate two (take Candidate three input)) (take Candidate two input) =
  Refl

theorem t_law_take
    : Equal (List Candidate) (take Candidate two (take Candidate three input)) (take Candidate two input) =
  (proof prefix for take) Candidate two three input Proved
"#;

const X_SAME: &str = "theorem x_same_list : Equal (List Candidate) (least_k Candidate seed_cost two input) (least_k Candidate seed_cost (add two one) input) = Refl\n";
const X_LAST: &str = "theorem x_last_two : Equal (List Candidate) (least_k Candidate seed_cost two input) (last_two (least_k Candidate seed_cost (add two one) input)) = Refl\n";
const X_BOUND: &str = "theorem x_take_bound : Equal (List Candidate) (take Candidate three (take Candidate two input)) (take Candidate three input) = (proof prefix for take) Candidate three two input Proved\n";

/// SOURCE: spec 72 §4.5. Promise class: durable invariant.
/// MEASURED: after the eight providers, exact trusted GlobalId sets remain
/// identical across Derived and Argmin; both laws belong to the loaded owner
/// and have Transparent bodies. Open selective imports apply their full types
/// and their checked bodies refer to the authenticated owner Const IDs.
/// CLAIMED: these public general laws add no trust and are usable at open types.
/// THE GAP: provider trust is inherited, not eliminated; the kernel checks the
/// proof bodies, while this client pins their importability and applications.
#[test]
fn both_prefix_laws_are_public_generic_and_zero_new_trust() {
    let (mut env, before, after_derived, after_argmin) = loaded();
    assert_eq!(after_derived, before, "Derived adds no trusted identity");
    assert_eq!(after_argmin, before, "Argmin adds no trusted identity");
    env.elaborate_file(W_GEN)
        .expect("both open selective-import clients check");
    for (client, owner) in [
        ("least_k_prefix_generic", format!("{ARGMIN}.least_k_prefix")),
        ("take_prefix_generic", format!("{DERIVED}.take::prefix")),
    ] {
        let owner_id = env.globals[&owner];
        let client_id = env.globals[client];
        let (_, body) = env
            .env
            .transparent_body(client_id)
            .expect("checked client body");
        assert!(
            has_const(&body, owner_id),
            "{client} must use the exact {owner} Const"
        );
    }
}

/// SOURCE: spec 72 §4.5. Promise class: durable invariant.
/// MEASURED: the complete tied-cost accepted prefix checks, including closed
/// values for least-k 2, least-k 3, its last two, and take 3 of take 2 versus
/// take 3. Each negative is tried alone after that whole prefix in a fresh
/// environment; its exact error kind and terminal span lie in its own theorem.
/// CLAIMED: same-size and suffix laws are false; the take composition needs
/// its bound. THE GAP: these closed witnesses do not replace the open laws.
#[test]
fn tied_prefix_values_accept_and_each_false_claim_refuses_at_own_terminal() {
    let (mut positive, _, _, _) = loaded();
    positive
        .elaborate_file(ACCEPTED)
        .expect("whole accepted block checks");
    for name in [
        "v_least_two",
        "v_least_three",
        "v_last_two_of_three",
        "c_closed_prefix",
        "c_law_prefix",
        "v_take_three_of_two",
        "v_take_three",
        "t_closed_take",
        "t_law_take",
    ] {
        let id = positive.globals[name];
        assert!(
            matches!(positive.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} must be a checked theorem"
        );
    }
    for (name, row, terminal) in [
        ("x_same_list", X_SAME, "Refl"),
        ("x_last_two", X_LAST, "Refl"),
        ("x_take_bound", X_BOUND, "Proved"),
    ] {
        let (mut fresh, _, _, _) = loaded();
        fresh
            .elaborate_file(ACCEPTED)
            .unwrap_or_else(|error| panic!("{name} prefix: {error:?}"));
        let decls = parse_decls(row).expect("isolated negative parses");
        let [SourceDecl::TheoremDecl {
            name: actual,
            span: claim,
            ..
        }] = decls.as_slice()
        else {
            panic!("{name} must be one theorem");
        };
        assert_eq!(actual, name);
        let span = match fresh
            .elaborate_file(row)
            .expect_err("false theorem must refuse")
        {
            ElabError::TypeMismatch { reason, span }
                if terminal == "Refl" && reason.contains("Refl expects an `Eq`-shaped goal") =>
            {
                span
            }
            ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                span,
            } if terminal == "Proved" => span,
            other => panic!("{name} must refuse with its ruled exact kind: {other:?}"),
        };
        assert!(
            claim.start <= span.start && span.start < span.end && span.end <= claim.end,
            "{name}: refusal must be within its own claim: {span:?} vs {claim:?}"
        );
        let terminal_start = row.rfind(terminal).expect("terminal exists");
        assert!(
            span.start <= terminal_start && terminal_start + terminal.len() <= span.end,
            "{name}: error must enclose its own terminal: {span:?}"
        );
        if terminal == "Refl" {
            assert_eq!(
                &row[span.start..span.end],
                terminal,
                "{name}: exact Refl span"
            );
        }
    }
}
