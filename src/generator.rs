use crate::normal_game::cell::Position;
use crate::normal_game::setting::BlockSize;
use crate::normal_game::setting::GameSetting;
use crate::normal_game::NormalGame;
use rand::prelude::*;

impl NormalGame {
    pub fn generate(block_size: BlockSize) -> NormalGame {
        let solved_game = NormalGame::generate_random_solved_game(block_size);
        let game = NormalGame::to_issue(solved_game);
        game
    }

    fn to_issue(solved_game: NormalGame) -> NormalGame {
        let mut game = solved_game.clone();
        let mut rng = thread_rng();
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

            let maybe_solved = tmp_game.simple_solve();
            if maybe_solved.is_some() {
                game.remove_answer(pos);
                count = 0;
            } else {
                // If it cannot be solved by a simple_solve, try another number.
                // If it is still not solved after few times,
                // the issue is considered complete before removing the answer.
                count += 1;
                if count > 3 {
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
        let solved_game = game.solve().unwrap();
        solved_game.shuffle();
        solved_game
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::normal_game::setting::BlockSize;
    #[test]
    #[ignore]
    fn generate_6x6() {
        let game = NormalGame::generate(BlockSize {
            height: 3,
            width: 2,
        });
        println!("{}", game.to_string_with_newline());
        assert!(false);
    }
    mod to_issue_9x9 {
        use super::*;
        #[test]
        #[ignore]
        fn test() {
            let mut game = NormalGame::new(GameSetting::new(BlockSize {
                height: 3,
                width: 3,
            }));
            game.load("174392865|682715943|935468721|528176439|417839652|369254187|893541276|746923518|251687394");
            let game = NormalGame::to_issue(game);
            println!("{}", game.to_string_with_newline());
            assert!(false);
        }
    }
    mod to_issue_16x16 {
        use super::*;
        #[test]
        #[ignore]
        fn test() {
            let mut game = NormalGame::new(GameSetting::new(BlockSize {
                height: 4,
                width: 4,
            }));
            game.load("1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16|5,6,7,8,9,10,11,12,13,14,15,16,1,2,3,4|9,10,11,12,13,14,15,16,1,2,3,4,5,6,7,8|13,14,15,16,1,2,3,4,5,6,7,8,9,10,11,12|2,3,4,1,6,7,8,5,10,11,12,9,14,15,16,13|6,7,8,5,10,11,12,9,14,15,16,13,2,3,4,1|10,11,12,9,14,15,16,13,2,3,4,1,6,7,8,5|14,15,16,13,2,3,4,1,6,7,8,5,10,11,12,9|3,4,1,2,7,8,5,6,11,12,9,10,15,16,13,14|7,8,5,6,11,12,9,10,15,16,13,14,3,4,1,2|11,12,9,10,15,16,13,14,3,4,1,2,7,8,5,6|15,16,13,14,3,4,1,2,7,8,5,6,11,12,9,10|4,1,2,3,8,5,6,7,12,9,10,11,16,13,14,15|8,5,6,7,12,9,10,11,16,13,14,15,4,1,2,3|12,9,10,11,16,13,14,15,4,1,2,3,8,5,6,7|16,13,14,15,4,1,2,3,8,5,6,7,12,9,10,11");
            let game = NormalGame::to_issue(game);
            println!("{}", game.to_string_with_newline());
            assert!(false);
        }
    }
}
