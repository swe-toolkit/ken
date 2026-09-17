use ken_elaborator::parser::parse_decls;
#[test]
fn conformance_rows() {
    // Row at seed-reserved-infix-names.md:126 (expect-negative), mine.
    for s in ["const grouped_op : Nat = <+>", "const grouped_op : Nat = \u{2264}"] {
        println!("bare      {:8} {s}", if parse_decls(s).is_err() {"REJECTS"} else {"PARSES"});
    }
    for s in ["const grouped_op : Nat = (<+>)", "const grouped_op : Nat = (\u{2264})",
              "const applied_op : Nat = <+> Zero", "const applied_op : Nat = \u{2264} Zero"] {
        println!("positive  {:8} {s}", if parse_decls(s).is_err() {"REJECTS"} else {"PARSES"});
    }
    // Row at :171 (expect-leading-if), the MERGED predecessor's.
    for s in ["const k : Nat = keep if true then Zero else Zero",
              "const k : Nat = keep (if true then Zero else Zero)"] {
        println!("if-row    {:8} {s}", if parse_decls(s).is_err() {"REJECTS"} else {"PARSES"});
    }
}
