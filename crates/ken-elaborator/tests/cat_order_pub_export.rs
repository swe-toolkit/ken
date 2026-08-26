//! CAT-ORDER-PUB-EXPORT D0/D1 acceptance controls.
//!
//! Promise class: durable invariants. The public Order facade must keep the
//! four operations it defines reachable at their provider `GlobalId`s while
//! carrying the canonical LawfulClasses relation and dictionary identities.
//! These controls use the roots loader, kernel artifacts, and class-registry
//! provenance; repository text and numeric allocation order are not oracles.

use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{GlobalId, Term};

const ORDER: &str = "Data.Numeric.Nat.Order";
const LAWFUL: &str = "Core.Classes.LawfulClasses";

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn load_order() -> ElabEnv {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_module_from_roots(&[catalog_root()], ORDER)
        .expect("Order facade must elaborate through the real roots loader");
    env
}

fn term_mentions(term: &Term, target: GlobalId) -> bool {
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
            .any(|child| term_mentions(child, target)),
    }
}

fn assert_transparent_body_mentions(env: &ElabEnv, wrapper: &str, provider: GlobalId) {
    let wrapper = env.globals[wrapper];
    let (_, body) = env
        .env
        .transparent_body(wrapper)
        .expect("consumer wrapper must remain kernel-checked and transparent");
    assert!(
        term_mentions(&body, provider),
        "consumer wrapper must retain the selected provider GlobalId"
    );
}

/// MEASURED: the real loader accepts one selective import containing every
/// Order-owned operation, each fully-qualified provider is transparent, and
/// wrappers retain the three Nat-returning provider identities. CLAIMED: the
/// facade publicly exposes all four operations it defines without aliases.
/// THE GAP: successful selected-item resolution is the loader's interface
/// oracle for `compare`; no second result-type or operation identity is minted.
#[test]
fn order_facade_exports_all_four_owned_operation_identities() {
    let mut env = load_order();
    let min = env.globals[&format!("{ORDER}.min")];
    let max = env.globals[&format!("{ORDER}.max")];
    let sub = env.globals[&format!("{ORDER}.sub")];
    let compare = env.globals[&format!("{ORDER}.compare")];
    for id in [min, max, sub, compare] {
        assert!(
            env.env.transparent_body(id).is_some(),
            "an Order-owned public operation must retain its transparent provider artifact"
        );
    }

    env.elaborate_file(
        "import Data.Numeric.Nat.Order (min, max, sub, compare)\n\
         fn cat_order_pub_min (x : Nat) (y : Nat) : Nat = min x y\n\
         fn cat_order_pub_max (x : Nat) (y : Nat) : Nat = max x y\n\
         fn cat_order_pub_sub (x : Nat) (y : Nat) : Nat = sub x y\n\
         theorem cat_order_pub_min_behavior\n\
           : Equal Nat (cat_order_pub_min Zero (Suc Zero)) Zero = Proved\n\
         theorem cat_order_pub_max_behavior\n\
           : Equal Nat (cat_order_pub_max Zero (Suc Zero)) (Suc Zero) = Proved\n\
         theorem cat_order_pub_sub_behavior\n\
           : Equal Nat (cat_order_pub_sub (Suc Zero) Zero) (Suc Zero) = Proved",
    )
    .expect("all four Order-owned operations must be selectively importable");

    assert_transparent_body_mentions(&env, "cat_order_pub_min", min);
    assert_transparent_body_mentions(&env, "cat_order_pub_max", max);
    assert_transparent_body_mentions(&env, "cat_order_pub_sub", sub);
    assert!(!env.globals.contains_key("min"));
    assert!(!env.globals.contains_key("max"));
    assert!(!env.globals.contains_key("sub"));
    assert!(!env.globals.contains_key("compare"));
}

/// MEASURED: loading Order also loads the fully-qualified LawfulClasses
/// totality bridge as a transparent artifact, while selecting it directly from
/// its provider remains an exact interface `UnboundName`. CLAIMED: the bridge
/// remains provider-private rather than becoming a public compatibility path.
/// THE GAP: privacy is observed through the real loader export table; kernel
/// transparency alone would not establish interface reachability.
#[test]
fn lawful_totality_bridge_remains_a_private_transparent_provider_artifact() {
    let mut env = load_order();
    let provider_name = format!("{LAWFUL}.total_leq_nat");
    let provider = env.globals[&provider_name];
    assert!(
        env.env.transparent_body(provider).is_some(),
        "the provider-private totality bridge must remain kernel-checked and transparent"
    );

    match env.elaborate_file("import Core.Classes.LawfulClasses (total_leq_nat)") {
        Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, provider_name),
        Err(other) => panic!("provider-private import must fail as UnboundName: {other:?}"),
        Ok(_) => panic!("provider-private totality bridge became selectively importable"),
    }
}

/// MEASURED: after the real roots loader closes Order, no fully-qualified
/// `Order.total_leq_nat` artifact exists. CLAIMED: the facade does not mint a
/// local totality bridge. THE GAP: the independent import-refusal control below
/// covers interface reachability; this control covers artifact existence.
#[test]
fn order_facade_does_not_mint_a_totality_bridge() {
    let env = load_order();
    assert!(
        !env.globals.contains_key(&format!("{ORDER}.total_leq_nat")),
        "Order must not mint a total_leq_nat artifact"
    );
}

/// MEASURED: the real loader rejects an Order-only selective import at the
/// exact fully-qualified target. CLAIMED: provider-private totality is absent
/// from the facade's public interface. THE GAP: the independent artifact
/// control above distinguishes export absence from a hidden Order-local alias.
#[test]
fn order_facade_does_not_export_the_totality_bridge() {
    let mut env = load_order();
    let forbidden = format!("{ORDER}.total_leq_nat");
    match env.elaborate_file("import Data.Numeric.Nat.Order (total_leq_nat)") {
        Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, forbidden),
        Err(other) => panic!("forbidden facade import must fail as UnboundName: {other:?}"),
        Ok(_) => panic!("Order.total_leq_nat became selectively importable"),
    }
}

/// MEASURED: an Order-only selective consumer retains the Order `sub`
/// `GlobalId`, the LawfulClasses `leq_nat` `GlobalId`, and the carried registry
/// dictionary provenance. CLAIMED: widening the local operation surface does
/// not disturb the canonical ownership migration. THE GAP: this is ordinary
/// compatibility-root evidence, not the deferred Strict Pair closure.
#[test]
fn selective_consumer_preserves_local_and_reexported_provider_identities() {
    let mut env = load_order();
    let order_sub = env.globals[&format!("{ORDER}.sub")];
    let lawful_leq = env.globals[&format!("{LAWFUL}.leq_nat")];
    let ord = env.globals["Ord"];
    let nat = env.globals["Nat"];
    let mut dictionaries = env
        .class_env
        .instances
        .iter()
        .filter(|((class, head), info)| {
            env.globals.get(class) == Some(&ord)
                && env.globals.get(head) == Some(&nat)
                && info.class_name == *class
        });
    let (_, dictionary) = dictionaries
        .next()
        .expect("the loader registry must contain canonical Ord Nat");
    assert!(
        dictionaries.next().is_none(),
        "the loader registry must contain exactly one canonical Ord Nat dictionary"
    );
    let dictionary_id = dictionary.instance_id;
    assert_eq!(dictionary.defining_package, LAWFUL);

    env.elaborate_file(
        "import Data.Numeric.Nat.Order (Ord, sub, leq_nat)\n\
         fn cat_order_pub_sub_identity (x : Nat) (y : Nat) : Nat = sub x y\n\
         fn cat_order_pub_leq_identity (x : Nat) (y : Nat) : Bool = leq_nat x y\n\
         fn cat_order_pub_dictionary (x : Nat) (y : Nat) : Bool where Ord Nat = d.leq x y\n\
         theorem cat_order_pub_sub_zero\n\
           : Equal Nat (cat_order_pub_sub_identity (Suc Zero) Zero) (Suc Zero) = Proved\n\
         theorem cat_order_pub_leq_zero_one\n\
           : Equal Bool (cat_order_pub_leq_identity Zero (Suc Zero)) True = Proved",
    )
    .expect("Order selective consumer must resolve local and re-exported identities");

    assert_transparent_body_mentions(&env, "cat_order_pub_sub_identity", order_sub);
    assert_transparent_body_mentions(&env, "cat_order_pub_leq_identity", lawful_leq);
    assert!(!env.globals.contains_key(&format!("{ORDER}.leq_nat")));
    assert!(
        !env.globals
            .contains_key(&format!("{ORDER}.bool_or::eq_true_of_or")),
        "Order must not mint a bridge alias"
    );

    let resolution = env
        .class_env
        .resolution_provenance
        .iter()
        .rev()
        .find(|resolution| resolution.class_name == "Ord" && resolution.head_type == "Nat")
        .expect("Order-only consumer must record Ord Nat resolution provenance");
    assert_eq!(resolution.instance_id, dictionary_id);
    assert_eq!(resolution.defining_package, LAWFUL);
}
