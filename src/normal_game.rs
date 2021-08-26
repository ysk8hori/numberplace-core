use std::rc::Rc;

pub mod cell;
pub mod group;
pub mod setting;

pub struct NormalGame {
    pub block_height: u8,
    pub block_width: u8,
    pub answer_candidate: Vec<u8>,
    pub cells: cell::Cells,
    pub groups: Vec<Rc<group::Group>>,
}

impl NormalGame {
    pub fn new(setting: setting::GameSetting) -> NormalGame {
        let cells = cell::Cells::create_cells(&setting);
        let groups = group::create_groups(&cells, &setting);
        NormalGame {
            block_height: setting.block_height,
            block_width: setting.block_width,
            answer_candidate: (1..=(setting.block_height * setting.block_width)).collect(),
            cells,
            groups,
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
        #[test]
        fn it_has_18_groups() {
            let game = NormalGame::new(setting::GameSetting {
                block_height: 2,
                block_width: 3,
            });
            assert_eq!(game.groups.len(), 18);
        }
    }
    mod _3_3 {
        use super::*;

        #[test]
        fn it_answer_candidate_is_1_to_9() {
            let game = NormalGame::new(setting::GameSetting {
                block_height: 3,
                block_width: 3,
            });
            assert_eq!(game.answer_candidate, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        }
        #[test]
        fn it_has_81_cells() {
            let game = NormalGame::new(setting::GameSetting {
                block_height: 3,
                block_width: 3,
            });
            assert_eq!(game.cells.len(), 81);
        }
        #[test]
        fn it_has_27_groups() {
            let game = NormalGame::new(setting::GameSetting {
                block_height: 3,
                block_width: 3,
            });
            assert_eq!(game.groups.len(), 27);
        }
    }
}
