pub(crate) mod format_generator_structures;

use super::text_parser::{Item, ParsedXml};
use format_generator_structures::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct FormatDetails {
    /// The class of the format.
    pub(super) class: ImageFormatClass,

    /// The number of bytes in a single block of texels.
    num_bytes: u32,

    /// The size in 3D space of each block of texels.
    block_extent: (u32, u32, u32),

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
    /// There may be padding to fill the words if the channels do not.
    packed: Option<u32>,

    /// The texel planes in the format.
    /// A plane is a memory layout.
    /// Lots of formats only have one plane, but some have more.
    planes: Vec<ImageFormatPlane>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FormatAttributeInfo {
    /// The format that this is the details for.
    name: String,
    format: ImageFormat,
    class: ImageFormatClass,
    num_bytes: u32,
    texels_per_block: u32,
    block_extent: (u32, u32, u32),
    packed: Option<u32>,
    compressed: Option<ImageFormatComponentCompressionScheme>,
    chroma_subsampling: Option<ChromaSubsampler>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct FormatElementInfo {
    components: Vec<FormatComponentInfo>,
    planes: Vec<FormatPlaneInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct FormatComponentInfo {
    channel: ImageFormatChannel,
    data_type_size: String,
    data_type_format: ImageFormatComponentDataTypeFormat,
    plane: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct FormatPlaneInfo {
    index: u32,
    scale_divisors: ImageFormatScaleDivisors,
    compatible: ImageFormat,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct NamedComponentInfo {
    channel: ImageFormatChannel,
    bits: String,
    plane: u32,
}

pub fn parse_formats<T: ParsedXml>(xml: &mut T, vec: &mut Vec<(ImageFormat, FormatDetails)>) {
    loop {
        match xml.next() {
            Some(Ok(Item::Element { name, attributes })) if name == String::from("format") => {
                if let Some(format) = get_format(attributes, xml) {
                    vec.push(format);
                }
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

fn get_format<T: ParsedXml>(
    attributes: Vec<(String, String)>,
    xml: &mut T,
) -> Option<(ImageFormat, FormatDetails)> {
    let attribute_info = match get_format_attribute_info(attributes) {
        Some(info) => info,
        None => {
            xml.skip_current_element().unwrap();
            return None
        },
    };
    let mut element_info = get_format_element_info(xml);
    element_info.planes.sort_by_key(|plane| plane.index);
    let mut planes = element_info
        .planes
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
    let named_format_info = get_named_components(
        &attribute_info.name,
        planes.len() as u32
    );
    if let Some(named_format_info) = named_format_info {
        compare_named_and_specified_components(&mut element_info.components, named_format_info);
    }
    for component in element_info.components {
        planes
            .get_mut(component.plane as usize)
            .unwrap()
            .components
            .push(ImageFormatComponent {
                channel: component.channel,
                data_type: ImageFormatComponentDataType {
                    size: get_size(component.data_type_size.as_str(), attribute_info.compressed),
                    format: component.data_type_format,
                },
            })
    }
    Some((
        attribute_info.format,
        FormatDetails {
            class: attribute_info.class,
            num_bytes: attribute_info.num_bytes,
            block_extent: attribute_info.block_extent,
            packed: attribute_info.packed,
            planes,
        },
    ))
}

fn get_format_attribute_info(attributes: Vec<(String, String)>) -> Option<FormatAttributeInfo> {
    const IGNORED_FORMATS: &[&str] = &[
        "VK_FORMAT_D24_UNORM_S8_UINT",
        "VK_FORMAT_X8_D24_UNORM_PACK32",
        "VK_FORMAT_R4G4_UNORM_PACK8",
        "VK_FORMAT_R5G6B5_UNORM_PACK16",
        "VK_FORMAT_B5G6R5_UNORM_PACK16",
        "VK_FORMAT_R4G4B4A4_UNORM_PACK16",
        "VK_FORMAT_B4G4R4A4_UNORM_PACK16",
        "VK_FORMAT_A4R4G4B4_UNORM_PACK16",
        "VK_FORMAT_A4B4G4R4_UNORM_PACK16",
        "VK_FORMAT_R5G5B5A1_UNORM_PACK16",
        "VK_FORMAT_B5G5R5A1_UNORM_PACK16",
        "VK_FORMAT_A1R5G5B5_UNORM_PACK16",
        "VK_FORMAT_A1B5G5R5_UNORM_PACK16",
        "VK_FORMAT_R16_SFLOAT_FPENCODING_BFLOAT16_ARM",
        "VK_FORMAT_R8_SFLOAT_FPENCODING_FLOAT8E4M3_ARM",
        "VK_FORMAT_R8_SFLOAT_FPENCODING_FLOAT8E5M2_ARM",
        "VK_FORMAT_PVRTC1_2BPP_UNORM_BLOCK_IMG",
        "VK_FORMAT_PVRTC1_2BPP_SRGB_BLOCK_IMG",
        "VK_FORMAT_PVRTC1_4BPP_UNORM_BLOCK_IMG",
        "VK_FORMAT_PVRTC1_4BPP_SRGB_BLOCK_IMG",
        "VK_FORMAT_PVRTC2_2BPP_UNORM_BLOCK_IMG",
        "VK_FORMAT_PVRTC2_2BPP_SRGB_BLOCK_IMG",
        "VK_FORMAT_PVRTC2_4BPP_UNORM_BLOCK_IMG",
        "VK_FORMAT_PVRTC2_4BPP_SRGB_BLOCK_IMG",
        "VK_FORMAT_R8_BOOL_ARM",
        "VK_FORMAT_R10X6_UINT_PACK16_ARM",
        "VK_FORMAT_R12X4_UINT_PACK16_ARM",
        "VK_FORMAT_R14X2_UINT_PACK16_ARM",
        "VK_FORMAT_R14X2_UNORM_PACK16_ARM",
        "VK_FORMAT_R10X6G10X6_UINT_2PACK16_ARM",
        "VK_FORMAT_R12X4G12X4_UINT_2PACK16_ARM",
        "VK_FORMAT_R14X2G14X2_UINT_2PACK16_ARM",
        "VK_FORMAT_R14X2G14X2_UNORM_2PACK16_ARM",
        "VK_FORMAT_G14X2_B14X2R14X2_2PLANE_420_UNORM_3PACK16_ARM",
        "VK_FORMAT_G14X2_B14X2R14X2_2PLANE_422_UNORM_3PACK16_ARM",
        "VK_FORMAT_R10X6G10X6B10X6A10X6_UINT_4PACK16_ARM",
        "VK_FORMAT_R12X4G12X4B12X4A12X4_UINT_4PACK16_ARM",
        "VK_FORMAT_R14X2G14X2B14X2A14X2_UINT_4PACK16_ARM",
        "VK_FORMAT_R14X2G14X2B14X2A14X2_UNORM_4PACK16_ARM",
        "VK_FORMAT_R16G16_SFIXED5_NV",
    ];
    let mut name = None;
    let mut class = None;
    let mut num_bytes = None;
    let mut texels_per_block = None;
    let mut block_extent = (1, 1, 1);
    let mut packed = None;
    let mut compressed = None;
    let mut chroma_subsampling = None;
    for attribute in attributes {
        match attribute.0.as_str() {
            "name" => {
                for ignored_format in IGNORED_FORMATS {
                    if attribute.1.as_str() == *ignored_format {
                        return None;
                    }
                }
                name = Some(attribute.1)
            },
            "class" => class = Some(ImageFormatClass::try_from(attribute.1.as_str()).unwrap()),
            "blockSize" => num_bytes = Some(attribute.1),
            "texelsPerBlock" => texels_per_block = Some(attribute.1),
            "blockExtent" => block_extent = to_block_extent(attribute.1),
            "packed" => packed = Some(attribute.1.parse::<u32>().unwrap()),
            "compressed" => {
                compressed = Some(
                    ImageFormatComponentCompressionScheme::try_from(attribute.1.as_str()).unwrap(),
                )
            }
            "chroma" => chroma_subsampling = Some(
                ChromaSubsampler::try_from(attribute.1.as_str()).unwrap()
            ),
            other => panic!(
                "Unexpected \"{other}\" attribute found with value \"{}\"",
                attribute.1,
            ),
        }
    }
    Some(FormatAttributeInfo {
        name: name.clone().unwrap(),
        format: ImageFormat::try_from(name.unwrap().as_str()).unwrap(),
        class: class.unwrap(),
        num_bytes: num_bytes.unwrap().parse::<u32>().unwrap(),
        texels_per_block: texels_per_block.unwrap().parse::<u32>().unwrap(),
        block_extent,
        packed,
        compressed,
        chroma_subsampling,
    })
}

fn get_format_element_info<T: ParsedXml>(xml: &mut T) -> FormatElementInfo {
    let mut elements = FormatElementInfo {
        components: Vec::new(),
        planes: Vec::new(),
    };
    loop {
        match xml.next().unwrap().unwrap() {
            Item::Element { name, attributes } => {
                if name.as_str() == "component" {
                    elements
                        .components
                        .push(get_format_component_info(attributes));
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

fn get_size(
    size: &str,
    compression: Option<ImageFormatComponentCompressionScheme>,
) -> ImageFormatComponentDataTypeSize {
    if size == "compressed" {
        ImageFormatComponentDataTypeSize::Compressed(compression.unwrap())
    } else {
        ImageFormatComponentDataTypeSize::Bits(
            size.parse()
                .expect("bits attribute must be integer or \"compressed\""),
        )
    }
}

fn get_named_components(name: &str, num_planes: u32) -> Option<Vec<NamedComponentInfo>> {
    let mut chunks = name.split('_');
    chunks.nth(1);
    let mut result = Vec::new();
    let mut plane = 0;
    for chunk in chunks {
        if ImageFormatComponentDataTypeFormat::try_from(chunk).is_ok() {
            continue
        }
        for compression_scheme in [
            "BC1", "BC2", "BC3", "BC4", "BC5", "BC6H", "BC7", "ETC2", "EAC", "ASTC", "PVRTC1",
            "PVRTC2",
        ] {
            if chunk == compression_scheme {
                return None;
            }
        }
        for ending_tag in [
            "420", "422", "444", "NV", "AMD", "INTEL", "ARM", "INTEL", "IMG", "QCOM", "EXT", "KHR",
        ] {
            if chunk == ending_tag {
                return Some(result);
            }
        }
        if chunk.contains("PACK") || chunk.contains("PLANE") {
            break;
        }
        let mut components = chunk.chars().peekable();
        while components.peek().is_some() {
            let channel = match components.next() {
                None => return Some(result),
                Some(char) => ImageFormatChannel::try_from(char)
                    .expect("Invalid image format character in format name"),
            };
            let mut bits = String::from(components.next().unwrap());
            loop {
                match components.peek() {
                    Some(component) if component.is_ascii_digit() => {
                        bits.push(components.next().unwrap())
                    },
                    _ => break,
                }
            }
            result.push(NamedComponentInfo {
                channel,
                bits,
                plane,
            })
        }
        plane += 1;

        // Some single plane formulas have their components split across multiple chunks
        if num_planes == 1 {
            plane = 0;
        }
    }
    Some(result)
}

fn compare_named_and_specified_components(
    specified: &mut Vec<FormatComponentInfo>,
    named: Vec<NamedComponentInfo>,
) {
    let mut i = 0;
    let mut named = named.into_iter();
    loop {
        match named.next() {
            None => {
                assert_eq!(i, specified.len());
                break;
            }
            Some(NamedComponentInfo {
                channel,
                bits,
                plane,
            }) if channel == ImageFormatChannel::Unused
                || channel == ImageFormatChannel::SharedExponent =>
            {
                if channel == ImageFormatChannel::SharedExponent {
                    assert_eq!(plane, 0)
                }
                specified.insert(
                    i,
                    FormatComponentInfo {
                        channel: ImageFormatChannel::Unused,
                        data_type_size: bits,
                        data_type_format: ImageFormatComponentDataTypeFormat::None,
                        plane,
                    },
                );
                i += 1;
            }
            Some(NamedComponentInfo {
                channel,
                bits,
                plane,
            }) => {
                let specified = specified.get(i).unwrap();
                assert_eq!(specified.channel, channel);
                assert_eq!(specified.data_type_size, bits);
                assert_eq!(specified.plane, plane);
                i += 1;
            }
        }
    }
}

fn to_block_extent(txt: String) -> (u32, u32, u32) {
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
            "numericFormat" => {
                format = Some(ImageFormatComponentDataTypeFormat::try_from(value.as_str()).unwrap())
            }
            "planeIndex" => plane = value.parse::<u32>().unwrap(),
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
            "index" => index = Some(value.parse::<u32>().unwrap()),
            "widthDivisor" => {
                width_scale_divisor =
                    Some(ImageFormatScaleDivisor::try_from(value.as_str()).unwrap())
            }
            "heightDivisor" => {
                height_scale_divisor =
                    Some(ImageFormatScaleDivisor::try_from(value.as_str()).unwrap())
            }
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
        compatible: compatible.unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn temp() {}
}
