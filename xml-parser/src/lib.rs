mod format_parser;
mod generator_structures;
pub mod text_parser;

use format_parser::{FormatDetails, format_generator_structures::ImageFormat};

use crate::format_parser::format_generator_structures::{
    ImageFormatChannel, ImageFormatClass, ImageFormatComponent,
    ImageFormatComponentCompressionScheme, ImageFormatComponentDataType,
    ImageFormatComponentDataTypeFormat, ImageFormatComponentDataTypeSize, ImageFormatPlane,
    ImageFormatScaleDivisor, ImageFormatScaleDivisors,
};
use crate::generator_structures::Generator;
use crate::text_parser::ParsedXml;
use std::path::Path;
use text_parser::Item;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ParsedDetails {
    formats: Vec<(ImageFormat, FormatDetails)>,
}

impl ParsedDetails {
    fn new() -> ParsedDetails {
        ParsedDetails {
            formats: Vec::new(),
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
    ImageFormat::generate_text();
    ImageFormatClass::generate_text();
    ImageFormatPlane::generate_text();
    ImageFormatScaleDivisors::generate_text();
    ImageFormatScaleDivisor::generate_text();
    ImageFormatComponent::generate_text();
    ImageFormatChannel::generate_text();
    ImageFormatComponentDataType::generate_text();
    ImageFormatComponentDataTypeFormat::generate_text();
    ImageFormatComponentDataTypeSize::generate_text();
    ImageFormatComponentCompressionScheme::generate_text();
    ImageFormat::generate_getters(Vec::new());
}

#[cfg(test)]
const TEST_XML_PATH: &str = "vulkan_XML/vk.xml";

#[cfg(feature = "bench")]
const BENCH_XML_PATH: &str = "vulkan_XML/vk.xml";