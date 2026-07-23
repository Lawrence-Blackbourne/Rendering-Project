//! This is a simple XML parser designed for parsing vk.xml for the build script.
//! Notably, this cannot handle comments.
//! It also cannot handle & codes, treating them as text.

mod format_parser;
mod parser;
mod tokenizer;

use parser::{ParsedXml, ParserError};
use std::{ path::Path};

fn get_parsed_xml(path: &Path) -> Result<ParsedXml, ParserError> {
    let tokenised_xml = tokenizer::tokenize_xml(path)?;

    parser::parse_xml(tokenised_xml)
}

pub fn temp() {
    let mut xml = get_parsed_xml(&Path::new("vulkan_XML/vk.xml")).unwrap();
    assert_ne!(xml.next().unwrap().unwrap(), parser::Item::EndFile);
    loop {
        match xml.next().unwrap().unwrap() {
            parser::Item::EndFile => break,
            _ => (),
        }
    }
    assert!(xml.next().is_none())
}

#[cfg(test)]
mod tests {
    use super::*;
    use parser::Item;

    #[test]
    fn can_parse_xml() {
        let mut xml = get_parsed_xml(&Path::new("vulkan_XML/vk.xml")).unwrap();
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
