//! Qualified constructor identity and scoped ResourceKind (`32 §4`, `33 §3.3`, `34 §1.1`).

use std::fs;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{GlobalId, KernelError, Term};

fn env() -> ElabEnv {
    ElabEnv::new().expect("checked prelude")
}

fn body(env: &ElabEnv, name: &str) -> Term {
    env.env
        .transparent_body(env.globals[name])
        .expect("checked definition has a body")
        .1
}

fn assert_constructor(term: Term, id: GlobalId) {
    assert!(
        matches!(&term, Term::Constructor { id: actual, .. } if *actual == id),
        "the checked expression must hold constructor {id:?}, got {term:?}"
    );
}

/// Promise class: durable invariant. MEASURED: `Resource ResourceKind.Buffer`
/// in a binder and result type produces the exact checked Pi that the
/// pre-scoping `Resource Buffer` spelling produced on base 0a94e80dc:
/// `Pi (Resource Buffer) (Resource Buffer)` with the same canonical IDs.
/// CLAIMED: T.C is an atomic type argument selecting C's checked identity.
/// THE GAP: other-parent and dual-meaning paths have independent controls.
#[test]
fn qualified_resource_type_matches_prescoping_bare_checked_term() {
    let mut env = env();
    let trust = env.env.trusted_base();
    let resource = env.globals["Resource"];
    let buffer = env.prelude_env.runtime_roles.resource_kind_buffer;
    env.elaborate_decl(
        "fn hold_buffer (r : Resource ResourceKind.Buffer) : Resource ResourceKind.Buffer = r",
    )
    .expect("qualified constructor is admitted as a type argument");
    let (_, actual) = env
        .env
        .const_type(env.globals["hold_buffer"])
        .expect("checked type");
    let resource_buffer = Term::app(
        Term::const_(resource, vec![]),
        Term::constructor(buffer, vec![]),
    );
    let expected = Term::pi(resource_buffer.clone(), resource_buffer);
    assert_eq!(
        actual, expected,
        "qualified type must preserve the bare-era kernel term"
    );
    assert_eq!(env.env.trusted_base(), trust);
}

/// Promise class: durable invariant. MEASURED: another data type's Buffer
/// resolves as its own checked constructor but cannot index Resource; the
/// error is kernel type mismatch, not a parser refusal. CLAIMED: a matching
/// leaf cannot donate the ResourceKind parent's type-argument identity.
/// THE GAP: the qualified ResourceKind.Buffer positive runs separately.
#[test]
fn other_parent_buffer_cannot_index_resource_type() {
    let mut env = env();
    let floor_buffer = env.prelude_env.runtime_roles.resource_kind_buffer;
    env.elaborate_file(
        "data RivalKind = Buffer\n\
         const rival : RivalKind = RivalKind.Buffer",
    )
    .expect("same-named rival constructor is a checked positive");
    let rival_buffer = env.globals["Buffer"];
    assert_ne!(rival_buffer, floor_buffer);
    assert_eq!(
        env.env
            .constructor(rival_buffer)
            .expect("checked rival constructor")
            .0
            .id,
        env.globals["RivalKind"]
    );
    assert_constructor(body(&env, "rival"), rival_buffer);
    env.elaborate_decl(
        "fn own (r : Resource ResourceKind.Buffer) : Resource ResourceKind.Buffer = r",
    )
    .expect("the floor kind is a valid type index in the same environment");
    let error = env
        .elaborate_decl("fn wrong (r : Resource RivalKind.Buffer) : Resource RivalKind.Buffer = r")
        .expect_err("a constructor of RivalKind cannot index ResourceKind");
    assert!(
        matches!(
            error,
            ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                ..
            }
        ),
        "wrong other-parent type-position refusal: {error:?}"
    );
}

/// Promise class: durable invariant. MEASURED: a module-only Kind.Buffer
/// checks as a Resource index; importing a distinct type Kind with its own
/// Buffer then makes that same type-argument spelling AmbiguousReference.
/// CLAIMED: type position never breaks a module/type tie by parser order.
/// THE GAP: both candidates must actually be visible at the conflict site.
#[test]
fn module_and_type_constructor_conflict_in_type_argument() {
    let mut env = env();
    env.elaborate_file(
        "module Source { data Kind = Buffer; export Kind, Buffer }\n\
         module Kind { pub const Buffer : ResourceKind = ResourceKind.Buffer }\n\
         import Kind",
    )
    .expect("one checked module Kind.Buffer is initially visible");
    env.elaborate_decl("fn module_only (r : Resource Kind.Buffer) : Resource Kind.Buffer = r")
        .expect("module-only type argument is well formed");
    let source_buffer = env.globals["Source.Buffer"];
    assert_eq!(
        env.env
            .constructor(source_buffer)
            .expect("checked source constructor")
            .0
            .id,
        env.globals["Source.Kind"]
    );
    assert_ne!(
        source_buffer,
        env.prelude_env.runtime_roles.resource_kind_buffer
    );
    env.elaborate_file("import Source (Kind)")
        .expect("second type meaning is visible");
    let error = env
        .elaborate_decl("fn clash (r : Resource Kind.Buffer) : Resource Kind.Buffer = r")
        .expect_err("type argument has two real namespace meanings");
    assert!(
        matches!(error, ElabError::AmbiguousReference { ref name, .. }
            if name == "Kind.Buffer"),
        "wrong type-position ambiguity: {error:?}"
    );
}

/// Promise class: durable invariant. MEASURED: local qualified and bare
/// expressions contain the same checked constructor ID. CLAIMED: T.C is a
/// type-family selector, never a fresh or spelling-derived declaration.
/// THE GAP: independent pattern, prelude, and imported-type paths follow.
#[test]
fn local_qualified_constructor_expression_preserves_bare_id() {
    let mut env = env();
    let trust = env.env.trusted_base();
    env.elaborate_file(
        "data Colour = Red | Blue\n\
         const qualified_red : Colour = Colour.Red\n\
         const bare_red : Colour = Red",
    )
    .expect("a local type admits both forms");
    let red = env.globals["Red"];
    assert_eq!(
        env.env.constructor(red).expect("real constructor").0.id,
        env.globals["Colour"]
    );
    assert_constructor(body(&env, "qualified_red"), red);
    assert_constructor(body(&env, "bare_red"), red);
    assert_eq!(env.env.trusted_base(), trust);
}

/// Promise class: durable invariant. MEASURED: a qualified pattern arm
/// elaborates to the same checked match as bare arms, and missing an arm is
/// refused. CLAIMED: coverage tracks IDs, not written constructor heads.
/// THE GAP: the closed two-constructor fixture cannot prove nested patterns.
#[test]
fn local_qualified_constructor_pattern_uses_identity_for_coverage() {
    let mut env = env();
    env.elaborate_file(
        "data Colour = Red | Blue\n\
         fn qualified (c : Colour) : Nat = match c {\
           Colour.Red |-> Zero; Colour.Blue |-> Suc Zero }\n\
         fn bare (c : Colour) : Nat = match c {\
           Red |-> Zero; Blue |-> Suc Zero }",
    )
    .expect("qualified and bare matches are checked");
    assert_eq!(body(&env, "qualified"), body(&env, "bare"));
    let error = env
        .elaborate_decl("fn incomplete (c : Colour) : Nat = match c { Colour.Red |-> Zero }")
        .expect_err("omitting Blue must remain nonexhaustive");
    assert!(
        matches!(error, ElabError::ExhaustivenessError { .. }),
        "wrong missing-arm result: {error:?}"
    );
}

/// Promise class: durable invariant. MEASURED: the prelude's Bool.True
/// expression carries the existing True constructor ID. CLAIMED: a prelude
/// type also admits qualification without changing the kernel declaration.
/// THE GAP: independent pattern and scope controls exercise the other paths.
#[test]
fn prelude_qualified_constructor_expression_keeps_true_id() {
    let mut env = env();
    let trust = env.env.trusted_base();
    env.elaborate_decl("const qualified_true : Bool = Bool.True")
        .expect("prelude Bool.True resolves");
    assert_constructor(body(&env, "qualified_true"), env.globals["True"]);
    assert_eq!(env.env.trusted_base(), trust);
}

/// Promise class: durable invariant. MEASURED: qualified Bool patterns
/// compile to the same checked branch coverage as bare True and False.
/// CLAIMED: imported and prelude constructors share an ID-keyed pattern path.
/// THE GAP: imported selection and privacy have separate controls.
#[test]
fn prelude_qualified_constructor_patterns_cover_bool() {
    let mut env = env();
    env.elaborate_file(
        "fn qualified_bool (b : Bool) : Nat = match b {\
           Bool.True |-> Zero; Bool.False |-> Suc Zero }\n\
         fn bare_bool (b : Bool) : Nat = match b {\
           True |-> Zero; False |-> Suc Zero }",
    )
    .expect("all Bool arms resolve");
    assert_eq!(body(&env, "qualified_bool"), body(&env, "bare_bool"));
}

fn exported_palette() -> ElabEnv {
    let mut env = env();
    env.elaborate_file("module Palette { data Colour = Red | Blue; export Colour, Red, Blue }")
        .expect("the fixture explicitly exports type AND constructors");
    env
}

/// Promise class: durable invariant. MEASURED: a public imported type selects
/// the provider's constructor ID even through a type-only selective import.
/// CLAIMED: the member is selected by provider visibility and exact parent,
/// not by the caller's last global spelling. THE GAP: private and rival
/// providers are separately discriminated.
#[test]
fn imported_type_qualified_expression_keeps_provider_identity() {
    let mut env = exported_palette();
    let provider_red = env.globals["Palette.Red"];
    let provider_type = env.globals["Palette.Colour"];
    env.elaborate_file("import Palette")
        .expect("qualified provider import");
    env.elaborate_decl("const first : Palette.Colour = Palette.Colour.Red")
        .expect("module-qualified family constructor");
    assert_constructor(body(&env, "first"), provider_red);
    assert_eq!(
        env.env.constructor(provider_red).unwrap().0.id,
        provider_type
    );

    // A selective type import authorizes T.C only when C is on that
    // provider's public interface; it must not import bare C as a side effect.
    let mut selective = exported_palette();
    let selected_id = selective.globals["Palette.Red"];
    selective
        .elaborate_file("import Palette (Colour as Shade)")
        .expect("selective type import");
    selective
        .elaborate_decl("const chosen : Shade = Shade.Red")
        .expect("renamed imported type retains its public constructor");
    assert_constructor(body(&selective, "chosen"), selected_id);
}

/// Promise class: durable invariant. MEASURED: a public imported constructor
/// can be used in a checked pattern under its exported type path.
/// CLAIMED: the qualified pattern uses the provider's original constructor
/// identity; the following equality compares the checked bare-module path.
/// THE GAP: a second provider and a hidden constructor are separate controls.
#[test]
fn imported_type_qualified_pattern_preserves_provider_id() {
    let mut env = exported_palette();
    env.elaborate_file("import Palette")
        .expect("provider import");
    env.elaborate_file(
        "fn qualified (c : Palette.Colour) : Nat = match c {\
           Palette.Colour.Red |-> Zero; Palette.Colour.Blue |-> Suc Zero }\n\
         fn bare_module (c : Palette.Colour) : Nat = match c {\
           Palette.Red |-> Zero; Palette.Blue |-> Suc Zero }",
    )
    .expect("imported constructor patterns elaborate");
    assert_eq!(body(&env, "qualified"), body(&env, "bare_module"));
}

/// Promise class: durable invariant. MEASURED: real file-backed strict roots
/// select a provider's public constructor by exact ID in an expression and
/// a pattern, while the same syntax cannot expose a private constructor.
/// CLAIMED: provider visibility and identity survive the lazy roots loader.
/// THE GAP: the inline-module controls alone cannot cover file export tables.
#[test]
fn strict_roots_type_constructor_selection_keeps_public_and_private_faces() {
    let root = tempfile::tempdir().expect("isolated strict root");
    fs::write(
        root.path().join("M.ken"),
        "data Colour = Red | Blue\nexport Colour, Red, Blue",
    )
    .expect("write public provider");
    fs::write(
        root.path().join("Entry.ken"),
        "import M\nconst selected : M.Colour = M.Colour.Red\n\
         fn choose (x : M.Colour) : Nat = match x {\
           M.Colour.Red |-> Zero; M.Colour.Blue |-> Suc Zero }",
    )
    .expect("write strict consumer");
    let mut env = env();
    env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "Entry")
        .expect("public checked constructor reaches both positions");
    let selected = env.globals["M.Red"];
    assert_constructor(body(&env, "Entry.selected"), selected);
    assert_eq!(
        env.env.constructor(selected).unwrap().0.id,
        env.globals["M.Colour"]
    );

    fs::write(
        root.path().join("Hidden.ken"),
        "pub data Colour = Red | Blue",
    )
    .expect("write abstract provider");
    fs::write(
        root.path().join("Private.ken"),
        "import Hidden\nconst denied : Hidden.Colour = Hidden.Colour.Red",
    )
    .expect("write denied strict consumer");
    let error = ElabEnv::new()
        .expect("checked prelude")
        .elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "Private")
        .expect_err("a private constructor must not be recovered through T.C");
    assert!(
        matches!(error, ElabError::UnboundName { ref name, .. }
        if name == "Hidden.Colour.Red"),
        "wrong strict private refusal: {error:?}"
    );
}

/// Promise class: durable invariant. MEASURED: a distinct same-shaped
/// provider's constructor is refused when the expected type is Palette.Colour.
/// CLAIMED: no cross-family donation via a matching leaf or spelling.
/// THE GAP: independent positive selection proves both providers loaded.
#[test]
fn same_shaped_other_type_cannot_donate_constructor() {
    let mut env = exported_palette();
    env.elaborate_file("module Rival { data Colour = Red | Blue; export Colour, Red, Blue }")
        .expect("same-shaped rival is another kernel family");
    env.elaborate_file("import Palette\nimport Rival")
        .expect("both public providers import");
    let provider_red = env.globals["Palette.Red"];
    let rival_red = env.globals["Rival.Red"];
    assert_ne!(provider_red, rival_red);
    env.elaborate_decl("const first : Palette.Colour = Palette.Colour.Red")
        .expect("own-family control");
    assert_constructor(body(&env, "first"), provider_red);
    let error = env
        .elaborate_decl("const wrong : Palette.Colour = Rival.Colour.Red")
        .expect_err("a rival checked constructor has a different family");
    assert!(
        matches!(
            error,
            ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                ..
            }
        ),
        "wrong cross-family rejection: {error:?}"
    );
}

/// Promise class: durable invariant. MEASURED: the owner can use local T.C
/// while a client importing an abstract T cannot use that hidden constructor.
/// CLAIMED: qualification never increases public export visibility.
/// THE GAP: an owner check and client refusal must refer to the same type ID.
#[test]
fn abstract_export_keeps_qualified_constructor_private() {
    let mut env = env();
    env.elaborate_file(
        "module Vault { pub data Colour = Red | Blue;\
           pub const owner : Colour = Colour.Red }",
    )
    .expect("owner has its private constructor");
    let owner_id = env.globals["Vault.Colour"];
    assert_constructor(body(&env, "Vault.owner"), env.globals["Vault.Red"]);
    env.elaborate_file("import Vault")
        .expect("client imports abstract type");
    assert_eq!(env.globals["Vault.Colour"], owner_id);
    let error = env
        .elaborate_decl("const denied : Vault.Colour = Vault.Colour.Red")
        .expect_err("private constructor cannot be recovered through T.C");
    assert!(
        matches!(error, ElabError::UnboundName { ref name, .. }
        if name == "Vault.Colour.Red"),
        "wrong private refusal: {error:?}"
    );
}

/// Promise class: durable invariant. MEASURED: ResourceKind qualified
/// expressions and all three pattern arms check with canonical prelude IDs,
/// even after the qualified flat spelling is forged to a Bool constructor.
/// CLAIMED: scoped constructor selection is per parent type, preserving
/// match exhaustiveness and trust. THE GAP: the bare and rebind tests inspect
/// absence from the bare namespace independently.
#[test]
fn scoped_resource_kind_qualifies_every_arm_and_checks_coverage() {
    let mut env = env();
    let trust = env.env.trusted_base();
    let kind_id = env.globals["ResourceKind"];
    let buffer = env.prelude_env.runtime_roles.resource_kind_buffer;
    let forged = env.globals["True"];
    assert_ne!(forged, buffer);
    // Forgery control: the checked ResourceKind member ignores the display-map ID.
    env.globals
        .insert("ResourceKind.Buffer".to_string(), forged);
    env.elaborate_file(
        "const selected_kind : ResourceKind = ResourceKind.Buffer\n\
         fn classify (k : ResourceKind) : Nat = match k {\
           ResourceKind.FsHandle |-> Zero;\
           ResourceKind.Buffer |-> Suc Zero;\
           ResourceKind.Mapping |-> Suc (Suc Zero) }",
    )
    .expect("every checked ResourceKind member qualifies");
    assert_eq!(env.env.constructor(buffer).unwrap().0.id, kind_id);
    assert_constructor(body(&env, "selected_kind"), buffer);
    let error = env
        .elaborate_decl(
            "fn incomplete (k : ResourceKind) : Nat = match k {\
               ResourceKind.Buffer |-> Zero }",
        )
        .expect_err("the other two constructors must remain required");
    assert!(
        matches!(error, ElabError::ExhaustivenessError { .. }),
        "wrong missing-identity refusal: {error:?}"
    );
    assert_eq!(env.env.trusted_base(), trust);
}

/// Promise class: durable invariant. MEASURED: all three ResourceKind
/// constructor spellings are absent from the bare global map; a bare Buffer
/// expression and pattern each reject with the exact unresolved spelling;
/// the qualified positive survives a forged flat display spelling.
/// CLAIMED: scoped C never becomes a pattern variable by fallback.
/// THE GAP: another source may legitimately bind C, tested separately.
#[test]
fn scoped_resource_kind_bare_member_neither_resolves_nor_binds() {
    let mut env = env();
    for bare in ["FsHandle", "Buffer", "Mapping"] {
        assert!(
            !env.globals.contains_key(bare),
            "{bare} must not occupy bare B"
        );
    }
    let actual = env.prelude_env.runtime_roles.resource_kind_buffer;
    let forged = env.globals["True"];
    assert_ne!(actual, forged);
    // Forgery control: scoped constructor resolution must ignore this display binding.
    env.globals
        .insert("ResourceKind.Buffer".to_string(), forged);
    env.elaborate_decl("const qualified : ResourceKind = ResourceKind.Buffer")
        .expect("qualified positive control survives a forged display spelling");
    assert_constructor(body(&env, "qualified"), actual);
    let error = env
        .elaborate_decl("const bare : ResourceKind = Buffer")
        .expect_err("bare scoped constructor is unavailable");
    assert!(
        matches!(error, ElabError::UnresolvedCon { ref name, .. } if name == "Buffer"),
        "wrong bare expression refusal: {error:?}"
    );
    let error = env
        .elaborate_decl(
            "fn bad (k : ResourceKind) : Nat = match k { Buffer |-> Zero;\
               ResourceKind.FsHandle |-> Zero; ResourceKind.Mapping |-> Zero }",
        )
        .expect_err("bare capitalized pattern must not become binder");
    assert!(
        matches!(error, ElabError::UnresolvedCon { ref name, .. } if name == "Buffer"),
        "wrong bare pattern refusal: {error:?}"
    );
}

/// Promise class: durable invariant. MEASURED: a user can introduce all
/// three erstwhile ResourceKind bare names as independent source families,
/// without redirecting ResourceKind.Buffer even after its flat map is forged.
/// CLAIMED: the scoped floor reserves only the ResourceKind type spelling.
/// THE GAP: the checked ResourceKind.Buffer arm proves the new user family
/// did not redirect the original scoped constructor identity.
#[test]
fn scoped_member_spellings_are_available_to_user_declarations() {
    let mut env = env();
    let resource_buffer = env
        .env
        .inductive(env.globals["ResourceKind"])
        .expect("floor kind is checked data")
        .constructors[1]
        .id;
    let forged = env.globals["True"];
    assert_ne!(resource_buffer, forged);
    // Forgery control: user-defined leaf names cannot redirect the floor member.
    env.globals
        .insert("ResourceKind.Buffer".to_string(), forged);
    env.elaborate_file(
        "data Buffer = MkMyBuffer\n\
         data Mapping = MkMyMapping\n\
         data FsHandle = MkMyFsHandle\n\
         const selected : ResourceKind = ResourceKind.Buffer",
    )
    .expect("the generic leaf names are not floor reservations");
    assert_ne!(env.globals["Buffer"], resource_buffer);
    assert_constructor(body(&env, "selected"), resource_buffer);
}

/// Promise class: durable invariant. MEASURED: a selective import can bind
/// bare Buffer from a user provider after the prelude's scoped family was
/// registered, and the floor kind survives a forged flat name. CLAIMED:
/// removing the scoped family's bare B name also
/// removes its root-scope local, so imports are not blocked by phantom
/// prelude ownership. THE GAP: the imported Buffer must be a distinct
/// checked provider type, not an alias of ResourceKind.Buffer.
#[test]
fn scoped_constructor_leaf_can_be_selectively_imported_from_user() {
    let mut env = env();
    let resource_ctor = env.prelude_env.runtime_roles.resource_kind_buffer;
    env.elaborate_file("module User { data Buffer = MkUserBuffer; export Buffer, MkUserBuffer }")
        .expect("a provider may use a scoped constructor leaf as its type name");
    let user_type = env.globals["User.Buffer"];
    assert_ne!(resource_ctor, user_type);
    env.elaborate_file("import User (Buffer)")
        .expect("no obsolete ResourceKind local may block the import");
    env.elaborate_decl("fn identity (x : Buffer) : Buffer = x")
        .expect("the selectively imported type is usable");
    let forged = env.globals["True"];
    assert_ne!(forged, resource_ctor);
    // Forgery control: a selective import cannot change the floor family ID.
    env.globals
        .insert("ResourceKind.Buffer".to_string(), forged);
    env.elaborate_decl("const kind : ResourceKind = ResourceKind.Buffer")
        .expect("selective import cannot redirect a separately checked floor kind");
    assert_constructor(body(&env, "kind"), resource_ctor);
}

/// Promise class: durable invariant. MEASURED: both a real module export
/// and a local type constructor spell Colour.Red; resolution refuses the
/// path rather than selecting by order or whether it is a pattern.
/// CLAIMED: the two namespace meanings never silently choose a winner.
/// THE GAP: a second control should use the same ID for both interpretations.
#[test]
fn module_and_type_constructor_path_conflict_in_both_positions() {
    let mut env = env();
    env.elaborate_file(
        "data Colour = Red\n\
         module Colour { pub const Red : Nat = Zero }\n\
         import Colour",
    )
    .expect("module and type both bind Colour.Red");
    let error = env
        .elaborate_decl("const conflict : Colour = Colour.Red")
        .expect_err("module and type meanings conflict in expression");
    assert!(
        matches!(error, ElabError::AmbiguousReference { ref name, .. }
        if name == "Colour.Red"),
        "wrong expression conflict: {error:?}"
    );
    let error = env
        .elaborate_decl("fn conflict_pattern (c : Colour) : Nat = match c { Colour.Red |-> Zero }")
        .expect_err("module and type meanings conflict in pattern");
    assert!(
        matches!(error, ElabError::AmbiguousReference { ref name, .. }
        if name == "Colour.Red"),
        "wrong pattern conflict: {error:?}"
    );
}

/// Promise class: durable invariant. MEASURED: a public facade module and a
/// selectively imported type both authorize Colour.Red to the SAME checked
/// constructor, but the dual interpretation remains ambiguous. CLAIMED:
/// identity equality cannot break a module-versus-type namespace tie.
/// THE GAP: the module-only control must actually select Source.Red first.
#[test]
fn module_and_type_path_conflict_even_when_both_select_one_id() {
    let mut env = env();
    env.elaborate_file(
        "module Source { data Colour = Red; export Colour, Red }\n\
         module Colour { export Source (Red) }\n\
         import Source\n\
         import Colour",
    )
    .expect("provider and facade establish two routes to one constructor");
    let source_red = env.globals["Source.Red"];
    env.elaborate_decl("const module_only : Source.Colour = Colour.Red")
        .expect("the module path alone selects the provider's constructor");
    assert_constructor(body(&env, "module_only"), source_red);
    env.elaborate_file("import Source (Colour)")
        .expect("select the same provider type under its bare name");
    let error = env
        .elaborate_decl("const collision : Colour = Colour.Red")
        .expect_err("two path meanings must conflict despite one target ID");
    assert!(
        matches!(error, ElabError::AmbiguousReference { ref name, .. }
        if name == "Colour.Red"),
        "same-ID path conflict was missed: {error:?}"
    );
}

/// Promise class: durable invariant. MEASURED: an imported module named
/// Colour with no Red export does not compete with a selectively imported
/// type Colour whose public constructor is Red. CLAIMED: ambiguity requires
/// two real visible meanings, not mere prefix/name-map coexistence.
/// THE GAP: both imports must be installed before the constructor use.
#[test]
fn empty_module_member_cannot_manufacture_type_path_ambiguity() {
    let mut env = env();
    env.elaborate_file(
        "module Source { data Colour = Red; export Colour, Red }\n\
         module Colour { pub const Other : Nat = Zero }\n\
         import Colour\n\
         import Source (Colour)",
    )
    .expect("both imports coexist without a Colour.Red module export");
    let expected = env.globals["Source.Red"];
    env.elaborate_decl("const selected : Colour = Colour.Red")
        .expect("only the imported type supplies Colour.Red");
    assert_constructor(body(&env, "selected"), expected);
}

/// Promise class: durable invariant. MEASURED: changing the flat
/// `globals["Colour.Red"]` map to an unrelated checked constructor does not
/// redirect the locally checked qualified expression. CLAIMED: the local
/// type's recorded constructor ID, not a mutable display spelling, is the
/// resolver's authority. THE GAP: the forged ID must be genuinely distinct.
#[test]
fn forged_qualified_global_spelling_cannot_redirect_checked_member() {
    let mut env = env();
    env.elaborate_decl("data Colour = Red | Blue")
        .expect("declare a real local family");
    let checked_red = env.globals["Red"];
    let forged = env.globals["True"];
    assert_ne!(checked_red, forged);
    // Forgery control: this dotted display key must not grant constructor authority.
    assert_eq!(env.globals.insert("Colour.Red".to_string(), forged), None);
    env.elaborate_decl("const selected : Colour = Colour.Red")
        .expect("the checked family member survives a forged spelling");
    assert_constructor(body(&env, "selected"), checked_red);
}
