use ash::vk;
use crate::renderer::Size;

/// A struct holding a potential physical device in a way that is easily usable
#[non_exhaustive]
pub struct PhysicalDevice {
    pub display_info: DisplayInfo,
    pub(crate) device: vk::PhysicalDevice,
}

/// This stores the details about what surface information is supported\
pub(crate) struct InternalDisplayInfo {
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub formats: Vec<vk::SurfaceFormatKHR>,
    pub presentation_modes: Vec<vk::PresentModeKHR>,
}

/// The info to be given to an external caller so they can make decisions about what setup they want
#[non_exhaustive]
pub struct DisplayInfo {
    pub capabilities: Capabilities,
    //TODO pub formats: Vec<Format>,
    //TODO pub presnetation_modes: Vec<PresentationMode>
}

/// A struct storing the info about what settings we want for our logical device
pub struct LogicalDeviceSettings {
    pub physical_device: PhysicalDevice,
    pub num_swap_frames: u8,
}

/// Allows us to transform our InternalDisplayInfo into a format that can be used for
impl From<InternalDisplayInfo> for DisplayInfo {
    fn from(internal: InternalDisplayInfo) -> DisplayInfo {
        //TODO
        panic!();
    }
}

/// What capabilities a physical device has
#[non_exhaustive]
pub struct Capabilities {
    pub min_swapchain_image_count: u32,
    pub max_swapchain_image_count: u32,
    pub current_image_size: Size,
    pub max_swapchain_size: Size,
    pub min_swapchain_size: Size,
    pub max_number_of_image_layers: u32,
    pub supported_image_transformations: ImageTransformations,
    pub current_image_transformations: ImageTransformations
    //TODO pub supported_alpha_composting_modes: AlphaCompostingModes,
    //TODO pub supportedUsageFlags: ImageUsageFlags
}


#[non_exhaustive]
struct ImageTransformations {

}