//! `LANG-MEMBERSHIP-OPERATOR-SURFACE` acceptance.
//!
//! The production seam is catalog provider registration → the standard
//! facade's identity certification → `RStandardOp` → A1's P2-extended
//! `resolve_instance_dictionary_inner` path. Fixtures use ordinary `.ken`
//! imports and `∈`; helper-level observations only inspect the checked core
//! term produced at the end of that path.

#[path = "support/catalog_or.rs"]
mod catalog_or;

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use ken_elaborator::error::ElabError;
use ken_elaborator::ElabEnv;
use ken_kernel::{Decl, GlobalId, Term};

const MEMBERSHIP: &str = "Core.Classes.Membership";
const ORDERED_SEARCH: &str = "Algorithm.Searching.OrderedSearch";
const MAP: &str = "Data.Collections.Map";
const STANDARD: &str = "Core.Operators.Standard";

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn catalog_env_with_owned() -> (ElabEnv, Vec<GlobalId>, Vec<GlobalId>, Vec<GlobalId>) {
    let root = catalog_root();
    let mut env = ElabEnv::new().expect("base environment");
    let ordered_owned = env
        .elaborate_module_from_roots(std::slice::from_ref(&root), ORDERED_SEARCH)
        .expect("OrderedSearch must roots-load");
    let map_owned = env
        .elaborate_module_from_roots(std::slice::from_ref(&root), MAP)
        .expect("Map must roots-load");
    env.elaborate_module_from_roots(std::slice::from_ref(&root), STANDARD)
        .expect("Standard must roots-load");
    let membership_owned = env
        .elaborate_module_from_roots(std::slice::from_ref(&root), MEMBERSHIP)
        .expect("Membership defining provider must roots-load");
    (env, ordered_owned, map_owned, membership_owned)
}

fn catalog_env() -> ElabEnv {
    catalog_env_with_owned().0
}

fn transparent_body(env: &ElabEnv, name: &str) -> Term {
    let (_, body) = env
        .env
        .transparent_body(env.globals[name])
        .unwrap_or_else(|| panic!("{name} must be transparent"));
    body
}

fn application_spine(mut term: &Term) -> (&Term, Vec<&Term>) {
    let mut args = Vec::new();
    while let Term::App(function, argument) = term {
        args.push(argument.as_ref());
        term = function.as_ref();
    }
    args.reverse();
    (term, args)
}

fn const_head(term: &Term) -> Option<GlobalId> {
    let (head, _) = application_spine(term);
    match head {
        Term::Const { id, .. } => Some(*id),
        _ => None,
    }
}

fn completed_dictionary(
    env: &ElabEnv,
    name: &str,
    membership_binding: GlobalId,
    carrier: GlobalId,
) -> GlobalId {
    let body = transparent_body(env, name);
    let mut observation = &body;
    while let Term::Lam(_, inner) = observation {
        observation = inner.as_ref();
    }
    let (head, args) = application_spine(observation);
    assert_eq!(
        head,
        &Term::const_(membership_binding, vec![]),
        "`{name}` must elaborate through the authenticated membership binding"
    );
    assert_eq!(
        args.len(),
        4,
        "membership completion supplies carrier and dictionary before the two written operands"
    );
    let (carrier_head, _) = application_spine(args[0]);
    assert!(
        matches!(carrier_head, Term::IndFormer { id, .. } if *id == carrier),
        "`{name}` must select the provider for its checked nominal carrier"
    );
    const_head(args[1]).expect("the completed dictionary must have a constant head")
}

fn membership_fixture() -> &'static str {
    r#"import Algorithm.Searching.OrderedSearch (ListMembership)
import Data.Collections.Map (Tree, OrderedKeyMembership, RelationEdgeMembership)
import Core.Operators.Standard (∈)

fn list_observed (view : ListMembership Nat) : Bool = Zero ∈ view
fn key_observed (view : OrderedKeyMembership Nat Unit) : Bool = Zero ∈ view
fn relation_observed (view : RelationEdgeMembership Nat) : Bool =
  mk_pair Nat Nat Zero (Suc Zero) ∈ view
fn comparator_observed (view : OrderedKeyMembership Nat Unit) : Bool = Suc Zero ∈ view
fn comparator_absent_observed (view : OrderedKeyMembership Nat Unit) : Bool =
  Suc (Suc Zero) ∈ view
fn relation_absent_source_observed (view : RelationEdgeMembership Nat) : Bool =
  mk_pair Nat Nat (Suc (Suc Zero)) (Suc Zero) ∈ view
fn relation_absent_target_observed (view : RelationEdgeMembership Nat) : Bool =
  mk_pair Nat Nat Zero (Suc (Suc Zero)) ∈ view
"#
}

/// Promise class: durable invariant.
///
/// MEASURED: seven ordinary public-import `∈` wrappers elaborate through
/// the one authenticated Membership binding; the checked carrier head picks
/// one of three different dictionaries from the real providers' owned ID
/// populations. List and ordered-key views share query `Nat`, so their
/// distinct selection cannot be keyed on the query alone. CLAIMED: the
/// operator admits each public view and chooses by carrier, not query.
/// THE GAP: Map's closed stored-vs-fresh comparator and edge witnesses live
/// in its checked provider-local examples, not in this abstract client.
#[test]
fn three_named_providers_are_carrier_first_on_public_views() {
    let (mut env, ordered_owned, map_owned, membership_owned) = catalog_env_with_owned();
    let binding =
        catalog_or::provider_owned_id(&env, &membership_owned, MEMBERSHIP, "membership_member_at")
            .expect("Membership must own the checked operator binding");
    let list_carrier =
        catalog_or::provider_owned_id(&env, &ordered_owned, ORDERED_SEARCH, "ListMembership")
            .expect("OrderedSearch must own the public list view");
    let key_carrier = catalog_or::provider_owned_id(&env, &map_owned, MAP, "OrderedKeyMembership")
        .expect("Map must own the public key view");
    let relation_carrier =
        catalog_or::provider_owned_id(&env, &map_owned, MAP, "RelationEdgeMembership")
            .expect("Map must own the public relation view");
    env.elaborate_file(membership_fixture())
        .expect("all three public nominal views must elaborate without private aliases");

    let list = completed_dictionary(&env, "list_observed", binding, list_carrier);
    let key = completed_dictionary(&env, "key_observed", binding, key_carrier);
    let relation = completed_dictionary(&env, "relation_observed", binding, relation_carrier);
    assert!(
        ordered_owned.contains(&list),
        "list instance must belong to OrderedSearch"
    );
    assert!(map_owned.contains(&key), "key instance must belong to Map");
    assert!(
        map_owned.contains(&relation),
        "edge instance must belong to Map"
    );
    assert_ne!(list, key, "same Query Nat must not select by the query");
    assert_ne!(list, relation);
    assert_ne!(
        key, relation,
        "distinct Map carriers need distinct instances"
    );
    for name in ["comparator_observed", "comparator_absent_observed"] {
        assert_eq!(
            completed_dictionary(&env, name, binding, key_carrier),
            key,
            "both abstract ordered-key uses must choose Map's key instance"
        );
    }
    for name in [
        "relation_absent_source_observed",
        "relation_absent_target_observed",
    ] {
        assert_eq!(
            completed_dictionary(&env, name, binding, relation_carrier),
            relation,
            "all abstract edge uses must choose Map's edge instance"
        );
    }
    for name in [
        "list_observed",
        "key_observed",
        "relation_observed",
        "comparator_observed",
        "comparator_absent_observed",
        "relation_absent_source_observed",
        "relation_absent_target_observed",
    ] {
        let id = env.globals[name];
        let mut ty = match env.env.lookup(id).expect("declared result") {
            Decl::Transparent { ty, .. } => ty,
            other => panic!("{name} must be transparent, got {other:?}"),
        };
        while let Term::Pi(_, result) = ty {
            ty = result.as_ref();
        }
        assert!(
            matches!(ty, Term::IndFormer { id, .. } if *id == env.numeric_env.bool_id),
            "{name} must return Bool, got {ty:?}"
        );
    }
}

/// Promise class: durable invariant.
///
/// The facade republishes the defining identity. It creates no declaration of
/// its own, and the global environment contains exactly one declaration with
/// that identity.
#[test]
fn membership_binding_is_defined_once_and_the_facade_reexports_its_identity() {
    let root = catalog_root();
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_module_from_roots(&[root], STANDARD)
        .expect("the production facade must roots-load");

    let canonical = format!("{MEMBERSHIP}.membership_member_at");
    let id = env.globals[&canonical];
    env.elaborate_file(
        "import Core.Operators.Standard (∈) \
         const imported_membership_binding = (∈)",
    )
    .expect("the facade route must import as an ordinary binding identity");
    assert_eq!(
        transparent_body(&env, "imported_membership_binding"),
        Term::const_(id, vec![]),
        "the facade must republish the defining GlobalId"
    );
    assert!(
        !env.globals.contains_key("Core.Operators.Standard.∈"),
        "re-export must not mint a facade declaration"
    );
    let defining_declarations = env
        .env
        .declarations()
        .iter()
        .filter(|decl| decl.id() == id)
        .count();
    assert_eq!(
        defining_declarations, 1,
        "membership_member_at must have exactly one defining declaration"
    );
}

/// Promise class: durable structural invariant.
///
/// Removing the membership export from an otherwise complete home must fail
/// in layer 3's required-role check and name `∈`. A later builtin lowering
/// cannot make this green because the occurrence is never reached.
#[test]
fn removing_the_catalog_role_fails_required_certification_naming_member() {
    let mut env = ElabEnv::new().expect("base environment");
    let result = env.elaborate_file(
        "class Ord a { leq : a -> a -> Bool } \
         class Membership c { Query : Type; member : Query -> c -> Bool } \
         module Provider { \
           pub fn bool_and (a : Bool) (b : Bool) : Bool = a \
           pub fn bool_or (a : Bool) (b : Bool) : Bool = a \
           pub fn ord_leq_at (a : Type) (d : Ord a) (x : a) (y : a) : Bool = d.leq x y \
           pub fn ord_geq_at (a : Type) (d : Ord a) (x : a) (y : a) : Bool = d.leq y x \
           pub fn membership_member_at \
             (c : Type) (d : Membership c) (q : d.Query) (x : c) : Bool = d.member q x \
         } \
         module Core.Operators.Standard { \
           export Provider (bool_and as ∧, bool_or as ∨, ord_leq_at as ≤, ord_geq_at as ≥) \
         }",
    );
    match result {
        Err(ElabError::StandardOperatorRoleUnfilled { role, home, .. }) => {
            assert_eq!(role, "∈");
            assert_eq!(home, STANDARD);
        }
        other => panic!("removing Member must fail the required-role check: {other:?}"),
    }
}

/// Promise class: durable invariant.
///
/// Raw `Tree` has no canonical membership meaning, while a public nominal
/// ordered-key parameter admits the same query through `∈`. Both forms use
/// only ordinary public imports; Map's private constructor/value witnesses
/// are checked in the provider's own example fence.
#[test]
fn raw_tree_is_refused_while_the_ordered_key_view_is_admitted() {
    let (mut env, _, map_owned, _) = catalog_env_with_owned();
    for name in ["Tree", "OrderedKeyMembership"] {
        catalog_or::provider_owned_id(&env, &map_owned, MAP, name)
            .unwrap_or_else(|error| panic!("Map carrier {name}: {error}"));
    }
    env.elaborate_file(
        "import Data.Collections.Map (OrderedKeyMembership) \
         import Core.Operators.Standard (∈) \
         fn public_key_member (view : OrderedKeyMembership Nat Unit) : Bool = \
           Suc Zero ∈ view",
    )
    .expect("the public ordered-key parameter must admit the operator");

    let result = env.elaborate_file(
        "import Data.Collections.Map (Tree) \
         import Core.Operators.Standard (∈) \
         fn raw_tree_member (tree : Tree Nat Unit) : Bool = Zero ∈ tree",
    );
    assert!(
        matches!(
            result,
            Err(ElabError::NoInstance { ref class, .. }) if class == "Membership"
        ),
        "raw Tree membership must fail as an ordinary missing instance: {result:?}"
    );
}

/// Promise class: durable invariant.
///
/// A second provider for one carrier is rejected as an overlap before an
/// occurrence can become iteration-order dependent. A real provider which is
/// present in the coherence registry but not directly admitted is rejected at
/// the occurrence as `UnadmittedInstance`; it is never silently selected.
#[test]
fn overlap_and_unadmitted_provider_are_ordinary_instance_errors() {
    let mut overlap = catalog_env();
    let duplicate = overlap.elaborate_file(
        "data DoubleProvider = MkDoubleProvider \
         instance Membership DoubleProvider { \
           Query = Nat; member = λq.λx.True \
         } \
         instance Membership DoubleProvider { \
           Query = Nat; member = λq.λx.False \
         }",
    );
    match duplicate {
        Err(ElabError::OverlappingInstances {
            class, head_type, ..
        }) => {
            assert_eq!(class, "Membership");
            assert_eq!(head_type, "DoubleProvider");
        }
        other => panic!("two canonical providers must be an overlap: {other:?}"),
    }

    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock after epoch")
        .as_nanos();
    let root = std::env::temp_dir().join(format!(
        "ken-membership-unadmitted-{}-{nonce}",
        std::process::id()
    ));
    fs::create_dir_all(&root).expect("create membership fixture root");
    for directory in ["Core", "Data"] {
        std::os::unix::fs::symlink(catalog_root().join(directory), root.join(directory))
            .unwrap_or_else(|error| panic!("link real catalog `{directory}`: {error}"));
    }
    fs::write(
        root.join("Provider.ken"),
        "import Core.Classes.Membership (Membership) \
         data HiddenMembership = MkHiddenMembership \
         export HiddenMembership, MkHiddenMembership \
         instance Membership HiddenMembership { \
           Query = Nat; member = λq.λx.True \
         }",
    )
    .expect("write provider fixture");
    fs::write(
        root.join("Entry.ken"),
        "program admits Core.Operators.Standard \
         import Core.Operators.Standard (∈) \
         import Provider (HiddenMembership, MkHiddenMembership) \
         const hidden_provider_use : Bool = Zero ∈ MkHiddenMembership",
    )
    .expect("write entry fixture");

    let mut unadmitted = ElabEnv::new().expect("base environment");
    let result = unadmitted.elaborate_module_from_roots(std::slice::from_ref(&root), "Entry");
    let _ = fs::remove_dir_all(&root);
    match result {
        Err(ElabError::UnadmittedInstance {
            defining_package,
            class,
            head_type,
            ..
        }) => {
            assert_eq!(defining_package, "Provider");
            assert_eq!(class, "Membership");
            assert_eq!(head_type, "Provider.HiddenMembership");
        }
        other => panic!("an unadmitted provider must refuse at the occurrence: {other:?}"),
    }
}

/// Promise class: durable invariant.
///
/// The new class, binding, views, and instances are ordinary checked
/// declarations. None enters the trusted base; `member_holds` goes from Bool
/// to Omega through `IsTrue`, never from Omega back to Bool.
#[test]
fn membership_surface_adds_no_trusted_declaration() {
    let env = catalog_env();
    let trusted = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();

    let member_holds = transparent_body(&env, "Core.Classes.Membership.member_holds");
    let mut observation = &member_holds;
    for _ in 0..4 {
        match observation {
            Term::Lam(_, body) => observation = body.as_ref(),
            other => panic!("member_holds must bind four parameters, got {other:?}"),
        }
    }
    let (head, args) = application_spine(observation);
    assert_eq!(
        head,
        &Term::const_(env.globals["Core.Classes.LawfulClasses.IsTrue"], vec![]),
        "member_holds must lift the Bool result through IsTrue"
    );
    assert_eq!(
        args.len(),
        1,
        "IsTrue takes the provider's Bool observation"
    );

    for name in [
        "Membership",
        "Core.Classes.Membership.membership_member_at",
        "Core.Classes.Membership.member_holds",
        "Core.Classes.Membership.same_members",
        "Algorithm.Searching.OrderedSearch.ListMembership",
        "Algorithm.Searching.OrderedSearch.MkListMembership",
        "Algorithm.Searching.OrderedSearch.list_membership_member",
        "Algorithm.Searching.OrderedSearch.list_membership_adapter_fidelity",
        "Membership_instance_Algorithm.Searching.OrderedSearch.ListMembership",
        "Data.Collections.Map.ordered_by",
        "Data.Collections.Map.OrderedKeyMembership",
        "Data.Collections.Map.MkOrderedKeyMembership",
        "Data.Collections.Map.ordered_key_membership_member",
        "Data.Collections.Map.ordered_key_membership_adapter_fidelity",
        "Membership_instance_Data.Collections.Map.OrderedKeyMembership",
        "Data.Collections.Map.successors_ordered",
        "Data.Collections.Map.RelationEdgeMembership",
        "Data.Collections.Map.MkRelationEdgeMembership",
        "Data.Collections.Map.relation_edge_membership_member",
        "Data.Collections.Map.relation_edge_membership_adapter_fidelity",
        "Membership_instance_Data.Collections.Map.RelationEdgeMembership",
    ] {
        let id = env.globals[name];
        assert!(
            !trusted.contains(&id),
            "new checked declaration `{name}` must not enter trusted_base()"
        );
    }
}
