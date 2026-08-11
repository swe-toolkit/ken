use ken_elaborator::lexer::{Lexer, Token};
use num_bigint::BigInt;

fn tokens(src: &str) -> Vec<Token> {
    Lexer::lex(src).expect("lex").into_iter().map(|(t, _)| t).collect()
}

#[test]
fn separators_and_radix_integers_have_boundaries() {
    assert_eq!(tokens("1_000"), vec![Token::Nat(1000), Token::Eof]);
    assert_eq!(tokens("0xFF"), vec![Token::Nat(255), Token::Eof]);
    assert_eq!(tokens("0b1010"), vec![Token::Nat(10), Token::Eof]);
    assert_eq!(tokens("0o17"), vec![Token::Nat(15), Token::Eof]);
    let wide = BigInt::parse_bytes(b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF", 16).unwrap();
    assert_eq!(tokens("0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"), vec![Token::IntLit(wide), Token::Eof]);
    for bad in ["0xGG", "1_.5", "1._5", "1__0", "1_"] {
        assert!(Lexer::lex(bad).is_err(), "{bad}");
    }
}

#[test]
fn exponent_values_are_not_zeroed() {
    assert_eq!(tokens("3.14e5"), vec![Token::FloatLit(314000.0), Token::Eof]);
    assert_eq!(tokens("1e-9"), vec![Token::FloatLit(1e-9), Token::Eof]);
    assert_eq!(tokens("3.14E-2"), vec![Token::FloatLit(0.0314), Token::Eof]);
    assert_eq!(tokens("1e1_0"), vec![Token::FloatLit(1e10), Token::Eof]);
    assert_eq!(tokens("1e+1_0"), vec![Token::FloatLit(1e10), Token::Eof]);
    for bad in ["1e", "1e+", "1e-", "1e1_", "1e1__0", "1e+_1", "1e-_1"] {
        assert!(Lexer::lex(bad).is_err(), "{bad}");
    }
}

#[test]
fn decimal_fraction_separators_do_not_change_exponent() {
    assert_eq!(tokens("1.0_0d"), vec![Token::DecimalLit(BigInt::from(100), -2), Token::Eof]);
}
