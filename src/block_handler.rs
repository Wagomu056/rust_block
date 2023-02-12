use alloc::vec::Vec;
use anyhow::Error;
use crankstart::graphics::{Graphics, rect_make};
use crankstart::sprite::{Sprite, SpriteManager};
use crankstart::system::System;
use crankstart_sys::{LCDBitmapFlip, PDButtons};
use crankstart_sys::{LCD_COLUMNS, LCD_ROWS};
use euclid::{vec2, Vector2D};

const x_num : i32 = 7;
const y_num : i32 = 5;
const total_num : i32 = x_num * y_num;

pub struct BlockHandler {
    sprites: Vec<Sprite>,
    positions: Vec<Vector2D<f32, f32>>,
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
        let mut positions: Vec<Vector2D<f32, f32>> = Vec::new();

        let start_x = 25.0;
        let start_y = 25.0;
        for y in 0..y_num {
            for x in 0..x_num {
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
                sprite_manager.add_sprite(&sprite)?;

                sprites.push(sprite);
                positions.push(vec2(pos_x, pos_y));
            }
        }

        Ok(
            BlockHandler {
                sprites,
                positions,
            }
        )
    }
}

