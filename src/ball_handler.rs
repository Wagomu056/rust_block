use anyhow::Error;
use alloc::{boxed::Box};
use crankstart::graphics::{Graphics, rect_make};
use crankstart::log_to_console;
use crankstart::sprite::{Sprite, SpriteCollider, SpriteManager};
use crankstart_sys::{LCDBitmapFlip, SpriteCollisionResponseType};
use crankstart_sys::{LCD_COLUMNS, LCD_ROWS};
use euclid::{vec2, Vector2D};
use crate::sprite_type::SpriteType;

#[derive(Debug)]
struct OverlapCollider;

impl SpriteCollider for OverlapCollider {
    fn response_type(&self, _: Sprite, _: Sprite) -> SpriteCollisionResponseType {
        SpriteCollisionResponseType::kCollisionTypeOverlap
    }
}

pub struct BallHandler {
    ball_sprite: Sprite,
    pos: Vector2D<f32, f32>,
    vel: Vector2D<f32, f32>,
    hit_skip_frame: u32,
}

impl BallHandler {
    pub fn create() -> Result<BallHandler, Error> {
        let sprite_manager = SpriteManager::get_mut();
        let graphics = Graphics::get();

        // setup ball
        let mut ball = sprite_manager.new_sprite()?;
        let ball_image = graphics.load_bitmap("assets/images/bullet")?;
        let ball_image_data = ball_image.get_data()?;
        let rect_size :Vector2D<f32, f32> = vec2(
            ball_image_data.width as f32,
            ball_image_data.height as f32);
        let cr = rect_make(
            0.0, 0.0,
            rect_size.x, rect_size.y
        );

        ball.set_image(ball_image, LCDBitmapFlip::kBitmapUnflipped)?;
        ball.set_collide_rect(&cr)?;
        ball.set_collision_response_type(Some(Box::new(OverlapCollider {})))?;
        ball.move_to( 100.0, 100.0 )?;
        sprite_manager.add_sprite(&ball)?;

        Ok(
            BallHandler{
                ball_sprite: ball,
                pos: vec2(100.0, 100.0),
                vel: vec2(5.0, 5.0),
                hit_skip_frame: 0,
                //vel: vec2(0.0, 3.0),
            }
        )
    }

    pub fn update(&mut self) -> Result<(), Error> {
        let mut new_pos = self.pos + self.vel;

        let lim_x = LCD_COLUMNS as f32;
        if new_pos.x < 0.0 || new_pos.x > lim_x {
            self.vel.x *= -1.0;

            if new_pos.x < 0.0 { new_pos.x = 0.0; }
            else { new_pos.x = lim_x; }
        }

        let lim_y = LCD_ROWS as f32;
        if new_pos.y < 0.0 || new_pos.y > lim_y {
            self.vel.y *= -1.0;

            if new_pos.y < 0.0 { new_pos.y = 0.0; }
            else { new_pos.y = lim_y; }
        }

        self.pos = new_pos;
        let (_, hit_y, collisions) =
            self.ball_sprite.move_with_collisions(
                self.pos.x, self.pos.y)?;

        for collision in collisions.iter() {
            let tag = collision.other.get_tag()?;
            if tag == SpriteType::Player as u8 {
                if hit_y > self.pos.y {
                    continue;
                }
                if self.vel.y > 0.0 {
                    self.vel.y *= -1.0;
                }
            }
            else {
                if self.hit_skip_frame == 0 {
                    let normal = collision.info.normal;
                    if normal.y != 0 {
                        self.vel.y *= -1.0;
                    }
                    else if normal.x != 0 {
                        self.vel.x *= -1.0;
                    }
                }
                self.hit_skip_frame = 3;
            }
        }

        if self.hit_skip_frame > 0 {
            self.hit_skip_frame -= 1;
        }
        Ok(())
    }

}

