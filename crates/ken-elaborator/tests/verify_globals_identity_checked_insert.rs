//! Source declaration identities share one checked namespace window
//! (spec/30-surface/33-declarations.md §1, §3, §5).

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::KernelError;

const PRE: &str = "class Lbl carrier { mark : carrier -> Int }\ndata A = MkA\n";
const INSTANCE: &str = "instance Lbl A { mark = \\x. 0 }";
const CONSTRUCTOR: &str = "data D : Type where { Lbl_instance_A : D }";

fn assert_collision(error: ElabError, key: &str, left: &str, right: &str) {
    let mut expected = [left, right];
    expected.sort();
    match error {
        ElabError::DeclarationIdentityCollision {
            identity,
            first,
            second,
            ..
        } => {
            assert_eq!(identity, key);
            assert_eq!([first.as_str(), second.as_str()], expected);
        }
        other => panic!("expected a checked identity collision at {key}, got {other:?}"),
    }
}

/// Promise class: durable invariant. MEASURED: the real source window refuses
/// a constructor and dictionary sharing one key in either order and leaves
/// the first checked ID bound. CLAIMED: registration order cannot overwrite
/// either declaration's identity. THE GAP: this tests one explicit-data
/// constructor and one named instance; the predecessor suite pins R1/R2.
#[test]
fn constructor_and_dictionary_refuse_in_both_orders_without_displacing_prior() {
    for (first, second) in [(INSTANCE, CONSTRUCTOR), (CONSTRUCTOR, INSTANCE)] {
        let mut env = ElabEnv::new().expect("prelude");
        env.elaborate_file_v1(&format!("{PRE}{first}\n"))
            .expect("class, carrier and first declaration admitted");
        let prior = env.globals["Lbl_instance_A"];
        let error = env
            .elaborate_file_v1(second)
            .expect_err("different identity must refuse");
        assert_collision(
            error,
            "Lbl_instance_A",
            "constructor Lbl_instance_A of data D",
            "instance Lbl A",
        );
        assert_eq!(env.globals["Lbl_instance_A"], prior);
        assert!(env.env.lookup(prior).is_some() || env.env.constructor(prior).is_some());
    }
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file_v1(PRE).expect("class and carrier");
    env.elaborate_file_v1(INSTANCE)
        .expect("dictionary admitted");
    let dictionary = env.globals["Lbl_instance_A"];
    env.elaborate_decl_v1("data D : Type where { DistinctD1Ctor : D }")
        .expect("a distinctly spelled constructor does not collide");
    assert_eq!(env.globals["Lbl_instance_A"], dictionary);
    assert!(env.env.constructor(env.globals["DistinctD1Ctor"]).is_some());
}

/// Promise class: durable invariant. MEASURED: a law-generated field and a
/// user-spelled const with the same key refuse in both orders while the old
/// checked binding remains. CLAIMED: law-field provenance is distinguishable
/// from source spelling without relying on the dictionary naming convention.
/// THE GAP: this exercises one field and one law name; the enumerator covers
/// all fields of each checked law.
#[test]
fn law_field_and_source_const_refuse_in_both_orders() {
    let law = "law Foo (x) { a : Top }";
    let source = "const Foo_a : Int = 0";
    for (first, second) in [(law, source), (source, law)] {
        let mut env = ElabEnv::new().expect("prelude");
        env.elaborate_decl_v1(first)
            .expect("first declaration admitted");
        let prior = env.globals["Foo_a"];
        let error = env
            .elaborate_decl_v1(second)
            .expect_err("mixed spelling must refuse");
        assert_collision(error, "Foo_a", "const Foo_a", "law field Foo.a");
        assert_eq!(env.globals["Foo_a"], prior);
    }
    let mut env = ElabEnv::new().expect("prelude");
    let first = env.elaborate_decl_v1(law).expect("first law admitted");
    let old_field = env.globals["Foo_a"];
    let second = env
        .elaborate_decl_v1(law)
        .expect("equal minted field may rebind");
    assert_ne!(first.def_id, second.def_id);
    assert_ne!(old_field, env.globals["Foo_a"]);
    assert!(
        env.env.lookup(old_field).is_some(),
        "old checked field survives"
    );
}

/// Promise class: durable invariant. MEASURED: a generated space-operation
/// key and an independently written const in a module of that name refuse
/// in either order. CLAIMED: dot-qualified minted symbols and source names
/// cannot overwrite one another. THE GAP: this uses a one-cell space and
/// a zero-argument operation; full space behavior has its own suite.
#[test]
fn space_operation_and_source_const_refuse_in_both_orders() {
    let space = "space S { mut cell : Int = 0 proc read () : Int visits [S] = cell }";
    let source = "module S { const read : Int = 0 }";
    for (first, second) in [(space, source), (source, space)] {
        let mut env = ElabEnv::new().expect("prelude");
        env.elaborate_file_v1(first)
            .expect("first declaration admitted");
        let prior = env.globals["S.read"];
        let error = env
            .elaborate_file_v1(second)
            .expect_err("mixed spelling must refuse");
        assert_collision(error, "S.read", "const S.read", "space operation S.read");
        assert_eq!(env.globals["S.read"], prior);
    }
}

/// Promise class: durable invariant. MEASURED: after an earlier source key
/// is displaced by the first law field, a later bad field fails kernel
/// checking and the prior checked ID remains bound. CLAIMED: rollback is
/// transactional even when the declaration failed before identity comparison.
/// THE GAP: only a postulate field is used; the same window covers all kinds.
#[test]
fn failed_law_after_displacement_restores_the_prior_checked_identity() {
    let mut env = ElabEnv::new().expect("prelude");
    let old = env
        .elaborate_decl_v1("const Foo_a : Int = 0")
        .expect("checked const")
        .def_id;
    let error = env
        .elaborate_decl_v1("law Foo (x) { a : Top ; b : True }")
        .expect_err("the second field is a Bool, not a proposition");
    assert!(
        matches!(
            error,
            ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                ..
            }
        ),
        "the first field must have been inserted before kernel failure: {error:?}",
    );
    assert_eq!(env.globals["Foo_a"], old);
    assert!(env.env.lookup(old).is_some());
}

/// Promise class: durable invariant. MEASURED: both data-former routes bind
/// their complete checked constructor population in the ordinary source
/// window. CLAIMED: omitting a constructor from the declared-key enumerator
/// cannot silently pass. THE GAP: the compile-preserving AC-5 enumerator
/// mutation must redden both rows with the exact undeclared-keys refusal.
#[test]
fn both_data_forms_bind_every_declared_constructor() {
    for (data, keys) in [
        (
            "data Legacy = LegacyFirst | LegacySecond",
            ["LegacyFirst", "LegacySecond"],
        ),
        (
            "data Explicit : Type where { ExplicitFirst : Explicit; ExplicitSecond : Explicit }",
            ["ExplicitFirst", "ExplicitSecond"],
        ),
    ] {
        let mut env = ElabEnv::new().expect("prelude");
        env.elaborate_decl_v1(data)
            .expect("whole data family admitted");
        for key in keys {
            let id = env.globals[key];
            assert!(
                env.env.constructor(id).is_some(),
                "{key} must be a checked constructor"
            );
        }
    }
}
