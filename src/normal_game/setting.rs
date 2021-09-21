#[derive(Debug, Clone)]
pub struct GameSetting {
    block_size: BlockSize,
    answer_candidate: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BlockSize {
    pub height: u8,
    pub width: u8,
}

impl GameSetting {
    pub fn new(block_size: BlockSize) -> GameSetting {
        GameSetting {
            block_size,
            answer_candidate: (1..=(block_size.height * block_size.width)).collect(),
        }
    }
    pub fn new_with_answer_candidate(
        block_size: BlockSize,
        answer_candidate: Vec<u8>,
    ) -> GameSetting {
        GameSetting {
            block_size,
            answer_candidate,
        }
    }
    pub fn side_size(&self) -> u8 {
        self.block_size.height * self.block_size.width
    }
    pub fn answer_candidate(&self) -> Vec<u8> {
        self.answer_candidate.clone()
    }
    pub fn block_height(&self) -> u8 {
        self.block_size.height
    }
    pub fn block_width(&self) -> u8 {
        self.block_size.width
    }
    pub fn block_size(&self) -> BlockSize {
        self.block_size
    }
}
