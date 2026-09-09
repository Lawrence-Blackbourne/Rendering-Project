use super::xml_parser::{ParsedXml, Item};

use std::io::Read;
use ash::vk;

pub type ParsedFormats = Vec<ParsedFormat>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedFormat {}

#[derive(Clone, Debug, Eq, PartialEq)]
struct FormatAttributeInfo {
    name: String,
}

pub fn parse_formats<T: Read>(xml: &mut ParsedXml<T>) -> ParsedFormats {
    let formats = Vec::new();
    loop {
        match xml.next() {
            Some(Ok(Item::Element {
                name,
                attributes,
            })) if name == String::from("format") => {
                let attribute_info = get_format_attribute_info(attributes);
                panic!()
            },
            Some(Ok(Item::Element {..})) =>
                panic!("Unexpected element found in the format section"),
            Some(Ok(Item::Text(_))) => (),
            Some(Ok(Item::EndCurrentElement)) => break,
            Some(Err(e)) => panic!("{e:?}"),
            None => panic!("XML ended suddenly"),
        }
    }
    formats
}

fn get_format_attribute_info(attributes: Vec<(String, String)>) -> FormatAttributeInfo {
    todo!()
}