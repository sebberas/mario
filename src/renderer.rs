use glam::*;
use image::imageops;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use crate::map::*;
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
        //self.draw_sprite(&scene.sprites[0], vec2(100.0, 100.0), 2);
        // self.draw_enemies();
        // self.draw_entities();
        // self.draw_text();
        // self.draw_tiles();

        self.canvas.present();
    }

    pub fn draw_image(&mut self, sprite: &Sprite, position: UVec2, size: u32) {
        // converter sprite til islam
        let mut assetpath_image = image::open(&sprite.asset_path)
            .expect("Image not found")
            .to_rgba8();

        // bounding box
        let start_x = sprite.bounding_box.0.x as u32;
        let start_y = sprite.bounding_box.0.y as u32;
        let end_x = sprite.bounding_box.1.x as u32;
        let end_y = sprite.bounding_box.1.y as u32;

        let sprite_image = imageops::crop(&mut assetpath_image, start_x, start_y, end_x, end_y);

        for (x, y, color) in sprite_image.to_image().enumerate_pixels() {
            let col_vec = vec4(
                map_range((0.0, 255.0), (0.0, 1.0), color.0[0] as f64) as f32,
                map_range((0.0, 255.0), (0.0, 1.0), color.0[1] as f64) as f32,
                map_range((0.0, 255.0), (0.0, 1.0), color.0[2] as f64) as f32,
                map_range((0.0, 255.0), (0.0, 1.0), color.0[3] as f64) as f32,
            );

            let color = Color::from(Rgba::from(col_vec));

            self.canvas.set_draw_color(color);

            //tilføj sprite til canvas, med den rigtige size
            self.canvas
                .fill_rect(Rect::new(
                    (position.x + x * size) as _,
                    (position.y + y * size) as _,
                    size,
                    size,
                ))
                .unwrap();
        }
    }

    pub fn draw_background(&mut self, color: Rgba) {
        self.canvas.set_draw_color(Color::from(color));
        self.canvas.clear();
    }

    pub fn draw_tiles(&mut self, scene: &mut Scene) {
        let tile_size = 8;

        for tile in &scene.map_tiles {
            let tile_sprite = Sprite::new(
                (uvec2(0, 0), uvec2(16, 16)),
                String::from(tile.block.asset()),
            );

            self.draw_image(
                &tile_sprite,
                uvec2(tile.coordinate.x, tile.coordinate.y),
                tile_size,
            );
        }
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

fn map_range(from_range: (f64, f64), to_range: (f64, f64), s: f64) -> f64 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}
