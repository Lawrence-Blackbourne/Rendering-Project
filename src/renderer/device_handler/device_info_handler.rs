//! This module is used for translating information from the ash style information the library
//! receives into useful information for the library user.
//! The module also contains structs helpful for setting up the device.

use crate::renderer::Size;
use RegularImageFormatConversion::{Float, Int, Norm, SRGB, Scaled};
use RegularImageFormatOrder::{ABGR, ARGB, BGR, BGRA, D, R, RG, RGB, RGBA, RX, RXGX, RXGXBXAX, XD};
use ash::vk;

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

/// The info about a specific device, including the capabilities of the device, the formats that the
/// rendering can be done in, and the presentation modes available.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeviceInfo {
    /// The capabilities of the device.
    pub capabilities: Capabilities,

    /// The formats that can be rendered in.
    pub available_formats: Vec<Format>,
    // pub presentation_modes: Vec<PresentationMode>
}

impl From<VulkanDisplayInfo> for DeviceInfo {
    fn from(value: VulkanDisplayInfo) -> Self {
        Self {
            capabilities: value.capabilities.into(),
            available_formats: value
                .formats
                .into_iter()
                .filter_map(|f| Format::try_from(f).ok())
                .collect(),
        }
    }
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
    mode: vk::PresentModeKHR,     // The presentation mode of the images
    size: vk::Extent2D,           // Stores the size of the swapchain images
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
                vk::Extent2D {
                    width: 0xFFFFFFFF,
                    height: 0xFFFFFFFF,
                } => None,
                value => Some(value.into()),
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

/// An image format and colour space pair.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Format {
    /// The format that the image will use.
    pub image_format: ImageFormat,
    pub colour_space: ColourSpace,
}

impl TryFrom<vk::SurfaceFormatKHR> for Format {
    type Error = FormatConversionError;

    fn try_from(value: vk::SurfaceFormatKHR) -> Result<Self, Self::Error> {
        Ok(Format {
            image_format: match value.format.try_into() {
                Ok(fmt) => fmt,
                Err(()) => return Err(FormatConversionError::ImageFormatError(value.format)),
            },
            colour_space: match value.color_space.try_into() {
                Ok(clr) => clr,
                Err(()) => return Err(FormatConversionError::ColourSpaceError(value.color_space)),
            },
        })
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
            mirror_and_rotate_90_degrees: value
                .contains(vk::SurfaceTransformFlagsKHR::HORIZONTAL_MIRROR_ROTATE_90),
            mirror_and_rotate_180_degrees: value
                .contains(vk::SurfaceTransformFlagsKHR::HORIZONTAL_MIRROR_ROTATE_180),
            mirror_and_rotate_270_degrees: value
                .contains(vk::SurfaceTransformFlagsKHR::HORIZONTAL_MIRROR_ROTATE_270),
            inherit: value.contains(vk::SurfaceTransformFlagsKHR::INHERIT),
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
pub struct ImageUsages {}

impl From<vk::ImageUsageFlags> for ImageUsages {
    fn from(_value: vk::ImageUsageFlags) -> Self {
        ImageUsages {}
    }
}

/// A colour space that the values in the image format can be interpreted in.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ColourSpace {
    /// The images are in sRGB colour space, encoded according to the sRGB specification.
    NonLinearSRGB,

    /// The images are in extended sRGB colour space, encoded using a linear transfer function.
    LinearExtendedSRGB,

    /// The images are in extended sRGB colour space, encoded accorded to the scRGB specification.
    NonLinearExtendedSRGB,

    /// The images are in Display-p3 colour space, encoded using a linear transfer function.
    LinearDisplayP3,

    /// The images are in Display-P3 colour space, encoded using a Display-P3 transfer function.
    NonLinearDisplayP3,

    /// The images are in DCI-P3 colour space, encoded using accorded to the DCI-P3 specification
    /// Values in the image are interpreted as XYZ encoded colour data by the vulkan presentation
    /// engine underpinning this library.
    NonLinearDCIP3,

    /// The images are in BT709 colour space, encoded using a linear transfer function.
    LinearBT709,

    /// The images are in BT709 colour space, encoded according to the BT709 specification
    NonLinearBT709,

    /// The images are in BT2020 colour space, encoded using a linear transfer function.
    LinearBT2020,

    /// The images are using the HDR10 standard (BT2020 colour space), encoded according to the
    /// SMPTE ST2084 Perceptual Quantizer (PQ) specification.
    NonLinearBT2020UsingST2084,

    /// The images are using the HDR10 standard (BT2020 colour space), encoded according to the
    /// Hybrid Log Gamma (HLG) specification.
    NonLinearBT2020UsingHLG,

    /// The images are in Adobe RGB colour space, encoded using a linear transfer function.
    LinearAdobeRGB,

    /// The images are in Adobe RGB colour space, encoding according to the Adobe RGB specification.
    NonLinearAdobeRGB,

    /// The images are in no particular colour space, rather the colour components are used as is.
    /// This is intended to allow applications to use colour spaces not explicitly supported.
    PassThrough,

    /// The images are in the display's native colour space.
    /// This matches the expectations of AMD's FreeSync2 standard, for any displays that support it.
    Native,
}

impl TryFrom<vk::ColorSpaceKHR> for ColourSpace {
    type Error = ();

    fn try_from(value: vk::ColorSpaceKHR) -> Result<Self, Self::Error> {
        match value {
            vk::ColorSpaceKHR::SRGB_NONLINEAR => Ok(Self::NonLinearSRGB),
            vk::ColorSpaceKHR::DISPLAY_P3_NONLINEAR_EXT => Ok(Self::NonLinearDisplayP3),
            vk::ColorSpaceKHR::EXTENDED_SRGB_LINEAR_EXT => Ok(Self::LinearExtendedSRGB),
            vk::ColorSpaceKHR::DISPLAY_P3_LINEAR_EXT => Ok(Self::LinearDisplayP3),
            vk::ColorSpaceKHR::DCI_P3_NONLINEAR_EXT => Ok(Self::NonLinearDCIP3),
            vk::ColorSpaceKHR::BT709_LINEAR_EXT => Ok(Self::LinearBT709),
            vk::ColorSpaceKHR::BT709_NONLINEAR_EXT => Ok(Self::NonLinearBT709),
            vk::ColorSpaceKHR::BT2020_LINEAR_EXT => Ok(Self::LinearBT2020),
            vk::ColorSpaceKHR::HDR10_ST2084_EXT => Ok(Self::NonLinearBT2020UsingST2084),
            vk::ColorSpaceKHR::DOLBYVISION_EXT => Err(()),
            vk::ColorSpaceKHR::HDR10_HLG_EXT => Ok(Self::NonLinearBT2020UsingHLG),
            vk::ColorSpaceKHR::ADOBERGB_LINEAR_EXT => Ok(Self::LinearAdobeRGB),
            vk::ColorSpaceKHR::ADOBERGB_NONLINEAR_EXT => Ok(Self::NonLinearAdobeRGB),
            vk::ColorSpaceKHR::PASS_THROUGH_EXT => Ok(Self::PassThrough),
            vk::ColorSpaceKHR::EXTENDED_SRGB_NONLINEAR_EXT => Ok(Self::NonLinearExtendedSRGB),
            vk::ColorSpaceKHR::DISPLAY_NATIVE_AMD => Ok(Self::Native),
            _ => Err(()),
        }
    }
}

/// An image format.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImageFormat {
    RegularFormat(RegularImageFormat),
}

impl TryFrom<vk::Format> for ImageFormat {
    type Error = ();

    fn try_from(value: vk::Format) -> Result<Self, Self::Error> {
        match RegularImageFormat::try_from(value) {
            Ok(fmt) => Ok(ImageFormat::RegularFormat(fmt)),
            Err(()) => Err(()),
        }
    }
}

/// An enum used in the conversion from vk::SurfaceFormatKHR to Format.
pub enum FormatConversionError {
    ImageFormatError(vk::Format),
    ColourSpaceError(vk::ColorSpaceKHR),
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

    /// The unused bits of the structure, which are optionally available for other uses
    pub unused_bits: u8,

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
    /// Some formats have a number of bits not equal to a full number of bytes, and unused bits
    /// after that to fill the gaps.
    /// For these, if packed is true, then the group of bits with the unused ones at the end form an
    /// unpacked word, and it is these words which are then packed
    pub packed: bool,

    /// Stores the order that the data is packed into the bitstring in.
    pub order: RegularImageFormatOrder,

    /// The way that the data is converted when passed to the shader.
    pub data_conversion: RegularImageFormatConversion,
}

impl TryFrom<vk::Format> for RegularImageFormat {
    type Error = ();

    fn try_from(value: vk::Format) -> Result<Self, Self::Error> {
        for item in REGULAR_IMAGE_FORMAT_CONVERSION_DATA {
            if value == item.0 {
                return Ok(RegularImageFormat {
                    red_channel: item.1,
                    green_channel: item.2,
                    blue_channel: item.3,
                    alpha_channel: item.4,
                    depth_channel: item.5,
                    unused_bits: item.6,
                    signed: item.7,
                    packed: item.8,
                    order: item.9,
                    data_conversion: item.10,
                });
            }
        }
        Err(())
    }
}

/// The order that the data is packed into the data structure for when it is ambiguous (e.g. when
/// the channels are packed into an int).
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RegularImageFormatOrder {
    R,
    D,
    RG,
    RX,
    XD,
    RGB,
    BGR,
    RGBA,
    BGRA,
    ARGB,
    ABGR,
    RXGX,
    RXGXBXAX,
}

/// This describes how the data gets converted when passed to the shader.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RegularImageFormatConversion {
    /// The data is stored as an integer given to the shaders as an integer directly.
    Int,

    /// The data is stored as an integer, and cast as a float, the value of which is equal to the
    /// value of the integer stored when passed to the shader.
    Scaled,

    /// The data is stored as an integer and cast as a float when passed to the shader.
    /// The value of the cast float is normalized to between 0 and 1 inclusively for an unsigned
    /// data type, and between -1 and 1 for a signed data type.
    Norm,

    /// The data is stored directly as a float type, and is passed as such to the shader.
    Float,

    /// The values are interpreted using sRGB non-linear encoding.
    /// Data with this type is always unsigned.
    SRGB,
}

// rustfmt::skip is used here to avoid cargo fmt from splitting every element over many
// lines, causing this code block to become incredibly long.
#[rustfmt::skip]
const REGULAR_IMAGE_FORMAT_CONVERSION_DATA: &[(
    vk::Format,
    u8, u8, u8, u8, u8, u8,
    bool, bool,
    RegularImageFormatOrder,
    RegularImageFormatConversion
)] = &[
    // Vulkan Version 1.0.
    (vk::Format::R4G4_UNORM_PACK8, 4, 4, 0, 0, 0, 0, false, true, RG, Norm),

    // Vulkan Version 1.0.
    (vk::Format::R4G4B4A4_UNORM_PACK16, 4, 4, 4, 4, 0, 0, false, true, RGBA, Norm),
    (vk::Format::B4G4R4A4_UNORM_PACK16, 4, 4, 4, 4, 0, 0, false, true, BGRA, Norm),
    // Vulkan Version 1.3.
    (vk::Format::A4R4G4B4_UNORM_PACK16, 4, 4, 4, 4, 0, 0, false, true, ARGB, Norm),
    (vk::Format::A4B4G4R4_UNORM_PACK16, 4, 4, 4, 4, 0, 0, false, true, ABGR, Norm),

    // Vulkan Version 1.0.
    (vk::Format::R5G6B5_UNORM_PACK16, 5, 6, 5, 0, 0, 0, false, true, RGB, Norm),
    (vk::Format::B5G6R5_UNORM_PACK16, 5, 6, 5, 0, 0, 0, false, true, BGR, Norm),

    // Vulkan Version 1.0.
    (vk::Format::R5G5B5A1_UNORM_PACK16, 5, 5, 5, 1, 0, 0, false, true, RGBA, Norm),
    (vk::Format::B5G5R5A1_UNORM_PACK16, 5, 5, 5, 1, 0, 0, false, true, BGRA, Norm),
    (vk::Format::A1R5G5B5_UNORM_PACK16, 5, 5, 5, 1, 0, 0, false, true, ARGB, Norm),

    // Vulkan Version 1.0.
    (vk::Format::R8_UNORM, 8, 0, 0, 0, 0, 0, false, false, R, Norm),
    (vk::Format::R8_SNORM, 8, 0, 0, 0, 0, 0, true, false, R, Norm),
    (vk::Format::R8_USCALED, 8, 0, 0, 0, 0, 0, false, false, R, Scaled),
    (vk::Format::R8_SSCALED, 8, 0, 0, 0, 0, 0, true, false, R, Scaled),
    (vk::Format::R8_UINT, 8, 0, 0, 0, 0, 0, false, false, R, Int),
    (vk::Format::R8_SINT, 8, 0, 0, 0, 0, 0, true, false, R, Int),
    (vk::Format::R8_SRGB, 8, 0, 0, 0, 0, 0, false, false, R, SRGB),

    // Vulkan Version 1.0.
    (vk::Format::R8G8_UNORM, 8, 8, 0, 0, 0, 0, false, false, RG, Norm),
    (vk::Format::R8G8_SNORM, 8, 8, 0, 0, 0, 0, true, false, RG, Norm),
    (vk::Format::R8G8_USCALED, 8, 8, 0, 0, 0, 0, false, false, RG, Scaled),
    (vk::Format::R8G8_SSCALED, 8, 8, 0, 0, 0, 0, true, false, RG, Scaled),
    (vk::Format::R8G8_UINT, 8, 8, 0, 0, 0, 0, false, false, RG, Int),
    (vk::Format::R8G8_SINT, 8, 8, 0, 0, 0, 0, true, false, RG, Int),
    (vk::Format::R8G8_SRGB, 8, 8, 0, 0, 0, 0, false, false, RG, SRGB),

    // Vulkan Version 1.0.
    (vk::Format::R8G8B8_UNORM, 8, 8, 8, 0, 0, 0, false, false, RGB, Norm),
    (vk::Format::R8G8B8_SNORM, 8, 8, 8, 0, 0, 0, true, false, RGB, Norm),
    (vk::Format::R8G8B8_USCALED, 8, 8, 8, 0, 0, 0, false, false, RGB, Scaled),
    (vk::Format::R8G8B8_SSCALED, 8, 8, 8, 0, 0, 0, true, false, RGB, Scaled),
    (vk::Format::R8G8B8_UINT, 8, 8, 8, 0, 0, 0, false, false, RGB, Int),
    (vk::Format::R8G8B8_SINT, 8, 8, 8, 0, 0, 0, true, false, RGB, Int),
    (vk::Format::R8G8B8_SRGB, 8, 8, 8, 0, 0, 0, false, false, RGB, SRGB),

    // Vulkan Version 1.0.
    (vk::Format::B8G8R8_UNORM, 8, 8, 8, 0, 0, 0, false, false, BGR, Norm),
    (vk::Format::B8G8R8_SNORM, 8, 8, 8, 0, 0, 0, true, false, BGR, Norm),
    (vk::Format::B8G8R8_USCALED, 8, 8, 8, 0, 0, 0, false, false, BGR, Scaled),
    (vk::Format::B8G8R8_SSCALED, 8, 8, 8, 0, 0, 0, true, false, BGR, Scaled),
    (vk::Format::B8G8R8_UINT, 8, 8, 8, 0, 0, 0, false, false, BGR, Int),
    (vk::Format::B8G8R8_SINT, 8, 8, 8, 0, 0, 0, true, false, BGR, Int),
    (vk::Format::B8G8R8_SRGB, 8, 8, 8, 0, 0, 0, false, false, BGR, SRGB),

    // Vulkan Version 1.0.
    (vk::Format::R8G8B8A8_UNORM, 8, 8, 8, 8, 0, 0, false, false, RGBA, Norm),
    (vk::Format::R8G8B8A8_SNORM, 8, 8, 8, 8, 0, 0, true, false, RGBA, Norm),
    (vk::Format::R8G8B8A8_USCALED, 8, 8, 8, 8, 0, 0, false, false, RGBA, Scaled),
    (vk::Format::R8G8B8A8_SSCALED, 8, 8, 8, 8, 0, 0, true, false, RGBA, Scaled),
    (vk::Format::R8G8B8A8_UINT, 8, 8, 8, 8, 0, 0, false, false, RGBA, Int),
    (vk::Format::R8G8B8A8_SINT, 8, 8, 8, 8, 0, 0, true, false, RGBA, Int),
    (vk::Format::R8G8B8A8_SRGB, 8, 8, 8, 8, 0, 0, false, false, RGBA, SRGB),

    // Vulkan Version 1.0.
    (vk::Format::B8G8R8A8_UNORM, 8, 8, 8, 8, 0, 0, false, false, BGRA, Norm),
    (vk::Format::B8G8R8A8_SNORM, 8, 8, 8, 8, 0, 0, true, false, BGRA, Norm),
    (vk::Format::B8G8R8A8_USCALED, 8, 8, 8, 8, 0, 0, false, false, BGRA, Scaled),
    (vk::Format::B8G8R8A8_SSCALED, 8, 8, 8, 8, 0, 0, true, false, BGRA, Scaled),
    (vk::Format::B8G8R8A8_UINT, 8, 8, 8, 8, 0, 0, false, false, BGRA, Int),
    (vk::Format::B8G8R8A8_SINT, 8, 8, 8, 8, 0, 0, true, false, BGRA, Int),
    (vk::Format::B8G8R8A8_SRGB, 8, 8, 8, 8, 0, 0, false, false, BGRA, SRGB),

    // Vulkan Version 1.0.
    (vk::Format::A8B8G8R8_UNORM_PACK32, 8, 8, 8, 8, 0, 0, false, true, ABGR, Norm),
    (vk::Format::A8B8G8R8_SNORM_PACK32, 8, 8, 8, 8, 0, 0, true, true, ABGR, Norm),
    (vk::Format::A8B8G8R8_USCALED_PACK32, 8, 8, 8, 8, 0, 0, false, true, ABGR, Scaled),
    (vk::Format::A8B8G8R8_SSCALED_PACK32, 8, 8, 8, 8, 0, 0, true, true, ABGR, Scaled),
    (vk::Format::A8B8G8R8_UINT_PACK32, 8, 8, 8, 8, 0, 0, false, true, ABGR, Int),
    (vk::Format::A8B8G8R8_SINT_PACK32, 8, 8, 8, 8, 0, 0, true, true, ABGR, Int),
    (vk::Format::A8B8G8R8_SRGB_PACK32, 8, 8, 8, 8, 0, 0, false, true, ABGR, SRGB),

    // Vulkan Version 1.0.
    (vk::Format::A2R10G10B10_UNORM_PACK32, 10, 10, 10, 2, 0, 0, false, true, ARGB, Norm),
    (vk::Format::A2R10G10B10_SNORM_PACK32, 10, 10, 10, 2, 0, 0, true, true, ARGB, Norm),
    (vk::Format::A2R10G10B10_USCALED_PACK32, 10, 10, 10, 2, 0, 0, false, true, ARGB, Scaled),
    (vk::Format::A2R10G10B10_SSCALED_PACK32, 10, 10, 10, 2, 0, 0, true, true, ARGB, Scaled),
    (vk::Format::A2R10G10B10_UINT_PACK32, 10, 10, 10, 2, 0, 0, false, true, ARGB, Int),
    (vk::Format::A2R10G10B10_SINT_PACK32, 10, 10, 10, 2, 0, 0, true, true, ARGB, Int),

    // Vulkan Version 1.0.
    (vk::Format::A2B10G10R10_UNORM_PACK32, 10, 10, 10, 2, 0, 0, false, true, ABGR, Norm),
    (vk::Format::A2B10G10R10_SNORM_PACK32, 10, 10, 10, 2, 0, 0, true, true, ABGR, Norm),
    (vk::Format::A2B10G10R10_USCALED_PACK32, 10, 10, 10, 2, 0, 0, false, true, ABGR, Scaled),
    (vk::Format::A2B10G10R10_SSCALED_PACK32, 10, 10, 10, 2, 0, 0, true, true, ABGR, Scaled),
    (vk::Format::A2B10G10R10_UINT_PACK32, 10, 10, 10, 2, 0, 0, false, true, ABGR, Int),
    (vk::Format::A2B10G10R10_SINT_PACK32, 10, 10, 10, 2, 0, 0, true, true, ABGR, Int),

    // Vulkan Version 1.0.
    (vk::Format::R16_UNORM, 16, 0, 0, 0, 0, 0, false, false, R, Norm),
    (vk::Format::R16_SNORM, 16, 0, 0, 0, 0, 0, true, false, R, Norm),
    (vk::Format::R16_USCALED, 16, 0, 0, 0, 0, 0, false, false, R, Scaled),
    (vk::Format::R16_SSCALED, 16, 0, 0, 0, 0, 0,true, false, R, Scaled),
    (vk::Format::R16_UINT, 16, 0, 0, 0, 0, 0, false, false, R, Int),
    (vk::Format::R16_SINT, 16, 0, 0, 0, 0, 0, true, false, R, Int),
    (vk::Format::R16_SFLOAT, 16, 0, 0, 0, 0, 0, true, false, R, Float),

    // Vulkan Version 1.0.
    (vk::Format::R16G16_UNORM, 16, 16, 0, 0, 0, 0, false, false, RG, Norm),
    (vk::Format::R16G16_SNORM, 16, 16, 0, 0, 0, 0, true, false, RG, Norm),
    (vk::Format::R16G16_USCALED, 16, 16, 0, 0, 0, 0, false, false, RG, Scaled),
    (vk::Format::R16G16_SSCALED, 16, 16, 0, 0, 0, 0, true, false, RG, Scaled),
    (vk::Format::R16G16_UINT, 16, 16, 0, 0, 0, 0, false, false, RG, Int),
    (vk::Format::R16G16_SINT, 16, 16, 0, 0, 0, 0, true, false, RG, Int),
    (vk::Format::R16G16_SFLOAT, 16, 16, 0, 0, 0, 0, true, false, RG, Float),

    // Vulkan Version 1.0.
    (vk::Format::R16G16B16_UNORM, 16, 16, 16, 0, 0, 0, false, false, RGB, Norm),
    (vk::Format::R16G16B16_SNORM, 16, 16, 16, 0, 0, 0, true, false, RGB, Norm),
    (vk::Format::R16G16B16_USCALED, 16, 16, 16, 0, 0, 0, false, false, RGB, Scaled),
    (vk::Format::R16G16B16_SSCALED, 16, 16, 16, 0, 0, 0, true, false, RGB, Scaled),
    (vk::Format::R16G16B16_UINT, 16, 16, 16, 0, 0, 0, false, false, RGB, Int),
    (vk::Format::R16G16B16_SINT, 16, 16, 16, 0, 0, 0, true, false, RGB, Int),
    (vk::Format::R16G16B16_SFLOAT, 16, 16, 16, 0, 0, 0, true, false, RGB, Float),

    // Vulkan Version 1.0.
    (vk::Format::R16G16B16A16_UNORM, 16, 16, 16, 16, 0, 0, false, false, RGBA, Norm),
    (vk::Format::R16G16B16A16_SNORM, 16, 16, 16, 16, 0, 0, true, false, RGBA, Norm),
    (vk::Format::R16G16B16A16_USCALED, 16, 16, 16, 16, 0, 0, false, false, RGBA, Scaled),
    (vk::Format::R16G16B16A16_SSCALED, 16, 16, 16, 16, 0, 0, true, false, RGBA, Scaled),
    (vk::Format::R16G16B16A16_UINT, 16, 16, 16, 16, 0, 0, false, false, RGBA, Int),
    (vk::Format::R16G16B16A16_SINT, 16, 16, 16, 16, 0, 0, true, false, RGBA, Int),
    (vk::Format::R16G16B16A16_SFLOAT, 16, 16, 16, 16, 0, 0, true, false, RGBA, Float),

    // Vulkan Version 1.0.
    (vk::Format::R32_UINT, 32, 0, 0, 0, 0, 0, false, false, R, Int),
    (vk::Format::R32_SINT, 32, 0, 0, 0, 0, 0, true, false, R, Int),
    (vk::Format::R32_SFLOAT, 32, 0, 0, 0, 0, 0, true, false, R, Float),

    // Vulkan Version 1.0.
    (vk::Format::R32G32_UINT, 32, 32, 0, 0, 0, 0, false, false, RG, Int),
    (vk::Format::R32G32_SINT, 32, 32, 0, 0, 0, 0, true, false, RG, Int),
    (vk::Format::R32G32_SFLOAT, 32, 32, 0, 0, 0, 0, true, false, RG, Float),

    // Vulkan Version 1.0.
    (vk::Format::R32G32B32_UINT, 32, 32, 32, 0, 0, 0, false, false, RGB, Int),
    (vk::Format::R32G32B32_SINT, 32, 32, 32, 0, 0, 0, true, false, RGB, Int),
    (vk::Format::R32G32B32_SFLOAT, 32, 32, 32, 0, 0, 0, true, false, RGB, Float),

    // Vulkan Version 1.0.
    (vk::Format::R32G32B32A32_UINT, 32, 32, 32, 32, 0, 0, false, false, RGBA, Int),
    (vk::Format::R32G32B32A32_SINT, 32, 32, 32, 32, 0, 0, true, false, RGBA, Int),
    (vk::Format::R32G32B32A32_SFLOAT, 32, 32, 32, 32, 0, 0, true, false, RGBA, Float),

    // Vulkan Version 1.0.
    (vk::Format::R64_UINT, 64, 0, 0, 0, 0, 0, false, false, R, Int),
    (vk::Format::R64_SINT, 64, 0, 0, 0, 0, 0, true, false, R, Int),
    (vk::Format::R64_SFLOAT, 64, 0, 0, 0, 0, 0, true, false, R, Float),

    // Vulkan Version 1.0.
    (vk::Format::R64G64_UINT, 64, 64, 0, 0, 0, 0, false, false, RG, Int),
    (vk::Format::R64G64_SINT, 64, 64, 0, 0, 0, 0, true, false, RG, Int),
    (vk::Format::R64G64_SFLOAT, 64, 64, 0, 0, 0, 0, true, false, RG, Float),

    // Vulkan Version 1.0.
    (vk::Format::R64G64B64_UINT, 64, 64, 64, 0, 0, 0, false, false, RGB, Int),
    (vk::Format::R64G64B64_SINT, 64, 64, 64, 0, 0, 0, true, false, RGB, Int),
    (vk::Format::R64G64B64_SFLOAT, 64, 64, 64, 0, 0, 0, true, false, RGB, Float),

    // Vulkan Version 1.0.
    (vk::Format::R64G64B64A64_UINT, 64, 64, 64, 64, 0, 0, false, false, RGBA, Int),
    (vk::Format::R64G64B64A64_SINT, 64, 64, 64, 64, 0, 0, true, false, RGBA, Int),
    (vk::Format::R64G64B64A64_SFLOAT, 64, 64, 64, 64, 0, 0, true, false, RGBA, Float),

    // Vulkan Version 1.0.
    (vk::Format::B10G11R11_UFLOAT_PACK32, 11, 11, 10, 0, 0, 0, false, true, BGR, Float),

    // Vulkan Version 1.0.
    (vk::Format::D16_UNORM, 0, 0, 0, 0, 16, 0, false, false, D, Norm),
    (vk::Format::X8_D24_UNORM_PACK32, 0, 0, 0, 0, 24, 8, false, true, XD, Norm),
    (vk::Format::D32_SFLOAT, 0, 0, 0, 0, 32, 0, true, false, D, Float),

    // Vulkan Version 1.1.
    (vk::Format::R10X6_UNORM_PACK16, 10, 0, 0, 0, 0, 6, false, true, RX, Norm),
    (vk::Format::R10X6G10X6_UNORM_2PACK16, 10, 10, 0, 0, 0, 12, false, true, RXGX, Norm),
    (
        vk::Format::R10X6G10X6B10X6A10X6_UNORM_4PACK16,
        10, 10, 10, 10, 0, 24,
        false,
        true,
        RXGXBXAX,
        Norm,
    ),

    // Vulkan Version 1.1.
    (vk::Format::R12X4_UNORM_PACK16, 12, 0, 0, 0, 0, 4, false, true, RX, Norm),
    (vk::Format::R12X4G12X4_UNORM_2PACK16, 12, 12, 0, 0, 0, 8, false, true, RXGX, Norm),
    (
        vk::Format::R12X4G12X4B12X4A12X4_UNORM_4PACK16,
        12, 12, 12, 12, 0, 16,
        false,
        true,
        RXGXBXAX,
        Norm,
    ),
];
