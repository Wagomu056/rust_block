use anyhow::Error;
use crankstart::graphics::{Graphics, rect_make};
use crankstart::sprite::{Sprite, SpriteManager};
use crankstart::system::System;
use crankstart_sys::{LCDBitmapFlip, PDButtons};
use crankstart_sys::{LCD_COLUMNS, LCD_ROWS};
use euclid::{vec2, Vector2D};

pub struct BlockHandler {
    sprite: Sprite,
    image_size: Vector2D<f32, f32>,
    pos: Vector2D<f32, f32>,
}

impl BlockHandler {
    pub fn new() -> Result<BlockHandler, Error> {
        let sprite_manager = SpriteManager::get_mut();
        let graphics = Graphics::get();

        // setup player
        let mut sprite = sprite_manager.new_sprite()?;
        let image = graphics.load_bitmap("assets/images/block")?;
        let image_data = image.get_data()?;
        let image_size :Vector2D<f32, f32> = vec2(
            image_data.width as f32,
            image_data.height as f32);
        let cr = rect_make(
            0.0, 0.0,
            image_size.x, image_size.y
        );

        sprite.set_image(image, LCDBitmapFlip::kBitmapUnflipped)?;
        sprite.set_collide_rect(&cr)?;

        let center_x = 25.0 + image_size.x * 0.5;
        let start_y = 25.0 + image_size.y * 0.5;
        sprite.move_to(center_x, start_y)?;
        sprite_manager.add_sprite(&sprite)?;

        Ok(
            BlockHandler {
                sprite,
                image_size,
                pos: vec2(center_x, start_y),
            }
        )
    }
}

