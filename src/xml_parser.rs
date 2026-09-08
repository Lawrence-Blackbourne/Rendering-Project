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

use parser::ParsedXml;
use std::fs::File;
use std::io;
use std::path::Path;

const VULKAN_XML_PATH: &str = "vulkan_XML/vk.xml";

fn get_parsed_xml_file(path: impl AsRef<Path>) -> Result<ParsedXml<File>, ParserError> {
    parser::parse_xml_file(path)
}

/// This function is only here to stop the many many compiler warnings about the code being unused
/// TODO remove once an actual use is implemented
pub fn temp() {
    let mut xml = get_parsed_xml_file(Path::new(VULKAN_XML_PATH)).unwrap();
    while xml.next().is_some() {}
    assert!(xml.next().is_none())
}

/// Describes what error occurred during the XML parsing
#[derive(Debug)]
enum ParserError {
    FileCutShortAbruptlyDuringXMLDeclaration,
    FileCutShortAbruptlyDuringTag,
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
            ParserError::FileCutShortAbruptlyDuringXMLDeclaration => {
                write!(f, "file cut short during the XML declaration")
            }
            ParserError::FileCutShortAbruptlyDuringTag => write!(f, "file cut short during a tag"),
            ParserError::NoRootElement => write!(f, "file does not contain a root element"),
            ParserError::MultipleRootElements => write!(f, "file contains multiple root elements"),
            ParserError::ElementNotClosed => write!(f, "an element was not closed"),
            ParserError::ElementsClosedOutOfOrder { found, correct } => write!(
                f,
                "closure for element {found} found when closure for element {correct} expected"
            ),
            ParserError::ElementClosedAfterRootElementClosed(e) => write!(
                f,
                "closure for element {e} found after closure for root element found"
            ),
            ParserError::InvalidToken(t) => write!(
                f,
                "an invalid \"{t}\" token was found where it should not have been"
            ),
            ParserError::IoError(_) => write!(f, "an io error occurred while reading the XML"),
        }
    }
}

impl std::error::Error for ParserError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ParserError::FileCutShortAbruptlyDuringXMLDeclaration => None,
            ParserError::FileCutShortAbruptlyDuringTag => None,
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
pub fn benchmark_tokeniser() {
    for _ in tokeniser::tokenise_xml_file(Path::new(VULKAN_XML_PATH)).unwrap() {}
}

#[cfg(feature = "bench")]
pub fn benchmark_parser() {
    for _ in parser::parse_xml_file(Path::new(VULKAN_XML_PATH)).unwrap() {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use io::Read;

    #[test]
    fn can_parse_xml() {
        let mut xml = get_parsed_xml_file(Path::new(VULKAN_XML_PATH)).unwrap();
        loop {
            let val = xml.next();
            if val.is_none() {
                break;
            }
            let _ = val.unwrap().unwrap();
        }
        assert!(xml.next().is_none())
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub(super) struct TestReader {
        data: io::Cursor<Vec<u8>>,
        fail: bool,
    }

    const TEST_READER_ERROR_STRING: &str = "Test";
    pub(super) const PARSER_IO_ERROR_TEST_STRING: &str =
        "an io error occurred while reading the XML";

    impl TestReader {
        pub(super) fn new(value: &str) -> Self {
            Self {
                data: io::Cursor::new(value.as_bytes().to_vec()),
                fail: false,
            }
        }

        pub(super) fn with_fail_on_end(mut self, value: bool) -> Self {
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
