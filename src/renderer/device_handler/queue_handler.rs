use ash::{vk::{self, Queue}, Device, Instance};

use crate::renderer::RendererError;

/// This function gets the queues from the physical device
pub(super) fn get_queues(
    vulkan_instance: &Instance,
    physical_device: vk::PhysicalDevice,
    logical_device: &Device,
) -> Result<Vec<Queue>, RendererError> {
    let mut queues = vec![];

    let queue_family_indices = super::physical_device_handler::get_queue_family_indices(
        vulkan_instance,
        physical_device,
    )?;

    let graphics_queue = unsafe {logical_device.get_device_queue(
        queue_family_indices.graphics_queue_family.unwrap(),
        0,
    )};
    queues.push(graphics_queue);

    Ok(queues)
}