use std::str::FromStr;

use chumsky::{error::Simple, primitive::just, text, Parser};
// use combine::{ParseError, Parser, Stream, many1, optional, parser::{char::{char, digit}}};
// use combine::

/// Splits a string at its' byte order mark (BOM)
pub fn split_bom(data: &str) -> (&str, &str) {
    if data.as_bytes().iter().take(3).eq([0xEF, 0xBB, 0xBF].iter()) {
        data.split_at(3)
    } else if data.as_bytes().iter().take(2).eq([0xFE, 0xFF].iter()) {
        data.split_at(2)
    } else {
        ("", data)
    }
}

#[test]
fn test_split_bom() {
    let bom1_vec = &[0xEF, 0xBB, 0xBF];
    let bom2_vec = &[0xFE, 0xFF];
    let bom1 = unsafe { ::std::str::from_utf8_unchecked(bom1_vec) };
    let bom2 = unsafe { ::std::str::from_utf8_unchecked(bom2_vec) };

    // Rust doesn't seem to let us create a BOM as str in a safe way.
    assert_eq!(
        split_bom(unsafe {
            ::std::str::from_utf8_unchecked(&[0xEF, 0xBB, 0xBF, b'a', b'b', b'c'])
        }),
        (bom1, "abc")
    );
    assert_eq!(
        split_bom(unsafe { ::std::str::from_utf8_unchecked(&[0xFE, 0xFF, b'd', b'e', b'g']) }),
        (bom2, "deg")
    );
    assert_eq!(split_bom("bla"), ("", "bla"));
    assert_eq!(split_bom(""), ("", ""));
}

/// Matches a positive or negative intger number.
pub fn number_i64() -> impl Parser<char, i64, Error = Simple<char>> {
    let signed = just("+").to(1).or(just("-").to(-1)).repeated();

    //
    signed
        .then(text::digits(10).from_str().unwrapped())
        .foldr(|a, b| a * b)
}
