#[derive(Debug)]
pub struct GameSetting {
    block_height: u8,
    block_width: u8,
}

impl GameSetting {
    pub fn new(block_height: u8, block_width: u8) -> GameSetting {
        GameSetting {
            block_height,
            block_width,
        }
    }
}
