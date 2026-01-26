use ash::{vk, Instance};

use crate::renderer::RendererError;

/// This function will return the most appropriate physical device for use, or an error if there is
/// not an appropriate physical device.
pub(super) fn get_physical_device(
    vulkan_instance: &Instance,
) -> Result<vk::PhysicalDevice, RendererError> {
    let available_devices = unsafe {vulkan_instance.enumerate_physical_devices()}?;

    let mut suitable_devices = Vec::new();

    struct RankedDevice {
        device: vk::PhysicalDevice,
        score: i32,
    }

    for device in available_devices {
        if is_physical_device_suitable(vulkan_instance, device)? {
            suitable_devices.push(RankedDevice {
                device,
                score: rank_device(vulkan_instance, device),
            });
        }
    }

    suitable_devices.sort_by(|a, b| b.score.cmp(&a.score));

    match suitable_devices.get(0) {
        Some(device) => Ok(device.device),
        None => Err(RendererError::UnableToFindSuitablePhysicalDeviceError),
    }
}

/// Returns true if a physical device is suitable for our needs, and false if not.
/// If any later part of the code requires support for a certain feature to be checked, do that
/// check here.
fn is_physical_device_suitable(
    vulkan_instance: &Instance, device: vk::PhysicalDevice,
) -> Result<bool, RendererError> {
    let mut device_properties = vk::PhysicalDeviceProperties2::default();
    unsafe { vulkan_instance.get_physical_device_properties2(device, &mut device_properties) };

    let mut device_features = vk::PhysicalDeviceFeatures2::default();
    unsafe { vulkan_instance.get_physical_device_features2(device, &mut device_features) };

    let queue_family_indices = get_queue_family_indices(vulkan_instance, device)?;

    Ok(queue_family_indices.graphics_queue_family != None)
}

/// A function to rank the physical devices available
/// Right now it just gives all dedicated GPUs a score of 1 and other GPUs a score of 0;
fn rank_device(vulkan_instance: &Instance, device: vk::PhysicalDevice) -> i32 {
    let device_properties = unsafe { vulkan_instance.get_physical_device_properties(device) };
    if device_properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU {
        1
    } else {
        0
    }
}

/// Finds the index for all the queue families we need
pub(super) fn get_queue_family_indices(
    vulkan_instance: &Instance,
    device: vk::PhysicalDevice
) -> Result<super::QueueFamilyIndices, RendererError> {

    let queue_families = unsafe {
        let num_queue_families = vulkan_instance
            .get_physical_device_queue_family_properties2_len(device);
        let mut queue_families = vec![vk::QueueFamilyProperties2::default(); num_queue_families];
        vulkan_instance.get_physical_device_queue_family_properties2(device, &mut queue_families);
        queue_families
    };

    let mut found_queue_families = super::QueueFamilyIndices {
        graphics_queue_family: None,
    };

    for (i, queue) in queue_families.iter().enumerate() {
        if found_queue_families.graphics_queue_family == None &&
            queue.queue_family_properties.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
            if i > u16::into(u16::MAX) {
                return Err(RendererError::TooManyQueuesAvailableToHandleError)
            } else {
                found_queue_families.graphics_queue_family = Some(i as u32);
            }
        }
    }

    Ok(found_queue_families)
}