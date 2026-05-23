use ash::{vk, khr, Device, Instance};
use super::QueueFamilyIndices;
use crate::renderer::RendererError;

/// This creates the logical device from the physical device.
/// Any features that need to be enabled need to have that done here.
pub(super) fn get_logical_device(
    vulkan_instance: &Instance,
    physical_device: vk::PhysicalDevice,
    surface_instance: &khr::surface::Instance,
    surface: vk::SurfaceKHR,
) -> Result<Device, RendererError> {

    let queue_family_indices = super::physical_device_handler::get_queue_family_indices(
        vulkan_instance,
        surface_instance,
        surface,
        physical_device,
    )?;

    let device_create_flags = vk::DeviceCreateFlags::empty();
    let queue_create_infos = get_queue_create_infos(queue_family_indices)?;

    let mut device_extensions = Vec::new();
    for extension in super::DEVICE_EXTENSION_NAMES{
        device_extensions.push(extension.as_ptr())
    }

    let enabled_features = vk::PhysicalDeviceFeatures::default();

    let device_create_info = vk::DeviceCreateInfo::default()
        .flags(device_create_flags)
        .queue_create_infos(&queue_create_infos)
        .enabled_extension_names(device_extensions.as_slice())
        .enabled_features(&enabled_features);

    Ok(unsafe {vulkan_instance.create_device(physical_device, &device_create_info ,None)}?)
}

/// The queue create info is made here.
/// This creates the information to create the queue, such as flags, priorities, etc.
fn get_queue_create_infos(
    queue_indices: QueueFamilyIndices,
) -> Result<Vec<vk::DeviceQueueCreateInfo<'static>>, RendererError>{
    let mut queues = vec![];

    // If we want to add any flags to a queue, we need to replace vk::GetDeviceQueue with
    // vk::GetDeviceQueue2
    let mut created_queue_family_indices = Vec::new();
    for index in queue_indices.queue_family_indices {
        let queue_flags = vk::DeviceQueueCreateFlags::empty();
        let index = match index {
            Some(index) => index,
            None => return Err(RendererError::LogicError(String::from("get_queue_create_info")))
        };
        // Queue families should only be set up once
        if created_queue_family_indices.contains(&index) {
            continue;
        }
        created_queue_family_indices.push(index);
        let queue_priorities: &[f32] = &[1.0];
        let queue = vk::DeviceQueueCreateInfo::default()
            .flags(queue_flags)
            .queue_family_index(index)
            .queue_priorities(&queue_priorities);
        queues.push(queue);
    }

    Ok(queues)
}