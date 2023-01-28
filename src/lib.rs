#![no_std]

mod ball_handler;

extern crate alloc;

use crankstart_sys::{PDRect};
use {
    alloc::boxed::Box,
    anyhow::Error,
    crankstart::{
        crankstart_game,
        sprite::{Sprite},
        //geometry::{ScreenPoint, ScreenVector},
        //graphics::{Graphics, LCDColor, LCDSolidColor},
        //sprite::{SpriteManager},
        //system::System,
        Game, Playdate,
    },
    //crankstart_sys::{LCD_COLUMNS, LCD_ROWS},
    //euclid::{point2, vec2},
};
use crate::ball_handler::BallHandler;

struct State {
    ball_handler: BallHandler,
}

impl State {
    pub fn new(_playdate: &Playdate) -> Result<Box<Self>, Error> {
        crankstart::display::Display::get().set_refresh_rate(20.0)?;

        // setup ball
        let ball_handler = BallHandler::create()?;

        Ok(Box::new(Self {
            ball_handler,
        }))
    }
}

impl Game for State {
    fn update_sprite(&mut self, _sprite: &mut Sprite, _playdate: &mut Playdate) -> Result<(), Error> {
        Ok(())
    }

    fn draw_sprite(
        &self,
        _sprite: &Sprite,
        _bounds: &PDRect,
        _draw_rect: &PDRect,
        _playdate: &Playdate,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn update(&mut self, _playdate: &mut Playdate) -> Result<(), Error> {
        self.ball_handler.update()?;


        Ok(())
    }

    fn draw_fps(&self) -> bool { true }
}

crankstart_game!(State);