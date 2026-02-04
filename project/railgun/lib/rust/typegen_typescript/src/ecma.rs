use unicode_properties::{GeneralCategory, UnicodeGeneralCategory};

use super::reserved::RESERVED_KEYWORDS;

/// Identifier ::
///     IdentifierName but not ReservedWord
///
/// IdentifierName ::
///     IdentifierStart
///     IdentifierName IdentifierPart
///
/// IdentifierStart ::
///     is_ident_start(...)
///
/// IdentifierPart ::
///     is_ident_part(...)
pub(super) fn is_ecma_ident(ident: &str) -> bool {
    if RESERVED_KEYWORDS.contains(&ident) {
        return false;
    }

    let mut chars = ident.chars();

    if chars.clone().count() == 0 {
        return false;
    }

    if !is_ecma_ident_start(chars.next().expect("We do a bounds check before this")) {
        return false;
    }

    chars.all(is_ecma_ident_part)
}

/// IdentifierStart ::
///     UnicodeLetter
///     $
///     _
///     \ UnicodeEscapeSequence
///
/// UnicodeLetter
///     any character in the Unicode categories:
///         “Uppercase letter (Lu)”
///         “Lowercase letter (Ll)”
///         “Titlecase letter (Lt)”
///         “Modifier letter (Lm)”
///         “Other letter (Lo)”
///         “Letter number (Nl)”
///
/// UnicodeEscapeSequence
///     u Hex4Digits
///     u{ CodePoint }
fn is_ecma_ident_start(item: char) -> bool {
    if item == '$' || item == '_' {
        return true;
    }

    match item.general_category() {
        GeneralCategory::UppercaseLetter
        | GeneralCategory::LowercaseLetter
        | GeneralCategory::TitlecaseLetter
        | GeneralCategory::ModifierLetter
        | GeneralCategory::OtherLetter
        | GeneralCategory::LetterNumber => true,
        GeneralCategory::NonspacingMark
        | GeneralCategory::SpacingMark
        | GeneralCategory::EnclosingMark
        | GeneralCategory::DecimalNumber
        | GeneralCategory::OtherNumber
        | GeneralCategory::ConnectorPunctuation
        | GeneralCategory::DashPunctuation
        | GeneralCategory::OpenPunctuation
        | GeneralCategory::ClosePunctuation
        | GeneralCategory::InitialPunctuation
        | GeneralCategory::FinalPunctuation
        | GeneralCategory::OtherPunctuation
        | GeneralCategory::MathSymbol
        | GeneralCategory::CurrencySymbol
        | GeneralCategory::ModifierSymbol
        | GeneralCategory::OtherSymbol
        | GeneralCategory::SpaceSeparator
        | GeneralCategory::LineSeparator
        | GeneralCategory::ParagraphSeparator
        | GeneralCategory::Control
        | GeneralCategory::Format
        | GeneralCategory::Surrogate
        | GeneralCategory::PrivateUse
        | GeneralCategory::Unassigned => false,
    }
}

/// IdentifierPart ::
///     IdentifierStart
///     UnicodeCombiningMark
///     UnicodeDigit
///     UnicodeConnectorPunctuation
///     \ UnicodeEscapeSequence
///
/// UnicodeCombiningMark
///     any character in the Unicode categories “Non-spacing mark (Mn)” or
/// “Combining spacing mark (Mc)”
///
/// UnicodeDigit
///     any character in the Unicode category “Decimal number (Nd)”
///
/// UnicodeConnectorPunctuation
///     any character in the Unicode category “Connector punctuation (Pc)”
///
/// UnicodeEscapeSequence
///     see 7.8.4.
///
/// HexDigit :: one of
///     0 1 2 3 4 5 6 7 8 9 a b c d e f A B C D E F
fn is_ecma_ident_part(item: char) -> bool {
    if is_ecma_ident_start(item) {
        return true;
    }

    match item.general_category() {
        GeneralCategory::NonspacingMark
        | GeneralCategory::SpacingMark
        | GeneralCategory::DecimalNumber
        | GeneralCategory::ConnectorPunctuation => true,
        GeneralCategory::UppercaseLetter
        | GeneralCategory::LowercaseLetter
        | GeneralCategory::TitlecaseLetter
        | GeneralCategory::ModifierLetter
        | GeneralCategory::OtherLetter
        | GeneralCategory::EnclosingMark
        | GeneralCategory::LetterNumber
        | GeneralCategory::OtherNumber
        | GeneralCategory::DashPunctuation
        | GeneralCategory::OpenPunctuation
        | GeneralCategory::ClosePunctuation
        | GeneralCategory::InitialPunctuation
        | GeneralCategory::FinalPunctuation
        | GeneralCategory::OtherPunctuation
        | GeneralCategory::MathSymbol
        | GeneralCategory::CurrencySymbol
        | GeneralCategory::ModifierSymbol
        | GeneralCategory::OtherSymbol
        | GeneralCategory::SpaceSeparator
        | GeneralCategory::LineSeparator
        | GeneralCategory::ParagraphSeparator
        | GeneralCategory::Control
        | GeneralCategory::Format
        | GeneralCategory::Surrogate
        | GeneralCategory::PrivateUse
        | GeneralCategory::Unassigned => false,
    }
}

#[cfg(test)]
mod test {
    use super::is_ecma_ident;

    #[test]
    fn reserved_ident() {
        assert!(!is_ecma_ident("while"));
    }

    #[test]
    fn unicode_ident() {
        assert!(is_ecma_ident("\u{3c0}"));
        assert!(is_ecma_ident("\u{ca0}_\u{ca0}"));
        assert!(is_ecma_ident("\u{10da}_\u{ca0}\u{76ca}\u{ca0}_\u{10da}"));
        assert!(is_ecma_ident("\u{3bb}"));
        assert!(is_ecma_ident("\u{a66c}\u{d7d}\u{2188}\u{2d31}"));
        assert!(is_ecma_ident("\u{1c79}"));
        assert!(is_ecma_ident("\u{3031}\u{3031}"));
        assert!(is_ecma_ident("price_9\u{336}9\u{336}_89"));
        assert!(is_ecma_ident("\u{2163}"));
        assert!(is_ecma_ident("\u{2164}"));
        assert!(is_ecma_ident(
            "H\u{36b}\u{306}\u{312}\u{310}\u{363}\u{30a}\u{304}\u{36f}\u{357}\u{34f}\u{335}\u{317}\u{33b}\u{330}\u{320}\u{32c}\u{35d}\u{345}E\u{334}\u{337}\u{32c}\u{34e}\u{331}\u{318}\u{347}\u{34d}\u{33e}\u{366}\u{34a}\u{352}\u{34a}\u{313}\u{313}\u{310}_\u{32b}\u{320}\u{331}\u{329}\u{32d}\u{324}\u{348}\u{311}\u{30e}\u{30b}\u{36e}\u{369}\u{312}\u{351}\u{33e}\u{34b}\u{358}\u{c7}\u{333}\u{355}\u{32f}\u{32d}\u{331}\u{332}\u{323}\u{320}\u{31c}\u{34b}\u{30d}O\u{334}\u{326}\u{317}\u{32f}\u{339}\u{33c}\u{36d}\u{310}\u{368}\u{30a}\u{308}\u{358}\u{360}M\u{336}\u{31d}\u{320}\u{32d}\u{32d}\u{324}\u{33b}\u{353}\u{351}\u{313}\u{30a}\u{363}\u{364}\u{30e}\u{35f}\u{360}E\u{322}\u{31e}\u{32e}\u{339}\u{34d}\u{31e}\u{333}\u{323}\u{363}\u{36a}\u{350}\u{308}T\u{321}\u{32f}\u{333}\u{32d}\u{31c}\u{320}\u{355}\u{34c}\u{308}\u{301}\u{33d}\u{33f}\u{364}\u{33f}\u{305}\u{311}\u{1e26}\u{331}\u{331}\u{33a}\u{330}\u{333}\u{339}\u{318}\u{330}\u{301}\u{30f}\u{36a}\u{302}\u{33d}\u{342}\u{300}\u{360}"
        ));
    }
}
