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

fn catalog_env() -> ElabEnv {
    let root = catalog_root();
    let mut env = ElabEnv::new().expect("base environment");
    for module in [ORDERED_SEARCH, MAP, STANDARD] {
        env.elaborate_module_from_roots(std::slice::from_ref(&root), module)
            .unwrap_or_else(|error| panic!("{module} must roots-load: {error:?}"));
    }
    env
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

fn completed_dictionary(env: &ElabEnv, name: &str) -> GlobalId {
    let body = transparent_body(env, name);
    let (head, args) = application_spine(&body);
    assert_eq!(
        head,
        &Term::const_(
            env.globals["Core.Classes.Membership.membership_member_at"],
            vec![]
        ),
        "`{name}` must elaborate through the one catalog membership binding"
    );
    assert_eq!(
        args.len(),
        4,
        "membership completion supplies carrier and dictionary before the two written operands"
    );
    const_head(args[1]).expect("the completed dictionary must have a constant head")
}

fn membership_fixture() -> &'static str {
    r#"
import Algorithm.Searching.OrderedSearch (ListMembership, MkListMembership)
import Data.Collections.Map
  (Tree, OrderedKeyMembership, MkOrderedKeyMembership,
   RelationEdgeMembership, MkRelationEdgeMembership)
import Core.Classes.LawfulClasses (Ord, leq_nat)
import Core.Operators.Standard (∈)

fn down_leq (x : Nat) (y : Nat) : Bool = leq_nat y x

fn down_below_zero (k2 : Nat) : Prop = Equal Bool (down_leq k2 Zero) True
fn down_above_zero (k2 : Nat) : Prop = Equal Bool (down_leq Zero k2) True
fn down_below_suc (k2 : Nat) : Prop = Equal Bool (down_leq k2 (Suc Zero)) True
fn down_above_suc (k2 : Nat) : Prop = Equal Bool (down_leq (Suc Zero) k2) True

const down_ord : Ord Nat = {
  leq = down_leq,
  refl = λx.(Ord_instance_Nat).refl x,
  antisym = λx.λy.λxy.λyx.(Ord_instance_Nat).antisym x y yx xy,
  trans = λx.λy.λz.λxy.λyz.(Ord_instance_Nat).trans z y x yz xy,
  total = λx.λy.(Ord_instance_Nat).total y x
}

const list_view : ListMembership Nat =
  MkListMembership Nat Ord_instance_Nat (Cons Nat Zero (Nil Nat))

const empty_key_view : OrderedKeyMembership Nat Unit =
  MkOrderedKeyMembership Nat Unit Ord_instance_Nat (Leaf Nat Unit) Proved

const down_left : Tree Nat Unit =
  Node Nat Unit (Leaf Nat Unit) (Suc Zero) MkUnit (Leaf Nat Unit)

const down_tree : Tree Nat Unit =
  Node Nat Unit down_left Zero MkUnit (Leaf Nat Unit)

theorem down_left_ordered : Ordered Nat Unit down_leq down_left =
  and_intro
    (all_keys Nat Unit down_below_suc (Leaf Nat Unit))
    (And
      (all_keys Nat Unit down_above_suc (Leaf Nat Unit))
      (And
        (Ordered Nat Unit down_leq (Leaf Nat Unit))
        (Ordered Nat Unit down_leq (Leaf Nat Unit))))
    Proved
    (and_intro
      (all_keys Nat Unit down_above_suc (Leaf Nat Unit))
      (And
        (Ordered Nat Unit down_leq (Leaf Nat Unit))
        (Ordered Nat Unit down_leq (Leaf Nat Unit)))
      Proved
      (and_intro
        (Ordered Nat Unit down_leq (Leaf Nat Unit))
        (Ordered Nat Unit down_leq (Leaf Nat Unit))
        Proved
        Proved))

theorem down_left_below_root
    : all_keys Nat Unit down_below_zero down_left =
  and_intro
    (Equal Bool (down_leq (Suc Zero) Zero) True)
    (And
      (all_keys Nat Unit down_below_zero (Leaf Nat Unit))
      (all_keys Nat Unit down_below_zero (Leaf Nat Unit)))
    Proved
    (and_intro
      (all_keys Nat Unit down_below_zero (Leaf Nat Unit))
      (all_keys Nat Unit down_below_zero (Leaf Nat Unit))
      Proved
      Proved)

theorem down_tree_ordered : Ordered Nat Unit down_leq down_tree =
  and_intro
    (all_keys Nat Unit down_below_zero down_left)
    (And
      (all_keys Nat Unit down_above_zero (Leaf Nat Unit))
      (And
        (Ordered Nat Unit down_leq down_left)
        (Ordered Nat Unit down_leq (Leaf Nat Unit))))
    down_left_below_root
    (and_intro
      (all_keys Nat Unit down_above_zero (Leaf Nat Unit))
      (And
        (Ordered Nat Unit down_leq down_left)
        (Ordered Nat Unit down_leq (Leaf Nat Unit)))
      Proved
      (and_intro
        (Ordered Nat Unit down_leq down_left)
        (Ordered Nat Unit down_leq (Leaf Nat Unit))
        down_left_ordered
        Proved))

const down_key_view : OrderedKeyMembership Nat Unit =
  MkOrderedKeyMembership Nat Unit down_ord down_tree down_tree_ordered

const down_adjacency : Tree Nat (Tree Nat Unit) =
  Node
    Nat
    (Tree Nat Unit)
    (Leaf Nat (Tree Nat Unit))
    Zero
    down_tree
    (Leaf Nat (Tree Nat Unit))

theorem down_adjacency_ordered
    : Ordered Nat (Tree Nat Unit) down_leq down_adjacency =
  and_intro
    (all_keys Nat (Tree Nat Unit) down_below_zero (Leaf Nat (Tree Nat Unit)))
    (And
      (all_keys Nat (Tree Nat Unit) down_above_zero (Leaf Nat (Tree Nat Unit)))
      (And
        (Ordered Nat (Tree Nat Unit) down_leq (Leaf Nat (Tree Nat Unit)))
        (Ordered Nat (Tree Nat Unit) down_leq (Leaf Nat (Tree Nat Unit)))))
    Proved
    (and_intro
      (all_keys Nat (Tree Nat Unit) down_above_zero (Leaf Nat (Tree Nat Unit)))
      (And
        (Ordered Nat (Tree Nat Unit) down_leq (Leaf Nat (Tree Nat Unit)))
        (Ordered Nat (Tree Nat Unit) down_leq (Leaf Nat (Tree Nat Unit))))
      Proved
      (and_intro
        (Ordered Nat (Tree Nat Unit) down_leq (Leaf Nat (Tree Nat Unit)))
        (Ordered Nat (Tree Nat Unit) down_leq (Leaf Nat (Tree Nat Unit)))
        Proved
        Proved))

theorem down_successors_ordered
    : successors_ordered Nat down_ord down_adjacency =
  and_intro
    (ordered_by Nat Unit down_ord down_tree)
    (And
      (successors_ordered Nat down_ord (Leaf Nat (Tree Nat Unit)))
      (successors_ordered Nat down_ord (Leaf Nat (Tree Nat Unit))))
    down_tree_ordered
    (and_intro
      (successors_ordered Nat down_ord (Leaf Nat (Tree Nat Unit)))
      (successors_ordered Nat down_ord (Leaf Nat (Tree Nat Unit)))
      Proved
      Proved)

const down_relation_view : RelationEdgeMembership Nat =
  MkRelationEdgeMembership
    Nat
    down_ord
    down_adjacency
    down_adjacency_ordered
    down_successors_ordered

const list_observed : Bool = Zero ∈ list_view
const key_observed : Bool = Zero ∈ empty_key_view
const relation_observed : Bool =
  mk_pair Nat Nat Zero (Suc Zero) ∈ down_relation_view
const comparator_observed : Bool = Suc Zero ∈ down_key_view

const wrong_comparator_observed : Bool =
  member Nat Unit (Ord_instance_Nat).leq (Suc Zero) down_tree

const wrong_relation_comparator_observed : Bool =
  set_member
    Nat
    (Ord_instance_Nat).leq
    (Suc Zero)
    (succ Nat (Ord_instance_Nat).leq Zero down_adjacency)

theorem stored_comparator_finds_the_key
    : Equal Bool comparator_observed True = Proved

theorem fresh_canonical_comparator_misses_the_same_key
    : Equal Bool wrong_comparator_observed False = Proved

theorem stored_relation_comparator_finds_the_edge
    : Equal Bool relation_observed True = Proved

theorem fresh_canonical_comparator_misses_the_same_edge
    : Equal Bool wrong_relation_comparator_observed False = Proved
"#
}

/// Promise class: durable invariant.
///
/// The three nominal provider heads select three different dictionary
/// identities through the shared resolver. The list and ordered-key rows both
/// use `Nat` queries, so their distinct result is the carrier-first
/// discriminator: choosing from the query cannot distinguish them.
///
/// The final four theorems are the comparator falsifiers. A lawful descending
/// dictionary is stored in both ordered views while the canonical ascending
/// `Ord Nat` remains available. The stored comparator finds a key and an edge
/// which fresh canonical lookups miss on the same trees. The relation witness
/// recursively proves its stored successor tree ordered under that same
/// dictionary.
#[test]
fn three_named_providers_are_carrier_first_and_retain_their_own_comparator() {
    let mut env = catalog_env();
    catalog_or::expose_module(&mut env, "Core.Classes.LawfulClasses");
    catalog_or::expose_module(&mut env, "Core.Logic.Or");
    catalog_or::expose_module(&mut env, MAP);
    env.elaborate_file(membership_fixture())
        .expect("all three production providers and the comparator control must elaborate");

    let list = completed_dictionary(&env, "list_observed");
    let key = completed_dictionary(&env, "key_observed");
    let relation = completed_dictionary(&env, "relation_observed");
    let comparator = completed_dictionary(&env, "comparator_observed");
    assert_eq!(
        list,
        env.globals["Membership_instance_Algorithm.Searching.OrderedSearch.ListMembership"]
    );
    assert_eq!(
        key,
        env.globals["Membership_instance_Data.Collections.Map.OrderedKeyMembership"]
    );
    assert_eq!(
        relation,
        env.globals["Membership_instance_Data.Collections.Map.RelationEdgeMembership"]
    );
    assert_eq!(comparator, key, "both ordered-key values use one provider");
    assert_ne!(list, key, "same Query Nat must not select by the query");

    for name in [
        "list_observed",
        "key_observed",
        "relation_observed",
        "comparator_observed",
        "wrong_comparator_observed",
        "wrong_relation_comparator_observed",
    ] {
        let id = env.globals[name];
        let ty = match env.env.lookup(id).expect("declared result") {
            Decl::Transparent { ty, .. } => ty,
            other => panic!("{name} must be transparent, got {other:?}"),
        };
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
/// Raw `Tree` has no canonical membership meaning. The ordered-key view over
/// the same representation is the positive control, so this cannot pass by
/// refusing every tree-backed carrier.
#[test]
fn raw_tree_is_refused_while_the_ordered_key_view_is_admitted() {
    let mut env = catalog_env();
    catalog_or::expose_module(&mut env, "Core.Classes.LawfulClasses");
    catalog_or::expose_module(&mut env, MAP);
    env.elaborate_file(
        "import Data.Collections.Map \
           (Tree, OrderedKeyMembership, MkOrderedKeyMembership) \
         import Core.Operators.Standard (∈) \
         const admitted : OrderedKeyMembership Nat Unit = \
           MkOrderedKeyMembership Nat Unit Ord_instance_Nat (Leaf Nat Unit) Proved \
         const positive : Bool = Zero ∈ admitted",
    )
    .expect("the witness-bound ordered-key view must be admitted");

    let result = env.elaborate_file(
        "import Data.Collections.Map (Tree) \
         import Core.Operators.Standard (∈) \
         const forbidden : Bool = Zero ∈ (Leaf Nat Unit)",
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
        "Membership_instance_Algorithm.Searching.OrderedSearch.ListMembership",
        "Data.Collections.Map.ordered_by",
        "Data.Collections.Map.OrderedKeyMembership",
        "Data.Collections.Map.MkOrderedKeyMembership",
        "Data.Collections.Map.ordered_key_membership_member",
        "Membership_instance_Data.Collections.Map.OrderedKeyMembership",
        "Data.Collections.Map.successors_ordered",
        "Data.Collections.Map.RelationEdgeMembership",
        "Data.Collections.Map.MkRelationEdgeMembership",
        "Data.Collections.Map.relation_edge_membership_member",
        "Membership_instance_Data.Collections.Map.RelationEdgeMembership",
    ] {
        let id = env.globals[name];
        assert!(
            !trusted.contains(&id),
            "new checked declaration `{name}` must not enter trusted_base()"
        );
    }
}
