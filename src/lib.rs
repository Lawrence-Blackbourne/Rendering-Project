mod renderer;

pub fn get_renderer(name: &str) -> Result<renderer::Renderer, renderer::RendererError> {
    renderer::Renderer::new(name)
}