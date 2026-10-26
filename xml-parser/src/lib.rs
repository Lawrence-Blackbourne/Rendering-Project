mod format_parser;
mod generator_structures;
pub mod text_parser;

use crate::text_parser::ParsedXml;
use std::path::Path;
use text_parser::Item;

#[derive(Debug)]
pub struct ParsedDetails {
    formats: format_parser::GenerationData,
}

impl ParsedDetails {
    fn new() -> ParsedDetails {
        ParsedDetails {
            formats: format_parser::GenerationData::new(),
        }
    }
}

pub fn parse_xml(text: impl AsRef<Path>) -> ParsedDetails {
    let mut xml = text_parser::get_parsed_xml_file(text).unwrap();
    loop {
        match xml.next() {
            Some(Ok(Item::Text(_))) => (),
            Some(Ok(Item::Element { name, .. })) if name.as_str() == "registry" => {
                break;
            }
            item => panic!("Invalid {item:?} item found"),
        }
    }
    let mut parsed_xml = ParsedDetails::new();
    loop {
        match xml.next() {
            Some(Ok(Item::Element { name, .. })) if name.as_str() == "formats" => {
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

/// Here to silence compiler warnings about not yet using the generate_text() options.
pub fn temp() {
    format_parser::generate_text(format_parser::GenerationData::new());
}

#[cfg(test)]
const TEST_XML_PATH: &str = "vulkan_XML/vk.xml";

#[cfg(feature = "bench")]
const BENCH_XML_PATH: &str = "vulkan_XML/vk.xml";