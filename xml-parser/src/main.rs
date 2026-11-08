use std::fs::File;
use std::io::Write;
use std::path::Path;
use xml_parser::{generate_text, parse_xml};

const VULKAN_XML_PATH: &str = "xml-parser/vulkan_XML/vk.xml";
const FORMAT_PATH: &str = "src/renderer/device_handler/device_info_handler/format_info.rs";

fn main() {
    let parsed_xml = parse_xml(Path::new(VULKAN_XML_PATH));
    let texts = generate_text(parsed_xml);
    File::create(Path::new(FORMAT_PATH))
        .unwrap()
        .write_all(texts[0].as_bytes())
        .unwrap();
}
