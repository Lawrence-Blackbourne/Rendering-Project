use ash::{Instance, vk};
use glfw::{self, Glfw};
use std::ptr;
use crate::renderer::RendererError;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

/// Creates the glfw window to render to
pub(crate) fn create_window(
    app_name: &str,
    glfw_instance: &mut Glfw,
) -> Result<glfw::PWindow, RendererError> {
    // We reset the window hints and then apply the ones we want
    glfw_instance.default_window_hints();
    glfw_instance.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
    glfw_instance.window_hint(glfw::WindowHint::Resizable(false));

    let result = glfw_instance.create_window(
        WIDTH,
        HEIGHT,
        app_name,
        glfw::WindowMode::Windowed
    );

    let (window, _) = match result {
        Some(result) => result,
        None => return Err(RendererError::GlfwCallFailed("create_window".to_string()))
    };

    Ok(window)
}

/// This gets the window surface. This is what the final image is presented to, to render it in the
/// window created.
/// This is done in a platform-agnostic way in this specific implementation.
pub(crate) fn create_window_surface(
    vulkan_instance: &mut Instance,
    window: &glfw::PWindow
) -> Result<vk::SurfaceKHR, RendererError> {

    let mut surface = vk::SurfaceKHR::null();

    let result = unsafe {window.create_window_surface(
        vk::Handle::as_raw(vulkan_instance.handle()) as glfw::ffi::VkInstance,
        ptr::null(),
        &raw mut surface as *mut glfw::ffi::VkSurfaceKHR,
    )};
    
    if result == vk::Result::as_raw(vk::Result::SUCCESS) {
        Ok(surface)
    } else {
        Err(RendererError::from(vk::Result::from_raw(result)))
    }
}

#[cfg(test)]
mod test{
    use super::*;
    use crate::renderer::debugger::tests;

    #[test]
    fn can_create_window() {
        let (_guard, _, mut glfw_instance) = tests::get_entries();
        match create_window("test", &mut glfw_instance) {
            Ok(_) => (),
            Err(e) => panic!("{e:?}"),
        }
    }
    
    #[test]
    fn can_create_window_surface() {
        let (_guard, vulkan_entry, mut glfw_instance) = tests::get_entries();
        let mut vulkan_instance = tests::get_vulkan_instance(& vulkan_entry, & glfw_instance);
        let window = create_window("test", &mut glfw_instance).unwrap();
        match create_window_surface(&mut vulkan_instance, & window) {
            Ok(_) => (),
            Err(e) => panic!("{e:?}"),
        }
    }
}