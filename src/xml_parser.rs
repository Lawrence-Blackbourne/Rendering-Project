//! This is a simple XML parser designed for parsing vk.xml for the build script.
//! Notably, this cannot handle comments.
//! It also cannot handle & codes, treating them as text.

mod format_parser;
mod tokenizer;
mod parser;

use std::{fs,
          path::Path};
use parser::{ParsedXml, ParserError};

fn get_parsed_xml() -> () {
    let xml_path = Path::new("vulkan_XML/vk.xml");
    let xml = String::from_utf8(fs::read(xml_path).unwrap()).unwrap();

    let tokenised_xml = tokenizer::tokenize_xml(xml.as_str());

    parser::parse_xml(tokenised_xml);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_xml() {
        get_parsed_xml();
    }
}