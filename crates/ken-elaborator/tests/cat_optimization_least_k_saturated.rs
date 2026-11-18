//! Spec 72 §4.6: least-k saturates at the sorted input's length.
//! Promise class: durable invariant. Two public, zero-new-trust laws accept
//! generic clients; a fixed stable-tie fixture separates sorted from input
//! order and checks the false short-bound premise.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

const DERIVED: &str = "Data.Collections.Derived";
const ARGMIN: &str = "Algorithm.Optimization.Argmin";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")
}

fn loaded() -> (
    ElabEnv,
    BTreeSet<GlobalId>,
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
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("provider {provider} roots-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect();
    let derived_owned = env
        .elaborate_module_from_roots(&[root()], DERIVED)
        .expect("Derived roots-loads");
    let after_derived = env.env.trusted_base().into_iter().collect();
    // NonEmpty imports Derived. Loading it as a provider before `before`
    // would make the Derived trust comparison vacuous.
    env.elaborate_module_from_roots(&[root()], "Data.Collections.NonEmpty")
        .expect("NonEmpty roots-loads after Derived");
    let after_nonempty = env.env.trusted_base().into_iter().collect();
    let argmin_owned = env
        .elaborate_module_from_roots(&[root()], ARGMIN)
        .expect("Argmin roots-loads");
    let after_argmin = env.env.trusted_base().into_iter().collect();
    for (module, law, owned) in [
        (DERIVED, "take::saturated", &derived_owned),
        (ARGMIN, "least_k_saturated", &argmin_owned),
    ] {
        let qualified = format!("{module}.{law}");
        let id = env.globals[&qualified];
        assert!(owned.contains(&id), "{qualified} is declared by its owner");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{qualified} has a checked transparent proof"
        );
    }
    (env, before, after_derived, after_nonempty, after_argmin)
}

fn has_const(body: &Term, owner: GlobalId) -> bool {
    let mut pending = vec![body];
    while let Some(term) = pending.pop() {
        if matches!(term, Term::Const { id, .. } if *id == owner) {
            return true;
        }
        pending.extend(term.children());
    }
    false
}

const OPEN: &str = r#"import Algorithm.Optimization.Argmin (least_k, cost_le, least_k_saturated)
import Core.Classes.LawfulClasses (leq_nat)
import Data.Collections.Derived (take, sort)
import Data.Collections.List (length)

theorem least_k_saturated_generic
      (a : Type) (cost : a → Nat) (k : Nat) (xs : List a)
    : Equal Bool (leq_nat (length a xs) k) True
      → Equal (List a) (least_k a cost k xs) (sort a (cost_le a cost) xs) =
  least_k_saturated a cost k xs

theorem take_saturated_generic
      (a : Type) (n : Nat) (xs : List a)
    : Equal Bool (leq_nat (length a xs) n) True → Equal (List a) (take a n xs) xs =
  (proof saturated for take) a n xs
"#;

const ACCEPTED: &str = r#"import Algorithm.Optimization.Argmin (least_k, cost_le, least_k_saturated)
import Data.Collections.Derived (take, sort)

data Candidate : Type where {
  A : Candidate;
  B : Candidate;
  C : Candidate;
  D : Candidate
}

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const four : Nat = Suc three
const nine : Nat = Suc (Suc (Suc (Suc (Suc four))))

fn seed_cost (x : Candidate) : Nat =
  match x {
    A ↦ two;
    B ↦ one;
    C ↦ two;
    D ↦ three
  }

const input : List Candidate =
  Cons Candidate A (Cons Candidate B (Cons Candidate C (Cons Candidate D (Nil Candidate))))

const sorted_input : List Candidate =
  Cons Candidate B (Cons Candidate A (Cons Candidate C (Cons Candidate D (Nil Candidate))))

theorem v_sort : Equal (List Candidate) (sort Candidate (cost_le Candidate seed_cost) input) sorted_input =
  Refl

theorem v_least_four : Equal (List Candidate) (least_k Candidate seed_cost four input) sorted_input =
  Refl

theorem v_least_nine : Equal (List Candidate) (least_k Candidate seed_cost nine input) sorted_input =
  Refl

theorem c_law_four
    : Equal (List Candidate) (least_k Candidate seed_cost four input)
        (sort Candidate (cost_le Candidate seed_cost) input) =
  least_k_saturated Candidate seed_cost four input Proved

theorem c_law_nine
    : Equal (List Candidate) (least_k Candidate seed_cost nine input)
        (sort Candidate (cost_le Candidate seed_cost) input) =
  least_k_saturated Candidate seed_cost nine input Proved
"#;

const X_INPUT: &str = r#"theorem r_input_unchanged : Equal (List Candidate) (least_k Candidate seed_cost four input) input =
  Refl
"#;

const X_LAW: &str = r#"theorem r_law_input : Equal (List Candidate) (least_k Candidate seed_cost four input) input =
  least_k_saturated Candidate seed_cost four input Proved
"#;

const X_BOUND: &str = r#"theorem r_short_bound
    : Equal (List Candidate) (least_k Candidate seed_cost three input)
        (sort Candidate (cost_le Candidate seed_cost) input) =
  least_k_saturated Candidate seed_cost three input Proved
"#;

/// SOURCE: spec/50-stdlib/72-optimization.md §4.6; Architect AC-0 D0.
/// PROMISE CLASS: durable invariant.
/// MEASURED: seven providers without a Derived dependency load first; NonEmpty,
/// which imports Derived, loads next; exact trusted GlobalId sets agree at
/// every stage, and both checked laws are owner-local
/// and Transparent, and open selective-import clients use each owner's Const.
/// CLAIMED: generic saturation adds no trust and consumers can apply both
/// public laws with precisely the specified bound.
/// THE GAP: inherited provider trust remains; a checked client does not
/// replace the catalog proof's kernel check.
#[test]
fn both_saturation_laws_are_public_generic_and_zero_new_trust() {
    let (mut env, before, after_derived, after_nonempty, after_argmin) = loaded();
    assert_eq!(after_derived, before, "Derived adds no trusted identity");
    assert_eq!(
        after_nonempty, after_derived,
        "NonEmpty adds no trusted identity"
    );
    assert_eq!(
        after_argmin, after_nonempty,
        "Argmin adds no trusted identity"
    );
    env.elaborate_file(OPEN)
        .expect("both generic selective-import clients check");
    for (client, owner) in [
        (
            "take_saturated_generic",
            format!("{DERIVED}.take::saturated"),
        ),
        (
            "least_k_saturated_generic",
            format!("{ARGMIN}.least_k_saturated"),
        ),
    ] {
        let owner_id = env.globals[&owner];
        let (_, body) = env
            .env
            .transparent_body(env.globals[client])
            .expect("open client has a checked proof body");
        assert!(has_const(&body, owner_id), "{client} cites {owner} Const");
    }
}

/// SOURCE: spec/50-stdlib/72-optimization.md §4.6; D0 closed client.
/// PROMISE CLASS: durable invariant with fixed closed-value controls.
/// MEASURED: complete accepted client checks stable-sort and least-k at k=4
/// and k=9, each returning [B,A,C,D], alongside both law applications.
/// CLAIMED: a saturated least-k returns the sorted list, not input order.
/// THE GAP: a finite fixture supplements but cannot prove the generic law.
#[test]
fn sorted_saturation_values_and_both_bound_cases_accept() {
    let (mut env, _, _, _, _) = loaded();
    env.elaborate_file(ACCEPTED)
        .expect("whole accepted sorted/least-k client checks");
    for name in [
        "v_sort",
        "v_least_four",
        "v_least_nine",
        "c_law_four",
        "c_law_nine",
    ] {
        assert!(
            matches!(
                env.env.lookup(env.globals[name]),
                Some(Decl::Transparent { .. })
            ),
            "{name} is a checked theorem"
        );
    }
}

/// SOURCE: spec/50-stdlib/72-optimization.md §4.6; D0 refused clients.
/// PROMISE CLASS: durable invariant.
/// MEASURED: each refusal follows a fresh complete accepted prefix. The
/// input-order claim rejects at its own Refl under ElabError::TypeMismatch;
/// both law-form false claims reject with KernelRejected(TypeMismatch) at
/// their own Proved, with errors contained in each parsed theorem span.
/// CLAIMED: saturation is not input-order preservation and k=3 violates
/// the bound on the same four-element input.
/// THE GAP: these closed refusals do not assert a converse of the bound.
#[test]
fn input_order_and_short_bound_refuse_at_their_own_claims() {
    for (name, row, terminal) in [
        ("r_input_unchanged", X_INPUT, "Refl"),
        ("r_law_input", X_LAW, "Proved"),
        ("r_short_bound", X_BOUND, "Proved"),
    ] {
        let (mut fresh, _, _, _, _) = loaded();
        fresh
            .elaborate_file(ACCEPTED)
            .unwrap_or_else(|error| panic!("{name} whole accepted prefix: {error:?}"));
        let parsed = parse_decls(row).expect("one isolated negative parses");
        let [SourceDecl::TheoremDecl {
            name: actual,
            span: claim,
            ..
        }] = parsed.as_slice()
        else {
            panic!("{name} must be one isolated theorem");
        };
        assert_eq!(actual, name);
        let error = fresh.elaborate_file(row).expect_err("false claim refuses");
        let span = match error {
            ElabError::TypeMismatch { reason, span }
                if terminal == "Refl" && reason == "Refl expects an `Eq`-shaped goal" =>
            {
                span
            }
            ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                span,
            } if terminal == "Proved" => span,
            other => panic!("{name} must refuse at the ruled error kind: {other:?}"),
        };
        let token = row.rfind(terminal).expect("own proof token");
        assert!(
            claim.start <= span.start
                && span.start <= token
                && token + terminal.len() <= span.end
                && span.end <= claim.end,
            "{name}: refusal must cover its own {terminal} in its own theorem: {span:?} vs {claim:?}"
        );
        if terminal == "Refl" {
            assert_eq!(&row[span.start..span.end], "Refl", "own terminal span");
        }
    }
}
