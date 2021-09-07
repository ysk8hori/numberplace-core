pub mod normal_game;
pub mod solver;

#[cfg(test)]
mod tests {
    use super::*;
    mod test_solver {
        use super::*;
        mod setting_3_3 {
            use super::*;

            const SETTING: normal_game::setting::GameSetting = normal_game::setting::GameSetting {
                block_height: 3,
                block_width: 3,
            };
            #[test]
            fn intermediate1_16_9x9() {
                let mut game = normal_game::NormalGame::new(SETTING);
                game.load(" 7     6 |6   1   3|  54 87  |  8   4  | 1  3  5 |  9   1  |  35 12  |7   2   8| 5     9 ");
                let mut solver = solver::Solver::new(game);
                solver.solving();
                // assert_eq!(solver.game.status(), GameState::Complete);
                assert_eq!(solver.game().to_string(), "174392865|682715943|935468721|528176439|417839652|369254187|893541276|746923518|251687394".to_string());
            }
            #[test]
            fn intermediate1_96_9x9() {
                let mut game = normal_game::NormalGame::new(SETTING);
                game.load(
                    "4       1| 5   1 4 |  8 476  | 79|  3 7 2|      59|  681 9| 4 9   7|2       5",
                );
                let mut solver = solver::Solver::new(game);
                solver.solving();
                assert_eq!(solver.game().to_string(), "462593781|957681342|318247659|679152438|583479216|124368597|736815924|845926173|291734865".to_string());
            }
            #[test]
            fn intermediate1_98_9x9() {
                let mut game = normal_game::NormalGame::new(SETTING);
                game.load(
                    "7  4 1  9| 62    3|   2   1|5     3 8||9 4     2| 7   9| 5    84|3  8 7  6",
                );
                let mut solver = solver::Solver::new(game);
                solver.solving();
                assert_eq!(solver.game().to_string(), "735461289|162985734|498273615|527194368|683752491|914638572|876549123|259316847|341827956".to_string());
            }
            #[test]
            // #[ignore]
            fn advanced0_5_9x9() {
                let mut game = normal_game::NormalGame::new(SETTING);
                game.load("  4   7 3|8  9 2| 3| 891|5       8|     926|       2|   8 4  5|6 5   1");
                let mut solver = solver::Solver::new(game);
                let solved_game = solver.solving();
                assert_eq!(solved_game.unwrap().to_string(), "124658793|857932416|936471852|289146537|561327948|743589261|418765329|392814675|675293184".to_string());
            }
        }
    }
}
