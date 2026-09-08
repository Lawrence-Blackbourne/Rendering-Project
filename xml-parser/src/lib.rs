mod format_parser;
mod xml_parser;

use std::path::Path;
use xml_parser::Item;

const VULKAN_XML_PATH: &str = "vulkan_XML/vk.xml";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParsedXml {
    parsed_formats: format_parser::ParsedFormats
}

fn parse_xml(text: impl AsRef<Path>) -> ParsedXml {
    let mut xml = xml_parser::get_parsed_xml_file(VULKAN_XML_PATH).unwrap();
    xml.next();
    loop {
        match xml.next() {
            Some(Ok(Item::Element{
                name,
                ..
            })) if name == String::from("formats") => {
                return ParsedXml{
                    parsed_formats: format_parser::parse_formats(&mut xml)
                };
            }
            Some(Ok(Item::Element {..})) => xml.skip_current_element().unwrap(),
            Some(Ok(Item::Text(_))) => (),
            Some(Ok(Item::EndCurrentElement)) => (),
            Some(Err(e)) => panic!("{e:?}"),
            None => panic!("The XML file ended without the required sections for the build script")
        }
        if let Some(Ok(xml_parser::Item::Element{
            name,
            ..
        })) = xml.next() && name == String::from("formats") {
            return ParsedXml{
                parsed_formats: format_parser::parse_formats(&mut xml)
            };
        }
    }
    panic!()
}