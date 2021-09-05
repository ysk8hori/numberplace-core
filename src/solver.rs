// use crate::normal_game::group::Group;
use crate::normal_game::{group::Group, setting::*, NormalGame};

pub struct Solver {
    game: NormalGame,
}

impl Solver {
    pub fn solving(&self) {}

    /// If there is only one possible answer, confirm it.
    pub fn fill_lonely(&self) {
        for group in self.game.groups().iter() {
            for (pos, answer) in group.borrow().get_lonely().iter() {
                self.game
                    .cells()
                    .iter()
                    .find(|c| c.borrow().pos() == *pos)
                    .unwrap()
                    .borrow_mut()
                    .set_answer(*answer);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const SETTING: GameSetting = GameSetting {
        block_height: 1,
        block_width: 3,
    };
    // const GAME: NormalGame = NormalGame::new(SETTING);
    mod fill_lonely {
        use super::*;
        #[test]
        /// 1 つのグループにおいてとある候補が 1 つの cell にしか存在しない場合に答えを確定できることを確認する
        fn test() {
            let game = NormalGame::new(SETTING);
            let solver = Solver { game };
            let group = solver.game.groups()[0].borrow();
            let target_answer = 2;
            let target_cell_index = 1;
            for (i, cell) in group.cells().iter().enumerate() {
                if i == target_cell_index {
                    continue;
                }
                cell.borrow_mut().remove_answer_candidate(target_answer);
            }
            solver.fill_lonely();
            assert_eq!(group.cells()[0].borrow().answer(), None);
            assert_eq!(
                group.cells()[target_cell_index].borrow().answer(),
                Some(target_answer)
            );
            assert_eq!(group.cells()[2].borrow().answer(), None);
        }
    }
}
