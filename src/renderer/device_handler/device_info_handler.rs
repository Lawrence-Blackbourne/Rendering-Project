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

    //TODO pub available_formats: Vec<ImageFormat>,
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
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ImageUsages {

}

impl From<vk::ImageUsageFlags> for ImageUsages {
    fn from(_value: vk::ImageUsageFlags) -> Self {
        ImageUsages {

        }
    }
}