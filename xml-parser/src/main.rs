use std::path::Path;
use xml_parser::parse_xml;

const VULKAN_XML_PATH: &str = "xml-parser/vulkan_XML/vk.xml";

fn main() {
    parse_xml(Path::new(VULKAN_XML_PATH));
}
