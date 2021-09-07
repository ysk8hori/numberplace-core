use crate::normal_game::cell::Cell;
use crate::normal_game::cell::Position;
use crate::normal_game::GameState;
use crate::normal_game::NormalGame;

pub struct Solver {
    game: NormalGame,
}

impl Solver {
    pub fn new(game: &NormalGame) -> Solver {
        Solver { game: game.clone() }
    }

    /// Solve the game.
    /// If the problem is solved, it returns a NormalGame instance with the solution filled in.
    /// If the problem is inconsistent, it returns None.
    /// Does not consider the case where there are multiple solutions.
    ///
    /// ゲームを解く。
    /// 問題を解けた場合は、解答を記入済みの NormalGame インスタンスを返す。
    /// 問題に矛盾がある場合は None を返す。
    /// 複数の解答が存在する場合は考慮していない。
    pub fn solve(&self) -> Option<NormalGame> {
        let mut game = self.game.clone();
        loop {
            let before_count = game.answered_count();
            Self::fill_lonely_in_cell(&mut game);
            Self::fill_lonely_in_group(&mut game);
            match game.status() {
                GameState::Complete => return Some(game),
                GameState::Failure => return None,
                _ => {}
            }
            if before_count == game.answered_count() {
                return self.assume_and_solve(&game);
            }
        }
    }

    /// If no cell or group of cells with a single answer_candidate is found,
    /// it finds the cell with the least answer_candidate among the unanswered cells, sets a temporary value, and solves.
    /// If it solves the problem, it returns NormalGame with the answer already filled in, otherwise it returns None.
    ///
    /// answer_candidate が 1つのセルやグループが見つからない場合に、未回答のセルのうち answer_candidate が最も少ないセルを見つけ、仮に値を設定して解く。
    /// 解けた場合は解答を記入済みの NormalGame を返却し、解けなかった場合は None を返却する。
    fn assume_and_solve(&self, game: &NormalGame) -> Option<NormalGame> {
        // Clone to avoid the effects of sorting.
        let mut cells = game.cells().clone();
        cells.sort_by(|a, b| {
            a.borrow()
                .answer_candidate_count()
                .partial_cmp(&b.borrow().answer_candidate_count())
                .unwrap()
        });
        let cells: Vec<&std::rc::Rc<std::cell::RefCell<Cell>>> = cells
            .iter()
            .filter(|c| c.borrow().answer_candidate_count() != 0)
            .collect();
        let solved_game = cells[0]
            .borrow()
            .answer_candidate()
            .map(|candidate| {
                let mut new_game = game.clone();
                new_game.set_answer(cells[0].borrow().pos(), *candidate);
                let solver = Solver::new(&new_game);
                solver.solve()
            })
            .find(|g| g.is_some());
        if let Some(Some(game)) = solved_game {
            return Some(game);
        } else {
            return None;
        }
    }

    /// 指定されたポジションのセルを保有するグループの解答候補から、指定した答えを削除する。
    fn remove_group_answer_candidate(game: &NormalGame, pos: Position, answer: u8) {
        game.groups()
            .iter()
            .filter(|g| g.borrow().cells().iter().any(|c| c.borrow().pos() == pos))
            .for_each(|g| g.borrow_mut().remove_answer_candidate(answer));
    }

    fn set_answer(game: &mut NormalGame, pos: Position, answer: u8) {
        game.set_answer(pos, answer);
        Self::remove_group_answer_candidate(game, pos, answer);
    }

    /// If there is only one possible answer in each cell, confirm it.
    fn fill_lonely_in_cell(game: &mut NormalGame) {
        let pos_and_answers: Vec<(Position, u8)> = game
            .cells()
            .iter()
            .map(|c| {
                let answer = c.borrow().get_lonely();
                if let Some(answer) = answer {
                    Some((c.borrow().pos(), answer))
                } else {
                    None
                }
            })
            .filter(|pos_and_answers| pos_and_answers.is_some())
            .map(|pos_and_answers| pos_and_answers.unwrap())
            .collect();
        pos_and_answers.iter().for_each(|item| {
            Self::set_answer(game, item.0, item.1);
        });
    }

    /// If there is only one possible answer in each group, confirm it.
    fn fill_lonely_in_group(game: &mut NormalGame) {
        let mut fillable_pos_answer: Vec<(Position, u8)> = vec![];
        for group in game.groups().iter() {
            for (pos, answer) in group.borrow().get_lonely().iter() {
                fillable_pos_answer.push((*pos, *answer))
            }
        }
        for fillable in fillable_pos_answer.iter() {
            let (pos, answer) = fillable;
            Self::set_answer(game, *pos, *answer);
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
        fn fill() {
            let mut game = NormalGame::new(SETTING);
            // [1][ ][3]
            // [ ][ ][ ]
            // [ ][ ][ ] の状態にする👇
            Solver::set_answer(&mut game, Position::new(0, 0), 1);
            Solver::set_answer(&mut game, Position::new(2, 0), 3);
            // [1][🌟][3]
            // [ ][ ][ ]
            // [ ][ ][ ] 🌟の部分が確定する
            Solver::fill_lonely_in_group(&mut game);

            fn get_answer(game: &NormalGame, x: u8, y: u8) -> Option<u8> {
                game.find_cell(Position::new(x, y))
                    .unwrap()
                    .borrow()
                    .answer()
            }

            assert_eq!(get_answer(&game, 0, 0), Some(1));
            assert_eq!(get_answer(&game, 1, 0), Some(2));
            assert_eq!(get_answer(&game, 2, 0), Some(3));
        }
    }
    mod solve {
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
                let solver = Solver { game };
                let solved_game = solver.solve();
                assert_eq!(solved_game.unwrap().to_string(), "12|21");
            }
        }
        mod setting_3_3 {
            use super::*;
            const SETTING: GameSetting = GameSetting {
                block_height: 3,
                block_width: 3,
            };
            #[test]
            // #[ignore]
            fn intermediate1_96_9x9() {
                let mut game = NormalGame::new(SETTING);
                game.load(
                    "4       1| 5   1 4 |  8 476  | 79|  3 7 2|      59|  681 9| 4 9   7|2       5",
                );
                let solver = Solver { game };
                let game = solver.solve().unwrap();
                assert_eq!(game.to_string(), "462593781|957681342|318247659|679152438|583479216|124368597|736815924|845926173|291734865".to_string());
            }
            #[test]
            fn returns_none_when_failed_to_solve() {
                let mut game = NormalGame::new(SETTING);
                game.load(
                    "45      1| 5   1 4 |  8 476  | 79|  3 7 2|      59|  681 9| 4 9   7|2       5",
                );
                let solver = Solver { game };
                let game = solver.solve();
                assert!(game.is_none());
            }
        }
    }
}
