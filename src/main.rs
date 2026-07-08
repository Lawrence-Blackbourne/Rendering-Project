use rendering_project::{self, renderer};

fn main() {
    let mut renderer = match rendering_project::get_renderer("test", 2) {
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
            renderer::RendererStatus::Error(e) => {
                println!("Error occurred!: {:?}", e);
                true
            }
        }
    }
    println!("Done")
}