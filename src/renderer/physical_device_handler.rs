use ash::{vk, Instance};

use crate::renderer::RendererError;

/// This function will return the most appropriate physical device for use, or an error if there is
/// not an appropriate physical device.
pub(crate) fn get_physical_device(vulkan_instance: &Instance) -> Result<vk::PhysicalDevice, RendererError> {
    let available_devices = unsafe {vulkan_instance.enumerate_physical_devices()}?;

    let mut suitable_devices = Vec::new();

    struct RankedDevice {
        device: vk::PhysicalDevice,
        score: i32,
    }

    for device in available_devices {
        if is_physical_device_suitable(vulkan_instance, device) {
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
fn is_physical_device_suitable(vulkan_instance: &Instance, device: vk::PhysicalDevice) -> bool {
    let _device_properties = unsafe { vulkan_instance.get_physical_device_properties(device) };
    let _device_features = unsafe { vulkan_instance.get_physical_device_features(device) };
    true
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