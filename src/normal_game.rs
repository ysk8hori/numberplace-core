pub mod cell;
pub mod group;
pub mod setting;

pub struct NormalGame {
    pub block_height: u8,
    pub block_width: u8,
    pub answer_candidate: Vec<u8>,
    pub cells: cell::Cells,
}

impl NormalGame {
    pub fn new(setting: setting::GameSetting) -> NormalGame {
        NormalGame {
            block_height: setting.block_height,
            block_width: setting.block_width,
            answer_candidate: (1..=(setting.block_height * setting.block_width)).collect(),
            cells: cell::Cells::create_cells(&setting),
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
            let game = NormalGame::new(setting::GameSetting {
                block_height: 2,
                block_width: 3,
            });
            assert_eq!(game.answer_candidate, vec![1, 2, 3, 4, 5, 6]);
        }
        #[test]
        fn it_has_36_cells() {
            let game = NormalGame::new(setting::GameSetting {
                block_height: 2,
                block_width: 3,
            });
            assert_eq!(game.cells.len(), 36);
        }
    }
}
