use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

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
    creator: TextureCreator<WindowContext>,
    cache: HashMap<&'static str, (Texture, usize)>,
}

impl Renderer {
    pub const TILES_X: u32 = 25;
    pub const TILES_Y: u32 = 18;
    pub const TILE_SIZE: u32 = 16;

    pub fn new(mut canvas: WindowCanvas) -> Renderer {
        canvas.set_blend_mode(BlendMode::Blend);

        canvas
            .set_logical_size(
                Self::TILES_X * Self::TILE_SIZE,
                Self::TILES_Y * Self::TILE_SIZE,
            )
            .unwrap();

        let creator = canvas.texture_creator();

        Renderer {
            canvas,
            creator,
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
            self.draw_image(&scene.camera, &sprite, *coordinate * 16, 1);
        }
    }

    pub fn draw_image(&mut self, camera: &Camera, sprite: &Sprite, mut position: UVec2, size: u32) {
        let Self {
            cache,
            canvas,
            creator,
        } = self;

        let Sprite {
            asset_path,
            bounding_box,
            mirror,
        } = sprite;

        let texture = if let Some((texture, _)) = cache.get(asset_path) {
            texture
        } else {
            let image = image::open(asset_path).unwrap();
            let image = image.to_rgba8();

            let pixels: Vec<_> = image
                .pixels()
                .flat_map(|color| {
                    // Both #00298C and #9290FF are used as background color on the tilesheets. This
                    // should just be made transparent. Since we cache the texture, we
                    // only pay the price of clearing these pixels once.
                    if color != &Rgba([0, 41, 140, 255]) && color != &Rgba([146, 144, 255, 255]) {
                        color.0
                    } else {
                        [0, 0, 0, 0]
                    }
                })
                .collect();

            let mut texture = creator
                .create_texture_static(PixelFormatEnum::ABGR8888, image.width(), image.height())
                .unwrap();

            let pitch = image.width() as usize * std::mem::size_of::<[u8; 4]>();
            texture.update(None, &pixels, pitch).unwrap();
            texture.set_blend_mode(BlendMode::Blend);

            cache.insert(asset_path, (texture, 1));
            cache.get(asset_path).map(|(texture, _)| texture).unwrap()
        };

        let [x, y] = bounding_box.0.as_ref();
        let [width, height] = bounding_box.1.as_ref();

        position += camera.position.as_uvec2();

        canvas
            .copy_ex(
                texture,
                Rect::new(*x as _, *y as _, *width, *height),
                Rect::new(position.x as _, position.y as _, *width, *height),
                0.0,
                None,
                *mirror,
                false,
            )
            .unwrap();
    }

    pub fn draw_background(&mut self, color: scene::Rgba) {
        self.canvas.set_draw_color(Color::from(color));
        self.canvas.fill_rect(None).unwrap();
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
