use ken_elaborator::lexer::{Lexer, Token};

fn t(s: &str) -> Token { Lexer::lex(s).unwrap().into_iter().next().unwrap().0 }

#[test]
fn hex_float_values_and_boundaries() {
    assert_eq!(t("0x1p-3"), Token::FloatLit(0.125));
    assert_eq!(t("0x1.8p3"), Token::FloatLit(12.0));
    assert_eq!(t("0x1p0"), Token::FloatLit(1.0));
    assert_eq!(t("0x10000000000001p-52"), Token::FloatLit(1.0000000000000002));
    assert_eq!(t("0x1p+1_0"), Token::FloatLit(1024.0));
    for bad in ["0x1.8", "0x1p", "0x1p+_3", "0x1p-_3", "0xG.p1"] {
        assert!(Lexer::lex(bad).is_err(), "{bad}");
    }
}
