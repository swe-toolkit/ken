// Shared exact checked source generator for nested bracket controls and D0.
#[cfg(target_os = "linux")]
pub(super) fn nested_bracket_source(depth: usize) -> String {
    let body = match depth {
        1 => "leaf_body",
        2 => {
            r#"(\resource.
          bind (Coproduct (FSOp AFull) AmbientOp)
            (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
            (Result FileError (ResourceBracketResult Unit Unit))
            (ResourceBodyResult Unit Unit)
            (withResource AFull Unit Unit cap (bytes_encode "held-1.bin")
              ResourceOpenMode.ResourceMetadata leaf_body)
            (\outcome. body_result outcome))"#
        }
        3 => {
            r#"(\resource.
          bind (Coproduct (FSOp AFull) AmbientOp)
            (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
            (Result FileError (ResourceBracketResult Unit Unit))
            (ResourceBodyResult Unit Unit)
            (withResource AFull Unit Unit cap (bytes_encode "held-1.bin")
              ResourceOpenMode.ResourceMetadata (\inner_resource.
                bind (Coproduct (FSOp AFull) AmbientOp)
                  (resp_coproduct (FSOp AFull) AmbientOp
                    (fs_resp AFull) ambient_resp)
                  (Result FileError (ResourceBracketResult Unit Unit))
                  (ResourceBodyResult Unit Unit)
                  (withResource AFull Unit Unit cap (bytes_encode "held-2.bin")
                    ResourceOpenMode.ResourceMetadata leaf_body)
                  (\inner_outcome. body_result inner_outcome)))
            (\outcome. body_result outcome))"#
        }
        _ => panic!("PX8-TA public control supports depths one through three"),
    };
    include_str!("../fixtures/d0_px8ta_nested_source.ken").replace("__ROOT_BODY__", body)
}
