//! Spec 67 §20: positive area contains the low corner.
//! Promise class: durable invariant. Open clients and exact trust identities
//! pin the generic export; closed opposite-value controls isolate its premise.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

const GEOMETRY: &str = "Algorithm.Geometry.NatBox";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")
}

fn loaded() -> (ElabEnv, BTreeSet<GlobalId>, BTreeSet<GlobalId>) {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Logic.Conjunction",
        "Core.Classes.LawfulClasses",
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("provider {provider} roots-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect();
    let owned = env
        .elaborate_module_from_roots(&[root()], GEOMETRY)
        .expect("NatBox and its positive-area proof root-load");
    let after = env.env.trusted_base().into_iter().collect();
    let law = env.globals[&format!("{GEOMETRY}.area_positive_contains_lo_corner")];
    assert!(owned.contains(&law), "law is declared by NatBox itself");
    assert!(matches!(
        env.env.lookup(law),
        Some(Decl::Transparent { .. })
    ));
    (env, before, after)
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

const OPEN: &str = r#"import Core.Classes.LawfulClasses (IsTrue as CatalogIsTrue, leq_nat)
import Algorithm.Geometry.NatBox
  (Point, MkBox, contains, area, area_positive_contains_lo_corner, contains_hi_corner)

theorem open_area_positive_lo (lo : Point) (hi : Point)
    : CatalogIsTrue (leq_nat (Suc Zero) (area (MkBox lo hi)))
      → CatalogIsTrue (contains (MkBox lo hi) lo) =
  area_positive_contains_lo_corner lo hi

theorem open_area_positive_hi (lo : Point) (hi : Point)
    : CatalogIsTrue (leq_nat (Suc Zero) (area (MkBox lo hi)))
      → CatalogIsTrue (contains (MkBox lo hi) hi) =
  λh. contains_hi_corner lo hi lo (area_positive_contains_lo_corner lo hi h)
"#;

const ACCEPTED: &str = r#"import Core.Classes.LawfulClasses (IsTrue as CatalogIsTrue, leq_nat)
import Algorithm.Geometry.NatBox
  (Point, MkPoint, MkBox, area, contains, area_positive_contains_lo_corner,
   contains_hi_corner)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const lo : Point = MkPoint one two
const hi : Point = MkPoint three two
const inverted_hi : Point = MkPoint three one

theorem valid_area_is_three : Equal Nat (area (MkBox lo hi)) three = Proved
theorem valid_low_is_member : CatalogIsTrue (contains (MkBox lo hi) lo) = Proved
theorem valid_high_is_member : CatalogIsTrue (contains (MkBox lo hi) hi) = Proved
theorem valid_low_by_law : CatalogIsTrue (contains (MkBox lo hi) lo) =
  area_positive_contains_lo_corner lo hi Proved
theorem valid_high_by_corollary : CatalogIsTrue (contains (MkBox lo hi) hi) =
  contains_hi_corner lo hi lo (area_positive_contains_lo_corner lo hi Proved)

theorem inverted_area_is_zero : Equal Nat (area (MkBox lo inverted_hi)) Zero = Proved
theorem inverted_low_is_excluded :
  Equal Bool (contains (MkBox lo inverted_hi) lo) False = Proved
"#;

const REFUSED: &str = r#"theorem refused_inverted_positive_premise :
  CatalogIsTrue (contains (MkBox lo inverted_hi) lo) =
  area_positive_contains_lo_corner lo inverted_hi Proved
"#;

/// SOURCE: spec/50-stdlib/67-geometry.md §20; Architect AC-0.
/// PROMISE CLASS: durable invariant.
/// MEASURED: five provider modules root-load before NatBox; exact before/after
/// trusted GlobalId identity sets agree. The NatBox-owned law is Transparent;
/// open clients apply that owner and derive high-corner membership via the
/// existing public corner law, with checked bodies containing both owner IDs.
/// CLAIMED: the generic low-corner law is public at zero added trust and its
/// high-corner result needs no new export.
/// THE GAP: inherited provider trust remains; closed values alone cannot
/// establish a universally quantified law.
#[test]
fn open_low_law_and_high_corollary_are_checked_without_new_trust() {
    let (mut env, before, after) = loaded();
    eprintln!(
        "NatBox trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(before, after, "NatBox adds no trusted GlobalId identity");
    env.elaborate_file(OPEN)
        .expect("open clients use the checked public law and corollary");
    let low = env.globals[&format!("{GEOMETRY}.area_positive_contains_lo_corner")];
    let high = env.globals[&format!("{GEOMETRY}.contains_hi_corner")];
    for (client, expected) in [
        ("open_area_positive_lo", &[low][..]),
        ("open_area_positive_hi", &[low, high][..]),
    ] {
        let (_, body) = env
            .env
            .transparent_body(env.globals[client])
            .expect("checked client");
        for owner in expected {
            assert!(
                has_const(&body, *owner),
                "{client} cites its exact public owner"
            );
        }
    }
    for private in [
        "axis_corner_from_span",
        "axis_span_positive_orders",
        "mul_positive_left",
        "mul_positive_right",
    ] {
        let id = env.globals[&format!("{GEOMETRY}.{private}")];
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
        match env.elaborate_file(&format!("import {GEOMETRY} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{GEOMETRY}.{private}"));
            }
            other => panic!("{private} must stay private: {other:?}"),
        }
    }
}

/// SOURCE: spec/50-stdlib/67-geometry.md §20 and AC-2 fixture.
/// PROMISE CLASS: durable invariant.
/// MEASURED: independent closed values establish area three and both corners
/// for the valid box, area zero and low-corner exclusion for the y-inverted
/// box; accepted law and high-corner client also check on the valid box.
/// CLAIMED: the new law establishes the low corner when area is positive,
/// while the inverted box really lacks that member.
/// THE GAP: these fixed observations supplement the open checked clients.
#[test]
fn positive_area_gives_both_corners_and_y_inversion_has_zero_area() {
    let (mut env, _, _) = loaded();
    env.elaborate_file(ACCEPTED)
        .expect("closed values and valid law applications check");
    for name in [
        "valid_area_is_three",
        "valid_low_is_member",
        "valid_high_is_member",
        "valid_low_by_law",
        "valid_high_by_corollary",
        "inverted_area_is_zero",
        "inverted_low_is_excluded",
    ] {
        assert!(matches!(
            env.env.lookup(env.globals[name]),
            Some(Decl::Transparent { .. })
        ));
    }
}

/// SOURCE: spec/50-stdlib/67-geometry.md §20; AC-2 refused form.
/// PROMISE CLASS: durable invariant.
/// MEASURED: an isolated fresh environment accepts the same prefix and the
/// inverted box's false-value controls, then the same law application with
/// only `hy` changed from two to one refuses as KernelRejected TypeMismatch
/// inside its own theorem declaration (the measured span is not token-level).
/// CLAIMED: an inverted box cannot satisfy the positive-area premise.
/// THE GAP: the kernel's declaration-level span cannot identify one subterm;
/// the independently checked area-zero and membership-false controls pin
/// the false premise and conclusion on that same box.
#[test]
fn inverted_y_positive_premise_refuses_on_its_own_claim() {
    let (mut env, _, _) = loaded();
    env.elaborate_file(ACCEPTED)
        .expect("fresh accepted and opposite-value prefix");
    let parsed = parse_decls(REFUSED).expect("isolated refusal parses");
    let [SourceDecl::TheoremDecl {
        name, span: claim, ..
    }] = parsed.as_slice()
    else {
        panic!("one isolated refused theorem");
    };
    assert_eq!(name, "refused_inverted_positive_premise");
    match env
        .elaborate_file(REFUSED)
        .expect_err("false premise must refuse")
    {
        ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        } if span.start >= claim.start && span.end <= claim.end && span.start < span.end => {}
        other => panic!("false positive-area premise must fail at own theorem: {other:?}"),
    }
}
