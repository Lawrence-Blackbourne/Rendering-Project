//! This module is used for translating information from the ash style information the library
//! receives into useful information for the library user.
//! The module also contains structs helpful for setting up the device.

use ash::vk;
use crate::renderer::Size;

/// A struct holding a potential physical device in a way that is easily usable.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PhysicalDevice {

    /// The information about what this device can be used for
    pub device_info: DeviceInfo,
    pub(crate) device: vk::PhysicalDevice,
}

/// This stores the details about what surface information is supported.
#[derive(Clone, Debug)]
pub(crate) struct VulkanDisplayInfo {
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub formats: Vec<vk::SurfaceFormatKHR>,
    pub presentation_modes: Vec<vk::PresentModeKHR>,
}

/// A struct storing the info about what settings we want for our logical device.
///
/// # Examples
/// ```
/// # use rendering_project::renderer::device_info_handler::{DeviceInfo, LogicalDeviceSettings};
/// let device_info: DeviceInfo = get_device_info();
/// let settings: LogicalDeviceSettings = get_device_settings()
///     .with_num_swap_frames(device_info.capabilities.min_swapchain_image_count);
/// ```
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LogicalDeviceSettings {
    pub(crate) physical_device: PhysicalDevice,
    format: vk::SurfaceFormatKHR, // The data format of the images
    mode: vk::PresentModeKHR, // The presentation mode of the images
    size: vk::Extent2D, // Stores the size of the swapchain images
    pub(crate) num_swap_frames: u8,
}

impl LogicalDeviceSettings {

    /// Returns the physical device that is currently stored within the settings.
    pub fn get_physical_device(&self) -> &PhysicalDevice {
        &self.physical_device
    }

    /// Sets the physical device to be used.
    pub fn with_physical_device(mut self, device: PhysicalDevice) -> Self {
        self.physical_device = device;
        self
    }

    /// Returns the number of swap frames that is currently stored within the settings.
    pub fn get_num_swap_frames(&self) -> u8 {
        self.num_swap_frames
    }

    /// Sets the number of swap frames to be used.
    pub fn with_num_swap_frames(mut self, num_swap_frames: u8) -> Self {
        self.num_swap_frames = num_swap_frames;
        self
    }
}

/// The info to be given to an external caller so they can choose a setup they want.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeviceInfo {

    /// The capabilities of the device.
    pub capabilities: Capabilities,
    //TODO pub available_formats: Vec<Format>,
    //TODO pub presentation_modes: Vec<PresentationMode>
}

impl From<VulkanDisplayInfo> for DeviceInfo {
    fn from(value: VulkanDisplayInfo) -> Self {
        Self {
            capabilities: value.capabilities.into(),
        }
    }
}

/// What capabilities a physical device has.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Capabilities {

    /// The minimum number of images that can be in the swapchain.
    pub min_swapchain_image_count: u32,

    /// The maximum number of images that can be in the swapchain.
    pub max_swapchain_image_count: u32,

    /// The current size of the image size to be displayed in pixels.
    /// A None for current_image_size is a special value meaning that the size of the image depends
    /// on the size of the provided swapchain.
    pub current_image_size: Option<Size>,

    /// The minimum size of the swapchain in pixels.
    pub min_swapchain_size: Size,

    /// The maximum size of the swapchain in pixels.
    pub max_swapchain_size: Size,

    /// The maximum number of layers to the image to be rendered to.
    pub max_number_of_image_layers: u32,

    /// The possible transformations that can be applied to the image.
    pub supported_image_transformations: ImageTransformations,

    /// The current transformations that are applied to the image.
    pub current_image_transformations: ImageTransformations,

    /// The possible alpha compositing modes that can be applied to the rendering process.
    pub supported_alpha_compositing_modes: AlphaCompositingModes,

    /// The possible usages of the image
    pub supported_image_usages: ImageUsages,
}

impl From<vk::SurfaceCapabilitiesKHR> for Capabilities {
    fn from(value: vk::SurfaceCapabilitiesKHR) -> Self {
        Self {
            min_swapchain_image_count: value.min_image_count,
            max_swapchain_image_count: value.max_image_count,
            current_image_size: match value.current_extent {
                vk::Extent2D{ width: 0xFFFFFFFF, height: 0xFFFFFFFF } => None,
                value => Some(value.into())
            },
            min_swapchain_size: value.min_image_extent.into(),
            max_swapchain_size: value.max_image_extent.into(),
            max_number_of_image_layers: value.max_image_array_layers,
            supported_image_transformations: value.supported_transforms.into(),
            current_image_transformations: value.current_transform.into(),
            supported_alpha_compositing_modes: value.supported_composite_alpha.into(),
            supported_image_usages: value.supported_usage_flags.into(),
        }
    }
}

/// Stores transformations of the image.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ImageTransformations {

    /// This transformation does nothing to the image.
    pub identity: bool,

    /// The image is rotated 90 degrees clockwise.
    pub rotate_90_degrees: bool,

    /// The image is rotated 180 degrees.
    pub rotate_180_degrees: bool,

    /// The image is rotated 270 degrees clockwise.
    pub rotate_270_degrees: bool,

    /// The image is mirrored horizontally.
    pub mirror: bool,

    /// The image is mirrored horizontally and then rotated 90 degrees clockwise
    pub mirror_and_rotate_90_degrees: bool,

    /// The image is mirrored horizontally and then rotated 180 degrees
    pub mirror_and_rotate_180_degrees: bool,

    /// The image is mirrored horizontally and then rotated 270 degrees clockwise
    pub mirror_and_rotate_270_degrees: bool,

    /// Inherit means the transformation is not specified, and is determined by platform specific
    /// considerations and mechanisms.
    pub inherit: bool,
}

impl From<vk::SurfaceTransformFlagsKHR> for ImageTransformations {
    fn from(value: vk::SurfaceTransformFlagsKHR) -> Self {
        Self {
            identity: value.contains(vk::SurfaceTransformFlagsKHR::IDENTITY),
            rotate_90_degrees: value.contains(vk::SurfaceTransformFlagsKHR::ROTATE_90),
            rotate_180_degrees: value.contains(vk::SurfaceTransformFlagsKHR::ROTATE_180),
            rotate_270_degrees: value.contains(vk::SurfaceTransformFlagsKHR::ROTATE_270),
            mirror: value.contains(vk::SurfaceTransformFlagsKHR::HORIZONTAL_MIRROR),
            mirror_and_rotate_90_degrees: value.contains(
                vk::SurfaceTransformFlagsKHR::HORIZONTAL_MIRROR_ROTATE_90
            ),
            mirror_and_rotate_180_degrees: value.contains(
                vk::SurfaceTransformFlagsKHR::HORIZONTAL_MIRROR_ROTATE_180
            ),
            mirror_and_rotate_270_degrees: value.contains(
                vk::SurfaceTransformFlagsKHR::HORIZONTAL_MIRROR_ROTATE_270
            ),
            inherit: value.contains(vk::SurfaceTransformFlagsKHR::INHERIT)
        }
    }
}

/// Stores how the alpha component of the image is determined.
/// If the image does not have an alpha component, these settings will not do anything (except
/// potentially inherit depending on the specific system).
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct AlphaCompositingModes {

    /// The alpha is ignored, and the image is treated as a constant alpha of 1.
    pub opaque: bool,

    /// The non-alpha components are expected to already be multiplied by the alpha component, and
    /// the alpha is respected.
    pub pre_multiplied: bool,

    /// The non-alpha components are not expected to already be multiplied by the alpha component,
    /// and the compositor will multiply the non-alpha components by the alpha component, and the
    /// alpha is respected.
    pub post_multiplied: bool,

    /// The way that the alpha component is used is platform-specific.
    pub inherit: bool,
}

impl From<vk::CompositeAlphaFlagsKHR> for AlphaCompositingModes {
    fn from(value: vk::CompositeAlphaFlagsKHR) -> Self {
        Self {
            opaque: value.contains(vk::CompositeAlphaFlagsKHR::OPAQUE),
            pre_multiplied: value.contains(vk::CompositeAlphaFlagsKHR::PRE_MULTIPLIED),
            post_multiplied: value.contains(vk::CompositeAlphaFlagsKHR::POST_MULTIPLIED),
            inherit: value.contains(vk::CompositeAlphaFlagsKHR::INHERIT),
        }
    }
}

/// Specifies how an image is used.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ImageUsages {

}

impl From<vk::ImageUsageFlags> for ImageUsages {
    fn from(_value: vk::ImageUsageFlags) -> Self {
        ImageUsages {

        }
    }
}

/// An image format and colour space pair.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Format {

    /// The format that the image will use.
    pub image_format: ImageFormat,
    //TODO pub colour_space: ColourSpace,
}

impl TryFrom<vk::SurfaceFormatKHR> for Format {
    type Error = FormatConversionError;

    fn try_from(value: vk::SurfaceFormatKHR) -> Result<Self, Self::Error> {
        Ok(Format {
            image_format: match value.format.try_into() {
                Ok(fmt) => fmt,
                Err(str) => return Err(FormatConversionError::ImageFormatError(str)),
            },
            //colour_space: value.into(),
        })
    }
}

/// An enum used in the conversion from vk::SurfaceFormatKHR to Format.
pub enum FormatConversionError {
    ImageFormatError(String),
    ColourSpaceError,
}

/// An image format.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImageFormat {
    RegularFormat(RegularImageFormat),
}

impl TryFrom<vk::Format> for ImageFormat {
    type Error = String;

    fn try_from(value: vk::Format) -> Result<Self, Self::Error> {
        match RegularImageFormat::try_from(value) {
            Ok(fmt) => Ok(ImageFormat::RegularFormat(fmt)),
            Err(str) => Err(str),
        }
    }
}

/// An image format with some number (potentially 0) of bits in the red, green, blue, and alpha
/// channels, with each pixel having its own data, packed into a bitstring.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RegularImageFormat {

    /// The number of bits in the red channel.
    pub red_channel: u8,

    /// The number of bits in the green channel.
    pub green_channel: u8,

    /// The number of bits in the blue channel.
    pub blue_channel: u8,

    /// The number of bits in the alpha channel.
    pub alpha_channel: u8,

    /// The number of bits in the depth channel.
    pub depth_channel: u8,

    /// Stores if the data channels are signed or not.
    pub signed: bool,

    /// Stores if the data is packed or not.
    /// On a packed data format, the data is stored in one big integer, and the first element in the
    /// order takes the least significant bits, all the way to the most significant bits for the
    /// last element.
    /// On a non-packed data format, the data is stored in a byte array, with the first elements of
    /// the order having the lower index bytes, and the later elements having the higher index.
    /// Non-packed data formats are only available when each element takes an integer number of
    /// bytes.
    /// The distinction between the two possibilities matters when reading the true values stored in
    /// RAM due to the endianness of systems.
    pub packed: bool,

    /// Stores the order that the data is packed into the bitstring in.
    pub order: RegularImageFormatOrder,

    /// The way that the data is converted when passed to the shader.
    pub data_conversion: RegularImageFormatConversion,
}

impl TryFrom<vk::Format> for RegularImageFormat {
    type Error = String;

    fn try_from(value: vk::Format) -> Result<Self, Self::Error> {
        let str = format!("{value:?}");
        let mut chars = str.chars().peekable();

        let mut r: u8 = 0;
        let mut g: u8 = 0;
        let mut b: u8 = 0;
        let mut a: u8 = 0;
        let mut d: u8 = 0;

        let order_string = match Self::get_order_string(
            &mut chars, &mut r, &mut g, &mut b, &mut a, &mut d
        ) {
            None => return Err(str),
            Some(str) => str,
        };
        let order = match Self::get_order_from_str(&order_string) {
            Some(order) => order,
            None => return Err(str),
        };

        let signed = match chars.next() {
            None => return Err(str),
            Some('U') => false,
            Some('S') => true,
            Some(_) => return Err(str),
        };

        let conversion_string = Self::get_conversion_string(&mut chars);
        let conversion = match Self::get_conversion_from_str(&conversion_string) {
            Some(conversion) => conversion,
            None => return Err(str),
        };

        let packed_string = match Self::get_packed_string(&mut chars) {
            None => return Err(str),
            Some(str) => str,
        };
        let packed = if packed_string.as_str() == "" {
            false
        } else {
            let pack_size = match packed_string.as_str() {
                "PACK8" => 8,
                "PACK16" => 16,
                "PACK32" => 32,
                _ => return Err(str)
            };
            if r + g + b + a + d != pack_size {
                return Err(str)
            }
            if r % 8 != 0 || g % 8 != 0 || b % 8 != 0 || a % 8 != 0 || d % 8 != 0 {
                return Err(str)
            }
            true
        };

        Ok(Self{
            red_channel: r,
            green_channel: g,
            blue_channel: b,
            alpha_channel: a,
            depth_channel: d,
            signed,
            packed,
            order,
            data_conversion: conversion,
        })
    }
}

impl RegularImageFormat {
    fn get_num_bits(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<u8> {
        let mut current: u8 = match chars.next() {
            None => return None,
            Some(c) => match c.to_digit(10) {
                None => return None,
                Some(d) => d as u8,
            }
        };
        loop {
            match chars.peek() {
                None => return Some(current),
                Some(c) => match c.to_digit(10) {
                    None => return None,
                    Some(d) => current = current * 10 + d as u8,
                }
            }
            chars.next();
        }
    }

    fn update_bit_count(count: &mut u8, chars: &mut std::iter::Peekable<std::str::Chars>) -> bool {
        *count = match Self::get_num_bits(chars) {
            None => return false,
            Some(v) => {
                if v == 0 || *count != 0 {
                    return false;
                }
                v
            },
        };
        true
    }

    fn get_order_string(
        chars: &mut std::iter::Peekable<std::str::Chars>,
        r: &mut u8, g: &mut u8, b: &mut u8, a: &mut u8, d: &mut u8,
    ) -> Option<String> {
        let mut order_string = String::new();
        loop {
            let next_char = match chars.next() {
                None => return None,
                Some(c) => c,
            };
            match next_char {
                'R' => if Self::update_bit_count(r, chars) {
                    order_string.push('R')
                } else {
                    return None;
                }
                'G' => if Self::update_bit_count(g, chars) {
                    order_string.push('G')
                } else {
                    return None;
                }
                'B' => if Self::update_bit_count(b, chars) {
                    order_string.push('B')
                } else {
                    return None;
                }
                'A' => if Self::update_bit_count(a, chars) {
                    order_string.push('A')
                } else {
                    return None;
                }
                'D' => if Self::update_bit_count(d, chars) {
                    order_string.push('D')
                } else {
                    return None;
                }
                '_' => break,
                _ => return None,
            }
            order_string.push(next_char);
        }
        Some(order_string)
    }

    fn get_order_from_str(order_string: &str) -> Option<RegularImageFormatOrder> {
        match order_string {
            "R" => Some(RegularImageFormatOrder::R),
            "A" => Some(RegularImageFormatOrder::A),
            "D" => Some(RegularImageFormatOrder::D),
            "RG" => Some(RegularImageFormatOrder::RG),
            "RGB" => Some(RegularImageFormatOrder::RGB),
            "BGR" => Some(RegularImageFormatOrder::BGR),
            "RGBA" => Some(RegularImageFormatOrder::RGBA),
            "BGRA" => Some(RegularImageFormatOrder::BGRA),
            "ARGB" => Some(RegularImageFormatOrder::ARGB),
            "ABGR" => Some(RegularImageFormatOrder::ABGR),
            _ => None,
        }
    }

    fn get_conversion_string(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
        let mut conversion_string = String::new();
        loop {
            let next_char = match chars.next() {
                None => break,
                Some(c) => c,
            };
            match next_char {
                '_' => break,
                c => conversion_string.push(c),
            }
        }
        conversion_string
    }

    fn get_conversion_from_str(conversion_string: &str) -> Option<RegularImageFormatConversion> {
        match conversion_string {
            "INT" => Some(RegularImageFormatConversion::Int),
            "NORM" => Some(RegularImageFormatConversion::Norm),
            "SCALED" => Some(RegularImageFormatConversion::Float),
            _ => None,
        }
    }

    fn get_packed_string(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<String> {
        let mut packed_string = String::new();
        loop {
            let next_char = match chars.next() {
                None => break,
                Some(c) => c,
            };
            match next_char {
                '_' => return None,
                c => packed_string.push(c),
            }
        }
        Some(packed_string)
    }
}

/// The order that the data is packed into the data structure for when it is ambiguous (e.g. when
/// the channels are packed into an int).
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RegularImageFormatOrder {
    R,
    A,
    D,
    RG,
    RGB,
    BGR,
    RGBA,
    BGRA,
    ARGB,
    ABGR,
}

/// This describes how the data gets converted when passed to the shader.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RegularImageFormatConversion {

    /// The data is given to the shaders as an integer directly.
    Int,

    /// The data is cast as a float, the value of which is equal to the value of the integer stored.
    Float,

    /// The data is cast as a float, the value of which is normalized to between 0 and 1 inclusively
    /// for an unsigned data type, and between -1 and 1 for a signed data type.
    Norm,
}