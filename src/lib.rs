use crate::normal_game::setting::BlockSize;
use crate::normal_game::setting::GameSetting;
use crate::normal_game::NormalGame;

mod generator;
pub mod normal_game;
mod pattern;

/// Generate a number-place problem.
/// The first element of the returned Tuple is the question, and the next element is the answer.
///
/// ナンバープレースの問題を生成する。
/// 返却される Tuple の最初の要素は問題で、次の要素は答えである。
///
/// Issues of the following block-sizes can be generated quickly. For other sizes, it takes time to generate them.
/// 以下のブロックサイズの問題はある程度素早く生成することが可能である。それ以外のサイズについては、生成に時間がかかる。
///
/// 1x3, 2x2, 2x3, 3x2, 4x2, 2x4, 3x3, 2x5, 5x2, 4x3, 3x4, 4x4, 4x5, 5x4, 5x5
///
/// Even though it is quick, it will take a few seconds for 3x4 and 4x3 sizes,
/// about 30 seconds for 4x5 and 5x4 sizes, and about a minute for 5x5 sizes.
/// 素早くと言っても、3x4,4x3のサイズで数秒、4x5,5x4のサイズで約30秒、5x5のサイズで1分程度の時間はかかると思われる。
///
pub fn generate_numberplace(block_size: BlockSize) -> (NormalGame, NormalGame) {
    NormalGame::generate(block_size)
}

/// Solve number-place issues.
/// The second argument, issue, can be a string such as the following.  
///
/// ナンバープレースの問題を解く。  
///  第二引数の issue は例えば以下のような文字列を指定可能である。  
///
/// - `"5 2 9 1|   1   8|3    6  2| 4    7|6       1|  5    9|9  7    4| 6   3|  7 2 5 3"`
/// - `"7, , ,11,4, , ,10,2, , ,1,12, , ,5| , ,6, , ,3, , , , ,16, , ,10| ,10,14, , ,13,7, , ,5,6, , ,3,1,|2, , , , , , ,15,13, , , , , , ,14|8, , , , , , ,1,12, , , , , , ,11| ,13,12, , ,8,15, , ,9,5, , ,14,16| , ,10, , ,2, , , , ,11, , ,7,|15, , ,9,5, , ,12,4, , ,2,6, , ,8|10, , ,5,1, , ,16,15, , ,9,8, , ,4| , ,8, , ,10, , , , ,1, , ,6| ,7,11, , ,4,8, , ,14,12, , ,5,3|4, , , , , , ,5,7, , , , , , ,10|9, , , , , , ,14,10, , , , , , ,1| ,6,2, , ,7,5, , ,11,9, , ,4,8| , ,13, , ,15, , , , ,3, , ,16|16, , ,3,10, , ,4,6, , ,14,13, , ,12"`
///
pub fn solve_numberplace(block_size: BlockSize, issue: &str) -> Option<NormalGame> {
    let mut game = NormalGame::new(GameSetting::new(block_size));
    game.load(issue);
    game.solve()
}

#[cfg(test)]
mod tests {
    use super::*;
    mod generate_numberplace {
        use super::*;
        #[test]
        #[ignore]
        fn test() {
            let (issue, solved) = generate_numberplace(BlockSize {
                height: 3,
                width: 3,
            });
            println!("{}", issue.to_string_with_comma());
            println!("{}", solved.to_string_with_comma());
            assert!(false);
        }
    }
    mod solve {
        use super::*;
        #[ignore]
        #[test]
        fn test() {
            let solved = solve_numberplace(
                BlockSize {
                    height: 3,
                    width: 3,
                },
                "5 2 9 1|   1   8|3    6  2| 4    7|6       1|  5    9|9  7    4| 6   3|  7 2 5 3",
            );
            println!("{}", solved.unwrap().to_string_with_comma());
            assert!(false);
        }
    }
}
