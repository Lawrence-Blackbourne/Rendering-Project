use ash::vk;
use crate::renderer::Size;

/// A struct holding a potential physical device in a way that is easily usable.
#[non_exhaustive]
pub struct PhysicalDevice {
    pub display_info: DisplayInfo,
    pub(crate) device: vk::PhysicalDevice,
}

/// This stores the details about what surface information is supported.
pub(crate) struct InternalDisplayInfo {
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub formats: Vec<vk::SurfaceFormatKHR>,
    pub presentation_modes: Vec<vk::PresentModeKHR>,
}

/// The info to be given to an external caller so they can choose a setup they want.
#[non_exhaustive]
pub struct DisplayInfo {
    pub capabilities: Capabilities,
    //TODO pub formats: Vec<Format>,
    //TODO pub presnetation_modes: Vec<PresentationMode>
}

/// A struct storing the info about what settings we want for our logical device.
pub struct LogicalDeviceSettings {
    pub physical_device: PhysicalDevice,
    pub num_swap_frames: u8,
}

/// Allows us to transform our InternalDisplayInfo into a format that can be used for.
impl From<InternalDisplayInfo> for DisplayInfo {
    fn from(internal: InternalDisplayInfo) -> Self {
        //TODO
        panic!();
    }
}

/// What capabilities a physical device has.
#[non_exhaustive]
pub struct Capabilities {
    pub min_swapchain_image_count: u32,
    pub max_swapchain_image_count: u32,
    pub current_image_size: Size,
    pub max_swapchain_size: Size,
    pub min_swapchain_size: Size,
    pub max_number_of_image_layers: u32,
    pub supported_image_transformations: ImageTransformations,
    pub current_image_transformations: ImageTransformations,
    pub supported_alpha_composting_modes: AlphaCompositingModes,
    pub supportedUsageFlags: ImageUsageFlags
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
        ImageTransformations {
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
        AlphaCompositingModes {
            opaque: value.contains(vk::CompositeAlphaFlagsKHR::OPAQUE),
            pre_multiplied: value.contains(vk::CompositeAlphaFlagsKHR::PRE_MULTIPLIED),
            post_multiplied: value.contains(vk::CompositeAlphaFlagsKHR::POST_MULTIPLIED),
            inherit: value.contains(vk::CompositeAlphaFlagsKHR::INHERIT),
        }
    }
}

/// Specifies how an image is used.
#[non_exhaustive]
pub struct ImageUsageFlags {
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