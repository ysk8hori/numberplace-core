use crate::normal_game::setting;
use std::rc::Rc;

#[derive(Debug)]
pub struct Cell {
    pos: Position,
}

#[derive(Debug, PartialEq)]
pub struct Position(u8, u8);
impl Position {
    pub fn row(&self) -> u8 {
        self.0
    }
    pub fn col(&self) -> u8 {
        self.1
    }
}

pub struct Cells {
    cells: Vec<Rc<Cell>>,
}

impl Cells {
    pub fn len(&self) -> usize {
        self.cells.len()
    }
}

impl Cells {
    pub fn filter_by_row(&self, row: u8) -> Vec<Rc<Cell>> {
        self.cells
            .iter()
            .filter(|c| c.pos.row() == row)
            .map(|c| c.clone())
            .collect()
    }

    pub fn create_cells(setting: &setting::GameSetting) -> Cells {
        let mut cells = Vec::new();
        for row in 1..=setting.side_size() {
            for col in 1..=setting.side_size() {
                cells.push(Rc::new(Cell {
                    pos: Position(row, col),
                }));
            }
        }
        Cells { cells }
    }
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
                    Cells::create_cells(&setting::GameSetting {
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
                    Cells::create_cells(&setting::GameSetting {
                        block_height: 2,
                        block_width: 3
                    })
                    .cells[0]
                        .pos,
                    Position(1, 1)
                );
            }
            #[test]
            fn second_cell_position_is_1_2() {
                assert_eq!(
                    Cells::create_cells(&setting::GameSetting {
                        block_height: 2,
                        block_width: 3
                    })
                    .cells[1]
                        .pos,
                    Position(1, 2)
                );
            }
            #[test]
            fn last_cell_position_is_6_6() {
                assert_eq!(
                    Cells::create_cells(&setting::GameSetting {
                        block_height: 2,
                        block_width: 3
                    })
                    .cells[35]
                        .pos,
                    Position(6, 6)
                );
            }
        }
    }
    mod cells {
        use super::*;
        #[test]
        fn test_filter_by_row() {
            let cells_by_row = Cells::create_cells(&setting::GameSetting {
                block_height: 2,
                block_width: 3,
            })
            .filter_by_row(3);
            let rows: Vec<&Position> = cells_by_row.iter().map(|c| &c.pos).collect();
            assert_eq!(
                rows,
                [
                    &Position(3, 1),
                    &Position(3, 2),
                    &Position(3, 3),
                    &Position(3, 4),
                    &Position(3, 5),
                    &Position(3, 6),
                ]
            )
        }
    }
}
