//! `KERNEL-RECURSOR-UNUSED-IH-REDUCTION` AC-1.
//!
//! A non-recursive match over `FokCert` must reduce at a constructor whose
//! recursive `children` field is still abstract. The generated method discards
//! its child IH, so forcing that IH cannot affect the result.

#[path = "support/catalog_or.rs"]
mod catalog_or;

use std::collections::BTreeSet;

use ken_elaborator::ElabEnv;

const FOK_SOURCE: &str =
    include_str!("../../../catalog/packages/Tooling/Verification/FoKripke.ken");

/// Durable invariant: the recursor may skip a provably dead IH argument, making
/// the exact outer `fok_check_tree` view convertible without trust growth.
#[test]
fn fok_check_node_abstract_children_probe_is_convertible_by_refl() {
    let mut env = ElabEnv::new().expect("base environment");
    catalog_or::load_core_logic_or(&mut env);
    env.elaborate_file(FOK_SOURCE)
        .expect("FoKripke package must elaborate");
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();

    env.elaborate_decl(
        "fn fok_check_node_probe \
           (expected : FokSequent) (node : FokCert) : Bool = \
         match node { \
           FokMkCert conclusion rule children |-> \
             match fok_sequent_eq conclusion expected { \
               False |-> False; \
               True |-> match conclusion { \
                 FokMkSequent gamma delta |-> \
                   fok_check_rule gamma delta rule children \
               } \
             } \
         }",
    )
    .expect("exact outer fok_check_tree body");
    env.elaborate_decl(
        "fn fok_check_node_view_probe \
           (expected : FokSequent) \
           (gamma : List FokForm) (delta : List FokForm) \
           (rule : FokRule) (children : List FokCert) : Bool = \
         match fok_sequent_eq (FokMkSequent gamma delta) expected { \
           False |-> False; \
           True |-> fok_check_rule gamma delta rule children \
         }",
    )
    .expect("IH-stripped comparison view");
    env.elaborate_decl(
        "theorem fok_check_node_abstract_children_probe \
           (expected : FokSequent) \
           (gamma : List FokForm) (delta : List FokForm) \
           (rule : FokRule) (children : List FokCert) \
         : Equal Bool \
             (fok_check_node_probe expected \
               (FokMkCert (FokMkSequent gamma delta) rule children)) \
             (fok_check_node_view_probe \
               expected gamma delta rule children) = Refl",
    )
    .expect("dead child IH must not block constructor iota reduction");

    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(before, after, "the reduction repair adds no trusted base");
}
