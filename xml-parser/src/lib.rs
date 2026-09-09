mod xml_parser;
mod format_parser;

use std::path::Path;
use xml_parser::Item;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParsedXml {
    parsed_formats: format_parser::ParsedFormats
}

fn parse_xml(text: impl AsRef<Path>) -> ParsedXml {
    let mut xml = xml_parser::get_parsed_xml_file(text).unwrap();
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
    }
    panic!()
}

#[cfg(test)]
mod tests {
    pub(crate) const TEST_XML_PATH: &str = "vulkan_XML/vk.xml";
}