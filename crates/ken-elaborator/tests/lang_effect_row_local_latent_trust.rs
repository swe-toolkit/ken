//! Spec 36 §1.6.4 and Architect AC-4. Promise class: durable invariant.
//! MEASURED: checked data/record/alias declarations and a refused callback
//! leave the kernel's trusted_base() set unchanged. CLAIMED: this surface
//! row-plane repair adds no trusted assumption. THE GAP: the test covers
//! these admitted forms; code review checks that no other constructor path
//! adds a postulate, and CI checks other elaboration routes.

use ken_elaborator::{ElabEnv, ElabError};

#[test]
fn carried_rows_change_escape_judgment_without_changing_trusted_base() {
    for (carrier, callback, owner) in [
        (
            "data Box = MkBox (Unit ->[Console] HostIO APartial Unit)",
            "fn open_box (b : Box) : HostIO APartial Unit = \
             match b { MkBox g ↦ g MkUnit }",
            "open_box",
        ),
        (
            "record Carrier { cb : Unit ->[Console] HostIO APartial Unit }",
            "fn open_record (r : Carrier) : HostIO APartial Unit = r.cb MkUnit",
            "open_record",
        ),
        (
            "def Callback = Unit ->[Console] HostIO APartial Unit",
            "fn open_alias (f : Callback) : HostIO APartial Unit = f MkUnit",
            "open_alias",
        ),
    ] {
        let mut elab = ElabEnv::new().expect("prelude");
        let trusted = elab.env.trusted_base();
        elab.elaborate_file(carrier).expect("carrier admission");
        assert_eq!(elab.env.trusted_base(), trusted, "{carrier}: admission");
        let err = elab
            .elaborate_file(callback)
            .expect_err("callback must escape");
        match err {
            ElabError::TypeMismatch { reason, .. } => assert!(
                reason.contains(&format!("EffectEscapes in '{owner}': Console")),
                "{owner}: wrong refusal: {reason}"
            ),
            other => panic!("{owner}: wrong error: {other:?}"),
        }
        assert_eq!(elab.env.trusted_base(), trusted, "{carrier}: refusal");
    }
}
