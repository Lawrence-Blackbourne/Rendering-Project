//! This is a simple XML parser designed for parsing vk.xml for the build script.
//! Notably, this cannot handle comments.
//! It also cannot handle & codes, treating them as text.
//! These restrictions are due to there being no traditional comments in vk.xml (instead using a
//! comment element with a text block, or a comment attribute), and there being no & codes in the
//! sections I care about right now.
//! It also cannot handle CDATA or using single quotes to encode text, as these are also not used in
//! vk.xml.
//!
//! It is worth noting that this code IS somewhat optimized.
//! Practically, this is completely pointless - his code is only used by the build script, and so
//! spending multiple days re-writing a tokeniser to go from taking ~100ms to tokenise vk.xml to
//! currently 32ms is silly.
//! However, this whole project is a learning tool for me, and so taking on the task of optimizing
//! this code is purely for educational purposes.
//! Having said that, the code is not highly optimized at all - again, learning experience and so
//! there are almost certainly efficiency issues.
//! The main focus of the optimizations has been dealing with unnecessary string allocations.

mod format_parser;
mod parser;
mod tokeniser;

use parser::ParsedXml;
use std::fs::File;
use std::io;
use std::path::Path;

const VULKAN_XML_PATH: &str = "vulkan_XML/vk.xml";

fn get_parsed_xml_file(path: &Path) -> Result<ParsedXml<File>, ParserError> {
    let tokenised_xml = tokeniser::tokenise_xml_file(path)?;

    parser::parse_xml(tokenised_xml)
}

/// This function is only here to stop the many many compiler warnings about the code being unused
/// TODO remove once an actual use is implemented
pub fn temp() {
    let mut xml = get_parsed_xml_file(Path::new(VULKAN_XML_PATH)).unwrap();
    assert_ne!(xml.next().unwrap().unwrap(), parser::Item::EndFile);
    loop {
        match xml.next().unwrap().unwrap() {
            parser::Item::EndFile => break,
            _ => (),
        }
    }
    assert!(xml.next().is_none())
}

#[cfg(feature = "bench")]
pub fn benchmark_tokeniser() {
    let mut tokens = tokeniser::tokenise_xml_file(Path::new(VULKAN_XML_PATH)).unwrap();
    loop {
        let token = tokens.next();
        match token {
            None => break,
            Some(_) => (),
        }
    }
    assert!(tokens.next().is_none())
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use parser::Item;

    #[test]
    fn can_parse_xml() {
        let mut xml = get_parsed_xml_file(&Path::new("vulkan_XML/vk.xml")).unwrap();
        assert_ne!(xml.next().unwrap().unwrap(), Item::EndFile);
        loop {
            match xml.next().unwrap().unwrap() {
                Item::EndFile => break,
                _ => (),
            }
        }
        assert!(xml.next().is_none())
    }
}

/// Describes what error occurred during the XML parsing
#[derive(Debug)]
pub(crate) enum ParserError {
    FileCutShortAbruptlyDuringXMLDeclaration,
    FileCutShortAbruptlyDuringTag,
    NoRootElement,
    MultipleRootElements,
    ElementNotClosed,
    ElementsClosedOutOfOrder {
        correct: String,
        found: String,
    },
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
            ParserError::FileCutShortAbruptlyDuringXMLDeclaration =>
                write!(f, "file cut short during the XML declaration"),
            ParserError::FileCutShortAbruptlyDuringTag =>
                write!(f, "file cut short during a tag"),
            ParserError::NoRootElement =>
                write!(f, "file does not contain a root element"),
            ParserError::MultipleRootElements =>
                write!(f, "file contains multiple root elements"),
            ParserError::ElementNotClosed =>
                write!(f, "wn element was not closed"),
            ParserError::ElementsClosedOutOfOrder{found, correct} =>
                write!(
                    f,
                    "closure for element {found} found when closure for element {correct} expected"
                ),
            ParserError::ElementClosedAfterRootElementClosed(e) =>
                write!(f, "closure for element {e} found after closure for root element found"),
            ParserError::InvalidToken(t) =>
                write!(f, "an invalid {t} token was found where it should not have been"),
            ParserError::IoError(e) =>
                write!(f, "an io error occurred: \"{e}\"")
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
            ParserError::IoError(e) => e.source(),
        }
    }
}