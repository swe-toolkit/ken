# `Console` — ordinary text-output helpers

This package keeps text policy above the byte-exact Console ABI. The built-in
`write` operation accepts bytes without encoding or newline behavior; these
four helpers choose UTF-8 encoding, select stdout or stderr, and make the
line-ending choice explicit. They are ordinary kernel-checked Ken definitions
with zero `trusted_base()` delta.

```ken
proc print (text : String) : IO (Result IOError Unit) visits [Console] =
  write Stdout (bytes_encode text)

proc printLine (text : String) : IO (Result IOError Unit) visits [Console] =
  write
    Stdout
    (bytes_concat
      (bytes_encode text)
      (bytes_encode (list_char_to_string (Cons Char (10 : Int) (Nil Char)))))

proc eprint (text : String) : IO (Result IOError Unit) visits [Console] =
  write Stderr (bytes_encode text)

proc eprintLine (text : String) : IO (Result IOError Unit) visits [Console] =
  write
    Stderr
    (bytes_concat
      (bytes_encode text)
      (bytes_encode (list_char_to_string (Cons Char (10 : Int) (Nil Char)))))

pub theorem print_write_tree
      (text : String)
    : Equal (IO (Result IOError Unit)) (print text) (write Stdout (bytes_encode text)) =
  console_tree_refl (IO (Result IOError Unit)) (print text)

pub theorem print_line_write_tree
      (text : String)
    : Equal
        (IO (Result IOError Unit))
        (printLine text)
        (write Stdout (console_line_payload text)) =
  console_tree_refl (IO (Result IOError Unit)) (printLine text)

pub theorem eprint_write_tree
      (text : String)
    : Equal (IO (Result IOError Unit)) (eprint text) (write Stderr (bytes_encode text)) =
  console_tree_refl (IO (Result IOError Unit)) (eprint text)

pub theorem eprint_line_write_tree
      (text : String)
    : Equal
        (IO (Result IOError Unit))
        (eprintLine text)
        (write Stderr (console_line_payload text)) =
  console_tree_refl (IO (Result IOError Unit)) (eprintLine text)

fn console_line_payload (text : String) : Bytes =
  bytes_concat
    (bytes_encode text)
    (bytes_encode (list_char_to_string (Cons Char (10 : Int) (Nil Char))))

theorem console_tree_refl (a : Type) (x : a) : Equal a x x = Refl
```

`print_write_tree`, `print_line_write_tree`, `eprint_write_tree`, and
`eprint_line_write_tree` state that each helper is exactly one `write` to its
stream with its exact payload, so it returns `write`'s `Result IOError Unit`
unchanged. `write` is transparent, so each law also converts to the one-node
interaction tree `Vis ConsoleOp console_resp (Result IOError Unit) (Write
stream payload) (λr. Ret … r)`, and a client may state that tree and use the
law directly. The newline is encoded explicitly as character 10; these laws
do not prove facts about the primitive encoder's output bytes.

The helpers preserve `write`'s total `Result IOError Unit`; broken pipes remain
named values visible to callers rather than host exceptions.
