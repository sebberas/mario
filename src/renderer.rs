use glam::Vec2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use image::io::Reader as ImageReader;
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};

pub struct Renderer<'a> {
    pub canvas: &'a mut WindowCanvas,
}

impl Renderer<'_> {
    pub fn new(canvas: &'_ mut WindowCanvas) -> Renderer {
        Renderer { canvas }
    }

    pub fn render_image(&mut self, position: Vec2, size: u32) {
        let sprite = image::open("assets/audio/sprites/micePng.png")
            .unwrap()
            .to_rgb8();

        for (x, y, color) in sprite.enumerate_pixels() {
            let color: Color = sdl2::pixels::Color {
                r: color.0[0],
                g: color.0[1],
                b: color.0[2],
                a: 255,
            };
            self.canvas.set_draw_color(color);
            self.canvas
                .fill_rect(Rect::new(
                    (position.x as u32 + x * size) as _,
                    (position.y as u32 + y * size) as _,
                    position.x as u32 + x * size + size,
                    position.y as u32 + x * size + size,
                ))
                .unwrap();
            self.canvas.present();
        }
    }
}
