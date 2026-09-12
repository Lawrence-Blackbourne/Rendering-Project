use super::text_parser::{Item, ParsedXml};
use crate::generator_structures::format_generator_enums::{ImageFormatClass,
                                                          ImageFormatCompressionScheme};

use std::io::Read;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ParsedFormat {
    pub attributes: FormatAttributeInfo,
    pub components: Vec<FormatComponentInfo>,
    pub planes: Vec<FormatPlaneInfo>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FormatAttributeInfo {
    name: String,
    class: ImageFormatClass,
    block_size: String,
    texels_per_block: String,
    block_extent: (String, String, String),
    packed: Option<String>,
    compressed: Option<ImageFormatCompressionScheme>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FormatElementInfo {
    components: Vec<FormatComponentInfo>,
    planes: Vec<FormatPlaneInfo>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FormatComponentInfo {
    name: String,
    bits: u8,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FormatPlaneInfo {}

pub fn parse_formats<T: Read>(xml: &mut ParsedXml<T>, vec: &mut Vec<ParsedFormat>) {
    loop {
        match xml.next() {
            Some(Ok(Item::Element { name, attributes })) if name == String::from("format") => {
                let attribute_info = get_format_attribute_info(attributes);
                let element_info = get_format_element_info(xml);
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
    let mut name = None;
    let mut class = None;
    let mut block_size = None;
    let mut texels_per_block = None;
    let mut block_extent = (String::from("1"), String::from("1"), String::from("1"));
    let mut packed = None;
    let mut compressed = None;
    for attribute in attributes {
        match attribute.0.as_str() {
            "name" => name = Some(attribute.1),
            "class" => class = Some(ImageFormatClass::try_from(attribute.1.as_str()).unwrap()),
            "blockSize" => block_size = Some(attribute.1),
            "texels_per_block" => texels_per_block = Some(attribute.1),
            "block_extent" => block_extent = to_block_extent(attribute.1),
            "packed" => packed = Some(attribute.1),
            "compressed" => compressed = Some(
                ImageFormatCompressionScheme::try_from(attribute.1.as_str()).unwrap()
            ),
            other => panic!(
                "Unexpected \"{other}\" attribute found with value \"{}\"",
                attribute.1
            ),
        }
    }
    FormatAttributeInfo {
        name: name.unwrap(),
        class: class.unwrap(),
        block_size: block_size.unwrap(),
        texels_per_block: texels_per_block.unwrap(),
        block_extent,
        packed,
        compressed,
    }
}

fn get_format_element_info<T: Read>(xml: &mut ParsedXml<T>) -> FormatElementInfo {
    let mut elements = FormatElementInfo {
        components: Vec::new(),
        planes: Vec::new(),
    };
    loop {
        match xml.next().unwrap().unwrap() {
            Item::Element{ name, attributes } => {
                if name.as_str() == "component" {
                    elements.components.push(get_format_component_info(attributes));
                } else if name.as_str() == "plane" {
                    elements.planes.push(get_format_plane_info(attributes));
                } else if !(name.as_str() == "spirvimageformat") {
                    panic!("Unknown element found with name \"{name}\"")
                }
                assert_eq!(xml.next().unwrap().unwrap(), Item::EndCurrentElement)
            }
            Item::Text(txt) => panic!("Unexpected text element found: \"{txt}\""),
            Item::EndCurrentElement => return elements,
        }
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

fn get_format_component_info(attributes: Vec<(String, String)>) -> FormatComponentInfo {
    let name = None
}

fn get_format_plane_info(attributes: Vec<(String, String)>) -> FormatPlaneInfo {
    todo!()
}