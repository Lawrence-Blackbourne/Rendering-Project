mod physical_device_handler;

use ash::{vk, Instance};

use crate::renderer::RendererError;

/// Retrieve the physical device for the system
pub(crate) fn get_physical_device(vulkan_instance: &Instance)
    -> Result<vk::PhysicalDevice, RendererError> {
    physical_device_handler::get_physical_device(vulkan_instance)
}