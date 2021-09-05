use std::cell::RefCell;
use std::rc::Rc;

pub mod cell;
pub mod group;
pub mod setting;

pub struct NormalGame {
    setting: setting::GameSetting,
    cells: Vec<Rc<RefCell<cell::Cell>>>,
    groups: Vec<Rc<RefCell<group::Group>>>,
    status: GameState,
}

impl NormalGame {
    pub fn new(setting: setting::GameSetting) -> NormalGame {
        let cells = cell::create_cells(&setting);
        let groups = group::create_groups(&cells, &setting);
        NormalGame {
            setting,
            cells,
            groups,
            status: GameState::Empty,
        }
    }

    pub fn setting(&self) -> &setting::GameSetting {
        &self.setting
    }
    pub fn cells(&self) -> &Vec<Rc<RefCell<cell::Cell>>> {
        &self.cells
    }
    pub fn groups(&self) -> &Vec<Rc<RefCell<group::Group>>> {
        &self.groups
    }
    pub fn status(&self) -> &GameState {
        &self.status
    }
    /// ' 7     6 |6   1   3|  54 87  |  8   4  | 1  3  5 |  9   1  |  35 12  |7   2   8| 5     9 '
    pub fn load(&mut self, issue: &str) {
        let answer_columns: Vec<&str> = issue.split("|").collect();
        for (y, horizontal_line) in answer_columns.iter().enumerate() {
            let chars: Vec<char> = horizontal_line.chars().collect();
            for (x, answer) in chars.iter().enumerate() {
                if *answer == ' ' {
                    continue;
                }
                let answer: u8 = String::from(*answer).parse().expect("issue is wrong.");
                self.cells
                    .iter()
                    .find(|c| c.borrow().pos() == cell::Position::new(x as u8, y as u8))
                    // .find_by_position(&cell::Position::new(x as u8, y as u8))
                    .unwrap()
                    .borrow_mut()
                    .set_answer(answer);
            }
        }
        self.status = GameState::Loaded;
    }

    pub fn set_answer(&self, pos: cell::Position, answer: u8) {
        self.cells()
            .iter()
            .find(|c| c.borrow().pos() == pos)
            .unwrap()
            .borrow_mut()
            .set_answer(answer);
        self.groups()
            .iter()
            .filter(|g| g.borrow().cells().iter().any(|c| c.borrow().pos() == pos))
            .for_each(|g| g.borrow_mut().remove_answer_candidate(answer));
    }

    pub fn find_cell(&self, pos: cell::Position) -> Option<&Rc<RefCell<cell::Cell>>> {
        self.cells.iter().find(|c| c.borrow().pos() == pos)
    }
}

pub enum GameState {
    Empty,
    Loaded,
    Complete,
}

#[cfg(test)]
mod tests {
    use super::*;

    const SETTING: setting::GameSetting = setting::GameSetting {
        block_height: 2,
        block_width: 3,
    };

    #[cfg(test)]
    mod _2_3 {
        use super::*;

        #[test]
        fn it_answer_candidate_is_1_to_6() {
            assert_eq!(SETTING.answer_candidate(), vec![1, 2, 3, 4, 5, 6]);
        }
        #[test]
        fn it_has_36_cells() {
            let game = NormalGame::new(SETTING);
            assert_eq!(game.cells.len(), 36);
        }
        #[test]
        fn it_has_18_groups() {
            let game = NormalGame::new(SETTING);
            assert_eq!(game.groups.len(), 18);
        }
    }
    mod _3_3 {
        use super::*;
        const SETTING: setting::GameSetting = setting::GameSetting {
            block_height: 3,
            block_width: 3,
        };

        #[test]
        fn it_answer_candidate_is_1_to_9() {
            assert_eq!(SETTING.answer_candidate(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        }
        #[test]
        fn it_has_81_cells() {
            let game = NormalGame::new(SETTING);
            assert_eq!(game.cells.len(), 81);
        }
        #[test]
        fn it_has_27_groups() {
            let game = NormalGame::new(SETTING);
            assert_eq!(game.groups.len(), 27);
        }
    }
    mod test_load {
        use super::*;
        fn game() -> NormalGame {
            let mut game = NormalGame::new(setting::GameSetting {
                block_height: 3,
                block_width: 3,
            });
            game.load(" 7     6 |6   1   3|  54 87  |  8   4  | 1  3  5 |  9   1  |  35 12  |7   2   8| 5     9 ");
            game
        }
        mod load_9_9 {
            use super::*;
            #[test]
            fn has_81_cells() {
                assert_eq!(game().cells.len(), 81);
            }

            #[test]
            fn none_0_0() {
                assert_eq!(
                    game()
                        .cells
                        .iter()
                        .find(|c| c.borrow().pos() == cell::Position::new(0, 0))
                        // .find_by_position(&cell::Position::new(0, 0))
                        .unwrap()
                        .borrow()
                        .answer(),
                    None
                );
            }
            #[test]
            fn some_1_0() {
                assert_eq!(
                    game()
                        .cells
                        .iter()
                        .find(|c| c.borrow().pos() == cell::Position::new(1, 0))
                        // .find_by_position(&cell::Position::new(1, 0))
                        .unwrap()
                        .borrow()
                        .answer(),
                    Some(7)
                );
            }
            #[test]
            fn some_2_0() {
                assert_eq!(
                    game()
                        .cells
                        .iter()
                        .find(|c| c.borrow().pos() == cell::Position::new(2, 0))
                        // .find_by_position(&cell::Position::new(2, 0))
                        .unwrap()
                        .borrow()
                        .answer(),
                    None
                );
            }
            #[test]
            fn some_7_0() {
                assert_eq!(
                    game()
                        .cells
                        .iter()
                        .find(|c| c.borrow().pos() == cell::Position::new(7, 0))
                        // .find_by_position(&cell::Position::new(7, 0))
                        .unwrap()
                        .borrow()
                        .answer(),
                    Some(6)
                );
            }
        }
    }
    mod set_answer {
        use super::*;
        #[test]
        fn test() {
            let game = NormalGame::new(SETTING);
            game.set_answer(cell::Position::new(0, 0), 2);
            assert_eq!(game.cells()[0].borrow().answer(), Some(2));
        }
        #[test]
        fn remove_unanswerd_candidate_from_groups() {
            let game = NormalGame::new(SETTING);
            game.set_answer(cell::Position::new(0, 0), 2);
            assert_eq!(game.cells()[1].borrow().has_answer_candidate(2), false);
        }
    }
}
