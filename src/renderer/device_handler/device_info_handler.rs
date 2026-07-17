//! This module is used for translating information from the ash style information the library
//! receives into useful information for the library user.
//! The module also contains structs helpful for setting up the device.
mod temp;

use crate::renderer::Size2D;
use ash::vk;

use crate::renderer::device_info_handler::temp::REGULAR_IMAGE_FORMAT_CONVERSION_DATA;


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

    /// The presentation modes that are available.
    pub presentation_modes: Vec<PresentationMode>,
}

impl TryFrom<&VulkanDisplayInfo> for DeviceInfo {
    type Error = TryIntoDeviceInfoError;

    fn try_from(value: &VulkanDisplayInfo) -> Result<Self, Self::Error> {
        Ok(Self {
            capabilities: value.capabilities.try_into()?,
            available_formats: value
                .formats
                .clone()
                .into_iter()
                .filter_map(|f| Format::try_from(&f).ok())
                .collect(),
            presentation_modes: value
                .presentation_modes
                .clone()
                .into_iter()
                .filter_map(|p| PresentationMode::try_from(p).ok())
                .collect(),
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TryIntoDeviceInfoError {
    CapabilitiesConversionError(TryIntoCapabilitiesError),
}

impl From<TryIntoCapabilitiesError> for TryIntoDeviceInfoError {
    fn from(value: TryIntoCapabilitiesError) -> Self {
        TryIntoDeviceInfoError::CapabilitiesConversionError(value)
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
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Capabilities {
    /// The minimum number of images that can be in the swapchain.
    pub min_swapchain_image_count: u32,

    /// The maximum number of images that can be in the swapchain.
    /// If the value is None, then there is no limit on the number of images, though there may be
    /// limits on the amount of memory used by presentable images.
    pub max_swapchain_image_count: Option<u32>,

    /// The current size of the image size to be displayed in pixels.
    /// A None for current_image_size is a special value meaning that the size of the image depends
    /// on the size of the provided swapchain.
    /// If it exists, this will be greater than or equal to min_swapchain_image_size in x and y
    pub current_image_size: Option<Size2D>,

    /// The minimum size of the swapchain in pixels.
    pub min_swapchain_image_size: Size2D,

    /// The maximum size of the swapchain in pixels.
    /// This will be greater than or equal to current_image_size in x and y current_image_size
    /// exists, and will be greater than or equal to min_swapchain_image_size in x any y.
    /// This is checked against min_swapchain_image_size before current_image_size.
    pub max_swapchain_image_size: Size2D,

    /// The maximum number of layers of the image to be rendered to.
    /// This will be at least one
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

impl TryFrom<vk::SurfaceCapabilitiesKHR> for Capabilities {
    type Error = TryIntoCapabilitiesError;

    fn try_from(value: vk::SurfaceCapabilitiesKHR) -> Result<Self, Self::Error> {
        let result = Self {
            min_swapchain_image_count: value.min_image_count,
            max_swapchain_image_count: match value.max_image_count {
                0 => None,
                val => Some(val),
            },
            current_image_size: match value.current_extent {
                vk::Extent2D {
                    width: 0xffffffff,
                    height: 0xffffffff,
                } => None,
                value => Some(value.into()),
            },
            min_swapchain_image_size: value.min_image_extent.into(),
            max_swapchain_image_size: value.max_image_extent.into(),
            max_number_of_image_layers: value.max_image_array_layers,
            supported_image_transformations: match value.supported_transforms.try_into() {
                Ok(val) => val,
                Err(()) => {
                    return Err(TryIntoCapabilitiesError::FailedToConvertSupportedTransformations);
                }
            },
            current_image_transformations: match value.current_transform.try_into() {
                Ok(val) => val,
                Err(()) => {
                    return Err(TryIntoCapabilitiesError::FailedToConvertCurrentTransformations);
                }
            },
            supported_alpha_compositing_modes: match value.supported_composite_alpha.try_into() {
                Ok(val) => val,
                Err(()) => return Err(TryIntoCapabilitiesError::FailedToConvertCompositeAlpha),
            },
            supported_image_usages: value.supported_usage_flags.into(),
        };
        if let Some(max) = result.max_swapchain_image_count {
            if result.min_swapchain_image_count > max {
                return Err(TryIntoCapabilitiesError::MinSwapchainImageCountGreaterThanMax);
            }
        }
        if !result
            .min_swapchain_image_size
            .fits_inside(&result.max_swapchain_image_size)
        {
            return Err(TryIntoCapabilitiesError::MinSwapchainImageSizeDoesNotFitInMax);
        }
        if let Some(val) = result.current_image_size {
            if !result.min_swapchain_image_size.fits_inside(&val) {
                return Err(TryIntoCapabilitiesError::MinSwapchainImageSizeDoesNotFitInCurrent);
            }
            if !val.fits_inside(&result.max_swapchain_image_size) {
                return Err(TryIntoCapabilitiesError::CurrentSwapchainImageSizeDoesNotFitInMax);
            }
        }
        if result.max_number_of_image_layers < 1 {
            return Err(TryIntoCapabilitiesError::MaxNumberOfImageLayersLessThanOne);
        }
        Ok(result)
    }
}

/// An image format and colour space pair.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Format {
    /// The format that the image will use.
    pub image_format: ImageFormat,
    pub colour_space: ColourSpace,
}

impl TryFrom<&vk::SurfaceFormatKHR> for Format {
    type Error = FormatConversionError;

    fn try_from(value: &vk::SurfaceFormatKHR) -> Result<Self, Self::Error> {
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

/// A way that the swapchain images are presented to the screen
/// The only option that is required to be supported is FIFO.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PresentationMode {
    /// When a new image is ready, it is presented immediately, and does not wait for the vertical
    /// blanking period to update, which may result in visible tearing.
    /// No internal queuing of presentation requests is needed.
    Immediate,

    /// When a new image is waiting, the image waits for the next vertical blanking period.
    /// This prevents any tearing.
    /// Internally, a queue sized to fit one image is used to hold the presentation requests.
    /// Any new entries to the queue when the queue is full, the new entry replaces the one already
    /// in the queue, making any images associated with the prior entry available for reuse.
    /// One request is removed from the queue whenever the queue is not empty for every vertical
    /// blanking interval.
    Mailbox,

    /// The images are held in a queue, each vertical blanking period, the image from the front of
    /// the queue is presented.
    /// One request is removed from the queue whenever the queue is not empty for every vertical
    /// blanking interval.
    FIFO,

    /// The images are held in a queue, and new ones are added to the end.
    /// The vulkan presentation engine will generally wait for the next vertical blanking period to
    /// update the current image.
    /// If the previous vertical blanking period passed with no new image in the queue to present,
    /// then the next image to reach the queue will be presented immediately.
    /// This will reduce potential stuttering, but may result in some visible tearing.
    FIFORelaxed,
}

impl TryFrom<vk::PresentModeKHR> for PresentationMode {
    type Error = vk::PresentModeKHR;

    fn try_from(value: vk::PresentModeKHR) -> Result<Self, Self::Error> {
        match value {
            vk::PresentModeKHR::IMMEDIATE => Ok(PresentationMode::Immediate),
            vk::PresentModeKHR::MAILBOX => Ok(PresentationMode::Mailbox),
            vk::PresentModeKHR::FIFO => Ok(PresentationMode::FIFO),
            vk::PresentModeKHR::FIFO_RELAXED => Ok(PresentationMode::FIFORelaxed),

            // Not Supported
            vk::PresentModeKHR::SHARED_DEMAND_REFRESH => Err(value),
            vk::PresentModeKHR::SHARED_CONTINUOUS_REFRESH => Err(value),

            _ => Err(value),
        }
    }
}

/// Gives the error that occurred when converting vk::SurfaceCapabilities to Capabilities.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TryIntoCapabilitiesError {
    FailedToConvertSupportedTransformations,
    FailedToConvertCurrentTransformations,
    FailedToConvertCompositeAlpha,
    MinSwapchainImageCountGreaterThanMax,
    MinSwapchainImageSizeDoesNotFitInMax,
    MinSwapchainImageSizeDoesNotFitInCurrent,
    CurrentSwapchainImageSizeDoesNotFitInMax,
    MaxNumberOfImageLayersLessThanOne,
}

/// Stores transformations of the image.
/// At least one option will always be true.
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

impl TryFrom<vk::SurfaceTransformFlagsKHR> for ImageTransformations {
    type Error = ();

    fn try_from(value: vk::SurfaceTransformFlagsKHR) -> Result<Self, Self::Error> {
        let result = Self {
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
        };
        if !(result.identity
            || result.rotate_90_degrees
            || result.rotate_180_degrees
            || result.rotate_270_degrees
            || result.mirror
            || result.mirror_and_rotate_90_degrees
            || result.mirror_and_rotate_180_degrees
            || result.mirror_and_rotate_270_degrees
            || result.inherit)
        {
            return Err(());
        }
        Ok(result)
    }
}

/// Stores how the alpha component of the image is determined.
/// If the image does not have an alpha component, these settings will not do anything (except
/// potentially inherit depending on the specific system).
/// At least one option will always be true.
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

impl TryFrom<vk::CompositeAlphaFlagsKHR> for AlphaCompositingModes {
    type Error = ();

    fn try_from(value: vk::CompositeAlphaFlagsKHR) -> Result<Self, Self::Error> {
        let result = Self {
            opaque: value.contains(vk::CompositeAlphaFlagsKHR::OPAQUE),
            pre_multiplied: value.contains(vk::CompositeAlphaFlagsKHR::PRE_MULTIPLIED),
            post_multiplied: value.contains(vk::CompositeAlphaFlagsKHR::POST_MULTIPLIED),
            inherit: value.contains(vk::CompositeAlphaFlagsKHR::INHERIT),
        };
        if !(result.opaque || result.pre_multiplied || result.post_multiplied || result.inherit) {
            return Err(());
        }
        Ok(result)
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
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

    /// The unused bits of the structure.
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
    pub packed: Option<u8>,

    /// Stores the order that the data is packed into the bitstring in.
    pub order: RegularImageFormatOrder,

    /// The way that the data is converted when passed to the shader.
    pub data_conversion: RegularImageFormatConversion,
}

impl TryFrom<vk::Format> for RegularImageFormat {
    type Error = ();

    //TODO fix
    fn try_from(value: vk::Format) -> Result<Self, Self::Error> {
        /*
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
        */
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::renderer::{debugger::tests, device_handler};
    use RegularImageFormatConversion::{Float, Int, Norm, Scaled, SRGB};
    use RegularImageFormatOrder::{ABGR, ARGB, BGR, BGRA, D, R, RG, RGB, RGBA, RX, RXGX, RXGXBXAX};

    #[test]
    fn can_get_device_info() {
        let into_device_info_test_data = &[
            (
                VulkanDisplayInfo {
                    capabilities: INTO_CAPABILITIES_TEST_DATA[0].0,
                    formats: vec![INTO_FORMAT_TEST_DATA[0].0],
                    presentation_modes: vec![vk::PresentModeKHR::FIFO],
                },
                Ok(DeviceInfo {
                    capabilities: INTO_CAPABILITIES_TEST_DATA[0].1.clone().unwrap(),
                    available_formats: vec![INTO_FORMAT_TEST_DATA[0].1.unwrap()],
                    presentation_modes: vec![PresentationMode::FIFO],
                }),
            ),
            (
                VulkanDisplayInfo {
                    capabilities: INTO_CAPABILITIES_TEST_DATA[2].0,
                    formats: vec![INTO_FORMAT_TEST_DATA[0].0],
                    presentation_modes: vec![vk::PresentModeKHR::FIFO],
                },
                Err(TryIntoDeviceInfoError::CapabilitiesConversionError(
                    TryIntoCapabilitiesError::MinSwapchainImageCountGreaterThanMax,
                )),
            ),
        ];
        for test in into_device_info_test_data {
            assert_eq!(DeviceInfo::try_from(&test.0), test.1)
        }
    }

    #[test]
    fn can_use_getters_and_setters_in_logical_device_settings() {
        let (_guard, vulkan_entry, mut glfw_instance) = tests::get_entries();
        let mut vulkan_instance = tests::get_vulkan_instance(&vulkan_entry, &glfw_instance);
        let surface_instance = tests::get_surface_instance(&vulkan_entry, &vulkan_instance);
        let window = tests::get_window(&mut glfw_instance);
        let surface = tests::get_window_surface(&mut vulkan_instance, &window);
        let device = device_handler::tests::get_physical_device(
            &vulkan_instance,
            &surface_instance,
            surface,
        );
        let device_info =
            device_handler::tests::get_device_display_info(&surface_instance, device, surface);
        let new_physical_device = PhysicalDevice {
            device_info: DeviceInfo {
                capabilities: Capabilities::try_from(device_info.capabilities).unwrap(),
                available_formats: Vec::new(),
                presentation_modes: Vec::new(),
            },
            device,
        };
        let logical_device_settings = LogicalDeviceSettings {
            physical_device: PhysicalDevice {
                device_info: DeviceInfo::try_from(&device_info).unwrap(),
                device,
            },
            num_swap_frames: 2,
        }
        .with_num_swap_frames(3)
        .with_physical_device(new_physical_device.clone());
        assert_eq!(logical_device_settings.get_num_swap_frames(), 3);
        assert_eq!(
            logical_device_settings.get_physical_device(),
            &new_physical_device
        );
    }

    #[test]
    fn can_get_capabilities() {
        for test in INTO_CAPABILITIES_TEST_DATA {
            assert_eq!(Capabilities::try_from(test.0), test.1)
        }
    }

    #[test]
    fn test_try_into_format() {
        for test in INTO_FORMAT_TEST_DATA {
            assert_eq!(Format::try_from(&test.0), test.1)
        }
    }

    #[test]
    fn test_try_into_presentation_mode() {
        // We use rustfmt::skip to prevent the formatter from splitting this across a massive amount
        // of lines.
        #[rustfmt::skip]
        let test_data = &[
            (vk::PresentModeKHR::IMMEDIATE, Ok(PresentationMode::Immediate)),
            (vk::PresentModeKHR::MAILBOX, Ok(PresentationMode::Mailbox)),
            (vk::PresentModeKHR::FIFO, Ok(PresentationMode::FIFO)),
            (vk::PresentModeKHR::FIFO_RELAXED, Ok(PresentationMode::FIFORelaxed)),
            (
                vk::PresentModeKHR::SHARED_DEMAND_REFRESH,
                Err(vk::PresentModeKHR::SHARED_DEMAND_REFRESH)
            ),
            (
                vk::PresentModeKHR::SHARED_CONTINUOUS_REFRESH,
                Err(vk::PresentModeKHR::SHARED_CONTINUOUS_REFRESH)
            ),
            (vk::PresentModeKHR::from_raw(-1), Err(vk::PresentModeKHR::from_raw(-1))),
        ];
        for test in test_data {
            assert_eq!(PresentationMode::try_from(test.0), test.1)
        }
    }

    #[test]
    fn test_into_image_transformations() {
        for test in INTO_TRANSFORMATION_TEST_DATA {
            if test.1 {
                assert_eq!(
                    ImageTransformations::try_from(test.0),
                    Ok(ImageTransformations {
                        identity: test.2,
                        rotate_90_degrees: test.3,
                        rotate_180_degrees: test.4,
                        rotate_270_degrees: test.5,
                        mirror: test.6,
                        mirror_and_rotate_90_degrees: test.7,
                        mirror_and_rotate_180_degrees: test.8,
                        mirror_and_rotate_270_degrees: test.9,
                        inherit: test.10,
                    })
                )
            } else {
                assert_eq!(ImageTransformations::try_from(test.0), Err(()))
            }
        }
    }

    #[test]
    fn test_into_alpha_compositing_modes() {
        // We use rustfmt::skip to prevent the formatter from splitting this across a massive amount
        // of lines.
        #[rustfmt::skip]
        let test_data = &[
            (vk::CompositeAlphaFlagsKHR::OPAQUE, true, true, false, false, false),
            (vk::CompositeAlphaFlagsKHR::PRE_MULTIPLIED, true, false, true, false, false),
            (vk::CompositeAlphaFlagsKHR::POST_MULTIPLIED, true, false, false, true, false),
            (vk::CompositeAlphaFlagsKHR::INHERIT, true, false, false, false, true),
            (vk::CompositeAlphaFlagsKHR::from_raw(0xffffffff), true, true, true, true, true),
            (vk::CompositeAlphaFlagsKHR::from_raw(0), false, false, false, false, false),
        ];
        for test in test_data {
            if test.1 {
                assert_eq!(
                    AlphaCompositingModes::try_from(test.0),
                    Ok(AlphaCompositingModes {
                        opaque: test.2,
                        pre_multiplied: test.3,
                        post_multiplied: test.4,
                        inherit: test.5,
                    })
                )
            } else {
                assert_eq!(AlphaCompositingModes::try_from(test.0), Err(()))
            }
        }
    }

    #[test]
    fn test_into_image_usages() {
        // WHEN UPDATING THIS ADD TESTS FOR THE SPECIFIC ITEMS IN IMAGE USAGES TO TEST EACH
        // INDIVIDUAL FLAGS.
        // Use something like the test above.
        assert_eq!(
            ImageUsages {},
            ImageUsages::from(vk::ImageUsageFlags::from_raw(0xffffffff))
        )
    }

    #[test]
    fn test_try_into_colour_spaces() {
        // We use rustfmt::skip to prevent the formatter from splitting this across a massive amount
        // of lines.
        #[rustfmt::skip]
        let test_data = &[
            (vk::ColorSpaceKHR::SRGB_NONLINEAR, Ok(ColourSpace::NonLinearSRGB)),
            (vk::ColorSpaceKHR::DISPLAY_P3_NONLINEAR_EXT, Ok(ColourSpace::NonLinearDisplayP3)),
            (vk::ColorSpaceKHR::EXTENDED_SRGB_LINEAR_EXT, Ok(ColourSpace::LinearExtendedSRGB)),
            (
                vk::ColorSpaceKHR::EXTENDED_SRGB_NONLINEAR_EXT,
                Ok(ColourSpace::NonLinearExtendedSRGB)
            ),
            (vk::ColorSpaceKHR::DISPLAY_P3_LINEAR_EXT, Ok(ColourSpace::LinearDisplayP3)),
            (vk::ColorSpaceKHR::DCI_P3_NONLINEAR_EXT, Ok(ColourSpace::NonLinearDCIP3)),
            (vk::ColorSpaceKHR::BT709_LINEAR_EXT, Ok(ColourSpace::LinearBT709)),
            (vk::ColorSpaceKHR::BT709_NONLINEAR_EXT, Ok(ColourSpace::NonLinearBT709)),
            (vk::ColorSpaceKHR::BT2020_LINEAR_EXT, Ok(ColourSpace::LinearBT2020)),
            (vk::ColorSpaceKHR::HDR10_ST2084_EXT, Ok(ColourSpace::NonLinearBT2020UsingST2084)),
            (vk::ColorSpaceKHR::HDR10_HLG_EXT, Ok(ColourSpace::NonLinearBT2020UsingHLG)),
            (vk::ColorSpaceKHR::ADOBERGB_LINEAR_EXT, Ok(ColourSpace::LinearAdobeRGB)),
            (vk::ColorSpaceKHR::ADOBERGB_NONLINEAR_EXT, Ok(ColourSpace::NonLinearAdobeRGB)),
            (vk::ColorSpaceKHR::PASS_THROUGH_EXT, Ok(ColourSpace::PassThrough)),
            (vk::ColorSpaceKHR::DISPLAY_NATIVE_AMD, Ok(ColourSpace::Native)),
            (vk::ColorSpaceKHR::DOLBYVISION_EXT, Err(())),
            (vk::ColorSpaceKHR::from_raw(-1), Err(())),
        ];
        for test in test_data {
            assert_eq!(ColourSpace::try_from(test.0), test.1)
        }
    }

    #[test]
    fn can_create_image_format_using_regular_image_format() {
        let result = ImageFormat::try_from(vk::Format::R4G4_UNORM_PACK8);
        assert_eq!(
            result,
            Ok(ImageFormat::RegularFormat(RegularImageFormat {
                red_channel: 4,
                green_channel: 4,
                blue_channel: 0,
                alpha_channel: 0,
                depth_channel: 0,
                unused_bits: 0,
                signed: false,
                packed: Some(8),
                order: RG,
                data_conversion: Norm,
            })),
        )
    }

    #[test]
    fn cannot_create_image_format_from_invalid_format() {
        assert_eq!(ImageFormat::try_from(vk::Format::from_raw(-1)), Err(()));
    }

    #[test]
    fn cannot_create_image_format_from_undefined_format() {
        assert_eq!(ImageFormat::try_from(vk::Format::UNDEFINED), Err(()));
    }

    #[test]
    fn cannot_create_image_format_from_unimplemented_format() {
        assert_eq!(
            ImageFormat::try_from(vk::Format::EAC_R11G11_UNORM_BLOCK),
            Err(())
        );
    }

    #[test]
    fn test_try_into_image_format() {
        for test in INTO_IMAGE_FORMAT_TEST_DATA {
            assert_eq!(ImageFormat::try_from(test.0), test.1);
        }
    }

    #[test]
    fn test_try_into_regular_image_format() {
        let last = REGULAR_IMAGE_FORMAT_CONVERSION_DATA.len() - 1;

        // We use rustfmt::skip to prevent the formatter from splitting this across a massive amount
        // of lines.
        // Data format is (vk::Format, passes, red, green, blue, alpha, depth, unused, signed,
        // packed, order, conversion).
        #[rustfmt::skip]
        let test_data = &[
            (vk::Format::R8_SNORM, true, 8, 0, 0, 0, 0, 0, true, None, R, Norm),
            (
                vk::Format::R5G6B5_UNORM_PACK16,
                true, 5, 6, 5, 0, 0, 0, false, Some(16), RGB, Norm,
            ),
            (
                vk::Format::R10X6G10X6B10X6A10X6_UNORM_4PACK16,
                true, 10, 10, 10, 10, 0, 24, false, Some(16), RXGXBXAX, Norm,
            ),
            (vk::Format::D32_SFLOAT, true, 0, 0, 0, 0, 32, 0, true, None, D, Float),
            //TODO Fix
            //(REGULAR_IMAGE_FORMAT_CONVERSION_DATA[0].0,true, 4, 4, 0, 0, 0, 0, false, true, RG, Norm),
            //(REGULAR_IMAGE_FORMAT_CONVERSION_DATA[last].0,true, 12, 12, 12, 12, 0, 16, false, true, RXGXBXAX, Norm,),
            (
                vk::Format::R12X4_UNORM_PACK16_KHR,
                true, 12, 0, 0, 0, 0, 4, false, Some(16), RX, Norm,
            ),
            (vk::Format::from_raw(-1), false, 0, 0, 0, 0, 0, 0, false, None, R, Int),
            (vk::Format::UNDEFINED, false, 0, 0, 0, 0, 0, 0, false, None, R, Int),
            (
                vk::Format::EAC_R11G11_UNORM_BLOCK,
                false, 0, 0, 0, 0, 0, 0, false, None, R, Int,
            ),
        ];
        for test in test_data {
            if test.1 {
                assert_eq!(
                    RegularImageFormat::try_from(test.0),
                    Ok(RegularImageFormat {
                        red_channel: test.2,
                        green_channel: test.3,
                        blue_channel: test.4,
                        alpha_channel: test.5,
                        depth_channel: test.6,
                        unused_bits: test.7,
                        signed: test.8,
                        packed: test.9,
                        order: test.10,
                        data_conversion: test.11,
                    }),
                )
            } else {
                assert_eq!(RegularImageFormat::try_from(test.0), Err(()))
            }
        }
    }

    #[test]
    fn regular_image_format_conversion_data_correct() {
        for format in REGULAR_IMAGE_FORMAT_CONVERSION_DATA {
            let correct = format.0;
            let correct_data = get_regular_image_format_stats(format!("{correct:?}").as_str())
                .unwrap();
            assert_eq!(
                *format,
                (
                    correct,
                    correct_data.0[0],
                    correct_data.0[1],
                    correct_data.0[2],
                    correct_data.0[3],
                    correct_data.0[4],
                    correct_data.0[5],
                    correct_data.1,
                    correct_data.2,
                    correct_data.3,
                    correct_data.4,
                )
            );
        }
    }

    /// We did not use this for the actual code, as a change to how debug works for vk::Format,
    /// which would be considered a non-breaking change, would break our code.
    /// It is fine here as worst case these tests stops working and get fixed with no impact on
    /// actual prod code.
    /// Will panic if the number of bits in any one channel exceeds the max value of u8.
    fn get_regular_image_format_stats(
        name: &str,
    ) -> Option<(
        [u8; 6],
        bool,
        bool,
        RegularImageFormatOrder,
        RegularImageFormatConversion,
    )> {
        let mut words = name.split('_');

        let mut channels = [0, 0, 0, 0, 0, 0];
        let mut first_word = words.next().unwrap().chars().peekable();
        let mut format_order_string = String::new();
        loop {
            match first_word.next() {
                Some('R') => {
                    channels[0] += get_number_from_char_iterator(&mut first_word);
                    format_order_string.push('R')
                }
                Some('G') => {
                    channels[1] += get_number_from_char_iterator(&mut first_word);
                    format_order_string.push('G')
                }
                Some('B') => {
                    channels[2] += get_number_from_char_iterator(&mut first_word);
                    format_order_string.push('B')
                }
                Some('A') => {
                    channels[3] += get_number_from_char_iterator(&mut first_word);
                    format_order_string.push('A')
                }
                Some('D') => {
                    channels[4] += get_number_from_char_iterator(&mut first_word);
                    format_order_string.push('D')
                }
                Some('X') => {
                    channels[5] += get_number_from_char_iterator(&mut first_word);
                    format_order_string.push('X')
                }
                Some(_) => return None,
                None => break,
            }
        }

        let order = match format_order_string.as_str() {
            "R" => R,
            "D" => D,
            "RG" => RG,
            "RX" => RX,
            "RGB" => RGB,
            "BGR" => BGR,
            "RGBA" => RGBA,
            "BGRA" => BGRA,
            "ARGB" => ARGB,
            "ABGR" => ABGR,
            "RXGX" => RXGX,
            "RXGXBXAX" => RXGXBXAX,
            _ => return None,
        };

        let (signed, conversion) = match words.next()? {
            "UINT" => (false, Int),
            "SINT" => (true, Int),
            "USCALED" => (false, Scaled),
            "SSCALED" => (true, Scaled),
            "UNORM" => (false, Norm),
            "SNORM" => (true, Norm),
            "UFLOAT" => (false, Float),
            "SFLOAT" => (true, Float),
            "SRGB" => (false, SRGB),
            _ => return None,
        };

        let packed = match words.next() {
            Some("PACK8") => true,
            Some("PACK16") => true,
            Some("2PACK16") => true,
            Some("4PACK16") => true,
            Some("PACK32") => true,
            Some(_) => return None,
            None => false,
        };

        if words.next() != None {
            return None;
        }

        Some((channels, signed, packed, order, conversion))
    }

    #[test]
    fn test_get_regular_image_format_stats() {
        let test_data = &[
            ("R8_SINT", Some(([8, 0, 0, 0, 0, 0], true, false, R, Int))),
            (
                "R1X9G2X9B3X9A4X9_SRGB_PACK32",
                Some(([1, 2, 3, 4, 0, 36], false, true, RXGXBXAX, SRGB))
            ),
            ("D8_SFLOAT", Some(([0, 0, 0, 0, 8, 0], true, false, D,  Float))),
            ("", None),
            ("R", None),
            ("[", None),
            ("RA", None),
            ("R_", None),
            ("_", None),
            ("R8_HE", None),
            ("R8_UINT_", None),
            ("R8_UINT_1", None),
            ("R8_UINT_PACK8_", None),
            ("R8_UINT_PACK8_H", None),
        ];
        for test in test_data {
            assert_eq!(get_regular_image_format_stats(test.0), test.1);
        }
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn getting_regular_image_statistics_fails_on_256() {
        get_regular_image_format_stats("R256_SINT");
    }

    /// Will panic if the number exceeds the max value of u8.
    fn get_number_from_char_iterator(chars: &mut std::iter::Peekable<std::str::Chars>) -> u8 {
        let mut val: u8 = 0;
        loop {
            match chars.peek() {
                None => return val,
                Some(c) => {
                    if c.is_digit(10) {
                        val = val * 10 + c.to_digit(10).unwrap() as u8
                    } else {
                        return val;
                    }
                }
            }
            chars.next();
        }
    }

    #[test]
    fn test_getting_number_from_char_iterator() {
        let test_data = &[
            ("", 0, ""),
            ("_", 0, "_"),
            ("A", 0, "A"),
            ("0", 0, ""),
            ("1", 1, ""),
            ("1A", 1, "A"),
            ("A1", 0, "A1"),
            ("00", 0, ""),
            ("01", 1, ""),
            ("10", 10, ""),
            ("100", 100, ""),
            ("255", 255, ""),
            ("255A", 255, "A"),
            ("25A_5", 25, "A_5"),
            ("000255A", 255, "A"),
        ];
        for test in test_data {
            let mut iter = test.0.chars().peekable();
            assert_eq!(get_number_from_char_iterator(&mut iter), test.1);
            let remaining: String = iter.collect();
            assert_eq!(remaining, test.2);
        }
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn getting_number_from_char_iterator_fails_on_256() {
        get_number_from_char_iterator(&mut "256".chars().peekable());
    }

    const INTO_CAPABILITIES_TEST_DATA: &[(
        vk::SurfaceCapabilitiesKHR,
        Result<Capabilities, TryIntoCapabilitiesError>,
    )] = &[
        (
            vk::SurfaceCapabilitiesKHR {
                min_image_count: 1,
                max_image_count: 2,
                current_extent: vk::Extent2D {
                    width: 0xffffffff,
                    height: 0xffffffff,
                },
                min_image_extent: vk::Extent2D {
                    width: 3,
                    height: 4,
                },
                max_image_extent: vk::Extent2D {
                    width: 7,
                    height: 8,
                },
                max_image_array_layers: 9,
                supported_transforms: vk::SurfaceTransformFlagsKHR::IDENTITY,
                current_transform: vk::SurfaceTransformFlagsKHR::INHERIT,
                supported_composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
                supported_usage_flags: vk::ImageUsageFlags::from_raw(0xffffffff),
            },
            Ok(Capabilities {
                min_swapchain_image_count: 1,
                max_swapchain_image_count: Some(2),
                current_image_size: None,
                min_swapchain_image_size: Size2D { x: 3, y: 4 },
                max_swapchain_image_size: Size2D { x: 7, y: 8 },
                max_number_of_image_layers: 9,
                supported_image_transformations: ImageTransformations {
                    identity: true,
                    rotate_90_degrees: false,
                    rotate_180_degrees: false,
                    rotate_270_degrees: false,
                    mirror: false,
                    mirror_and_rotate_90_degrees: false,
                    mirror_and_rotate_180_degrees: false,
                    mirror_and_rotate_270_degrees: false,
                    inherit: false,
                },
                current_image_transformations: ImageTransformations {
                    identity: false,
                    rotate_90_degrees: false,
                    rotate_180_degrees: false,
                    rotate_270_degrees: false,
                    mirror: false,
                    mirror_and_rotate_90_degrees: false,
                    mirror_and_rotate_180_degrees: false,
                    mirror_and_rotate_270_degrees: false,
                    inherit: true,
                },
                supported_alpha_compositing_modes: AlphaCompositingModes {
                    opaque: true,
                    pre_multiplied: false,
                    post_multiplied: false,
                    inherit: false,
                },
                supported_image_usages: ImageUsages {},
            }),
        ),
        (
            vk::SurfaceCapabilitiesKHR {
                min_image_count: 1,
                max_image_count: 0,
                current_extent: vk::Extent2D {
                    width: 5,
                    height: 6,
                },
                min_image_extent: vk::Extent2D {
                    width: 3,
                    height: 4,
                },
                max_image_extent: vk::Extent2D {
                    width: 7,
                    height: 8,
                },
                max_image_array_layers: 9,
                supported_transforms: vk::SurfaceTransformFlagsKHR::IDENTITY,
                current_transform: vk::SurfaceTransformFlagsKHR::INHERIT,
                supported_composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
                supported_usage_flags: vk::ImageUsageFlags::from_raw(0xffffffff),
            },
            Ok(Capabilities {
                min_swapchain_image_count: 1,
                max_swapchain_image_count: None,
                current_image_size: Some(Size2D { x: 5, y: 6 }),
                min_swapchain_image_size: Size2D { x: 3, y: 4 },
                max_swapchain_image_size: Size2D { x: 7, y: 8 },
                max_number_of_image_layers: 9,
                supported_image_transformations: ImageTransformations {
                    identity: true,
                    rotate_90_degrees: false,
                    rotate_180_degrees: false,
                    rotate_270_degrees: false,
                    mirror: false,
                    mirror_and_rotate_90_degrees: false,
                    mirror_and_rotate_180_degrees: false,
                    mirror_and_rotate_270_degrees: false,
                    inherit: false,
                },
                current_image_transformations: ImageTransformations {
                    identity: false,
                    rotate_90_degrees: false,
                    rotate_180_degrees: false,
                    rotate_270_degrees: false,
                    mirror: false,
                    mirror_and_rotate_90_degrees: false,
                    mirror_and_rotate_180_degrees: false,
                    mirror_and_rotate_270_degrees: false,
                    inherit: true,
                },
                supported_alpha_compositing_modes: AlphaCompositingModes {
                    opaque: true,
                    pre_multiplied: false,
                    post_multiplied: false,
                    inherit: false,
                },
                supported_image_usages: ImageUsages {},
            }),
        ),
        (
            vk::SurfaceCapabilitiesKHR {
                min_image_count: 2,
                max_image_count: 1,
                current_extent: vk::Extent2D {
                    width: 5,
                    height: 6,
                },
                min_image_extent: vk::Extent2D {
                    width: 3,
                    height: 4,
                },
                max_image_extent: vk::Extent2D {
                    width: 7,
                    height: 8,
                },
                max_image_array_layers: 9,
                supported_transforms: vk::SurfaceTransformFlagsKHR::IDENTITY,
                current_transform: vk::SurfaceTransformFlagsKHR::INHERIT,
                supported_composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
                supported_usage_flags: vk::ImageUsageFlags::from_raw(0xffffffff),
            },
            Err(TryIntoCapabilitiesError::MinSwapchainImageCountGreaterThanMax),
        ),
        (
            vk::SurfaceCapabilitiesKHR {
                min_image_count: 1,
                max_image_count: 2,
                current_extent: vk::Extent2D {
                    width: 5,
                    height: 6,
                },
                min_image_extent: vk::Extent2D {
                    width: 7,
                    height: 4,
                },
                max_image_extent: vk::Extent2D {
                    width: 7,
                    height: 8,
                },
                max_image_array_layers: 9,
                supported_transforms: vk::SurfaceTransformFlagsKHR::IDENTITY,
                current_transform: vk::SurfaceTransformFlagsKHR::INHERIT,
                supported_composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
                supported_usage_flags: vk::ImageUsageFlags::from_raw(0xffffffff),
            },
            Err(TryIntoCapabilitiesError::MinSwapchainImageSizeDoesNotFitInCurrent),
        ),
        (
            vk::SurfaceCapabilitiesKHR {
                min_image_count: 1,
                max_image_count: 2,
                current_extent: vk::Extent2D {
                    width: 5,
                    height: 6,
                },
                min_image_extent: vk::Extent2D {
                    width: 3,
                    height: 4,
                },
                max_image_extent: vk::Extent2D {
                    width: 7,
                    height: 3,
                },
                max_image_array_layers: 9,
                supported_transforms: vk::SurfaceTransformFlagsKHR::IDENTITY,
                current_transform: vk::SurfaceTransformFlagsKHR::INHERIT,
                supported_composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
                supported_usage_flags: vk::ImageUsageFlags::from_raw(0xffffffff),
            },
            Err(TryIntoCapabilitiesError::MinSwapchainImageSizeDoesNotFitInMax),
        ),
        (
            vk::SurfaceCapabilitiesKHR {
                min_image_count: 1,
                max_image_count: 2,
                current_extent: vk::Extent2D {
                    width: 9,
                    height: 6,
                },
                min_image_extent: vk::Extent2D {
                    width: 3,
                    height: 4,
                },
                max_image_extent: vk::Extent2D {
                    width: 7,
                    height: 8,
                },
                max_image_array_layers: 9,
                supported_transforms: vk::SurfaceTransformFlagsKHR::IDENTITY,
                current_transform: vk::SurfaceTransformFlagsKHR::INHERIT,
                supported_composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
                supported_usage_flags: vk::ImageUsageFlags::from_raw(0xffffffff),
            },
            Err(TryIntoCapabilitiesError::CurrentSwapchainImageSizeDoesNotFitInMax),
        ),
        (
            vk::SurfaceCapabilitiesKHR {
                min_image_count: 1,
                max_image_count: 2,
                current_extent: vk::Extent2D {
                    width: 5,
                    height: 6,
                },
                min_image_extent: vk::Extent2D {
                    width: 3,
                    height: 4,
                },
                max_image_extent: vk::Extent2D {
                    width: 7,
                    height: 8,
                },
                max_image_array_layers: 0,
                supported_transforms: vk::SurfaceTransformFlagsKHR::IDENTITY,
                current_transform: vk::SurfaceTransformFlagsKHR::INHERIT,
                supported_composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
                supported_usage_flags: vk::ImageUsageFlags::from_raw(0xffffffff),
            },
            Err(TryIntoCapabilitiesError::MaxNumberOfImageLayersLessThanOne),
        ),
        (
            vk::SurfaceCapabilitiesKHR {
                min_image_count: 1,
                max_image_count: 2,
                current_extent: vk::Extent2D {
                    width: 5,
                    height: 6,
                },
                min_image_extent: vk::Extent2D {
                    width: 3,
                    height: 4,
                },
                max_image_extent: vk::Extent2D {
                    width: 7,
                    height: 8,
                },
                max_image_array_layers: 9,
                supported_transforms: vk::SurfaceTransformFlagsKHR::from_raw(0),
                current_transform: vk::SurfaceTransformFlagsKHR::INHERIT,
                supported_composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
                supported_usage_flags: vk::ImageUsageFlags::from_raw(0xffffffff),
            },
            Err(TryIntoCapabilitiesError::FailedToConvertSupportedTransformations),
        ),
        (
            vk::SurfaceCapabilitiesKHR {
                min_image_count: 1,
                max_image_count: 2,
                current_extent: vk::Extent2D {
                    width: 5,
                    height: 6,
                },
                min_image_extent: vk::Extent2D {
                    width: 3,
                    height: 4,
                },
                max_image_extent: vk::Extent2D {
                    width: 7,
                    height: 8,
                },
                max_image_array_layers: 9,
                supported_transforms: vk::SurfaceTransformFlagsKHR::IDENTITY,
                current_transform: vk::SurfaceTransformFlagsKHR::from_raw(0),
                supported_composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
                supported_usage_flags: vk::ImageUsageFlags::from_raw(0xffffffff),
            },
            Err(TryIntoCapabilitiesError::FailedToConvertCurrentTransformations),
        ),
        (
            vk::SurfaceCapabilitiesKHR {
                min_image_count: 1,
                max_image_count: 2,
                current_extent: vk::Extent2D {
                    width: 5,
                    height: 6,
                },
                min_image_extent: vk::Extent2D {
                    width: 3,
                    height: 4,
                },
                max_image_extent: vk::Extent2D {
                    width: 7,
                    height: 8,
                },
                max_image_array_layers: 9,
                supported_transforms: vk::SurfaceTransformFlagsKHR::IDENTITY,
                current_transform: vk::SurfaceTransformFlagsKHR::INHERIT,
                supported_composite_alpha: vk::CompositeAlphaFlagsKHR::from_raw(0),
                supported_usage_flags: vk::ImageUsageFlags::from_raw(0xffffffff),
            },
            Err(TryIntoCapabilitiesError::FailedToConvertCompositeAlpha),
        ),
    ];

    // rustfmt::skip is used here to avoid cargo fmt from splitting every element over many
    // lines, causing this code block to become incredibly long.
    // Data is (successful, flags, identity, rotate 90, rotate 180, rotate 270, mirror, mirror and
    // rotate 90, mirror and rotate 180, mirror and rotate 270, inherit)
    #[rustfmt::skip]
    const INTO_TRANSFORMATION_TEST_DATA: &[(
        vk::SurfaceTransformFlagsKHR,
        bool, bool, bool, bool, bool, bool, bool, bool, bool, bool
    ); 11] = &[
        (
            vk::SurfaceTransformFlagsKHR::from_raw(0),
            false, false, false, false, false, false, false, false, false, false,
        ),
        (
            vk::SurfaceTransformFlagsKHR::IDENTITY,
            true, true, false, false, false, false, false, false, false, false,
        ),
        (
            vk::SurfaceTransformFlagsKHR::ROTATE_90,
            true, false, true, false, false, false, false, false, false, false,
        ),
        (
            vk::SurfaceTransformFlagsKHR::ROTATE_180,
            true, false, false, true, false, false, false, false, false, false,
        ),
        (
            vk::SurfaceTransformFlagsKHR::ROTATE_270,
            true, false, false, false, true, false, false, false, false, false,
        ),
        (
            vk::SurfaceTransformFlagsKHR::HORIZONTAL_MIRROR,
            true, false, false, false, false, true, false, false, false, false,
        ),
        (
            vk::SurfaceTransformFlagsKHR::HORIZONTAL_MIRROR_ROTATE_90,
            true, false, false, false, false, false, true, false, false, false,
        ),
        (
            vk::SurfaceTransformFlagsKHR::HORIZONTAL_MIRROR_ROTATE_180,
            true, false, false, false, false, false, false, true, false, false,
        ),
        (
            vk::SurfaceTransformFlagsKHR::HORIZONTAL_MIRROR_ROTATE_270,
            true, false, false, false, false, false, false, false, true, false,
        ),
        (
            vk::SurfaceTransformFlagsKHR::INHERIT,
            true, false, false, false, false, false, false, false, false, true,
        ),
        (
            vk::SurfaceTransformFlagsKHR::from_raw(0xffffffff),
            true, true, true, true, true, true , true, true, true, true,
        ),
    ];

    // We use rustfmt::skip to prevent the formatter from splitting this across a massive amount
    // of lines.
    #[rustfmt::skip]
    const INTO_FORMAT_TEST_DATA :&[(
        vk::SurfaceFormatKHR,
        Result<Format, FormatConversionError>
    )]= &[
        (
            vk::SurfaceFormatKHR{
                format: vk::Format::R8_UINT,
                color_space: vk::ColorSpaceKHR::SRGB_NONLINEAR,
            },
            Ok(Format{
                image_format: ImageFormat::RegularFormat(RegularImageFormat{
                    red_channel: 8,
                    green_channel: 0,
                    blue_channel: 0,
                    alpha_channel: 0,
                    depth_channel: 0,
                    unused_bits: 0,
                    signed: false,
                    packed: None,
                    order: R,
                    data_conversion: Int,
                }),
                colour_space: ColourSpace::NonLinearSRGB,
            }
        )),
        (
            vk::SurfaceFormatKHR{
                format: vk::Format::UNDEFINED,
                color_space: vk::ColorSpaceKHR::SRGB_NONLINEAR,
            },
            Err(FormatConversionError::ImageFormatError(vk::Format::UNDEFINED)),
        ),
        (
            vk::SurfaceFormatKHR{
                format: vk::Format::R8_UINT,
                color_space: vk::ColorSpaceKHR::DOLBYVISION_EXT,
            },
            Err(FormatConversionError::ColourSpaceError(vk::ColorSpaceKHR::DOLBYVISION_EXT)),
        ),
        (
            vk::SurfaceFormatKHR{
                format: vk::Format::UNDEFINED,
                color_space: vk::ColorSpaceKHR::DOLBYVISION_EXT,
            },
            Err(FormatConversionError::ImageFormatError(vk::Format::UNDEFINED)),
        ),
        (
            vk::SurfaceFormatKHR{
                format: vk::Format::EAC_R11G11_UNORM_BLOCK,
                color_space: vk::ColorSpaceKHR::SRGB_NONLINEAR,
            },
            Err(FormatConversionError::ImageFormatError(vk::Format::EAC_R11G11_UNORM_BLOCK)),
        ),
    ];

    // We use rustfmt::skip to prevent the formatter from splitting this across a massive amount
    // of lines.
    #[rustfmt::skip]
    const INTO_IMAGE_FORMAT_TEST_DATA: &[(vk::Format, Result<ImageFormat, ()>)] = &[
        (
            vk::Format::R8_UINT,
            Ok(ImageFormat::RegularFormat(RegularImageFormat{
                    red_channel: 8,
                    green_channel: 0,
                    blue_channel: 0,
                    alpha_channel: 0,
                    depth_channel: 0,
                    unused_bits: 0,
                    signed: false,
                    packed: None,
                    order: R,
                    data_conversion: Int,
            })),
        ),
        (
            vk::Format::from_raw(-1),
            Err(()),
        ),
        (
            vk::Format::UNDEFINED,
            Err(()),
        ),
        (
            vk::Format::EAC_R11G11_UNORM_BLOCK,
            Err(()),
        ),
    ];
}
