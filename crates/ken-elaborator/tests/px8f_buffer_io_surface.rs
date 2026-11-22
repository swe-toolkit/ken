//! PX8-F checked positioned-buffer surface and real structural proof terms.

#[path = "support/catalog_roots.rs"]
mod catalog_roots;

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{convert_type, whnf, Context, Decl, GlobalId, KernelError, Term};

/// Load the landed Buffer + IO surface as its clients load it: through the
/// strict roots loader. IO imports Buffer, so flat fences cannot stand for the
/// pair. Catalog declarations enter `globals` under their qualified paths;
/// checked prelude names stay bare, so look catalog laws up with `io`.
fn load_landed_surface(env: &mut ElabEnv) {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages");
    for module in ["Capability.System.IO", "Capability.System.Buffer"] {
        env.elaborate_module_from_roots(&[root.clone()], module)
            .unwrap_or_else(|error| panic!("{module} must roots-load: {error:?}"));
    }
}

/// The checked identity a `Capability.System.IO` declaration owns.
fn io(env: &ElabEnv, name: &str) -> GlobalId {
    env.globals[&format!("Capability.System.IO.{name}")]
}

fn result_head(env: &ken_kernel::GlobalEnv, ty: &Term) -> Term {
    let mut context = Context::new();
    let mut head = whnf(env, &context, ty);
    loop {
        match head {
            Term::Pi(domain, codomain) => {
                context.push(*domain);
                head = whnf(env, &context, &codomain);
            }
            result => return result,
        }
    }
}

fn public_buffer_span_producers(env: &ElabEnv) -> BTreeSet<String> {
    let buffer_span = env.globals["BufferSpan"];
    env.globals
        .iter()
        .filter_map(|(name, id)| {
            let ty = match env.env.lookup(*id) {
                Some(decl) => match decl {
                    Decl::Transparent { ty, .. }
                    | Decl::Opaque { ty, .. }
                    | Decl::Primitive { ty, .. } => ty,
                    Decl::Inductive(inductive) => &inductive.former_type,
                },
                None => match env.env.constructor(*id) {
                    Some((inductive, index)) => &inductive.constructors[index].type_,
                    None => {
                        panic!("public global `{name}` is neither a declaration nor a constructor")
                    }
                },
            };
            matches!(
                result_head(&env.env, ty),
                Term::IndFormer { id, .. } if id == buffer_span
            )
            .then(|| name.clone())
        })
        .collect()
}

#[test]
fn checked_surface_is_public_but_proof_carrying_constructors_stay_private() {
    let mut env = ElabEnv::empty().expect("PX8-F prelude");
    let base_trusted = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    load_landed_surface(&mut env);
    assert_eq!(
        env.env.trusted_base().into_iter().collect::<BTreeSet<_>>(),
        base_trusted,
        "checked Buffer and IO declarations must not grow the trusted base"
    );

    // HR-i: public signatures and sequencing can be written with HostIO and
    // checked package operations, without spelling the hidden FSOp tree.
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages");
    env.elaborate_isolated_file_strict_v1(
        &[root],
        r#"import Capability.System.Buffer (BufferHandle, BufferWindow, freeze, transfer_count_int)
import Capability.System.IO (readAt, writeAt, writeAll)
import Capability.Process.Exit (host_pure)

proc px8f_exact_read
  (a : Auth) (file : Resource ResourceKind.FsHandle) (offset : Int)
  (buffer : BufferHandle) (window : BufferWindow)
  : HostIO a (Result ResourceError ReadProgress) visits [FS] =
  readAt a file offset buffer window

proc px8f_exact_write
  (a : Auth) (file : Resource ResourceKind.FsHandle) (offset : Int)
  (buffer : BufferHandle) (span : BufferSpan)
  : HostIO a (Result ResourceError WriteProgress) visits [FS] =
  writeAt a file offset buffer span

proc px8f_exact_freeze
  (a : Auth) (buffer : BufferHandle) (span : BufferSpan)
  : HostIO a (Result ResourceError Bytes) visits [FS] =
  freeze a buffer span

proc px8f_exact_write_all
  (a : Auth) (file : Resource ResourceKind.FsHandle) (offset : Int)
  (buffer : BufferHandle) (span : BufferSpan)
  : HostIO a (Result ResourceError Unit) visits [FS] =
  writeAll a file offset buffer span

proc px8f_readsome_public_consumers
  (a : Auth) (file : Resource ResourceKind.FsHandle) (offset : Int)
  (buffer : BufferHandle) (window : BufferWindow)
  : HostIO a (Result ResourceError Unit) visits [FS] =
  let read_outcome = readAt a file offset buffer window in
  match read_outcome {
    Err error ↦ host_pure a (Result ResourceError Unit) (Err ResourceError Unit error);
    Ok progress ↦ match progress {
      ReadProgress.ReadEof ↦
        host_pure a (Result ResourceError Unit) (Ok ResourceError Unit MkUnit);
      ReadProgress.ReadSome span count ↦
        let written = writeAt a file (add_int offset (transfer_count_int count)) buffer span in
        let frozen = freeze a buffer span in
        writeAll a file offset buffer span
    }
  }
"#,
    )
    .expect("exact PX8-F public signatures elaborate");
    // The IO theorem concludes about IO's own checked subject. Both are
    // package-private, so the check reads their checked identities; a bare
    // client spelling would now name the distinct prelude copy.
    let subject = io(&env, "write_all_all_success");
    assert_ne!(
        subject, env.globals["write_all_all_success"],
        "the catalog subject is not the bare prelude copy"
    );
    let Some(Decl::Transparent { ty, .. }) =
        env.env.lookup(io(&env, "write_all_all_success_holds"))
    else {
        panic!("write_all_all_success_holds must have a checked body");
    };
    let mut conclusion = ty;
    while let Term::Pi(_, body) = conclusion {
        conclusion = body;
    }
    let Term::App(equal_and_left, _) = conclusion else {
        panic!("write_all_all_success_holds must state an Equal application");
    };
    let Term::App(_, left) = equal_and_left.as_ref() else {
        panic!("write_all_all_success_holds must carry a left endpoint");
    };
    let mut head = left.as_ref();
    while let Term::App(function, _) = head {
        head = function;
    }
    assert!(
        matches!(head, Term::Const { id, .. } if *id == subject),
        "write_all_all_success_holds must conclude about IO's checked subject"
    );

    for public in [
        "BufferHandle",
        "BufferWindow",
        "BufferSpan",
        "TransferCount",
        "ReadProgress",
        "WriteProgress",
        "readAt",
        "writeAt",
        "spanBytes",
        "freeze",
        "writeAll",
        "transfer_count_predecessor",
        "transfer_count_nat_succ",
        "write_all_entry",
        "write_all_stop",
        "write_all_request",
        "write_all_first_error_step",
        "write_all_done",
        "write_all_continue",
        "write_all_advance_start_prop",
        "write_all_advance_start",
        "write_all_exact_prefix_prop",
        "write_all_exact_prefix_prop::exact_prefix",
    ] {
        assert!(env.globals.contains_key(public), "missing `{public}`");
    }
    for private in [
        "PrivateBufferHandle",
        "buffer_handle_resource",
        "buffer_handle_capacity",
        "PrivateBufferSpan",
        "write_all_advance_span",
        "PrivateTransferCount",
        "PrivateFsReadAt",
        "PrivateFsWriteAt",
        "PrivateBufferFreeze",
        "private_read_at_positive",
        "private_write_all_fuel",
        "private_write_all_step",
        "private_write_all_after_wrote",
        "private_write_all_next",
        "write_all_refl",
    ] {
        assert!(!env.globals.contains_key(private), "`{private}` escaped");
    }
    for law in [
        "write_all_exact_prefix_prop",
        "write_all_exact_prefix_prop::exact_prefix",
        "transfer_count_nat_succ",
        "write_all_entry",
        "write_all_stop",
        "write_all_request",
        "write_all_first_error_step",
        "write_all_done",
        "write_all_continue",
        "write_all_advance_start",
    ] {
        assert!(
            env.env.transparent_body(env.globals[law]).is_some(),
            "law `{law}` must be a real checked body"
        );
    }
    for law in [
        "write_all_terminates",
        "write_all_preserves_exact_prefix",
        "write_all_success_is_complete",
        "write_all_preserves_first_error",
        "write_all_all_success_holds",
        "write_all_strict_decrease",
        "write_all_fuel_sufficient",
        "write_all_complete_after_wrote",
        "write_all_zero_budget",
    ] {
        assert!(
            env.env.transparent_body(io(&env, law)).is_some(),
            "IO law `{law}` must be a real checked body"
        );
    }
    assert_ne!(
        io(&env, "write_all_all_success_holds"),
        io(&env, "write_all_all_success"),
        "the theorem and its native-prelude subject must remain distinct globals"
    );
}

/// Promise class: durable invariant (`38 §1.7.3`). MEASURED: the actual
/// checked step referenced by the public loop law has the independently
/// written first-error proposition. A Strict identity theorem abstracts the
/// representation and private step; kernel substitution binds the checked
/// identities without source access. CLAIMED: a writeAt error cannot turn
/// into a different ResourceError. THE GAP: the separately checked
/// `write_all_request` must bind its continuation to this step; this test
/// does not drive file-side effects. Write-partition conformance does.
#[test]
fn write_all_checked_step_preserves_the_exact_first_error() {
    let mut env = ElabEnv::empty().expect("checked prelude");
    let trusted_before_clients = env.env.trusted_base();
    let law = env.globals["write_all_first_error_step"];
    let Some(Decl::Transparent { ty, .. }) = env.env.lookup(law) else {
        panic!("first-error law must have a checked body");
    };
    let mut conclusion = ty;
    while let Term::Pi(_, body) = conclusion {
        conclusion = body;
    }
    let Term::App(equal_and_left, _) = conclusion else {
        panic!("first-error law must state an Equal application");
    };
    let Term::App(equal_and_type, left) = equal_and_left.as_ref() else {
        panic!("first-error law must carry a left endpoint");
    };
    let Term::App(equal_head, _) = equal_and_type.as_ref() else {
        panic!("first-error law must carry a type parameter");
    };
    assert!(matches!(
        equal_head.as_ref(),
        Term::Const { id, .. } if *id == env.globals["Equal"]
    ));
    let mut head = left.as_ref();
    while let Term::App(function, _) = head {
        head = function;
    }
    let Term::Const {
        id: private_step, ..
    } = head
    else {
        panic!("first-error law must name the real checked step");
    };
    assert!(env.env.transparent_body(*private_step).is_some());
    assert!(!env.globals.values().any(|id| id == private_step));
    assert!(!env.globals.contains_key("private_write_all_step"));
    let private_step = *private_step;
    let law_type = ty.clone();
    // HR-ii representation: FSOp, AmbientOp, fs_resp, ambient_resp and the
    // hidden checked step are parameters of the Strict source theorem. None
    // becomes a source-accessible prelude or package name.
    let fs_op = Term::indformer(env.prelude_env.runtime_roles.fs_family, vec![]);
    let ambient_id = env.globals["AmbientOp"];
    let ambient = if env.env.inductive(ambient_id).is_some() {
        Term::indformer(ambient_id, vec![])
    } else {
        Term::const_(ambient_id, vec![])
    };
    let no_progress = Term::constructor(env.globals["ResourceError.NoProgress"], vec![]);
    let hidden = [
        fs_op,
        ambient,
        Term::const_(env.globals["fs_resp"], vec![]),
        Term::const_(env.globals["ambient_resp"], vec![]),
        Term::const_(private_step, vec![]),
    ];
    let header = r#"theorem first_error_is_no_progress
  (FSOp : Auth -> Type) (AmbientOp : Type)
  (fs_resp : (a : Auth) -> FSOp a -> Type)
  (ambient_resp : AmbientOp -> Type)
  (private_write_all_step : (a : Auth) -> Int -> BufferSpan ->
    (Int -> BufferSpan -> HostIO a (Result ResourceError Unit)) ->
    Result ResourceError WriteProgress -> HostIO a (Result ResourceError Unit))
  (a : Auth) (file : Resource ResourceKind.FsHandle)
  (offset : Int) (buffer : BufferHandle) (span : BufferSpan)
  (next : Int -> BufferSpan -> HostIO a (Result ResourceError Unit))"#;
    let proposition = r#"Equal (HostIO a (Result ResourceError Unit))
      (private_write_all_step a offset span next
        (Err ResourceError WriteProgress ResourceError.NoProgress))
      (ITree.Ret (Coproduct (FSOp a) AmbientOp)
        (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
        (Result ResourceError Unit) (Err ResourceError Unit ResourceError.NoProgress))"#;
    // HostIO's concrete body does not convert while FSOp is abstract.
    // State its unfolded tree in this private-by-design proposition; after
    // substitution it converts back to the checked HostIO law type.
    let tree = "ITree (Coproduct (FSOp a) AmbientOp) \
        (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp) \
        (Result ResourceError Unit)";
    let header = header.replace("HostIO a (Result ResourceError Unit)", tree);
    let proposition = proposition.replace("HostIO a (Result ResourceError Unit)", tree);
    let witness = format!("{header}\n  (h : {proposition}) : {proposition} = h");
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages");
    let checked = env
        .elaborate_isolated_file_strict_v1(
            &[root.clone()],
            &format!("import Capability.System.Buffer (BufferHandle)\n{witness}"),
        )
        .expect("Strict identity theorem over private representation checks");
    let checked_id = checked.last().expect("identity theorem declaration").def_id;
    let Some(Decl::Transparent {
        ty: checked_type, ..
    }) = env.env.lookup(checked_id)
    else {
        panic!("independent client theorem must have a checked body")
    };
    let mut statement = checked_type.clone();
    for identity in &hidden {
        let Term::Pi(_, tail) = statement else {
            panic!("hidden binder missing")
        };
        statement = ken_kernel::subst::subst0(&tail, identity);
    }
    fn without_witness(ty: Term) -> Term {
        match ty {
            Term::Pi(domain, tail) => match *tail {
                next @ Term::Pi(..) => Term::Pi(domain, Box::new(without_witness(next))),
                _ => *domain,
            },
            other => panic!("missing final witness binder: {other:?}"),
        }
    }
    fn at_no_progress(ty: Term, error: &Term) -> Term {
        match ty {
            Term::Pi(domain, tail) => match *tail {
                next @ Term::Pi(..) => Term::Pi(domain, Box::new(at_no_progress(next, error))),
                conclusion => ken_kernel::subst::subst0(&conclusion, error),
            },
            other => panic!("checked law lost its error binder: {other:?}"),
        }
    }
    let expected_type = at_no_progress(law_type, &no_progress);
    assert!(convert_type(&env.env, &Context::new(), &without_witness(statement), &expected_type),
        "the independently written NoProgress tree proposition must convert to the checked HostIO law");

    // Wrong-error conclusion with the *same* witness premise must fail at
    // kernel equality; a generic parse or missing-name refusal is not enough.
    let wrong_proposition = proposition.replace(
        "(Err ResourceError Unit ResourceError.NoProgress))",
        "(Err ResourceError Unit ResourceError.Closed))",
    );
    assert_ne!(
        wrong_proposition, proposition,
        "wrong-error conclusion differs"
    );
    let wrong = format!(
        "{}\n  (h : {proposition}) : {wrong_proposition} = h",
        header.replace("first_error_is_no_progress", "first_error_is_closed")
    );
    let rejection = env
        .elaborate_isolated_file_strict_v1(
            &[root],
            &format!("import Capability.System.Buffer (BufferHandle)\n{wrong}"),
        )
        .expect_err("a distinct error cannot be derived from the first-error witness");
    assert!(
        matches!(
            rejection,
            ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                ..
            }
        ),
        "wrong error must reach the kernel equality, got {rejection:?}"
    );
    assert_eq!(
        env.env.trusted_base(),
        trusted_before_clients,
        "first-error clients must not add trust"
    );
}

/// Promise class: normative compatibility vector (`38 §1.9`). The public
/// Mapping reads take immutable windows directly, while writes take an offset
/// and derive their extent from the byte payload. The handle constructor,
/// resource projections, and frozen host operations remain absent from checked
/// source. `mapView`/`MappingSpan` are intentionally not surface names: mappings
/// reject rather than cap, so no minted subrange is needed.
#[test]
fn abi_s6_mapping_surface_is_direct_and_the_handle_stays_opaque() {
    let mut env = ElabEnv::empty().expect("ABI-S6 prelude");
    catalog_roots::load(&mut env, "Capability.System.Mapping");
    env.elaborate_file(
        r#"import Capability.System.Mapping (MappingHandle, MappingWindow, mapWrite, mapBytes)
proc mapping_surface_write
  (a : Auth) (mapping : MappingHandle) (offset : Int) (bytes : Bytes)
  : HostIO a (Result ResourceError Unit) visits [FS] =
  mapWrite a mapping offset bytes

proc mapping_surface_read
  (a : Auth) (mapping : MappingHandle) (window : MappingWindow)
  : HostIO a (Result ResourceError Bytes) visits [FS] =
  mapBytes a mapping window
"#,
    )
    .expect("the checked direct Mapping surface elaborates");

    for public in [
        "MappingSource",
        "Anonymous",
        "FileBacked",
        "MappingProt",
        "MappingProt.ReadOnly",
        "MappingProt.ReadWrite",
        "MappingExtent",
        "MappingWindow",
        "MkMappingWindow",
        "MappingHandle",
        "withMapping",
        "mapBytes",
        "mapWrite",
    ] {
        assert!(env.globals.contains_key(public), "missing `{public}`");
    }
    for unavailable in [
        "mapView",
        "MappingSpan",
        "PrivateMappingHandle",
        "mapping_handle_resource",
        "mapping_handle_length",
        "PrivateMappingAllocate",
        "PrivateMappingReadView",
        "PrivateMappingWriteView",
        "PrivateMappingAcquireFile",
    ] {
        assert!(
            !env.globals.contains_key(unavailable),
            "`{unavailable}` escaped into the checked surface"
        );
    }
}

/// Promise class: durable opacity invariant (`38 §1.9`). MEASURED: checked
/// source cannot resolve the sole MappingHandle constructor or either internal
/// field projection. CLAIMED: users cannot forge a handle/resource/extent pair
/// or extract its governed resource. THE GAP: native host code remains trusted
/// to preserve token integrity after the checked boundary.
#[test]
fn checked_source_cannot_forge_or_project_mapping_handles() {
    for (private, source) in [
        ("PrivateMappingHandle", "let escaped = PrivateMappingHandle"),
        (
            "mapping_handle_resource",
            "fn escaped (mapping : MappingHandle) : Resource ResourceKind.Mapping = \
             mapping_handle_resource mapping",
        ),
        (
            "mapping_handle_length",
            "fn escaped (mapping : MappingHandle) : Int = \
             mapping_handle_length mapping",
        ),
    ] {
        let mut env = ElabEnv::empty().expect("ABI-S6 prelude");
        catalog_roots::load(&mut env, "Capability.System.Mapping");
        env.elaborate_file("import Capability.System.Mapping (MappingHandle)")
            .expect("select public mapping handle before testing private names");
        match env.elaborate_file(source) {
            Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, private),
            other => panic!("`{private}` must reject as an unbound private name, got {other:?}"),
        }
    }
}

#[test]
fn buffer_span_producer_closure_is_derived_from_public_globals() {
    let mut env = ElabEnv::empty().expect("SPAN-SEAL prelude");
    load_landed_surface(&mut env);
    assert_eq!(public_buffer_span_producers(&env), BTreeSet::new());
}

#[test]
fn buffer_span_producer_closure_reduces_transparent_type_aliases() {
    let mut env = ElabEnv::empty().expect("SPAN-SEAL prelude");
    env.elaborate_file(
        r#"
def BufferSpanAlias = BufferSpan

fn escaped_result_alias (span : BufferSpan) : BufferSpanAlias = span

def BufferSpanFunctionAlias = BufferSpan → BufferSpan

const escaped_function_alias : BufferSpanFunctionAlias = λspan.span
"#,
    )
    .expect("transparent result and whole-function aliases elaborate");

    assert_eq!(
        public_buffer_span_producers(&env),
        BTreeSet::from([
            "escaped_function_alias".to_string(),
            "escaped_result_alias".to_string(),
        ])
    );
}

#[test]
fn buffer_span_producer_closure_resolves_public_constructors() {
    let mut env = ElabEnv::empty().expect("SPAN-SEAL prelude");
    let private_buffer_span = {
        let buffer_span = env
            .env
            .inductive(env.globals["BufferSpan"])
            .expect("BufferSpan inductive");
        assert_eq!(buffer_span.constructors.len(), 1);
        buffer_span.constructors[0].id
    };
    assert_eq!(
        private_buffer_span, env.prelude_env.private_buffer_span_id,
        "discriminator must re-expose the actual sealed constructor"
    );
    assert!(env.env.lookup(private_buffer_span).is_none());
    assert!(env.env.constructor(private_buffer_span).is_some());
    assert!(!env.globals.values().any(|id| *id == private_buffer_span));
    env.globals.insert(
        "escaped_private_buffer_span_constructor".to_string(),
        private_buffer_span,
    );

    assert_eq!(
        public_buffer_span_producers(&env),
        BTreeSet::from(["escaped_private_buffer_span_constructor".to_string()])
    );
}

#[test]
#[should_panic(
    expected = "public global `escaped_unknown_id` is neither a declaration nor a constructor"
)]
fn buffer_span_producer_closure_rejects_unknown_public_ids() {
    let mut env = ElabEnv::empty().expect("SPAN-SEAL prelude");
    env.globals
        .insert("escaped_unknown_id".to_string(), GlobalId(u32::MAX));
    let _ = public_buffer_span_producers(&env);
}

#[test]
fn checked_source_rejects_private_buffer_span_producers() {
    for (private, source) in [
        (
            "PrivateBufferSpan",
            "fn escaped (start : Int) (budget : Nat) : BufferSpan = \
             PrivateBufferSpan start budget",
        ),
        (
            "write_all_advance_span",
            "fn escaped (span : BufferSpan) (count : TransferCount) : BufferSpan = \
             write_all_advance_span span count",
        ),
    ] {
        let mut env = ElabEnv::empty().expect("SPAN-SEAL prelude");
        match (private, env.elaborate_file(source)) {
            (
                "PrivateBufferSpan",
                Err(ElabError::UnresolvedCon { name, .. } | ElabError::UnboundName { name, .. }),
            ) => assert_eq!(name, private),
            (
                "write_all_advance_span",
                Err(ElabError::UnresolvedCon { name, .. } | ElabError::UnboundName { name, .. }),
            ) => assert_eq!(name, private),
            other => panic!("`{private}` must reject as an unresolved private name, got {other:?}"),
        }
    }
}

/// Durable invariant (`38 §1.7.1`): the public handle is an ordinary product
/// internally, but checked source can neither forge it nor project either
/// acquisition-bound field.
#[test]
fn checked_source_cannot_forge_or_project_buffer_handles() {
    for (private, source) in [
        (
            "PrivateBufferHandle",
            "fn escaped (resource : Resource ResourceKind.Buffer) : BufferHandle = \
             PrivateBufferHandle resource 8",
        ),
        (
            "buffer_handle_resource",
            "fn escaped (buffer : BufferHandle) : Resource ResourceKind.Buffer = \
             buffer_handle_resource buffer",
        ),
        (
            "buffer_handle_capacity",
            "fn escaped (buffer : BufferHandle) : Int = \
             buffer_handle_capacity buffer",
        ),
    ] {
        let mut env = ElabEnv::empty().expect("PX8-F-CAP-41 prelude");
        catalog_roots::load(&mut env, "Capability.System.Buffer");
        env.elaborate_file("import Capability.System.Buffer (BufferHandle)")
            .expect("select the checked public handle former");
        match env.elaborate_file(source) {
            Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, private),
            other => panic!("`{private}` must reject as an unbound private name, got {other:?}"),
        }
    }
}

#[test]
fn positioned_operations_keep_the_landed_wire_identities() {
    assert_eq!(ken_host::HostOpV1::FsReadAt as u16, 0x030D);
    assert_eq!(ken_host::HostOpV1::FsWriteAt as u16, 0x030E);
    assert_eq!(ken_host::HostOpV1::BufferFreeze as u16, 0x0403);
}
