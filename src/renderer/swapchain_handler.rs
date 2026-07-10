use ash::vk;
use super::device_handler::device_info_handler;
use crate::renderer::RendererError;

/// Stores the settings needed to create the swapchain.
struct SwapchainSettings {
    format: vk::SurfaceFormatKHR, // The data format of the images
    mode: vk::PresentModeKHR, // The presentation mode of the images
    size: vk::Extent2D, // Stores the size of the swapchain images
    image_count: u8, // The number of frames in the swapchain
}

/// Takes in the capabilities of the device and then chooses the setting for the swapchain.
fn get_swapchain_settings(
    //TODO look at changing this to using DisplayInfo not VulkanDisplayInfo
    info: device_info_handler::VulkanDisplayInfo,
    desired_image_count: u8,
) -> Result<SwapchainSettings, RendererError> {

    let mut desired_format = None;
    const PREFERRED_FORMAT: vk::Format = vk::Format::R8G8B8A8_SRGB;

    for format in &info.formats {
        if format.format.as_raw() == PREFERRED_FORMAT.as_raw() {
            desired_format = Some(format);
        }
    }
    if desired_format == None {
        desired_format = info.formats.first();
    }

    let mut desired_present_mode = vk::PresentModeKHR::FIFO;
    const PREFERRED_PRESENT_MODE: vk::PresentModeKHR = vk::PresentModeKHR::MAILBOX;

    for mode in &info.presentation_modes {
        if mode.as_raw() == PREFERRED_PRESENT_MODE.as_raw() {
            desired_present_mode = vk::PresentModeKHR::from_raw(mode.as_raw());
        }
    }

    

    panic!();
}

//TODO finish and create tests