pub mod setting;

#[derive(Debug)]
pub struct Game {
    pub block_height: u8,
    pub block_width: u8,
    pub answer_candidate: Vec<u8>,
}

impl Game {
    pub fn new(setting: setting::GameSetting) -> Game {
        Game {
            block_height: setting.block_height,
            block_width: setting.block_width,
            answer_candidate: (1..=(setting.block_height * setting.block_width)).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(test)]
    mod _2_3 {
        use super::*;

        #[test]
        fn it_answer_candidate_is_1_to_6() {
            let game = Game::new(setting::GameSetting {
                block_height: 2,
                block_width: 3,
            });
            assert_eq!(game.answer_candidate, vec![1, 2, 3, 4, 5, 6])
        }
    }
}
