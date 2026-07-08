use ash::{vk, khr, Instance};
use super::{info_handler, QueueFamilyIndices};
use crate::{renderer::RendererError,
            string_handler};

/// This function will return the most appropriate physical device for use, or an error if there is
/// not an appropriate physical device.
pub(super) fn get_physical_device(
    vulkan_instance: &Instance,
    surface_instance: &khr::surface::Instance,
    surface: vk::SurfaceKHR,
) -> Result<vk::PhysicalDevice, RendererError> {
    let available_devices = unsafe {vulkan_instance.enumerate_physical_devices()}?;

    let mut suitable_devices = Vec::new();

    struct RankedDevice {
        device: vk::PhysicalDevice,
        score: i32,
    }

    for device in available_devices {
        if is_physical_device_suitable(vulkan_instance, surface_instance, surface, device)? {
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
    vulkan_instance: &Instance,
    surface_instance: &khr::surface::Instance,
    surface: vk::SurfaceKHR,
    device: vk::PhysicalDevice
) -> Result<bool, RendererError> {
    let mut device_properties = vk::PhysicalDeviceProperties2::default();
    unsafe { vulkan_instance.get_physical_device_properties2(device, &mut device_properties) };

    let mut device_features = vk::PhysicalDeviceFeatures2::default();
    unsafe { vulkan_instance.get_physical_device_features2(device, &mut device_features) };

    let queue_family_indices = get_queue_family_indices(
        vulkan_instance,
        surface_instance,
        surface,
        device,
    )?;

    for queue_family_index in queue_family_indices.queue_family_indices {
        if queue_family_index == None {
            return Ok(false);
        }
    }

    match check_device_extension_support(vulkan_instance, device) {
        Ok(true) => (),
        other => return other,
    }

    match check_device_display_capabilities(surface_instance, device, surface) {
        Ok(true) => (),
        other => return other,
    }

    Ok(true)
}

/// Finds the index for all the queue families we need
pub(super) fn get_queue_family_indices(
    vulkan_instance: &Instance,
    surface_instance: &khr::surface::Instance,
    surface: vk::SurfaceKHR,
    device: vk::PhysicalDevice
) -> Result<QueueFamilyIndices, RendererError> {

    let queue_families = unsafe {
        let num_queue_families = vulkan_instance
            .get_physical_device_queue_family_properties2_len(device);
        let mut queue_families = vec![vk::QueueFamilyProperties2::default(); num_queue_families];
        vulkan_instance.get_physical_device_queue_family_properties2(device, &mut queue_families);
        queue_families
    };

    // If updating this to have more queue families, make sure to assign them below
    let mut found_queue_families = QueueFamilyIndices {
        queue_family_indices: [None, None],
    };

    for (i, queue) in queue_families.iter().enumerate() {
        if i > u16::into(u16::MAX) {
            return Err(RendererError::TooManyQueuesAvailableToHandleError)
        }

        // Checks if the current queue is suitable for the graphics family
        if found_queue_families.queue_family_indices[QueueFamilyIndices::GRAPHICS_QUEUE] == None &&
            queue.queue_family_properties.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
            found_queue_families.queue_family_indices[QueueFamilyIndices::GRAPHICS_QUEUE] =
                Some(i as u32);
        }

        if found_queue_families.queue_family_indices[QueueFamilyIndices::PRESENTATION_QUEUE]
            == None
            && unsafe {surface_instance.get_physical_device_surface_support(
                device,
                i as u32,
                surface,
            )?} {
            found_queue_families.queue_family_indices[QueueFamilyIndices::PRESENTATION_QUEUE] =
                Some(i as u32);
        }
    }

    Ok(found_queue_families)
}

/// A function to check that the physical device selected supports the required device extensions
fn check_device_extension_support(vulkan_instance: &Instance, device: vk::PhysicalDevice)
    -> Result<bool, RendererError> {
    let available_extensions = unsafe {
        vulkan_instance.enumerate_device_extension_properties(device)
    }?;

    for needed_extension in super::DEVICE_EXTENSION_NAMES{
        let mut available = false;
        for available_extension in &available_extensions {
            let name = unsafe{
                string_handler::char_array_to_cstr(&available_extension.extension_name)
            };
            if name == needed_extension {
                available = true;
            }
        }
        if !available {
            return Ok(false)
        }
    }
    Ok(true)
}

/// A function to rank the physical devices available
/// Right now it just gives all dedicated GPUs a score of 1 and other GPUs a score of 0;
/// TODO figure out a way to allow the user to select what physical device to use
fn rank_device(vulkan_instance: &Instance, device: vk::PhysicalDevice) -> i32 {
    let device_properties = unsafe { vulkan_instance.get_physical_device_properties(device) };
    if device_properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU {
        1
    } else {
        0
    }
}

fn get_device_display_info(
    surface_instance: &khr::surface::Instance,
    device: vk::PhysicalDevice,
    surface: vk::SurfaceKHR)
    -> Result<info_handler::InternalDisplayInfo, RendererError> {

    let surface_capabilities = unsafe{
        surface_instance.get_physical_device_surface_capabilities(device, surface)
    }?;

    let surface_formats = unsafe{
        surface_instance.get_physical_device_surface_formats(device, surface)
    }?;

    let surface_presentation_modes = unsafe{
        surface_instance.get_physical_device_surface_present_modes(device, surface)
    }?;

    Ok(info_handler::InternalDisplayInfo{
        capabilities: surface_capabilities,
        formats: surface_formats,
        presentation_modes: surface_presentation_modes,
    })
}

/// This function checks that the physical device and surface can handle an adequate swapchain.
/// Must have verified that the required extensions are supported before running
fn check_device_display_capabilities(
    surface_instance: &khr::surface::Instance,
    device: vk::PhysicalDevice,
    surface: vk::SurfaceKHR,
) -> Result<bool, RendererError> {

    let display_info = get_device_display_info(surface_instance, device, surface)?;

    Ok(!display_info.formats.is_empty() && !display_info.presentation_modes.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::renderer::debugger::tests;

    // This test will also fail on any device that is not suitable for the program to be run on
    #[test]
    fn can_get_physical_device() {
        let (_guard, vulkan_entry, mut glfw_instance) = tests::get_entries();
        let mut vulkan_instance = tests::get_vulkan_instance(& vulkan_entry, &glfw_instance);
        let window = tests::get_window(&mut glfw_instance);
        let surface = tests::get_window_surface(&mut vulkan_instance, &window);
        let surface_instance = tests::get_surface_instance(&vulkan_entry, &vulkan_instance);
        match get_physical_device(&vulkan_instance, &surface_instance, surface) {
            Ok(_) => (),
            Err(e) => panic!("{e:?}"),
        }
    }

    #[test]
    fn physical_device_is_suitable() {
        let (_guard, vulkan_entry, mut glfw_instance) = tests::get_entries();
        let mut vulkan_instance = tests::get_vulkan_instance(& vulkan_entry, &glfw_instance);
        let window = tests::get_window(&mut glfw_instance);
        let surface = tests::get_window_surface(&mut vulkan_instance, &window);
        let surface_instance = tests::get_surface_instance(&vulkan_entry, &vulkan_instance);
        let physical_device = get_physical_device(
            &vulkan_instance,
            &surface_instance,
            surface
        ).unwrap();
        match is_physical_device_suitable(
            &vulkan_instance,
            &surface_instance,
            surface,
            physical_device,
        ) {
            Ok(true) => (),
            Ok(false) => panic!("Physical device not suitable!"),
            Err(e) => panic!("{e:?}"),
        }
    }

    #[test]
    fn can_get_device_queue_indices() {
        let (_guard, vulkan_entry, mut glfw_instance) = tests::get_entries();
        let mut vulkan_instance = tests::get_vulkan_instance(& vulkan_entry, &glfw_instance);
        let window = tests::get_window(&mut glfw_instance);
        let surface = tests::get_window_surface(&mut vulkan_instance, &window);
        let surface_instance = tests::get_surface_instance(&vulkan_entry, &vulkan_instance);
        let physical_device = get_physical_device(
            &vulkan_instance,
            &surface_instance,
            surface
        ).unwrap();
        match get_queue_family_indices(
            &vulkan_instance,
            &surface_instance,
            surface,
            physical_device,
        ) {
            Ok(_) => (),
            Err(e) => panic!("{e:?}"),
        }
    }

    #[test]
    fn device_queue_indices_all_valid() {
        let (_guard, vulkan_entry, mut glfw_instance) = tests::get_entries();
        let mut vulkan_instance = tests::get_vulkan_instance(& vulkan_entry, &glfw_instance);
        let window = tests::get_window(&mut glfw_instance);
        let surface = tests::get_window_surface(&mut vulkan_instance, &window);
        let surface_instance = tests::get_surface_instance(&vulkan_entry, &vulkan_instance);
        let physical_device = get_physical_device(
            &vulkan_instance,
            &surface_instance,
            surface
        ).unwrap();
        let indices = get_queue_family_indices(
            &vulkan_instance,
            &surface_instance,
            surface,
            physical_device,
        ).unwrap();
        for index in indices.queue_family_indices {
            if index == None {
                panic!("Empty queue family index for chosen physical device")
            }
        }
    }

    #[test]
    fn device_had_extension_support() {
        let (_guard, vulkan_entry, mut glfw_instance) = tests::get_entries();
        let mut vulkan_instance = tests::get_vulkan_instance(& vulkan_entry, &glfw_instance);
        let window = tests::get_window(&mut glfw_instance);
        let surface = tests::get_window_surface(&mut vulkan_instance, &window);
        let surface_instance = tests::get_surface_instance(&vulkan_entry, &vulkan_instance);
        let physical_device = get_physical_device(
            &vulkan_instance,
            &surface_instance,
            surface
        ).unwrap();
        match check_device_extension_support(&vulkan_instance, physical_device) {
            Ok(true) => (),
            Ok(false) => panic!("Physical device not suitable!"),
            Err(e) => panic!("{e:?}"),
        }
    }

    #[test]
    fn can_get_device_display_info() {
        let (_guard, vulkan_entry, mut glfw_instance) = tests::get_entries();
        let mut vulkan_instance = tests::get_vulkan_instance(& vulkan_entry, &glfw_instance);
        let window = tests::get_window(&mut glfw_instance);
        let surface = tests::get_window_surface(&mut vulkan_instance, &window);
        let surface_instance = tests::get_surface_instance(&vulkan_entry, &vulkan_instance);
        let physical_device = get_physical_device(
            &vulkan_instance,
            &surface_instance,
            surface
        ).unwrap();
        match get_device_display_info(&surface_instance, physical_device, surface) {
            Ok(_) => (),
            Err(e) => panic!("{e:?}"),
        }
    }

    #[test]
    fn device_has_display_capabilities() {
        let (_guard, vulkan_entry, mut glfw_instance) = tests::get_entries();
        let mut vulkan_instance = tests::get_vulkan_instance(& vulkan_entry, &glfw_instance);
        let window = tests::get_window(&mut glfw_instance);
        let surface = tests::get_window_surface(&mut vulkan_instance, &window);
        let surface_instance = tests::get_surface_instance(&vulkan_entry, &vulkan_instance);
        let physical_device = get_physical_device(
            &vulkan_instance,
            &surface_instance,
            surface
        ).unwrap();
        match check_device_display_capabilities(&surface_instance, physical_device, surface) {
            Ok(true) => (),
            Ok(false) => panic!("Physical device does not have required display capabilities!"),
            Err(e) => panic!("{e:?}"),
        }
    }
}