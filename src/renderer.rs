use std::collections::HashMap;
use std::path::Path;

use ::glam::*;
use ::image::*;
use ::sdl2::pixels::{Color, PixelFormat, PixelFormatEnum};
use ::sdl2::rect::{Point, Rect};
use ::sdl2::render::*;
use ::sdl2::video::*;

use crate::map::*;
use crate::scene;
use crate::scene::*;

pub struct Renderer {
    pub canvas: WindowCanvas,
    texture_creator: TextureCreator<WindowContext>,
    cache: HashMap<&'static str, Texture>,
}

impl Renderer {
    const TILES_X: u32 = 25;
    const TILES_Y: u32 = 18;
    const TILE_SIZE: u32 = 16;

    pub fn new(mut canvas: WindowCanvas) -> Renderer {
        canvas.set_blend_mode(BlendMode::Blend);

        canvas
            .set_logical_size(
                Self::TILES_X * Self::TILE_SIZE,
                Self::TILES_Y * Self::TILE_SIZE,
            )
            .unwrap();

        let texture_creator = canvas.texture_creator();

        Renderer {
            canvas,
            texture_creator,
            cache: HashMap::with_capacity(128),
        }
    }

    pub fn update(&mut self, scene: &mut Scene) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        // self.move_camera(scene, scene.camera.position);
        self.draw_background(scene::Rgba::from(scene.background.as_vec3() / 255.0));
        self.draw_tiles(scene);

        for enemy in &scene.enemies {
            let sprite = enemy.to_sprite();
            self.draw_image(&scene.camera, &sprite, enemy.position.as_uvec2(), 1);
        }

        for entity in &scene.entities {
            let sprite = entity.to_sprite();
            self.draw_image(&scene.camera, &sprite, entity.position, 1);
        }

        // self.draw_sprites(scene);
        self.draw_player(scene);

        self.canvas.present();
    }

    pub fn draw_tiles(&mut self, scene: &mut Scene) {
        for MapTile { block, coordinate } in &scene.tiles {
            let sprite = block.to_sprite();
            self.draw_image(&scene.camera, &sprite, *coordinate, 1);
        }
    }

    // TODO: Figure out how to sub-sample textures.
    pub fn draw_image(&mut self, camera: &Camera, sprite: &Sprite, mut position: UVec2, size: u32) {
        let Self { cache, canvas, .. } = self;

        let Sprite {
            bounding_box,
            asset_path,
        } = sprite;

        let texture = if let Some(texture) = cache.get(asset_path) {
            texture
        } else {
            let UVec2 { x, y } = bounding_box.0;
            let [width, height] = bounding_box.1.to_array();

            // If we are unable find the texture in the cache we have to
            // load it in from disk.
            let mut image = image::open(asset_path).unwrap().to_rgba8();

            // #00298C  or #9290FF is used as background color on the tilesheets. This
            // should just be made transparent. Since we cache the texture, we
            // only pay the price of clearing these pixels once.
            for x in 0..image.width() {
                for y in 0..image.height() {
                    let pixel = image.get_pixel(x, y);
                    if pixel == &Rgba([0, 41, 140, 255]) || pixel == &Rgba([146, 144, 255, 255]) {
                        // image.put_pixel(x, y, Rgba([0, 0, 0, 0]));
                    }
                }
            }

            let image_view = image.view(x, y, width, height);
            let pixels: Vec<_> = image_view
                .pixels()
                .flat_map(|(_, _, color)| color.0)
                .collect();

            let mut texture = self
                .texture_creator
                .create_texture_streaming(PixelFormatEnum::ABGR8888, width, height)
                .unwrap();

            texture.set_blend_mode(BlendMode::Blend);

            texture.update(None, &pixels, (width * 4) as _).unwrap();

            cache.insert(asset_path, texture);
            cache.get(asset_path).unwrap()
        };

        position += camera.position.as_uvec2();

        canvas
            .copy(
                texture,
                None,
                Rect::new(
                    position.x as _,
                    position.y as _,
                    bounding_box.1.x,
                    bounding_box.1.y,
                ),
            )
            .unwrap();
    }

    pub fn draw_background(&mut self, color: scene::Rgba) {
        self.canvas.set_draw_color(Color::from(color));
        self.canvas.fill_rect(None).unwrap();
    }

    pub fn draw_sprites(&mut self, scene: &mut Scene) {
        let sprite = scene.player.to_sprite();
    }

    pub fn draw_player(&mut self, scene: &mut Scene) {
        let position = uvec2(
            scene.player.position.x as u32,
            scene.player.position.y as u32,
        );

        self.draw_image(&scene.camera, &scene.player.to_sprite(), position, 1);
    }

    pub fn draw_text(&mut self) {
        todo!()
    }

    pub fn move_camera(&mut self, scene: &mut Scene, camera_movement: Vec2) -> Vec2 {
        scene.camera.position + camera_movement
    }
}

fn map_range(from_range: (f32, f32), to_range: (f32, f32), s: f32) -> f32 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}
