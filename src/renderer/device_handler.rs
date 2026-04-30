mod physical_device_handler;
mod logical_device_handler;
mod queue_handler;

use ash::{vk, khr, Device, Instance};

use std::ffi::CStr;

use crate::renderer::RendererError;

const DEVICE_EXTENSION_NAMES: [&CStr; 1] = [khr::swapchain::NAME];

/// Creates a virtual device on the appropriate physical device
/// Also creates the appropriate queues
pub(crate) fn get_device(
    vulkan_instance: &Instance,
    surface_instance: &khr::surface::Instance,
    surface: vk::SurfaceKHR,
) -> Result<(Device, Vec<vk::Queue>), RendererError> {
    let physical_device = physical_device_handler::get_physical_device(
        vulkan_instance,
        surface_instance,
        surface,
    )?;
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

// If updating, remember to update get_queue_create_infos in logical_device_handler
struct QueueFamilyIndices {
    //This stores all the queue families
    queue_family_indices: [Option<u32>; 2],
}

impl QueueFamilyIndices {
    // This queue is for graphics commands
    const GRAPHICS_QUEUE: usize = 0;

    // This queue is for presenting the final image to the surface
    const PRESENTATION_QUEUE: usize = 1;
}

/// This stores the details about what surface information is supported
struct DisplayInfo {
    capabilities: vk::SurfaceCapabilitiesKHR,
    formats: Vec<vk::SurfaceFormatKHR>,
    presentation_modes: Vec<vk::PresentModeKHR>,
}