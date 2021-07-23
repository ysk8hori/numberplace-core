use crate::normal_game::setting;

#[derive(Debug)]
pub struct Cell {
    pos: Position,
}

#[derive(Debug, PartialEq)]
pub struct Position {
    row: u8,
    col: u8,
}

pub fn create_cells(setting: setting::GameSetting) -> Vec<Cell> {
    let mut ret = Vec::new();
    for row in 1..=setting.side_size() {
        for col in 1..=setting.side_size() {
            ret.push(Cell {
                pos: Position { row, col },
            });
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    mod call_create_cells {
        use super::*;
        mod given_3_2 {
            use super::*;
            #[test]
            fn returns_36_cells() {
                assert_eq!(
                    create_cells(setting::GameSetting {
                        block_height: 2,
                        block_width: 3
                    })
                    .len(),
                    36
                )
            }
            #[test]
            fn first_cell_position_is_1_1() {
                assert_eq!(
                    create_cells(setting::GameSetting {
                        block_height: 2,
                        block_width: 3
                    })[0]
                        .pos,
                    Position { row: 1, col: 1 }
                );
            }
            #[test]
            fn second_cell_position_is_2_1() {
                assert_eq!(
                    create_cells(setting::GameSetting {
                        block_height: 2,
                        block_width: 3
                    })[1]
                        .pos,
                    Position { row: 1, col: 2 }
                );
            }
            #[test]
            fn last_cell_position_is_9_9() {
                assert_eq!(
                    create_cells(setting::GameSetting {
                        block_height: 2,
                        block_width: 3
                    })[35]
                        .pos,
                    Position { row: 6, col: 6 }
                );
            }
        }
    }
}
