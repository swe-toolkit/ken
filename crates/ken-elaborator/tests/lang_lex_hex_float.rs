use ken_elaborator::lexer::{Lexer, Token};

fn t(s: &str) -> Token { Lexer::lex(s).unwrap().into_iter().next().unwrap().0 }

#[test]
fn hex_float_values_and_boundaries() {
    assert_eq!(t("0x1p-3"), Token::FloatLit(0.125));
    assert_eq!(t("0x1.8p3"), Token::FloatLit(12.0));
    assert_eq!(t("0x1p0"), Token::FloatLit(1.0));
    assert_eq!(t("0x100000000000008p-56"), Token::FloatLit(1.0));
    assert_eq!(t("0x100000000000009p-56"), Token::FloatLit(1.0000000000000002));
    assert_eq!(t("0x1p-1022"), Token::FloatLit(f64::from_bits(0x0010_0000_0000_0000)));
    assert_eq!(t("0x1p-1023"), Token::FloatLit(f64::from_bits(0x0008_0000_0000_0000)));
    assert_eq!(t("0x1p-1075"), Token::FloatLit(0.0));
    assert_eq!(t("0x1p+1_0"), Token::FloatLit(1024.0));
    assert!(Lexer::lex("0x1._8p0").is_err());
    let huge = format!("0x1{}p-1024", "0".repeat(256));
    assert_eq!(t(&huge), Token::FloatLit(1.0));
    for bad in ["0x1.8", "0x1p", "0x1p+_3", "0x1p-_3", "0x1p1__0", "0xG.p1"] {
        assert!(Lexer::lex(bad).is_err(), "{bad}");
    }
    assert!(matches!(Lexer::lex("0xFF + p").unwrap()[0].0, Token::Nat(255)));
    assert!(matches!(Lexer::lex("0xFF:p").unwrap()[0].0, Token::Nat(255)));
    assert!(matches!(Lexer::lex("0xFF:x.1").unwrap()[0].0, Token::Nat(255)));
    assert!(matches!(Lexer::lex("0b10 + p").unwrap()[0].0, Token::Nat(2)));
    assert!(matches!(Lexer::lex("0o7 + p").unwrap()[0].0, Token::Nat(7)));
}
