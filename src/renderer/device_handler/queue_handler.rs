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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::renderer::{debugger::tests,
                          device_handler};

    #[test]
    fn can_get_queues() {
        get_queues_for_testing();
    }
    
    #[test]
    fn get_correct_number_of_queues() {
        let queues = get_queues_for_testing();
        assert_eq!(queues.len(), device_handler::NUM_QUEUE_FAMILIES);
    }

    fn get_queues_for_testing() -> Vec<Queue> {
        let (_guard, vulkan_entry, mut glfw_instance) = tests::get_entries();
        let mut vulkan_instance = tests::get_vulkan_instance(& vulkan_entry, &glfw_instance);
        let window = tests::get_window(&mut glfw_instance);
        let surface = tests::get_window_surface(&mut vulkan_instance, & window);
        let surface_instance = tests::get_surface_instance(& vulkan_entry, &vulkan_instance);
        let physical_device = device_handler::tests::get_physical_device(
            &vulkan_instance,
            &surface_instance,
            surface,
        );
        let logical_device = device_handler::tests::get_logical_device(
            &vulkan_instance,
            physical_device,
            &surface_instance,
            surface,
        );
        match get_queues(
            &vulkan_instance,
            physical_device,
            &logical_device,
            &surface_instance,
            surface) {
            Ok(r) => r,
            Err(e) => panic!("{e:?}"),
        }
    }
}