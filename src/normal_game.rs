use std::cell::RefCell;
use std::rc::Rc;

pub mod cell;
pub mod group;
pub mod remove_answer;
pub mod setting;
pub mod shuffle;
pub mod solve;

pub struct NormalGame {
    setting: setting::GameSetting,
    cells: Vec<Rc<RefCell<cell::Cell>>>,
    groups: Vec<Rc<RefCell<group::Group>>>,
    answered_count: u32,
}

impl NormalGame {
    pub fn new(setting: setting::GameSetting) -> NormalGame {
        let cells = cell::create_cells(&setting);
        let groups = group::create_groups(&cells, &setting);
        NormalGame {
            setting,
            cells,
            groups,
            answered_count: 0,
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
    pub fn answered_count(&self) -> u32 {
        self.answered_count
    }
    /// ' 7     6 |6   1   3|  54 87  |  8   4  | 1  3  5 |  9   1  |  35 12  |7   2   8| 5     9 '
    pub fn load(&mut self, issue: &str) {
        let answer_columns: Vec<&str> = issue.split("|").collect();
        for (y, horizontal_line) in answer_columns.iter().enumerate() {
            let horizontal_line = horizontal_line.to_string();
            let answers: Vec<String> = if horizontal_line.contains(',') {
                horizontal_line.split(',').map(|s| s.to_string()).collect()
            } else {
                horizontal_line.chars().map(|c| format!("{}", c)).collect()
            };
            for (x, answer) in answers.iter().enumerate() {
                if *answer == " " || *answer == "" {
                    continue;
                }
                let answer: u8 = answer.parse().expect("issue is wrong.");
                self.set_answer(cell::Position::new(x as u8, y as u8), answer);
            }
        }
    }

    pub fn set_answer(&mut self, pos: cell::Position, answer: u8) {
        let cell = self.find_cell(pos).unwrap();
        if cell.borrow().answer().is_some() {
            return;
        }
        cell.borrow_mut().set_answer(answer);
        self.groups()
            .iter()
            .filter(|g| g.borrow().cells().iter().any(|c| c.borrow().pos() == pos))
            .for_each(|g| g.borrow_mut().remove_answer_candidate(answer));
        self.answered_count += 1;
    }

    pub fn check_status(&mut self) -> GameState {
        if self
            .cells()
            .iter()
            .filter(|c| c.borrow().answer() == None)
            .any(|c| c.borrow().answer_candidate_count() == 0)
        {
            return GameState::Failure;
        }

        if self
            .groups()
            .iter()
            .any(|g| g.borrow().is_duplicate_answer())
        {
            return GameState::Failure;
        }

        if (self.setting.side_size() as u32 * self.setting.side_size() as u32)
            == self.answered_count
        {
            return if self.is_all_clear_groups_answer_candidate() {
                GameState::Complete
            } else {
                GameState::Failure
            };
        }
        return GameState::Solving;
    }

    fn is_all_clear_groups_answer_candidate(&self) -> bool {
        self.groups()
            .iter()
            .all(|g| g.borrow().is_all_clear_answer_candidate())
    }

    pub fn find_cell(&self, pos: cell::Position) -> Option<&Rc<RefCell<cell::Cell>>> {
        self.cells.iter().find(|c| c.borrow().pos() == pos)
    }

    pub fn to_string(&self) -> String {
        let mut str = String::new();
        for y in 0..self.setting.side_size() {
            for x in 0..self.setting.side_size() {
                let str2 = match self
                    .find_cell(cell::Position::new(x, y))
                    .unwrap()
                    .borrow()
                    .answer()
                {
                    Some(a) => a.to_string(),
                    None => " ".to_string(),
                };
                str = format!("{}{}", str, str2);
            }
            str = format!("{}{}", str, '|');
        }
        str.pop();
        return str;
    }
    pub fn to_string_with_comma(&self) -> String {
        let mut str = String::new();
        for y in 0..self.setting.side_size() {
            for x in 0..self.setting.side_size() {
                let str2 = match self
                    .find_cell(cell::Position::new(x, y))
                    .unwrap()
                    .borrow()
                    .answer()
                {
                    Some(a) => a.to_string(),
                    None => " ".to_string(),
                };
                str = format!("{}{}{}", str, if x == 0 { "" } else { "," }, str2);
            }
            str = format!("{}{}", str, '|');
        }
        str.pop();
        return str;
    }
    pub fn to_string_with_newline(&self) -> String {
        let mut str = String::new();
        for y in 0..self.setting.side_size() {
            for x in 0..self.setting.side_size() {
                let str2 = match self
                    .find_cell(cell::Position::new(x, y))
                    .unwrap()
                    .borrow()
                    .answer()
                {
                    Some(a) => a.to_string(),
                    None => " ".to_string(),
                };
                str = format!("{}{}{}", str, if x == 0 { "" } else { "," }, str2);
            }
            str = format!("{}{}", str, '\n');
        }
        str.pop();
        return str;
    }
}

impl PartialEq for NormalGame {
    fn eq(&self, other: &Self) -> bool {
        self.setting().block_size() == other.setting().block_size()
            && self.to_string_with_comma() == other.to_string_with_comma()
    }
}

impl Clone for NormalGame {
    fn clone(&self) -> Self {
        let mut new_game = NormalGame::new(self.setting.clone());
        self.cells()
            .iter()
            .filter(|c| c.borrow().answer() != None)
            .for_each(|c| new_game.set_answer(c.borrow().pos(), c.borrow().answer().unwrap()));
        new_game
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GameState {
    Solving,
    Complete,
    Failure,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::normal_game::setting::BlockSize;
    use crate::normal_game::setting::GameSetting;
    fn setting() -> GameSetting {
        GameSetting::new(BlockSize {
            height: 2,
            width: 3,
        })
    }

    #[cfg(test)]
    mod _2_3 {
        use super::*;

        #[test]
        fn it_answer_candidate_is_1_to_6() {
            assert_eq!(setting().answer_candidate(), vec![1, 2, 3, 4, 5, 6]);
        }
        #[test]
        fn it_has_36_cells() {
            let game = NormalGame::new(setting());
            assert_eq!(game.cells.len(), 36);
        }
        #[test]
        fn it_has_18_groups() {
            let game = NormalGame::new(setting());
            assert_eq!(game.groups.len(), 18);
        }
    }
    mod _3_3 {
        use super::*;
        fn setting() -> GameSetting {
            GameSetting::new(BlockSize {
                height: 3,
                width: 3,
            })
        }

        #[test]
        fn it_answer_candidate_is_1_to_9() {
            assert_eq!(
                setting().answer_candidate(),
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
            );
        }
        #[test]
        fn it_has_81_cells() {
            let game = NormalGame::new(setting());
            assert_eq!(game.cells.len(), 81);
        }
        #[test]
        fn it_has_27_groups() {
            let game = NormalGame::new(setting());
            assert_eq!(game.groups.len(), 27);
        }
    }
    mod test_load {
        use super::*;
        const GAME_STRING:&str = " 7     6 |6   1   3|  54 87  |  8   4  | 1  3  5 |  9   1  |  35 12  |7   2   8| 5     9 ";
        fn game() -> NormalGame {
            let mut game = NormalGame::new(setting::GameSetting::new(BlockSize {
                height: 3,
                width: 3,
            }));
            game.load(GAME_STRING);
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
        #[test]
        fn test_to_string() {
            assert_eq!(game().to_string(), GAME_STRING.to_string());
        }
        #[test]
        fn test_load_9_9_2() {
            let mut game = NormalGame::new(setting::GameSetting::new(BlockSize {
                height: 3,
                width: 3,
            }));
            game.load(
                "4       1| 5   1 4 |  8 476  | 79|  3 7 2|      59|  681 9| 4 9   7|2       5",
            );
            assert_eq!(game.to_string(), "4       1| 5   1 4 |  8 476  | 79      |  3 7 2  |      59 |  681 9  | 4 9   7 |2       5")
        }
        #[test]
        fn test_load_12x12() {
            let mut game = NormalGame::new(setting::GameSetting::new(BlockSize {
                height: 3,
                width: 4,
            }));
            game.load(
                " , , ,6, , , , ,8| , , , ,12,10,5,11| , ,10,4, ,9,7, ,1,11|10, ,3, , , , , , ,7, ,12| ,5, , , ,12,10, , , ,9| ,7,8, ,9, , ,2, ,5,10| ,1,7, ,8, , ,6, ,3,4,| ,10, , , ,5,1, , , ,2|11, ,4, , , , , , ,12, ,7| , ,9,10, ,8,4, ,3,6,| , , , ,2,1,6,9,| , , ,11, , , , ,9",
            );
            assert_eq!(game.to_string_with_comma(), " , , ,6, , , , ,8, , , | , , , ,12,10,5,11, , , , | , ,10,4, ,9,7, ,1,11, , |10, ,3, , , , , , ,7, ,12| ,5, , , ,12,10, , , ,9, | ,7,8, ,9, , ,2, ,5,10, | ,1,7, ,8, , ,6, ,3,4, | ,10, , , ,5,1, , , ,2, |11, ,4, , , , , , ,12, ,7| , ,9,10, ,8,4, ,3,6, , | , , , ,2,1,6,9, , , , | , , ,11, , , , ,9, , , ")
        }
    }
    mod set_answer {
        use super::*;
        #[test]
        fn test() {
            let mut game = NormalGame::new(setting());
            game.set_answer(cell::Position::new(0, 0), 2);
            assert_eq!(game.cells()[0].borrow().answer(), Some(2));
        }
        #[test]
        fn remove_unanswerd_candidate_from_groups() {
            let mut game = NormalGame::new(setting());
            game.set_answer(cell::Position::new(0, 0), 2);
            assert_eq!(game.cells()[1].borrow().has_answer_candidate(2), false);
        }
    }
}
