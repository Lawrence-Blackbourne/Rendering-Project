use ash::{vk::{self, Queue}, khr, Device, Instance};

use super::QueueFamilyIndices;

use crate::renderer::RendererError;

/// This function gets the queues from the physical device
pub(super) fn get_queues(
    vulkan_instance: &Instance,
    physical_device: vk::PhysicalDevice,
    logical_device: &Device,
    surface_instance: &khr::surface::Instance,
    surface: vk::SurfaceKHR,
) -> Result<Vec<Queue>, RendererError> {
    let mut queues = vec![];

    let queue_family_indices = super::physical_device_handler::get_queue_family_indices(
        vulkan_instance,
        surface_instance,
        surface,
        physical_device,
    )?;

    let graphics_queue = unsafe {logical_device.get_device_queue(
        queue_family_indices.queue_family_indices[QueueFamilyIndices::GRAPHICS_QUEUE].unwrap(),
        0,
    )};
    queues.push(graphics_queue);

    let presentation_queue = unsafe {logical_device.get_device_queue(
        queue_family_indices.queue_family_indices[QueueFamilyIndices::PRESENTATION_QUEUE].unwrap(),
        0,
    )};
    queues.push(presentation_queue);

    Ok(queues)
}