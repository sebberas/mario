use glam::*;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use crate::scene::{self, Rgba, Scene};
use crate::Sprite;

use crate::scene::*;

pub struct Renderer<'a> {
    pub canvas: &'a mut WindowCanvas,
}

impl Renderer<'_> {
    pub fn new<'a>(canvas: &'a mut WindowCanvas) -> Renderer<'a> {
        Renderer { canvas }
    }

    pub fn update(&mut self, scene: &mut Scene) {
        self.draw_background(scene.background);
        self.draw_image(&scene.sprites);
        // self.draw_enemies();
        // self.draw_entities();
        // self.draw_text();
        // self.draw_tiles();

        self.canvas.present();
    }

    pub fn draw_image(&mut self, sprites: &[Sprite]) {
        for sprite in sprites {
            // converter sprite til islam
            let sprite_picture = image::open(&sprite.asset_path)
                .expect("Image not found")
                .to_rgba8();

            for (x, y, color) in sprite_picture.enumerate_pixels() {
                let col_vec = vec4(
                    (color.0[0] * (color.0[3] / 255)) as f32,
                    (color.0[1] * (color.0[3] / 255)) as f32,
                    (color.0[2] * (color.0[3] / 255)) as f32,
                    color.0[3] as f32,
                );
                let color = Color::from(Rgba::from(col_vec));

                self.canvas.set_draw_color(color);

                //tilføj sprite til canvas, med den rigtige size
                self.canvas
                    .fill_rect(Rect::new(
                        (sprite.position.x as u32 + x * sprite.size) as _,
                        (sprite.position.y as u32 + y * sprite.size) as _,
                        sprite.size,
                        sprite.size,
                    ))
                    .unwrap();
            }
        }
    }

    pub fn draw_background(&mut self, color: Rgba) {
        self.canvas.set_draw_color(Color::from(color));
        self.canvas.clear();
    }

    pub fn draw_tiles(&mut self) {
        todo!()
    }

    pub fn draw_text(&mut self) {
        todo!()
    }

    pub fn draw_entities(&mut self) {
        todo!()
    }

    pub fn draw_enemies(&mut self) {
        todo!()
    }
}
