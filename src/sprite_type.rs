
#[repr(u8)]
pub enum SpriteType {
    Player = 0,
    Block = 1,
}

impl From<u8> for SpriteType {
    fn from(tag: u8) -> Self {
        let sprite_type = match tag {
            0 => SpriteType::Player,
            1 => SpriteType::Block,
            _ => SpriteType::Block,
        };
        sprite_type
    }
}
