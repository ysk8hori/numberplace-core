use crate::normal_game::cell::Cell;
use crate::normal_game::cell::Position;
use crate::normal_game::NormalGame;
use core::cell::RefCell;
use rand::prelude::*;
use std::rc::Rc;

impl NormalGame {
    pub fn shuffle(&mut self) {
        self.shuffle_manualy(|len: usize| -> usize {
            let mut rng = thread_rng();
            rng.gen_range(0..len)
        })
    }
    fn shuffle_manualy<F>(&mut self, random_index: F)
    where
        F: Fn(usize) -> usize,
    {
        self.shuffle_cols(&random_index);
        self.shuffle_rows(&random_index);
        self.shuffle_row_blocks(&random_index);
        self.shuffle_col_blocks(&random_index);
    }
    fn shuffle_rows<F>(&mut self, random_index: F)
    where
        F: Fn(usize) -> usize,
    {
        for block_index in 0..self.setting().block_width() {
            let start_y = block_index * self.setting().block_height();
            let mut row_indexes: Vec<u8> =
                (start_y..(self.setting().block_height() + start_y)).collect();
            // Since it is not possible to iterate while changing the y-coordinate,
            // list the cells with the specified y-coordinate in advance.
            let mut cell_lines: Vec<Vec<&Rc<RefCell<Cell>>>> = Vec::new();
            for row in row_indexes.clone() {
                cell_lines.push(
                    self.cells()
                        .iter()
                        .filter(|c| c.borrow().pos().y() == row)
                        .collect(),
                );
            }
            for cell_line in cell_lines {
                let i = random_index(row_indexes.len());
                let to = row_indexes.remove(i);
                cell_line.iter().for_each(|c| {
                    let pos = c.borrow().pos();
                    c.borrow_mut().move_to(Position::new(pos.x(), to));
                });
            }
        }
    }
    fn shuffle_cols<F>(&mut self, random_index: F)
    where
        F: Fn(usize) -> usize,
    {
        for block_index in 0..self.setting().block_height() {
            let start_x = block_index * self.setting().block_width();
            let mut col_indexes: Vec<u8> =
                (start_x..(self.setting().block_width() + start_x)).collect();
            // Since it is not possible to iterate while changing the x-coordinate,
            // list the cells with the specified x-coordinate in advance.
            let mut cell_lines: Vec<Vec<&Rc<RefCell<Cell>>>> = Vec::new();
            for col in col_indexes.clone() {
                cell_lines.push(
                    self.cells()
                        .iter()
                        .filter(|c| c.borrow().pos().x() == col)
                        .collect(),
                );
            }
            for cell_line in cell_lines {
                let i = random_index(col_indexes.len());
                let to = col_indexes.remove(i);
                cell_line.iter().for_each(|c| {
                    let pos = c.borrow().pos();
                    c.borrow_mut().move_to(Position::new(to, pos.y()));
                });
            }
        }
    }
    fn shuffle_row_blocks<F>(&mut self, random_index: F)
    where
        F: Fn(usize) -> usize,
    {
        let mut block_indexes: Vec<u8> = (0..self.setting().block_width()).collect();

        let mut block_lines: Vec<Vec<&Rc<RefCell<Cell>>>> = Vec::new();

        // ブロック行の単位でセルの配列を収集する
        for block_row_index in block_indexes.clone() {
            let mut inner_lines: Vec<Vec<&Rc<RefCell<Cell>>>> = Vec::new();
            for inner_row_no in 0..self.setting().block_height() {
                inner_lines.push(
                    self.cells()
                        .iter()
                        .filter(|c| {
                            c.borrow().pos().y()
                                == block_row_index * self.setting().block_height() + inner_row_no
                        })
                        .collect(),
                );
            }
            block_lines.push(inner_lines.concat());
        }

        // セルの移動を行う
        for (block_row, block_line) in block_lines.iter().enumerate() {
            let i = random_index(block_indexes.len());
            let to = block_indexes.remove(i);
            // block_line 内のセルを移動する。現在の block row と to の値の差分に self.setting().block_height() の値を掛け算し、セルの y 座標をムーブするのが良い気がする。
            let move_count =
                ((to as i16) - (block_row as i16)) * (self.setting().block_height() as i16);
            block_line.iter().for_each(|c| {
                let pos = c.borrow().pos();
                c.borrow_mut().move_to(pos.move_y(move_count))
            });
        }
    }
    fn shuffle_col_blocks<F>(&mut self, random_index: F)
    where
        F: Fn(usize) -> usize,
    {
        let mut block_indexes: Vec<u8> = (0..self.setting().block_height()).collect();

        let mut block_lines: Vec<Vec<&Rc<RefCell<Cell>>>> = Vec::new();

        // ブロック行の単位でセルの配列を収集する
        for block_col_index in block_indexes.clone() {
            let mut inner_lines: Vec<Vec<&Rc<RefCell<Cell>>>> = Vec::new();
            for inner_col_no in 0..self.setting().block_width() {
                inner_lines.push(
                    self.cells()
                        .iter()
                        .filter(|c| {
                            c.borrow().pos().x()
                                == block_col_index * self.setting().block_width() + inner_col_no
                        })
                        .collect(),
                );
            }
            block_lines.push(inner_lines.concat());
        }

        // セルの移動を行う
        for (block_col, block_line) in block_lines.iter().enumerate() {
            let i = random_index(block_indexes.len());
            let to = block_indexes.remove(i);
            let move_count =
                ((to as i16) - (block_col as i16)) * (self.setting().block_width() as i16);
            block_line.iter().for_each(|c| {
                let pos = c.borrow().pos();
                c.borrow_mut().move_to(pos.move_x(move_count))
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::normal_game::setting::BlockSize;
    use crate::normal_game::setting::GameSetting;
    #[test]
    fn test_shuffle_rows() {
        let mut game = NormalGame::new(GameSetting::new(BlockSize {
            height: 2,
            width: 2,
        }));
        game.load("1234|3412|2143|4321");
        game.shuffle_rows(|len| len - 1);
        assert_eq!(game.to_string(), "3412|1234|4321|2143");
    }
    #[test]
    fn test_shuffle_cols() {
        let mut game = NormalGame::new(GameSetting::new(BlockSize {
            height: 2,
            width: 2,
        }));
        game.load("1234|3412|2143|4321");
        game.shuffle_cols(|len| len - 1);
        assert_eq!(game.to_string(), "2143|4321|1234|3412");
    }
    #[test]
    fn test_shuffle_row_blocks() {
        let mut game = NormalGame::new(GameSetting::new(BlockSize {
            height: 2,
            width: 3,
        }));
        game.load("123456|456123|312645|645312|231564|564231");
        game.shuffle_row_blocks(|len| len - 1);
        assert_eq!(
            game.to_string(),
            "231564|564231|312645|645312|123456|456123"
        );
    }
    #[test]
    fn test_shuffle_col_blocks() {
        let mut game = NormalGame::new(GameSetting::new(BlockSize {
            height: 3,
            width: 2,
        }));
        game.load("143625|251436|362514|416352|524163|635241");
        game.shuffle_col_blocks(|len| len - 1);
        assert_eq!(
            game.to_string(),
            "253614|361425|142536|526341|634152|415263"
        );
    }
    #[test]
    fn how_to_reverse() {
        let mut game = NormalGame::new(GameSetting::new(BlockSize {
            height: 3,
            width: 3,
        }));
        game.load("582397146|496152387|371486952|148639725|629875431|735241698|953768214|264513879|817924563");
        game.shuffle_manualy(|len| len - 1);
        assert_eq!(
            game.to_string(),
            "365429718|978315462|412867359|896142537|134578926|527936841|259684173|783251694|641793285"
        );
    }
    #[test]
    fn test_shuffle() {
        let mut game = NormalGame::new(GameSetting::new(BlockSize {
            height: 3,
            width: 3,
        }));
        game.load("582397146|496152387|371486952|148639725|629875431|735241698|953768214|264513879|817924563");
        game.shuffle();
        assert_ne!(
            game.to_string(),
            "582397146|496152387|371486952|148639725|629875431|735241698|953768214|264513879|817924563"
        );
    }
}
