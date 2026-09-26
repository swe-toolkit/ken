//! Raw postulates preserve import precedence when their spelling is imported.

use ken_elaborator::ElabEnv;
use ken_kernel::Term;

/// Promise class: durable invariant. MEASURED: a raw postulate can write the
/// flat spelling occupied by an imported alias, while the next source unit
/// still selects the imported provider's checked identity. CLAIMED: session
/// ID recording does not change the raw API's admission result or import
/// precedence. THE GAP: this pair does not cover other collision classes;
/// ordinary raw rebinding is tested in lang_session_scope.
#[test]
fn raw_postulate_over_imported_alias_preserves_write_and_import_selection() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(
        "module Provider { pub const item : Nat = Zero } \
         import Provider (item)",
    )
    .expect("provider and selective import");
    let provider = env.globals["Provider.item"];
    let nat = env.globals["Nat"];
    let raw = env
        .declare_postulate_raw("item", Term::const_(nat, vec![]))
        .expect("raw API succeeds on imported alias, as on base");
    assert_ne!(
        provider, raw,
        "raw writer minted a distinct checked identity"
    );
    assert_eq!(env.globals["item"], raw, "legacy flat writer overwrites");
    let selected = env
        .elaborate_decl("const selected : Nat = item")
        .expect("import remains the checked selection");
    let (_, body) = env.env.transparent_body(selected).expect("checked term");
    assert_eq!(body, Term::const_(provider, vec![]));
}
