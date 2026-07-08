mod physical_device_handler;
mod logical_device_handler;
mod queue_handler;
pub(crate) mod info_handler;

use ash::{vk, khr, Device, Instance};
use std::ffi::CStr;
use crate::renderer::{RendererError, Size};

const DEVICE_EXTENSION_NAMES: [&CStr; 1] = [khr::swapchain::NAME];
const NUM_QUEUE_FAMILIES: usize = 2;

/// Gives the caller the

/// Creates a virtual device with the given settings
/// Also creates the appropriate queues
pub(crate) fn get_device(
    vulkan_instance: &Instance,
    surface_instance: &khr::surface::Instance,
    surface: vk::SurfaceKHR,
    logical_device_settings: info_handler::LogicalDeviceSettings,
) -> Result<(Device, Vec<vk::Queue>), RendererError> {
    let physical_device = logical_device_settings.physical_device.device;
    let logical_device = logical_device_handler::get_logical_device(
        vulkan_instance,
        physical_device,
        surface_instance,
        surface,
    )?;
    let queues = queue_handler::get_queues(
        vulkan_instance,
        physical_device,
        &logical_device,
        surface_instance,
        surface,
    )?;

    Ok((logical_device, queues))
}

/// Stores the indexes of the queue families used
/// If updating, remember to update get_queue_create_infos in logical_device_handler
#[derive(Debug)]
struct QueueFamilyIndices {
    //This stores all the queue families
    queue_family_indices: [Option<u32>; NUM_QUEUE_FAMILIES],
}

/// Consts to store where each queue family is stored in the array
impl QueueFamilyIndices {
    // This queue is for graphics commands
    const GRAPHICS_QUEUE: usize = 0;

    // This queue is for presenting the final image to the surface
    const PRESENTATION_QUEUE: usize = 1;
}

#[cfg(test)]
mod tests {
    use super::*;
    //TODO tests including checking queue_family_indices is set up correctly with good constants

    /// Gets a physical device for running tests
    pub(super) fn get_physical_device(
        vulkan_instance: &Instance,
        surface_instance: &khr::surface::Instance,
        surface: vk::SurfaceKHR,
    ) -> vk::PhysicalDevice {
        physical_device_handler::get_physical_device(vulkan_instance, surface_instance, surface)
            .unwrap()
    }

    /// Gets a logical device for running tests
    pub(super) fn get_logical_device(
        vulkan_instance: &Instance,
        physical_device: vk::PhysicalDevice,
        surface_instance: &khr::surface::Instance,
        surface: vk::SurfaceKHR,
    ) -> Device {
        logical_device_handler::get_logical_device(
            vulkan_instance,
            physical_device,
            surface_instance,
            surface)
            .unwrap()
    }
}