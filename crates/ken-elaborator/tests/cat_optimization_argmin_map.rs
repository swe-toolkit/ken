//! Spec 72 §§1.9 and 3.9: selection commutes with mapping candidates.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

const ARGMIN: &str = "Algorithm.Optimization.Argmin";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")
}

fn trusted_ids(env: &ElabEnv) -> BTreeSet<GlobalId> {
    env.env.trusted_base().into_iter().collect()
}

fn loaded() -> (ElabEnv, Vec<GlobalId>) {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Classes.LawfulClasses",
        "Core.Function.Combinators",
        "Core.Logic.Conjunction",
        "Core.Logic.Or",
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Collections.List",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("provider {provider} roots-loads: {error:?}"));
    }
    let providers = trusted_ids(&env);
    env.elaborate_module_from_roots(&[root()], "Data.Collections.Derived")
        .expect("Derived roots-loads");
    let derived = trusted_ids(&env);
    assert_eq!(providers, derived, "Derived adds no trusted identity");
    env.elaborate_module_from_roots(&[root()], "Data.Collections.NonEmpty")
        .expect("NonEmpty roots-loads after Derived");
    let nonempty = trusted_ids(&env);
    assert_eq!(derived, nonempty, "NonEmpty adds no trusted identity");
    let owned = env
        .elaborate_module_from_roots(&[root()], ARGMIN)
        .expect("Argmin roots-loads");
    let argmin = trusted_ids(&env);
    eprintln!(
        "Trusted GlobalIds (providers → Derived → NonEmpty → Argmin): {} → {} → {} → {}",
        providers.len(),
        derived.len(),
        nonempty.len(),
        argmin.len()
    );
    assert_eq!(nonempty, argmin, "Argmin adds no trusted identity");
    (env, owned)
}

fn assert_cites(env: &ElabEnv, client: &str, owner: GlobalId) {
    let (_, body) = env
        .env
        .transparent_body(env.globals[client])
        .expect("checked client body");
    let mut pending = vec![&body];
    while let Some(term) = pending.pop() {
        if matches!(term, Term::Const { id, .. } if *id == owner) {
            return;
        }
        pending.extend(term.children());
    }
    panic!("{client} must cite the owned law Const {owner:?}");
}

const OPEN: &str = r#"import Algorithm.Optimization.Argmin (argmin, argmax, argmin_map, argmax_map, argmin_option)
import Data.Collections.NonEmpty (NonEmpty, nonempty_cons, nonempty_map)
import Data.Collections.Derived (map)
import Core.Function.Combinators (comp)
import Core.Logic.Transport (cong)

theorem open_argmin_map
      (a : Type) (b : Type) (f : a → b) (cost : b → Nat) (xs : NonEmpty a)
    : Equal b (argmin b cost (nonempty_map a b f xs)) (f (argmin a (comp a b Nat cost f) xs)) =
  argmin_map a b f cost xs

theorem open_argmax_map
      (a : Type) (b : Type) (f : a → b) (cost : b → Nat) (xs : NonEmpty a)
    : Equal b (argmax b cost (nonempty_map a b f xs)) (f (argmax a (comp a b Nat cost f) xs)) =
  argmax_map a b f cost xs

theorem option_client
      (a : Type) (b : Type) (f : a → b) (cost : b → Nat) (x : a) (rest : List a)
    : Equal (Option b)
        (argmin_option b cost (map a b f (Cons a x rest)))
        (Some b (f (argmin a (comp a b Nat cost f) (nonempty_cons a x rest)))) =
  cong b (Option b)
    (argmin b cost (nonempty_map a b f (nonempty_cons a x rest)))
    (f (argmin a (comp a b Nat cost f) (nonempty_cons a x rest)))
    (Some b)
    (argmin_map a b f cost (nonempty_cons a x rest))
"#;

const FIXTURE: &str = r#"import Algorithm.Optimization.Argmin (argmin, argmax, argmin_map, argmax_map)
import Data.Collections.NonEmpty (NonEmpty, nonempty_cons, nonempty_map)
import Core.Function.Combinators (comp)

const one : Nat = Suc Zero
const two : Nat = Suc one

fn identity_cost (n : Nat) : Nat = n

fn reverse_three (n : Nat) : Nat =
  match n {
    Zero ↦ two;
    Suc m ↦
      match m {
        Zero ↦ one;
        Suc r ↦ Zero
      }
  }

const xs : NonEmpty Nat = nonempty_cons Nat Zero (Cons Nat one (Cons Nat two (Nil Nat)))
"#;

const COMPOSED: &str = "(comp Nat Nat Nat identity_cost reverse_three)";
const UNCOMPOSED: &str = "identity_cost";

fn law_claim(name: &str, op: &str, source_cost: &str) -> String {
    format!(
        "theorem {name} : Equal Nat
    ({op} Nat identity_cost (nonempty_map Nat Nat reverse_three xs))
    (reverse_three ({op} Nat {source_cost} xs)) =
  {op}_map Nat Nat reverse_three identity_cost xs
"
    )
}

fn accepted_prefix() -> ElabEnv {
    let (mut env, _) = loaded();
    let mut source = FIXTURE.to_owned();
    for (op, expected, other) in [("argmin", "Zero", "two"), ("argmax", "two", "Zero")] {
        let client = format!("{op}_law_at_seed");
        source.push_str(&law_claim(&client, op, COMPOSED));
        source.push_str(&format!(
            "theorem {op}_mapped : Equal Nat
    ({op} Nat identity_cost (nonempty_map Nat Nat reverse_three xs)) {expected} = Proved
theorem {op}_composed : Equal Nat
    (reverse_three ({op} Nat {COMPOSED} xs)) {expected} = Proved
theorem {op}_uncomposed : Equal Nat
    (reverse_three ({op} Nat {UNCOMPOSED} xs)) {other} = Proved
"
        ));
    }
    env.elaborate_file(&source)
        .expect("the complete accepted mapping prefix checks");
    for op in ["argmin", "argmax"] {
        assert_cites(
            &env,
            &format!("{op}_law_at_seed"),
            env.globals[&format!("{ARGMIN}.{op}_map")],
        );
    }
    env
}

fn assert_refuses(source: &str, declaration: &str, terminal: &str) {
    let mut env = accepted_prefix();
    let parsed = parse_decls(source).expect("the refusal parses");
    let [
        SourceDecl::TheoremDecl {
            name, span: claim, ..
        },
    ] = parsed.as_slice()
    else {
        panic!("expected exactly one refused theorem");
    };
    assert_eq!(name, declaration);
    let token = source.rfind(terminal).expect("closing proof token");
    match env.elaborate_file(source) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if claim.start <= span.start
            && span.start <= token
            && token + terminal.len() <= span.end
            && span.end <= claim.end =>
        {
            eprintln!("{declaration}: KernelRejected(TypeMismatch) at {span:?}");
        }
        other => panic!("must reject at {declaration} covering {terminal}: {other:?}"),
    }
}

/// Source: Spec 72 §§1.9/3.9 and Architect AC-0/AC-T. Durable invariant.
/// Measured: exact provider-first trust identities, ownership, Transparency,
/// and open selective-import clients that cite the two laws. The Option
/// client lifts the minimum law through `Some`. Claimed: the generic laws
/// compose without extra premises or trust. Gap: imported trust is not audited.
/// Proof rewrites are compatible; hiding a law or requiring injectivity is not.
#[test]
fn open_map_laws_are_owned_transparent_and_add_no_trust() {
    let (mut env, owned) = loaded();
    env.elaborate_file(OPEN)
        .expect("all three open clients check");
    for (law, clients) in [
        ("argmin_map", &["open_argmin_map", "option_client"][..]),
        ("argmax_map", &["open_argmax_map"][..]),
    ] {
        let name = format!("{ARGMIN}.{law}");
        let id = env.globals[&name];
        assert!(owned.contains(&id), "{name} belongs to Argmin");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
        for client in clients {
            assert_cites(&env, client, id);
        }
    }
}

/// Source: Spec 72 §§1.9/3.9 and AC-2. Durable invariant.
/// Measured: three value equations per selector and one checked application
/// per owned law. Claimed: composition gives the same answer as mapping
/// first; uncomposed selection gives the other extreme here. Gap: these
/// values are fixture-specific. Proof rewrites are compatible; losing the
/// reversed ranking or bypassing the laws does not preserve this evidence.
#[test]
fn reversed_costs_distinguish_mapped_composed_and_uncomposed_routes() {
    accepted_prefix();
}

/// Source: Spec 72 §§1.9/3.9 and AC-2. Durable invariant.
/// Measured: with fresh accepted prefixes, dropping only the composed cost
/// from each law instance rejects the original law application. Independent
/// closing claims also reject the composed result for the uncomposed route.
/// Claimed: composition cannot be dropped. Gap: the span locates the proof
/// declaration, not the mismatched subterm. Diagnostic wording can change;
/// accepting the false equation or failing at an earlier boundary cannot.
#[test]
fn uncomposed_cost_refuses_at_each_law_application_and_value_claim() {
    for (op, expected) in [("argmin", "Zero"), ("argmax", "two")] {
        let law_claim_name = format!("{op}_refused_law");
        assert_refuses(
            &law_claim(&law_claim_name, op, UNCOMPOSED),
            &law_claim_name,
            &format!("{op}_map"),
        );
        let value_claim_name = format!("{op}_refused_value");
        assert_refuses(
            &format!(
                "theorem {value_claim_name} : Equal Nat
    (reverse_three ({op} Nat {UNCOMPOSED} xs)) {expected} = Proved
"
            ),
            &value_claim_name,
            "Proved",
        );
    }
}

/// Source: AC-0 publication boundary. Durable invariant.
/// Measured: selective import of each qualified helper refuses with
/// `UnboundName`. Claimed: the three helpers are not public API. Gap: this
/// is not an export census. Private refactoring is compatible; making any
/// of these helpers importable is not.
#[test]
fn map_selection_helpers_are_private() {
    for helper in ["select_map", "choose_map", "select_from_map"] {
        let (mut env, _) = loaded();
        let source = format!("import {ARGMIN} ({helper})\n");
        match env.elaborate_file(&source) {
            Err(ElabError::UnboundName { name, .. }) if name == format!("{ARGMIN}.{helper}") => {}
            other => panic!("{helper} must refuse selective import: {other:?}"),
        }
    }
}
