#[derive(Debug)]
pub struct GameSetting {
    pub block_height: u8,
    pub block_width: u8,
}
impl GameSetting {
    pub fn side_size(&self) -> u8 {
        self.block_height * self.block_width
    }
    pub fn answer_candidate(&self) -> Vec<u8> {
        (1..=(self.block_height * self.block_width)).collect()
    }
}
