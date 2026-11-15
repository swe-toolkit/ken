//! Independently stated Console.Text interaction-tree laws.

#[path = "support/catalog_or.rs"]
mod catalog_or;

use ken_elaborator::ElabEnv;
use ken_kernel::Decl;
use std::collections::BTreeSet;

#[test]
fn console_text_laws_check_against_independent_trees_without_trust() {
    let mut env = ElabEnv::empty().expect("prelude with transparent Console.write");
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    let console_owned = env
        .elaborate_module_from_roots(&[catalog_or::catalog_root()], "Capability.Console.Text")
        .expect("Console.Text package and four checked laws");
    env.execute_loaded_entry_checked_fences("Capability.Console.Text")
        .expect("Capability.Console.Text checked fences keep their contract");
    let after_package: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(
        before, after_package,
        "package adds no trusted declarations"
    );

    // These do not use the package's console_line_payload.
    // They give the clients an independently authored expected tree and byte
    // expression. A package law has to convert to each client's statement.
    env.elaborate_file(&format!(
        "import Capability.Console.Text (IO)\n{}",
        r"fn client_console_tree (stream : ConsoleStream) (payload : Bytes)
             : IO (Result IOError Unit) =
           ITree.Vis ConsoleOp console_resp (Result IOError Unit)
             (ConsoleOp.Write stream payload)
             (λr. ITree.Ret ConsoleOp console_resp (Result IOError Unit) r)",
    ))
    .expect("independent one-Write Result-passthrough tree");
    env.elaborate_decl(
        r"fn client_console_line_bytes (text : String) : Bytes =
           bytes_concat
             (bytes_encode text)
             (bytes_encode (list_char_to_string (Cons Char (10 : Int) (Nil Char))))",
    )
    .expect("independent single-newline byte expression");

    let cases = [
        (
            "print_write_tree",
            "print, print_write_tree",
            r"theorem client_print_write_tree (text : String)
                 : Equal (IO (Result IOError Unit))
                     (print text)
                     (client_console_tree ConsoleStream.Stdout (bytes_encode text)) =
               print_write_tree text",
        ),
        (
            "print_line_write_tree",
            "printLine, print_line_write_tree",
            r"theorem client_print_line_write_tree (text : String)
                 : Equal (IO (Result IOError Unit))
                     (printLine text)
                     (client_console_tree ConsoleStream.Stdout (client_console_line_bytes text)) =
               print_line_write_tree text",
        ),
        (
            "eprint_write_tree",
            "eprint, eprint_write_tree",
            r"theorem client_eprint_write_tree (text : String)
                 : Equal (IO (Result IOError Unit))
                     (eprint text)
                     (client_console_tree ConsoleStream.Stderr (bytes_encode text)) =
               eprint_write_tree text",
        ),
        (
            "eprint_line_write_tree",
            "eprintLine, eprint_line_write_tree",
            r"theorem client_eprint_line_write_tree (text : String)
                 : Equal (IO (Result IOError Unit))
                     (eprintLine text)
                     (client_console_tree ConsoleStream.Stderr (client_console_line_bytes text)) =
               eprint_line_write_tree text",
        ),
    ];
    for (law, imports, client) in cases {
        let id = catalog_or::provider_owned_id(&env, &console_owned, "Capability.Console.Text", law)
            .unwrap_or_else(|error| panic!("Console.Text owner {law}: {error}"));
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{law} must be a kernel-checked transparent proof"
        );
        assert!(!env.env.trusted_base().contains(&id));
        env.elaborate_file(&format!("import Capability.Console.Text ({imports})\n{client}"))
            .unwrap_or_else(|error| panic!("independent type of {law} must check: {error:?}"));
    }
    let after_clients: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(before, after_clients, "clients add no trusted declarations");
}
