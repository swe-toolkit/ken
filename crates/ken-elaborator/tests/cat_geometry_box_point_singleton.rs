//! The point-box singleton law and its public derivations: spec 67 §15.
//! Promise class: durable invariant. An open client consumes the checked
//! theorem, and an inhabited two-point box separates a false equality from
//! an accepted claim with the same proof form.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::Decl;

const PACKAGE: &str = "Algorithm.Geometry.NatBox";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_package() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("NatBox and its provider closure root-load");
    env
}

// A generic consumer needs neither the new private coordinate lemma nor any
// private area helper. The two final theorems are derivations, not exports.
const GENERIC_CLIENT: &str = r#"
import Core.Classes.LawfulClasses (leq_nat)

import Core.Logic.Transport (cong, sym, trans)

import Algorithm.Geometry.NatBox
  (Point, Box, MkBox, contains, inside, area, contains_point_box, contains_point_box_equal, inside_complete, area_inside, area_point_box)

theorem client_singleton (p : Point) (q : Point) : IsTrue (contains (MkBox p p) q) → Equal Point q p =
  contains_point_box_equal p q

theorem client_point_box_inside (c : Box) (p : Point) : IsTrue (contains c p) → IsTrue (inside (MkBox p p) c) =
  λhp.
    inside_complete
      (MkBox p p)
      c
      p
      (contains_point_box p)
      (λq.
        λhq.
          trans
            Bool
            (contains c q)
            (contains c p)
            True
            (cong Point Bool q p (λr. contains c r) (contains_point_box_equal p q hq))
            hp)

theorem client_member_area (c : Box) (p : Point) : IsTrue (contains c p) → IsTrue (leq_nat (Suc Zero) (area c)) =
  λhp.
    trans
      Bool
      (leq_nat (Suc Zero) (area c))
      (leq_nat (area (MkBox p p)) (area c))
      True
      (sym
        Bool
        (leq_nat (area (MkBox p p)) (area c))
        (leq_nat (Suc Zero) (area c))
        (cong Nat Bool (area (MkBox p p)) (Suc Zero) (λn. leq_nat n (area c)) (area_point_box p)))
      (area_inside (MkBox p p) c (client_point_box_inside c p hp))
"#;

/// SOURCE: spec/50-stdlib/67-geometry.md §15.
/// PROMISE CLASS: durable invariant.
/// MEASURED: a roots-loaded public singleton theorem belongs to NatBox, has a
/// Transparent checked body, and accepts an open selective-import client;
/// two generic downstream consequences also check. The complete before/after
/// trusted GlobalId sets match, while the new coordinate lemma refuses import.
/// CLAIMED: the singleton theorem is public and usable without adding trust
/// or promoting its proof scaffold; inclusion and area need no new exports.
/// THE GAP: imported providers can carry pre-existing trust. Loading the
/// providers before the baseline isolates this law's contribution; the open
/// clients test types rather than proving the clients exhaust all uses.
#[test]
fn singleton_law_and_both_derived_clients_are_checked_without_new_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Logic.Conjunction",
        "Core.Classes.LawfulClasses",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
        "Core.Logic.Transport",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} root-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("the NatBox singleton proof kernel-checks");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(after, before, "the singleton law adds no trusted GlobalId");

    let law = env.globals[&format!("{PACKAGE}.contains_point_box_equal")];
    assert!(owned.contains(&law), "the singleton law is NatBox-owned");
    assert!(matches!(
        env.env.lookup(law),
        Some(Decl::Transparent { .. })
    ));
    let helper = env.globals[&format!("{PACKAGE}.axis_point_equal")];
    assert!(
        owned.contains(&helper),
        "the coordinate lemma is NatBox-owned"
    );
    assert!(matches!(
        env.env.lookup(helper),
        Some(Decl::Transparent { .. })
    ));
    env.elaborate_file(GENERIC_CLIENT)
        .expect("the public law and both derived uses check with open points");
    match env.elaborate_file(&format!("import {PACKAGE} (axis_point_equal)")) {
        Err(ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, format!("{PACKAGE}.axis_point_equal"));
        }
        other => panic!("the coordinate proof must remain private: {other:?}"),
    }
}

// These two points differ only in their y coordinate. The pair box contains
// both; a point box is the accepted same-form neighbour of the false claim.
const CLOSED_PREFIX: &str = r#"
import Algorithm.Geometry.NatBox (Point, MkPoint, MkBox, contains)

const n0 : Nat = Zero
const n1 : Nat = Suc n0
const n2 : Nat = Suc n1
const n3 : Nat = Suc n2
const n4 : Nat = Suc n3
const n5 : Nat = Suc n4
const off : Point = MkPoint n3 n5
const other : Point = MkPoint n3 n4

theorem off_coordinates : Equal Point off (MkPoint n3 n5) = Refl
theorem other_coordinates : Equal Point other (MkPoint n3 n4) = Refl
theorem other_in_pair : IsTrue (contains (MkBox other off) other) = Proved
theorem off_in_point_box : IsTrue (contains (MkBox off off) off) = Proved

theorem control_point_equality : IsTrue (contains (MkBox off off) off) → Equal Point off off = λh. Refl
"#;

const REFUSED: &str = r#"
theorem false_member_equality : IsTrue (contains (MkBox other off) other) → Equal Point other off = λh. Refl
"#;

/// SOURCE: spec 67 §15; the closed two-point refusal and control from AC-2.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the same-form `Refl` control checks after independently checked
/// coordinate and membership witnesses. In that complete accepted context,
/// the appended false member-equality theorem is refused at its own `Refl`.
/// CLAIMED: membership of a two-point box does not imply that every member
/// equals its upper corner. THE GAP: this finite counterexample does not
/// establish the general law; the generic open client above consumes it.
#[test]
fn inhabited_two_point_box_refuses_false_singleton_claim_after_control() {
    let mut env = loaded_package();
    env.elaborate_file(CLOSED_PREFIX)
        .expect("both coordinate and membership witnesses and Refl control check");
    for name in [
        "off_coordinates",
        "other_coordinates",
        "other_in_pair",
        "off_in_point_box",
        "control_point_equality",
    ] {
        let id = env.globals[name];
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    let parsed = parse_decls(REFUSED).expect("the false theorem parses");
    let [SourceDecl::TheoremDecl {
        name: claim,
        span: claim_span,
        ..
    }] = parsed.as_slice()
    else {
        panic!("the isolated claim must be one theorem");
    };
    assert_eq!(claim, "false_member_equality");
    match env.elaborate_file(REFUSED) {
        Err(ElabError::TypeMismatch { span, reason })
            if span.start >= claim_span.start
                && span.start < span.end
                && span.end <= claim_span.end
                && reason == "Refl expects an `Eq`-shaped goal" => {}
        other => panic!("the false equality must refuse at its own Refl: {other:?}"),
    }
}
