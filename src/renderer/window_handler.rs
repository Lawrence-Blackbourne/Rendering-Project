use glfw::Glfw;
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