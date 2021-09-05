use crate::normal_game::setting;
// use std::boxed::Box;
use std::cell::RefCell;
use std::rc::Rc;

type Callback = fn(u8);

#[derive(Debug, PartialEq)]
pub struct Cell {
    pos: Position,
    answer_candidate: Vec<u8>,
    answer: Option<u8>,
    on_answered_callback: Vec<Callback>,
}

impl Cell {
    pub fn new(position: Position, answer_candidate: Vec<u8>) -> Cell {
        Cell {
            pos: position,
            answer_candidate,
            answer: None,
            on_answered_callback: vec![],
        }
    }
    pub fn pos(&self) -> Position {
        self.pos
    }
    /// Deletes the specified candidate answer.
    pub fn remove_answer_candidate(&mut self, target: u8) {
        if let Ok(index) = self.answer_candidate.binary_search(&target) {
            self.answer_candidate.remove(index);
        }
    }
    /// Cell to confirm the answer when there is only one candidate left, and return the answer.
    pub fn try_fill_own_answer(&mut self) -> Option<u8> {
        if self.answer_candidate.len() == 1 {
            self.answer = Some(self.answer_candidate[0]);
            self.answer_candidate.clear();
            return self.answer;
        }
        return None;
    }
    /// Fill in the Cell with your answer and clear the answer suggestions.
    pub fn set_answer(&mut self, answer: u8) {
        self.answer = Some(answer);
        self.answer_candidate.clear();
        for cb in self.on_answered_callback.iter() {
            cb(answer);
        }
    }

    pub fn has_answer_candidate(&self, candidate: u8) -> bool {
        self.answer_candidate.iter().find(|a| **a == candidate) != None
    }

    pub fn answer(&self) -> Option<u8> {
        self.answer
    }
}

/// Position(x, y)
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Position(u8, u8);
impl Position {
    pub fn new(x: u8, y: u8) -> Position {
        Position(x, y)
    }
    pub fn x(&self) -> u8 {
        self.0
    }
    pub fn y(&self) -> u8 {
        self.1
    }
    pub fn move_x(&self, count: u8) -> Position {
        Position(self.0 + count, self.1)
    }
    pub fn move_y(&self, count: u8) -> Position {
        Position(self.0, self.1 + count)
    }
}

pub fn create_cells(setting: &setting::GameSetting) -> Vec<Rc<RefCell<Cell>>> {
    let mut cells = Vec::new();
    for x in 0..setting.side_size() {
        for y in 0..setting.side_size() {
            cells.push(Rc::new(RefCell::new(Cell::new(
                Position(x, y),
                setting.answer_candidate(),
            ))));
        }
    }
    cells
}

#[cfg(test)]
mod tests {
    use super::*;
    const SETTING: setting::GameSetting = setting::GameSetting {
        block_height: 2,
        block_width: 3,
    };
    mod test_gamesetting {
        use super::*;
        #[test]
        fn answer_candidate_is_generated_each_time() {
            let mut candidate1 = SETTING.answer_candidate();
            let candidate2 = SETTING.answer_candidate();
            candidate1.remove(0);
            assert_ne!(candidate1, candidate2);
        }
    }
    mod call_create_cells {
        use super::*;
        mod given_3_2 {
            use super::*;
            #[test]
            fn returns_36_cells() {
                assert_eq!(create_cells(&SETTING).len(), 36)
            }
            #[test]
            fn first_cell_position_is_0_0() {
                assert_eq!(create_cells(&SETTING)[0].borrow().pos, Position(0, 0));
            }
            #[test]
            fn second_cell_position_is_0_1() {
                assert_eq!(create_cells(&SETTING)[1].borrow().pos, Position(0, 1));
            }
            #[test]
            fn last_cell_position_is_5_5() {
                assert_eq!(create_cells(&SETTING)[35].borrow().pos, Position(5, 5));
            }
        }
        mod given_3_3 {
            use super::*;
            const SETTING: setting::GameSetting = setting::GameSetting {
                block_height: 3,
                block_width: 3,
            };
            #[test]
            fn returns_81_cells() {
                assert_eq!(create_cells(&SETTING).len(), 81)
            }
            #[test]
            fn last_cell_position_is_8_8() {
                assert_eq!(create_cells(&SETTING)[80].borrow().pos, Position(8, 8));
            }
        }
    }
    mod test_position {
        use super::*;
        #[test]
        fn test_assert_eq() {
            assert_eq!(Position::new(1, 2), Position::new(1, 2));
        }
        #[test]
        fn test_move_x() {
            assert_eq!(Position::new(1, 2).move_x(3), Position::new(4, 2))
        }
        #[test]
        fn test_move_y() {
            assert_eq!(Position::new(1, 2).move_y(3), Position::new(1, 5))
        }
    }
    mod test_cell_utilities {
        use super::*;
        #[test]
        fn cell_can_remove_answer_candidate() {
            let mut cell = Cell::new(Position(1, 1), SETTING.answer_candidate());
            assert_eq!(cell.answer_candidate, [1, 2, 3, 4, 5, 6]);
            cell.remove_answer_candidate(4);
            assert_eq!(cell.answer_candidate, [1, 2, 3, 5, 6]);
            cell.remove_answer_candidate(4);
            assert_eq!(cell.answer_candidate, [1, 2, 3, 5, 6]);
        }
        #[test]
        fn test_remove_all_candidate() {
            let mut cell = Cell::new(Position(1, 1), SETTING.answer_candidate());
            assert_eq!(cell.answer_candidate, [1, 2, 3, 4, 5, 6]);
            cell.remove_answer_candidate(1);
            cell.remove_answer_candidate(2);
            cell.remove_answer_candidate(3);
            cell.remove_answer_candidate(4);
            cell.remove_answer_candidate(5);
            cell.remove_answer_candidate(6);
            assert_eq!(cell.answer_candidate, []);
        }
        mod try_fill_own_answer {
            use super::*;

            #[test]
            fn test_try_fill_own_answer() {
                let mut cell = Cell::new(Position(1, 1), SETTING.answer_candidate());
                assert_eq!(cell.answer_candidate, [1, 2, 3, 4, 5, 6]);
                assert_eq!(cell.answer, None);
                cell.remove_answer_candidate(1);
                cell.remove_answer_candidate(2);
                cell.remove_answer_candidate(4);
                cell.remove_answer_candidate(5);
                cell.remove_answer_candidate(6);
                assert_eq!(cell.answer, None);
                cell.try_fill_own_answer();
                assert_eq!(cell.answer, Some(3));
                assert_eq!(cell.answer_candidate, []);
            }
            #[test]
            fn clear_candidate_when_setted_answer() {
                let mut cell = Cell::new(Position(1, 1), SETTING.answer_candidate());
                assert_eq!(cell.answer_candidate, [1, 2, 3, 4, 5, 6]);
                cell.remove_answer_candidate(1);
                cell.remove_answer_candidate(2);
                cell.remove_answer_candidate(4);
                cell.remove_answer_candidate(5);
                cell.remove_answer_candidate(6);
                cell.try_fill_own_answer();
                assert_eq!(cell.answer_candidate, []);
            }
        }
        mod set_answer {
            use super::*;
            #[test]
            fn clear_candidate() {
                let mut cell = Cell::new(Position(1, 1), SETTING.answer_candidate());
                assert_eq!(cell.answer_candidate, [1, 2, 3, 4, 5, 6]);
                cell.set_answer(4);
                assert_eq!(cell.answer_candidate, []);
            }
        }
        #[test]
        fn has_answer_candidate_returns_true_when_candidate_not_exists() {
            let mut cell = Cell::new(Position(1, 1), SETTING.answer_candidate());
            cell.remove_answer_candidate(1);
            cell.remove_answer_candidate(3);
            cell.remove_answer_candidate(5);
            assert_eq!(cell.has_answer_candidate(3), false);
        }
        #[test]
        fn has_answer_candidate_returns_true_when_candidate_exists() {
            let mut cell = Cell::new(Position(1, 1), SETTING.answer_candidate());
            cell.remove_answer_candidate(1);
            cell.remove_answer_candidate(3);
            cell.remove_answer_candidate(5);
            assert_eq!(cell.has_answer_candidate(2), true);
        }
        mod test_answer {
            use super::*;
            #[test]
            fn return_answer_if_answered() {
                let mut cell = Cell::new(Position(1, 1), SETTING.answer_candidate());
                cell.set_answer(1);
                assert_eq!(cell.answer(), Some(1));
            }
        }
    }
}
