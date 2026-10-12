//! Checked admission for the single supported Ken program entrypoint ABI.

use ken_kernel::{Context, Decl, GlobalId, Term};

use crate::{capabilities, effects::EffectRow, ElabEnv};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckedMainDescriptor {
    pub main: GlobalId,
    pub process_input: GlobalId,
    pub process_input_constructor: GlobalId,
    pub program_caps: GlobalId,
    pub program_caps_constructor: GlobalId,
    pub cap: GlobalId,
    pub authority_constructor: GlobalId,
    pub host_io: GlobalId,
    pub host_exit: GlobalId,
    pub exit_code: GlobalId,
    pub success_constructor: GlobalId,
    pub failure_constructor: GlobalId,
    pub ret_constructor: GlobalId,
    pub list_nil_constructor: GlobalId,
    pub list_cons_constructor: GlobalId,
    pub prod_constructor: GlobalId,
    pub authority_name: String,
    pub authority: capabilities::Authority,
    pub fs_root_spec: ken_host::FsRootSpec,
    pub allow_root_execution: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProgramAdmissionError {
    MissingMain,
    MissingProgramBoundary,
    MissingCapability { effect: &'static str },
    DuplicateCapability { effect: &'static str },
    UnsupportedEffectRow,
    InvalidMainAbi { authority: String },
}

/// Resolve the checked entry ABI from a pre-source capture. The mutable source
/// globals table is not available through this signature; only the program's
/// own `main` is selected by its spelling in the caller.
fn checked_prelude_entry_abi(
    prelude: &crate::prelude::PreludeEnv,
    main: GlobalId,
    authority_name: &str,
    authority: capabilities::Authority,
    fs_root_spec: ken_host::FsRootSpec,
    allow_root_execution: bool,
) -> CheckedMainDescriptor {
    let roles = &prelude.runtime_roles;
    let authority_constructor = match authority_name {
        "ANone" => roles.authority_none,
        "APartial" => roles.authority_partial,
        "AFull" => roles.authority_full,
        _ => unreachable!("parser admitted invalid FS authority"),
    };
    CheckedMainDescriptor {
        main,
        process_input: roles.process_input_type,
        process_input_constructor: roles.process_input,
        program_caps: roles.program_caps_type,
        program_caps_constructor: roles.program_caps_constructor,
        cap: roles.capability,
        authority_constructor,
        host_io: roles.host_io_type,
        host_exit: roles.host_exit,
        exit_code: roles.exit_code_type,
        success_constructor: roles.exit_success,
        failure_constructor: roles.exit_failure,
        ret_constructor: roles.ret,
        list_nil_constructor: roles.list_nil,
        list_cons_constructor: roles.list_cons,
        prod_constructor: roles.prod,
        authority_name: authority_name.to_string(),
        authority,
        fs_root_spec,
        allow_root_execution,
    }
}

/// Admit the one process-shaped Program I entrypoint against the live
/// elaboration environment that produced it.
pub fn admit_checked_main(env: &ElabEnv) -> Result<CheckedMainDescriptor, ProgramAdmissionError> {
    let main = env
        .globals
        .get("main")
        .copied()
        .ok_or(ProgramAdmissionError::MissingMain)?;

    let header = env
        .boundary_header()
        .filter(|header| header.kind == crate::BoundaryKind::Program)
        .ok_or(ProgramAdmissionError::MissingProgramBoundary)?;
    let capabilities = header
        .capabilities
        .as_ref()
        .ok_or(ProgramAdmissionError::MissingCapability { effect: "FS" })?;
    let fs = capabilities
        .iter()
        .filter(|capability| capability.family == "FS")
        .collect::<Vec<_>>();
    let declaration = match fs.as_slice() {
        [] => return Err(ProgramAdmissionError::MissingCapability { effect: "FS" }),
        [declaration] => *declaration,
        _ => return Err(ProgramAdmissionError::DuplicateCapability { effect: "FS" }),
    };
    let (authority_name, authority) = match declaration.authority.as_str() {
        "ANone" => ("ANone", capabilities::AUTH_NONE),
        "APartial" => ("APartial", capabilities::AUTH_PARTIAL),
        "AFull" => ("AFull", capabilities::AUTH_FULL),
        other => unreachable!("parser admitted invalid FS authority {other}"),
    };
    let fs_root_spec = declaration
        .root
        .as_deref()
        .and_then(ken_host::FsRootSpec::parse_declared)
        .unwrap_or_default();

    if let Some(row) = env.effect_rows.get("main") {
        let granted = EffectRow::from_effects([
            "Console".to_string(),
            "Clock".to_string(),
            "Entropy".to_string(),
            "FS".to_string(),
        ]);
        if !row.row_vars().is_empty() || !row.concrete_effects().is_subset_of(&granted) {
            return Err(ProgramAdmissionError::UnsupportedEffectRow);
        }
    }

    let checked = checked_prelude_entry_abi(
        &env.prelude_env,
        main,
        authority_name,
        authority,
        fs_root_spec,
        header.allow_root_execution,
    );

    let actual = match env.env.lookup(main) {
        Some(Decl::Transparent { ty, .. })
        | Some(Decl::Opaque { ty, .. })
        | Some(Decl::Primitive { ty, .. }) => ty,
        _ => {
            return Err(ProgramAdmissionError::InvalidMainAbi {
                authority: authority_name.to_string(),
            });
        }
    };
    let authority_term = Term::constructor(checked.authority_constructor, vec![]);
    let expected = Term::pi(
        Term::indformer(checked.process_input, vec![]),
        Term::pi(
            Term::app(
                Term::indformer(checked.program_caps, vec![]),
                authority_term.clone(),
            ),
            Term::app(
                Term::app(Term::const_(checked.host_io, vec![]), authority_term),
                Term::indformer(checked.exit_code, vec![]),
            ),
        ),
    );
    if !ken_kernel::convert_type(&env.env, &Context::new(), actual, &expected) {
        return Err(ProgramAdmissionError::InvalidMainAbi {
            authority: authority_name.to_string(),
        });
    }

    Ok(checked)
}
