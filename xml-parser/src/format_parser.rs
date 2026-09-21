pub(crate) mod format_generator_structures;

use super::text_parser::{Item, ParsedXml};
use format_generator_structures::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct FormatDetails {
    /// The class of the format.
    pub(super) class: ImageFormatClass,

    /// The number of bytes in a single block of texels.
    num_bytes: u8,

    /// The size in 3D space of each block of texels.
    block_extent: (u8, u8, u8),

    /// Stores if the data is packed or not.
    /// On a non-packed data format, the data is stored in a byte array, with each component
    /// taking up as many of these bytes as needed.
    /// The components are stored with the first components in the least significant indexes in
    /// the array.
    /// On a packed data format, the data is stored in integer(s) with the same number of bits
    /// as the packed value, and these are stored in an array. Multiple components may be in a
    /// single packed value, and there may (rarely) be unused space at the end of each packed
    /// value.
    /// The components are stored with the first components in the least significant bits in the
    /// integer.
    /// The distinction matters due to the endianness of CPUs.
    /// There may be padding to fill the the words if the channels do not.
    packed: Option<u8>,

    /// The texel planes in the format.
    /// A plane is a memory layout.
    /// Lots of formats only have one plane, but some have more.
    planes: Vec<ImageFormatPlane>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FormatAttributeInfo {
    /// The format that this is the details for.
    format: ImageFormat,
    class: ImageFormatClass,
    num_bytes: u8,
    block_extent: (u8, u8, u8),
    packed: Option<u8>,
    compressed: Option<ImageFormatComponentCompressionScheme>
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct FormatElementInfo {
    components: Vec<FormatComponentInfo>,
    planes: Vec<FormatPlaneInfo>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct FormatComponentInfo {
    channel: ImageFormatChannel,
    data_type_size: String,
    data_type_format: ImageFormatComponentDataTypeFormat,
    plane: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct FormatPlaneInfo {
    index: usize,
    scale_divisors: ImageFormatScaleDivisors,
    compatible: ImageFormat,
}

pub fn parse_formats<T: ParsedXml>(
    xml: &mut T,
    vec: &mut Vec<(ImageFormat, FormatDetails)>
) {
    loop {
        match xml.next() {
            Some(Ok(Item::Element { name, attributes })) if name == String::from("format") => {
                let attribute_info = get_format_attribute_info(attributes);
                let mut element_info = get_format_element_info(xml);
                element_info.planes.sort_by_key(|plane| plane.index);
                let mut planes = element_info.planes
                    .into_iter()
                    .map(|plane| ImageFormatPlane {
                        components: Vec::new(),
                        scale_divisors: plane.scale_divisors,
                        compatible_single_plane_format: plane.compatible,
                    })
                    .collect::<Vec<_>>();
                if planes.is_empty() {
                    planes.push(ImageFormatPlane {
                        components: Vec::new(),
                        scale_divisors: ImageFormatScaleDivisors::default(),
                        compatible_single_plane_format: attribute_info.format,
                    })
                }
                for component in element_info.components {
                    planes.get_mut(component.plane).unwrap().components.push(ImageFormatComponent {
                        channel: component.channel,
                        data_type: ImageFormatComponentDataType {
                            size: {
                                if component.data_type_size.as_str() == "compressed" {
                                    ImageFormatComponentDataTypeSize::Compressed(
                                        attribute_info.compressed.unwrap()
                                    )
                                } else {
                                    ImageFormatComponentDataTypeSize::Bits(
                                        component.data_type_size.parse().expect(
                                            "bits attribute must be integer or \"compressed\""
                                        )
                                    )
                                }
                            },
                            format: component.data_type_format,
                        },
                    })
                }
                vec.push((
                    attribute_info.format,
                    FormatDetails {
                        class: attribute_info.class,
                        num_bytes: attribute_info.num_bytes,
                        block_extent: attribute_info.block_extent,
                        packed: attribute_info.packed,
                        planes,
                    },
                ));
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
    let mut num_bytes = None;
    let mut texels_per_block = None;
    let mut block_extent = (1, 1, 1);
    let mut packed = None;
    let mut compressed = None;
    for attribute in attributes {
        match attribute.0.as_str() {
            "name" => name = Some(attribute.1),
            "class" => class = Some(ImageFormatClass::try_from(attribute.1.as_str()).unwrap()),
            "blockSize" => num_bytes = Some(attribute.1),
            "texelsPerBlock" => texels_per_block = Some(attribute.1),
            "blockExtent" => block_extent = to_block_extent(attribute.1),
            "packed" => packed = Some(attribute.1.parse::<u8>().unwrap()),
            "compressed" => compressed = Some(
                ImageFormatComponentCompressionScheme::try_from(attribute.1.as_str()).unwrap()
            ),
            other => panic!(
                "Unexpected \"{other}\" attribute found with value \"{}\"",
                attribute.1,
            ),
        }
    }
    assert_eq!(
        texels_per_block.unwrap().parse::<u8>().unwrap(),
        block_extent.0 * block_extent.1 * block_extent.2,
    );
    FormatAttributeInfo {
        format: ImageFormat::try_from(name.unwrap().as_str()).unwrap(),
        class: class.unwrap(),
        num_bytes: num_bytes.unwrap().parse::<u8>().unwrap(),
        block_extent,
        packed,
        compressed,
    }
}

fn get_format_element_info<T: ParsedXml>(xml: &mut T) -> FormatElementInfo {
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
            Item::Text(_) => (),
            Item::EndCurrentElement => return elements,
        }
    }
}

fn to_block_extent(txt: String) -> (u8, u8, u8) {
    let mut sections = txt.split(',');
    let msg = "blockExtent tag should have three comma separated integers in value";
    let result = (
        sections.next().expect(msg).parse().expect(msg),
        sections.next().expect(msg).parse().expect(msg),
        sections.next().expect(msg).parse().expect(msg),
    );
    if !sections.next().is_none() {
        panic!("{msg}")
    }
    result
}

fn get_format_component_info(attributes: Vec<(String, String)>) -> FormatComponentInfo {
    let mut channel = None;
    let mut bits = None;
    let mut format = None;
    let mut plane = 0;
    for (item, value) in attributes.into_iter() {
        match item.as_str() {
            "name" => channel = Some(ImageFormatChannel::try_from(value.as_str()).unwrap()),
            "bits" => bits = Some(value),
            "numericFormat" => format = Some(
                ImageFormatComponentDataTypeFormat::try_from(value.as_str()).unwrap()
            ),
            "planeIndex" => plane = value.parse::<usize>().unwrap(),
            _ => panic!("Unexpected \"{item}\" attribute found with value \"{value}\""),
        }
    }
    FormatComponentInfo {
        channel: channel.unwrap(),
        data_type_size: bits.unwrap(),
        data_type_format: format.unwrap(),
        plane,
    }
}

fn get_format_plane_info(attributes: Vec<(String, String)>) -> FormatPlaneInfo {
    let mut index = None;
    let mut width_scale_divisor = None;
    let mut height_scale_divisor = None;
    let mut compatible = None;
    for (item, value) in attributes.into_iter() {
        match item.as_str() {
            "index" => index = Some(value.parse::<usize>().unwrap()),
            "widthDivisor" => width_scale_divisor = Some(
                ImageFormatScaleDivisor::try_from(value.as_str()).unwrap()
            ),
            "heightDivisor" => height_scale_divisor = Some(
                ImageFormatScaleDivisor::try_from(value.as_str()).unwrap()
            ),
            "compatible" => compatible = Some(ImageFormat::try_from(value.as_str()).unwrap()),
            _ => panic!("Unexpected \"{item}\" attribute found with value \"{value}\""),
        }
    }
    FormatPlaneInfo {
        index: index.unwrap(),
        scale_divisors: ImageFormatScaleDivisors {
            width: width_scale_divisor.unwrap(),
            height: height_scale_divisor.unwrap(),
        },
        compatible: compatible.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn temp() {}
}