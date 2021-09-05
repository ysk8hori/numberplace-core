// use crate::normal_game::group::Group;
use crate::normal_game::cell::Position;
use crate::normal_game::GameState;
use crate::normal_game::NormalGame;

pub struct Solver {
    game: NormalGame,
}

impl Solver {
    pub fn new(game: NormalGame) -> Solver {
        Solver { game }
    }
    pub fn game(&self) -> &NormalGame {
        &self.game
    }
    pub fn solving(&mut self) {
        let mut count = 0;
        loop {
            let before = self.game.answered_counter();
            for cell in self.game.cells().iter() {
                let answer = cell.borrow_mut().try_fill_own_answer();
                if let Some(answer) = answer {
                    self.game
                        .groups()
                        .iter()
                        .filter(|g| {
                            g.borrow()
                                .cells()
                                .iter()
                                .any(|c| c.borrow().pos() == cell.borrow().pos())
                        })
                        .for_each(|g| g.borrow_mut().remove_answer_candidate(answer));
                }
            }
            self.fill_lonely();
            if self.game.status() == GameState::Complete {
                return;
            }
            if before == self.game.answered_counter() {
                // if count == 0 {
                return;
                // }
                // count += 1;
            }
            // else {
            //     count = 0;
            // }
        }
    }

    pub fn set_answer(&mut self, pos: Position, answer: u8) {
        self.game.set_answer(pos, answer);
    }

    /// If there is only one possible answer, confirm it.
    pub fn fill_lonely(&mut self) {
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
            let mut solver = Solver { game };
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
    mod solving {
        use super::*;
        mod setting_1_2 {
            use super::*;
            const SETTING: GameSetting = GameSetting {
                block_height: 1,
                block_width: 2,
            };
            #[test]
            fn test() {
                let mut game = NormalGame::new(SETTING);
                game.load("1");
                let mut solver = Solver { game };
                solver.solving();
                assert_eq!(solver.game.to_string(), "12|21");
            }
        }
        mod setting_3_3 {
            use super::*;
            const SETTING: GameSetting = GameSetting {
                block_height: 3,
                block_width: 3,
            };
            #[test]
            fn intermediate1_96_9x9() {
                let mut game = NormalGame::new(SETTING);
                game.load(
                    "4       1| 5   1 4 |  8 476  | 79|  3 7 2|      59|  681 9| 4 9   7|2       5",
                );
                let mut solver = Solver { game };
                solver.solving();
                assert_eq!(solver.game.to_string(), "462593781|957681342|318247659|679152438|583479216|124368597|736815924|845926173|291734865".to_string());
            }
        }
    }
}
