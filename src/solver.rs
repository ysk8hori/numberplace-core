// use crate::normal_game::group::Group;
use crate::normal_game::cell::Position;
use crate::normal_game::NormalGame;

pub struct Solver {
    game: NormalGame,
}

impl Solver {
    pub fn solving(&self) {}

    pub fn set_answer(&self, pos: Position, answer: u8) {
        self.game.set_answer(pos, answer);
    }

    /// If there is only one possible answer, confirm it.
    pub fn fill_lonely(&self) {
        let mut fillable_pos_answer: Vec<(Position, u8)> = vec![];
        for group in self.game.groups().iter() {
            for (pos, answer) in group.borrow().get_lonely().iter() {
                fillable_pos_answer.push((*pos, *answer))
            }
        }
        for fillable in fillable_pos_answer.iter() {
            let (pos, answer) = fillable;
            self.game.set_answer(*pos, *answer);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::normal_game::setting::GameSetting;
    const SETTING: GameSetting = GameSetting {
        block_height: 1,
        block_width: 3,
    };
    // const GAME: NormalGame = NormalGame::new(SETTING);
    mod fill_lonely {
        use super::*;
        /// Verify that the answer can be determined if a certain candidate exists in only one cell in a group.
        /// 1 つのグループにおいてとある候補が 1 つの cell にしか存在しない場合に答えを確定できることを確認する
        #[test]
        fn test2() {
            let game = NormalGame::new(SETTING);
            let solver = Solver { game };
            // [1][ ][3]
            // [ ][ ][ ]
            // [ ][ ][ ] の状態にする👇
            solver.set_answer(Position::new(0, 0), 1);
            solver.set_answer(Position::new(2, 0), 3);
            // [1][🌟][3]
            // [ ][ ][ ]
            // [ ][ ][ ] 🌟の部分が確定する
            solver.fill_lonely();

            fn get_answer(solver: &Solver, x: u8, y: u8) -> Option<u8> {
                solver
                    .game
                    .find_cell(Position::new(x, y))
                    .unwrap()
                    .borrow()
                    .answer()
            }

            assert_eq!(get_answer(&solver, 0, 0), Some(1));
            assert_eq!(get_answer(&solver, 1, 0), Some(2));
            assert_eq!(get_answer(&solver, 2, 0), Some(3));
        }
    }
}
