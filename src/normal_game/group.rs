use crate::normal_game::cell;
use crate::normal_game::setting;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

// #[derive(Debug)]
pub struct Group {
    cells: Vec<Rc<RefCell<cell::Cell>>>,
    answer_candidate: Vec<u8>,
}

impl Group {
    pub fn cells(&self) -> &Vec<Rc<RefCell<cell::Cell>>> {
        &self.cells
    }

    /// Get lonely, in the Group.
    ///
    /// Returns the answer_candidate and the position of the cell that is held by only one of the cells that belong to the group.
    ///
    /// Group に所属する Cell の うち 1 つの Cell のみが保有している answer_candidate とその Cell の Position を返却する。
    pub fn get_lonely(&self) -> Vec<(cell::Position, u8)> {
        let mut lonelies: Vec<(cell::Position, u8)> = vec![];
        for candidate in self.answer_candidate.iter() {
            let cells: Vec<cell::Position> = self
                .cells
                .iter()
                .filter(|c| c.borrow().has_answer_candidate(*candidate))
                .map(|c| c.borrow().pos())
                .collect();
            if cells.len() == 1 {
                lonelies.push((cells[0], *candidate));
            }
        }
        lonelies
    }

    /// Remove the specified answer from the unanswerd_candidate.
    pub fn remove_answer_candidate(&mut self, answer: u8) {
        self.answer_candidate = self
            .answer_candidate
            .iter()
            .filter(|n| **n != answer)
            .map(|n| *n)
            .collect();
        self.cells
            .iter()
            .for_each(|c| c.borrow_mut().remove_answer_candidate(answer));
    }

    pub fn is_all_clear_answer_candidate(&self) -> bool {
        self.answer_candidate.len() == 0
    }

    pub fn is_duplicate_answer(&self) -> bool {
        let answers: Vec<u8> = self
            .cells()
            .iter()
            .map(|c| c.borrow().answer())
            .filter(|a| a.is_some())
            .map(|a| a.unwrap())
            .collect();
        let answers_len = answers.len();
        let answers_hash: HashSet<u8> = answers.into_iter().collect();
        return answers_len != answers_hash.len();
    }
}

pub fn create_groups(
    cells: &Vec<Rc<RefCell<cell::Cell>>>,
    setting: &setting::GameSetting,
) -> Vec<Rc<RefCell<Group>>> {
    let hg = create_horizontal_groups(&cells, &setting);
    let vg = create_vertical_groups(&cells, &setting);
    let gg = create_block_groups(&cells, &setting);
    vec![hg, vg, gg]
        .iter()
        .flatten()
        .map(|g| g.clone())
        .collect()
}

fn create_vertical_groups(
    cells: &Vec<Rc<RefCell<cell::Cell>>>,
    setting: &setting::GameSetting,
) -> Vec<Rc<RefCell<Group>>> {
    let x_pos: Vec<u8> = (0..setting.side_size()).collect();
    x_pos
        .iter()
        .map(|x| {
            Rc::new(RefCell::new(Group {
                cells: cells
                    .iter()
                    .filter(|c| c.borrow().pos().x() == *x)
                    .map(|c| c.clone())
                    .collect(),
                answer_candidate: setting.answer_candidate().clone(),
            }))
        })
        .collect()
}

fn create_horizontal_groups(
    cells: &Vec<Rc<RefCell<cell::Cell>>>,
    setting: &setting::GameSetting,
) -> Vec<Rc<RefCell<Group>>> {
    let y_pos: Vec<u8> = (0..setting.side_size()).collect();
    y_pos
        .iter()
        .map(|y| {
            Rc::new(RefCell::new(Group {
                cells: cells
                    .iter()
                    .filter(|c| c.borrow().pos().y() == *y)
                    .map(|c| c.clone())
                    .collect(),
                answer_candidate: setting.answer_candidate().clone(),
            }))
        })
        .collect()
}

fn create_block_groups(
    cells: &Vec<Rc<RefCell<cell::Cell>>>,
    setting: &setting::GameSetting,
) -> Vec<Rc<RefCell<Group>>> {
    let block_start_positions = create_block_start_positions(setting);
    let mut vec: Vec<Rc<RefCell<Group>>> = vec![];
    for start_pos in block_start_positions {
        let mut one_group_cells: Vec<Rc<RefCell<cell::Cell>>> = vec![];
        for y in 0..setting.block_height() {
            for x in 0..setting.block_width() {
                let pos = start_pos.move_y(y as i16).move_x(x as i16);
                one_group_cells.push(
                    cells
                        .iter()
                        .find(|c| c.borrow().pos() == pos)
                        .unwrap()
                        .clone(),
                );
            }
        }
        vec.push(Rc::new(RefCell::new(Group {
            cells: one_group_cells,
            answer_candidate: setting.answer_candidate().clone(),
        })))
    }
    return vec;
}

fn create_block_start_positions(setting: &setting::GameSetting) -> Vec<cell::Position> {
    let side_num_list: Vec<u8> = (0..setting.side_size()).collect();
    let block_start_y_list: Vec<u8> = side_num_list
        .iter()
        .filter(|n| *n % setting.block_height() == 0)
        .map(|n| *n)
        .collect();
    let block_start_x_list: Vec<u8> = side_num_list
        .iter()
        .filter(|n| *n % setting.block_width() == 0)
        .map(|n| *n)
        .collect();
    block_start_y_list
        .iter()
        .map(|y| {
            block_start_x_list
                .iter()
                .map(|x| cell::Position::new(*x, *y))
                .collect::<Vec<cell::Position>>()
        })
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::normal_game::setting::BlockSize;

    fn setting() -> setting::GameSetting {
        setting::GameSetting::new(BlockSize {
            height: 2,
            width: 3,
        })
    }
    #[test]
    fn test_create_vertical_groups() {
        let vg = create_vertical_groups(&cell::create_cells(&setting()), &setting());
        assert_eq!(vg.len(), 6);
        assert_eq!(vg[0].borrow().cells.len(), 6);
        assert_eq!(
            vg[0].borrow().cells.get(0).unwrap().borrow().pos(),
            cell::Position::new(0, 0)
        );
        assert_eq!(
            vg[0].borrow().cells.get(5).unwrap().borrow().pos(),
            cell::Position::new(0, 5)
        );
        assert_eq!(
            vg[5].borrow().cells.get(0).unwrap().borrow().pos(),
            cell::Position::new(5, 0)
        );
        assert_eq!(
            vg[5].borrow().cells.get(5).unwrap().borrow().pos(),
            cell::Position::new(5, 5)
        );
    }
    #[test]
    fn test_create_horizontal_groups() {
        let hg = create_horizontal_groups(&cell::create_cells(&setting()), &setting());
        assert_eq!(hg.len(), 6);
        assert_eq!(hg[0].borrow().cells.len(), 6);
        assert_eq!(
            hg[0].borrow().cells.get(0).unwrap().borrow().pos(),
            cell::Position::new(0, 0)
        );
        assert_eq!(
            hg[0].borrow().cells.get(5).unwrap().borrow().pos(),
            cell::Position::new(5, 0)
        );
        assert_eq!(
            hg[5].borrow().cells.get(0).unwrap().borrow().pos(),
            cell::Position::new(0, 5)
        );
        assert_eq!(
            hg[5].borrow().cells.get(5).unwrap().borrow().pos(),
            cell::Position::new(5, 5)
        );
    }
    mod test_create_block_groups {
        use super::*;
        #[test]
        fn test_create_block_start_positions() {
            let block_start_positions = create_block_start_positions(&setting());
            assert_eq!(
                block_start_positions,
                vec![
                    cell::Position::new(0, 0),
                    cell::Position::new(3, 0),
                    cell::Position::new(0, 2),
                    cell::Position::new(3, 2),
                    cell::Position::new(0, 4),
                    cell::Position::new(3, 4)
                ]
            )
        }
        #[test]
        fn block_group_count() {
            let groups = create_block_groups(&cell::create_cells(&setting()), &setting());
            assert_eq!(groups.len(), 6);
        }
        #[test]
        fn block_group_cell_count() {
            let groups = create_block_groups(&cell::create_cells(&setting()), &setting());
            assert!(groups.iter().all(|g| g.borrow().cells.len() == 6));
        }
        #[test]
        fn first_block_group_cells() {
            let groups = create_block_groups(&cell::create_cells(&setting()), &setting());
            assert_eq!(
                groups[0]
                    .borrow()
                    .cells
                    .iter()
                    .map(|c| c.borrow().pos())
                    .collect::<Vec<cell::Position>>(),
                vec![
                    cell::Position::new(0, 0),
                    cell::Position::new(1, 0),
                    cell::Position::new(2, 0),
                    cell::Position::new(0, 1),
                    cell::Position::new(1, 1),
                    cell::Position::new(2, 1),
                ]
            );
        }
        #[test]
        fn last_block_group_cells() {
            let groups = create_block_groups(&cell::create_cells(&setting()), &setting());
            assert_eq!(
                groups[5]
                    .borrow()
                    .cells
                    .iter()
                    .map(|c| c.borrow().pos())
                    .collect::<Vec<cell::Position>>(),
                vec![
                    cell::Position::new(3, 4),
                    cell::Position::new(4, 4),
                    cell::Position::new(5, 4),
                    cell::Position::new(3, 5),
                    cell::Position::new(4, 5),
                    cell::Position::new(5, 5),
                ]
            );
        }
    }
    #[test]
    fn get_lonely_returns_lonely() {
        let g = create_horizontal_groups(&cell::create_cells(&setting()), &setting());
        // index が 2 以外の cell の解答候補から 3 を除去
        for (i, cell) in g[0].borrow().cells.iter().enumerate() {
            if i == 2 {
                continue;
            };
            cell.borrow_mut().remove_answer_candidate(3);
        }
        // index が 4 以外の cell の解答候補から 5 を除去
        for (i, cell) in g[0].borrow().cells.iter().enumerate() {
            if i == 4 {
                continue;
            };
            cell.borrow_mut().remove_answer_candidate(5);
        }
        assert_eq!(
            g[0].borrow().get_lonely(),
            vec![
                (cell::Position::new(2, 0), 3),
                (cell::Position::new(4, 0), 5)
            ]
        );
    }
    mod is_duplicate_answer {
        use super::*;
        #[test]
        fn duplicated() {
            let g = create_horizontal_groups(&cell::create_cells(&setting()), &setting());
            let g = g[0].borrow_mut();
            let cells = g.cells();
            cells[0].borrow_mut().set_answer(1);
            cells[1].borrow_mut().set_answer(1);
            cells[2].borrow_mut().set_answer(3);
            cells[3].borrow_mut().set_answer(4);
            cells[4].borrow_mut().set_answer(5);
            cells[5].borrow_mut().set_answer(6);
            assert!(g.is_duplicate_answer());
        }
        #[test]
        fn not_duplicated() {
            let g = create_horizontal_groups(&cell::create_cells(&setting()), &setting());
            let g = g[0].borrow_mut();
            let cells = g.cells();
            cells[0].borrow_mut().set_answer(1);
            cells[1].borrow_mut().set_answer(2);
            cells[2].borrow_mut().set_answer(3);
            cells[3].borrow_mut().set_answer(4);
            cells[4].borrow_mut().set_answer(5);
            cells[5].borrow_mut().set_answer(6);
            assert!(!g.is_duplicate_answer());
        }
    }
}
