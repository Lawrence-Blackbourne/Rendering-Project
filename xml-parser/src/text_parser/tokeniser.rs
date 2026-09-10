//! The tokeniser that turns the text file into a stream of tokens.
//!
//! A note on performance - using Word(String) is inefficient, causing many heap allocations.
//! However, this parser is benchmarked at parsing the entirety of vk.xml in 24.042ms.
//! This is plenty performant for the use case (literally just the build script).

use super::ParserError;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::iter::FusedIterator;
use std::path::Path;

pub(super) fn tokenise_xml_file(
    file_path: impl AsRef<Path>,
) -> Result<TokenisedXml<File>, ParserError> {
    Ok(TokenisedXml::new(File::open(file_path)?))
}

/// The functions of the tokeniser that this creates use blocking functions for reading in the data.
/// The data is read in using lines, so an attacker could stream in data with no newlines and no EOF
/// to hang the tokeniser.
/// This is obviously fine for this use case, as we are reading in a file already stored on the
/// computer, and this code is in no way related to security anyway.
#[derive(Debug)]
pub(super) struct TokenisedXml<T: Read> {
    // The source of text that the buffer reads from
    text: BufReader<T>,

    // Stores if we have reached the end of the tokens.
    finished: bool,

    // The current section of text we are working from.
    buffer: String,

    // How far into the buffer we are currently looking.
    byte_offset: usize,
}

impl<T: Read> TokenisedXml<T> {
    /// Creates a new instance of the tokeniser from the provided text.
    pub(super) fn new(text: T) -> Self {
        let text = BufReader::new(text);
        Self {
            text,
            finished: false,
            buffer: String::new(),
            byte_offset: 0,
        }
    }
}

impl<T: Read> Iterator for TokenisedXml<T> {
    type Item = Result<Token, ParserError>;

    /// Returns the next token available, returning None if we have reached the end of the stream or
    /// `Some(Err(e))` if we have run into some error.
    /// Acts as a layer on top of `get_next_token` to ensure that it is not called again once it
    /// gives a bad response.
    fn next(&mut self) -> Option<Self::Item> {
        if !self.finished {
            let result = self.get_next_token();
            self.finished = matches!(result, Some(Err(_)) | None);
            result
        } else {
            None
        }
    }
}

impl<T: Read> FusedIterator for TokenisedXml<T> {}

impl<T: Read> TokenisedXml<T> {
    /// Returns the next token in the sequence.
    /// The behaviour after a `Some(Err(e))` or `None` result is returned is unspecified.
    /// For this reason, this should be only used through `next()`.
    fn get_next_token(&mut self) -> Option<Result<Token, ParserError>> {
        // This function cannot handle tokens that span across `\n` characters, but no token can do
        // that so this is fine.
        match self.update_line() {
            Ok(true) => (),
            Ok(false) => return None,
            Err(e) => return Some(Err(e.into())),
        }

        // The following unwrap is safe because we ran self.update_line above, and we only continue
        // to this point if we got Ok(true).
        let mut chars = self.buffer[self.byte_offset..].chars();
        let (byte_length, token) = match chars.next().unwrap() {
            '<' => (1, Token::StartTag),
            '>' => (1, Token::EndTag),
            '=' => (1, Token::Equals),
            '"' => (1, Token::QuotationMark),
            '?' => (1, Token::QuestionMark),
            '/' => (1, Token::Slash),
            char if char.is_whitespace() => (char.len_utf8(), Token::Whitespace(char)),
            char => {
                let mut word_length = char.len_utf8();
                loop {
                    match chars.next() {
                        Some('<' | '>' | '=' | '"' | '?' | '/') => break,
                        Some(char) if char.is_whitespace() => break,
                        Some(char) => word_length += char.len_utf8(),
                        None => break,
                    }
                }
                (
                    word_length,
                    Token::Word(String::from(
                        &self.buffer[self.byte_offset..self.byte_offset + word_length],
                    )),
                )
            }
        };
        self.byte_offset += byte_length;
        Some(Ok(token))
    }

    /// A value of `Ok(true)` means that there is definitely at least one character to read.
    /// A value of `Ok(false)` means that we have reached the end of the file and there is no more
    /// data to read.
    fn update_line(&mut self) -> Result<bool, io::Error> {
        if self.byte_offset >= self.buffer.len() {
            self.get_new_line()
        } else {
            Ok(true)
        }
    }

    /// A value of `Ok(false)` means that we have reached the end of the file, and there is no new
    /// line
    fn get_new_line(&mut self) -> Result<bool, io::Error> {
        self.buffer.clear();
        self.byte_offset = 0;
        match self.text.read_line(&mut self.buffer) {
            Ok(0) => Ok(false),
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        }
    }
}

/// Describes a single token found in the XML stream.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    StartTag,
    EndTag,
    QuestionMark,
    Slash,
    Equals,
    QuotationMark,
    Word(String),
    Whitespace(char),
}

impl Token {
    pub fn append_to(self, str: &mut String) {
        match self {
            Token::StartTag => str.push('<'),
            Token::EndTag => str.push('>'),
            Token::QuestionMark => str.push('?'),
            Token::Slash => str.push('/'),
            Token::Equals => str.push('='),
            Token::QuotationMark => str.push('\"'),
            Token::Word(word) => str.push_str(&word),
            Token::Whitespace(char) => str.push(char),
        };
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::StartTag => write!(f, "Start tag"),
            Token::EndTag => write!(f, "End tag"),
            Token::QuestionMark => write!(f, "Question mark"),
            Token::Slash => write!(f, "Slash"),
            Token::Equals => write!(f, "Equals"),
            Token::QuotationMark => write!(f, "Quotation mark"),
            Token::Word(word) => write!(f, "Word({word})"),
            Token::Whitespace(c) => write!(f, "Whitespace character({c})"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{PARSER_IO_ERROR_TEST_STRING, TestReader};
    use super::*;

    #[test]
    fn can_tokenise_xml_file() {
        for _ in tokenise_xml_file(Path::new(crate::TEST_XML_PATH)).unwrap() {}
    }

    #[test]
    fn cannot_tokenise_invalid_file() {
        let _ = tokenise_xml_file(Path::new("")).unwrap_err();
    }

    #[test]
    fn test_individual_tokens() {
        let test_data = [
            ("<", Token::StartTag),
            (">", Token::EndTag),
            ("?", Token::QuestionMark),
            ("/", Token::Slash),
            ("=", Token::Equals),
            ("\"", Token::QuotationMark),
            (" ", Token::Whitespace(' ')),
            ("\t", Token::Whitespace('\t')),
            ("\n", Token::Whitespace('\n')),
            ("a", Token::Word(String::from("a"))),
            ("abc", Token::Word(String::from("abc"))),
        ];
        for test in test_data {
            let mut xml = TokenisedXml::new(TestReader::new(test.0));
            assert_eq!(xml.next().unwrap().unwrap(), test.1);
            assert!(xml.next().is_none());
            assert!(xml.next().is_none());
        }
    }

    #[test]
    fn test_failures() {
        let test_data = [
            ("", Vec::new()),
            ("<", Vec::new()),
            ("<\n", vec![Token::StartTag, Token::Whitespace('\n')]),
            (
                "<? test?as<much>as=you\"possibly\ncan\t>",
                vec![
                    Token::StartTag,
                    Token::QuestionMark,
                    Token::Whitespace(' '),
                    Token::Word(String::from("test")),
                    Token::QuestionMark,
                    Token::Word(String::from("as")),
                    Token::StartTag,
                    Token::Word(String::from("much")),
                    Token::EndTag,
                    Token::Word(String::from("as")),
                    Token::Equals,
                    Token::Word(String::from("you")),
                    Token::QuotationMark,
                    Token::Word(String::from("possibly")),
                    Token::Whitespace('\n'),
                ],
            ),
        ];
        for test in test_data {
            let mut xml = TokenisedXml::new(TestReader::new(test.0).with_fail_on_end(true));
            for token in test.1 {
                assert_eq!(xml.next().unwrap().unwrap(), token);
            }
            assert_eq!(
                xml.next().unwrap().unwrap_err().to_string(),
                PARSER_IO_ERROR_TEST_STRING
            );
            assert!(xml.next().is_none());
        }
    }

    #[test]
    fn test_token_to_text() {
        let data = [
            ("test", Token::StartTag, "test<"),
            ("", Token::EndTag, ">"),
            (" ", Token::QuestionMark, " ?"),
            ("\n", Token::Slash, "\n/"),
            ("", Token::Equals, "="),
            ("", Token::QuotationMark, "\""),
            ("", Token::Whitespace(' '), " "),
            ("", Token::Word(String::new()), ""),
            ("", Token::Word(String::from("test")), "test"),
            ("test", Token::Word(String::new()), "test"),
            ("test", Token::Word(String::from(" more")), "test more"),
        ];
        for test in data {
            let mut str = String::from(test.0);
            test.1.append_to(&mut str);
            assert_eq!(str, test.2)
        }
    }

    #[test]
    fn test_token_debug() {
        let data = [
            (Token::StartTag, "Start tag"),
            (Token::EndTag, "End tag"),
            (Token::QuestionMark, "Question mark"),
            (Token::Slash, "Slash"),
            (Token::Equals, "Equals"),
            (Token::QuotationMark, "Quotation mark"),
            (Token::Word(String::from("")), "Word()"),
            (Token::Word(String::from("Test")), "Word(Test)"),
            (Token::Whitespace(' '), "Whitespace character( )"),
            (Token::Whitespace('猫'), "Whitespace character(猫)"),
        ];
        for test in data {
            assert_eq!(test.0.to_string(), test.1);
        }
    }
}
