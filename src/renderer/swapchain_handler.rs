use super::device_handler::device_info_handler;
use crate::renderer::RendererError;
use ash::vk;

/// Takes in the capabilities of the device and then chooses the setting for the swapchain.
fn get_swapchain_settings(
    //TODO look at changing this to using DisplayInfo not VulkanDisplayInfo
    info: device_info_handler::VulkanDisplayInfo,
    _desired_image_count: u8,
) -> Result<device_info_handler::LogicalDeviceSettings, RendererError> {
    let mut _desired_format = None;
    const PREFERRED_FORMAT: vk::Format = vk::Format::R8G8B8A8_SRGB;

    for format in &info.formats {
        if format.format.as_raw() == PREFERRED_FORMAT.as_raw() {
            _desired_format = Some(format);
        }
    }
    if _desired_format.is_none() {
        _desired_format = info.formats.first();
    }

    let mut _desired_present_mode = vk::PresentModeKHR::FIFO;
    const PREFERRED_PRESENT_MODE: vk::PresentModeKHR = vk::PresentModeKHR::MAILBOX;

    for mode in &info.presentation_modes {
        if mode.as_raw() == PREFERRED_PRESENT_MODE.as_raw() {
            _desired_present_mode = vk::PresentModeKHR::from_raw(mode.as_raw());
        }
    }

    panic!();
}

//TODO finish and create tests
