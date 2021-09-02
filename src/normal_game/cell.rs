use crate::normal_game::setting;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Cell {
    pos: Position,
    answer_candidate: Vec<u8>,
    answer: Option<u8>,
}

impl Cell {
    pub fn new(position: Position, answer_candidate: Vec<u8>) -> Cell {
        Cell {
            pos: position,
            answer_candidate,
            answer: None,
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
    /// Cell to confirm the answer when there is only one candidate left.
    pub fn try_fill_own_answer(&mut self) {
        if self.answer_candidate.len() == 1 {
            self.answer = Some(self.answer_candidate[0]);
            self.answer_candidate.clear();
        }
        return;
    }
    /// Fill in the Cell with your answer and clear the answer suggestions.
    pub fn set_answer(&mut self, answer: u8) {
        self.answer = Some(answer);
        self.answer_candidate.clear();
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

#[derive(Debug)]
pub struct Cells {
    cells: Vec<Rc<RefCell<Cell>>>,
}

pub fn create_cells(setting: &setting::GameSetting) -> Cells {
    let mut cells = Vec::new();
    for x in 0..setting.side_size() {
        for y in 0..setting.side_size() {
            cells.push(Rc::new(RefCell::new(Cell::new(
                Position(x, y),
                setting.answer_candidate(),
            ))));
        }
    }
    Cells { cells }
}

impl Cells {
    pub fn new(cells: Vec<Rc<RefCell<Cell>>>) -> Cells {
        Cells { cells }
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }
    pub fn filter<P>(&self, predicate: P) -> Cells
    where
        P: FnMut(&&Rc<RefCell<Cell>>) -> bool,
    {
        Cells {
            cells: self
                .cells
                .iter()
                .filter(predicate)
                .map(|c| c.clone())
                .collect(),
        }
    }
    pub fn filter_by_x(&self, x: u8) -> Cells {
        self.filter(|c| c.borrow().pos.x() == x)
    }
    pub fn filter_by_y(&self, y: u8) -> Cells {
        self.filter(|c| c.borrow().pos.y() == y)
    }
    pub fn find<P>(&self, predicate: P) -> Option<Rc<RefCell<Cell>>>
    where
        P: FnMut(&&Rc<RefCell<Cell>>) -> bool,
    {
        match self.cells.iter().find(predicate) {
            Some(rc) => Some(rc.clone()),
            None => None,
        }
    }
    pub fn find_by_position(&self, position: &Position) -> Option<Rc<RefCell<Cell>>> {
        self.find(|c| c.borrow().pos() == *position)
    }
    pub fn get(&self, index: usize) -> Option<Rc<RefCell<Cell>>> {
        self.cells.get(index).map(|rc| rc.clone())
    }

    pub fn positions(&self) -> Vec<Position> {
        self.cells
            .iter()
            .map(|c| c.borrow().pos())
            .collect::<Vec<Position>>()
    }

    pub fn on_answered(&self, answer: u8) {
        for cell in self.cells.iter() {
            cell.borrow_mut().remove_answer_candidate(answer);
        }
    }
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
                assert_eq!(create_cells(&SETTING).cells[0].borrow().pos, Position(0, 0));
            }
            #[test]
            fn second_cell_position_is_0_1() {
                assert_eq!(create_cells(&SETTING).cells[1].borrow().pos, Position(0, 1));
            }
            #[test]
            fn last_cell_position_is_5_5() {
                assert_eq!(
                    create_cells(&SETTING).cells[35].borrow().pos,
                    Position(5, 5)
                );
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
                assert_eq!(
                    create_cells(&SETTING).cells[80].borrow().pos,
                    Position(8, 8)
                );
            }
        }
    }
    mod cells {
        use super::*;
        #[test]
        fn test_filter_by_x() {
            let vertical_line = create_cells(&SETTING).filter_by_x(2);
            let vertical_line_pos = vertical_line.positions();
            assert_eq!(
                vertical_line_pos,
                [
                    Position(2, 0),
                    Position(2, 1),
                    Position(2, 2),
                    Position(2, 3),
                    Position(2, 4),
                    Position(2, 5),
                ]
            )
        }
        #[test]
        fn test_filter() {
            let horizontal_line = create_cells(&SETTING).filter(|c| c.borrow().pos.y() == 2);
            assert_eq!(
                horizontal_line.positions(),
                [
                    Position(0, 2),
                    Position(1, 2),
                    Position(2, 2),
                    Position(3, 2),
                    Position(4, 2),
                    Position(5, 2),
                ]
            )
        }
        #[test]
        fn test_filter_by_y() {
            let horizontal_line = create_cells(&SETTING).filter_by_y(2);
            let horizontal_line_pos = horizontal_line.positions();
            assert_eq!(
                horizontal_line_pos,
                [
                    Position(0, 2),
                    Position(1, 2),
                    Position(2, 2),
                    Position(3, 2),
                    Position(4, 2),
                    Position(5, 2),
                ]
            )
        }
        #[test]
        fn test_find_by_position() {
            let pos = Position(0, 0);
            assert_eq!(
                create_cells(&SETTING)
                    .find_by_position(&pos)
                    .unwrap()
                    .as_ref()
                    .borrow()
                    .pos(),
                pos
            );
        }
        #[test]
        fn get_returns_none_when_outofbounds() {
            assert_eq!(create_cells(&SETTING).get(36), None);
        }
        #[test]
        fn get_returns_some_cell() {
            assert_eq!(
                create_cells(&SETTING).get(35).unwrap().borrow().pos(),
                Position(5, 5)
            );
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
        mod cell_clear_candidate_when_setted_answer {
            use super::*;
            #[test]
            fn when_try_fill_own_answer() {
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

            #[test]
            fn when_setted_answer() {
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
