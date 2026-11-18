//! Inline modules inherit checked enclosing declarations, not enclosing imports.
//! An ancestor import blocks a farther same-spelled local even for descendants.
//! Promise class: durable Strict lexical-resolution invariants
//! (spec 33 §§3.1, 3.3).
//! Forward-reference ordering is intentionally not pinned in this child WP.

use std::fs;
use std::path::Path;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{GlobalId, Term};

fn strict(env: &mut ElabEnv, roots: &[std::path::PathBuf], source: &str) -> Result<(), ElabError> {
    env.elaborate_isolated_file_strict_v1(roots, source)?;
    Ok(())
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

fn body_mentions(env: &ElabEnv, name: &str, target: GlobalId) {
    let (_, body) = env
        .env
        .transparent_body(env.globals[name])
        .expect("checked function");
    assert!(
        mentions(&body, target),
        "{name} must select the checked {target:?}: {body:?}"
    );
}

fn provider_with_helper(root: &Path) -> Vec<std::path::PathBuf> {
    fs::write(
        root.join("Provider.ken"),
        "pub fn helper (b : Bool) : Bool = False\n",
    )
    .expect("write checked provider");
    vec![root.to_path_buf()]
}

fn provider_with_shadow_candidates(root: &Path) -> Vec<std::path::PathBuf> {
    fs::write(
        root.join("Provider.ken"),
        "pub fn helper (b : Bool) : Bool = False\n\
         pub fn other (b : Bool) : Bool = b\n",
    )
    .expect("write checked provider with two spellings");
    vec![root.to_path_buf()]
}

/// MEASURED: a Strict inline child checks a parameterized class in its type
/// and a prior enclosing function in its body, both at the enclosing IDs.
/// CLAIMED: checked local identities, not bare flat globals, cross the lexical
/// boundary. THE GAP: a same-spelled flat-global decoy is exercised at the
/// resolver seam by a separate unit pin; this source checks the real pipeline.
#[test]
fn earlier_enclosing_class_and_function_reach_child_at_checked_ids() {
    let mut env = ElabEnv::new().expect("base");
    strict(
        &mut env,
        &[],
        "class Ord a { leq : a -> a -> Bool }\n\
         fn helper (b : Bool) : Bool = b\n\
         module Child {\n\
           pub fn ord_leq_at (a : Type) (d : Ord a) (x : a) (y : a) : Bool = d.leq x y\n\
           pub fn use_helper (b : Bool) : Bool = helper b\n\
         }\n",
    )
    .expect("earlier enclosing declarations must be visible in child");
    let ord = env.globals["Ord"];
    let helper = env.globals["helper"];
    let (_, ty) = env
        .env
        .const_type(env.globals["Child.ord_leq_at"])
        .expect("checked child type");
    assert!(
        mentions(&ty, ord),
        "child type must select its enclosing class: {ty:?}"
    );
    body_mentions(&env, "Child.use_helper", helper);
}

/// MEASURED: an outer import succeeds at the root but its spelling refuses
/// at the reference in a child that has no matching import. CLAIMED: child
/// scopes inherit enclosing declarations only. THE GAP: this names a real
/// checked provider, not an absent global; an import replay would turn this
/// refusal into acceptance and redden this row.
#[test]
fn enclosing_import_does_not_leak_into_child() {
    let root = tempfile::tempdir().expect("isolated catalog");
    let roots = provider_with_helper(root.path());
    let mut root_env = ElabEnv::new().expect("base");
    strict(
        &mut root_env,
        &roots,
        "import Provider (helper)\nfn root_use (b : Bool) : Bool = helper b\n",
    )
    .expect("positive control: the enclosing import is valid");
    body_mentions(&root_env, "root_use", root_env.globals["Provider.helper"]);

    let source = "import Provider (helper)\nmodule Child {\n\
                  pub fn child_use (b : Bool) : Bool = helper b\n}\n";
    let mut child_env = ElabEnv::new().expect("base");
    match strict(&mut child_env, &roots, source) {
        Err(ElabError::UnboundName { name, span }) => {
            assert_eq!(name, "helper");
            let start = source.rfind("helper").expect("child reference");
            assert_eq!((span.start, span.end), (start, start + name.len()));
        }
        other => panic!("enclosing-only import must refuse at child reference: {other:?}"),
    }
}

/// MEASURED: a child's own checked function wins over its same-spelled
/// enclosing function, and a later enclosing declaration retains its own
/// original ID. CLAIMED: a child local shadows, without rebinding, the outer
/// local. THE GAP: the import-shadow and same-child-clash cases are separate
/// rows; this one alone cannot tell them apart.
#[test]
fn child_local_shadows_enclosing_local_without_rebinding_parent() {
    let mut env = ElabEnv::new().expect("base");
    strict(
        &mut env,
        &[],
        "fn helper (b : Bool) : Bool = b\n\
         module Child {\n\
           fn helper (b : Bool) : Bool = True\n\
           pub fn child_use (b : Bool) : Bool = helper b\n\
         }\n\
         fn root_use (b : Bool) : Bool = helper b\n",
    )
    .expect("child local may shadow its enclosing local");
    let outer = env.globals["helper"];
    let inner = env.globals["Child.helper"];
    assert_ne!(outer, inner);
    body_mentions(&env, "Child.child_use", inner);
    body_mentions(&env, "root_use", outer);
}

/// MEASURED: a child import selects its provider ID over an enclosing local
/// while a later outer reference still selects the enclosing ID. CLAIMED:
/// lexical child imports shadow ancestor locals. THE GAP: an import/local
/// collision within the child is checked by a separate refusal row.
#[test]
fn child_import_shadows_enclosing_local_without_rebinding_parent() {
    let root = tempfile::tempdir().expect("isolated catalog");
    let roots = provider_with_helper(root.path());
    let mut env = ElabEnv::new().expect("base");
    strict(
        &mut env,
        &roots,
        "fn helper (b : Bool) : Bool = b\n\
         module Child {\n\
           import Provider (helper)\n\
           pub fn child_use (b : Bool) : Bool = helper b\n\
         }\n\
         fn root_use (b : Bool) : Bool = helper b\n",
    )
    .expect("child import may shadow its enclosing local");
    let outer = env.globals["helper"];
    let imported = env.globals["Provider.helper"];
    assert_ne!(outer, imported);
    body_mentions(&env, "Child.child_use", imported);
    body_mentions(&env, "root_use", outer);
}

/// MEASURED: a child local and a child import of a distinct checked
/// declaration with the same bare spelling refuse AmbiguousReference at
/// that binding. CLAIMED: lexical shadowing does not weaken within-scope
/// ambiguity. THE GAP: the two valid cross-scope shadow rows above show this
/// refusal is about the child's own two bindings, not an ancestor collision.
#[test]
fn child_own_local_import_collision_remains_ambiguous() {
    let root = tempfile::tempdir().expect("isolated catalog");
    let roots = provider_with_helper(root.path());
    let source = "fn helper (b : Bool) : Bool = b\n\
                  module Child {\n\
                    fn helper (b : Bool) : Bool = True\n\
                    import Provider (helper)\n\
                  }\n";
    let mut env = ElabEnv::new().expect("base");
    match strict(&mut env, &roots, source) {
        Err(ElabError::AmbiguousReference { name, .. }) => assert_eq!(name, "helper"),
        other => panic!("within-child clash must refuse as ambiguous: {other:?}"),
    }
}

/// MEASURED: two lexical ancestor layers carry distinct checked functions;
/// the nearest layer wins for a repeated spelling, and a farther root-only
/// name remains reachable. CLAIMED: grandchild lookup composes outward.
/// THE GAP: root and child imports are not part of those layers; their
/// independent negative rows exercise the excluded population.
#[test]
fn grandchild_selects_nearest_child_then_farthest_root_local() {
    let mut env = ElabEnv::new().expect("base");
    strict(
        &mut env,
        &[],
        "fn shared (b : Bool) : Bool = b\n\
         fn root_only (b : Bool) : Bool = b\n\
         module Child {\n\
           fn shared (b : Bool) : Bool = True\n\
           module Grandchild {\n\
             pub fn choose_near (b : Bool) : Bool = shared b\n\
             pub fn choose_far (b : Bool) : Bool = root_only b\n\
           }\n\
         }\n",
    )
    .expect("grandchild sees child and root checked locals");
    assert_ne!(env.globals["shared"], env.globals["Child.shared"]);
    body_mentions(
        &env,
        "Child.Grandchild.choose_near",
        env.globals["Child.shared"],
    );
    body_mentions(
        &env,
        "Child.Grandchild.choose_far",
        env.globals["root_only"],
    );
}

/// MEASURED: an import into the immediate parent cannot resolve inside its
/// grandchild, while the import itself checks at that parent position.
/// CLAIMED: each lexical layer carries declarations, never imports. THE GAP:
/// the child-only-import row above separately covers a root-level import.
#[test]
fn grandchild_does_not_inherit_immediate_parent_import() {
    let root = tempfile::tempdir().expect("isolated catalog");
    let roots = provider_with_helper(root.path());
    let source = "module Child {\n\
                    import Provider (helper)\n\
                    module Grandchild {\n\
                      pub fn use_helper (b : Bool) : Bool = helper b\n\
                    }\n\
                  }\n";
    let mut env = ElabEnv::new().expect("base");
    match strict(&mut env, &roots, source) {
        Err(ElabError::UnboundName { name, span }) => {
            assert_eq!(name, "helper");
            let start = source.rfind("helper b").expect("grandchild reference");
            assert_eq!((span.start, span.end), (start, start + name.len()));
        }
        other => panic!("parent import must not enter grandchild: {other:?}"),
    }
}

/// MEASURED: Mid's imported `helper` selects Provider's checked ID, but
/// Mid.G's same spelling refuses at G's reference instead of selecting the
/// root local. CLAIMED: the ancestor import shadows the root for all of Mid's
/// text without becoming inherited by G. THE GAP: the accept twin verifies
/// Mid's import at its actual body, and the mutant proves G otherwise selects
/// the farther root ID.
#[test]
fn grandchild_stops_at_parent_import_shadow_and_parent_keeps_provider_id() {
    let root = tempfile::tempdir().expect("isolated catalog");
    let roots = provider_with_shadow_candidates(root.path());
    let source = "fn helper (b : Bool) : Bool = b\n\
                  module Mid {\n\
                    import Provider (helper)\n\
                    pub fn m (b : Bool) : Bool = helper b\n\
                    module G { pub fn g (b : Bool) : Bool = helper b }\n\
                  }\n";
    let twin = source.replace(
        "pub fn g (b : Bool) : Bool = helper b",
        "pub fn g (b : Bool) : Bool = b",
    );
    assert_ne!(twin, source, "the twin must remove G's reference only");
    let mut positive = ElabEnv::new().expect("base");
    strict(&mut positive, &roots, &twin).expect("Mid's import remains checked");
    assert_ne!(
        positive.globals["helper"],
        positive.globals["Provider.helper"]
    );
    body_mentions(&positive, "Mid.m", positive.globals["Provider.helper"]);

    let mut env = ElabEnv::new().expect("base");
    match strict(&mut env, &roots, source) {
        Err(ElabError::UnboundName { name, span }) => {
            assert_eq!(name, "helper");
            let start = source.rfind("helper b").expect("G's reference");
            assert_eq!((span.start, span.end), (start, start + name.len()));
        }
        Ok(()) => {
            body_mentions(&env, "Mid.G.g", env.globals["helper"]);
            panic!("G must not bind the shadowed root helper");
        }
        Err(error) => panic!("G must refuse at its helper reference: {error:?}"),
    }
}

/// MEASURED: an import renamed from `other` to `helper` blocks a grandchild
/// at its own reference to `helper`. CLAIMED: shadowing keys on the bound
/// spelling, not the exported item's old name. THE GAP: the controls below
/// distinguish a missing Mid binding from a renamed import binding.
#[test]
fn renamed_parent_import_shadows_root_helper_for_grandchild() {
    let root = tempfile::tempdir().expect("isolated catalog");
    let roots = provider_with_shadow_candidates(root.path());
    let source = "fn helper (b : Bool) : Bool = b\n\
                  module Mid {\n\
                    import Provider (other as helper)\n\
                    module G { pub fn g (b : Bool) : Bool = helper b }\n\
                  }\n";
    let mut env = ElabEnv::new().expect("base");
    match strict(&mut env, &roots, source) {
        Err(ElabError::UnboundName { name, span }) => {
            assert_eq!(name, "helper");
            let start = source.rfind("helper b").expect("G's reference");
            assert_eq!((span.start, span.end), (start, start + name.len()));
        }
        Ok(()) => {
            body_mentions(&env, "Mid.G.g", env.globals["helper"]);
            panic!("renamed Mid import must block the root helper");
        }
        Err(error) => panic!("G must refuse at its helper reference: {error:?}"),
    }
}

/// MEASURED: the walk passes an empty G layer but stops at Mid's import
/// before reaching the root helper. CLAIMED: shadowing is enforced at each
/// ancestor, not only in the grandchild's immediate parent. THE GAP: the
/// direct-grandchild row above separately exercises the nonempty boundary.
#[test]
fn import_shadow_stops_after_an_empty_intermediate_layer() {
    let root = tempfile::tempdir().expect("isolated catalog");
    let roots = provider_with_shadow_candidates(root.path());
    let source = "fn helper (b : Bool) : Bool = b\n\
                  module Mid {\n\
                    import Provider (helper)\n\
                    module G { module H { pub fn h (b : Bool) : Bool = helper b } }\n\
                  }\n";
    let mut env = ElabEnv::new().expect("base");
    match strict(&mut env, &roots, source) {
        Err(ElabError::UnboundName { name, span }) => {
            assert_eq!(name, "helper");
            let start = source.rfind("helper b").expect("H's reference");
            assert_eq!((span.start, span.end), (start, start + name.len()));
        }
        Ok(()) => {
            body_mentions(&env, "Mid.G.H.h", env.globals["helper"]);
            panic!("H must not walk through Mid's import to the root helper");
        }
        Err(error) => panic!("H must refuse at its helper reference: {error:?}"),
    }
}

/// MEASURED: an actual local in Mid is granted to G, ahead of the root
/// local. CLAIMED: tagging imported shadows must not block checked nearer
/// locals. THE GAP: the other controls cover an absent binding and an own
/// import in G.
#[test]
fn grandchild_still_selects_mid_local_before_root_local() {
    let root = tempfile::tempdir().expect("isolated catalog");
    let roots = provider_with_shadow_candidates(root.path());
    let mut env = ElabEnv::new().expect("base");
    strict(
        &mut env,
        &roots,
        "fn helper (b : Bool) : Bool = b\n\
         module Mid {\n\
           fn helper (b : Bool) : Bool = True\n\
           module G { pub fn g (b : Bool) : Bool = helper b }\n\
         }\n",
    )
    .expect("a checked Mid local takes precedence");
    assert_ne!(env.globals["Mid.helper"], env.globals["helper"]);
    body_mentions(&env, "Mid.G.g", env.globals["Mid.helper"]);
}

/// MEASURED: an unrelated imported spelling in Mid does not prevent G
/// from selecting the root's checked helper. CLAIMED: shadowing tests the
/// *bound spelling*, not merely whether a layer holds any import. THE GAP:
/// the renamed-import row verifies the complementary same-spelling case.
#[test]
fn grandchild_reaches_root_local_past_unrelated_parent_import() {
    let root = tempfile::tempdir().expect("isolated catalog");
    let roots = provider_with_shadow_candidates(root.path());
    let mut env = ElabEnv::new().expect("base");
    strict(
        &mut env,
        &roots,
        "fn helper (b : Bool) : Bool = b\n\
         module Mid {\n\
           import Provider (other)\n\
           module G { pub fn g (b : Bool) : Bool = helper b }\n\
         }\n",
    )
    .expect("a different import spelling cannot shadow helper");
    body_mentions(&env, "Mid.G.g", env.globals["helper"]);
}

/// MEASURED: G's own import of `helper` resolves to the provider's ID,
/// ahead of the import shadow in its parent. CLAIMED: a shadow denies only
/// inherited visibility; it does not forbid the descendant's own import.
/// THE GAP: the other controls cover the nearer and farther local routes.
#[test]
fn grandchild_own_import_wins_over_parent_shadow() {
    let root = tempfile::tempdir().expect("isolated catalog");
    let roots = provider_with_shadow_candidates(root.path());
    let mut env = ElabEnv::new().expect("base");
    strict(
        &mut env,
        &roots,
        "fn helper (b : Bool) : Bool = b\n\
         module Mid {\n\
           import Provider (helper)\n\
           module G {\n\
             import Provider (helper)\n\
             pub fn g (b : Bool) : Bool = helper b\n\
           }\n\
         }\n",
    )
    .expect("G's own import may still select its provider");
    assert_ne!(env.globals["helper"], env.globals["Provider.helper"]);
    body_mentions(&env, "Mid.G.g", env.globals["Provider.helper"]);
}

/// MEASURED: an inline child republishes an enclosing declaration at its
/// original checked identity, and a sibling imports that exported identity.
/// CLAIMED: in-scope export selection reads the enclosing layer rather than
/// `globals[canonical]`. THE GAP: the decoy pin tests the resolver directly;
/// this row tests the export and subsequent import through a real source.
#[test]
fn child_export_of_enclosing_local_republishes_its_checked_identity() {
    let mut env = ElabEnv::new().expect("base");
    strict(
        &mut env,
        &[],
        "fn helper (b : Bool) : Bool = b\n\
         module Child { export helper }\n\
         module Friend {\n\
           import Child (helper)\n\
           pub fn use_helper (b : Bool) : Bool = helper b\n\
         }\n",
    )
    .expect("child may republish an enclosing checked declaration");
    body_mentions(&env, "Friend.use_helper", env.globals["helper"]);
}

/// MEASURED: an inherited `T.C` selector is exported by the child and
/// imported by a sibling at its original checked constructor identity.
/// CLAIMED: constructor-path export does not reselect a flat global key.
/// THE GAP: the parent-family route is also tested directly by the child
/// expression below; export alone would not establish local resolution.
#[test]
fn child_can_export_enclosing_type_constructor_by_checked_parent() {
    let mut env = ElabEnv::new().expect("base");
    strict(
        &mut env,
        &[],
        "data LocalTag = MkLocalTag\n\
         module Child { export LocalTag.MkLocalTag as makeTag }\n\
         module Friend {\n\
           import Child (makeTag)\n\
           pub const made : LocalTag = makeTag\n\
         }\n",
    )
    .expect("ancestor family member export must select checked constructor");
    body_mentions(&env, "Friend.made", env.globals["MkLocalTag"]);
}

/// MEASURED: the checked constructor is selected from its enclosing data
/// family's parent ID, and a standard floor constructor remains usable in
/// the same child. CLAIMED: `T.C` and the unshadowable B floor preserve their
/// identity across the layer. THE GAP: no import of either family is involved.
#[test]
fn child_can_select_enclosing_type_constructor_and_floor() {
    let mut env = ElabEnv::new().expect("base");
    strict(
        &mut env,
        &[],
        "data LocalTag = MkLocalTag\n\
         module Child {\n\
           pub const make_tag : LocalTag = LocalTag.MkLocalTag\n\
           pub fn use_floor (b : Bool) : Bool = True\n\
         }\n",
    )
    .expect("ancestor constructor and prelude floor remain available");
    body_mentions(&env, "Child.make_tag", env.globals["MkLocalTag"]);
    let (_, floor_ty) = env
        .env
        .const_type(env.globals["Child.use_floor"])
        .expect("checked floor use");
    assert!(mentions(&floor_ty, env.numeric_env.bool_id));
}

/// MEASURED: a child import trying to bind a different checked declaration
/// as `Bool` is rejected at the floor spelling. CLAIMED: the B floor stays
/// unshadowable even though ordinary enclosing locals may be shadowed.
/// THE GAP: the successful `Bool` use above proves the name is live in child.
#[test]
fn child_import_cannot_shadow_the_bool_floor() {
    let root = tempfile::tempdir().expect("isolated catalog");
    let roots = provider_with_helper(root.path());
    let mut env = ElabEnv::new().expect("base");
    match strict(
        &mut env,
        &roots,
        "module Child { import Provider (helper as Bool) }\n",
    ) {
        Err(ElabError::AmbiguousReference { name, .. }) => assert_eq!(name, "Bool"),
        other => panic!("floor collision must refuse at Bool: {other:?}"),
    }
}

/// MEASURED: an inline instance names an earlier enclosing class and its
/// child-local carrier, producing a dictionary whose checked type includes
/// the enclosing class ID. CLAIMED: direct class/dictionary consumers use the
/// lexical identity, not just RCon type and expression lookups. THE GAP:
/// the separate decoy unit pin rules out spelling-keyed class selection.
#[test]
fn child_instance_uses_enclosing_checked_class_identity() {
    let mut env = ElabEnv::new().expect("base");
    strict(
        &mut env,
        &[],
        "class Ord a { leq : a -> a -> Bool }\n\
         module Child {\n\
           data Local = MkLocal\n\
           instance Ord Local { leq = λx. λy. True }\n\
           pub fn use_where (b : Local) : Bool where Ord Local = d.leq b b\n\
         }\n",
    )
    .expect("child instance and where binder must select enclosing checked class");
    let ord = env.globals["Ord"];
    let dict = env
        .globals
        .iter()
        .filter(|(name, _)| name.contains("Ord_instance_"))
        .map(|(_, id)| *id)
        .collect::<Vec<_>>();
    assert_eq!(dict.len(), 1, "one checked child dictionary: {dict:?}");
    let (_, ty) = env
        .env
        .const_type(dict[0])
        .expect("checked dictionary type");
    assert!(
        mentions(&ty, ord),
        "dictionary must be for enclosing class: {ty:?}"
    );
    body_mentions(&env, "Child.use_where", dict[0]);
}

/// MEASURED: the original five-role provider/home shape in the held
/// standard-operator row checks under Strict with both enclosing classes
/// already admitted. CLAIMED: the unchanged held facade's provider can now
/// use class types without Legacy fallback. THE GAP: this is a corresponding
/// Strict client, not an edit to the original held fixture or assertions.
#[test]
fn held_standard_operator_provider_shape_checks_under_strict() {
    let mut env = ElabEnv::new().expect("base");
    strict(
        &mut env,
        &[],
        "class Ord a { leq : a -> a -> Bool }\n\
         class Membership c { Query : Type; member : Query -> c -> Bool }\n\
         module Provider {\n\
           pub fn bool_and (a : Bool) (b : Bool) : Bool = a\n\
           pub fn bool_or (a : Bool) (b : Bool) : Bool = a\n\
           pub fn ord_leq_at (a : Type) (d : Ord a) (x : a) (y : a) : Bool = d.leq x y\n\
           pub fn ord_geq_at (a : Type) (d : Ord a) (x : a) (y : a) : Bool = d.leq y x\n\
           pub fn membership_member_at\n\
             (c : Type) (d : Membership c) (q : d.Query) (x : c) : Bool = d.member q x\n\
         }\n\
         module Core.Operators.Standard {\n\
           export Provider (bool_and as ∧, bool_or as ∨,\n\
                            ord_leq_at as ≤, ord_geq_at as ≥,\n\
                            membership_member_at as ∈)\n\
         }\n",
    )
    .expect("held standard facade shape must check under Strict");
    let ord = env.globals["Ord"];
    let membership = env.globals["Membership"];
    let (_, ord_ty) = env
        .env
        .const_type(env.globals["Provider.ord_leq_at"])
        .expect("checked Ord binding");
    let (_, membership_ty) = env
        .env
        .const_type(env.globals["Provider.membership_member_at"])
        .expect("checked Membership binding");
    assert!(mentions(&ord_ty, ord));
    assert!(mentions(&membership_ty, membership));
}

/// MEASURED: the held membership fixture's two enclosing classes and five
/// provider functions reach the required-role gate under Strict, which
/// refuses its intentionally absent `∈` role by name. CLAIMED: the held row
/// no longer fails earlier at its enclosing class references. THE GAP:
/// presence of every role is checked by the preceding successful facade row.
#[test]
fn held_membership_missing_role_reaches_its_strict_certification_gate() {
    let mut env = ElabEnv::new().expect("base");
    match strict(
        &mut env,
        &[],
        "class Ord a { leq : a -> a -> Bool }\n\
         class Membership c { Query : Type; member : Query -> c -> Bool }\n\
         module Provider {\n\
           pub fn bool_and (a : Bool) (b : Bool) : Bool = a\n\
           pub fn bool_or (a : Bool) (b : Bool) : Bool = a\n\
           pub fn ord_leq_at (a : Type) (d : Ord a) (x : a) (y : a) : Bool = d.leq x y\n\
           pub fn ord_geq_at (a : Type) (d : Ord a) (x : a) (y : a) : Bool = d.leq y x\n\
           pub fn membership_member_at\n\
             (c : Type) (d : Membership c) (q : d.Query) (x : c) : Bool = d.member q x\n\
         }\n\
         module Core.Operators.Standard {\n\
           export Provider (bool_and as ∧, bool_or as ∨,\n\
                            ord_leq_at as ≤, ord_geq_at as ≥)\n\
         }\n",
    ) {
        Err(ElabError::StandardOperatorRoleUnfilled { role, .. }) => assert_eq!(role, "∈"),
        other => panic!("missing ∈ must reach the held role refusal: {other:?}"),
    }
}
