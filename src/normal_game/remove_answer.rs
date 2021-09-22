use crate::normal_game::cell::Position;
use crate::normal_game::group::Group;
use crate::normal_game::NormalGame;
use core::cell::RefCell;
use slice_group_by::GroupBy;
use std::rc::Rc;

impl NormalGame {
    pub fn remove_answer(&mut self, pos: Position) -> Option<u8> {
        let cell = self.find_cell(pos).unwrap();
        let removed_answer = cell.borrow_mut().remove_answer();
        if removed_answer.is_none() {
            return removed_answer;
        }
        let removed_answer = removed_answer.unwrap();
        let target_groups = self.find_groups(pos);
        target_groups
            .iter()
            .for_each(|g| g.borrow_mut().restore_answer_candidate(removed_answer));
        let positions_into_target_groups: Vec<Position> = target_groups
            .iter()
            .map(|g| {
                let poslist: Vec<Position> = g
                    .borrow()
                    .cells()
                    .iter()
                    .map(|c| c.borrow().pos())
                    .collect();
                poslist
            })
            .flatten()
            .collect();
        for pos in positions_into_target_groups {
            let groups = self.find_groups(pos);
            self.find_cell(pos)
                .unwrap()
                .borrow_mut()
                .restore_answer_candidate(&self.find_answer_candidate_that_all_groups_hold(groups));
        }
        self.answered_count -= 1;
        Some(removed_answer)
    }

    fn find_groups(&self, pos: Position) -> Vec<&Rc<RefCell<Group>>> {
        self.groups()
            .iter()
            .filter(|g| g.borrow().cells().iter().any(|c| c.borrow().pos() == pos))
            .collect()
    }

    fn find_answer_candidate_that_all_groups_hold(
        &self,
        target_groups: Vec<&Rc<RefCell<Group>>>,
    ) -> Vec<u8> {
        let mut target_groups_answer_candidate: Vec<u8> = target_groups
            .iter()
            .map(|g| g.borrow().answer_candidate())
            .flatten()
            .collect();
        target_groups_answer_candidate.sort();
        let mut answer_candidate: Vec<u8> = Vec::new();
        for array in target_groups_answer_candidate.linear_group_by(|a, b| a == b) {
            if array.len() == target_groups.len() {
                answer_candidate.push(array[0]);
            }
        }
        answer_candidate
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::normal_game::setting::BlockSize;
    use crate::normal_game::setting::GameSetting;
    use crate::normal_game::NormalGame;
    fn setting() -> GameSetting {
        GameSetting::new(BlockSize {
            height: 2,
            width: 2,
        })
    }
    fn pos(x: u8, y: u8) -> Position {
        Position::new(x, y)
    }
    mod find_groups {
        use super::*;
        #[test]
        fn it_find_the_groups_that_contains_the_specified_positions_cell() {
            let game = NormalGame::new(setting());
            let groups = game.find_groups(Position::new(0, 0));
            fn to_positions(group: &Rc<RefCell<Group>>) -> Vec<Position> {
                group
                    .borrow()
                    .cells()
                    .iter()
                    .map(|c| c.borrow().pos())
                    .collect()
            }
            assert_eq!(groups.len(), 3);
            assert_eq!(
                to_positions(groups[0]),
                [pos(0, 0), pos(1, 0), pos(2, 0), pos(3, 0)]
            );
            assert_eq!(
                to_positions(groups[1]),
                [pos(0, 0), pos(0, 1), pos(0, 2), pos(0, 3)]
            );
            assert_eq!(
                to_positions(groups[2]),
                [pos(0, 0), pos(1, 0), pos(0, 1), pos(1, 1)]
            );
        }
    }
    mod find_answer_candidate_that_all_groups_hold {
        use super::*;
        #[test]
        fn it_find_answer_candidate_that_all_groups_hold() {
            let game = NormalGame::new(setting());
            let groups = game.find_groups(pos(0, 0));
            groups[0].borrow_mut().remove_answer_candidate(1);
            groups[1].borrow_mut().remove_answer_candidate(1);
            groups[2].borrow_mut().remove_answer_candidate(1);
            groups[2].borrow_mut().remove_answer_candidate(3);
            let answer_candidate = game.find_answer_candidate_that_all_groups_hold(groups);
            assert_eq!(answer_candidate, [2, 4]);
        }
    }
    mod remove_answer {
        use super::*;
        fn setting() -> GameSetting {
            GameSetting::new(BlockSize {
                height: 3,
                width: 3,
            })
        }
        fn to_answer_candidate_vec(game: &NormalGame) -> Vec<Vec<u8>> {
            let groups = game.find_groups(pos(1, 0));
            vec![
                groups[0].borrow().answer_candidate(),
                groups[1].borrow().answer_candidate(),
                groups[2].borrow().answer_candidate(),
            ]
        }
        #[test]
        fn test() {
            let mut game = NormalGame::new(setting());
            // target cell 👉 "5⭐2 9 1|   1   8|3    6  2| 4    7|6       1|  5    9|9  7    4| 6   3|  7 2 5 3"
            game.load(
                "5 2 9 1|   1   8|3    6  2| 4    7|6       1|  5    9|9  7    4| 6   3|  7 2 5 3",
            );
            assert_eq!(
                to_answer_candidate_vec(&game),
                [
                    vec![3, 4, 6, 7, 8],
                    vec![1, 2, 3, 5, 7, 8, 9],
                    vec![1, 4, 6, 7, 8, 9]
                ]
            );
            game.set_answer(pos(1, 0), 8);
            assert_eq!(
                to_answer_candidate_vec(&game),
                [
                    vec![3, 4, 6, 7],
                    vec![1, 2, 3, 5, 7, 9],
                    vec![1, 4, 6, 7, 9]
                ]
            );
            let removed_answer = game.remove_answer(pos(1, 0));
            assert_eq!(
                to_answer_candidate_vec(&game),
                [
                    vec![3, 4, 6, 7, 8],
                    vec![1, 2, 3, 5, 7, 9, 8],
                    vec![1, 4, 6, 7, 9, 8]
                ]
            );
            assert_eq!(removed_answer, Some(8));
            assert!(game
                .find_cell(pos(1, 0))
                .unwrap()
                .borrow()
                .answer_candidate()
                .any(|c| *c == 8));
        }
        #[test]
        fn test2() {
            let mut game = NormalGame::new(setting());
            // target cell 👉 "5⭐2 9 1|   1   8|3    6  2| 4    7|6       1|  5    9|9  7    4| 6   3|  7 2 5 3"
            game.load(
                "5 2 9 1|   1   8|3    6  2| 4    7|6       1|  5    9|9  7    4| 6   3|  7 2 5 3",
            );
            assert_eq!(
                to_answer_candidate_vec(&game),
                [
                    vec![3, 4, 6, 7, 8],
                    vec![1, 2, 3, 5, 7, 8, 9],
                    vec![1, 4, 6, 7, 8, 9]
                ]
            );
            game.set_answer(pos(1, 0), 9);
            assert_eq!(
                to_answer_candidate_vec(&game),
                [
                    vec![3, 4, 6, 7, 8],
                    vec![1, 2, 3, 5, 7, 8],
                    vec![1, 4, 6, 7, 8]
                ]
            );
            let removed_answer = game.remove_answer(pos(1, 0));
            assert_eq!(removed_answer, Some(9));
            assert_eq!(
                to_answer_candidate_vec(&game),
                [
                    vec![3, 4, 6, 7, 8],
                    vec![1, 2, 3, 5, 7, 8, 9],
                    vec![1, 4, 6, 7, 8, 9]
                ]
            );
            assert_eq!(
                game.find_cell(pos(1, 0))
                    .unwrap()
                    .borrow()
                    .answer_candidate()
                    .map(|a| *a)
                    .collect::<Vec<u8>>(),
                vec![7, 8]
            );
        }
    }
}
