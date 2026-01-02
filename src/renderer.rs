use ash::{vk, Entry, Instance};

use std::ffi::{self, CString};

const VALIDATION_LAYER_NAMES: &[&str] = &["VK_LAYER_KHRONOS_validation"];

pub struct Renderer {
    instance: Instance,

    #[cfg(debug_assertions)]
    debug_utils_messenger: vk::DebugUtilsMessengerEXT,
}

impl Renderer {
    pub fn new(name: &str) -> Result<Renderer, RendererSetupError> {
        let entry = Entry::linked();

        let app_name = CString::new(name)?;

        // This unwrap is safe as we know the string passed to it does not contain a 0 char
        let engine_name = CString::new("Vulkan Engine")
            .expect("Hard coded string should not contain any 0 chars except at end");

        let app_info = vk::ApplicationInfo::default()
            .application_name(app_name.as_c_str())
            .application_version(vk::make_api_version(0, 0, 1, 0))
            .engine_name(engine_name.as_c_str())
            .engine_version(vk::make_api_version(0, 0, 1, 0))
            .api_version(vk::make_api_version(0, 1, 3, 0));

        let (_layer_names, layer_name_pointers) = Self::get_setup_layer_names();

        let available_layers = unsafe { entry.enumerate_instance_layer_properties() };

        let create_info = vk::InstanceCreateInfo::default()
            .flags(vk::InstanceCreateFlags::empty())
            .application_info(&app_info)
            .enabled_layer_names(&layer_name_pointers)
            .enabled_extension_names(&[]) //TODO Implement extensions
            //TODO setup temporary debug utils messenger
            ;



        let instance = unsafe { entry.create_instance(&create_info, None).unwrap() };
        //TODO instance creation error handling
        //TODO implement cleanup for instance

        Ok(Renderer {
            instance
        })
    }

//TODO setup debug messenger

    fn get_setup_layer_names() -> (Vec<CString>, Vec<*const i8>) {

        let mut layers = vec![];

        if cfg!(debug_assertions) {
            for layer in VALIDATION_LAYER_NAMES {
                layers.push(*layer);
            }
        }

        let layer_names : Vec<_> = layers
            .iter()
            .map(|name| CString::new(*name)
                .expect("Hard coded layer names should not contain any 0 chars"))
            .collect();

        let layer_pointers = layer_names
            .iter()
            .map(|name| name.as_ptr())
            .collect();

        (layer_names, layer_pointers)
    }
}

pub enum RendererSetupError {
    StringContainingNullCharError(ffi::NulError),
}

impl From<ffi::NulError> for RendererSetupError {
    fn from(error: ffi::NulError) -> Self {
        RendererSetupError::StringContainingNullCharError(error)
    }
}

impl From<vk::Result> for RendererSetupError {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_renderer() {
        let _ = Renderer::new("test");
    }
}