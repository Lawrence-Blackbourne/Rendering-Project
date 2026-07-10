pub mod renderer;
mod string_handler;

pub fn get_renderer(
    _name: &str,
    _num_frames: u8
) -> Result<renderer::Renderer, renderer::RendererError> {
    //TODO renderer::Renderer::new(name, num_frames)
    panic!();
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::renderer::debugger::tests;

    #[test]
    fn cannot_get_renderer_with_0_frames() {
        let (_guard, _, _) = tests::get_entries();
        match get_renderer("test", 0) {
            Ok(_) => panic!("Renderer created with no frames"),
            Err(_) => (),
        }
    }

    #[test]
    fn can_get_renderer_with_1_frame() {
        let (_guard, _, _) = tests::get_entries();
        match get_renderer("test", 1) {
            Ok(_) => (),
            Err(e) => panic!("{e:?}"),
        }
    }

    #[test]
    fn can_get_renderer_with_2_frames() {
        let (_guard, _, _) = tests::get_entries();
        match get_renderer("test", 2) {
            Ok(_) => (),
            Err(e) => panic!("{e:?}"),
        }
    }

    #[test]
    fn can_get_renderer_with_3_frames() {
        let (_guard, _, _) = tests::get_entries();
        match get_renderer("test", 3) {
            Ok(_) => (),
            Err(e) => panic!("{e:?}"),
        }
    }
}