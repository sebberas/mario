use std::collections::HashMap;
use std::path::Path;

use glam::*;
use image::{imageops, GenericImageView, RgbaImage};
use sdl2::pixels::{Color, PixelFormat, PixelFormatEnum};
use sdl2::rect::{Point, Rect};
use sdl2::render::*;
use sdl2::video::*;

use crate::map::*;
use crate::scene;
use crate::scene::*;

pub struct Renderer {
    pub canvas: WindowCanvas,
    texture_creator: TextureCreator<WindowContext>,
    cache: HashMap<&'static str, Texture>,
}

impl Renderer {
    const TILES: u32 = 16;

    pub fn new(mut canvas: WindowCanvas) -> Renderer {
        canvas.set_blend_mode(BlendMode::Blend);

        let (width, height) = canvas.window().size();

        canvas.set_logical_size(256, 256);
        // canvas.set_integer_scale(true);
        // canvas.set_scale(height as f32 / 16f32, height as f32 / 16f32);

        let texture_creator = canvas.texture_creator();

        Renderer {
            canvas,
            texture_creator,
            cache: HashMap::with_capacity(128),
        }
    }

    pub fn update(&mut self, scene: &mut Scene) {
        self.canvas.clear();

        // self.move_camera(scene, scene.camera.position);
        self.draw_background(scene.background);
        self.draw_tiles(scene);
        // self.draw_sprites(scene);
        self.draw_player(scene);

        self.canvas.present();
    }

    pub fn draw_tiles(&mut self, scene: &mut Scene) {
        for MapTile { block, coordinate } in &scene.map_tiles {
            let sprite = block.to_sprite();
            self.draw_image(&scene.camera, &sprite, *coordinate, 1);
        }
    }

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

            // if we are unable find the texture in the cache we have to
            // load it in from disk.
            let image = image::open(asset_path).unwrap().to_rgba8();
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

    pub fn draw_background(&mut self, color: Rgba) {
        self.canvas.set_draw_color(Color::from(color));
        self.canvas.clear();
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
