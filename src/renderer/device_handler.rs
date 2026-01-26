mod physical_device_handler;
mod logical_device_handler;
mod queue_handler;

use ash::{vk, Device, Instance};

use crate::renderer::RendererError;

/// Creates a virtual device on the appropriate physical device
/// Also creates the appropriate queues
pub(crate) fn get_device(
    vulkan_instance: &Instance,
) -> Result<(Device, Vec<vk::Queue>), RendererError> {
    let physical_device = physical_device_handler::get_physical_device(vulkan_instance)?;
    let logical_device = logical_device_handler::get_logical_device(
        vulkan_instance,
        physical_device,
    )?;
    let queues = queue_handler::get_queues(
        vulkan_instance,
        physical_device,
        &logical_device)?;
    
    Ok((logical_device, queues))
}

struct QueueFamilyIndices {
    graphics_queue_family: Option<u32>,
}