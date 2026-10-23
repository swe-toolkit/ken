//! An annotated let in a nested recursive branch keeps its moved field at
//! the goal coordinate without rewriting a dependent context binder.
//! Promise class: durable typed-admission/refusal invariant. Spec 34 §3.5
//! and 39 §2.1; Architect D0 evt_qd02djxjvf70.

use ken_elaborator::{ElabEnv, ElabError};

// The existing RSC fixture measured a 1,982,464-byte prelude peak; 8 MiB
// provides more than fourfold headroom for the same recursive fixture family.
const ELABORATION_STACK_BYTES: usize = 8 * 1024 * 1024;

fn on_elaboration_stack(f: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .stack_size(ELABORATION_STACK_BYTES)
        .spawn(f)
        .expect("spawn stated-stack elaboration")
        .join()
        .expect("stated-stack elaboration panicked");
}

const FIXTURES: &[&str] = &[
    "data DualFin : Nat -> Type where { DualFZ : (n : Nat) -> DualFin (Suc n); DualFS : (n : Nat) -> DualFin n -> DualFin (Suc n) }",
    "data DualEnv (a : Type) : Nat -> Type where { DualNil : DualEnv a Zero; DualCons : (n : Nat) -> a -> DualEnv a n -> DualEnv a (Suc n) }",
    "fn dual_fin_to_nat (n : Nat) (i : DualFin n) : Nat = match i { DualFZ m ↦ Zero; DualFS m j ↦ Suc (dual_fin_to_nat m j) }",
    "fn dual_lookup (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : a = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ x }; DualFS m j ↦ match xs { DualCons k x rest ↦ dual_lookup a m rest j } }",
    "fn dual_len (a : Type) (n : Nat) (xs : DualEnv a n) : Nat = match xs { DualNil ↦ Zero; DualCons k x rest ↦ Suc (dual_len a k rest) }",
];

const LOOKUP: &str = "dual_lookup a n xs i";
const EQUALITY_HEAD: &str =
    "(a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : Equal a (LK) (LK)";
const PREDICATE_HEAD: &str = "(a : Type) (pp : a → Prop) (hp : (x : a) → pp x) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : pp (LK)";

fn row(name: &str, head: &str, zero: &str, successor: &str) -> String {
    let head = head.replace("LK", LOOKUP);
    let successor = successor.replace("REC", name);
    format!(
        "theorem {name} {head} = match i {{ DualFZ m ↦ match xs {{ DualCons k x rest ↦ {zero} }}; DualFS m j ↦ match xs {{ DualCons k x rest ↦ {successor} }} }}"
    )
}

fn fresh_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude");
    let trust = env.env.trusted_base();
    for source in FIXTURES {
        env.elaborate_decl(source)
            .unwrap_or_else(|error| panic!("fixture {source}: {error:?}"));
    }
    assert_eq!(env.env.trusted_base(), trust, "fixtures add no trust");
    env
}

fn accepts(source: &str) {
    let mut env = fresh_env();
    let trust = env.env.trusted_base();
    env.elaborate_decl(source)
        .unwrap_or_else(|error| panic!("true branch must check: {source}: {error:?}"));
    assert_eq!(env.env.trusted_base(), trust, "no new trusted identity");
}

fn refuses_typed(source: &str) {
    let mut env = fresh_env();
    let trust = env.env.trusted_base();
    let error = env
        .elaborate_decl(source)
        .expect_err("false branch must refuse");
    assert!(
        matches!(
            error,
            ElabError::KernelRejected { .. } | ElabError::TypeMismatch { .. }
        ),
        "false branch must refuse by a type error, not Internal: {error:?}"
    );
    assert_eq!(env.env.trusted_base(), trust, "refusal adds no trust");
}

/// MEASURED: R1 and its predicate sibling check despite an annotated recursive
/// IH using the moved field. CLAIMED: type-position locals resolve to the
/// branch's checked field coordinate. THE GAP: other index families are not
/// enumerated by these two sources; R2 independently exercises the let leaf.
#[test]
fn recursive_annotated_lets_check_at_the_body_coordinate() {
    on_elaboration_stack(|| {
        accepts(&row("r1", EQUALITY_HEAD, "Refl", "let ih : Equal a (dual_lookup a m rest j) (dual_lookup a m rest j) = REC a m rest j in ih"));
        accepts(&row(
            "r1pp",
            PREDICATE_HEAD,
            "hp x",
            "let ih : pp (dual_lookup a m rest j) = REC a pp hp m rest j in ih",
        ));
    });
}

/// MEASURED: R2's nonrecursive let and R2len's unrelated annotation both
/// check. CLAIMED: dependent let leaves skip only coordinates they cannot
/// safely rewrite. THE GAP: each fresh index family has its own leaf geometry;
/// the separate ds5b suite controls an admissible outer-index substitution.
#[test]
fn moved_field_let_skips_only_nonclosed_refinement_leaves() {
    on_elaboration_stack(|| {
        accepts(&row(
            "r2",
            PREDICATE_HEAD,
            "hp x",
            "let ih : pp (dual_lookup a m rest j) = hp (dual_lookup a m rest j) in ih",
        ));
        accepts(&row("r2len", PREDICATE_HEAD, "hp x", "let u : Equal Nat (dual_len a m rest) (dual_len a m rest) = Refl in hp (dual_lookup a m rest j)"));
    });
}

/// MEASURED: the two otherwise matching false target annotations refuse
/// through typed errors, while five no-let/ordinary-let controls still check.
/// CLAIMED: coordinate repair does not turn an inconsistent proof into one
/// accepted by trust or Internal. THE GAP: these are deliberate false twins,
/// not a complete enumeration of wrong propositions.
#[test]
fn false_let_annotations_refuse_and_noncoordinate_controls_check() {
    on_elaboration_stack(|| {
        refuses_typed(&row(
            "r1f",
            PREDICATE_HEAD,
            "hp x",
            "let ih : pp x = REC a pp hp m rest j in ih",
        ));
        refuses_typed(&row(
            "r2f",
            PREDICATE_HEAD,
            "hp x",
            "let ih : pp x = hp (dual_lookup a m rest j) in ih",
        ));
        accepts(&row(
            "c_nolet",
            PREDICATE_HEAD,
            "hp x",
            "hp (dual_lookup a m rest j)",
        ));
        accepts(&row("c_rec", EQUALITY_HEAD, "Refl", "REC a m rest j"));
        accepts(&row(
            "c_unann",
            PREDICATE_HEAD,
            "hp x",
            "let ih = REC a pp hp m rest j in ih",
        ));
        accepts(&row(
            "c_m",
            PREDICATE_HEAD,
            "hp x",
            "let u : Nat = m in hp (dual_lookup a m rest j)",
        ));
        accepts(&row(
            "c_lenval",
            PREDICATE_HEAD,
            "hp x",
            "let u : Nat = dual_len a m rest in hp (dual_lookup a m rest j)",
        ));
    });
}

/// MEASURED: six legitimate Vec let and nested-match forms still check; a
/// seventh genuinely missing VNil arm refuses by typed exhaustiveness.
/// CLAIMED: the dependency condition is not a blanket skip for annotated
/// lets or field aliases. THE GAP: this is one second indexed family, not
/// every context-alias combination.
#[test]
fn vec_siblings_do_not_overrestrict_fresh_annotated_lets() {
    on_elaboration_stack(|| {
        let vec = "data Vec (A : Type) : Nat -> Type where { VNil : Vec A 0; VCons : (n : Nat) -> A -> Vec A n -> Vec A (n+1) }";
        let repl = "fn repl (n : Nat) : Vec Nat n = match n { Zero |-> VNil Nat; Suc m |-> VCons Nat m Zero (repl m) }";
        let body = |name: &str, middle: &str| {
            format!(
            "fn {name} (n : Nat) (v : Vec Nat n) (w : Vec Nat n) : Vec Nat n = match v {{ VNil |-> VNil Nat; VCons m a xs |-> {middle} }}"
        )
        };
        let rows = [
            ("v_fresh", "let k = repl n in match w { VCons _ b ys |-> match k { VCons _ c ks |-> VCons Nat m a xs } }"),
            ("v_fresh_ann", "let k : Vec Nat n = repl n in match k { VCons _ c ks |-> VCons Nat m a xs }"),
            ("v_outer_unann", "let k = w in match k { VCons _ c ks |-> VCons Nat m a xs }"),
            ("v_outer", "let k : Vec Nat n = w in match k { VCons _ c ks |-> VCons Nat m a xs }"),
            ("v_outer_succ", "let k : Vec Nat (Suc m) = w in match k { VCons _ c ks |-> VCons Nat m a xs }"),
            ("v_direct", "match w { VCons _ c ks |-> VCons Nat m a xs }"),
            ("v_field", "let k : Vec Nat m = xs in VCons Nat m a k"),
            ("v_mixed", "let k : Vec Nat n = VCons Nat m a xs in match k { VCons _ c ks |-> k }"),
        ];
        for (name, middle) in rows {
            let mut env = ElabEnv::new().expect("prelude");
            let trust = env.env.trusted_base();
            env.elaborate_decl(vec).expect("Vec declaration");
            env.elaborate_decl(repl).expect("repl declaration");
            let source = body(name, middle);
            env.elaborate_decl(&source)
                .unwrap_or_else(|error| panic!("{name} must check: {error:?}"));
            assert_eq!(env.env.trusted_base(), trust, "{name} added trust");
        }
        let mut env = ElabEnv::new().expect("prelude");
        let trust = env.env.trusted_base();
        env.elaborate_decl(vec).expect("Vec declaration");
        env.elaborate_decl(repl).expect("repl declaration");
        // `p` is independent of the matched variable `v` and its bare index
        // `n`: §3.3 cannot turn this omitted VNil branch into an impossible one.
        let source = "fn v_indep (n : Nat) (p : Nat) (u : Vec Nat p) (v : Vec Nat n) (w : Vec Nat n) : Vec Nat n = match v { VNil |-> VNil Nat; VCons m a xs |-> let k : Vec Nat p = u in match k { VCons _ c ks |-> VCons Nat m a xs } }";
        let error = env
            .elaborate_decl(&source)
            .expect_err("missing VNil must refuse");
        assert!(
            matches!(&error, ElabError::ExhaustivenessError { missing, .. } if missing.constructor == "VNil"),
            "missing VNil must report typed exhaustiveness: {error:?}"
        );
        assert_eq!(env.env.trusted_base(), trust, "refusal added trust");
    });
}
