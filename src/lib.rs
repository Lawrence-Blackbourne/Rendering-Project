pub mod renderer;
mod string_handler;

pub fn get_renderer(name: &str, num_frames: u8) -> Result<renderer::Renderer, renderer::RendererError> {
    renderer::Renderer::new(name, num_frames)
}