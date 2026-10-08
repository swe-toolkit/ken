//! Deterministic automata: checked laws, exact provider reuse, and execution.
//!
//! Contract: spec/50-stdlib/61-formal-languages.md §1.
//! Promise class: durable semantic invariants; the concrete words and
//! automata are fixed fixtures with independent expected Boolean results.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_interp::eval::{eval, EvalStore, EvalVal};
use ken_kernel::{Decl, GlobalId, Term};

const DFA: &str = "Algorithm.FormalLanguages.Dfa";
const LAWFUL: &str = "Core.Classes.LawfulClasses";
const DERIVED: &str = "Data.Collections.Derived";
const TRANSPORT: &str = "Core.Logic.Transport";
const MAP: &str = "Data.Collections.Map";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn provider_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    for module in [LAWFUL, TRANSPORT, DERIVED] {
        env.elaborate_module_from_roots(&[root()], module)
            .unwrap_or_else(|error| panic!("{module} provider must roots-load: {error:?}"));
    }
    env
}

fn loaded_dfa() -> (ElabEnv, Vec<GlobalId>) {
    let mut env = provider_env();
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    let owned = env
        .elaborate_module_from_roots(&[root()], DFA)
        .expect("Dfa and its seven proofs must roots-load");
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(before, after, "Dfa must add no local trust");
    (env, owned)
}

fn mentions(term: &Term, target: GlobalId) -> bool {
    match term {
        Term::Const { id, .. } | Term::IndFormer { id, .. } | Term::Constructor { id, .. }
            if *id == target =>
        {
            true
        }
        Term::Elim { fam, .. } if *fam == target => true,
        _ => term
            .children()
            .into_iter()
            .any(|child| mentions(child, target)),
    }
}

fn bool_result(env: &ElabEnv, name: &str) -> bool {
    let id = env.globals[name];
    let Some(Decl::Transparent { body, .. }) = env.env.lookup(id) else {
        panic!("{name} must be a checked, transparent fixture");
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

/// MEASURED: the real roots loader checks the seven public, transparent
/// theorems against the actual Dfa functions and package-owned identities;
/// its exact trusted-GlobalId set equals the already loaded provider set.
/// CLAIMED: all seven contract laws are checked, with zero local trust delta.
/// THE GAP: these artifacts prove their own propositions only; execution
/// vectors below independently exercise concrete acceptance and operation
/// wiring, not runtime complexity or finiteness of the carrier.
#[test]
fn seven_public_proofs_and_provider_identities_are_kernel_checked() {
    let (mut env, owned) = loaded_dfa();
    for law in [
        "run_append",
        "run_complement",
        "accepts_complement",
        "run_product",
        "accepts_product",
        "accepts_intersection",
        "accepts_union",
    ] {
        let id = env.globals[&format!("{DFA}.{law}")];
        assert!(owned.contains(&id), "{law} must belong to Dfa");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{law} must be kernel-checked and transparent"
        );
    }
    for operation in [
        "step",
        "start",
        "final",
        "run",
        "accepts",
        "complement",
        "product",
        "intersection",
        "union",
    ] {
        let id = env.globals[&format!("{DFA}.{operation}")];
        assert!(owned.contains(&id));
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    assert!(owned.contains(&env.globals[&format!("{DFA}.Dfa")]));
    for provider in [
        format!("{LAWFUL}.bool_and"),
        format!("{LAWFUL}.bool_or"),
        format!("{LAWFUL}.bool_not"),
        format!("{TRANSPORT}.cong"),
        format!("{DERIVED}.list_append"),
    ] {
        assert!(env.globals.contains_key(&provider), "missing {provider}");
        assert!(
            !owned.contains(&env.globals[&provider]),
            "Dfa must reuse {provider}, not claim its identity"
        );
    }
    let product = env.globals[&format!("{DFA}.product")];
    let combine = env.globals[&format!("{DFA}.accepts_product")];
    let (_, product_body) = env.env.transparent_body(product).expect("real product");
    let (_, combine_type) = env.env.const_type(combine).expect("real law");
    assert!(mentions(&combine_type, product));
    assert!(mentions(
        &product_body,
        env.globals[&format!("{DFA}.MkDfa")]
    ));

    env.elaborate_file(
        "import Algorithm.FormalLanguages.Dfa \
         (Dfa, MkDfa, step, start, final, run, accepts, complement, product, \
          intersection, union, run_append, run_complement, accepts_complement, \
          run_product, accepts_product, accepts_intersection, accepts_union)",
    )
    .expect("all prescribed public operations and theorems must be selectable");
    match env.elaborate_file("import Algorithm.FormalLanguages.Dfa (run_product_nil)") {
        Err(ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, format!("{DFA}.run_product_nil"));
        }
        Err(other) => panic!("private helper failed for another reason: {other:?}"),
        Ok(_) => panic!("run_product_nil must remain private"),
    }
}

/// MEASURED: checked, roots-loaded Dfa operations execute on two concrete
/// automata with different state types and six words covering all four
/// operand-result pairs. Each operand has an independent fixed expectation;
/// complement, intersection, union and both noncommutative projections are
/// compared to the Boolean operations on those observed operands.
/// CLAIMED: the constructions compute §1.2's acceptance meanings.
/// THE GAP: finite vectors do not replace the universally quantified proofs.
#[test]
fn complement_intersection_union_and_ordered_product_compute() {
    let (mut env, _) = loaded_dfa();
    let mut fixture = String::from(
        "import Algorithm.FormalLanguages.Dfa \
         (Dfa, MkDfa, accepts, complement, product, intersection, union)\n\
         import Core.Classes.LawfulClasses (bool_not, bool_and, bool_or)\n\
         const left : Dfa Nat Bool = MkDfa Nat Bool \
           (λs. λx. match x { True ↦ Suc s; False ↦ s }) \
           Zero (λs. match s { Zero ↦ False; Suc n ↦ True })\n\
         const right : Dfa Bool Bool = MkDfa Bool Bool \
           (λs. λx. match x { True ↦ s; False ↦ bool_not s }) \
           True (λs. s)\n\
         fn take_left (l : Bool) (r : Bool) : Bool = l\n\
         fn take_right (l : Bool) (r : Bool) : Bool = r\n",
    );
    let words = [
        ("empty", "Nil Bool", false, true),
        ("true", "Cons Bool True (Nil Bool)", true, true),
        ("false", "Cons Bool False (Nil Bool)", false, false),
        (
            "true_false",
            "Cons Bool True (Cons Bool False (Nil Bool))",
            true,
            false,
        ),
        (
            "false_true",
            "Cons Bool False (Cons Bool True (Nil Bool))",
            true,
            false,
        ),
        (
            "false_false",
            "Cons Bool False (Cons Bool False (Nil Bool))",
            false,
            true,
        ),
    ];
    for (label, word, _, _) in words {
        fixture.push_str(&format!(
            "const left_{label} : Bool = accepts Nat Bool left ({word})\n\
             const right_{label} : Bool = accepts Bool Bool right ({word})\n\
             const not_left_{label} : Bool = \
               accepts Nat Bool (complement Nat Bool left) ({word})\n\
             const not_right_{label} : Bool = \
               accepts Bool Bool (complement Bool Bool right) ({word})\n\
             const and_{label} : Bool = \
               accepts (Pair Nat Bool) Bool (intersection Nat Bool Bool left right) ({word})\n\
             const or_{label} : Bool = \
               accepts (Pair Nat Bool) Bool (union Nat Bool Bool left right) ({word})\n\
             const project_left_{label} : Bool = \
               accepts (Pair Nat Bool) Bool \
                 (product Nat Bool Bool take_left left right) ({word})\n\
             const project_right_{label} : Bool = \
               accepts (Pair Nat Bool) Bool \
                 (product Nat Bool Bool take_right left right) ({word})\n"
        ));
    }
    env.elaborate_file(&fixture)
        .expect("concrete Dfa clients must typecheck against public imports");
    for (label, _, expected_left, expected_right) in words {
        let got_left = bool_result(&env, &format!("left_{label}"));
        let got_right = bool_result(&env, &format!("right_{label}"));
        assert_eq!(got_left, expected_left, "left fixture: {label}");
        assert_eq!(got_right, expected_right, "right fixture: {label}");
        assert_eq!(bool_result(&env, &format!("not_left_{label}")), !got_left);
        assert_eq!(bool_result(&env, &format!("not_right_{label}")), !got_right);
        assert_eq!(
            bool_result(&env, &format!("and_{label}")),
            got_left && got_right
        );
        assert_eq!(
            bool_result(&env, &format!("or_{label}")),
            got_left || got_right
        );
        assert_eq!(
            bool_result(&env, &format!("project_left_{label}")),
            got_left
        );
        assert_eq!(
            bool_result(&env, &format!("project_right_{label}")),
            got_right
        );
    }
}

/// MEASURED: the actual roots-loaded Map has no local `bool_not` identity,
/// its checked definitions refer to LawfulClasses' exact public GlobalId,
/// and loading it after dependencies leaves the trusted set unchanged.
/// CLAIMED: Map's duplicate was removed without changing its Boolean
/// negation provider or trust boundary. THE GAP: targeted existing Map
/// suites, not this identity check alone, exercise its full consumer surface.
#[test]
fn map_consumes_public_bool_not_without_duplicate_or_trust_delta() {
    let mut env = provider_env();
    let canonical = env.globals[&format!("{LAWFUL}.bool_not")];
    assert!(matches!(
        env.env.lookup(canonical),
        Some(Decl::Transparent { .. })
    ));
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    let owned = env
        .elaborate_module_from_roots(&[root()], MAP)
        .expect("Map must load after importing public bool_not");
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(before, after, "Map must add no local trust");
    assert!(
        !env.globals.contains_key(&format!("{MAP}.bool_not")),
        "Map must not keep a private duplicate"
    );
    assert!(!owned.contains(&canonical));
    let hits = owned
        .iter()
        .filter_map(|id| env.env.transparent_body(*id))
        .filter(|(_, body)| mentions(body, canonical))
        .count();
    assert!(hits > 0, "Map's checked bodies must use LC bool_not");
    env.elaborate_file("import Core.Classes.LawfulClasses (bool_not)")
        .expect("bool_not must now be publicly importable");
}
