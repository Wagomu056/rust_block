use alloc::vec::Vec;
use anyhow::Error;
use crankstart::graphics::{Graphics, rect_make};
use crankstart::sprite::{Sprite, SpriteManager};
use crankstart_sys::{LCDBitmapFlip};
use euclid::{vec2, Vector2D};
use crate::sprite_type;

extern crate alloc;

const X_NUM: i32 = 7;
const Y_NUM: i32 = 5;

pub struct BlockHandler {
    sprites: Vec<Sprite>,
    id_vec: Vec<u32>,
}

impl BlockHandler {
    pub fn new() -> Result<BlockHandler, Error> {
        let sprite_manager = SpriteManager::get_mut();
        let graphics = Graphics::get();

        // load image
        let image = graphics.load_bitmap("assets/images/block")?;
        let image_data = image.get_data()?;
        let image_size : Vector2D<f32, f32>= vec2(
            image_data.width as f32,
            image_data.height as f32);
        let half_image_size : Vector2D<f32, f32> = vec2(
            image_size.x * 0.5,
            image_size.y * 0.5);

        let mut sprites: Vec<Sprite> = Vec::new();
        let mut id_vec: Vec<u32> = Vec::new();

        let mut id = 0;
        let start_x = 25.0;
        let start_y = 25.0;
        for y in 0..Y_NUM {
            for x in 0..X_NUM {
                let mut sprite = sprite_manager.new_sprite()?;
                let cr = rect_make(
                    0.0, 0.0,
                    image_size.x, image_size.y
                );

                sprite.set_image(image.clone(), LCDBitmapFlip::kBitmapUnflipped)?;
                sprite.set_collide_rect(&cr)?;

                let pos_x = start_x + image_size.x * x as f32 + half_image_size.x;
                let pos_y = start_y + image_size.y * y as f32 + half_image_size.y;
                sprite.move_to(pos_x, pos_y)?;
                sprite.set_tag(sprite_type::SpriteType::Block as u8)?;

                sprite_manager.add_sprite(&sprite)?;

                sprites.push(sprite);
                id_vec.push(id);
                id += 1;
            }
        }

        Ok(
            BlockHandler {
                sprites,
                id_vec,
            }
        )
    }

    pub fn remove_sprites(&mut self, sprites: Vec<Sprite>) {
        for sprite in sprites {
            if let Some(pos) = self.sprites.iter().position(|s| *s == sprite) {
                self.sprites.remove(pos);
            }
        }
    }

    pub fn get_id(&mut self, sprite: &Sprite) -> Option<u32> {
        match self.sprites.iter().position(|s| *s == *sprite) {
            Some(pos) => {
                Some(self.id_vec[pos])
            }
            None => None
        }
    }
}

