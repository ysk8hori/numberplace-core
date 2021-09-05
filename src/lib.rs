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
            // #[ignore]
            fn intermediate1_16_9x9() {
                let mut game = normal_game::NormalGame::new(SETTING);
                game.load(" 7     6 |6   1   3|  54 87  |  8   4  | 1  3  5 |  9   1  |  35 12  |7   2   8| 5     9 ");
                let mut solver = solver::Solver::new(game);
                solver.solving();
                // assert_eq!(solver.game.status(), GameState::Complete);
                assert_eq!(solver.game().to_string(), "174392865|682715943|935468721|528176439|417839652|369254187|893541276|746923518|251687394".to_string());
            }
            #[test]
            // #[ignore]
            fn intermediate1_96_9x9() {
                let mut game = normal_game::NormalGame::new(SETTING);
                game.load(
                    "4       1| 5   1 4 |  8 476  | 79|  3 7 2|      59|  681 9| 4 9   7|2       5",
                );
                let mut solver = solver::Solver::new(game);
                solver.solving();
                assert_eq!(solver.game().to_string(), "462593781|957681342|318247659|679152438|583479216|124368597|736815924|845926173|291734865".to_string());
            }
        }
    }
}
