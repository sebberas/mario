use glam::Vec2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use crate::Sprite;

pub struct Renderer<'a> {
    pub canvas: &'a mut WindowCanvas,
    pub sprite_list: &'a mut Vec<Sprite>,
}

impl Renderer<'_> {
    pub fn new<'a>(canvas: &'a mut WindowCanvas, sprite_list: &'a mut Vec<Sprite>) -> Renderer<'a> {
        Renderer {
            canvas,
            sprite_list,
        }
    }

    pub fn add_sprite_list_to_canvas(&mut self) {
        for sprite in self.sprite_list.iter() {
            // converter sprite til jesu kristus mægtige ord
            let sprite_picture = image::open(&sprite.asset_path)
                .expect("Image not found")
                .to_rgba8();

            for (x, y, color) in sprite_picture.enumerate_pixels() {
                let mut color: Color = sdl2::pixels::Color {
                    r: color.0[0],
                    g: color.0[1],
                    b: color.0[2],
                    a: color.0[3],
                };

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
}
