use ash::vk;
use crate::renderer::Size;

/// A struct holding a potential physical device in a way that is easily usable.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PhysicalDevice {
    pub display_info: DisplayInfo,
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
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LogicalDeviceSettings {
    pub(crate) physical_device: PhysicalDevice,
    pub(crate) num_swap_frames: u8,
}

impl LogicalDeviceSettings {
    pub fn get_physical_device(&self) -> &PhysicalDevice {
        &self.physical_device
    }

    pub fn with_physical_device(mut self, device: PhysicalDevice) -> Self {
        self.physical_device = device;
        self
    }

    pub fn get_num_swap_frames(&self) -> u8 {
        self.num_swap_frames
    }

    pub fn with_num_swap_frames(mut self, num_swap_frames: u8) -> Self {
        self.num_swap_frames = num_swap_frames;
        self
    }
}

/// The info to be given to an external caller so they can choose a setup they want.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DisplayInfo {
    pub capabilities: Capabilities,
    //TODO pub available_formats: Vec<ImageFormat>,
    //TODO pub presentation_modes: Vec<PresentationMode>
}

impl From<VulkanDisplayInfo> for DisplayInfo {
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
    pub min_swapchain_image_count: u32,
    pub max_swapchain_image_count: u32,

    /// A None for current_image_size is a special value meaning that the size of the image depends
    /// on the size of the provided swapchain.
    pub current_image_size: Option<Size>,
    pub min_swapchain_size: Size,
    pub max_swapchain_size: Size,
    pub max_number_of_image_layers: u32,
    pub supported_image_transformations: ImageTransformations,
    pub current_image_transformations: ImageTransformations,
    pub supported_alpha_compositing_modes: AlphaCompositingModes,
    pub supported_image_usages: ImageUsages,
    pub supported_vulkan_image_usages: VulkanImageUsages,
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
            supported_vulkan_image_usages: value.supported_usage_flags.into(),
        }
    }
}

/// Stores transformations of the image.
/// Mirroring is horizontal.
/// Rotation is clockwise.
/// Mirroring is always done before rotation.
/// Inherit means the transformation is not specified, and is determined by platform specific
/// considerations and mechanisms.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ImageTransformations {
    pub identity: bool,
    pub rotate_90_degrees: bool,
    pub rotate_180_degrees: bool,
    pub rotate_270_degrees: bool,
    pub mirror: bool,
    pub mirror_and_rotate_90_degrees: bool,
    pub mirror_and_rotate_180_degrees: bool,
    pub mirror_and_rotate_270_degrees: bool,
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
/// Opaque means that the alpha is ignored, and the image is treated as a constant alpha of 1.
/// Pre Multiplied and Post Multiplied means that the alpha is respected.
/// In Pre Multiplied the non-alpha components are expected to already be multiplied by the
/// alpha component.
/// In Post Multiplied the non-alpha components are not expected to already be multiplied by the
/// alpha component, and the compositor will multiply the non-alpha components by the alpha
/// component.
/// In Inherit, the way that the alpha component is used is platform-specific.
/// If the image does not have an alpha component, these settings will not do anything (except
/// potentially inherit depending on the specific system).
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct AlphaCompositingModes {
    pub opaque: bool,
    pub pre_multiplied: bool,
    pub post_multiplied: bool,
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

/// Specifies how an image is used.
// TODO phase out in favor of ImageUsages.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct VulkanImageUsages {
    /// Can be used as a source of transfer operations.
    pub transfer_source: bool,

    /// Can be used as a destination for transfer operations.
    pub transfer_destination: bool,

    /// Can be sampled from by shaders, and create an image view which can be used for a sampler.
    pub sampled: bool,

    /// Can create an image view which can be used for image storage.
    pub storage: bool,

    /// Can create an image view which can be used in a framebuffer.
    pub colour_attachment: bool,

    /// Can create an image view which can be used as a depth/stencil or depth/stencil resolve
    /// attachment in a framebuffer.
    pub depth_stencil_attachment: bool,

    /// Can create an image view which can be used as a descriptor set, can be read from a
    /// shader as an input attachment, and can be used as an input attachment to a framebuffer.
    pub input_attachment: bool,

    /// Can be used as a decode output picture in a video decode operation.
    pub video_decode_destination: bool,

    /// Can be used as an output reconstructed picture or an input reference picture in a video
    /// decode operation.
    pub video_decode_decoded_picture_buffer: bool,

    /// Can be used as an encode input picture in a video encode operation.
    pub video_encode_source: bool,

    /// Can be used as an output reconstructed picture or an input reference picture in a video
    /// encode operation.
    pub video_encode_decoded_picture_buffer: bool,

    /// Can create an image view which can be used as a fragment shading rate attachment.
    pub shading_rate_image: bool,

    /// Can create an image view which can be used as a fragment density map image.
    pub fragment_density_map: bool,

    /// Can create an image view which can be used as a fragment shading rate attachment, or as a
    /// shading rate image.
    pub fragment_shading_rate_attachment: bool,

    /// Can be used with host copy commands and host layout transitions.
    pub host_transfer: bool,
}

impl From<vk::ImageUsageFlags> for VulkanImageUsages {
    fn from(value: vk::ImageUsageFlags) -> Self {
        Self {
            transfer_source: value.contains(vk::ImageUsageFlags::TRANSFER_SRC),
            transfer_destination: value.contains(vk::ImageUsageFlags::TRANSFER_DST),
            sampled: value.contains(vk::ImageUsageFlags::SAMPLED),
            storage: value.contains(vk::ImageUsageFlags::STORAGE),
            colour_attachment: value.contains(vk::ImageUsageFlags::COLOR_ATTACHMENT),
            depth_stencil_attachment: value.contains(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT),
            input_attachment: value.contains(vk::ImageUsageFlags::INPUT_ATTACHMENT),
            video_decode_destination: value.contains(vk::ImageUsageFlags::VIDEO_DECODE_DST_KHR),
            video_decode_decoded_picture_buffer: value.contains(
                vk::ImageUsageFlags::VIDEO_DECODE_DPB_KHR
            ),
            video_encode_source: value.contains(vk::ImageUsageFlags::VIDEO_ENCODE_SRC_KHR),
            video_encode_decoded_picture_buffer: value.contains(
                vk::ImageUsageFlags::VIDEO_ENCODE_DPB_KHR
            ),
            shading_rate_image: value.contains(vk::ImageUsageFlags::SHADING_RATE_IMAGE_NV),
            fragment_density_map: value.contains(vk::ImageUsageFlags::FRAGMENT_DENSITY_MAP_EXT),
            fragment_shading_rate_attachment: value.contains(
                vk::ImageUsageFlags::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR
            ),
            host_transfer: value.contains(vk::ImageUsageFlags::HOST_TRANSFER_EXT),
        }
    }
}