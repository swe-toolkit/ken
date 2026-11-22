//! Spec 72 §4.7: least-k selection composes across a bounded append.
//! Promise class: durable invariant. Open clients and exact trust identities
//! pin the generic laws; closed values and same-form false claim distinguish
//! sorting the append from merely taking the appended selections.

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
    env.elaborate_module_from_roots(&[root()], "Data.Collections.NonEmpty")
        .expect("NonEmpty roots-loads after Derived");
    let argmin_owned = env
        .elaborate_module_from_roots(&[root()], ARGMIN)
        .expect("Argmin roots-loads");
    let after_argmin = env.env.trusted_base().into_iter().collect();
    for (module, law, owned) in [
        (DERIVED, "sort::take_append", &derived_owned),
        (ARGMIN, "least_k_append_merge", &argmin_owned),
    ] {
        let qualified = format!("{module}.{law}");
        let id = env.globals[&qualified];
        assert!(owned.contains(&id), "{qualified} is declared by its owner");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{qualified} is a checked Transparent proof"
        );
    }
    (env, before, after_derived, after_argmin)
}

fn has_const(body: &Term, id: GlobalId) -> bool {
    let mut pending = vec![body];
    while let Some(term) = pending.pop() {
        if matches!(term, Term::Const { id: found, .. } if *found == id) {
            return true;
        }
        pending.extend(term.children());
    }
    false
}

const OPEN: &str = r#"import Algorithm.Optimization.Argmin (least_k, least_k_append_merge)
import Data.Collections.Derived (sort, take, list_append)
import Core.Classes.LawfulClasses (IsTrue, bool_or)

theorem open_least_k_append
      (a : Type) (cost : a → Nat) (k : Nat) (xs : List a) (ys : List a)
    : Equal (List a)
        (least_k a cost k (list_append a xs ys))
        (least_k a cost k (list_append a (least_k a cost k xs) (least_k a cost k ys))) =
  least_k_append_merge a cost k xs ys

theorem open_sort_take_append
      (a : Type) (le : a → a → Bool)
      (total : (x : a) → (y : a) → IsTrue (bool_or (le x y) (le y x)))
      (transitive : (x : a) → (y : a) → (z : a)
        → Equal Bool (le x y) True → Equal Bool (le y z) True → Equal Bool (le x z) True)
      (k : Nat) (xs : List a) (ys : List a)
    : Equal (List a)
        (take a k (sort a le (list_append a xs ys)))
        (take a k (sort a le (list_append a (take a k (sort a le xs)) (take a k (sort a le ys))))) =
  (proof take_append for sort) a le total transitive k xs ys
"#;

const ACCEPTED: &str = r#"import Algorithm.Optimization.Argmin (least_k, least_k_append_merge)
import Data.Collections.Derived (list_append, take)

data Candidate : Type where {
  A : Candidate;
  B : Candidate;
  C : Candidate;
  D : Candidate
}

const one : Nat = Suc Zero
const two : Nat = Suc one

fn seed_cost (x : Candidate) : Nat =
  match x {
    A ↦ two;
    B ↦ one;
    C ↦ one;
    D ↦ Zero
  }

fn tie_cost (x : Candidate) : Nat = one

const left : List Candidate = Cons Candidate A (Cons Candidate B (Nil Candidate))
const right : List Candidate = Cons Candidate C (Cons Candidate D (Nil Candidate))
const selected_left : List Candidate = Cons Candidate B (Cons Candidate A (Nil Candidate))
const selected_right : List Candidate = Cons Candidate D (Cons Candidate C (Nil Candidate))
const selected_all : List Candidate = Cons Candidate D (Cons Candidate B (Nil Candidate))
const tie_left : List Candidate = Cons Candidate A (Nil Candidate)
const tie_right : List Candidate = Cons Candidate C (Nil Candidate)

theorem v_left : Equal (List Candidate) (least_k Candidate seed_cost two left) selected_left = Refl
theorem v_right : Equal (List Candidate) (least_k Candidate seed_cost two right) selected_right = Refl
theorem v_all : Equal (List Candidate)
    (least_k Candidate seed_cost two (list_append Candidate left right)) selected_all = Refl
theorem v_merge : Equal (List Candidate)
    (least_k Candidate seed_cost two
      (list_append Candidate (least_k Candidate seed_cost two left) (least_k Candidate seed_cost two right)))
    selected_all = Refl

theorem accepted_merge : Equal (List Candidate)
    (least_k Candidate seed_cost two (list_append Candidate left right))
    (least_k Candidate seed_cost two
      (list_append Candidate (least_k Candidate seed_cost two left) (least_k Candidate seed_cost two right))) =
  least_k_append_merge Candidate seed_cost two left right

theorem v_tie : Equal (List Candidate)
    (least_k Candidate tie_cost one (list_append Candidate tie_left tie_right)) tie_left = Refl

theorem accepted_tie : Equal (List Candidate)
    (least_k Candidate tie_cost one (list_append Candidate tie_left tie_right))
    (least_k Candidate tie_cost one
      (list_append Candidate (least_k Candidate tie_cost one tie_left)
        (least_k Candidate tie_cost one tie_right))) =
  least_k_append_merge Candidate tie_cost one tie_left tie_right
"#;

const REFUSED: &str = r#"theorem refused_take_instead_of_reselect : Equal (List Candidate)
    (least_k Candidate seed_cost two (list_append Candidate left right))
    (take Candidate two
      (list_append Candidate (least_k Candidate seed_cost two left) (least_k Candidate seed_cost two right))) =
  least_k_append_merge Candidate seed_cost two left right
"#;

/// SOURCE: spec/50-stdlib/72-optimization.md §4.7; Architect AC-0.
/// PROMISE CLASS: durable invariant. MEASURED: seven providers before Derived
/// and NonEmpty, then Argmin, with exact trusted GlobalId set equality at the
/// Derived and final Argmin boundaries; both laws are owner-local Transparent
/// and open selective-import clients contain their respective owner Const.
/// CLAIMED: public generic laws add no trust and expose the stated types.
/// THE GAP: inherited provider trust remains; root-only loading is not measured.
#[test]
fn both_public_laws_are_generic_and_zero_new_trust() {
    let (mut env, before, after_derived, after_argmin) = loaded();
    assert_eq!(before, after_derived, "Derived adds no trusted identity");
    assert_eq!(
        after_derived, after_argmin,
        "Argmin adds no trusted identity"
    );
    env.elaborate_file(OPEN)
        .expect("both open selective-import law clients check");
    for (client, owner) in [
        (
            "open_sort_take_append",
            format!("{DERIVED}.sort::take_append"),
        ),
        (
            "open_least_k_append",
            format!("{ARGMIN}.least_k_append_merge"),
        ),
    ] {
        let (_, body) = env
            .env
            .transparent_body(env.globals[client])
            .expect("open client has checked body");
        assert!(
            has_const(&body, env.globals[&owner]),
            "{client} cites exact owner Const"
        );
    }
}

/// SOURCE: spec/50-stdlib/72-optimization.md §4.7; AC-0 fixture.
/// PROMISE CLASS: durable invariant. MEASURED: both part selections yield
/// [B,A] and [D,C], and both append-and-select routes yield [D,B] by closed
/// Refl; the law applications and stable tie winner [A] check separately.
/// CLAIMED: the law reselects, rather than taking the appended part results.
/// THE GAP: finite fixtures do not prove the open theorem.
#[test]
fn both_routes_reselect_d_b_and_ties_keep_left_a() {
    let (mut env, _, _, _) = loaded();
    env.elaborate_file(ACCEPTED)
        .expect("complete closed fixture and both law applications check");
    for name in [
        "v_left",
        "v_right",
        "v_all",
        "v_merge",
        "accepted_merge",
        "v_tie",
        "accepted_tie",
    ] {
        assert!(
            matches!(
                env.env.lookup(env.globals[name]),
                Some(Decl::Transparent { .. })
            ),
            "{name} is checked"
        );
    }
}

/// SOURCE: spec/50-stdlib/72-optimization.md §4.7; AC-0 refused form.
/// PROMISE CLASS: durable invariant. MEASURED: a fresh accepted prefix checks;
/// only the right endpoint changes to take two of the two selected parts,
/// yielding [B,A], not [D,B]. The same law application rejects as a kernel
/// TypeMismatch in its own theorem span containing its proof expression.
/// CLAIMED: append cannot omit re-selection after part selections.
/// THE GAP: declaration-level span containment does not localize a subterm.
#[test]
fn same_law_cannot_justify_taking_unreselected_parts() {
    let (mut env, _, _, _) = loaded();
    env.elaborate_file(ACCEPTED).expect("fresh accepted prefix");
    let parsed = parse_decls(REFUSED).expect("refusal parses");
    let [SourceDecl::TheoremDecl {
        name, span: claim, ..
    }] = parsed.as_slice()
    else {
        panic!("one isolated refused theorem");
    };
    assert_eq!(name, "refused_take_instead_of_reselect");
    let token = REFUSED
        .rfind("least_k_append_merge")
        .expect("law proof token");
    match env
        .elaborate_file(REFUSED)
        .expect_err("false append selection rejects")
    {
        ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        } if claim.start <= span.start
            && span.start <= token
            && token + "least_k_append_merge".len() <= span.end
            && span.end <= claim.end => {}
        other => panic!("must reject at its own wrong law application: {other:?}"),
    }
}
