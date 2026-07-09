use ash::vk;
use crate::renderer::Size;

/// A struct holding a potential physical device in a way that is easily usable.
#[non_exhaustive]
pub struct PhysicalDevice {
    pub display_info: DisplayInfo,
    pub(crate) device: vk::PhysicalDevice,
}

/// This stores the details about what surface information is supported.
pub(crate) struct VulkanDisplayInfo {
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub formats: Vec<vk::SurfaceFormatKHR>,
    pub presentation_modes: Vec<vk::PresentModeKHR>,
}

/// A struct storing the info about what settings we want for our logical device.
pub struct LogicalDeviceSettings {
    pub physical_device: PhysicalDevice,
    pub num_swap_frames: u8,
}

/// The info to be given to an external caller so they can choose a setup they want.
#[non_exhaustive]
pub struct DisplayInfo {
    pub capabilities: Capabilities,
    //TODO pub formats: Vec<Format>,
    //TODO pub presentation_modes: Vec<PresentationMode>
}

/// Allows us to transform our InternalDisplayInfo into a format that can be used for.
impl From<VulkanDisplayInfo> for DisplayInfo {
    fn from(value: VulkanDisplayInfo) -> Self {
        Self {
            capabilities: value.capabilities.into(),
        }
    }
}

/// What capabilities a physical device has.
#[non_exhaustive]
pub struct Capabilities {
    pub min_swapchain_image_count: u32,
    pub max_swapchain_image_count: u32,
    pub current_image_size: Size,
    pub min_swapchain_size: Size,
    pub max_swapchain_size: Size,
    pub max_number_of_image_layers: u32,
    pub supported_image_transformations: ImageTransformations,
    pub current_image_transformations: ImageTransformations,
    pub supported_alpha_composting_modes: AlphaCompositingModes,
    pub supported_image_usages: ImageUsages,
    pub supported_vulkan_image_usages: VulkanImageUsages,
}

impl From<vk::SurfaceCapabilitiesKHR> for Capabilities {
    fn from(value: vk::SurfaceCapabilitiesKHR) -> Self {
        Self {
            min_swapchain_image_count: value.min_image_count,
            max_swapchain_image_count: value.max_image_count,
            current_image_size: value.current_extent.into(),
            min_swapchain_size: value.min_image_extent.into(),
            max_swapchain_size: value.max_image_extent.into(),
            max_number_of_image_layers: value.max_image_array_layers,
            supported_image_transformations: value.supported_transforms.into(),
            current_image_transformations: value.current_transform.into(),
            supported_alpha_composting_modes: value.supported_composite_alpha.into(),
            supported_image_usages: value.supported_usage_flags.into(),
            supported_vulkan_image_usages: value.supported_usage_flags.into(),
        }
    }
}

/// Stores transformations of the image.
/// Mirror is horizontal.
/// Rotate is clockwise.
/// Mirror is always done before rotate.
/// Inherit means the transformation is not specified, and is determined by platform specific
/// considerations and mechanisms.
#[non_exhaustive]
pub struct ImageTransformations {
    identity: bool,
    rotate_90_degrees: bool,
    rotate_180_degrees: bool,
    rotate_270_degrees: bool,
    mirror: bool,
    mirror_and_rotate_90_degrees: bool,
    mirror_and_rotate_180_degrees: bool,
    mirror_and_rotate_270_degrees: bool,
    inherit: bool,
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
/// alpha component, and the compositer will multiply the non-alpha components by the alpha
/// component.
/// In Inherit, the way that the alpha component is used is platform-specific.
/// If the image does not have an alpha component, these settings will not do anything (except
/// potentially inherit depending on the system and things out of the code's control).
#[non_exhaustive]
pub struct AlphaCompositingModes {
    opaque: bool,
    pre_multiplied: bool,
    post_multiplied: bool,
    inherit: bool,
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

pub struct ImageUsages {

}

impl From<vk::ImageUsageFlags> for ImageUsages {
    fn from(value: vk::ImageUsageFlags) -> Self {
        ImageUsages {

        }
    }
}

/// Specifies how an image is used.
// TODO phase out in favor of ImageUsages.
#[non_exhaustive]
pub struct VulkanImageUsages {
    /// Can be used as a source of transfer operations.
    transfer_source: bool,

    /// Can be used as a destination for transfer operations.
    transfer_destination: bool,

    /// Can be sampled from by shaders, and create an image view which can be used for a sampler.
    sampled: bool,

    /// Can create an image view which can be used for image storage.
    storage: bool,

    /// Can create an image view which can be used in a framebuffer.
    colour_attachment: bool,

    /// Can create an image view which can be used as a depth/stencil or depth/stencil resolve
    /// attachment in a framebuffer.
    depth_stencil_attachment: bool,

    /// Can create an image view which can be used as a descriptor set, can be read from a
    /// shader as an input attachment, and can be used as an input attachment to a framebuffer.
    input_attachment: bool,

    /// Can be used as a decode output picture in a video decode operation.
    video_decode_destination: bool,

    /// Can be used as an output reconstructed picture or an input reference picture in a video
    /// decode operation.
    video_decode_decoded_picture_buffer: bool,

    /// Can be used as an encode input picture in a video decode operation.
    video_encode_source: bool,

    /// Can be used as an output reconstructed picture or an input reference picture in a video
    /// encode operation.
    video_encode_decoded_picture_buffer: bool,

    /// Can create an image view which can be used as a fragment shading rate attachment.
    shading_rate_image: bool,

    /// Can create an image view which can be used as a fragment density map image.
    fragment_density_map: bool,

    /// Can create an image view which can be used as a fragment shading rate attachment, or as a
    /// shading rate image.
    fragment_shading_rate_attachment: bool,

    /// Can be used with host copy commands and host layout transitions.
    host_transfer: bool,
}

impl From <vk::ImageUsageFlags> for VulkanImageUsages {
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
            video_encode_source: value.contains(vk::ImageUsageFlags::VIDEO_DECODE_SRC_KHR),
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