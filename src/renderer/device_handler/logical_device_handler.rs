use ash::{vk, Device, Instance};

use crate::renderer::RendererError;

/// This creates the logical device from the physical device.
/// Any features that need to be enabled need to have that done here.
pub(super) fn get_logical_device(
    vulkan_instance: &Instance,
    physical_device: vk::PhysicalDevice,
) -> Result<Device, RendererError> {

    let queue_family_indices = super::physical_device_handler::get_queue_family_indices(
        vulkan_instance,
        physical_device,
    )?;

    let device_create_flags = vk::DeviceCreateFlags::empty();
    let queue_create_infos = get_queue_create_infos(queue_family_indices)?;
    let enabled_features = vk::PhysicalDeviceFeatures::default();

    let device_create_info = vk::DeviceCreateInfo::default()
        .flags(device_create_flags)
        .queue_create_infos(&queue_create_infos)
        .enabled_extension_names(&[])
        .enabled_features(&enabled_features);

    Ok(unsafe {vulkan_instance.create_device(physical_device, &device_create_info ,None)}?)
}

/// The queue create info is made here.
/// This creates the information to create the queue, such as flags, priorities, etc.
fn get_queue_create_infos(
    queue_indices: super::QueueFamilyIndices,
) -> Result<Vec<vk::DeviceQueueCreateInfo<'static>>, RendererError>{
    let mut queues = vec![];

    // If we want to add any flags to a queue, we need to replace vk::GetDeviceQueue with
    // vk::GetDeviceQueue2
    let graphics_queue_flags = vk::DeviceQueueCreateFlags::empty();
    let graphics_queue_index = match queue_indices.graphics_queue_family {
        Some(index) => index,
        None => return Err(RendererError::LogicError(String::from("get_queue_create_info"))),
    };
    let graphics_queue_priorities: &[f32] = &[1.0];
    let graphics_queue = vk::DeviceQueueCreateInfo::default()
        .flags(graphics_queue_flags)
        .queue_family_index(graphics_queue_index)
        .queue_priorities(&graphics_queue_priorities);
    queues.push(graphics_queue);

    Ok(queues)
}