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
            match game.check_status() {
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
        if cells.len() == 0 {
            return Some(game.clone());
        }
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

    fn set_answer(game: &mut NormalGame, pos: Position, answer: u8) {
        game.set_answer(pos, answer);
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
    use crate::normal_game::setting::BlockSize;
    use crate::normal_game::setting::GameSetting;
    fn setting() -> GameSetting {
        GameSetting::new(BlockSize {
            height: 1,
            width: 3,
        })
    }
    // const GAME: NormalGame = NormalGame::new(SETTING);
    mod fill_lonely {
        use super::*;
        /// Verify that the answer can be determined if a certain candidate exists in only one cell in a group.
        /// 1 つのグループにおいてとある候補が 1 つの cell にしか存在しない場合に答えを確定できることを確認する
        #[test]
        fn fill() {
            let mut game = NormalGame::new(setting());
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
        use crate::normal_game::setting::BlockSize;
        use crate::normal_game::setting::GameSetting;
        mod setting_1_2 {
            use super::*;
            fn setting() -> GameSetting {
                GameSetting::new(BlockSize {
                    height: 1,
                    width: 2,
                })
            }
            #[test]
            fn test() {
                let mut game = NormalGame::new(setting());
                game.load("1");
                let solver = Solver { game };
                let solved_game = solver.solve();
                assert_eq!(solved_game.unwrap().to_string(), "12|21");
            }
        }
        mod setting_3_3 {
            use super::*;

            fn setting() -> GameSetting {
                GameSetting::new_with_answer_candidate(
                    BlockSize {
                        height: 3,
                        width: 3,
                    },
                    // vec![9, 8, 7, 6, 5, 4, 3, 2, 1],
                    vec![2, 3, 4, 5, 6, 8, 7, 9, 1],
                )
            }
            #[test]
            fn intermediate1_16_9x9() {
                let mut game = NormalGame::new(setting());
                game.load(" 7     6 |6   1   3|  54 87  |  8   4  | 1  3  5 |  9   1  |  35 12  |7   2   8| 5     9 ");
                let solver = Solver::new(&game);
                let game = solver.solve();
                // assert_eq!(solver.game.status(), GameState::Complete);
                assert_eq!(game.unwrap().to_string(), "174392865|682715943|935468721|528176439|417839652|369254187|893541276|746923518|251687394".to_string());
            }
            #[test]
            fn intermediate1_96_9x9() {
                let mut game = NormalGame::new(setting());
                game.load(
                    "4       1| 5   1 4 |  8 476  | 79|  3 7 2|      59|  681 9| 4 9   7|2       5",
                );
                let solver = Solver::new(&game);
                let game = solver.solve().unwrap();
                assert_eq!(game.to_string(), "462593781|957681342|318247659|679152438|583479216|124368597|736815924|845926173|291734865".to_string());
            }
            #[test]
            fn intermediate1_98_9x9() {
                let mut game = NormalGame::new(setting());
                game.load(
                    "7  4 1  9| 62    3|   2   1|5     3 8||9 4     2| 7   9| 5    84|3  8 7  6",
                );
                let solver = Solver::new(&game);
                let game = solver.solve();
                assert_eq!(game.unwrap().to_string(), "735461289|162985734|498273615|527194368|683752491|914638572|876549123|259316847|341827956".to_string());
            }
            #[test]
            // #[ignore]
            fn advanced0_5_9x9() {
                let mut game = NormalGame::new(setting());
                game.load("  4   7 3|8  9 2| 3| 891|5       8|     926|       2|   8 4  5|6 5   1");
                let solver = Solver::new(&game);
                let solved_game = solver.solve();
                assert_eq!(solved_game.unwrap().to_string(), "124658793|857932416|936471852|289146537|561327948|743589261|418765329|392814675|675293184".to_string());
            }
            #[test]
            // #[ignore]
            fn advanced0_6_9x9() {
                let mut game = NormalGame::new(setting());
                game.load("  4  37|9  82   6|  7   9|6      8| 1  3  2| 9      5|  9   1|1   42  3|  85  2");
                let solver = Solver::new(&game);
                let solved_game = solver.solve();
                assert_eq!(solved_game.unwrap().to_string(), "584693712|931827546|267154938|642975381|715438629|893261475|429386157|156742893|378519264".to_string());
            }
            #[test]
            // #[ignore]
            fn advanced0_7_9x9() {
                let mut game = NormalGame::new(setting());
                game.load(" 4   6 3|7   4   1|   8  9|  1     8| 2  3  6|3     1|  7  4|1   8   7| 6 3   2");
                let solver = Solver::new(&game);
                let solved_game = solver.solve();
                assert_eq!(solved_game.unwrap().to_string(), "248196735|796543281|513872946|671425398|829731564|354968172|987254613|132689457|465317829".to_string());
            }
            #[test]
            // #[ignore]
            fn advanced0_8_9x9() {
                let mut game = NormalGame::new(setting());
                game.load("5 2 9 1|   1   8|3    6  2| 4    7|6       1|  5    9|9  7    4| 6   3|  7 2 5 3");
                let solver = Solver::new(&game);
                let solved_game = solver.solve();
                assert_eq!(solved_game.unwrap().to_string(), "582397146|496152387|371486952|148639725|629875431|735241698|953768214|264513879|817924563".to_string());
            }
        }

        mod setting_3_4 {
            use super::*;
            #[test]
            // #[ignore]
            fn advanced0_33_12x12() {
                let mut game = NormalGame::new(GameSetting::new(BlockSize {
                    height: 3,
                    width: 4,
                }));
                game.load(" , , ,6, , , , ,8| , , , ,12,10,5,11| , ,10,4, ,9,7, ,1,11|10, ,3, , , , , , ,7, ,12| ,5, , , ,12,10, , , ,9| ,7,8, ,9, , ,2, ,5,10| ,1,7, ,8, , ,6, ,3,4,| ,10, , , ,5,1, , , ,2|11, ,4, , , , , , ,12, ,7| , ,9,10, ,8,4, ,3,6,| , , , ,2,1,6,9,| , , ,11, , , , ,9");
                let solver = Solver::new(&game);
                let solved_game = solver.solve();
                assert_eq!(solved_game.unwrap().to_string_with_comma(), "7,12,11,6,1,3,2,4,8,9,5,10|8,9,2,1,12,10,5,11,7,4,3,6|5,3,10,4,6,9,7,8,1,11,12,2|10,11,3,9,5,4,8,1,2,7,6,12|4,5,6,2,7,12,10,3,11,1,9,8|1,7,8,12,9,6,11,2,4,5,10,3|2,1,7,5,8,11,12,6,10,3,4,9|9,10,12,3,4,5,1,7,6,8,2,11|11,6,4,8,3,2,9,10,5,12,1,7|12,2,9,10,11,8,4,5,3,6,7,1|3,8,5,7,2,1,6,9,12,10,11,4|6,4,1,11,10,7,3,12,9,2,8,5".to_string());
            }
        }

        mod setting_4_4 {
            use super::*;
            #[test]
            // #[ignore]
            fn advanced0_33_16x16() {
                let mut game = NormalGame::new(GameSetting::new(BlockSize {
                    height: 4,
                    width: 4,
                }));
                game.load("7, , ,11,4, , ,10,2, , ,1,12, , ,5| , ,6, , ,3, , , , ,16, , ,10| ,10,14, , ,13,7, , ,5,6, , ,3,1,|2, , , , , , ,15,13, , , , , , ,14|8, , , , , , ,1,12, , , , , , ,11| ,13,12, , ,8,15, , ,9,5, , ,14,16| , ,10, , ,2, , , , ,11, , ,7,|15, , ,9,5, , ,12,4, , ,2,6, , ,8|10, , ,5,1, , ,16,15, , ,9,8, , ,4| , ,8, , ,10, , , , ,1, , ,6| ,7,11, , ,4,8, , ,14,12, , ,5,3|4, , , , , , ,5,7, , , , , , ,10|9, , , , , , ,14,10, , , , , , ,1| ,6,2, , ,7,5, , ,11,9, , ,4,8| , ,13, , ,15, , , , ,3, , ,16|16, , ,3,10, , ,4,6, , ,14,13, , ,12");
                let solver = Solver::new(&game);
                let solved_game = solver.solve();
                assert_eq!(solved_game.unwrap().to_string_with_comma(), "7,15,16,11,4,9,14,10,2,3,8,1,12,13,6,5|5,4,6,13,8,3,1,11,14,15,16,12,2,10,9,7|12,10,14,8,16,13,7,2,9,5,6,11,4,3,1,15|2,3,9,1,6,5,12,15,13,10,4,7,16,8,11,14|8,2,5,6,9,16,10,1,12,7,14,13,3,15,4,11|1,13,12,4,11,8,15,7,3,9,5,6,10,14,16,2|3,16,10,14,13,2,4,6,8,1,11,15,5,7,12,9|15,11,7,9,5,14,3,12,4,16,10,2,6,1,13,8|10,14,3,5,1,11,13,16,15,6,2,9,8,12,7,4|13,12,8,15,7,10,2,3,11,4,1,5,9,6,14,16|6,7,11,2,15,4,8,9,16,14,12,10,1,5,3,13|4,9,1,16,14,12,6,5,7,8,13,3,11,2,15,10|9,5,4,12,3,6,16,14,10,13,15,8,7,11,2,1|14,6,2,10,12,7,5,13,1,11,9,16,15,4,8,3|11,1,13,7,2,15,9,8,5,12,3,4,14,16,10,6|16,8,15,3,10,1,11,4,6,2,7,14,13,9,5,12".to_string());
            }
        }
    }
    mod it_can_specify_arbitrary_answer_candidate {
        // It is possible to specify an arbitrary answer_candidate at the time of game generation.
        use super::*;

        #[test]
        fn test_1234() {
            let game = NormalGame::new(GameSetting::new_with_answer_candidate(
                BlockSize {
                    height: 2,
                    width: 2,
                },
                vec![1, 2, 3, 4],
            ));
            let solved_game = Solver::new(&game).solve();
            assert_eq!(solved_game.unwrap().to_string(), "1234|3412|2143|4321")
        }
        #[test]
        fn test_4321() {
            let game = NormalGame::new(GameSetting::new_with_answer_candidate(
                BlockSize {
                    height: 2,
                    width: 2,
                },
                vec![4, 3, 2, 1],
            ));
            let solved_game = Solver::new(&game).solve();
            assert_eq!(solved_game.unwrap().to_string(), "4321|2143|1432|3214")
        }
    }
}
