//! PX11 checked stream- and datagram-socket operations and the typed stream
//! option families. The carrier is the existing FSOp; `Net` is its
//! separately admitted effect-row label, not another authority.

use crate::{error::ElabError, ElabEnv};

fn checked(elab: &mut ElabEnv, name: &str, declaration: &str) -> Result<(), ElabError> {
    elab.elaborate_decl(declaration)
        .map_err(|error| ElabError::Internal(format!("prelude {name} failed: {error}")))?;
    Ok(())
}

pub(super) fn register_postbytes_socket_address(elab: &mut ElabEnv) -> Result<(), ElabError> {
    checked(
        elab,
        "private_socket_ipv4_octets",
        r#"
        fn private_socket_ipv4_octets
          (a : UInt8) (b : UInt8) (c : UInt8) (d : UInt8) (port : UInt16)
          : SocketAddress = PrivateSocketIPv4
            (list_to_bytes (Cons UInt8 a (Cons UInt8 b (Cons UInt8 c
              (Cons UInt8 d (Nil UInt8)))))) port
    "#,
    )?;
    checked(
        elab,
        "private_socket_ipv6_octets",
        r#"
        fn private_socket_ipv6_octets
          (a : UInt8) (b : UInt8) (c : UInt8) (d : UInt8)
          (e : UInt8) (f : UInt8) (g : UInt8) (h : UInt8)
          (i : UInt8) (j : UInt8) (k : UInt8) (l : UInt8)
          (m : UInt8) (n : UInt8) (o : UInt8) (p : UInt8)
          (port : UInt16) : SocketAddress = PrivateSocketIPv6
            (list_to_bytes (Cons UInt8 a (Cons UInt8 b (Cons UInt8 c
              (Cons UInt8 d (Cons UInt8 e (Cons UInt8 f (Cons UInt8 g
              (Cons UInt8 h (Cons UInt8 i (Cons UInt8 j (Cons UInt8 k
              (Cons UInt8 l (Cons UInt8 m (Cons UInt8 n (Cons UInt8 o
              (Cons UInt8 p (Nil UInt8)))))))))))))))))) port
    "#,
    )?;
    checked(
        elab,
        "private_socket_has_nul",
        r#"
        fn private_socket_has_nul (octets : List UInt8) : Bool =
          match octets {
            Nil |-> False;
            Cons byte rest |-> match eq_int (uint8_to_int byte) (0 : Int) {
              True |-> True;
              False |-> private_socket_has_nul rest
            }
          }
    "#,
    )?;
    checked(
        elab,
        "socket_unix_path",
        r#"
        fn socket_unix_path (path : Bytes) : Result IOError SocketAddress =
          match leq_int (2 : Int) (bytes_length path) {
            False |-> Err IOError SocketAddress IOError.InvalidInput;
            True |-> match leq_int (bytes_length path) (107 : Int) {
              False |-> Err IOError SocketAddress IOError.InvalidInput;
              True |-> match bytes_at path (0 : Int) {
                None |-> Err IOError SocketAddress IOError.InvalidInput;
                Some first |-> match eq_int (uint8_to_int first) (47 : Int) {
                  False |-> Err IOError SocketAddress IOError.InvalidInput;
                  True |-> match private_socket_has_nul (bytes_to_list path) {
                    True |-> Err IOError SocketAddress IOError.InvalidInput;
                    False |-> Ok IOError SocketAddress (PrivateSocketUnix path)
                  }
                }
              }
            }
          }
    "#,
    )?;
    Ok(())
}

pub(super) fn register_prelude_net(elab: &mut ElabEnv) -> Result<(), ElabError> {
    checked(
        elab,
        "private_net_error",
        r#"
        fn private_net_error (a : Auth) (r : Type) (error : ResourceError)
          : HostIO a (Result ResourceError r) =
          Ret (Coproduct (FSOp a) AmbientOp)
            (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
            (Result ResourceError r) (Err ResourceError r error)
    "#,
    )?;
    checked(
        elab,
        "private_net_after_acquire",
        r#"
        proc private_net_after_acquire (a : Auth) (e : Type) (r : Type)
          (kind : ResourceKind)
          (body : Resource kind -> HostIO a (ResourceBodyResult e r))
          (acquired : Result ResourceError (Resource kind))
          : HostIO a (Result ResourceError (ResourceBracketResult e r)) visits [Net] =
          match acquired {
            Err error |-> private_net_error a (ResourceBracketResult e r) error;
            Ok resource |-> bind (Coproduct (FSOp a) AmbientOp)
              (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
              (ResourceBodyResult e r)
              (Result ResourceError (ResourceBracketResult e r))
              (body resource)
              (\body_result. Vis (Coproduct (FSOp a) AmbientOp)
                (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
                (Result ResourceError (ResourceBracketResult e r))
                (InL (FSOp a) AmbientOp
                  (PrivateBracketRelease a kind resource))
                (\settled. Ret (Coproduct (FSOp a) AmbientOp)
                  (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
                  (Result ResourceError (ResourceBracketResult e r))
                  (resource_settle_result_for ResourceError e r
                    body_result settled)))
          }
    "#,
    )?;
    for (name, kind, cap, address, op) in [
        (
            "with_socket_listener",
            "SocketListener",
            "(cap : Cap a)",
            "(address : SocketAddress)",
            "PrivateNetListen a cap address",
        ),
        (
            "with_socket_accept",
            "SocketStream",
            "",
            "(listener : Resource ResourceKind.SocketListener)",
            "PrivateNetAccept a listener",
        ),
        (
            "with_socket_connection",
            "SocketStream",
            "(cap : Cap a)",
            "(address : SocketAddress)",
            "PrivateNetConnect a cap address",
        ),
        (
            "with_datagram_socket",
            "SocketDatagram",
            "(cap : Cap a)",
            "(address : SocketAddress)",
            "PrivateNetDatagramBind a cap address",
        ),
        // PX11-SOCKET-OPTION-FAMILIES: address reuse is a bind-time
        // property, so it is an acquisition rather than a settable option.
        (
            "with_reusable_socket_listener",
            "SocketListener",
            "(cap : Cap a)",
            "(address : SocketAddress)",
            "PrivateNetListenReusable a cap address",
        ),
    ] {
        let declaration = format!(
            r#"
            proc {name} (a : Auth) (e : Type) (r : Type)
              {cap} {address}
              (body : Resource ResourceKind.{kind} -> HostIO a (ResourceBodyResult e r))
              : HostIO a (Result ResourceError (ResourceBracketResult e r)) visits [Net] =
              Vis (Coproduct (FSOp a) AmbientOp)
                (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
                (Result ResourceError (ResourceBracketResult e r))
                (InL (FSOp a) AmbientOp ({op}))
                (\acquired. private_net_after_acquire a e r ResourceKind.{kind}
                  body acquired)
        "#
        );
        checked(elab, name, &declaration)?;
    }
    checked(
        elab,
        "receive_socket",
        r#"
        proc receive_socket (a : Auth) (stream : Resource ResourceKind.SocketStream)
          (buffer : BufferHandle) (window : BufferWindow)
          : HostIO a (Result ResourceError ReadProgress) visits [Net] =
          match leq_int (0 : Int) (buffer_window_start window) {
            False |-> private_net_error a ReadProgress InvalidBounds;
            True |-> match leq_int (0 : Int) (buffer_window_length window) {
              False |-> private_net_error a ReadProgress InvalidBounds;
              True |-> match leq_int (buffer_window_start window)
                  (buffer_handle_capacity buffer) {
                False |-> private_net_error a ReadProgress InvalidBounds;
                True |-> let effective = buffer_min_int (buffer_window_length window)
                    (sub_int (buffer_handle_capacity buffer) (buffer_window_start window)) in
                  match eq_int effective (0 : Int) {
                    True |-> Ret (Coproduct (FSOp a) AmbientOp)
                      (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
                      (Result ResourceError ReadProgress)
                      (Ok ResourceError ReadProgress ReadEof);
                    False |-> Vis (Coproduct (FSOp a) AmbientOp)
                      (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
                      (Result ResourceError ReadProgress)
                      (InL (FSOp a) AmbientOp (PrivateNetReceive a stream
                        (buffer_handle_resource buffer) (buffer_window_start window)
                        effective))
                      (\response. Ret (Coproduct (FSOp a) AmbientOp)
                        (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
                        (Result ResourceError ReadProgress) response)
                  }
              }
            }
          }
    "#,
    )?;
    checked(
        elab,
        "send_socket",
        r#"
        proc send_socket (a : Auth) (stream : Resource ResourceKind.SocketStream)
          (buffer : BufferHandle) (span : BufferSpan)
          : HostIO a (Result ResourceError WriteProgress) visits [Net] =
          Vis (Coproduct (FSOp a) AmbientOp)
            (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
            (Result ResourceError WriteProgress)
            (InL (FSOp a) AmbientOp (PrivateNetSend a stream
              (buffer_handle_resource buffer) (buffer_span_start span)
              (buffer_span_length span) (buffer_span_origin span)))
            (\response. Ret (Coproduct (FSOp a) AmbientOp)
              (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
              (Result ResourceError WriteProgress) response)
    "#,
    )?;
    checked(
        elab,
        "send_datagram",
        r#"
        proc send_datagram (a : Auth) (socket : Resource ResourceKind.SocketDatagram)
          (peer : SocketAddress) (buffer : BufferHandle) (span : BufferSpan)
          : HostIO a (Result ResourceError Unit) visits [Net] =
          Vis (Coproduct (FSOp a) AmbientOp)
            (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
            (Result ResourceError Unit)
            (InL (FSOp a) AmbientOp (PrivateNetDatagramSendTo a socket
              (buffer_handle_resource buffer) (buffer_span_start span)
              (buffer_span_length span) (buffer_span_origin span) peer))
            (\response. Ret (Coproduct (FSOp a) AmbientOp)
              (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
              (Result ResourceError Unit) response)
    "#,
    )?;
    // A datagram socket has no end of stream, so a zero effective window is
    // InvalidBounds without a host call rather than ReadEof.
    checked(
        elab,
        "receive_datagram",
        r#"
        proc receive_datagram (a : Auth) (socket : Resource ResourceKind.SocketDatagram)
          (buffer : BufferHandle) (window : BufferWindow)
          : HostIO a (Result ResourceError DatagramReceipt) visits [Net] =
          match leq_int (0 : Int) (buffer_window_start window) {
            False |-> private_net_error a DatagramReceipt InvalidBounds;
            True |-> match leq_int (0 : Int) (buffer_window_length window) {
              False |-> private_net_error a DatagramReceipt InvalidBounds;
              True |-> match leq_int (buffer_window_start window)
                  (buffer_handle_capacity buffer) {
                False |-> private_net_error a DatagramReceipt InvalidBounds;
                True |-> let effective = buffer_min_int (buffer_window_length window)
                    (sub_int (buffer_handle_capacity buffer) (buffer_window_start window)) in
                  match eq_int effective (0 : Int) {
                    True |-> private_net_error a DatagramReceipt InvalidBounds;
                    False |-> Vis (Coproduct (FSOp a) AmbientOp)
                      (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
                      (Result ResourceError DatagramReceipt)
                      (InL (FSOp a) AmbientOp (PrivateNetDatagramReceiveFrom a socket
                        (buffer_handle_resource buffer) (buffer_window_start window)
                        effective))
                      (\response. Ret (Coproduct (FSOp a) AmbientOp)
                        (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
                        (Result ResourceError DatagramReceipt) response)
                  }
              }
            }
          }
    "#,
    )?;
    for (name, ty, operand, result) in [
        (
            "datagram_local_address",
            "(socket : Resource ResourceKind.SocketDatagram)",
            "PrivateNetDatagramLocalAddress a socket",
            "SocketAddress",
        ),
        (
            "shutdown_socket",
            "(stream : Resource ResourceKind.SocketStream) (how : SocketShutdown)",
            "PrivateNetShutdown a stream how",
            "Unit",
        ),
        (
            "socket_local_address",
            "(target : SocketTarget)",
            "PrivateNetLocalAddress a target",
            "SocketAddress",
        ),
        (
            "set_stream_option",
            "(stream : Resource ResourceKind.SocketStream) (option : StreamOption) (enabled : Bool)",
            "PrivateNetStreamOptionSet a stream option enabled",
            "Unit",
        ),
        (
            "stream_option",
            "(stream : Resource ResourceKind.SocketStream) (option : StreamOption)",
            "PrivateNetStreamOptionGet a stream option",
            "Bool",
        ),
    ] {
        let declaration = format!(
            r#"
            proc {name} (a : Auth) {ty}
              : HostIO a (Result ResourceError {result}) visits [Net] =
              Vis (Coproduct (FSOp a) AmbientOp)
                (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
                (Result ResourceError {result})
                (InL (FSOp a) AmbientOp ({operand}))
                (\response. Ret (Coproduct (FSOp a) AmbientOp)
                  (resp_coproduct (FSOp a) AmbientOp (fs_resp a) ambient_resp)
                  (Result ResourceError {result}) response)
        "#
        );
        checked(elab, name, &declaration)?;
    }
    Ok(())
}
