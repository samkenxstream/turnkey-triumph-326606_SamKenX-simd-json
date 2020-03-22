use std::fmt;

/// Error types encountered while parsing
#[derive(Debug)]
pub enum ErrorType {
    /// The key of a map isn't a string
    BadKeyType,
    /// The data ended early
    EarlyEnd,
    /// Expected an array
    ExpectedArray,
    /// Expected a `,` in an array
    ExpectedArrayComma,
    /// expected an boolean
    ExpectedBoolean,
    /// Expected an enum
    ExpectedEnum,
    /// Expected a float
    ExpectedFloat,
    /// Expected an integer
    ExpectedInteger,
    /// Expected a map
    ExpectedMap,
    /// Expected an `:` to seperate key and value in an object
    ExpectedObjectColon,
    /// Expected a `,` in an object
    ExpectedMapComma,
    /// Expected the object to end
    ExpectedMapEnd,
    /// Expected a null
    ExpectedNull,
    /// Expected a number
    ExpectedNumber,
    /// Expected a signed number
    ExpectedSigned,
    /// Expected a string
    ExpectedString,
    /// Expected an unsigned number
    ExpectedUnsigned,
    /// Internal error
    InternalError,
    /// Invalid escape sequence
    InvalidEscape,
    /// Invalid exponent in a floating point number
    InvalidExponent,
    /// Invalid number
    InvalidNumber,
    /// Inbalid UTF8 codepoint
    InvalidUTF8,
    /// Invalid Unicode escape sequence
    InvalidUnicodeEscape,
    /// Inbalid Unicode codepoint
    InvlaidUnicodeCodepoint,
    /// Object Key isn't a string
    KeyMustBeAString,
    /// Non structural character
    NoStructure,
    /// Parser Erropr
    Parser,
    /// Early End Of File
    EOF,
    /// Generic serde error
    Serde(String),
    /// Generic syntax error
    Syntax,
    /// Training characters
    TrailingCharacters,
    /// Unexpected character
    UnexpectedCharacter,
    /// Unexpected end
    UnexpectedEnd,
    /// Unterminated string
    UnterminatedString,
    /// Expected Array elements
    ExpectedArrayContent,
    /// Expected Object elements
    ExpectedObjectContent,
    /// Expected Object Key
    ExpectedObjectKey,
    /// Overflow of a limited buffer
    Overflow,
    /// IO error
    IO(std::io::Error),
}

#[cfg_attr(tarpaulin, skip)]
impl PartialEq for ErrorType {
    #[must_use]
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::IO(_), Self::IO(_))
            | (Self::BadKeyType, Self::BadKeyType)
            | (Self::EarlyEnd, Self::EarlyEnd)
            | (Self::ExpectedArray, Self::ExpectedArray)
            | (Self::ExpectedArrayComma, Self::ExpectedArrayComma)
            | (Self::ExpectedBoolean, Self::ExpectedBoolean)
            | (Self::ExpectedEnum, Self::ExpectedEnum)
            | (Self::ExpectedFloat, Self::ExpectedFloat)
            | (Self::ExpectedInteger, Self::ExpectedInteger)
            | (Self::ExpectedMap, Self::ExpectedMap)
            | (Self::ExpectedObjectColon, Self::ExpectedObjectColon)
            | (Self::ExpectedMapComma, Self::ExpectedMapComma)
            | (Self::ExpectedMapEnd, Self::ExpectedMapEnd)
            | (Self::ExpectedNull, Self::ExpectedNull)
            | (Self::ExpectedNumber, Self::ExpectedNumber)
            | (Self::ExpectedSigned, Self::ExpectedSigned)
            | (Self::ExpectedString, Self::ExpectedString)
            | (Self::ExpectedUnsigned, Self::ExpectedUnsigned)
            | (Self::InternalError, Self::InternalError)
            | (Self::InvalidEscape, Self::InvalidEscape)
            | (Self::InvalidExponent, Self::InvalidExponent)
            | (Self::InvalidNumber, Self::InvalidNumber)
            | (Self::InvalidUTF8, Self::InvalidUTF8)
            | (Self::InvalidUnicodeEscape, Self::InvalidUnicodeEscape)
            | (Self::InvlaidUnicodeCodepoint, Self::InvlaidUnicodeCodepoint)
            | (Self::KeyMustBeAString, Self::KeyMustBeAString)
            | (Self::NoStructure, Self::NoStructure)
            | (Self::Parser, Self::Parser)
            | (Self::EOF, Self::EOF)
            | (Self::Syntax, Self::Syntax)
            | (Self::TrailingCharacters, Self::TrailingCharacters)
            | (Self::UnexpectedCharacter, Self::UnexpectedCharacter)
            | (Self::UnexpectedEnd, Self::UnexpectedEnd)
            | (Self::UnterminatedString, Self::UnterminatedString)
            | (Self::ExpectedArrayContent, Self::ExpectedArrayContent)
            | (Self::ExpectedObjectContent, Self::ExpectedObjectContent)
            | (Self::ExpectedObjectKey, Self::ExpectedObjectKey)
            | (Self::Overflow, Self::Overflow) => true,
            (Self::Serde(s1), Self::Serde(s2)) => s1 == s2,
            _ => false,
        }
    }
}
/// Parser error
#[derive(Debug, PartialEq)]
pub struct Error {
    /// Byte index it was encountered at
    index: usize,
    /// Current character
    character: char,
    /// Tyep of error
    error: ErrorType,
}

impl Error {
    pub(crate) fn new(index: usize, character: char, error: ErrorType) -> Self {
        Self {
            index,
            character,
            error,
        }
    }
    pub(crate) fn generic(t: ErrorType) -> Self {
        Self {
            index: 0,
            character: '💩', //this is the poop emoji
            error: t,
        }
    }
}

#[cfg_attr(tarpaulin, skip)]
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} at character {} ('{}')",
            self.error, self.index, self.character
        )
    }
}

impl From<Error> for std::io::Error {
    fn from(e: Error) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, e)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn fmt() {
        let e = Error::generic(ErrorType::InternalError);
        assert_eq!(
            format!("{}", e),
            "InternalError at character 0 ('\u{1f4a9}')"
        )
    }
}
