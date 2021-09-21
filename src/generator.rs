use crate::normal_game::cell::Cell;
use crate::normal_game::cell::Position;
use crate::normal_game::setting::BlockSize;
use crate::normal_game::setting::GameSetting;
// use crate::solver::Solver;
// use crate::normal_game::shuffle::shuffle;
use crate::normal_game::NormalGame;
use core::cell::RefCell;
use rand::prelude::*;
use std::rc::Rc;

impl NormalGame {
    pub fn generate(block_size: BlockSize) -> NormalGame {
        let solved_game = NormalGame::generate_random_solved_game(block_size);
        let game = NormalGame::to_issue(solved_game);
        game
    }

    fn to_issue(solved_game: NormalGame) -> NormalGame {
        let mut game = solved_game.clone();
        // let mut poslist: Vec<Position> = game.cells().iter().map(|c| c.borrow().pos()).collect();
        let mut rng = thread_rng();
        // while poslist.len() > game.cells().len() / 2 {
        //     let index = rng.gen_range(0..poslist.len());
        //     let pos = poslist.remove(index);
        //     game.remove_answer(pos);
        // }
        let mut count = 0;
        loop {
            let mut poslist: Vec<Position> = game
                .groups()
                .iter()
                .reduce(|a, b| {
                    if a.borrow().answer_count() < b.borrow().answer_count() {
                        b
                    } else {
                        a
                    }
                })
                .unwrap()
                .borrow()
                .cells()
                .iter()
                .filter(|c| c.borrow().answer().is_some())
                .map(|c| c.borrow().pos())
                .collect();
            let index = rng.gen_range(0..poslist.len());
            let pos = poslist.remove(index);

            let mut tmp_game = game.clone();
            tmp_game.remove_answer(pos);

            let solved = tmp_game.solve().unwrap();
            let reverse = tmp_game.reverse().solve().unwrap().reverse();
            let reverse_y = tmp_game.reverse_y().solve().unwrap().reverse_y();
            let reverse_x = tmp_game.reverse_x().solve().unwrap().reverse_x();
            if solved_game == solved
                && solved_game == reverse
                && solved_game == reverse_y
                && solved_game == reverse_x
            {
                game.remove_answer(pos);
                count = 0;
            } else {
                count += 1;
                if count > game.setting().side_size() {
                    break;
                }
            }
        }
        game
    }

    fn generate_random_solved_game(block_size: BlockSize) -> NormalGame {
        let setting = GameSetting::new(block_size);
        let mut answer_candidate = setting.answer_candidate();
        let mut random_sort_answer_candidate: Vec<u8> = Vec::new();
        let mut rng = thread_rng();
        while answer_candidate.len() != 0 {
            let index = rng.gen_range(0..answer_candidate.len());
            random_sort_answer_candidate.push(answer_candidate.remove(index));
        }

        let game = NormalGame::new(GameSetting::new_with_answer_candidate(
            block_size,
            random_sort_answer_candidate,
        ));
        let mut solved_game = game.solve().unwrap();
        solved_game.shuffle();
        solved_game
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::normal_game::setting::BlockSize;
    #[test]
    fn test() {
        let game = NormalGame::generate(BlockSize {
            height: 3,
            width: 2,
        });
        println!("{}", game.to_string_with_newline());
        assert!(true);
    }
    #[test]
    fn test_2_2() {
        let game = NormalGame::generate(BlockSize {
            height: 2,
            width: 2,
        });
        assert!(true);
    }
    mod to_issue {
        use super::*;
        #[test]
        fn test() {
            let mut game = NormalGame::new(GameSetting::new(BlockSize {
                height: 3,
                width: 3,
            }));
            game.load("174392865|682715943|935468721|528176439|417839652|369254187|893541276|746923518|251687394");
            let game = NormalGame::to_issue(game);
            println!("{}", game.to_string_with_newline());
            assert!(true);
        }
    }
}
