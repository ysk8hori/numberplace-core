use crate::normal_game::setting;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Cell {
    pos: Position,
}

impl Cell {
    pub fn new(position: Position) -> Cell {
        Cell { pos: position }
    }
    pub fn pos(&self) -> Position {
        self.pos
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Position(u8, u8);
impl Position {
    pub fn new(row: u8, col: u8) -> Position {
        Position(row, col)
    }
    pub fn row(&self) -> u8 {
        self.0
    }
    pub fn col(&self) -> u8 {
        self.1
    }
    pub fn add_row(&self, count: u8) -> Position {
        Position(self.0 + count, self.1)
    }
    pub fn add_col(&self, count: u8) -> Position {
        Position(self.0, self.1 + count)
    }
}

#[derive(Debug)]
pub struct Cells {
    cells: Vec<Rc<Cell>>,
}

impl Cells {
    pub fn new(cells: Vec<Rc<Cell>>) -> Cells {
        Cells { cells }
    }
    pub fn len(&self) -> usize {
        self.cells.len()
    }
    pub fn filter<P>(&self, predicate: P) -> Vec<Rc<Cell>>
    where
        P: FnMut(&&Rc<Cell>) -> bool,
    {
        self.cells
            .iter()
            .filter(predicate)
            .map(|c| c.clone())
            .collect()
    }
    pub fn filter_by_row(&self, row: u8) -> Vec<Rc<Cell>> {
        self.filter(|c| c.pos.row() == row)
    }
    pub fn filter_by_column(&self, column: u8) -> Vec<Rc<Cell>> {
        self.filter(|c| c.pos.col() == column)
    }
    pub fn find_by_position(&self, position: &Position) -> Option<Rc<Cell>> {
        self.cells
            .iter()
            .map(|cell| Rc::clone(&cell))
            .find(|cell| cell.pos() == *position)
    }
    pub fn find_by_index(&self, index: usize) -> Option<Rc<Cell>> {
        if self.cells.len() <= index {
            None
        } else {
            Some(self.cells[index].clone())
        }
    }

    pub fn create_cells(setting: &setting::GameSetting) -> Cells {
        let mut cells = Vec::new();
        for row in 0..setting.side_size() {
            for col in 0..setting.side_size() {
                cells.push(Rc::new(Cell {
                    pos: Position(row, col),
                }));
            }
        }
        Cells { cells }
    }

    pub fn positions(&self) -> Vec<Position> {
        self.cells
            .iter()
            .map(|c| c.pos())
            .collect::<Vec<Position>>()
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
                    Position(0, 0)
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
                    Position(0, 1)
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
                    Position(5, 5)
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
            .filter_by_row(2);
            let rows: Vec<&Position> = cells_by_row.iter().map(|c| &c.pos).collect();
            assert_eq!(
                rows,
                [
                    &Position(2, 0),
                    &Position(2, 1),
                    &Position(2, 2),
                    &Position(2, 3),
                    &Position(2, 4),
                    &Position(2, 5),
                ]
            )
        }
        #[test]
        fn test_filter() {
            let cells_by_col = Cells::create_cells(&setting::GameSetting {
                block_height: 2,
                block_width: 3,
            })
            .filter(|c| c.pos.col() == 2);
            let rows: Vec<&Position> = cells_by_col.iter().map(|c| &c.pos).collect();
            assert_eq!(
                rows,
                [
                    &Position(0, 2),
                    &Position(1, 2),
                    &Position(2, 2),
                    &Position(3, 2),
                    &Position(4, 2),
                    &Position(5, 2),
                ]
            )
        }
        #[test]
        fn test_filter_by_column() {
            let cells_by_col = Cells::create_cells(&setting::GameSetting {
                block_height: 2,
                block_width: 3,
            })
            .filter_by_column(2);
            let rows: Vec<&Position> = cells_by_col.iter().map(|c| &c.pos).collect();
            assert_eq!(
                rows,
                [
                    &Position(0, 2),
                    &Position(1, 2),
                    &Position(2, 2),
                    &Position(3, 2),
                    &Position(4, 2),
                    &Position(5, 2),
                ]
            )
        }
        #[test]
        fn test_find_by_position() {
            let pos = Position(0, 0);
            assert_eq!(
                *Cells::create_cells(&setting::GameSetting {
                    block_height: 2,
                    block_width: 3,
                })
                .find_by_position(&pos)
                .unwrap()
                .as_ref(),
                Cell::new(pos)
            );
        }
    }
    mod test_position {
        use super::*;
        #[test]
        fn test_assert_eq() {
            assert_eq!(Position::new(1, 2), Position::new(1, 2));
        }
        #[test]
        fn test_add_row() {
            assert_eq!(Position::new(1, 2).add_row(3), Position::new(4, 2))
        }
        #[test]
        fn test_add_col() {
            assert_eq!(Position::new(1, 2).add_col(3), Position::new(1, 5))
        }
    }
}
