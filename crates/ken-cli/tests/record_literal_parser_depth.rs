use std::process::Command;

const DEPTH_CHILD: &str = "KEN_RECORD_LITERAL_PARSER_DEPTH_CHILD";
const NESTED_MATCH_DEPTH: usize = 31;

fn nested_match_expression(depth: usize) -> String {
    (0..depth).fold("0".to_owned(), |body, _| {
        format!("match 0 {{ _ => {body} }}")
    })
}

#[test]
fn record_literal_parser_retains_nested_match_depth() {
    if std::env::var_os(DEPTH_CHILD).is_some() {
        ken_elaborator::parser::parse_expr(&nested_match_expression(NESTED_MATCH_DEPTH))
            .expect("nested match expression must parse");
        return;
    }

    let status = Command::new(std::env::current_exe().expect("test executable path"))
        .args([
            "record_literal_parser_retains_nested_match_depth",
            "--exact",
        ])
        .env(DEPTH_CHILD, "1")
        .status()
        .expect("depth-control child must start");

    assert!(status.success(), "depth-control child failed: {status}");
}
