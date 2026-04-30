mod renderer;

fn main() {
    let result = renderer::Renderer::new("test");
    let mut renderer = match result {
        Ok(renderer) => renderer,
        Err(e) => panic!("Error occurred!: {:?}", e)
    };
    let mut should_close = false;
    let mut frame = 0;
    while !should_close {
        should_close = match renderer.update() {
            renderer::RendererStatus::Ok => {
                println!("Running frame {frame}");
                frame += 1;
                false
            },
            renderer::RendererStatus::ShouldClose => true,
            renderer::RendererStatus::_Error(e) => {
                println!("Error occurred!: {:?}", e);
                true
            }
        }
    }
}