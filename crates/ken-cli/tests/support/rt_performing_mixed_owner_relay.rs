use ken_kernel::{Decl, GlobalEnv, GlobalId, Term};

fn head(env: &GlobalEnv, id: GlobalId) -> Term {
    let decl = env
        .lookup(id)
        .unwrap_or_else(|| panic!("relay head {id:?} is not declared"));
    assert!(
        decl.level_params().is_empty(),
        "relay head {id:?} is level-polymorphic; the construction passes no level arguments"
    );
    assert!(
        env.constructor(id).is_none(),
        "relay head {id:?} is a constructor; build it with Term::constructor"
    );
    if env.inductive(id).is_some() {
        Term::indformer(id, vec![])
    } else {
        Term::const_(id, vec![])
    }
}

// Strict source uses explicit binders for pre-source private identities. The
// checked relay is built below; this source is not itself the program executed.
pub(super) const RELAY_TEMPLATE_SOURCE: &str = r#"program capabilities FS AFull
import Capability.System.Buffer (BufferHandle)
import Capability.System.Resource (ResourceBodyResult, ResourceBodyOk, ResourceBodyErr, ResourceBracketResult, withBuffer, withResource)
fn template_rt_branched_body
  (local_fs_op : Auth → Type)
  (local_ambient_op : Type)
  (local_fs_resp : (a : Auth) → local_fs_op a → Type)
  (local_ambient_resp : local_ambient_op → Type)
  (_buffer : BufferHandle)
  : ITree (Coproduct (local_fs_op AFull) local_ambient_op)
    (resp_coproduct (local_fs_op AFull) local_ambient_op
      (local_fs_resp AFull) local_ambient_resp)
    (ResourceBodyResult Unit Unit) =
  ITree.Ret (Coproduct (local_fs_op AFull) local_ambient_op)
    (resp_coproduct (local_fs_op AFull) local_ambient_op
      (local_fs_resp AFull) local_ambient_resp)
    (ResourceBodyResult Unit Unit) (ResourceBodyOk Unit Unit MkUnit)
fn template_rt_computed_read
  (local_fs_op : Auth → Type)
  (local_ambient_op : Type)
  (local_read_file : (a : Auth) → Cap a → Bytes → local_fs_op a)
  (cap : Cap AFull) : Coproduct (local_fs_op AFull) local_ambient_op =
  Coproduct.InL (local_fs_op AFull) local_ambient_op
    (local_read_file AFull cap (bytes_encode "source"))
fn template_rt_branched_endpoint_buffer
  (local_fs_op : Auth → Type)
  (local_ambient_op : Type)
  (local_fs_resp : (a : Auth) → local_fs_op a → Type)
  (local_ambient_resp : local_ambient_op → Type)
  (local_read_file : (a : Auth) → Cap a → Bytes → local_fs_op a)
  (local_computed_read : Cap AFull → Coproduct (local_fs_op AFull) local_ambient_op)
  (cap : Cap AFull) (file : Resource ResourceKind.FsHandle) (buffer : BufferHandle)
  : ITree (Coproduct (local_fs_op AFull) local_ambient_op)
    (resp_coproduct (local_fs_op AFull) local_ambient_op
      (local_fs_resp AFull) local_ambient_resp)
    (ResourceBodyResult Unit Unit) =
  itree_bind (Coproduct (local_fs_op AFull) local_ambient_op)
    (resp_coproduct (local_fs_op AFull) local_ambient_op
      (local_fs_resp AFull) local_ambient_resp)
    (ResourceBodyResult Unit Unit) (ResourceBodyResult Unit Unit)
    (ITree.Vis (Coproduct (local_fs_op AFull) local_ambient_op)
      (resp_coproduct (local_fs_op AFull) local_ambient_op
        (local_fs_resp AFull) local_ambient_resp)
      (ResourceBodyResult Unit Unit)
      (Coproduct.InL (local_fs_op AFull) local_ambient_op
        (local_read_file AFull cap (bytes_encode "source")))
      (\first. ITree.Vis (Coproduct (local_fs_op AFull) local_ambient_op)
        (resp_coproduct (local_fs_op AFull) local_ambient_op
          (local_fs_resp AFull) local_ambient_resp)
        (ResourceBodyResult Unit Unit) (local_computed_read cap)
        (\second. ITree.Ret (Coproduct (local_fs_op AFull) local_ambient_op)
          (resp_coproduct (local_fs_op AFull) local_ambient_op
            (local_fs_resp AFull) local_ambient_resp)
          (ResourceBodyResult Unit Unit) (ResourceBodyOk Unit Unit MkUnit))))
    (\outcome. ITree.Ret (Coproduct (local_fs_op AFull) local_ambient_op)
      (resp_coproduct (local_fs_op AFull) local_ambient_op
        (local_fs_resp AFull) local_ambient_resp)
      (ResourceBodyResult Unit Unit) outcome)
fn template_after_buffer_arm
  (local_fs_op : Auth → Type)
  (local_ambient_op : Type)
  (local_fs_resp : (a : Auth) → local_fs_op a → Type)
  (local_ambient_resp : local_ambient_op → Type)
  : ITree (Coproduct (local_fs_op AFull) local_ambient_op)
    (resp_coproduct (local_fs_op AFull) local_ambient_op
      (local_fs_resp AFull) local_ambient_resp)
    (ResourceBodyResult Unit Unit) =
  ITree.Ret (Coproduct (local_fs_op AFull) local_ambient_op)
    (resp_coproduct (local_fs_op AFull) local_ambient_op
      (local_fs_resp AFull) local_ambient_resp)
    (ResourceBodyResult Unit Unit) (ResourceBodyErr Unit Unit MkUnit)
proc template_rt_branched_after_buffer
  (err_arm : HostIO AFull (ResourceBodyResult Unit Unit))
  (ok_arm : HostIO AFull (ResourceBodyResult Unit Unit))
  (outcome : Result ResourceError (ResourceBracketResult Unit Unit))
  : HostIO AFull (ResourceBodyResult Unit Unit) visits [FS] =
  match outcome {
    Err error |-> err_arm;
    Ok bracket |-> ok_arm
  }
const template_eight : Int = (8 : Int)
fn template_done_err_arm
  (local_host_exit : (a : Auth) → ExitCode → HostIO a ExitCode)
  : HostIO AFull ExitCode =
  local_host_exit AFull (ExitCode.Failure 71)
fn template_done_ok_arm
  (local_host_exit : (a : Auth) → ExitCode → HostIO a ExitCode)
  : HostIO AFull ExitCode =
  local_host_exit AFull (ExitCode.Failure 72)
proc template_rt_branched_done
  (err_arm : HostIO AFull ExitCode)
  (ok_arm : HostIO AFull ExitCode)
  (outcome : Result FileError (ResourceBracketResult Unit Unit))
  : HostIO AFull ExitCode visits [FS] =
  match outcome {
    Err error |-> err_arm;
    Ok bracket |-> ok_arm
  }
const template_source : Bytes = bytes_encode "source"
proc template_main
  (local_stage : Cap AFull → HostIO AFull ExitCode)
  (_input : ProcessInput) (caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [FS] =
  match caps {
    ProgramCaps.MkProgramCaps cap |-> local_stage cap
  }
"#;

fn app(head: Term, args: impl IntoIterator<Item = Term>) -> Term {
    args.into_iter().fold(head, Term::app)
}

fn result_for(
    template: &ken_elaborator::ElabResult,
    name: &str,
    id: GlobalId,
) -> ken_elaborator::ElabResult {
    let mut result = template.clone();
    result.name = name.to_owned();
    result.def_id = id;
    result
}

fn standalone_result(name: &str, id: GlobalId) -> ken_elaborator::ElabResult {
    ken_elaborator::ElabResult {
        name: name.to_owned(),
        def_id: id,
        auxiliary_ids: vec![],
        postulate_ids: vec![],
        obligations: vec![],
        foreign_binding: None,
        temporal_obligations: vec![],
        effect_row_type: None,
    }
}

pub(super) fn relay_program() -> (ken_elaborator::ElabEnv, Vec<ken_elaborator::ElabResult>) {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages");
    let mut env = ken_elaborator::ElabEnv::new().expect("checked prelude");
    // Capture checked identities before catalog registration can mutate the
    // spelling table. The template never resolves these private names itself.
    let private_ids = [
        "FSOp",
        "AmbientOp",
        "fs_resp",
        "ambient_resp",
        "ReadFile",
        "host_exit",
    ]
    .map(|name| (name, env.globals[name]))
    .into_iter()
    .collect::<std::collections::HashMap<_, _>>();
    let private = ["FSOp", "AmbientOp", "fs_resp", "ambient_resp"].map(|s| private_ids[s]);
    let templates = env
        .elaborate_isolated_file_strict_v1(&[root], RELAY_TEMPLATE_SOURCE)
        .unwrap_or_else(|e| panic!("strict relay templates: {e:?}"));
    let mut program = Vec::with_capacity(8);
    let results = &templates;
    let template = &results[0];
    let Some(Decl::Transparent { body, .. }) = env.env.lookup(template.def_id) else {
        panic!("template not transparent")
    };
    let mut instantiated = body.clone();
    for (idx, id) in private.iter().copied().enumerate() {
        let Term::Lam(_, inner) = instantiated else {
            panic!("missing leading binder {idx}")
        };
        instantiated = ken_kernel::subst::subst0(&inner, &head(&env.env, id));
    }
    let afull = Term::constructor(env.globals["AFull"], vec![]);
    let result_type = app(
        head(&env.env, env.globals["ResourceBodyResult"]),
        vec![
            head(&env.env, env.globals["Unit"]),
            head(&env.env, env.globals["Unit"]),
        ],
    );
    let ty = Term::pi(
        head(&env.env, env.globals["BufferHandle"]),
        app(
            head(&env.env, env.globals["HostIO"]),
            vec![afull, result_type],
        ),
    );
    let id = ken_kernel::declare_def(&mut env.env, vec![], ty, instantiated)
        .expect("kernel admitted rt_branched_body");
    program.push(result_for(template, "rt_branched_body", id));
    let template2 = &results[1];
    let Some(Decl::Transparent { body, .. }) = env.env.lookup(template2.def_id) else {
        panic!("template 2 not transparent")
    };
    let mut second = body.clone();
    for (idx, key) in ["FSOp", "AmbientOp", "ReadFile"].iter().enumerate() {
        let Term::Lam(_, inner) = second else {
            panic!("missing template2 binder {idx}")
        };
        let selected = private_ids[*key];
        let arg = if env.env.constructor(selected).is_some() {
            Term::constructor(selected, vec![])
        } else {
            head(&env.env, selected)
        };
        second = ken_kernel::subst::subst0(&inner, &arg);
    }
    let afull = Term::constructor(env.globals["AFull"], vec![]);
    let cap = app(head(&env.env, env.globals["Cap"]), vec![afull.clone()]);
    let fs = app(head(&env.env, private_ids["FSOp"]), vec![afull]);
    let coproduct = app(
        head(&env.env, env.globals["Coproduct"]),
        vec![fs, head(&env.env, private_ids["AmbientOp"])],
    );
    let ty = Term::pi(cap, coproduct);
    let id = ken_kernel::declare_def(&mut env.env, vec![], ty, second)
        .expect("kernel admitted rt_computed_read");
    program.push(result_for(template2, "rt_computed_read", id));
    let computed = id;
    let template3 = &results[2];
    let Some(Decl::Transparent { body, .. }) = env.env.lookup(template3.def_id) else {
        panic!("template 3 not transparent")
    };
    let mut third = body.clone();
    for (idx, key) in [
        "FSOp",
        "AmbientOp",
        "fs_resp",
        "ambient_resp",
        "ReadFile",
        "rt_computed_read",
    ]
    .iter()
    .enumerate()
    {
        let Term::Lam(_, inner) = third else {
            panic!("missing template3 binder {idx}")
        };
        let selected = if idx == 5 {
            computed
        } else {
            private_ids[*key]
        };
        let arg = if env.env.constructor(selected).is_some() {
            Term::constructor(selected, vec![])
        } else {
            head(&env.env, selected)
        };
        third = ken_kernel::subst::subst0(&inner, &arg);
    }
    let afull = Term::constructor(env.globals["AFull"], vec![]);
    let cap = app(head(&env.env, env.globals["Cap"]), vec![afull.clone()]);
    let file = app(
        head(&env.env, env.globals["Resource"]),
        vec![Term::constructor(
            env.globals["ResourceKind.FsHandle"],
            vec![],
        )],
    );
    let buffer = head(&env.env, env.globals["BufferHandle"]);
    let unit = head(&env.env, env.globals["Unit"]);
    let result_type = app(
        head(&env.env, env.globals["ResourceBodyResult"]),
        vec![unit.clone(), unit],
    );
    let io = app(
        head(&env.env, env.globals["HostIO"]),
        vec![afull, result_type],
    );
    let ty = Term::pi(cap, Term::pi(file, Term::pi(buffer, io)));
    let id = ken_kernel::declare_def(&mut env.env, vec![], ty, third)
        .expect("kernel admitted rt_branched_endpoint_buffer");
    program.push(result_for(template3, "rt_branched_endpoint_buffer", id));
    let endpoint_buffer = id;
    let arm = &results[3];
    let Some(Decl::Transparent { body, .. }) = env.env.lookup(arm.def_id) else {
        panic!("template arm not transparent")
    };
    let mut arm = body.clone();
    for (idx, key) in ["FSOp", "AmbientOp", "fs_resp", "ambient_resp"]
        .iter()
        .enumerate()
    {
        let Term::Lam(_, inner) = arm else {
            panic!("missing arm binder {idx}")
        };
        arm = ken_kernel::subst::subst0(&inner, &head(&env.env, private_ids[*key]));
    }
    let template4 = &results[4];
    let Some(Decl::Transparent { body, .. }) = env.env.lookup(template4.def_id) else {
        panic!("template 4 not transparent")
    };
    let mut fourth = body.clone();
    for idx in 0..2 {
        let Term::Lam(_, inner) = fourth else {
            panic!("missing template4 arm {idx}")
        };
        fourth = ken_kernel::subst::subst0(&inner, &arm);
    }
    let unit = head(&env.env, env.globals["Unit"]);
    let bracket = app(
        head(&env.env, env.globals["ResourceBracketResult"]),
        vec![unit.clone(), unit.clone()],
    );
    let result = app(
        head(&env.env, env.globals["Result"]),
        vec![head(&env.env, env.globals["ResourceError"]), bracket],
    );
    let body_result = app(
        head(&env.env, env.globals["ResourceBodyResult"]),
        vec![unit.clone(), unit],
    );
    let io = app(
        head(&env.env, env.globals["HostIO"]),
        vec![Term::constructor(env.globals["AFull"], vec![]), body_result],
    );
    let ty = Term::pi(result, io);
    let id = ken_kernel::declare_def(&mut env.env, vec![], ty, fourth)
        .expect("kernel admitted rt_branched_after_buffer");
    program.push(result_for(template4, "rt_branched_after_buffer", id));
    let after_buffer = id;
    let literal_eight = match env.env.lookup(results[5].def_id) {
        Some(Decl::Transparent { body, .. }) => body.clone(),
        _ => panic!("checked eight template not transparent"),
    };
    let afull = Term::constructor(env.globals["AFull"], vec![]);
    let cap = app(head(&env.env, env.globals["Cap"]), vec![afull.clone()]);
    let file = app(
        head(&env.env, env.globals["Resource"]),
        vec![Term::constructor(
            env.globals["ResourceKind.FsHandle"],
            vec![],
        )],
    );
    let unit = head(&env.env, env.globals["Unit"]);
    let resource_body = app(
        head(&env.env, env.globals["ResourceBodyResult"]),
        vec![unit.clone(), unit.clone()],
    );
    let bracket = app(
        head(&env.env, env.globals["ResourceBracketResult"]),
        vec![unit.clone(), unit.clone()],
    );
    let outcome = app(
        head(&env.env, env.globals["Result"]),
        vec![head(&env.env, env.globals["ResourceError"]), bracket],
    );
    let fs = app(head(&env.env, private_ids["FSOp"]), vec![afull.clone()]);
    let ambient = head(&env.env, private_ids["AmbientOp"]);
    let ops = app(
        head(&env.env, env.globals["Coproduct"]),
        vec![fs.clone(), ambient.clone()],
    );
    let resp = app(
        head(&env.env, env.globals["resp_coproduct"]),
        vec![
            fs,
            ambient,
            app(head(&env.env, private_ids["fs_resp"]), vec![afull.clone()]),
            head(&env.env, private_ids["ambient_resp"]),
        ],
    );
    let endpoint_call = app(
        head(&env.env, endpoint_buffer),
        vec![Term::var(1), Term::var(0)],
    );
    let with_buffer = app(
        head(&env.env, env.globals["withBuffer"]),
        vec![
            afull.clone(),
            unit.clone(),
            unit,
            literal_eight,
            endpoint_call,
        ],
    );
    let on_result = Term::lam(
        outcome.clone(),
        Term::app(head(&env.env, after_buffer), Term::var(0)),
    );
    let bind = app(
        head(&env.env, env.globals["itree_bind"]),
        vec![
            ops,
            resp,
            outcome,
            resource_body.clone(),
            with_buffer,
            on_result,
        ],
    );
    let body = Term::lam(cap.clone(), Term::lam(file.clone(), bind));
    let io = app(
        head(&env.env, env.globals["HostIO"]),
        vec![afull, resource_body],
    );
    let ty = Term::pi(cap, Term::pi(file, io));
    let id = ken_kernel::declare_def(&mut env.env, vec![], ty, body)
        .expect("kernel admitted rt_branched_file");
    program.push(standalone_result("rt_branched_file", id));
    let file_def = id;
    let host_exit = head(&env.env, private_ids["host_exit"]);
    let arm_for = |position: usize| {
        let Some(Decl::Transparent { body, .. }) = env.env.lookup(results[position].def_id) else {
            panic!("done arm not transparent")
        };
        let Term::Lam(_, inner) = body.clone() else {
            panic!("done arm missing binder")
        };
        ken_kernel::subst::subst0(&inner, &host_exit)
    };
    let arms = [arm_for(6), arm_for(7)];
    let Some(Decl::Transparent { body, .. }) = env.env.lookup(results[8].def_id) else {
        panic!("done template not transparent")
    };
    let mut sixth = body.clone();
    for (idx, arm) in arms.iter().enumerate() {
        let Term::Lam(_, inner) = sixth else {
            panic!("done missing arm {idx}")
        };
        sixth = ken_kernel::subst::subst0(&inner, arm);
    }
    let unit = head(&env.env, env.globals["Unit"]);
    let bracket = app(
        head(&env.env, env.globals["ResourceBracketResult"]),
        vec![unit.clone(), unit],
    );
    let result = app(
        head(&env.env, env.globals["Result"]),
        vec![head(&env.env, env.globals["FileError"]), bracket],
    );
    let io = app(
        head(&env.env, env.globals["HostIO"]),
        vec![
            Term::constructor(env.globals["AFull"], vec![]),
            head(&env.env, env.globals["ExitCode"]),
        ],
    );
    let ty = Term::pi(result, io);
    let id = ken_kernel::declare_def(&mut env.env, vec![], ty, sixth)
        .expect("kernel admitted rt_branched_done");
    program.push(result_for(&results[8], "rt_branched_done", id));
    let done_def = id;
    let source_bytes = match env.env.lookup(results[9].def_id) {
        Some(Decl::Transparent { body, .. }) => body.clone(),
        _ => panic!("checked source template not transparent"),
    };
    let afull = Term::constructor(env.globals["AFull"], vec![]);
    let cap = app(head(&env.env, env.globals["Cap"]), vec![afull.clone()]);
    let unit = head(&env.env, env.globals["Unit"]);
    let bracket = app(
        head(&env.env, env.globals["ResourceBracketResult"]),
        vec![unit.clone(), unit.clone()],
    );
    let result = app(
        head(&env.env, env.globals["Result"]),
        vec![head(&env.env, env.globals["FileError"]), bracket],
    );
    let fs = app(head(&env.env, private_ids["FSOp"]), vec![afull.clone()]);
    let ambient = head(&env.env, private_ids["AmbientOp"]);
    let ops = app(
        head(&env.env, env.globals["Coproduct"]),
        vec![fs.clone(), ambient.clone()],
    );
    let resp = app(
        head(&env.env, env.globals["resp_coproduct"]),
        vec![
            fs,
            ambient,
            app(head(&env.env, private_ids["fs_resp"]), vec![afull.clone()]),
            head(&env.env, private_ids["ambient_resp"]),
        ],
    );
    let with_resource = app(
        head(&env.env, env.globals["withResource"]),
        vec![
            afull.clone(),
            unit.clone(),
            unit,
            Term::var(0),
            source_bytes,
            Term::constructor(env.globals["ResourceOpenMode.ResourceRead"], vec![]),
            Term::app(head(&env.env, file_def), Term::var(0)),
        ],
    );
    let on_result = Term::lam(
        result.clone(),
        Term::app(head(&env.env, done_def), Term::var(0)),
    );
    let body = Term::lam(
        cap.clone(),
        app(
            head(&env.env, env.globals["itree_bind"]),
            vec![
                ops,
                resp,
                result,
                head(&env.env, env.globals["ExitCode"]),
                with_resource,
                on_result,
            ],
        ),
    );
    let io = app(
        head(&env.env, env.globals["HostIO"]),
        vec![afull, head(&env.env, env.globals["ExitCode"])],
    );
    let ty = Term::pi(cap, io);
    let id = ken_kernel::declare_def(&mut env.env, vec![], ty, body)
        .expect("kernel admitted rt_branched_stage");
    program.push(standalone_result("rt_branched_stage", id));
    let stage_id = id;
    let Some(Decl::Transparent { body, .. }) = env.env.lookup(results[10].def_id) else {
        panic!("main template not transparent")
    };
    let Term::Lam(_, inner) = body.clone() else {
        panic!("main template missing stage binder")
    };
    let body = ken_kernel::subst::subst0(&inner, &head(&env.env, stage_id));
    let afull = Term::constructor(env.globals["AFull"], vec![]);
    let caps = app(
        head(&env.env, env.globals["ProgramCaps"]),
        vec![afull.clone()],
    );
    let io = app(
        head(&env.env, env.globals["HostIO"]),
        vec![afull, head(&env.env, env.globals["ExitCode"])],
    );
    let ty = Term::pi(
        head(&env.env, env.globals["ProcessInput"]),
        Term::pi(caps, io),
    );
    let id = ken_kernel::declare_def(&mut env.env, vec![], ty, body).expect("kernel admitted main");
    program.push(result_for(&results[10], "main", id));
    env.globals.insert("main".to_owned(), id);
    for result in &program {
        env.globals.insert(result.name.clone(), result.def_id);
    }
    (env, program)
}
