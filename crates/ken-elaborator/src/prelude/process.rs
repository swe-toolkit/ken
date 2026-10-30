//! Checked PX10 pure-plan replay and bracketed child surface. All host work
//! remains in the shared FSOp carrier; the Process row is an effect label,
//! not another capability or interpreter.

use crate::{ElabEnv, error::ElabError};

fn checked(elab: &mut ElabEnv, name: &str, declaration: &str) -> Result<(), ElabError> {
    elab.elaborate_decl(declaration)
        .map_err(|error| ElabError::Internal(format!("prelude {name} failed: {error}")))?;
    Ok(())
}

pub(super) fn register_prelude_process(elab: &mut ElabEnv) -> Result<(), ElabError> {
    checked(elab, "private_process_error", r#"
        fn private_process_error (a : Auth) (r : Type) (error : ResourceError)
          : HostIO a (Result ResourceError r) =
          Ret (Coproduct (FSOp a) AmbientOp)
            (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
            (Result ResourceError r) (Err ResourceError r error)
    "#)?;
    checked(elab, "private_process_push_arguments", r#"
        proc private_process_push_arguments (a : Auth)
          (plan : Resource ResourceKind.SpawnPlan) (arguments : List Bytes)
          : HostIO a (Result ResourceError Unit) visits [Process] =
          match arguments {
            Nil |-> Ret (Coproduct (FSOp a) AmbientOp)
              (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
              (Result ResourceError Unit) (Ok ResourceError Unit MkUnit);
            Cons argument tail |-> bind (Coproduct (FSOp a) AmbientOp)
              (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
              (Result ResourceError Unit) (Result ResourceError Unit)
              (Vis (Coproduct (FSOp a) AmbientOp)
                (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
                (Result ResourceError Unit)
                (InL (FSOp a) AmbientOp (PrivateProcessPlanPush a plan 0 argument b"" 0))
                (\result. Ret (Coproduct (FSOp a) AmbientOp)
                  (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
                  (Result ResourceError Unit) result))
              (\result. match result {
                Err error |-> private_process_error a Unit error;
                Ok _ |-> private_process_push_arguments a plan tail
              })
          }
    "#)?;
    checked(elab, "private_process_push_environment", r#"
        proc private_process_push_environment (a : Auth)
          (plan : Resource ResourceKind.SpawnPlan) (environment : List (Prod Bytes Bytes))
          : HostIO a (Result ResourceError Unit) visits [Process] =
          match environment {
            Nil |-> Ret (Coproduct (FSOp a) AmbientOp)
              (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
              (Result ResourceError Unit) (Ok ResourceError Unit MkUnit);
            Cons entry tail |-> match entry {
              MkProd key value |-> bind (Coproduct (FSOp a) AmbientOp)
                (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
                (Result ResourceError Unit) (Result ResourceError Unit)
                (Vis (Coproduct (FSOp a) AmbientOp)
                  (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
                  (Result ResourceError Unit)
                  (InL (FSOp a) AmbientOp (PrivateProcessPlanPush a plan 1 key value 0))
                  (\result. Ret (Coproduct (FSOp a) AmbientOp)
                    (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
                    (Result ResourceError Unit) result))
                (\result. match result {
                  Err error |-> private_process_error a Unit error;
                  Ok _ |-> private_process_push_environment a plan tail
                })
            }
          }
    "#)?;
    checked(elab, "process_stdio_scalar", r#"
        fn process_stdio_scalar (fd : Int) (target : ChildStdio) : Int =
          match target {
            ChildPipe |-> fd;
            ChildNull |-> add_int fd 256;
            ChildInherit parent |-> match parent {
              Stdin |-> add_int fd 512;
              Stdout |-> add_int fd 768;
              Stderr |-> add_int fd 1024
            }
          }
    "#)?;
    checked(elab, "private_process_push_stdio", r#"
        proc private_process_push_stdio (a : Auth)
          (plan : Resource ResourceKind.SpawnPlan) (fd : Int) (target : ChildStdio)
          : HostIO a (Result ResourceError Unit) visits [Process] =
          Vis (Coproduct (FSOp a) AmbientOp)
            (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
            (Result ResourceError Unit)
            (InL (FSOp a) AmbientOp (PrivateProcessPlanPush a plan 2 b"" b""
              (process_stdio_scalar fd target)))
            (\result. Ret (Coproduct (FSOp a) AmbientOp)
              (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
              (Result ResourceError Unit) result)
    "#)?;
    checked(elab, "private_process_then", r#"
        proc private_process_then (a : Auth) (r : Type)
          (plan : Resource ResourceKind.SpawnPlan)
          (action : HostIO a (Result ResourceError Unit))
          (next : Unit -> HostIO a (Result ResourceError r))
          : HostIO a (Result ResourceError r) visits [Process] =
          bind (Coproduct (FSOp a) AmbientOp)
            (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
            (Result ResourceError Unit) (Result ResourceError r) action
            (\result. match result {
              Err error |-> bind (Coproduct (FSOp a) AmbientOp)
                (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
                (Result ResourceError Unit) (Result ResourceError r)
                (Vis (Coproduct (FSOp a) AmbientOp)
                  (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
                  (Result ResourceError Unit)
                  (InL (FSOp a) AmbientOp
                    (PrivateResourceRelease a ResourceKind.SpawnPlan plan))
                  (\settled. Ret (Coproduct (FSOp a) AmbientOp)
                    (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
                    (Result ResourceError Unit) settled))
                (\_. private_process_error a r error);
              Ok _ |-> next MkUnit
            })
    "#)?;
    checked(elab, "private_process_release_child", r#"
        proc private_process_release_child (a : Auth) (e : Type) (r : Type)
          (child : Resource ResourceKind.ChildProcess)
          (body_result : ResourceBodyResult e r)
          : HostIO a (Result ResourceError (ResourceBracketResult e r)) visits [Process] =
          Vis (Coproduct (FSOp a) AmbientOp)
            (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
            (Result ResourceError (ResourceBracketResult e r))
            (InL (FSOp a) AmbientOp
              (PrivateBracketRelease a ResourceKind.ChildProcess child))
            (\settled. Ret (Coproduct (FSOp a) AmbientOp)
              (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
              (Result ResourceError (ResourceBracketResult e r))
              (resource_settle_result_for ResourceError e r body_result settled))
    "#)?;
    checked(elab, "private_process_after_push", r#"
        proc private_process_after_push (a : Auth) (e : Type) (r : Type)
          (plan : Resource ResourceKind.SpawnPlan)
          (body : Resource ResourceKind.ChildProcess -> HostIO a (ResourceBodyResult e r))
          : HostIO a (Result ResourceError (ResourceBracketResult e r)) visits [Process] =
          Vis (Coproduct (FSOp a) AmbientOp)
            (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
            (Result ResourceError (ResourceBracketResult e r))
            (InL (FSOp a) AmbientOp (PrivateProcessSpawn a plan))
            (\acquired. match acquired {
              Err error |-> private_process_error a (ResourceBracketResult e r) error;
              Ok child |-> bind (Coproduct (FSOp a) AmbientOp)
                (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
                (ResourceBodyResult e r)
                (Result ResourceError (ResourceBracketResult e r))
                (body child)
                (\body_result. private_process_release_child a e r child body_result)
            })
    "#)?;
    checked(elab, "private_process_after_open", r#"
        proc private_process_after_open (a : Auth) (e : Type) (r : Type)
          (pure_plan : SpawnPlan)
          (body : Resource ResourceKind.ChildProcess -> HostIO a (ResourceBodyResult e r))
          (acquired : Result ResourceError (Resource ResourceKind.SpawnPlan))
          : HostIO a (Result ResourceError (ResourceBracketResult e r)) visits [Process] =
          match acquired {
            Err error |-> private_process_error a (ResourceBracketResult e r) error;
            Ok plan |-> match pure_plan {
              PrivateSpawnPlan program arguments environment stdin stdout stderr |->
                private_process_then a (ResourceBracketResult e r) plan
                  (private_process_push_arguments a plan arguments)
                  (\_. private_process_then a (ResourceBracketResult e r) plan
                    (private_process_push_environment a plan environment)
                    (\_. private_process_then a (ResourceBracketResult e r) plan
                      (private_process_push_stdio a plan 0 stdin)
                      (\_. private_process_then a (ResourceBracketResult e r) plan
                        (private_process_push_stdio a plan 1 stdout)
                        (\_. private_process_then a (ResourceBracketResult e r) plan
                          (private_process_push_stdio a plan 2 stderr)
                          (\_. private_process_after_push a e r plan body)))))
            }
          }
    "#)?;
    checked(elab, "withChild", r#"
        proc withChild (a : Auth) (e : Type) (r : Type)
          (cap : Cap a) (plan : SpawnPlan)
          (body : Resource ResourceKind.ChildProcess -> HostIO a (ResourceBodyResult e r))
          : HostIO a (Result ResourceError (ResourceBracketResult e r)) visits [Process] =
          match plan {
            PrivateSpawnPlan program arguments environment stdin stdout stderr |->
              Vis (Coproduct (FSOp a) AmbientOp)
                (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
                (Result ResourceError (ResourceBracketResult e r))
                (InL (FSOp a) AmbientOp (PrivateProcessPlanOpen a cap program))
                (\acquired. private_process_after_open a e r plan body acquired)
          }
    "#)?;
    for (name, result, op) in [
        ("take_stdin", "Resource ResourceKind.PipeWrite", "PrivateProcessTakePipeWrite a child"),
        ("take_stdout", "Resource ResourceKind.PipeRead", "PrivateProcessTakePipeRead a child 1"),
        ("take_stderr", "Resource ResourceKind.PipeRead", "PrivateProcessTakePipeRead a child 2"),
        ("wait_child", "ChildExit", "PrivateProcessWait a child"),
    ] {
        let declaration = format!(r#"
            proc {name} (a : Auth) (child : Resource ResourceKind.ChildProcess)
              : HostIO a (Result ResourceError ({result})) visits [Process] =
              Vis (Coproduct (FSOp a) AmbientOp)
                (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
                (Result ResourceError ({result})) (InL (FSOp a) AmbientOp ({op}))
                (\result. Ret (Coproduct (FSOp a) AmbientOp)
                  (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
                  (Result ResourceError ({result})) result)
        "#);
        checked(elab, name, &declaration)?;
    }
    checked(elab, "private_process_read_result", r#"
        fn private_process_read_result (a : Auth) (result : Result ResourceError ReadProgress)
          : HostIO a (Result ResourceError ReadProgress) =
          Ret (Coproduct (FSOp a) AmbientOp)
            (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
            (Result ResourceError ReadProgress) result
    "#)?;
    checked(elab, "read_pipe", r#"
        proc read_pipe (a : Auth) (pipe : Resource ResourceKind.PipeRead)
          (buffer : BufferHandle) (window : BufferWindow)
          : HostIO a (Result ResourceError ReadProgress) visits [Process] =
          match leq_int (0 : Int) (buffer_window_start window) {
            False |-> private_process_read_result a (Err ResourceError ReadProgress InvalidBounds);
            True |-> match leq_int (0 : Int) (buffer_window_length window) {
              False |-> private_process_read_result a (Err ResourceError ReadProgress InvalidBounds);
              True |-> match leq_int (buffer_window_start window) (buffer_handle_capacity buffer) {
                False |-> private_process_read_result a (Err ResourceError ReadProgress InvalidBounds);
                True |-> let effective = buffer_min_int (buffer_window_length window)
                    (sub_int (buffer_handle_capacity buffer) (buffer_window_start window)) in
                  match eq_int effective (0 : Int) {
                    True |-> private_process_read_result a (Ok ResourceError ReadProgress ReadEof);
                    False |-> Vis (Coproduct (FSOp a) AmbientOp)
                      (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
                      (Result ResourceError ReadProgress)
                      (InL (FSOp a) AmbientOp (PrivateResourceStreamRead a pipe
                        (buffer_handle_resource buffer) (buffer_window_start window) effective))
                      (\result. private_process_read_result a result)
                  }
              }
            }
          }
    "#)?;
    checked(elab, "write_pipe", r#"
        proc write_pipe (a : Auth) (pipe : Resource ResourceKind.PipeWrite)
          (buffer : BufferHandle) (span : BufferSpan)
          : HostIO a (Result ResourceError WriteProgress) visits [Process] =
          Vis (Coproduct (FSOp a) AmbientOp)
            (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
            (Result ResourceError WriteProgress)
            (InL (FSOp a) AmbientOp (PrivateResourceStreamWrite a pipe
              (buffer_handle_resource buffer) (buffer_span_start span)
              (buffer_span_length span) (buffer_span_origin span)))
            (\result. Ret (Coproduct (FSOp a) AmbientOp)
              (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
              (Result ResourceError WriteProgress) result)
    "#)?;
    for (name, kind) in [
        ("close_pipe_write", "PipeWrite"),
        ("close_pipe_read", "PipeRead"),
    ] {
        let declaration = format!(r#"
            proc {name} (a : Auth) (pipe : Resource ResourceKind.{kind})
              : HostIO a (Result ResourceError Unit) visits [Process] =
              Vis (Coproduct (FSOp a) AmbientOp)
                (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
                (Result ResourceError Unit)
                (InL (FSOp a) AmbientOp (PrivateResourceRelease a ResourceKind.{kind} pipe))
                (\result. Ret (Coproduct (FSOp a) AmbientOp)
                  (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
                  (Result ResourceError Unit) result)
        "#);
        checked(elab, name, &declaration)?;
    }
    Ok(())
}
