use crate::renderer::{Renderer, RendererError};

mod renderer;

pub fn get_renderer(name: &str) -> Result<Renderer, RendererError> {
    Renderer::new(name)
}