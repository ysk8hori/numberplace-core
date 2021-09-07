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
                let solver = solver::Solver::new(&game);
                let game = solver.solving();
                // assert_eq!(solver.game.status(), GameState::Complete);
                assert_eq!(game.unwrap().to_string(), "174392865|682715943|935468721|528176439|417839652|369254187|893541276|746923518|251687394".to_string());
            }
            #[test]
            fn intermediate1_96_9x9() {
                let mut game = normal_game::NormalGame::new(SETTING);
                game.load(
                    "4       1| 5   1 4 |  8 476  | 79|  3 7 2|      59|  681 9| 4 9   7|2       5",
                );
                let solver = solver::Solver::new(&game);
                let game = solver.solving().unwrap();
                assert_eq!(game.to_string(), "462593781|957681342|318247659|679152438|583479216|124368597|736815924|845926173|291734865".to_string());
            }
            #[test]
            fn intermediate1_98_9x9() {
                let mut game = normal_game::NormalGame::new(SETTING);
                game.load(
                    "7  4 1  9| 62    3|   2   1|5     3 8||9 4     2| 7   9| 5    84|3  8 7  6",
                );
                let solver = solver::Solver::new(&game);
                let game = solver.solving();
                assert_eq!(game.unwrap().to_string(), "735461289|162985734|498273615|527194368|683752491|914638572|876549123|259316847|341827956".to_string());
            }
            #[test]
            // #[ignore]
            fn advanced0_5_9x9() {
                let mut game = normal_game::NormalGame::new(SETTING);
                game.load("  4   7 3|8  9 2| 3| 891|5       8|     926|       2|   8 4  5|6 5   1");
                let solver = solver::Solver::new(&game);
                let solved_game = solver.solving();
                assert_eq!(solved_game.unwrap().to_string(), "124658793|857932416|936471852|289146537|561327948|743589261|418765329|392814675|675293184".to_string());
            }
            #[test]
            // #[ignore]
            fn advanced0_6_9x9() {
                let mut game = normal_game::NormalGame::new(SETTING);
                game.load("  4  37|9  82   6|  7   9|6      8| 1  3  2| 9      5|  9   1|1   42  3|  85  2");
                let solver = solver::Solver::new(&game);
                let solved_game = solver.solving();
                assert_eq!(solved_game.unwrap().to_string(), "584693712|931827546|267154938|642975381|715438629|893261475|429386157|156742893|378519264".to_string());
            }
            #[test]
            // #[ignore]
            fn advanced0_7_9x9() {
                let mut game = normal_game::NormalGame::new(SETTING);
                game.load(" 4   6 3|7   4   1|   8  9|  1     8| 2  3  6|3     1|  7  4|1   8   7| 6 3   2");
                let solver = solver::Solver::new(&game);
                let solved_game = solver.solving();
                assert_eq!(solved_game.unwrap().to_string(), "248196735|796543281|513872946|671425398|829731564|354968172|987254613|132689457|465317829".to_string());
            }
            #[test]
            // #[ignore]
            fn advanced0_8_9x9() {
                let mut game = normal_game::NormalGame::new(SETTING);
                game.load("5 2 9 1|   1   8|3    6  2| 4    7|6       1|  5    9|9  7    4| 6   3|  7 2 5 3");
                let solver = solver::Solver::new(&game);
                let solved_game = solver.solving();
                assert_eq!(solved_game.unwrap().to_string(), "582397146|496152387|371486952|148639725|629875431|735241698|953768214|264513879|817924563".to_string());
            }
        }

        mod setting_3_4 {
            use super::*;
            const SETTING: normal_game::setting::GameSetting = normal_game::setting::GameSetting {
                block_height: 3,
                block_width: 4,
            };
            #[test]
            // #[ignore]
            fn advanced0_33_12x12() {
                let mut game = normal_game::NormalGame::new(SETTING);
                game.load(" , , ,6, , , , ,8| , , , ,12,10,5,11| , ,10,4, ,9,7, ,1,11|10, ,3, , , , , , ,7, ,12| ,5, , , ,12,10, , , ,9| ,7,8, ,9, , ,2, ,5,10| ,1,7, ,8, , ,6, ,3,4,| ,10, , , ,5,1, , , ,2|11, ,4, , , , , , ,12, ,7| , ,9,10, ,8,4, ,3,6,| , , , ,2,1,6,9,| , , ,11, , , , ,9");
                let solver = solver::Solver::new(&game);
                let solved_game = solver.solving();
                assert_eq!(solved_game.unwrap().to_string_with_comma(), "7,12,11,6,1,3,2,4,8,9,5,10|8,9,2,1,12,10,5,11,7,4,3,6|5,3,10,4,6,9,7,8,1,11,12,2|10,11,3,9,5,4,8,1,2,7,6,12|4,5,6,2,7,12,10,3,11,1,9,8|1,7,8,12,9,6,11,2,4,5,10,3|2,1,7,5,8,11,12,6,10,3,4,9|9,10,12,3,4,5,1,7,6,8,2,11|11,6,4,8,3,2,9,10,5,12,1,7|12,2,9,10,11,8,4,5,3,6,7,1|3,8,5,7,2,1,6,9,12,10,11,4|6,4,1,11,10,7,3,12,9,2,8,5".to_string());
            }
        }

        mod setting_4_4 {
            use super::*;
            const SETTING: normal_game::setting::GameSetting = normal_game::setting::GameSetting {
                block_height: 4,
                block_width: 4,
            };
            #[test]
            // #[ignore]
            fn advanced0_33_16x16() {
                let mut game = normal_game::NormalGame::new(SETTING);
                game.load("7, , ,11,4, , ,10,2, , ,1,12, , ,5| , ,6, , ,3, , , , ,16, , ,10| ,10,14, , ,13,7, , ,5,6, , ,3,1,|2, , , , , , ,15,13, , , , , , ,14|8, , , , , , ,1,12, , , , , , ,11| ,13,12, , ,8,15, , ,9,5, , ,14,16| , ,10, , ,2, , , , ,11, , ,7,|15, , ,9,5, , ,12,4, , ,2,6, , ,8|10, , ,5,1, , ,16,15, , ,9,8, , ,4| , ,8, , ,10, , , , ,1, , ,6| ,7,11, , ,4,8, , ,14,12, , ,5,3|4, , , , , , ,5,7, , , , , , ,10|9, , , , , , ,14,10, , , , , , ,1| ,6,2, , ,7,5, , ,11,9, , ,4,8| , ,13, , ,15, , , , ,3, , ,16|16, , ,3,10, , ,4,6, , ,14,13, , ,12");
                let solver = solver::Solver::new(&game);
                let solved_game = solver.solving();
                assert_eq!(solved_game.unwrap().to_string_with_comma(), "7,15,16,11,4,9,14,10,2,3,8,1,12,13,6,5|5,4,6,13,8,3,1,11,14,15,16,12,2,10,9,7|12,10,14,8,16,13,7,2,9,5,6,11,4,3,1,15|2,3,9,1,6,5,12,15,13,10,4,7,16,8,11,14|8,2,5,6,9,16,10,1,12,7,14,13,3,15,4,11|1,13,12,4,11,8,15,7,3,9,5,6,10,14,16,2|3,16,10,14,13,2,4,6,8,1,11,15,5,7,12,9|15,11,7,9,5,14,3,12,4,16,10,2,6,1,13,8|10,14,3,5,1,11,13,16,15,6,2,9,8,12,7,4|13,12,8,15,7,10,2,3,11,4,1,5,9,6,14,16|6,7,11,2,15,4,8,9,16,14,12,10,1,5,3,13|4,9,1,16,14,12,6,5,7,8,13,3,11,2,15,10|9,5,4,12,3,6,16,14,10,13,15,8,7,11,2,1|14,6,2,10,12,7,5,13,1,11,9,16,15,4,8,3|11,1,13,7,2,15,9,8,5,12,3,4,14,16,10,6|16,8,15,3,10,1,11,4,6,2,7,14,13,9,5,12".to_string());
            }
        }
    }
}
