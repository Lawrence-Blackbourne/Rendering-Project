use ash::{ext, vk, Entry, Instance};
use glfw::Glfw;
use std::{ffi::c_void,
          ops::BitOr,
          ptr};
use crate::{renderer::RendererError,
            string_handler::convert_to_cstring};

const VALIDATION_LAYER_NAMES: &[&str] = &["VK_LAYER_KHRONOS_validation"];
const VALIDATION_EXTENSION_NAMES: &[&str] = &["VK_EXT_debug_utils"];

/// Creates the main debug messenger the debugging instance to go along with it.
#[cfg(debug_assertions)]
pub(crate) fn get_debug_messenger(
    vulkan_entry: &Entry,
    vulkan_instance: &Instance,
) -> Result<(ext::debug_utils::Instance, vk::DebugUtilsMessengerEXT), RendererError> {
    let debug_instance = ext::debug_utils::Instance::new(vulkan_entry, vulkan_instance);
    let debug_messenger = unsafe {
        debug_instance.create_debug_utils_messenger(
            &get_debug_messenger_info(),
            None
        )
    }?;

    Ok((debug_instance, debug_messenger))
}

/// Gets the names of the layers needed to set up the vulkan instance including debugging.
pub(crate) fn get_setup_layer_names() -> Vec<String> {

    let mut layers = vec![];

    if cfg!(debug_assertions) {
        for layer in VALIDATION_LAYER_NAMES {
            layers.push(*layer);
        }
    }

    layers.iter()
        .map(|name| String::from(*name))
        .collect()
}

/// Validates that the layers that are passed to it are available to use.
pub(crate) fn validate_setup_layers_exist(
    layer_names: &Vec<String>,
    entry: &Entry,
) -> Result<(), RendererError> {
    let available_layers = unsafe {
        entry.enumerate_instance_layer_properties()
    }?;

    for layer_name in layer_names {
        let mut layer_exists = false;
        for available_layer in available_layers.iter() {
            if convert_to_cstring(layer_name.as_str())? ==
                available_layer.layer_name_as_c_str()?.to_owned() {
                layer_exists = true;
                break;
            }
        }
        if !layer_exists {
            return Err(RendererError::LayerRequiredNotSupportedError);
        }
    }
    Ok(())
}

/// Gets the names of the extensions needed to set up the vulkan instance including debugging.
pub(crate) fn get_setup_extension_names(
    glfw_instance: &Glfw,
) -> Result<Vec<String>, RendererError> {

    let mut extension_names = vec![];

    let glfw_extensions = match glfw_instance.get_required_instance_extensions() {
        Some(glfw_extensions) => {
            glfw_extensions
        }
        None => return Err(RendererError::GlfwCallFailed("get_required_instance_extensions"
            .to_string()))
    };

    for extension in glfw_extensions {
        extension_names.push(String::from(extension));
    }

    let mut debug_extensions = vec![];

    if cfg!(debug_assertions) {
        for extension in VALIDATION_EXTENSION_NAMES {
            debug_extensions.push(*extension);
        }
    }

    for extension in debug_extensions {
        extension_names.push(String::from(extension));
    }

    Ok(extension_names)
}

/// Sets up the debug messenger extension.
pub(crate) fn get_debug_messenger_info() -> vk::DebugUtilsMessengerCreateInfoEXT<'static> {
    // We want warnings and errors but not verbose diagnostic messages.
    let severity_flags = vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
        .bitor(vk::DebugUtilsMessageSeverityFlagsEXT::ERROR);

    let type_flags = vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
        .bitor(vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION)
        .bitor(vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE);

    let debug_messenger_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
        .flags(vk::DebugUtilsMessengerCreateFlagsEXT::empty())
        .message_severity(severity_flags)
        .message_type(type_flags)
        .pfn_user_callback(Some(debug_messenger_callback_function))
        .user_data(ptr::null_mut());

    debug_messenger_info
}

#[unsafe(no_mangle)]
unsafe extern "system" fn debug_messenger_callback_function(
    _severity_flags: vk::DebugUtilsMessageSeverityFlagsEXT,
    _type_flags: vk::DebugUtilsMessageTypeFlagsEXT,
    callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT<'_>,
    _user_data_pointer: *mut c_void
) -> u32 {

    let data = match unsafe { callback_data.as_ref() } {
        Some(data) => data,
        None => {
            eprintln!("Callback data reference dangling!");
            return vk::FALSE;
        }
    };

    let message = match unsafe { data.message_as_c_str() } {
        Some(message) => message.to_str().unwrap_or_else(|_| "Message conversion failed!"),
        None => "Callback has no message!",
    };

    eprintln!("Callback message: {}", message);

    vk::FALSE
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use ash::khr;
    use std::sync::{Mutex, MutexGuard};

    #[test]
    fn can_get_debug_messenger() {
        let (_guard, vulkan_entry, glfw_instance) = get_entries();
        let vulkan_instance = get_vulkan_instance(& vulkan_entry, & glfw_instance);
        match get_debug_messenger(&vulkan_entry, &vulkan_instance) {
            Ok(_) => (),
            Err(e) => panic!("{e:?}"),
        }
    }

    #[test]
    fn can_get_setup_layer_names() {
        get_setup_layer_names();
    }

    #[test]
    fn setup_layer_names_not_empty() {
        assert_ne!(get_setup_layer_names().len(), 0);
    }

    #[test]
    fn layer_support_no_layers() {
        let layers = vec![];
        assert_eq!(test_layer_support(&layers), true)
    }

    #[test]
    fn layer_support_valid_layers() {
        let layers = vec![String::from("VK_LAYER_KHRONOS_validation")];
        assert_eq!(test_layer_support(&layers), true)
    }

    #[test]
    fn layer_support_invalid_layers() {
        let layers = vec![String::from("random name")];
        assert_eq!(test_layer_support(&layers), false)
    }

    #[test]
    fn layer_support_valid_collection() {
        let layers = vec![String::from("VK_LAYER_KHRONOS_profiles"),
                          String::from("VK_LAYER_KHRONOS_validation")];
        assert_eq!(test_layer_support(&layers), true)
    }

    #[test]
    fn layer_support_invalid_collection() {
        let layers = vec![String::from("random name"), String::from("VK_LAYER_KHRONOS_validation")];
        assert_eq!(test_layer_support(&layers), false)
    }

    #[test]
    fn can_get_setup_extension_names() {
        let (_guard, _, glfw_instance) = get_entries();
        match get_setup_extension_names(& glfw_instance) {
            Ok(_) => (),
            Err(e) => panic!("{e:?}"),
        }
    }

    #[test]
    fn setup_extension_names_not_empty() {
        let (_guard, _, glfw_instance) = get_entries();
        assert_ne!(get_setup_extension_names(& glfw_instance).unwrap().len(), 0);
    }

    /// A Mutex used for ensuring tests that use the rendering libraries do not run in parallel.
    static TEST_MUTEX: Mutex<()> = Mutex::new(());

    /// Returns a vulkan entry and a GLFW entry for running tests.
    pub(crate) fn get_entries() -> (MutexGuard<'static, ()>, Entry, Glfw) {
        let guard = get_test_mutex_guard();
        let entry = Entry::linked();
        let glfw = glfw::init_no_callbacks().expect("Failed to get a GLFW entry");
        (guard, entry, glfw)
    }

    /// Returns the unwrapped TEST_MUTEX.
    /// A Poisoned Mutex will just be reset to attempt running the tests anyway.
    /// Worst case the tests fail anyway, and we know the state in the mutex is fine.
    fn get_test_mutex_guard() -> MutexGuard<'static, ()>{
        match TEST_MUTEX.lock() {
            Ok(m) => m,
            Err(e) => {
                Mutex::clear_poison(&TEST_MUTEX);
                e.into_inner()
            }
        }

    }

    /// Returns a vulkan instance for running tests.
    pub(crate) fn get_vulkan_instance(vulkan_entry: &Entry, glfw_instance: &Glfw) -> Instance {
        crate::renderer::Renderer::create_vulkan_instance("test", vulkan_entry, glfw_instance)
            .unwrap()
    }

    /// Returns a window for running tests.
    pub(crate) fn get_window(glfw_instance: &mut Glfw) -> glfw::PWindow {
        crate::renderer::window_handler::create_window("test", glfw_instance).unwrap()
    }

    /// Returns a window surface for running tests.
    pub(crate) fn get_window_surface(
        vulkan_instance: &mut Instance,
        window: &glfw::PWindow
    ) -> vk::SurfaceKHR {
        crate::renderer::window_handler::create_window_surface(vulkan_instance, window).unwrap()
    }

    /// Gets a surface instance for running tests.
    pub(crate) fn get_surface_instance(
        vulkan_entry: & Entry,
        vulkan_instance: & Instance
    ) -> khr::surface::Instance{
        khr::surface::Instance::new(vulkan_entry, vulkan_instance)
    }

    /// Used by the tests in this module to validate that we are correctly testing the layers.
    fn test_layer_support(layers_string: &Vec<String>) -> bool {
        let (_guard, vulkan_entry, _) = get_entries();
        match validate_setup_layers_exist(&layers_string, &vulkan_entry) {
            Ok(()) => true,
            Err(RendererError::LayerRequiredNotSupportedError) => false,
            Err(_) => panic!("Should not happen!")
        }
    }
}