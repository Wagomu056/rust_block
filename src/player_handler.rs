use anyhow::Error;
use crankstart::graphics::{Graphics, rect_make};
use crankstart::sprite::{Sprite, SpriteManager};
use crankstart::system::System;
use crankstart_sys::{LCDBitmapFlip, PDButtons};
use crankstart_sys::{LCD_COLUMNS, LCD_ROWS};
use euclid::{vec2, Vector2D};

pub struct PlayerHandler {
    player_sprite: Sprite,
    player_size: Vector2D<f32, f32>,
    pos: Vector2D<f32, f32>,
}

impl PlayerHandler {
    pub fn new() -> Result<PlayerHandler, Error> {
        let sprite_manager = SpriteManager::get_mut();
        let graphics = Graphics::get();

        // setup player
        let mut player = sprite_manager.new_sprite()?;
        let player_image = graphics.load_bitmap("assets/images/player_board")?;
        let player_image_data = player_image.get_data()?;
        let player_size :Vector2D<f32, f32> = vec2(
            player_image_data.width as f32,
            player_image_data.height as f32);
        let cr = rect_make(
            0.0, 0.0,
            player_size.x, player_size.y
        );

        player.set_image(player_image, LCDBitmapFlip::kBitmapUnflipped)?;
        player.set_collide_rect(&cr)?;

        let center_x = LCD_COLUMNS as f32 * 0.5;
        let start_y = LCD_ROWS as f32 - 25.0;
        player.move_to(center_x, start_y)?;
        sprite_manager.add_sprite(&player)?;

        Ok(
            PlayerHandler{
                player_sprite: player,
                player_size,
                pos: vec2(center_x, start_y),
            }
        )
    }

    pub fn update(&mut self) -> Result<(), Error> {
        let move_speed = 10.0;

        let mut new_pos = self.pos;

        let (current, _, _) = System::get().get_button_state()?;
        if (current & PDButtons::kButtonRight) == PDButtons::kButtonRight {
            new_pos.x += move_speed;
            let limit_x = LCD_COLUMNS as f32 - self.player_size.x * 0.5;
            if new_pos.x >= limit_x {
                new_pos.x = limit_x;
            }
        }
        if (current & PDButtons::kButtonLeft) == PDButtons::kButtonLeft {
            new_pos.x -= move_speed;
            let limit_x= self.player_size.x * 0.5;
            if new_pos.x <= limit_x {
               new_pos.x = limit_x;
            }
        }

        self.pos = new_pos;
        self.player_sprite.move_to(self.pos.x, self.pos.y)?;

        Ok(())
    }
}

