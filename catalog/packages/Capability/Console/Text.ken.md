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

fn console_line_payload (text : String) : Bytes =
  bytes_concat
    (bytes_encode text)
    (bytes_encode (list_char_to_string (Cons Char (10 : Int) (Nil Char))))

fn console_write_tree (stream : Stream) (payload : Bytes) : IO (Result IOError Unit) =
  Vis
    ConsoleOp
    console_resp
    (Result IOError Unit)
    (Write stream payload)
    (λr. Ret ConsoleOp console_resp (Result IOError Unit) r)

theorem console_tree_refl (a : Type) (x : a) : Equal a x x = Refl

pub theorem print_write_tree
      (text : String)
    : Equal
        (IO (Result IOError Unit))
        (print text)
        (console_write_tree Stdout (bytes_encode text)) =
  console_tree_refl (IO (Result IOError Unit)) (print text)

pub theorem print_line_write_tree
      (text : String)
    : Equal
        (IO (Result IOError Unit))
        (printLine text)
        (console_write_tree Stdout (console_line_payload text)) =
  console_tree_refl (IO (Result IOError Unit)) (printLine text)

pub theorem eprint_write_tree
      (text : String)
    : Equal
        (IO (Result IOError Unit))
        (eprint text)
        (console_write_tree Stderr (bytes_encode text)) =
  console_tree_refl (IO (Result IOError Unit)) (eprint text)

pub theorem eprint_line_write_tree
      (text : String)
    : Equal
        (IO (Result IOError Unit))
        (eprintLine text)
        (console_write_tree Stderr (console_line_payload text)) =
  console_tree_refl (IO (Result IOError Unit)) (eprintLine text)
```

`print_write_tree`, `print_line_write_tree`, `eprint_write_tree`, and
`eprint_line_write_tree` expose each helper's single `Write`, chosen stream,
payload, and unchanged response in a checked interaction tree. The newline is
encoded explicitly as character 10; these laws do not prove facts about the
primitive encoder's output bytes.

The helpers preserve `write`'s total `Result IOError Unit`; broken pipes remain
named values visible to callers rather than host exceptions.
