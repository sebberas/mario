use sdl2::image::LoadTexture;
use sdl2::render::{Texture, WindowCanvas};

pub struct Renderer<'a> {
    canvas: &'a mut WindowCanvas,
}

impl Renderer<'_> {
    fn new(&mut self, canvas: &mut WindowCanvas) -> Self {
        Self { canvas }
    }

    fn render_image(&mut self) {
        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator.load_texture("texture_test.png");
    }
}
