use super::text_parser::{Item, ParsedXml};

use std::io::Read;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedFormat {
    pub attributes: FormatAttributeInfo,
    pub components: Vec<FormatComponentInfo>,
    pub planes: Vec<FormatPlaneInfo>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormatAttributeInfo {
    name: String,
    class: String,
    block_size: String,
    texels_per_block: String,
    block_extent: (String, String, String),
    packed: Option<String>,
    compressed: Option<String>,
    chroma: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormatComponentInfo {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormatPlaneInfo {}

pub fn parse_formats<T: Read>(xml: &mut ParsedXml<T>, vec: &mut Vec<ParsedFormat>) {
    loop {
        match xml.next() {
            Some(Ok(Item::Element { name, attributes })) if name == String::from("format") => {
                let attribute_info = get_format_attribute_info(attributes);
                panic!()
            }
            Some(Ok(Item::Element { .. })) => {
                panic!("Unexpected element found in the format section")
            }
            Some(Ok(Item::Text(_))) => (),
            Some(Ok(Item::EndCurrentElement)) => break,
            Some(Err(e)) => panic!("{e:?}"),
            None => panic!("XML ended suddenly"),
        }
    }
}

fn get_format_attribute_info(attributes: Vec<(String, String)>) -> FormatAttributeInfo {
    let mut attribute_info = FormatAttributeInfo {
        name: String::new(),
        class: String::new(),
        block_size: String::new(),
        texels_per_block: String::new(),
        block_extent: (String::from("1"), String::from("1"), String::from("1")),
        packed: None,
        compressed: None,
        chroma: None,
    };
    for attribute in attributes {
        match attribute.0.as_str() {
            "name" => attribute_info.name = attribute.1,
            "class" => attribute_info.class = to_class(attribute.1),
            "blockSize" => attribute_info.block_size = attribute.1,
            "texels_per_block" => attribute_info.texels_per_block = attribute.1,
            "block_extent" => attribute_info.block_extent = to_block_extent(attribute.1),
            "packed" => attribute_info.packed = Some(attribute.1),
            "compressed" => attribute_info.compressed = Some(to_compression(attribute.1)),
            "chroma" => attribute_info.chroma = Some(to_chroma(attribute.1)),
            other => panic!(
                "Unexpected \"{other}\" attribute found with value \"{}\"",
                attribute.1
            ),
        }
    }
    attribute_info
}

fn to_class(txt: String) -> String {
    //TODO figure out how to sync with the relevant enums
    match txt {
        other => panic!("Unknown class \"{other}\" found"),
    }
}

fn to_block_extent(txt: String) -> (String, String, String) {
    let mut sections = txt.split(',');
    let msg = "blockExtent tag should have three comma separated integers in value";
    let result = (
        String::from(sections.next().expect(msg)),
        String::from(sections.next().expect(msg)),
        String::from(sections.next().expect(msg)),
    );
    if !sections.next().is_none() {
        panic!("{msg}")
    }
    result
}

fn to_compression(txt: String) -> String {
    match txt {
        other => panic!("Unknown compression \"{other}\" found"),
    }
}

fn to_chroma(txt: String) -> String {
    match txt {
        other => panic!("Unknown chroma \"{other}\" found"),
    }
}
