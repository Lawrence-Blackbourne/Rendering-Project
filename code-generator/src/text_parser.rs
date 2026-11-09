//! This is a simple XML parser designed for parsing vk.xml for the build script.
//! Notably, this cannot handle comments.
//! It also cannot handle & codes, treating them as text.
//! These restrictions are due to there being no traditional comments in vk.xml (instead using a
//! comment element with a text block, or a comment attribute), and there being no & codes in the
//! sections I care about right now.
//! It also cannot handle CDATA or using single quotes to encode text, as these are also not used in
//! vk.xml.
//! It also ignores the contents of the `<? ?>` block if it is present, and just skips until it
//! finds the ending `?` character.
//! If there is text after the end of the root element, it is also ignored
//! Another element after the end of the root element will cause error.
//! If the file ends or has an invalid token during a self-closing element, e.g. `<element/` or
//! `<element/<`, the opening of the element will be returned, followed by an error.
//! We also do not enforce whitespace between the end of one value and the start of another.
//!
//! Performance wise, we can tokenise and parse the entirety of vk.xml in a benchmarked 45.025ms,
//! which is more than fast enough for code used only in the build script
mod parser;
mod tokeniser;

use std::fs::File;
use std::io;
use std::path::Path;

pub(super) use parser::{Item, StreamedParsedXml};

pub trait ParsedXml: Iterator<Item = Result<Item, ParserError>> {
    /// Skips to the end of the element currently being looked at by the parser.
    /// If the parser is outside the root element, skips to the very end of the file.
    fn skip_current_element(&mut self) -> Result<(), ParserError> {
        let mut current_depth = 1;
        loop {
            match self.next() {
                Some(Ok(Item::Element { .. })) => current_depth += 1,
                Some(Ok(Item::EndCurrentElement)) => current_depth -= 1,
                Some(Ok(_)) => (),
                Some(Err(e)) => return Err(e),
                None => return Ok(()),
            }
            if current_depth == 0 {
                break;
            }
        }
        Ok(())
    }
}

/// Parses the XML found in a given file
pub fn get_parsed_xml_file(path: impl AsRef<Path>) -> Result<StreamedParsedXml<File>, ParserError> {
    parser::parse_xml_file(path)
}

/// Describes what error occurred during the XML parsing
#[derive(Debug)]
pub enum ParserError {
    TextCutShortAbruptlyDuringXMLDeclaration,
    TextCutShortAbruptlyDuringTag,
    NoRootElement,
    MultipleRootElements,
    ElementNotClosed,
    ElementsClosedOutOfOrder { correct: String, found: String },
    ElementClosedAfterRootElementClosed(String),
    InvalidToken(tokeniser::Token),
    IoError(io::Error),
}

impl From<io::Error> for ParserError {
    fn from(value: io::Error) -> Self {
        ParserError::IoError(value)
    }
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::TextCutShortAbruptlyDuringXMLDeclaration => {
                write!(f, "text cut short during the XML declaration")
            }
            ParserError::TextCutShortAbruptlyDuringTag => write!(f, "text cut short during a tag"),
            ParserError::NoRootElement => write!(f, "text does not contain a root element"),
            ParserError::MultipleRootElements => write!(f, "text contains multiple root elements"),
            ParserError::ElementNotClosed => write!(f, "an element was not closed"),
            ParserError::ElementsClosedOutOfOrder { found, correct } => write!(
                f,
                "closure for element \"{found}\" found when closure for element \"{correct}\" \
                expected"
            ),
            ParserError::ElementClosedAfterRootElementClosed(e) => write!(
                f,
                "closure for element \"{e}\" found after closure for root element found"
            ),
            ParserError::InvalidToken(t) => write!(f, "an invalid \"{t}\" token was found"),
            ParserError::IoError(_) => write!(f, "an io error occurred while reading the text"),
        }
    }
}

impl std::error::Error for ParserError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ParserError::TextCutShortAbruptlyDuringXMLDeclaration => None,
            ParserError::TextCutShortAbruptlyDuringTag => None,
            ParserError::NoRootElement => None,
            ParserError::MultipleRootElements => None,
            ParserError::ElementNotClosed => None,
            ParserError::ElementsClosedOutOfOrder { .. } => None,
            ParserError::ElementClosedAfterRootElementClosed(_) => None,
            ParserError::InvalidToken(_) => None,
            ParserError::IoError(e) => Some(e),
        }
    }
}

#[cfg(feature = "bench")]
#[doc(hidden)]
pub fn benchmark_tokeniser() {
    for _ in tokeniser::tokenise_xml_file(Path::new(crate::BENCH_XML_PATH)).unwrap() {}
}

#[cfg(feature = "bench")]
#[doc(hidden)]
pub fn benchmark_parser() {
    for _ in parser::parse_xml_file(Path::new(crate::BENCH_XML_PATH)).unwrap() {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text_parser::tokeniser::{Token, TokenisedXml};
    use io::Read;
    use std::error::Error;

    #[test]
    fn can_parse_xml() {
        let mut xml = get_parsed_xml_file(Path::new(crate::TEST_XML_PATH)).unwrap();
        loop {
            let val = xml.next();
            if val.is_none() {
                break;
            }
            let _ = val.unwrap().unwrap();
        }
        assert!(xml.next().is_none())
    }

    #[test]
    fn test_skip_current_element() {
        let mut xml = StreamedParsedXml::new(TokenisedXml::new(TestReader::new(
            "<root><middle1 name=\"value\"><inner1> text </inner1>\
            <inner2/></middle1><middle2/></root>",
        )));
        assert_eq!(
            xml.next().unwrap().unwrap(),
            Item::Element {
                name: String::from("root"),
                attributes: Vec::new(),
            },
        );
        assert_eq!(
            xml.next().unwrap().unwrap(),
            Item::Element {
                name: String::from("middle1"),
                attributes: vec![(String::from("name"), String::from("value"))],
            },
        );
        assert_eq!(xml.skip_current_element().unwrap(), ());
        assert_eq!(
            xml.next().unwrap().unwrap(),
            Item::Element {
                name: String::from("middle2"),
                attributes: Vec::new(),
            },
        );
        assert_eq!(xml.skip_current_element().unwrap(), ());
        assert_eq!(xml.next().unwrap().unwrap(), Item::EndCurrentElement);
        assert!(xml.next().is_none());
    }

    #[test]
    fn test_skipping_root_element() {
        let mut xml = StreamedParsedXml::new(TokenisedXml::new(TestReader::new(
            "<root><middle1 name=\"value\"><inner1> text </inner1>\
            <inner2/></middle1><middle2/></root>",
        )));
        assert_eq!(
            xml.next().unwrap().unwrap(),
            Item::Element {
                name: String::from("root"),
                attributes: Vec::new(),
            },
        );
        assert_eq!(xml.skip_current_element().unwrap(), ());
        assert!(xml.next().is_none());
    }

    #[test]
    fn test_skipping_outside_root_element() {
        let mut xml = StreamedParsedXml::new(TokenisedXml::new(TestReader::new("<root/>")));
        assert_eq!(xml.skip_current_element().unwrap(), ());
        assert!(xml.next().is_none());
        let mut xml = StreamedParsedXml::new(TokenisedXml::new(TestReader::new("<root/>")));
        assert_eq!(
            xml.next().unwrap().unwrap(),
            Item::Element {
                name: String::from("root"),
                attributes: Vec::new(),
            },
        );
        assert_eq!(xml.next().unwrap().unwrap(), Item::EndCurrentElement);
        assert_eq!(xml.skip_current_element().unwrap(), ());
        assert!(xml.next().is_none());
    }

    #[test]
    fn test_errors_in_skipping_current_element() {
        let mut xml = StreamedParsedXml::new(TokenisedXml::new(TestReader::new("<root><inner>")));
        assert_eq!(
            xml.next().unwrap().unwrap(),
            Item::Element {
                name: String::from("root"),
                attributes: Vec::new(),
            },
        );
        assert_eq!(
            xml.skip_current_element().unwrap_err().to_string(),
            ParserError::ElementNotClosed.to_string(),
        );
        assert!(xml.next().is_none());
    }

    #[test]
    fn test_parser_error_display() {
        let data = [
            (
                ParserError::TextCutShortAbruptlyDuringXMLDeclaration,
                "text cut short during the XML declaration",
            ),
            (
                ParserError::TextCutShortAbruptlyDuringTag,
                "text cut short during a tag",
            ),
            (
                ParserError::NoRootElement,
                "text does not contain a root element",
            ),
            (
                ParserError::MultipleRootElements,
                "text contains multiple root elements",
            ),
            (ParserError::ElementNotClosed, "an element was not closed"),
            (
                ParserError::ElementsClosedOutOfOrder {
                    found: String::from("found"),
                    correct: String::from("correct"),
                },
                "closure for element \"found\" found when closure for element \"correct\" expected",
            ),
            (
                ParserError::ElementClosedAfterRootElementClosed(String::from("test")),
                "closure for element \"test\" found after closure for root element found",
            ),
            (
                ParserError::InvalidToken(Token::Equals),
                "an invalid \"Equals\" token was found",
            ),
            (
                ParserError::from(io::Error::other("test")),
                PARSER_IO_ERROR_TEST_STRING,
            ),
        ];
        for test in data {
            assert_eq!(format!("{}", test.0), test.1);
        }
    }

    #[test]
    fn test_parser_error_source() {
        let no_source_data = [
            ParserError::TextCutShortAbruptlyDuringXMLDeclaration,
            ParserError::TextCutShortAbruptlyDuringTag,
            ParserError::NoRootElement,
            ParserError::MultipleRootElements,
            ParserError::ElementNotClosed,
            ParserError::ElementsClosedOutOfOrder {
                found: String::from("found"),
                correct: String::from("correct"),
            },
            ParserError::ElementClosedAfterRootElementClosed(String::from("test")),
            ParserError::InvalidToken(Token::Equals),
        ];
        for test in no_source_data {
            assert!(test.source().is_none())
        }
        assert_eq!(
            format!(
                "{}",
                ParserError::from(io::Error::other("test"))
                    .source()
                    .unwrap()
            ),
            format!("{}", io::Error::other("test")),
        )
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(crate) struct TestReader {
        data: io::Cursor<Vec<u8>>,
        fail: bool,
    }

    const TEST_READER_ERROR_STRING: &str = "Test";
    pub(crate) const PARSER_IO_ERROR_TEST_STRING: &str =
        "an io error occurred while reading the text";

    impl TestReader {
        pub(crate) fn new(value: &str) -> Self {
            Self {
                data: io::Cursor::new(value.as_bytes().to_vec()),
                fail: false,
            }
        }

        pub(crate) fn with_fail_on_end(mut self, value: bool) -> Self {
            self.fail = value;
            self
        }
    }

    impl Read for TestReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            match self.data.read(buf) {
                Ok(0) if self.fail => Err(io::Error::other(TEST_READER_ERROR_STRING)),
                other => other,
            }
        }
    }
}
