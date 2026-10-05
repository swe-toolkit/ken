//! Independently stated Console.Text interaction-tree laws.

use ken_elaborator::ElabEnv;
use ken_kernel::Decl;
use std::collections::BTreeSet;

const CONSOLE_TEXT: &str = include_str!("../../../catalog/packages/Capability/Console/Text.ken.md");

#[test]
fn console_text_laws_check_against_independent_trees_without_trust() {
    let mut env = ElabEnv::empty().expect("prelude with transparent Console.write");
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_ken_md_file(CONSOLE_TEXT)
        .expect("Console.Text package and four checked laws");
    let after_package: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(
        before, after_package,
        "package adds no trusted declarations"
    );

    // These are not the package's console_write_tree or console_line_payload.
    // They give the clients an independently authored expected tree and byte
    // expression. A package law has to convert to each client's statement.
    env.elaborate_decl(
        r"fn client_console_tree (stream : Stream) (payload : Bytes)
             : IO (Result IOError Unit) =
           Vis ConsoleOp console_resp (Result IOError Unit)
             (Write stream payload)
             (λr. Ret ConsoleOp console_resp (Result IOError Unit) r)",
    )
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
            r"theorem client_print_write_tree (text : String)
                 : Equal (IO (Result IOError Unit))
                     (print text)
                     (client_console_tree Stdout (bytes_encode text)) =
               print_write_tree text",
        ),
        (
            "print_line_write_tree",
            r"theorem client_print_line_write_tree (text : String)
                 : Equal (IO (Result IOError Unit))
                     (printLine text)
                     (client_console_tree Stdout (client_console_line_bytes text)) =
               print_line_write_tree text",
        ),
        (
            "eprint_write_tree",
            r"theorem client_eprint_write_tree (text : String)
                 : Equal (IO (Result IOError Unit))
                     (eprint text)
                     (client_console_tree Stderr (bytes_encode text)) =
               eprint_write_tree text",
        ),
        (
            "eprint_line_write_tree",
            r"theorem client_eprint_line_write_tree (text : String)
                 : Equal (IO (Result IOError Unit))
                     (eprintLine text)
                     (client_console_tree Stderr (client_console_line_bytes text)) =
               eprint_line_write_tree text",
        ),
    ];
    for (law, client) in cases {
        let id = env.globals[law];
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{law} must be a kernel-checked transparent proof"
        );
        assert!(!env.env.trusted_base().contains(&id));
        env.elaborate_decl(client)
            .unwrap_or_else(|error| panic!("independent type of {law} must check: {error:?}"));
    }
    let after_clients: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(before, after_clients, "clients add no trusted declarations");
}
