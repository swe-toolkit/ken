//! ES3-build acceptance tests: modules/imports/visibility made real in the
//! elaborator (`spec/30-surface/33-declarations.md` §3-4), netted against
//! `conformance/surface/modules/seed-modules.md`'s 7 discriminating cases.
//!
//! Producer-grep discipline (the WP's load-bearing gate): every case here
//! drives the **real** `crates/ken-elaborator/src/modules.rs` expansion +
//! resolution path via `ElabEnv::elaborate_file`/`elaborate_decl` — never a
//! hand-constructed `M.foo -> GlobalId` binding.

use std::fs;

use ken_elaborator::modules::catalog_module_from_path;
use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::env::Decl as KernelDecl;
use ken_kernel::{Level, Term};

fn mk_env() -> ElabEnv {
    ElabEnv::new().expect("base env construction failed")
}

// ─────────────────────────────────────────────────────────────────────────
// A. Modules elaborate away — zero TCB delta (AC1 ★)
// ─────────────────────────────────────────────────────────────────────────

/// `module-elaborates-to-identical-flat-sigma`: a `module`/`import` program
/// and its fully-qualified single-namespace equivalent produce the
/// **identical** flat `Σ` / `trusted_base()` — discriminating on that
/// identity, not "both type-check" (which would pass vacuously even if a
/// design leaked a kernel-level module/visibility primitive).
#[test]
fn module_elaborates_to_identical_flat_sigma() {
    let mut a = mk_env();
    a.elaborate_file(
        "module M { pub const foo : Int = 0 } \
         import M \
         const bar : Int = M.foo",
    )
    .expect("module program elaborates");

    let mut b = mk_env();
    b.elaborate_file(
        "const M_foo : Int = 0 \
         const bar : Int = M_foo",
    )
    .expect("flat equivalent elaborates");

    assert_eq!(
        a.env.decls().count(),
        b.env.decls().count(),
        "AC1: a module program must add exactly the same NUMBER of Σ decls \
         as its flat equivalent — no extra kernel-level module/visibility entry"
    );
    assert_eq!(
        a.env.trusted_base(),
        b.env.trusted_base(),
        "AC1: module wrapping must not perturb trusted_base() at all"
    );

    let fixture = std::env::temp_dir().join(format!(
        "ken-es3-loader-flat-sigma-{}/catalog/packages",
        std::process::id()
    ));
    let fixture_parent = fixture.parent().expect("fixture has parent");
    let _ = fs::remove_dir_all(fixture_parent);
    fs::create_dir_all(&fixture).expect("create catalog fixture");
    fs::write(fixture.join("M.ken"), "pub const foo : Int = 0")
        .expect("write provider");
    let core_logic = fixture.join("Core/Logic");
    fs::create_dir_all(&core_logic).expect("create Core.Logic fixture path");
    fs::write(
        core_logic.join("Or.ken"),
        "data Or (a : Omega) (b : Omega) : Type where { \
           Inl : a -> Or a b; Inr : b -> Or a b \
         } export Or, Inl, Inr",
    )
    .expect("write proof-relevant Or provider");
    let entry_path = fixture.join("Entry.ken");
    fs::write(&entry_path, "import M\nconst bar : Int = M.foo")
        .expect("write entry");
    let address = catalog_module_from_path(&entry_path)
        .expect("derive catalog module address");
    let mut c = mk_env();
    c.elaborate_module_from_roots(&[address.root.clone()], &address.entry)
        .expect("roots-loaded module program elaborates");

    assert_eq!(
        c.env.decls().count(),
        b.env.decls().count(),
        "AC1: the roots-loaded entry and dependency add the same flat Σ count"
    );
    assert_eq!(
        c.env.trusted_base(),
        b.env.trusted_base(),
        "AC1: roots loading must preserve the flattened zero-trust boundary"
    );
    let provider = c.globals["M.foo"];
    let (_, imported_body) = c.env
        .transparent_body(c.globals["Entry.bar"])
        .expect("entry binding is transparent");
    assert!(
        matches!(imported_body, Term::Const { id, .. } if id == provider),
        "the imported name must retain the provider's existing GlobalId"
    );

    let mut strict = mk_env();
    strict
        .elaborate_module_from_roots_strict(&[address.root.clone()], &address.entry)
        .expect("opt-in strict roots program elaborates");
    assert_eq!(
        strict.env.decls().count(),
        b.env.decls().count(),
        "strict resolution adds no kernel declaration to flat Sigma"
    );
    assert_eq!(
        strict.env.trusted_base(),
        b.env.trusted_base(),
        "strict resolution preserves the zero-trust boundary"
    );
    let strict_provider = strict.globals["M.foo"];
    let (_, strict_body) = strict
        .env
        .transparent_body(strict.globals["Entry.bar"])
        .expect("strict entry binding is transparent");
    assert!(
        matches!(strict_body, Term::Const { id, .. } if id == strict_provider),
        "strict import must retain the provider's existing GlobalId"
    );

    // Extend the same flat-Sigma pin over NODE B's exact relevant-data shape.
    // Legacy and strict roots loading both erase the module/export layer; the
    // flat program emits the identical three kernel declarations.
    strict
        .elaborate_module_from_roots_strict(&[address.root], "Core.Logic.Or")
        .expect("strict roots load proof-relevant Or");
    b.elaborate_file(
        "data Or (a : Omega) (b : Omega) : Type where { \
           Inl : a -> Or a b; Inr : b -> Or a b \
         }",
    )
    .expect("flat proof-relevant Or elaborates");
    assert_eq!(
        strict.env.decls().count(),
        b.env.decls().count(),
        "strict roots Or adds only its flat family and two constructors"
    );
    assert_eq!(strict.env.trusted_base(), b.env.trusted_base());
    fs::remove_dir_all(fixture_parent).expect("remove catalog fixture");
}

// ─────────────────────────────────────────────────────────────────────────
// B. Abstract export is owner-transparent and client-opaque (AC2)
// ─────────────────────────────────────────────────────────────────────────

/// Durable invariant: abstract export is two-faced. The defining module
/// elaborates the ordinary checked inductive and may use its constructor,
/// while the client surface exports only the type name. The client's nullary
/// type former has the same kind as a hand-written opaque declaration and the
/// module layer adds no trust.
///
/// MEASURED: the owner constructs and matches `MkT`, the client calls the
/// public operations but gets an exact surface rejection for `M.MkT`, the type
/// kind matches an independent opaque declaration, and trust stays unchanged.
/// CLAIMED: nullary abstract export preserves the owner/client split.
/// THE GAP: the client operation call must share the exported type identity;
/// mere owner success plus constructor rejection would not establish that.
#[test]
fn nullary_abstract_export_is_transparent_to_owner_and_opaque_to_client() {
    let mut env = mk_env();
    let trusted_before = env.env.trusted_base();
    env.elaborate_file(
        "module M { \
           pub data T = MkT \
           pub const make : T = MkT \
           pub fn inspect (value : T) : Int = match value { MkT |-> 0 } \
         }",
    )
    .expect("the defining module retains transparent constructor access");
    let t_id = env.globals["M.T"];

    let (_, t_kind) = env.env.const_type(t_id).expect("exported type has a kind");
    assert_eq!(
        t_kind,
        Term::ty(Level::Zero),
        "the nullary abstract type keeps the hand-written opaque kind"
    );
    assert!(
        matches!(env.env.lookup(t_id), Some(KernelDecl::Inductive(_))),
        "the defining module must elaborate the real inductive"
    );
    assert!(
        env.globals.contains_key("M.MkT"),
        "the defining module's constructor must exist in the flat checked environment"
    );

    env.elaborate_decl("const observed : Int = M.inspect M.make")
        .expect("a client may consume the opaque surface through public operations");
    match env.elaborate_decl("const forbidden : M.T = M.MkT") {
        Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, "M.MkT"),
        other => panic!("the qualified hidden constructor must reject at the surface: {other:?}"),
    }
    assert_eq!(
        env.env.trusted_base(),
        trusted_before,
        "module abstract export must add no trusted declaration"
    );
}

/// Regression (language-qa, `evt_6pp9m18vp5bj6`): abstract export is a
/// `module { … }`-only concept — there is no "outside" to hide from at the
/// true file root. A top-level `pub data T = MkT` (no enclosing module)
/// must NOT be silently reinterpreted as an opaque constant; `MkT` stays a
/// real, constructible/matchable constructor, exactly as an unmarked
/// top-level `data` would (matching `pub`'s already-inert behavior on
/// top-level `View`/`Let`/`TypeAlias`).
#[test]
fn top_level_pub_data_is_not_abstract_exported() {
    let mut env = mk_env();
    env.elaborate_file("pub data T = MkT").expect("top-level pub data elaborates");

    let t_id = env.globals["T"];
    assert!(
        env.env.inductive(t_id).is_some(),
        "a top-level `pub data T` must stay a real inductive, not become an opaque constant"
    );
    assert!(env.globals.contains_key("MkT"), "the constructor must remain registered");

    // The constructor must still be constructible AND matchable in the
    // same compilation unit — the exact capability the defect silently
    // destroyed.
    env.elaborate_decl("const mk : T = MkT").expect("MkT must be constructible");
    env.elaborate_decl("fn unwrap (t : T) : Int = match t { MkT |-> 0 }")
        .expect("MkT must be matchable");
}

/// `client-match-hidden-ctor-rejected-at-surface`: a client that `import`s
/// `M` and attempts to `match` on the withheld constructor is rejected at
/// the surface — the constructor was never registered, so this fails
/// during surface elaboration, never reaching the kernel.
#[test]
fn client_match_hidden_ctor_rejected_at_surface() {
    let mut env = mk_env();
    env.elaborate_file("module M { pub data T = MkT }").expect("module M elaborates");

    match env.elaborate_decl("fn bad (t : M.T) : Int = match t { MkT |-> 0 }") {
        Err(ElabError::UnresolvedCon { name, .. }) => assert_eq!(name, "MkT"),
        other => panic!("AC2: hidden constructor match must be a surface rejection: {other:?}"),
    }
}

/// Durable invariant: a parameterized abstract export retains the full kind,
/// and its public operations and attached proof elaborate against the same
/// owner-visible inductive identity. Clients can use those operations with the
/// exported type but cannot name the raw constructor in construction or match.
///
/// MEASURED: the complete NonEmpty append and associativity proof check in the
/// owner, its exported kind is the literal one-parameter Pi kind, public smart
/// construction and elimination check at a client, both constructor uses get
/// exact surface errors, and module trust stays unchanged.
/// CLAIMED: parameterized abstract export preserves arity and both faces.
/// THE GAP: the proof and client operations must use the same family identity;
/// independent owner/client lookalikes would not establish the claim.
#[test]
fn parameterized_abstract_export_preserves_kind_and_owner_proof() {
    let mut env = mk_env();
    env.elaborate_decl(
        "axiom cong : \
           (a : Type) -> (b : Type) -> (x : a) -> (y : a) -> \
           (f : a -> b) -> Equal a x y -> Equal b (f x) (f y)",
    )
    .expect("fixture supplies Transport.cong's public type");
    let trusted_before = env.env.trusted_base();
    env.elaborate_file(
        r#"
        fn list_append (a : Type) (xs : List a) (ys : List a) : List a =
          match xs {
            Nil |-> ys;
            Cons h rest |-> Cons a h (list_append a rest ys)
          }

        proof assoc for list_append
              (a : Type) (xs : List a) (ys : List a) (zs : List a)
            : Equal
                (List a)
                (list_append a (list_append a xs ys) zs)
                (list_append a xs (list_append a ys zs)) =
          match xs {
            Nil |-> Refl;
            Cons h rest |->
              cong
                (List a)
                (List a)
                (list_append a (list_append a rest ys) zs)
                (list_append a rest (list_append a ys zs))
                (Cons a h)
                ((proof assoc for list_append) a rest ys zs)
          }

        module M {
          pub data NonEmpty a = NonEmptyCons a (List a)

          pub fn nonempty_singleton (a : Type) (x : a) : NonEmpty a =
            NonEmptyCons a x (Nil a)

          pub fn nonempty_head (a : Type) (xs : NonEmpty a) : a =
            match xs { NonEmptyCons x rest |-> x }

          pub fn nonempty_tail (a : Type) (xs : NonEmpty a) : List a =
            match xs { NonEmptyCons x rest |-> rest }

          pub fn nonempty_append
              (a : Type) (xs : NonEmpty a) (ys : NonEmpty a) : NonEmpty a =
            match xs {
              NonEmptyCons x rest |->
                match ys {
                  NonEmptyCons y more |->
                    NonEmptyCons a x (list_append a rest (Cons a y more))
                }
            }

          pub proof assoc for nonempty_append
                (a : Type) (xs : NonEmpty a) (ys : NonEmpty a) (zs : NonEmpty a)
              : Equal
                  (NonEmpty a)
                  (nonempty_append a (nonempty_append a xs ys) zs)
                  (nonempty_append a xs (nonempty_append a ys zs)) =
            match xs {
              NonEmptyCons x rest |->
                match ys {
                  NonEmptyCons y more |->
                    match zs {
                      NonEmptyCons z last |->
                        cong
                          (List a)
                          (NonEmpty a)
                          (list_append a (list_append a rest (Cons a y more)) (Cons a z last))
                          (list_append a rest (Cons a y (list_append a more (Cons a z last))))
                          (NonEmptyCons a x)
                          (list_append::assoc a rest (Cons a y more) (Cons a z last))
                    }
                }
            }
        }
        "#,
    )
    .expect("the exact NonEmpty owner definition and associativity proof elaborate");

    let nonempty_id = env.globals["M.NonEmpty"];
    let (_, exported_kind) = env
        .env
        .const_type(nonempty_id)
        .expect("parameterized abstract type has a kind");
    let expected_kind = Term::pi(Term::ty(Level::Zero), Term::ty(Level::Zero));
    assert_eq!(
        exported_kind, expected_kind,
        "the abstract export must retain the hand-written opaque Pi kind"
    );

    env.elaborate_decl("const one : M.NonEmpty Nat = M.nonempty_singleton Nat Zero")
        .expect("a client may construct through the public smart constructor");
    env.elaborate_decl("const first : Nat = M.nonempty_head Nat one")
        .expect("a client may consume the same exposed type through a public operation");

    match env.elaborate_decl("const forbidden : M.NonEmpty Nat = M.NonEmptyCons Nat Zero (Nil Nat)")
    {
        Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, "M.NonEmptyCons"),
        other => panic!("client construction with the hidden constructor must reject: {other:?}"),
    }
    match env.elaborate_decl(
        "fn forbidden_match (xs : M.NonEmpty Nat) : Nat = \
           match xs { NonEmptyCons x rest |-> x }",
    ) {
        Err(ElabError::UnresolvedCon { name, .. }) => assert_eq!(name, "NonEmptyCons"),
        other => panic!("client match with the hidden constructor must reject: {other:?}"),
    }
    assert_eq!(
        env.env.trusted_base(),
        trusted_before,
        "the real inductive and module visibility add no trust"
    );
}

/// Durable invariant: the strict roots loader gives a provider its transparent
/// owner face while importing only its abstract public face into a distinct
/// client unit. This is the cross-unit path that catalog packages use.
///
/// MEASURED: strict roots loading checks an owner constructor and a client
/// smart-constructor/eliminator pair, retains the Pi kind, rejects the hidden
/// qualified constructor, and leaves trust unchanged.
/// CLAIMED: the two-face invariant survives the real cross-unit loader.
/// THE GAP: the provider and client must be distinct loader units; the same-file
/// control above cannot establish the import/export boundary by itself.
#[test]
fn strict_roots_parameterized_abstract_export_keeps_two_faces() {
    let fixture = std::env::temp_dir().join(format!(
        "ken-abstract-export-param-{}/catalog/packages",
        std::process::id()
    ));
    let fixture_parent = fixture.parent().expect("fixture has parent");
    let _ = fs::remove_dir_all(fixture_parent);
    fs::create_dir_all(&fixture).expect("create catalog fixture");
    fs::write(
        fixture.join("M.ken"),
        "pub data Boxed a = MkBoxed a\n\
         pub fn make (a : Type) (value : a) : Boxed a = MkBoxed a value\n\
         pub fn get (a : Type) (boxed : Boxed a) : a = \
           match boxed { MkBoxed value |-> value }\n",
    )
    .expect("write abstract provider");
    fs::write(
        fixture.join("Entry.ken"),
        "import M (Boxed, make, get)\n\
         const one : Boxed Nat = make Nat Zero\n\
         const observed : Nat = get Nat one\n",
    )
    .expect("write abstract client");

    let mut env = mk_env();
    let trusted_before = env.env.trusted_base();
    env.elaborate_module_from_roots_strict(&[fixture.clone()], "Entry")
        .expect("strict roots provider and client elaborate through the two faces");
    let boxed_id = env.globals["M.Boxed"];
    let (_, boxed_kind) = env.env.const_type(boxed_id).expect("Boxed has a kind");
    assert_eq!(
        boxed_kind,
        Term::pi(Term::ty(Level::Zero), Term::ty(Level::Zero)),
        "the strict client sees the complete parameterized type former"
    );
    assert!(
        env.globals.contains_key("M.MkBoxed"),
        "the provider's checked constructor remains in its owner environment"
    );
    assert_eq!(
        env.env.trusted_base(),
        trusted_before,
        "strict cross-unit abstract export must add no trust"
    );
    match env.elaborate_decl("const forbidden : M.Boxed Nat = M.MkBoxed Nat Zero") {
        Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, "M.MkBoxed"),
        other => panic!("strict client must not name the provider constructor: {other:?}"),
    }
    fs::remove_dir_all(fixture_parent).expect("remove catalog fixture");
}

// ─────────────────────────────────────────────────────────────────────────
// C. Visibility + resolution — surface-only, well-defined (AC3/AC4)
// ─────────────────────────────────────────────────────────────────────────

/// `private-name-access-rejected-at-surface` (+ AC4 witness): a non-`pub`
/// name is module-private; a client's qualified reference to it fails at
/// the surface, while the `pub` sibling resolves.
#[test]
fn private_name_access_rejected_at_surface() {
    let mut env = mk_env();
    env.elaborate_file(
        "module M { const secret : Int = 0 pub const api : Int = 1 } \
         import M",
    )
    .expect("module M elaborates");

    let ok = env.elaborate_decl("const getApi : Int = M.api");
    assert!(ok.is_ok(), "AC3/AC4: M.api (pub) must resolve");

    let bad = env.elaborate_decl("const getSecret : Int = M.secret");
    assert!(bad.is_err(), "AC3/AC4: M.secret (private) must be rejected");
    match bad.unwrap_err() {
        ElabError::KernelRejected { .. } => {
            panic!("AC3: private-name rejection must be surface, never kernel")
        }
        _ => {}
    }
}

/// Two selective imports binding the same bare name to different declarations
/// reject latently at the second binding (`33 §3.3`), even when no later
/// expression references the name.
#[test]
fn selective_import_ambiguity_rejected_naming_both() {
    let mut env = mk_env();
    let bad = env.elaborate_file(
        "module M { pub const foo : Int = 0 } \
         module N { pub const foo : Int = 1 } \
         import M (foo) \
         import N (foo)",
    );
    match bad {
        Err(ElabError::AmbiguousReference { name, sources, .. }) => {
            assert_eq!(name, "foo");
            assert!(sources.contains(&"M.foo".to_string()));
            assert!(sources.contains(&"N.foo".to_string()));
        }
        other => panic!("AC3: expected AmbiguousReference naming both M.foo and N.foo, got {:?}", other),
    }
}

/// N3 reversal: a TOP-LEVEL local and a selective import of the same bare name
/// clash even when the name is never referenced. This was ES3's local-wins
/// seed; N3 deliberately flips it while retaining narrower lexical shadowing.
#[test]
fn top_level_local_import_clash_is_rejected_latently() {
    let mut env = mk_env();
    let result = env.elaborate_file(
        "module M { pub const foo : Int = 0 } \
         import M (foo) \
         const foo : Int = 9",
    );
    match result {
        Err(ElabError::AmbiguousReference { name, sources, .. }) => {
            assert_eq!(name, "foo");
            assert!(sources.contains(&"foo".to_string()));
            assert!(sources.contains(&"M.foo".to_string()));
        }
        other => panic!("N3: expected latent top-level clash, got {other:?}"),
    }
}

/// `three-import-forms-resolve-to-one-binding`: qualified / aliased /
/// selective all resolve to the **same** underlying `GlobalId` — the
/// accept anchor confirming import is re-naming, not re-declaration.
#[test]
fn three_import_forms_resolve_to_one_binding() {
    let mut env = mk_env();
    env.elaborate_file("module M { pub const foo : Int = 0 }").expect("module M elaborates");
    let m_foo = env.globals["M.foo"];

    env.elaborate_file("import M").unwrap();
    let via_qualified = env
        .elaborate_decl("const c1 : Int = M.foo")
        .expect("import M / qualified M.foo");
    let (_, b1) = env.env.transparent_body(via_qualified).unwrap();

    env.elaborate_file("import M as N").unwrap();
    let via_aliased = env.elaborate_decl("const c2 : Int = N.foo").expect("import M as N");
    let (_, b2) = env.env.transparent_body(via_aliased).unwrap();

    env.elaborate_file("import M (foo)").unwrap();
    let via_selective = env.elaborate_decl("const c3 : Int = foo").expect("import M (foo)");
    let (_, b3) = env.env.transparent_body(via_selective).unwrap();

    for (label, body) in [("qualified", &b1), ("aliased", &b2), ("selective", &b3)] {
        assert!(
            matches!(body, Term::Const { id, .. } if *id == m_foo),
            "AC3/AC1: the {} import form must resolve to the SAME GlobalId as \
             `M.foo` (re-naming, not re-declaration); got {:?}",
            label, body
        );
    }
}

#[test]
fn retired_use_reports_the_migration_diagnostic() {
    let mut env = mk_env();
    let result = env.elaborate_file("use Capability.Parsing.Parsing");
    match result {
        Err(ElabError::ParseError { msg, span }) => {
            assert_eq!(
                msg,
                "`use` is retired (ADR-0015); use `import M`, `import M as N`, or \
                 `import M (…)` for a provenance-preserving import."
            );
            assert_eq!(span, ken_elaborator::Span::new(0, 3));
        }
        other => panic!("expected the specific retired-`use` ParseError, got {other:?}"),
    }
}
