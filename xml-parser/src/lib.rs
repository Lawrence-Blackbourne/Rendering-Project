mod format_parser;
mod generated_enums;
pub mod text_parser;

use std::path::Path;
use text_parser::Item;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedXml {
    formats: Vec<format_parser::ParsedFormat>,
}

impl ParsedXml {
    fn new() -> ParsedXml {
        ParsedXml {
            formats: Vec::new(),
        }
    }
}

pub fn parse_xml(text: impl AsRef<Path>) -> ParsedXml {
    let mut xml = text_parser::get_parsed_xml_file(text).unwrap();
    loop {
        match xml.next() {
            Some(Ok(Item::Text(_))) => (),
            Some(Ok(Item::Element { name, .. })) if name == String::from("registry") => {
                break;
            }
            item => panic!("Invalid {item:?} item found"),
        }
    }
    let mut parsed_xml = ParsedXml::new();
    loop {
        match xml.next() {
            Some(Ok(Item::Element { name, .. })) if name == String::from("formats") => {
                format_parser::parse_formats(&mut xml, &mut parsed_xml.formats);
            }
            Some(Ok(Item::Element { .. })) => xml.skip_current_element().unwrap(),
            Some(Ok(Item::Text(_))) => (),
            Some(Ok(Item::EndCurrentElement)) => break,
            Some(Err(e)) => panic!("{e:?}"),
            None => panic!("XML stream ended suddenly"),
        }
    }
    parsed_xml
}

#[cfg(test)]
const TEST_XML_PATH: &str = "vulkan_XML/vk.xml";

#[cfg(feature = "bench")]
const BENCH_XML_PATH: &str = "vulkan_XML/vk.xml";
